#!/usr/bin/env python3
"""The live round's rig (plan.md step 18): one print-mode `claude` start whose one MCP server is
`viola-mcp-tap` in front of `viola mcp`.

  live-tap.py setup            make the rig; print its stamp
  live-tap.py start <stamp> <form>   one start (ledgered before it, by live-ledger.py); form 1 ran
                                     start 1, form 2 is the stdin form a second start would take
  live-tap.py read <stamp>     the capture's codes and counts

Run from the repository root. Every child is a direct spawn under a declared environment of eight
names (no `CLAUDE*`, no `VIOLA*`), the stamped CLI's install dir first on `PATH`. Nothing it prints
holds a path, the prompt, an assistant text or a session id: codes, counts, times and hash prefixes.
"""
import datetime
import hashlib
import json
import os
import pathlib
import subprocess
import sys

CLI_VERSION = "2.1.302"
PROMPT = "reply with the single word ok"
START_BOUND_S = 300


def home():
    return pathlib.Path(os.path.expanduser("~"))


def cli_dir():
    return home() / ".local/share/mise/installs/claude" / CLI_VERSION


def declared_env():
    keep = {k: os.environ[k] for k in ("HOME", "LANG", "LOGNAME", "SHELL", "USER", "XDG_RUNTIME_DIR")
            if k in os.environ}
    keep["PATH"] = f"{cli_dir()}:/usr/local/bin:/usr/bin:/bin"
    keep["TERM"] = "xterm-256color"
    return keep


def dirs(stamp):
    root = pathlib.Path.cwd()
    base = root / "target/e2e-home" / f"viola-live-tap-{stamp}"
    priv = root / "target" / f"mcp-live-tap-{stamp}" / "priv"
    return root, base, priv


def now():
    return datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%S.%fZ")[:-4] + "Z"


def sha8(path):
    return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()[:16]


def setup():
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    root, base, priv = dirs(stamp)
    for d in (base, base / "proj", priv):
        d.mkdir(mode=0o700, parents=True, exist_ok=False)
        os.chmod(d, 0o700)
    env = declared_env()
    git = subprocess.run(["/usr/bin/git", "init", "-q"], cwd=base / "proj", env=env)
    top = subprocess.run(["/usr/bin/git", "rev-parse", "--show-toplevel"], cwd=base / "proj",
                         env=env, capture_output=True, text=True)
    own_top = pathlib.Path(top.stdout.strip()).resolve() == (base / "proj").resolve()
    outside = not (base / "proj").resolve().is_relative_to(root.resolve())
    memory = [str(p.relative_to(p.anchor)) for a in [(base / "proj").resolve(), *(base / "proj").resolve().parents]
              for p in (a / "CLAUDE.md", a / ".claude/CLAUDE.md", a / "CLAUDE.local.md") if p.exists()]
    config = {"mcpServers": {"viola": {
        "type": "stdio",
        "command": str(root / "target/debug/viola-mcp-tap"),
        "args": ["record", str(priv / "capture.ndjson"), str(root / "target/debug/viola"),
                 "--home", str(base / "vhome"), "mcp"],
        "env": {},
    }}}
    fd = os.open(priv / "mcp.json", os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
    with os.fdopen(fd, "w") as f:
        json.dump(config, f)
    version = subprocess.run([str(cli_dir() / "claude"), "--version"], env=env, capture_output=True,
                             text=True, timeout=60)
    print(json.dumps({
        "stamp": stamp, "git_init": git.returncode, "toplevel_is_proj": own_top,
        "proj_outside_work_tree": outside, "ancestor_memory_files": len(memory),
        "home_exists": (base / "vhome").exists(), "mcp_json_mode": oct(os.stat(priv / "mcp.json").st_mode & 0o777),
        "cli_version": version.stdout.strip(), "env_names": sorted(env),
        "viola_sha": sha8(root / "target/debug/viola"), "tap_sha": sha8(root / "target/debug/viola-mcp-tap"),
        "prompt_sha": hashlib.sha256(PROMPT.encode()).hexdigest()[:16],
    }))


def start(stamp, form):
    """`form` 1 is start 1's argv (the prompt after `--mcp-config`, which 2.1.302 reads as
    variadic `<configs...>`); `form` 2 gives the prompt on stdin and nothing after the config."""
    root, base, priv = dirs(stamp)
    env = declared_env()
    argv = [str(cli_dir() / "claude"), "-p", "--strict-mcp-config", "--mcp-config", str(priv / "mcp.json")]
    stdin = subprocess.DEVNULL
    if form == "1":
        argv.append(PROMPT)
    began = now()
    try:
        done = subprocess.run(argv, cwd=base / "proj", env=env,
                              input=PROMPT.encode() if form == "2" else None,
                              stdin=None if form == "2" else stdin,
                              capture_output=True, timeout=START_BOUND_S)
        code, timed_out = done.returncode, False
        out, err = done.stdout, done.stderr
    except subprocess.TimeoutExpired as e:
        code, timed_out, out, err = None, True, e.stdout or b"", e.stderr or b""
    ended = now()
    fd = os.open(priv / f"stderr-{form}.txt", os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
    with os.fdopen(fd, "wb") as f:
        f.write(err)
    reply = out.decode("utf-8", "replace").strip()
    print(json.dumps({"form": form, "began": began, "ended": ended, "exit": code, "timed_out": timed_out,
                      "stdout_bytes": len(out), "stderr_bytes": len(err),
                      "stderr_sha": hashlib.sha256(err).hexdigest()[:16],
                      "reply_is_the_word": reply.lower().rstrip(".") == "ok"}))


def read(stamp):
    _, base, priv = dirs(stamp)
    cap = priv / "capture.ndjson"
    lines = cap.read_text().splitlines() if cap.exists() else []
    rows = [json.loads(l) for l in lines]
    seq = []
    for r in rows:
        try:
            frame = json.loads(r["line"])
        except (ValueError, KeyError):
            seq.append((r.get("dir"), "unparsed"))
            continue
        word = frame.get("method") or ("response" if "result" in frame else "error" if "error" in frame else "?")
        seq.append((r.get("dir"), word))
    print(json.dumps({
        "capture_exists": cap.exists(),
        "capture_mode": oct(cap.stat().st_mode & 0o777) if cap.exists() else None,
        "lines": len(lines),
        "by_dir": {d: sum(1 for x, _ in seq if x == d) for d in ("client", "server")},
        "sequence": seq,
        "mcp_log_lines": len((base / "vhome/diagnostics/mcp.ndjson").read_text().splitlines())
        if (base / "vhome/diagnostics/mcp.ndjson").exists() else 0,
    }))


if __name__ == "__main__":
    verb = sys.argv[1] if len(sys.argv) > 1 else ""
    if verb == "setup":
        setup()
    elif verb == "start" and len(sys.argv) == 4 and sys.argv[3] in ("1", "2"):
        start(sys.argv[2], sys.argv[3])
    elif verb == "read" and len(sys.argv) == 3:
        read(sys.argv[2])
    else:
        sys.exit(__doc__)
