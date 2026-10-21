// Windows probe for the `onboard` loopback sign-in: does the approval page's
// redirect reach the CLI, and if not, where does it go?
//
// Runs the real CLI (CLI_BIN) against a stub approval page on 127.0.0.1:9000
// and a stub API on 127.0.0.1:9100, and drives real Chrome and Edge over CDP.
// Every request to elevenlabs.io is answered locally; nothing leaves the runner.
//
//   CLI_BIN=...\elevenlabs.exe LABEL=1.7.0 node probe.mjs [scenario-filter]

import { spawn, spawnSync } from "node:child_process";
import fs from "node:fs";
import http from "node:http";
import net from "node:net";
import os from "node:os";
import path from "node:path";
import dns from "node:dns/promises";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright-core";

const CLI = process.env.CLI_BIN;
const LABEL = process.env.LABEL ?? "cli";
const FILTER = process.argv[2] ? new RegExp(process.argv[2]) : null;
const HERE = path.dirname(fileURLToPath(import.meta.url));
const PORTS = [8484, 8483, 8482];
const rows = [];

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
// `promise`, or a rejection after `ms`: no step may hang the probe.
const within = (ms, promise, what) =>
  Promise.race([promise, sleep(ms).then(() => Promise.reject(new Error(`${what}: no answer in ${ms} ms`)))]);
const tmp = (p) => fs.mkdtempSync(path.join(os.tmpdir(), p));

function ps(script, timeout = 30000) {
  const r = spawnSync("powershell", ["-NoProfile", "-Command", script], { encoding: "utf8", timeout });
  return (r.stdout ?? "").trim() + (r.stderr ? `\n${r.stderr.trim()}` : "");
}

function listeners() {
  const out = ps(
    `Get-NetTCPConnection -State Listen -LocalPort ${PORTS.join(",")} -ErrorAction SilentlyContinue | ` +
      `ForEach-Object { "$($_.LocalAddress)|$($_.LocalPort)|$((Get-Process -Id $_.OwningProcess).ProcessName)" }`
  );
  return out
    .split(/\r?\n/)
    .filter((l) => l.includes("|"))
    .map((l) => {
      const [addr, port, proc] = l.split("|");
      return `${addr.includes(":") ? `[${addr}]` : addr}:${port} ${proc}`;
    })
    .sort();
}

async function killCli() {
  spawnSync("taskkill", ["/F", "/IM", path.basename(CLI)], { stdio: "ignore" });
  for (let i = 0; i < 20; i++) {
    if (!listeners().some((l) => l.includes(path.basename(CLI, ".exe")))) return;
    await sleep(250);
  }
}

// --- stub servers -------------------------------------------------------------

const STUB_HTML = `<!doctype html><title>stub approval</title><button id=a>Authorize</button><script>
const q = new URLSearchParams(location.search);
async function go() {
  await fetch('/grant', { method: 'POST' }).catch(() => {});
  const u = new URL(q.get('redirect_uri'));
  if (q.get('probe_mode') === 'approve') u.searchParams.set('code', 'probe-code');
  else u.searchParams.set('error', 'access_denied');
  u.searchParams.set('state', q.get('state'));
  setTimeout(() => { window.location.href = u.toString(); }, Number(q.get('probe_delay') || '0'));
}
document.getElementById('a').onclick = go;
if (q.get('probe_click') !== '1') go();
</script>`;

http
  .createServer((req, res) => {
    if (req.url.startsWith("/grant")) return res.end("{}");
    res.setHeader("content-type", "text/html");
    res.end(STUB_HTML);
  })
  .listen(9000, "127.0.0.1");

http
  .createServer((req, res) => {
    res.setHeader("content-type", "application/json");
    if (req.url.startsWith("/v1/oauth/token")) {
      return res.end(JSON.stringify({ access_token: "probe-token", token_type: "Bearer", expires_in: 3600 }));
    }
    res.end(JSON.stringify({ user_id: "probe", subscription: { tier: "free" } }));
  })
  .listen(9100, "127.0.0.1");

// A process that is not this CLI, holding a loopback port.
let onDecoyHit = null;
function decoy(name, host, port, ipv6Only) {
  const server = http.createServer((req, res) => {
    onDecoyHit?.(name, req.url);
    res.setHeader("connection", "close");
    res.end(`decoy ${name}`);
  });
  const close = () => {
    server.closeAllConnections();
    return within(5000, new Promise((r) => server.close(r)), `close decoy ${name}`).catch(() => {});
  };
  return new Promise((resolve, reject) => {
    server.once("error", reject);
    server.listen({ host, port, ipv6Only }, () => resolve({ name, close }));
  });
}

