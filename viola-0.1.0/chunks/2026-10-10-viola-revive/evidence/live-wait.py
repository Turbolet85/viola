#!/usr/bin/env python3
"""A bounded wait on an instance's event log (plan.md step 12).

It polls `<home>/instances/<name>/events.ndjson` every 50 ms for the first complete line of the given kind that
starts at or after the given byte offset, up to the bound. It prints one JSON line of codes: whether the record
was found, how long the wait took, what it settled on (the record's `ts`, and for a `session-start` its `cause`
and `agent_session_id`), the log's size at the end, and how many records of each kind stand past the offset. It
prints no prompt text, assistant text or payload text. The same line is appended to `<private dir>/waits.ndjson`.

  live-wait.py <private dir> <home> <name> <label> <offset> <kind> <bound seconds>
"""
import datetime
import json
import os
import sys
import time


def stamp():
    return datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%S.%f")[:-3] + "Z"


def records(path, offset):
    try:
        with open(path, "rb") as f:
            f.seek(offset)
            data = f.read()
    except OSError:
        return []
    end = data.rfind(b"\n")
    if end < 0:
        return []
    out = []
    for line in data[:end].split(b"\n"):
        try:
            doc = json.loads(line)
        except ValueError:
            continue
        if isinstance(doc, dict):
            out.append(doc)
    return out


def main():
    if len(sys.argv) != 8:
        sys.exit(__doc__)
    priv, home, name, label, offset, kind, bound = sys.argv[1:8]
    offset, bound = int(offset), float(bound)
    path = os.path.join(home, "instances", name, "events.ndjson")
    began = time.monotonic()
    began_at = stamp()
    found = None
    while True:
        seen = records(path, offset)
        found = next((r for r in seen if r.get("kind") == kind), None)
        waited = time.monotonic() - began
        if found is not None or waited >= bound:
            break
        time.sleep(0.05)
    kinds = {}
    for r in seen:
        k = r.get("kind")
        kinds[k] = kinds.get(k, 0) + 1
    doc = {
        "label": label,
        "instance": name,
        "kind": kind,
        "offset": offset,
        "bound_s": bound,
        "began_at": began_at,
        "found": found is not None,
        "waited_s": round(waited, 3),
        "log_bytes": os.path.getsize(path) if os.path.exists(path) else 0,
        "kinds_past_offset": kinds,
    }
    if found is not None:
        data = found.get("data") if isinstance(found.get("data"), dict) else {}
        doc["settled_on"] = {"ts": found.get("ts"), "source": found.get("source")}
        if kind == "session-start":
            doc["settled_on"]["cause"] = data.get("cause")
            doc["settled_on"]["agent_session_id"] = data.get("agent_session_id")
    line = json.dumps(doc, sort_keys=True)
    with open(os.path.join(priv, "waits.ndjson"), "a", encoding="utf-8") as f:
        f.write(line + "\n")
    print(line)


if __name__ == "__main__":
    main()
