#!/usr/bin/env python3
"""The rig's SessionStart recorder (plan.md step 12).

A second plugin folder, the rig's own, registers this for SessionStart beside viola's hook. It reads the hook
payload on stdin and appends one line to the file it is given: the payload's key names, sorted, and the four
values the readings compare (`source`, `session_id`, `cwd`, the name of the directory `transcript_path` lies in).
That file stands in the private directory outside the tree. It prints nothing and exits 0 whatever it read.

  live-keys.py <out file>
"""
import datetime
import json
import os
import sys


def main():
    out = sys.argv[1]
    try:
        doc = json.load(sys.stdin)
    except ValueError:
        doc = None
    rec = {
        "at": datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%S.%f")[:-3] + "Z",
        "object": isinstance(doc, dict),
    }
    if isinstance(doc, dict):
        rec["keys"] = sorted(doc)
        for key in ("source", "session_id", "cwd", "hook_event_name"):
            value = doc.get(key)
            rec[key] = value if isinstance(value, str) else None
        path = doc.get("transcript_path")
        rec["transcript_dir"] = os.path.basename(os.path.dirname(path)) if isinstance(path, str) else None
    with open(out, "a", encoding="utf-8") as f:
        f.write(json.dumps(rec, sort_keys=True) + "\n")


if __name__ == "__main__":
    try:
        main()
    except Exception:  # a recorder never fails its session
        pass
    sys.exit(0)
