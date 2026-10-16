#!/usr/bin/env python3
"""The P4 / P12 reader (plan.md step 17): equalities and counts only, never an id, a title or a text.

For each named instance it reads `<home>/instances/<name>/snapshot.json` (its `child_pid`) and the logged
`session-start` records of `events.ndjson` (cause and id), then makes one `claude agents --json` read with the
given CLI, spawned directly with every CLAUDE* and VIOLA* name removed from its environment, stdin empty, killed at
5 s. A read is no start: it opens no session. It prints one JSON line:

- `rows`: how many rows the read listed;
- per instance: the causes of its logged starts in order, whether its latest logged id equals its first, how many
  rows carry its child pid, whether that row's id equals the latest or the first logged id, that row's status word,
  and how many rows carry its latest logged id;
- per ordered pair of instances: whether their latest logged ids are equal.

  live-p4-read.py <home> <claude> <name> [<name>...]
"""
import json
import os
import subprocess
import sys


def snapshot(home, name):
    try:
        with open(os.path.join(home, "instances", name, "snapshot.json"), encoding="utf-8") as f:
            return json.load(f).get("data") or {}
    except (OSError, ValueError):
        return {}


def starts(home, name):
    out = []
    try:
        with open(os.path.join(home, "instances", name, "events.ndjson"), "rb") as f:
            data = f.read()
    except OSError:
        return out
    for line in data.split(b"\n"):
        try:
            doc = json.loads(line)
        except ValueError:
            continue
        if isinstance(doc, dict) and doc.get("kind") == "session-start":
            d = doc.get("data") if isinstance(doc.get("data"), dict) else {}
            out.append((d.get("cause"), d.get("agent_session_id")))
    return out


def agents(claude):
    env = {k: v for k, v in os.environ.items() if not k.startswith(("CLAUDE", "VIOLA"))}
    try:
        p = subprocess.run([claude, "agents", "--json"], stdin=subprocess.DEVNULL, capture_output=True,
                           env=env, timeout=5, check=False)
    except subprocess.TimeoutExpired:
        return None
    if p.returncode != 0:
        return None
    try:
        rows = json.loads(p.stdout)
    except ValueError:
        return None
    return rows if isinstance(rows, list) else None


def main():
    if len(sys.argv) < 4:
        sys.exit(__doc__)
    home, claude, names = sys.argv[1], sys.argv[2], sys.argv[3:]
    rows = agents(claude)
    doc = {"read": rows is not None, "rows": len(rows) if rows is not None else None, "instances": {}, "pairs": {}}
    latest = {}
    for name in names:
        snap = snapshot(home, name)
        logged = starts(home, name)
        ids = [i for _, i in logged if isinstance(i, str)]
        last = ids[-1] if ids else None
        first = ids[0] if ids else None
        latest[name] = last
        entry = {
            "causes": [c for c, _ in logged],
            "latest_equals_first": last is not None and last == first,
            "snapshot_id_is_latest": last is not None and snap.get("agent_session_id") == last,
        }
        if rows is not None:
            pid = snap.get("child_pid")
            own = [r for r in rows if isinstance(r, dict) and pid is not None and r.get("pid") == pid]
            entry["child_rows"] = len(own)
            if len(own) == 1:
                entry["child_row_id_is_latest"] = own[0].get("sessionId") == last
                entry["child_row_id_is_first"] = own[0].get("sessionId") == first
                status = own[0].get("status")
                entry["child_row_status"] = status if status in ("idle", "busy") else "other"
            entry["rows_with_latest_id"] = sum(1 for r in rows if isinstance(r, dict) and last is not None and r.get("sessionId") == last)
        doc["instances"][name] = entry
    for a in names:
        for b in names:
            if a < b:
                doc["pairs"][a + "~" + b] = latest[a] is not None and latest[a] == latest[b]
    print(json.dumps(doc, sort_keys=True))


if __name__ == "__main__":
    main()
