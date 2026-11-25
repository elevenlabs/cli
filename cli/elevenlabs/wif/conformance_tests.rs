//! Runs the shared WIF conformance scenarios against this provider, in-process.
//!
//! Needs the conformance repo: WIF_CONFORMANCE_DIR (default ../wif-conformance) with Go installed,
//! or WIF_CONFORMANCE_URL pointing at a running fake server plus WIF_CONFORMANCE_DIR for the scenarios.
//! Skips (and says so) when neither is available.

use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

use serde_json::{json, Value};

use super::*;

const CLOCK_START: f64 = 1000.0;

/// Scenarios the CLI cannot pass yet, and why.
const UNSUPPORTED: &[(&str, &str)] = &[
    (
        "E7",
        "no hook to repeat a request after a 401 (needs a Fern CLI runtime change)",
    ),
    (
        "E8",
        "no hook to repeat a request after a 401 (needs a Fern CLI runtime change)",
    ),
    (
        "E9",
        "no hook to repeat a request after a 401 (needs a Fern CLI runtime change)",
    ),
    ("W1", "the CLI opens no WebSocket connections"),
];

const ENV_TO_CLEAR: &[&str] = &[
    API_KEY_ENV,
    CONFIG_ENV,
    clock::TEST_CLOCK_ENV,
    "ACTIONS_ID_TOKEN_REQUEST_URL",
    "ACTIONS_ID_TOKEN_REQUEST_TOKEN",
    "GCE_METADATA_HOST",
    "AZURE_POD_IDENTITY_AUTHORITY_HOST",
    "AWS_REGION",
    "AWS_DEFAULT_REGION",
    "AWS_ACCESS_KEY_ID",
    "AWS_SECRET_ACCESS_KEY",
    "AWS_SESSION_TOKEN",
    "AWS_ENDPOINT_URL_STS",
    "AWS_EC2_METADATA_SERVICE_ENDPOINT",
    "AWS_CONTAINER_CREDENTIALS_FULL_URI",
    "AWS_CONTAINER_CREDENTIALS_RELATIVE_URI",
    "AWS_CONTAINER_AUTHORIZATION_TOKEN",
];

fn conformance_dir() -> PathBuf {
    std::env::var_os("WIF_CONFORMANCE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../wif-conformance"))
}

struct Server {
    url: String,
    child: Option<Child>,
    _bin_dir: Option<tempfile::TempDir>,
}

