#!/usr/bin/env python3
"""The unstamped first-start race, read through the harness (plan step 3 / step 8).

Run from the repository root:  python3 viola-0.1.0/chunks/<marker>/evidence/repro-unstamped.py <label>

Rounds of 6 concurrent `viola-harness boot --unstamped` sessions, spawned directly from
`target/harness/debug` (no shell, never a piped boot: each boot's stdout goes to a file), each fake
agent given `--receipt`, each session cleaned up through the harness. Two inputs: the default
`overseer` + `builder` pair, then `builder` alone (the control). Every child has a kill deadline,
and every session this script booted is cleaned up on any exit, an interrupt included.
"""

import json
import os
import shutil
import signal
import subprocess
import sys
import tempfile
import time

ROUNDS = 4
SESSIONS = 6
BUILD_DEADLINE_S = 900
BOOT_DEADLINE_S = 120
CLEANUP_DEADLINE_S = 60

ROOT = os.getcwd()
HARNESS = os.path.join(ROOT, "target", "harness", "debug", "viola-harness")
INPUTS = [("pair", ["overseer", "builder"]), ("builder-alone", ["builder"])]

scratch = tempfile.mkdtemp(prefix="viola-race-")
booted = []


def bounded(argv, deadline, stdout, env=None):
    """Runs argv with stdout to a file and a kill deadline; returns (exit code or None, seconds)."""
    t0 = time.monotonic()
    p = subprocess.Popen(argv, cwd=ROOT, stdin=subprocess.DEVNULL, stdout=stdout,
                         stderr=subprocess.DEVNULL, env=env, start_new_session=True)
    try:
        code = p.wait(timeout=deadline)
    except subprocess.TimeoutExpired:
        os.killpg(p.pid, signal.SIGTERM)
        try:
            p.wait(timeout=5)
        except subprocess.TimeoutExpired:
            os.killpg(p.pid, signal.SIGKILL)
            p.wait()
        code = None
    return code, round(time.monotonic() - t0, 2)


def cleanup(session):
    with open(os.path.join(scratch, f"cleanup-{session}.json"), "wb") as out:
        code, _ = bounded([HARNESS, "cleanup", "--session", session], CLEANUP_DEADLINE_S, out)
    with open(os.path.join(scratch, f"cleanup-{session}.json"), "rb") as f:
        text = f.read().decode(errors="replace").strip()
    try:
        doc = json.loads(text)
    except ValueError:
        doc = {"unparsed": True}
    return code, doc


def teardown():
    for s in list(booted):
        cleanup(s)
        booted.remove(s)
    shutil.rmtree(scratch, ignore_errors=True)


def on_signal(signum, _frame):
    teardown()
    sys.exit(128 + signum)


signal.signal(signal.SIGINT, on_signal)
signal.signal(signal.SIGTERM, on_signal)


def receipts(path):
    if not os.path.exists(path):
        return []
    rows = []
    with open(path, "rb") as f:
        for line in f.read().decode(errors="replace").splitlines():
            try:
                rows.append(json.loads(line))
            except ValueError:
                pass
    return [r for r in rows if r.get("kind") == "hook" and r.get("event") == "SessionStart"]


def one_round(label, names, rnd):
    procs = []
    for n in range(1, SESSIONS + 1):
        session = f"race-{label}-r{rnd}-s{n}-{os.getpid()}"
        args = [HARNESS, "boot", "--session", session, "--unstamped"]
        for name in names:
            receipt = os.path.join(scratch, f"{session}-{name}.ndjson")
            args += ["--instance", f"{name}:--receipt {receipt}"]
        out = open(os.path.join(scratch, f"boot-{session}.json"), "wb")
        p = subprocess.Popen(args, cwd=ROOT, stdin=subprocess.DEVNULL, stdout=out,
                             stderr=subprocess.DEVNULL, start_new_session=True)
        booted.append(session)
        procs.append((session, p, out, time.monotonic()))
    results = []
    for session, p, out, t0 in procs:
        left = max(1.0, BOOT_DEADLINE_S - (time.monotonic() - t0))
        try:
            code = p.wait(timeout=left)
        except subprocess.TimeoutExpired:
            os.killpg(p.pid, signal.SIGTERM)
            try:
                p.wait(timeout=5)
            except subprocess.TimeoutExpired:
                os.killpg(p.pid, signal.SIGKILL)
                p.wait()
            code = None
        out.close()
        with open(os.path.join(scratch, f"boot-{session}.json"), "rb") as f:
            text = f.read().decode(errors="replace").strip()
        try:
            doc = json.loads(text.splitlines()[-1]) if text else {}
        except ValueError:
            doc = {"unparsed": True}
        hooks = {}
        for name in names:
            hooks[name] = [
                {"ran": r.get("ran"), "exit_code": r.get("exit_code"),
                 "spawn_os_error": r.get("spawn_os_error")}
                for r in receipts(os.path.join(scratch, f"{session}-{name}.ndjson"))
            ]
        results.append({"session": session, "exit": code, "ok": doc.get("ok"),
                        "reason": doc.get("reason"), "instance": doc.get("instance"),
                        "missing": doc.get("missing"), "session_start_hooks": hooks})
    for r in results:
        ccode, cdoc = cleanup(r["session"])
        booted.remove(r["session"])
        r["cleanup"] = {"exit": ccode, "ok": cdoc.get("ok"),
                        "processes_gone": cdoc.get("processes_gone"),
                        "endpoint_gone": cdoc.get("endpoint_gone")}
    return results


def main():
    label = sys.argv[1] if len(sys.argv) > 1 else "run"
    env = dict(os.environ, CARGO_TARGET_DIR="target/harness")
    with open(os.path.join(scratch, "build.log"), "wb") as out:
        code, secs = bounded(["cargo", "build", "--workspace", "--features", "viola/fake-agent"],
                             BUILD_DEADLINE_S, out, env)
    print(json.dumps({"build_exit": code, "build_s": secs}))
    if code != 0:
        return 1
    for input_label, names in INPUTS:
        sessions = failed = 0
        for rnd in range(1, ROUNDS + 1):
            t0 = time.monotonic()
            for r in one_round(input_label, names, rnd):
                sessions += 1
                if r["ok"] is not True:
                    failed += 1
                r["round"] = rnd
                print(json.dumps({"label": label, "input": input_label, **r}))
            print(json.dumps({"label": label, "input": input_label, "round": rnd,
                              "round_s": round(time.monotonic() - t0, 2)}))
        print(json.dumps({"label": label, "input": input_label, "sessions": sessions,
                          "failed": failed}))
    return 0


try:
    rc = main()
finally:
    teardown()
sys.exit(rc)
