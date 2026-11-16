"""Temporary probe: on Windows, which sign-in still answers the browser's
callback when a coding agent runs the command the way agents do?

Two sign-ins from the published CLI:
  worker      `onboard connect`: hands the sign-in to a detached worker, waits
              25 s, then returns.
  worker-nb   `onboard connect --no-browser`: returns as soon as the link is known.
  foreground  `auth login`: listens in the command itself until approved.

Each runs under a model of how an agent runs a command (see HARNESSES), and at
a set time after the command starts the probe sends the callback the approval
page would send for Deny. "answered" means a sign-in was still listening.

    python probe.py <path to elevenlabs.exe>
"""

import ctypes
import glob
import json
import os
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
import time
import urllib.error
import urllib.parse
import urllib.request
from ctypes import wintypes

CLI = sys.argv[1]
CLIS = sys.argv[1:]
HOME = os.environ["USERPROFILE"]
SESSIONS = os.path.join(HOME, ".elevenlabs", "onboard")

k32 = ctypes.WinDLL("kernel32", use_last_error=True)
ntdll = ctypes.WinDLL("ntdll")

CREATE_SUSPENDED = 0x4
KILL_ON_JOB_CLOSE = 0x2000
BREAKAWAY_OK = 0x800
SILENT_BREAKAWAY_OK = 0x1000
JobObjectExtendedLimitInformation = 9


class IO_COUNTERS(ctypes.Structure):
    _fields_ = [(n, ctypes.c_ulonglong) for n in (
        "Read", "Write", "Other", "ReadBytes", "WriteBytes", "OtherBytes")]


class BASIC(ctypes.Structure):
    _fields_ = [
        ("PerProcessUserTimeLimit", ctypes.c_int64),
        ("PerJobUserTimeLimit", ctypes.c_int64),
        ("LimitFlags", wintypes.DWORD),
        ("MinimumWorkingSetSize", ctypes.c_size_t),
        ("MaximumWorkingSetSize", ctypes.c_size_t),
        ("ActiveProcessLimit", wintypes.DWORD),
        ("Affinity", ctypes.c_size_t),
        ("PriorityClass", wintypes.DWORD),
        ("SchedulingClass", wintypes.DWORD),
    ]


class EXTENDED(ctypes.Structure):
    _fields_ = [
        ("BasicLimitInformation", BASIC),
        ("IoInfo", IO_COUNTERS),
        ("ProcessMemoryLimit", ctypes.c_size_t),
        ("JobMemoryLimit", ctypes.c_size_t),
        ("PeakProcessMemoryUsed", ctypes.c_size_t),
        ("PeakJobMemoryUsed", ctypes.c_size_t),
    ]


k32.CreateJobObjectW.restype = wintypes.HANDLE
k32.CreateJobObjectW.argtypes = [wintypes.LPVOID, wintypes.LPCWSTR]
k32.SetInformationJobObject.argtypes = [wintypes.HANDLE, ctypes.c_int, wintypes.LPVOID, wintypes.DWORD]
k32.AssignProcessToJobObject.argtypes = [wintypes.HANDLE, wintypes.HANDLE]
k32.TerminateJobObject.argtypes = [wintypes.HANDLE, wintypes.UINT]
k32.CloseHandle.argtypes = [wintypes.HANDLE]
ntdll.NtResumeProcess.argtypes = [wintypes.HANDLE]

# How an agent runs a command: the job it puts it in (None: no job), how long
# it lets it run (None: until it exits), and how it ends the job afterwards.
HARNESSES = [
    ("no job, runs to the end", None, None, None),
    ("no job, tree killed at 10 s", None, 10, None),
    ("job kill-on-close, runs to the end", KILL_ON_JOB_CLOSE, None, "close"),
    ("job kill-on-close, terminated at 10 s", KILL_ON_JOB_CLOSE, 10, "terminate"),
    ("job kill-on-close + breakaway ok, terminated at the end", KILL_ON_JOB_CLOSE | BREAKAWAY_OK, None, "terminate"),
    ("job kill-on-close + breakaway ok, terminated at 10 s", KILL_ON_JOB_CLOSE | BREAKAWAY_OK, 10, "terminate"),
    ("job kill-on-close + silent breakaway (Node), runs to the end",
     KILL_ON_JOB_CLOSE | BREAKAWAY_OK | SILENT_BREAKAWAY_OK, None, "close"),
]
DESIGNS = ["worker", "worker-nb", "foreground"]


