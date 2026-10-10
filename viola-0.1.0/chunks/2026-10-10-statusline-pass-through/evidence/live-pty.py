#!/usr/bin/env python3
"""The headless host of a session (plan.md steps 1, 2 and 13; inputs#I4).

It opens a plain pty itself, sets its size to 210 columns by 45 rows, starts the given command on it as the
session leader, and drains the master. The bytes are counted and discarded: no screen is kept. Nothing is
written to the master while the session runs. No compositor, no window, no key of a desktop.

Added in chunk 2026-10-10-statusline-pass-through (the rest is the copy of chunk 2026-10-10-viola-revive): when
`<private dir>/<label>.needles` exists, each of its lines is an ASCII literal, and the status's `needle_counts`
says how many times each one occurred in the drained stream. The stream itself is still discarded.

The stop is the form the harness's supervise uses (crates/viola-e2e/src/harness/supervise.rs, `stop`): Ctrl-C
written to the pty, again every 500 ms, until the child exits. It begins when `<private dir>/<label>.stop`
appears or the host gets TERM / INT. A child still alive 10 s into the stop is killed and the status says so.

The command's environment is declared, not inherited: HOME, USER, LOGNAME, SHELL, PATH, LANG, XDG_RUNTIME_DIR and
TERM=xterm-256color. No CLAUDE* and no VIOLA* name crosses.

  live-pty.py <private dir> <label> -- <command> [args...]

Status: `<private dir>/<label>.pty.json`, written at the start and again at the end (codes and counts only).
Run from the repository root.
"""
import datetime
import errno
import json
import os
import select
import signal
import sys
import termios
import time

COLS, ROWS = 210, 45
CTRL_C_EVERY = 0.5
STOP_BOUND = 10.0
KEPT = ("HOME", "USER", "LOGNAME", "SHELL", "PATH", "LANG", "XDG_RUNTIME_DIR")


def stamp():
    return datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%S.%f")[:-3] + "Z"


def declared_env():
    env = {k: os.environ[k] for k in KEPT if k in os.environ}
    env["TERM"] = "xterm-256color"
    return env


class Needles:
    """Counts of ASCII literals in a stream fed chunk by chunk: a literal split across two chunks still counts."""

    def __init__(self, path):
        self.needles = []
        if os.path.exists(path):
            with open(path, "rb") as f:
                self.needles = [line.strip() for line in f if line.strip()]
        self.counts = {n.decode("ascii"): 0 for n in self.needles}
        self.keep = max((len(n) for n in self.needles), default=1) - 1
        self.tail = b""

    def feed(self, chunk):
        if not self.needles:
            return
        data = self.tail + chunk
        for n in self.needles:
            # only matches that end inside the new chunk: the ones before it were counted already
            start = max(0, len(self.tail) - len(n) + 1)
            self.counts[n.decode("ascii")] += data.count(n, start)
        self.tail = data[-self.keep:] if self.keep else b""


def write_status(path, doc):
    tmp = path + ".tmp"
    with open(tmp, "w", encoding="utf-8") as f:
        f.write(json.dumps(doc, sort_keys=True) + "\n")
    os.replace(tmp, path)


def main():
    if len(sys.argv) < 5 or sys.argv[3] != "--":
        sys.exit(__doc__)
    priv, label, cmd = sys.argv[1], sys.argv[2], sys.argv[4:]
    if not os.path.isdir(priv):
        sys.exit("no private directory")
    status = os.path.join(priv, label + ".pty.json")
    stop_file = os.path.join(priv, label + ".stop")
    if os.path.exists(stop_file):
        sys.exit("a stop request for this label already stands")

    needles = Needles(os.path.join(priv, label + ".needles"))

    master, slave = os.openpty()
    termios.tcsetwinsize(master, (ROWS, COLS))
    env = declared_env()
    pid = os.fork()
    if pid == 0:
        os.close(master)
        os.login_tty(slave)
        try:
            os.execvpe(cmd[0], cmd, env)
        finally:
            os._exit(127)
    os.close(slave)

    doc = {
        "label": label,
        "host_pid": os.getpid(),
        "child_pid": pid,
        "cols": COLS,
        "rows": ROWS,
        "env_names": sorted(env),
        "started_at": stamp(),
        "state": "running",
    }
    write_status(status, doc)

    asked = {"signal": None}

    def on_signal(signum, _frame):
        asked["signal"] = signal.Signals(signum).name

    signal.signal(signal.SIGTERM, on_signal)
    signal.signal(signal.SIGINT, on_signal)

    drained = 0
    presses = 0
    stop_began = None
    last_press = 0.0
    killed = False
    exit_status = None
    while True:
        try:
            ready, _, _ = select.select([master], [], [], 0.1)
        except InterruptedError:
            ready = []
        if ready:
            try:
                chunk = os.read(master, 65536)
            except OSError as e:
                if e.errno != errno.EIO:
                    raise
                chunk = b""
            drained += len(chunk)
            needles.feed(chunk)
        done, st = os.waitpid(pid, os.WNOHANG)
        if done:
            exit_status = st
            break
        now = time.monotonic()
        if stop_began is None and (asked["signal"] or os.path.exists(stop_file)):
            stop_began = now
            doc["stop_cause"] = asked["signal"] or "stop-file"
            doc["stop_began_at"] = stamp()
        if stop_began is not None:
            if now - stop_began > STOP_BOUND and not killed:
                killed = True
                try:
                    os.killpg(pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
            elif not killed and now - last_press >= CTRL_C_EVERY:
                try:
                    os.write(master, b"\x03")
                    presses += 1
                except OSError:
                    pass
                last_press = now

    # what the child's group still wrote after its leader was reaped
    while True:
        ready, _, _ = select.select([master], [], [], 0.2)
        if not ready:
            break
        try:
            chunk = os.read(master, 65536)
        except OSError:
            break
        if not chunk:
            break
        drained += len(chunk)
        needles.feed(chunk)
    os.close(master)

    doc.update(
        {
            "state": "ended",
            "ended_at": stamp(),
            "drained_bytes": drained,
            "written_before_stop": 0,
            "ctrl_c_presses": presses,
            "killed": killed,
            "exit_code": os.waitstatus_to_exitcode(exit_status),
            "stopped": stop_began is not None,
            "needle_counts": needles.counts,
        }
    )
    write_status(status, doc)
    print(json.dumps({k: doc.get(k) for k in ("label", "state", "exit_code", "killed", "ctrl_c_presses", "drained_bytes", "stopped")}, sort_keys=True))


if __name__ == "__main__":
    main()
