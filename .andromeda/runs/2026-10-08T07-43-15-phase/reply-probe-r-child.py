#!/usr/bin/env python3
"""The revision's addition to the terminal-reply probe: the queries whose replies the 2.1.287 CLI's own
reply parser names and the implement run's battery did not send. One query, the terminal's answer recorded.
It runs as the child of `viola run` in a foot window on the chunk's own compositor. No claude, no key.

  reply-probe-r-child.py [--plugin-dir <dir>] <query id> <side file>
"""
import json
import os
import select
import sys
import termios
import time
import tty

ESC = b"\x1b"
QUERIES = {
    "da1": ESC + b"[c",
    "decxcpr": ESC + b"[?6n",
    "xtversion-bare": ESC + b"[>q",
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
