#!/usr/bin/env python3
"""The reader of a headless session's records (plan.md steps 1, 2 and 13).

Every line it prints or appends holds codes, counts, times and equalities only: never a prompt text, an assistant
text or an event payload text. The sent text is read from a file in the private directory, outside the tree, and
only compared.

  live-read.py await-start <home> <name> [bound seconds]
  live-read.py hold <home> <name> <seconds>
  live-read.py reading <id> <session> <home> <name> <private dir> <text file> [--settle-s N] [--append <file>]
  live-read.py census <private dir> <label> <wrapper pid> <child pid>

Run from the repository root.
"""
import json
import os
import sys
import time

START = ["wheel:start", "budget-gate", "session-start:startup"]


def lines_from(path, offset=0):
    try:
        with open(path, "rb") as f:
            f.seek(offset)
            raw = f.read()
    except OSError:
        return []
    out = []
    for ln in raw.split(b"\n"):
        if not ln.strip():
            continue
        try:
            out.append(json.loads(ln))
        except ValueError:
            out.append({"kind": "torn"})
    return out


def code(rec):
    kind = rec.get("kind")
    data = rec.get("data") if isinstance(rec.get("data"), dict) else {}
    if kind in ("wheel", "session-start") and isinstance(data.get("cause"), str):
        return "%s:%s" % (kind, data["cause"])
    return kind


def alive(pid):
    return isinstance(pid, int) and os.path.isdir("/proc/%d" % pid)


def snapshot(home, name):
    try:
        with open(os.path.join(home, "instances", name, "snapshot.json"), encoding="utf-8") as f:
            d = json.load(f).get("data") or {}
    except (OSError, ValueError):
        return {}
    return d


def snap_codes(home, name):
    d = snapshot(home, name)
    return {
        "snapshot": bool(d),
        "cli_version": d.get("cli_version"),
        "cli_verified": d.get("cli_verified"),
        "wheel": d.get("wheel"),
        "endpoint_present": d.get("endpoint") is not None,
        "wrapper_pid": d.get("pid"),
        "child_pid": d.get("child_pid"),
        "wrapper_alive": alive(d.get("pid")),
        "child_alive": alive(d.get("child_pid")),
    }


def events_path(home, name):
    return os.path.join(home, "instances", name, "events.ndjson")


def await_start(home, name, bound):
    t0 = time.monotonic()
    recs = []
    while time.monotonic() - t0 < bound:
        recs = lines_from(events_path(home, name))
        if len(recs) >= 3 and snapshot(home, name).get("child_pid"):
            break
        time.sleep(0.1)
    codes = [code(r) for r in recs]
    doc = {
        "came_up": codes[:3] == START and len(codes) >= 3,
        "first_records": codes[:6],
        "first_ts": [r.get("ts") for r in recs[:3]],
        "records": len(recs),
        "wheel_human_records": sum(1 for r in recs if r.get("kind") == "wheel" and (r.get("data") or {}).get("holder") == "human"),
        "waited_s": round(time.monotonic() - t0, 2),
        "bound_s": bound,
    }
    doc.update(snap_codes(home, name))
    print(json.dumps(doc, sort_keys=True))


def hold(home, name, seconds):
    before = lines_from(events_path(home, name))
    t0 = time.monotonic()
    lost = None
    while time.monotonic() - t0 < seconds:
        s = snap_codes(home, name)
        if not (s["wrapper_alive"] and s["child_alive"]):
            lost = round(time.monotonic() - t0, 2)
            break
        time.sleep(0.5)
    after = lines_from(events_path(home, name))
    new = after[len(before):]
    doc = {
        "held_s": round(time.monotonic() - t0, 2),
        "asked_s": seconds,
        "process_lost_at_s": lost,
        "new_records": [code(r) for r in new],
        "new_wheel_records": sum(1 for r in new if r.get("kind") == "wheel"),
        "wheel_records_total": sum(1 for r in after if r.get("kind") == "wheel"),
    }
    doc.update(snap_codes(home, name))
    print(json.dumps(doc, sort_keys=True))


def ending_codes(text):
    names = {"\r": "cr", "\n": "lf"}
    tail = []
    for ch in reversed(text):
        if ch not in names:
            break
        tail.append(names[ch])
    return list(reversed(tail))