impl Drop for Server {
    fn drop(&mut self) {
        if let Some(child) = &mut self.child {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

fn start_server(dir: &Path) -> Option<Server> {
    if let Ok(url) = std::env::var("WIF_CONFORMANCE_URL") {
        return Some(Server {
            url,
            child: None,
            _bin_dir: None,
        });
    }
    // Build, then run the binary itself: killing `go run` would leave the server it spawned running.
    let bin_dir = tempfile::tempdir().ok()?;
    let bin = bin_dir.path().join("wif-fake-server");
    let built = Command::new("go")
        .args(["build", "-o"])
        .arg(&bin)
        .arg("./cmd/wif-fake-server")
        .current_dir(dir)
        .status()
        .ok()?;
    if !built.success() {
        return None;
    }
    let mut child = Command::new(&bin).stdout(Stdio::piped()).spawn().ok()?;
    let mut line = String::new();
    std::io::BufReader::new(child.stdout.take()?)
        .read_line(&mut line)
        .ok()?;
    let url = serde_json::from_str::<Value>(&line).ok()?["listening"]
        .as_str()?
        .to_string();
    Some(Server {
        url,
        child: Some(child),
        _bin_dir: Some(bin_dir),
    })
}

#[test]
fn conformance() {
    let dir = conformance_dir();
    let Ok(entries) = std::fs::read_dir(dir.join("scenarios")) else {
        eprintln!(
            "skipping WIF conformance: no scenarios at {}",
            dir.display()
        );
        return;
    };
    let Some(server) = start_server(&dir) else {
        eprintln!("skipping WIF conformance: cannot start the fake server (is Go installed?)");
        return;
    };
    let mut paths: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    paths.sort();
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let http = reqwest::Client::new();

    let mut failures = Vec::new();
    for path in paths {
        let scenario: Value =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        let id = scenario["id"].as_str().unwrap_or_default().to_string();
        if let Some((_, why)) = UNSUPPORTED.iter().find(|(u, _)| *u == id) {
            eprintln!("{id}: skipped ({why})");
            continue;
        }
        let result = run_scenario(&scenario, &server.url, &rt, &http);
        for name in ENV_TO_CLEAR {
            std::env::remove_var(name);
        }
        match result {
            Ok(()) => eprintln!("{id}: ok"),
            Err(e) => failures.push(format!("{id}: {e}")),
        }
    }
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
}

fn run_scenario(
    scenario: &Value,
    server: &str,
    rt: &tokio::runtime::Runtime,
    http: &reqwest::Client,
) -> Result<(), String> {
    let driver = scenario.get("driver").cloned().unwrap_or(json!({}));
    let mut server_half = scenario.clone();
    server_half.as_object_mut().unwrap().remove("driver");
    let setup: Value = rt
        .block_on(async {
            http.post(format!("{server}/_control/reset"))
                .json(&server_half)
                .send()
                .await?
                .error_for_status()?
                .json()
                .await
        })
        .map_err(|e| format!("reset failed: {e}"))?;

    for name in ENV_TO_CLEAR {
        std::env::remove_var(name);
    }
    for (layer, vars) in [("server", &setup["env"]), ("driver", &driver["env"])] {
        for (k, v) in vars.as_object().into_iter().flatten() {
            let v = v
                .as_str()
                .ok_or(format!("{layer} env {k} is not a string"))?;
            std::env::set_var(k, v);
        }
    }

    let tmp = tempfile::tempdir().unwrap();
    let clock_path = tmp.path().join("clock");
    let mut now = CLOCK_START;
    write_clock(&clock_path, now);
    std::env::set_var(clock::TEST_CLOCK_ENV, &clock_path);
    let config_path = tmp.path().join("wif.json");
    std::fs::write(
        &config_path,
        config_json(scenario, &setup, tmp.path()).to_string(),
    )
    .unwrap();

    // The CLI's argv for this scenario, then the same precedence and env handling as `register`.
    let mut args = vec!["elevenlabs".to_string()];
    if driver["client"] == "api_key_arg" {
        args.extend([
            "--api-key".into(),
            driver["api_key"].as_str().unwrap_or_default().into(),
        ]);
    } else if driver["env_config"] == true {
        std::env::set_var(CONFIG_ENV, &config_path);
    } else {
        args.extend(["--wif-config".into(), config_path.display().to_string()]);
    }
    let argv: Vec<OsString> = args.iter().map(OsString::from).collect();
    let process_env = |name: &str| std::env::var(name).ok().filter(|v| !v.is_empty());
    let auth = match select(&argv, &process_env).map_err(|e| e.message)? {
        Selection::Wif { path, explicit } => {
            if explicit {
                shadow_env_api_key();
            }
            Some(WifAuth::new(path))
        }
        Selection::None => None,
    };
    // What the CLI's xi-api-key global parameter sends: the flag, else the env var.
    let api_key = flag_value(&argv, "--api-key").or_else(|| process_env(API_KEY_ENV));
    let base_url = setup["base_url"]
        .as_str()
        .ok_or("setup has no base_url")?
        .trim_end_matches('/')
        .to_string();

    let mut messages: Vec<String> = Vec::new();
    for step in driver["steps"].as_array().into_iter().flatten() {
        if let Some(seconds) = step["advance_clock"].as_f64() {
            now += seconds;
            write_clock(&clock_path, now);
            continue;
        }
        let call = step["call"]
            .as_str()
            .ok_or(format!("unsupported step {step}"))?;
        let (method, path) = call.split_once(' ').ok_or(format!("bad call {call}"))?;
        let expected = step.get("expect").cloned().unwrap_or(json!("ok"));
        let n = step["parallel"].as_u64().unwrap_or(1) as usize;
        let outcomes: Vec<(Value, Option<String>)> = std::thread::scope(|s| {
            let handles: Vec<_> = (0..n)
                .map(|_| {
                    s.spawn(|| {
                        let mut request = http
                            .request(
                                reqwest::Method::from_bytes(method.as_bytes()).unwrap(),
                                format!("{base_url}{path}"),
                            )
                            .build()
                            .unwrap();
                        if let Some(key) = &api_key {
                            request
                                .headers_mut()
                                .insert("xi-api-key", key.parse().unwrap());
                        } else if let Some(auth) = &auth {
                            // The CLI consults the auth provider only when no API key header is set.
                            let token = auth
                                .token_for(request.url().as_str())
                                .and_then(|t| set_bearer(request.headers_mut(), &t).map(|_| t));
                            if let Err(e) = token {
                                return (json!(e.category), Some(e.message));
                            }
                        }
                        match rt.block_on(http.execute(request)) {
                            Ok(r) if r.status().as_u16() < 300 => (json!("ok"), None),
                            Ok(r) => (json!(r.status().as_u16()), None),
                            Err(e) => (json!(format!("transport error: {e}")), None),
                        }
                    })
                })
                .collect();
            handles.into_iter().map(|h| h.join().unwrap()).collect()
        });
        let got: Vec<Value> = outcomes.iter().map(|(o, _)| o.clone()).collect();
        messages.extend(outcomes.into_iter().filter_map(|(_, m)| m));
        if got.iter().any(|o| *o != expected) {
            return Err(format!("step {step}: got {got:?}"));
        }
    }

    let report: Value = rt
        .block_on(async {
            http.get(format!("{server}/_control/report"))
                .send()
                .await?
                .json()
                .await
        })
        .map_err(|e| format!("report failed: {e}"))?;
    if report["passed"] != true {
        return Err(format!("server checks failed: {}", report["failures"]));
    }
    let visible = messages.join("\n");
    let leaked = report["secrets"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .filter(|s| visible.contains(s))
        .count();
    // A count only: the failure message must not repeat the leaked values.
    if leaked > 0 {
        return Err(format!("{leaked} credential(s) leaked into error messages"));
    }
    Ok(())
}

fn config_json(scenario: &Value, setup: &Value, dir: &Path) -> Value {
    let source = &scenario["source"];
    let token_source = match source["type"].as_str().unwrap_or_default() {
        "static" => {
            std::fs::write(
                dir.join("source-token"),
                setup["source_token"].as_str().unwrap_or_default(),
            )
            .unwrap();
            json!({"type": "file", "path": "source-token"})
        }
        kind => {
            let mut s = json!({"type": if kind == "github" { "github_actions" } else { kind }});
            if let Some(audience) = source.get("audience").filter(|a| !a.is_null()) {
                s["audience"] = audience.clone();
            }
            s
        }
    };
    let mut config = json!({
        "type": "elevenlabs_workload_identity_federation",
        "version": 1,
        "pool": scenario["pool"],
        "resource": scenario["resource"],
        "token_source": token_source,
    });
    if let Some(scope) = scenario
        .get("scope")
        .filter(|s| s.as_str().is_some_and(|s| !s.is_empty()))
    {
        config["scope"] = scope.clone();
    }
    config
}

fn write_clock(path: &Path, value: f64) {
    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, value.to_string()).unwrap();
    std::fs::rename(&tmp, path).unwrap();
}