// --- the CLI ----------------------------------------------------------------

function newProject() {
  const home = tmp("probe-home-");
  const dir = tmp("probe-proj-");
  fs.writeFileSync(path.join(dir, "package.json"), '{"name":"probe","version":"1.0.0"}\n');
  return { home, dir };
}

function cliEnv(home) {
  const env = { ...process.env, HOME: home, USERPROFILE: home, ELEVENLABS_ONBOARD_NO_CLIPBOARD: "1" };
  for (const k of ["ELEVENLABS_API_KEY", "ELEVENLABS_BASE_URL", "XDG_CONFIG_HOME"]) delete env[k];
  return env;
}

function connectArgs(extra = []) {
  return [
    "--base-url", "http://127.0.0.1:9100",
    "onboard", "connect", "--no-browser",
    "--authorize-url", "http://127.0.0.1:9000/authorize",
    ...extra,
  ];
}

function parseReply(stdout) {
  const line = stdout.trim().split(/\r?\n/).pop() ?? "";
  try {
    const reply = JSON.parse(line);
    const url = (reply.say ?? "").match(/http:\/\/127\.0\.0\.1:9000\/authorize\?\S+/)?.[0];
    return { reply, url };
  } catch {
    return { reply: { raw: stdout.slice(-400) }, url: undefined };
  }
}

// `connect` the way an agent runs it: returns once the approval link is known.
function connect(p, extra = [], wrap = null, cli = CLI) {
  const args = connectArgs(extra);
  const [cmd, argv] = wrap ? wrap(cli, args) : [cli, args];
  const r = spawnSync(cmd, argv, { cwd: p.dir, env: cliEnv(p.home), encoding: "utf8", timeout: 60000 });
  const parsed = parseReply(r.stdout ?? "");
  if (!parsed.url) parsed.error = `${r.status} ${(r.stderr ?? "").slice(-400)}`;
  return parsed;
}

function workerResult(p) {
  const root = path.join(p.home, ".elevenlabs", "onboard");
  for (const d of fs.existsSync(root) ? fs.readdirSync(root) : []) {
    const f = path.join(root, d, "result.json");
    if (fs.existsSync(f)) {
      const r = JSON.parse(fs.readFileSync(f, "utf8"));
      return r.details?.key_status ?? r.next ?? "?";
    }
  }
  return "none";
}

function workerLog(p) {
  const root = path.join(p.home, ".elevenlabs", "onboard");
  return (fs.existsSync(root) ? fs.readdirSync(root) : [])
    .map((d) => path.join(root, d, "worker.log"))
    .filter((f) => fs.existsSync(f))
    .map((f) => fs.readFileSync(f, "utf8").split(/\r?\n/).filter((l) => !l.includes("URL:")).slice(-4).join(" / "))
    .join(" | ");
}

// --- browsers ---------------------------------------------------------------

const EXE = {
  chrome: process.env.PROBE_CHROME ?? "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
  msedge: process.env.PROBE_EDGE ?? "C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe",
};
const browsers = {};

async function browser(name) {
  if (browsers[name]) return browsers[name];
  const port = name === "chrome" ? 9222 : 9223;
  const proc = spawn(
    EXE[name],
    [
      "--headless=new",
      `--remote-debugging-port=${port}`,
      `--user-data-dir=${tmp(`probe-${name}-`)}`,
      "--no-first-run",
      "--no-default-browser-check",
      // Nothing may reach the real site: pages there are served by `nav`, and
      // the result page the CLI redirects to fails to resolve.
      "--host-resolver-rules=MAP elevenlabs.io ~NOTFOUND, MAP *.elevenlabs.io ~NOTFOUND",
      "about:blank",
    ],
    { stdio: "ignore" }
  );
  for (let i = 0; i < 60; i++) {
    try {
      const b = await chromium.connectOverCDP(`http://127.0.0.1:${port}`);
      browsers[name] = { b, proc };
      return browsers[name];
    } catch {
      await sleep(500);
    }
  }
  throw new Error(`${name} did not start`);
}

