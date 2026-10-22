# clip.py set <text> | get | hold <seconds>
#
# `hold` opens the clipboard and keeps it open, the way a clipboard manager or
# remote desktop does, so no other program can read it until it closes.
import ctypes
import sys
import time
from ctypes import wintypes

user32 = ctypes.WinDLL("user32", use_last_error=True)
kernel32 = ctypes.WinDLL("kernel32", use_last_error=True)
user32.OpenClipboard.argtypes = [wintypes.HWND]
user32.SetClipboardData.argtypes = [wintypes.UINT, wintypes.HANDLE]
user32.SetClipboardData.restype = wintypes.HANDLE
user32.GetClipboardData.argtypes = [wintypes.UINT]
user32.GetClipboardData.restype = wintypes.HANDLE
kernel32.GlobalAlloc.argtypes = [wintypes.UINT, ctypes.c_size_t]
kernel32.GlobalAlloc.restype = wintypes.HGLOBAL
kernel32.GlobalLock.argtypes = [wintypes.HGLOBAL]
kernel32.GlobalLock.restype = ctypes.c_void_p
kernel32.GlobalUnlock.argtypes = [wintypes.HGLOBAL]

CF_UNICODETEXT = 13
GMEM_MOVEABLE = 0x0002


def open_clipboard():
    for _ in range(100):
        if user32.OpenClipboard(None):
            return
        time.sleep(0.05)
    raise SystemExit(f"OpenClipboard failed: {ctypes.get_last_error()}")


if sys.argv[1] == "set":
    data = (sys.argv[2] + "\0").encode("utf-16-le")
    open_clipboard()
    user32.EmptyClipboard()
    handle = kernel32.GlobalAlloc(GMEM_MOVEABLE, len(data))
    ctypes.memmove(kernel32.GlobalLock(handle), data, len(data))
    kernel32.GlobalUnlock(handle)
    user32.SetClipboardData(CF_UNICODETEXT, handle)
    user32.CloseClipboard()
elif sys.argv[1] == "get":
    open_clipboard()
    handle = user32.GetClipboardData(CF_UNICODETEXT)
    if handle:
        print(ctypes.wstring_at(kernel32.GlobalLock(handle)))
        kernel32.GlobalUnlock(handle)
    user32.CloseClipboard()
elif sys.argv[1] == "hold":
    open_clipboard()
    print("held", flush=True)
    time.sleep(float(sys.argv[2]))
    user32.CloseClipboard()
