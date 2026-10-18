#!/usr/bin/env python3
"""The headless host of a session, with one trust key (plan.md step 7; inputs#I6 Q1).

`live-pty.py` (byte for byte from chunk 2026-10-17-mcp-server-for-drivers), extended by the founder's own answer
«Одна клавиша в стенде»: the drained bytes also feed an in-memory screen (never written anywhere), and the rig may
write ONE Enter into the pty it owns, only when that screen shows the folder-trust dialog with its confirm row
selected. Anything else on screen: no key.

The trust dialog is read as: one row holding the literal `Yes, I trust this folder` (the measured modal signature,
crates/viola-agent-claude/src/screen.rs `SIGNATURES.modals`) with the selection pointer `❯` on that same row, no
row holding `Yes, allow external imports` (the other measured modal), and the output quiet for QUIET seconds. The
read runs only during the first TRUST_WINDOW seconds of the start and only until the key is written; the key is
written at most once. No desktop key, no other byte.

It opens a plain pty itself, sets its size to 210 columns by 45 rows, starts the given command on it as the
session leader, and drains the master. The stop is the form the harness's supervise uses: Ctrl-C written to the
pty, again every 500 ms, until the child exits. It begins when `<private dir>/<label>.stop` appears or the host
gets TERM / INT. A child still alive 10 s into the stop is killed and the status says so.

The command's environment is declared, not inherited: HOME, USER, LOGNAME, SHELL, PATH, LANG, XDG_RUNTIME_DIR and
TERM=xterm-256color. No CLAUDE* and no VIOLA* name crosses.

  live-pty-trust.py <private dir> <label> -- <command> [args...]

Status: `<private dir>/<label>.pty.json`, written at the start, at the key, and at the end (codes and counts only).
Run from the repository root.
"""
import codecs
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
TRUST = "Yes, I trust this folder"
OTHER_MODALS = ("Yes, allow external imports",)
POINTER = "❯"
QUIET = 0.5
TRUST_WINDOW = 45.0


def stamp():
    return datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%S.%f")[:-3] + "Z"


def declared_env():
    env = {k: os.environ[k] for k in KEPT if k in os.environ}
    env["TERM"] = "xterm-256color"
    return env


def write_status(path, doc):
    tmp = path + ".tmp"
    with open(tmp, "w", encoding="utf-8") as f:
        f.write(json.dumps(doc, sort_keys=True) + "\n")
    os.replace(tmp, path)