// Open `url` the way the approval page is opened, and report where its
// redirect to the loopback callback lands.
async function nav(name, url, { timeoutMs = 15000, click = false, during = null } = {}) {
  const { b } = await browser(name);
  const context = b.contexts()[0];
  const page = await within(15000, context.newPage(), "new page");
  const t0 = Date.now();
  let finish;
  const result = new Promise((resolve) => {
    finish = (outcome, detail = "") => resolve({ outcome, detail, ms: Date.now() - t0 });
  });
  onDecoyHit = (decoyName) => finish(`decoy ${decoyName}`);
  await within(10000, page.route("https://elevenlabs.io/**", async (route) => {
    const u = new URL(route.request().url());
    if (u.pathname === "/app/oauth/authorize") return route.fulfill({ status: 200, contentType: "text/html", body: STUB_HTML });
    if (u.pathname === "/grant") return route.fulfill({ status: 200, contentType: "application/json", body: "{}" });
    return route.abort();
  }), "route");
  if (process.env.PROBE_DEBUG) {
    page.on("request", (r) => console.log("  req", r.url().slice(0, 120)));
    page.on("response", (r) => console.log("  res", r.status(), r.url().slice(0, 120)));
  }
  // The CLI answers the callback with a redirect to the result page.
  page.on("response", (r) => {
    if (!/^http:\/\/localhost:\d+\/callback/.test(r.url())) return;
    const location = r.headers().location;
    if (!location) return finish("cli?", `HTTP ${r.status()}`);
    const u = new URL(location);
    finish("cli", `${u.pathname.split("/").pop()}${u.searchParams.get("error") ? `:${u.searchParams.get("error")}` : ""}`);
  });
  page.on("requestfailed", (r) => {
    if (r.isNavigationRequest() && /^https?:\/\/localhost/.test(r.url())) finish("neterr", r.failure()?.errorText ?? "");
  });
  const timer = setTimeout(() => finish("hang"), timeoutMs);
  page.goto(url, { timeout: timeoutMs }).catch(() => {});
  if (click) page.click("#a", { timeout: timeoutMs }).catch(() => {});
  if (during) during();
  const r = await result;
  clearTimeout(timer);
  onDecoyHit = null;
  await within(5000, page.close(), "close page").catch(() => {});
  return r;
}

function withProbe(url, params) {
  const u = new URL(url);
  for (const [k, v] of Object.entries(params)) u.searchParams.set(k, v);
  return u.toString();
}

// --- scenarios --------------------------------------------------------------

function record(scenario, browserName, bind, r, worker, note = "") {
  const row = { cli: LABEL, scenario, browser: browserName, bind: bind.join(", "), nav: `${r.outcome}${r.detail ? ` (${r.detail})` : ""}`, ms: r.ms, worker, note };
  rows.push(row);
  console.log(JSON.stringify(row));
}

// One sign-in: optional setup, `connect`, one or more approvals, teardown.
async function signIn(scenario, { browsers: names = ["chrome", "msedge"], setup, teardown, params = {}, click = false, public: pub = false, timeoutMs, during, wrap } = {}) {
  for (const name of names) {
    await killCli();
    const ctx = setup ? await setup() : {};
    const p = newProject();
    const c = connect(p, [], wrap);
    if (wrap) await sleep(1000);
    const bind = listeners();
    if (!c.url) {
      record(scenario, name, bind, { outcome: "no-link", detail: c.error, ms: 0 }, workerResult(p));
    } else {
      let url = withProbe(c.url, { probe_click: click ? "1" : "0", ...params });
      if (pub) url = url.replace("http://127.0.0.1:9000", "https://elevenlabs.io/app/oauth");
      const r = await nav(name, url, { timeoutMs, click, during: during ? () => during(bind) : null });
      await sleep(1500);
      record(scenario, name, bind, r, workerResult(p), r.outcome === "cli" ? "" : workerLog(p).slice(0, 300));
    }
    if (teardown) await teardown(ctx);
    else for (const d of ctx.decoys ?? []) await d.close();
  }
}