def reading(argv):
    rid, session, home, name, priv, text_file = argv[:6]
    rest = argv[6:]
    settle = 0.0
    append = None
    i = 0
    while i < len(rest):
        if rest[i] == "--settle-s":
            settle = float(rest[i + 1]); i += 2
        elif rest[i] == "--append":
            append = rest[i + 1]; i += 2
        else:
            sys.exit("usage: unknown option " + rest[i])
    with open(text_file, "rb") as f:
        sent_raw = f.read()
    sent = sent_raw.decode("utf-8")
    journal = [j for j in lines_from(os.path.join(priv, "journal.ndjson")) if j.get("label") == rid and j.get("verb") == "send"]
    if not journal:
        sys.exit("no send journalled under this id")
    j = journal[-1]

    # the records of the send, then what follows it until a turn ends or the settle bound passes
    t0 = time.monotonic()
    while True:
        recs = lines_from(events_path(home, name), j["events_before"])
        kinds = [r.get("kind") for r in recs]
        if "turn-ended" in kinds or time.monotonic() - t0 >= settle:
            break
        time.sleep(0.25)
    prompts = [r for r in recs if r.get("kind") == "prompt-submitted"]
    wheels = [r for r in recs if r.get("kind") == "wheel"]
    issued = [r for r in recs if r.get("kind") == "send-issued"]
    refused = [r for r in recs if r.get("kind") == "send-refused"]
    confirmed = [r for r in recs if r.get("kind") == "send-confirmed"]

    refusal = detail = side = None
    if refused:
        refusal = refused[-1]["data"].get("refusal")
        detail = refused[-1]["data"].get("detail")
        side = "wrapper"
    else:
        client = [c for c in lines_from(os.path.join(home, "diagnostics", "cli-%s.ndjson" % name)) if c.get("event") == "send-refused" and c.get("timestamp", "") >= j["t0"]]
        if client:
            refusal, detail, side = client[-1].get("refusal"), client[-1].get("detail"), client[-1].get("side")

    text_bytes = None
    cursor = issued[-1]["data"].get("cursor") if issued else None
    run_log = lines_from(os.path.join(home, "diagnostics", "run-%s.ndjson" % name))
    for r in run_log:
        if r.get("event") == "send-issued" and cursor is not None and r.get("corr") == cursor:
            text_bytes = r.get("text_bytes")
    confirmed_lines = [r for r in run_log if r.get("event") == "send-confirmed" and cursor is not None and r.get("corr") == cursor]

    doc = {
        "kind": "reading",
        "id": rid,
        "session": session,
        "outcome": "measured",
        "at": j["t0"],
        "exit": j["exit"],
        "confirmed": bool(confirmed) or bool(confirmed_lines) or j.get("result") == "read-back",
        "refusal": refusal,
        "detail": detail,
        "refused_side": side,
        "sent_bytes": len(sent_raw),
        "sent_ending": ending_codes(sent),
        "text_bytes": text_bytes,
        "records": [code(r) for r in recs],
        "send_issued_records": len(issued),
        "send_refused_records": len(refused),
        "prompt_submitted_records": len(prompts),
        "prompt_origin": (prompts[0].get("data") or {}).get("origin") if prompts else None,
        "prompt_text_bytes": None,
        "prompt_equals_sent_without_last_char": None,
        "prompt_equals_sent_without_cr_lf_ending": None,
        "prompt_equals_sent_unchanged": None,
        "wheel_records": len(wheels),
        "wheel_holder": (wheels[-1].get("data") or {}).get("holder") if wheels else None,
        "wheel_cause": (wheels[-1].get("data") or {}).get("cause") if wheels else None,
        "snapshot_wheel": snapshot(home, name).get("wheel"),
        "turn_ended": "turn-ended" in kinds,
        "stderr_hint_lines": j.get("stderr_hint_lines"),
        "hint_names_release": j.get("hint_names_release"),
    }
    if prompts:
        p = (prompts[0].get("data") or {}).get("text")
        if isinstance(p, str):
            doc["prompt_text_bytes"] = len(p.encode("utf-8"))
            doc["prompt_equals_sent_without_last_char"] = p == sent[:-1]
            doc["prompt_equals_sent_without_cr_lf_ending"] = p == sent.rstrip("\r\n")
            doc["prompt_equals_sent_unchanged"] = p == sent
            doc["prompt_cr_count"] = p.count("\r")
            doc["prompt_lf_count"] = p.count("\n")
    if issued and wheels:
        doc["wheel_after_send_issued"] = wheels[0].get("ts", "") >= issued[0].get("ts", "")
    line = json.dumps(doc, sort_keys=True)
    if append:
        with open(append, "a", encoding="utf-8") as f:
            f.write(line + "\n")
    print(line)


def census(priv, label, wrapper_pid, child_pid):
    try:
        with open(os.path.join(priv, label + ".pty.json"), encoding="utf-8") as f:
            st = json.load(f)
    except (OSError, ValueError):
        st = {}
    sessions = {p for p in (st.get("child_pid"), wrapper_pid, child_pid) if isinstance(p, int)}
    left = []
    for entry in os.listdir("/proc"):
        if not entry.isdigit():
            continue
        try:
            with open("/proc/%s/stat" % entry, encoding="utf-8", errors="replace") as f:
                stat = f.read()
        except OSError:
            continue
        fields = stat[stat.rindex(")") + 2:].split()
        state, ppid, pgid, sid = fields[0], int(fields[1]), int(fields[2]), int(fields[3])
        if state == "Z":
            continue
        if sid in sessions or pgid in sessions or ppid in sessions or int(entry) in sessions:
            left.append(int(entry))
    doc = {
        "pty_state": st.get("state"),
        "pty_exit_code": st.get("exit_code"),
        "pty_killed": st.get("killed"),
        "ctrl_c_presses": st.get("ctrl_c_presses"),
        "drained_bytes": st.get("drained_bytes"),
        "written_before_stop": st.get("written_before_stop"),
        "host_gone": not alive(st.get("host_pid")),
        "wrapper_gone": not alive(wrapper_pid),
        "child_gone": not alive(child_pid),
        "rig_processes_left": len(left),
    }
    print(json.dumps(doc, sort_keys=True))


def main():
    a = sys.argv[1:]
    if len(a) >= 3 and a[0] == "await-start":
        await_start(a[1], a[2], float(a[3]) if len(a) > 3 else 60.0)
    elif len(a) == 4 and a[0] == "hold":
        hold(a[1], a[2], float(a[3]))
    elif len(a) >= 7 and a[0] == "reading":
        reading(a[1:])
    elif len(a) == 5 and a[0] == "census":
        census(a[1], a[2], int(a[3]), int(a[4]))
    else:
        sys.exit(__doc__)


if __name__ == "__main__":
    main()