class Screen:
    """A minimal terminal: printable text, CR / LF / BS / TAB, CSI cursor moves and erases, the alternate screen
    as a clear; colours, modes and OSC strings are read and dropped. A sequence it does not know is dropped."""

    def __init__(self, rows, cols):
        self.rows, self.cols = rows, cols
        self.decoder = codecs.getincrementaldecoder("utf-8")("replace")
        self.state = "ground"
        self.seq = ""
        self.saved = (0, 0)
        self.clear()

    def clear(self):
        self.grid = [[" "] * self.cols for _ in range(self.rows)]
        self.r = self.c = 0

    def lines(self):
        return ["".join(row).rstrip() for row in self.grid]

    def scroll(self):
        self.grid.pop(0)
        self.grid.append([" "] * self.cols)

    def linefeed(self):
        if self.r == self.rows - 1:
            self.scroll()
        else:
            self.r += 1

    def put(self, ch):
        if self.c >= self.cols:
            self.c = 0
            self.linefeed()
        self.grid[self.r][self.c] = ch
        self.c += 1

    def csi(self, body):
        final, params = body[-1], body[:-1]
        if params.startswith("?"):
            if final in "hl" and any(p in ("47", "1047", "1049") for p in params[1:].split(";")):
                self.clear()
            return
        nums = [int(p) if p.isdigit() else 0 for p in params.split(";")] if params else []
        n = (nums[0] if nums else 0) or 1
        if final == "A":
            self.r = max(0, self.r - n)
        elif final == "B":
            self.r = min(self.rows - 1, self.r + n)
        elif final == "C":
            self.c = min(self.cols - 1, self.c + n)
        elif final == "D":
            self.c = max(0, self.c - n)
        elif final == "E":
            self.r, self.c = min(self.rows - 1, self.r + n), 0
        elif final == "F":
            self.r, self.c = max(0, self.r - n), 0
        elif final == "G":
            self.c = min(self.cols - 1, n - 1)
        elif final == "d":
            self.r = min(self.rows - 1, n - 1)
        elif final in "Hf":
            row = (nums[0] if nums else 0) or 1
            col = (nums[1] if len(nums) > 1 else 0) or 1
            self.r, self.c = min(self.rows - 1, row - 1), min(self.cols - 1, col - 1)
        elif final == "J":
            mode = nums[0] if nums else 0
            if mode in (2, 3):
                self.grid = [[" "] * self.cols for _ in range(self.rows)]
            elif mode == 0:
                self.grid[self.r][self.c:] = [" "] * (self.cols - self.c)
                for row in range(self.r + 1, self.rows):
                    self.grid[row] = [" "] * self.cols
            elif mode == 1:
                for row in range(self.r):
                    self.grid[row] = [" "] * self.cols
                self.grid[self.r][: self.c + 1] = [" "] * (self.c + 1)
        elif final == "K":
            mode = nums[0] if nums else 0
            if mode == 0:
                self.grid[self.r][self.c:] = [" "] * (self.cols - self.c)
            elif mode == 1:
                self.grid[self.r][: self.c + 1] = [" "] * (self.c + 1)
            else:
                self.grid[self.r] = [" "] * self.cols

    def feed(self, data):
        for ch in self.decoder.decode(data):
            if self.state == "esc":
                if ch == "[":
                    self.state, self.seq = "csi", ""
                elif ch == "]":
                    self.state = "osc"
                elif ch == "7":
                    self.saved, self.state = (self.r, self.c), "ground"
                elif ch == "8":
                    (self.r, self.c), self.state = self.saved, "ground"
                else:
                    self.state = "ground"
            elif self.state == "csi":
                self.seq += ch
                if "\x40" <= ch <= "\x7e":
                    self.csi(self.seq)
                    self.state = "ground"
                elif len(self.seq) > 64:
                    self.state = "ground"
            elif self.state == "osc":
                if ch == "\x07":
                    self.state = "ground"
                elif ch == "\x1b":
                    self.state = "osc-esc"
            elif self.state == "osc-esc":
                self.state = "ground"
            elif ch == "\x1b":
                self.state = "esc"
            elif ch == "\r":
                self.c = 0
            elif ch == "\n":
                self.linefeed()
            elif ch == "\b":
                self.c = max(0, self.c - 1)
            elif ch == "\t":
                self.c = min(self.cols - 1, (self.c // 8 + 1) * 8)
            elif ch >= " " and ch != "\x7f":
                self.put(ch)


def trust_dialog(lines):
    """`selected` when the trust confirm row is on screen with the pointer on it and no other modal shows;
    `unselected` when the row shows without the pointer; else `absent`."""
    if any(m in line for line in lines for m in OTHER_MODALS):
        return "other-modal"
    rows = [line for line in lines if TRUST in line]
    if not rows:
        return "absent"
    return "selected" if any(POINTER in line for line in rows) else "unselected"


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
        "trust_key": {"typed": False, "last_read": None},
    }
    write_status(status, doc)

    asked = {"signal": None}

    def on_signal(signum, _frame):
        asked["signal"] = signal.Signals(signum).name

    signal.signal(signal.SIGTERM, on_signal)
    signal.signal(signal.SIGINT, on_signal)

    screen = Screen(ROWS, COLS)
    began = time.monotonic()
    last_byte = began
    reading = True
    written = 0
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
            if chunk and reading:
                screen.feed(chunk)
                last_byte = time.monotonic()
        done, st = os.waitpid(pid, os.WNOHANG)
        if done:
            exit_status = st
            break
        now = time.monotonic()
        if reading and stop_began is None and now - last_byte >= QUIET:
            verdict = trust_dialog(screen.lines())
            doc["trust_key"]["last_read"] = verdict
            if verdict == "selected":
                os.write(master, b"\r")
                written += 1
                reading = False
                doc["trust_key"] = {"typed": True, "at": stamp(), "read": verdict, "after_s": round(now - began, 1)}
                write_status(status, doc)
            elif now - began > TRUST_WINDOW:
                reading = False
                doc["trust_key"]["window_closed_at"] = stamp()
                write_status(status, doc)
        if stop_began is None and (asked["signal"] or os.path.exists(stop_file)):
            stop_began = now
            reading = False
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
    os.close(master)

    doc.update(
        {
            "state": "ended",
            "ended_at": stamp(),
            "drained_bytes": drained,
            "written_before_stop": written,
            "ctrl_c_presses": presses,
            "killed": killed,
            "exit_code": os.waitstatus_to_exitcode(exit_status),
            "stopped": stop_began is not None,
        }
    )
    write_status(status, doc)
    print(json.dumps({k: doc.get(k) for k in ("label", "state", "exit_code", "killed", "ctrl_c_presses", "drained_bytes", "stopped", "trust_key")}, sort_keys=True))


if __name__ == "__main__":
    main()