const scenarios = {
  async env() {
    console.log("## environment");
    console.log(ps("[System.Environment]::OSVersion.VersionString; (Get-CimInstance Win32_OperatingSystem).Caption"));
    console.log("dotnet localhost:", ps("[System.Net.Dns]::GetHostAddresses('localhost') | ForEach-Object { $_.ToString() }").replace(/\r?\n/g, ", "));
    console.log("node localhost:", JSON.stringify(await dns.lookup("localhost", { all: true, order: "verbatim" })));
    console.log(ps("netsh interface ipv4 show excludedportrange protocol=tcp; netsh interface ipv6 show excludedportrange protocol=tcp"));
    console.log(ps("netsh interface ipv4 show dynamicport tcp"));
    console.log(ps("netsh interface ipv6 show prefixpolicies"));
    console.log("chrome", ps(`(Get-Item '${EXE.chrome}').VersionInfo.ProductVersion`), "edge", ps(`(Get-Item '${EXE.msedge}').VersionInfo.ProductVersion`));
    console.log("wsl:", ps("wsl.exe --status 2>&1 | Out-String", 15000).replace(/\u0000/g, "").slice(0, 300));
  },

  // How long a refused loopback connect takes, per family.
  async refused() {
    for (const host of ["::1", "127.0.0.1"]) {
      const t0 = Date.now();
      const err = await new Promise((r) => net.connect({ host, port: 8499 }).on("error", (e) => r(e.code)).on("connect", () => r("connected")));
      record("refused-connect", host, [], { outcome: err, ms: Date.now() - t0 }, "-");
    }
  },

  // Which loopback family each browser tries first for `localhost`, and how
  // long the fallback to the other one takes.
  async family() {
    for (const name of ["chrome", "msedge"]) {
      for (const [label, hosts] of [["both", ["::1", "127.0.0.1"]], ["only-v4", ["127.0.0.1"]], ["only-v6", ["::1"]]]) {
        const ds = [];
        for (const h of hosts) ds.push(await decoy(`[${h}]`, h, 8490, h === "::1"));
        const r = await nav(name, "http://localhost:8490/callback", { timeoutMs: 10000 });
        record(`family ${label}`, name, [], r, "-");
        for (const d of ds) await d.close();
      }
    }
  },

  async baseline() {
    await signIn("baseline deny");
    // Stores a token; never on a developer's machine.
    if (process.env.CI) await signIn("baseline approve", { browsers: ["chrome"], params: { probe_mode: "approve" } });
    await signIn("baseline click", { browsers: ["chrome"], click: true });
    await signIn("public origin", { pub: true });
    await signIn("public origin, 6 s later", { pub: true, params: { probe_delay: "6000" } });
  },

  // Another program already on one loopback family of the CLI's port.
  async decoys() {
    for (const [label, host, v6only] of [
      ["[::1]:8484", "::1", true],
      ["127.0.0.1:8484", "127.0.0.1", false],
      ["[::]:8484 dual-stack", "::", false],
      ["0.0.0.0:8484", "0.0.0.0", false],
    ]) {
      await signIn(`other program on ${label}`, {
        setup: async () => {
          try {
            return { decoys: [await decoy(label, host, 8484, v6only)] };
          } catch (e) {
            console.log(`decoy ${label} failed: ${e.code}`);
            return { decoys: [] };
          }
        },
      });
    }
  },

  // A sign-in from another session (another HOME: another user, sandbox or
  // shell) is still waiting when this one starts.
  async staleSession() {
    for (const name of ["chrome", "msedge"]) {
      await killCli();
      const other = newProject();
      // OLDER_CLI_BIN: the older sign-in is a released CLI, as after an upgrade.
      const first = connect(other, [], null, process.env.OLDER_CLI_BIN ?? CLI);
      const p = newProject();
      const c = connect(p);
      const bind = listeners();
      const r1 = await nav(name, withProbe(c.url, { probe_click: "0" }));
      await sleep(1000);
      record("older sign-in still waiting: 1st approval", name, bind, r1, `this ${workerResult(p)}, older ${workerResult(other)}`);
      const r2 = await nav(name, withProbe(c.url, { probe_click: "0" }));
      await sleep(1000);
      record("older sign-in still waiting: 2nd approval", name, listeners(), r2, `this ${workerResult(p)}, older ${workerResult(other)}`, first.url ? "" : "older connect failed");
    }
  },

  // The agent runs `connect` again with different flags: the first tab is
  // still open and gets approved after the second sign-in started.
  async staleTab() {
    for (const name of ["chrome"]) {
      await killCli();
      const p = newProject();
      const first = connect(p);
      const second = connect(p, ["--replace"]);
      const bind = listeners();
      const r1 = await nav(name, withProbe(first.url, { probe_click: "0" }));
      await sleep(1000);
      record("older tab approved first", name, bind, r1, workerResult(p));
      const r2 = await nav(name, withProbe(second.url, { probe_click: "0" }));
      await sleep(1000);
      record("then the current tab", name, listeners(), r2, workerResult(p), workerLog(p).slice(0, 300));
    }
  },

  // The agent runs `connect` inside a Job Object and ends the job when the
  // command returns.
  async jobs() {
    for (const mode of ["kill-on-close", "codex-terminate", "codex-preserve", "libuv"]) {
      await signIn(`agent job: ${mode}`, {
        browsers: ["chrome"],
        wrap: (cli, args) => ["python", [path.join(HERE, "jobrun.py"), mode, "--", cli, ...args]],
      });
    }
    await signIn("agent: node child_process, node exits", {
      browsers: ["chrome"],
      wrap: (cli, args) => [process.execPath, ["-e", `require('child_process').spawnSync(${JSON.stringify(cli)}, ${JSON.stringify(args)}, {stdio:'inherit'})`]],
    });
  },

  // A connection that never sends a request, open when the browser arrives.
  async idle() {
    await signIn("idle connection open", {
      browsers: ["chrome"],
      timeoutMs: 25000,
      during: (bind) => {
        const m = bind.find((l) => l.includes(":8484"))?.match(/^\[?([^\]\s]+?)\]?:8484/);
        // The CLI may close it first.
        const s = net.connect({ host: m?.[1] ?? "127.0.0.1", port: 8484 }).on("error", () => {});
        setTimeout(() => s.destroy(), 10000);
      },
    });
  },

  // Eight connections that close without a request (port scanners, health
  // checks, an antivirus probing the port).
  async junk() {
    await killCli();
    const p = newProject();
    const c = connect(p);
    const bind = listeners();
    const host = bind.find((l) => l.includes(":8484"))?.match(/^\[?([^\]\s]+?)\]?:8484/)?.[1] ?? "127.0.0.1";
    for (let i = 0; i < 8; i++) {
      await new Promise((r) => net.connect({ host, port: 8484 }, function () { this.end(); r(); }).on("error", r));
      await sleep(50);
    }
    await sleep(500);
    const r = await nav("chrome", withProbe(c.url, { probe_click: "0" }));
    record("8 empty connections, then the browser", "chrome", bind, r, workerResult(p), workerLog(p).slice(0, 300));
  },

  // One family of 8484 reserved (Hyper-V, WSL and Docker reserve port ranges).
  async excluded() {
    for (const fam of ["ipv6", "ipv4"]) {
      await signIn(`8484 reserved for ${fam}`, {
        browsers: ["chrome"],
        setup: async () => {
          console.log(ps(`netsh int ${fam} add excludedportrange protocol=tcp startport=8484 numberofports=1 store=active`));
          return {};
        },
        teardown: async () => {
          console.log(ps(`netsh int ${fam} delete excludedportrange protocol=tcp startport=8484 numberofports=1 store=active`));
        },
      });
    }
  },
};

