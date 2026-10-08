#!/usr/bin/env python3
"""The child of the terminal-reply probe (inputs#I15): it sends ONE terminal query and records what the terminal
answered. It runs as the child of `viola run`, in a foot window on the chunk's own compositor, so the terminal's
answer crosses viola's stdin, where the wheel's classifier reads it. No claude, no key.

viola hands its child `--plugin-dir <dir>` first and asks it `--version` once; both are taken here.

  reply-probe-child.py [--plugin-dir <dir>] <query id> <side file>
"""
import json
import os
import select
import sys
import termios
import time
import tty

ESC = b"\x1b"
ST = ESC + b"\\"
QUERIES = {
    "none": b"",
    "da1": ESC + b"[c",
    "da2": ESC + b"[>c",
    "xtversion": ESC + b"[>0q",
    "kitty-flags": ESC + b"[?u",
    "decrqm-2026": ESC + b"[?2026$p",
    "cpr": ESC + b"[6n",
    "dsr": ESC + b"[5n",
    "osc10": ESC + b"]10;?" + ST,
    "osc11": ESC + b"]11;?" + ST,
    "osc4": ESC + b"]4;0;?" + ST,
    "theme-996": ESC + b"[?996n",
    "theme-2031": ESC + b"[?2031h",
    "winops-14": ESC + b"[14t",
    "winops-16": ESC + b"[16t",
    "winops-18": ESC + b"[18t",
    "inband-resize-2048": ESC + b"[?2048h",
    "modify-other-keys": ESC + b"[?4m",
    "xtgettcap-tn": ESC + b"P+q544e" + ST,
    "xtgettcap-rgb": ESC + b"P+q524742" + ST,
    "decrqss-cursor": ESC + b"P$q q" + ST,
    "focus-1004": ESC + b"[?1004h",
    "kitty-graphics": ESC + b"_Gi=31,s=1,v=1,a=q,t=d,f=24;AAAA" + ST,
}


def main():
    args = sys.argv[1:]
    if "--version" in args:
        print("0.0.0 (reply probe)")
        return 0
    if len(args) >= 2 and args[0] == "--plugin-dir":
        args = args[2:]
    if len(args) != 2 or args[0] not in QUERIES:
        return 2
    qid, side = args
    query = QUERIES[qid]
    fd = sys.stdin.fileno()
    old = termios.tcgetattr(fd)
    reply = b""
    try:
        tty.setraw(fd)
        time.sleep(0.5)
        os.write(sys.stdout.fileno(), query)
        end = time.monotonic() + 1.5
        while True:
            left = end - time.monotonic()
            if left <= 0:
                break
            ready, _, _ = select.select([fd], [], [], left)
            if ready:
                chunk = os.read(fd, 4096)
                if not chunk:
                    break
                reply += chunk
    finally:
        termios.tcsetattr(fd, termios.TCSADRAIN, old)
    with open(side, "a", encoding="utf-8") as f:
        f.write(json.dumps({"id": qid, "query_hex": query.hex(), "reply_hex": reply.hex(), "reply_bytes": len(reply)}) + "\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
