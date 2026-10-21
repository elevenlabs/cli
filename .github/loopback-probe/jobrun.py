"""Run a command inside a Windows Job Object, the way some coding agents do,
then end the job the way they do once the command returns.

    python jobrun.py <mode> -- <program> [args...]

Modes (flags on the job, then what happens after the command exits):
  kill-on-close    KILL_ON_JOB_CLOSE; the job handle is closed.
  codex-terminate  KILL_ON_JOB_CLOSE | BREAKAWAY_OK; TerminateJobObject (an
                   interrupted or timed-out command).
  codex-preserve   KILL_ON_JOB_CLOSE | BREAKAWAY_OK; kill-on-close cleared,
                   then closed (a command that exited normally).
  libuv            KILL_ON_JOB_CLOSE | BREAKAWAY_OK | SILENT_BREAKAWAY_OK;
                   closed (Node's child_process).
"""

import ctypes
import subprocess
import sys
from ctypes import wintypes

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


def set_flags(job, flags):
    info = EXTENDED()
    info.BasicLimitInformation.LimitFlags = flags
    if not k32.SetInformationJobObject(job, JobObjectExtendedLimitInformation, ctypes.byref(info), ctypes.sizeof(info)):
        raise ctypes.WinError(ctypes.get_last_error())


def main():
    mode = sys.argv[1]
    cmd = sys.argv[sys.argv.index("--") + 1:]
    flags = {
        "kill-on-close": KILL_ON_JOB_CLOSE,
        "codex-terminate": KILL_ON_JOB_CLOSE | BREAKAWAY_OK,
        "codex-preserve": KILL_ON_JOB_CLOSE | BREAKAWAY_OK,
        "libuv": KILL_ON_JOB_CLOSE | BREAKAWAY_OK | SILENT_BREAKAWAY_OK,
    }[mode]
    job = k32.CreateJobObjectW(None, None)
    if not job:
        raise ctypes.WinError(ctypes.get_last_error())
    set_flags(job, flags)
    child = subprocess.Popen(cmd, creationflags=CREATE_SUSPENDED, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if not k32.AssignProcessToJobObject(job, int(child._handle)):
        raise ctypes.WinError(ctypes.get_last_error())
    ntdll.NtResumeProcess(int(child._handle))
    out, err = child.communicate()
    sys.stdout.write(out.decode("utf-8", "replace"))
    sys.stderr.write(err.decode("utf-8", "replace"))
    if mode == "codex-terminate":
        k32.TerminateJobObject(job, 1)
    elif mode == "codex-preserve":
        set_flags(job, BREAKAWAY_OK)
    k32.CloseHandle(job)
    sys.stderr.write(f"jobrun: {mode} done, command exited {child.returncode}\n")


main()