process.on("unhandledRejection", (e) => console.log("unhandled:", e?.message ?? e));
process.on("uncaughtException", (e) => console.log("uncaught:", e?.message ?? e));

for (const [name, run] of Object.entries(scenarios)) {
  if (FILTER && !FILTER.test(name)) continue;
  console.log(`\n### ${name}`);
  try {
    await within(6 * 60 * 1000, run(), `scenario ${name}`);
  } catch (e) {
    console.log(`scenario ${name} failed: ${e.stack ?? e}`);
  }
}
await killCli();

const table = [
  "| cli | scenario | browser | CLI listening on | browser's redirect | ms | worker | note |",
  "|---|---|---|---|---|---|---|---|",
  ...rows.map((r) => `| ${r.cli} | ${r.scenario} | ${r.browser} | ${r.bind} | ${r.nav} | ${r.ms} | ${r.worker} | ${String(r.note).replace(/\|/g, "/")} |`),
].join("\n");
console.log(`\n${table}`);
if (process.env.GITHUB_STEP_SUMMARY) fs.appendFileSync(process.env.GITHUB_STEP_SUMMARY, `\n## ${LABEL}\n\n${table}\n`);
fs.writeFileSync(path.join(HERE, `results-${LABEL}.json`), JSON.stringify(rows, null, 2));
for (const { b, proc } of Object.values(browsers)) {
  await b.close().catch(() => {});
  proc.kill();
}
process.exit(0);
