import json, glob, os

root = os.path.expanduser("~/.viola/sessions")
for f in sorted(glob.glob(root + "/*/events.ndjson")):
    sess = f.split("/")[-2]
    last_pre = {}
    for n, line in enumerate(open(f, encoding="utf-8", errors="replace")):
        try:
            e = json.loads(line)
        except Exception:
            continue
        d = e.get("data")
        if not isinstance(d, dict) or d.get("tool_name") not in ("AskUserQuestion", "ExitPlanMode"):
            continue
        ev, tool = d.get("hook_event_name"), d.get("tool_name")
        if ev == "PreToolUse":
            last_pre[tool] = (n, d)
        elif ev == "PermissionRequest":
            pn, pd = last_pre.get(tool, (None, None))
            same = pd is not None and pd.get("tool_input") == d.get("tool_input")
            extra = sorted(set(d) - set(pd or {})) if pd else None
            missing = sorted(set(pd or {}) - set(d)) if pd else None
            print(sess, tool, "PermissionRequest line", n, "preceding PreToolUse line", pn,
                  "tool_input equal", same, "same session_id", pd is not None and pd.get("session_id") == d.get("session_id"),
                  "keys only in PR", extra, "keys only in PTU", missing,
                  "pr bytes", len(json.dumps(d)), "ptu bytes", len(json.dumps(pd)) if pd else None)