def make_job(flags):
    job = k32.CreateJobObjectW(None, None)
    if not job:
        raise ctypes.WinError(ctypes.get_last_error())
    info = EXTENDED()
    info.BasicLimitInformation.LimitFlags = flags
    if not k32.SetInformationJobObject(job, JobObjectExtendedLimitInformation, ctypes.byref(info), ctypes.sizeof(info)):
        raise ctypes.WinError(ctypes.get_last_error())
    return job


def quiet(cmd):
    subprocess.run(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)


def clean():
    quiet(["taskkill", "/F", "/IM", "elevenlabs.exe"])
    for browser in ("msedge.exe", "chrome.exe", "firefox.exe"):
        quiet(["taskkill", "/F", "/IM", browser])
    time.sleep(1)
    shutil.rmtree(SESSIONS, ignore_errors=True)


def cli_alive():
    out = subprocess.run(["tasklist", "/FI", "IMAGENAME eq elevenlabs.exe", "/FO", "CSV", "/NH"],
                         capture_output=True, text=True).stdout
    return sum(1 for line in out.splitlines() if line.lower().startswith('"elevenlabs.exe"'))


def find_url(err_path):
    paths = glob.glob(os.path.join(SESSIONS, "*", "worker.log")) + [err_path]
    for path in paths:
        try:
            with open(path, "r", encoding="utf-8", errors="replace") as f:
                text = f.read()
        except OSError:
            continue
        found = re.search(r"URL:\s*(https?://\S+)", text) or re.search(r"(https?://\S*redirect_uri=\S+)", text)
        if found:
            return found.group(1)
    return None


class NoRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, *args, **kwargs):
        return None


def send_callback(url):
    query = urllib.parse.parse_qs(urllib.parse.urlparse(url).query)
    callback = query["redirect_uri"][0]
    state = query["state"][0]
    target = f"{callback}?error=access_denied&state={urllib.parse.quote(state)}"
    opener = urllib.request.build_opener(NoRedirect)
    try:
        opener.open(target, timeout=8)
        return "answered 200"
    except urllib.error.HTTPError as e:
        return f"answered {e.code}"
    except Exception as e:  # refused, timed out, reset
        reason = getattr(e, "reason", e)
        return f"NOT answered ({type(reason).__name__}: {reason})"[:90]


def connect_args(project):
    """`onboard init` says what to run next; take its arguments."""
    done = subprocess.run([CLI, "onboard", "init"], cwd=project, capture_output=True, text=True)
    lines = [l for l in done.stdout.splitlines() if l.strip().startswith("{")]
    if not lines:
        raise SystemExit(f"onboard init gave no reply: {done.stdout!r} {done.stderr!r}")
    reply = json.loads(lines[-1])
    if reply.get("next") != "run":
        raise SystemExit(f"onboard init didn't say to run connect: {reply}")
    words = shlex.split(reply["run"])
    return words[words.index("onboard"):]


def run(design, harness, at):
    name, flags, limit, ending = harness
    clean()
    project = tempfile.mkdtemp(prefix="probe-")
    with open(os.path.join(project, "package.json"), "w") as f:
        f.write('{"name":"probe","version":"1.0.0"}')
    if design == "foreground":
        args = ["auth", "login"]
    else:
        args = connect_args(project)
        if design == "worker-nb":
            args.append("--no-browser")
    out_path = os.path.join(project, "out.txt")
    err_path = os.path.join(project, "err.txt")
    job = make_job(flags) if flags is not None else None
    with open(out_path, "wb") as out, open(err_path, "wb") as err:
        child = subprocess.Popen([CLI] + args, cwd=project, stdin=subprocess.DEVNULL, stdout=out, stderr=err,
                                 creationflags=CREATE_SUSPENDED)
        if job and not k32.AssignProcessToJobObject(job, int(child._handle)):
            raise ctypes.WinError(ctypes.get_last_error())
        ntdll.NtResumeProcess(int(child._handle))
        started = time.monotonic()
        ended_at = None
        ended_how = None
        result = None
        sent_at = None
        link_at = None
        running_at_send = None
        alive_at_send = None

        def end_command():
            nonlocal job
            if job:
                if ending == "terminate":
                    k32.TerminateJobObject(job, 1)
                k32.CloseHandle(job)
                job = None

        while True:
            now = time.monotonic() - started
            if ended_at is None and child.poll() is not None:
                ended_at, ended_how = now, f"exited {child.returncode}"
                end_command()
            if ended_at is None and limit is not None and now >= limit:
                ended_at, ended_how = now, "stopped by the agent"
                if job:
                    k32.TerminateJobObject(job, 1)
                    k32.CloseHandle(job)
                    job = None
                else:
                    quiet(["taskkill", "/T", "/F", "/PID", str(child.pid)])
            if link_at is None and find_url(err_path):
                link_at = now
            if result is None and now >= at:
                url = find_url(err_path)
                if url:
                    running_at_send = ended_at is None
                    alive_at_send = cli_alive()
                    sent_at = now
                    result = send_callback(url)
                elif now >= at + 20:
                    sent_at = now
                    result = "no approval link appeared"
            if result is not None and (ended_at is not None or now >= at + 15):
                break
            if now > 120:
                break
            time.sleep(0.2)
        if ended_at is None:
            quiet(["taskkill", "/T", "/F", "/PID", str(child.pid)])
            ended_how = "still running when the probe moved on"
        end_command()
    row = {
        "design": design,
        "harness": name,
        "callback_at_s": at,
        "link_known_at_s": None if link_at is None else round(link_at, 1),
        "callback_sent_at_s": None if sent_at is None else round(sent_at, 1),
        "command_running_then": running_at_send,
        "cli_processes_then": alive_at_send,
        "command_ended": ended_how if ended_at is None else f"{ended_how} at {ended_at:.0f} s",
        "result": result,
    }
    print(json.dumps(row), flush=True)
    if result and not result.startswith("answered"):
        for path in [err_path] + glob.glob(os.path.join(SESSIONS, "*", "worker.log")):
            try:
                with open(path, "r", encoding="utf-8", errors="replace") as f:
                    tail = f.read()[-600:]
                print(f"  --- {os.path.basename(path)}: {tail!r}", flush=True)
            except OSError:
                pass
    shutil.rmtree(project, ignore_errors=True)
    return row


def main():
    global CLI
    tables = []
    for cli in CLIS:
        CLI = cli
        tables.append(one_cli())
    summary = os.environ.get("GITHUB_STEP_SUMMARY")
    if summary:
        with open(summary, "a", encoding="utf-8") as f:
            f.write("\n\n".join(tables) + "\n")


def one_cli():
    version = subprocess.run([CLI, "--version"], capture_output=True, text=True).stdout.strip()
    title = f"### {version} ({CLI})"
    print(title, flush=True)
    rows = []
    for harness in HARNESSES:
        times = [3, 15] if harness[2] is not None else [3, 40]
        for design in DESIGNS:
            for at in times:
                try:
                    rows.append(run(design, harness, at))
                except SystemExit:
                    raise
                except Exception as e:
                    print(f"run failed: {design} / {harness[0]} / {at}: {e!r}", flush=True)
    clean()
    lines = [title, "",
             "| How the agent runs the command | Callback sent | worker | worker, --no-browser | foreground (auth login) |",
             "|---|---|---|---|---|"]
    seen = []
    for row in rows:
        key = (row["harness"], row["callback_at_s"])
        if key not in seen:
            seen.append(key)
    for harness, at in seen:
        cells = []
        for design in DESIGNS:
            hit = [r for r in rows if r["harness"] == harness and r["callback_at_s"] == at and r["design"] == design]
            cells.append(f"{hit[0]['result']}; command {hit[0]['command_ended']}" if hit else "not run")
        lines.append(f"| {harness} | {at} s | " + " | ".join(cells) + " |")
    table = "\n".join(lines)
    print(table, flush=True)
    return table


main()
