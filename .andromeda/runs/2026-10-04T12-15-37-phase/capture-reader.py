import json, glob, os, sys

root = os.path.expanduser("~/.viola/sessions")
rows = []
for f in sorted(glob.glob(root + "/*/events.ndjson")):
    sess = f.split("/")[-2]
    for n, line in enumerate(open(f, encoding="utf-8", errors="replace")):
        try:
            e = json.loads(line)
        except Exception:
            continue
        # find the hook payload object
        found = []
        def walk(x, path):
            if isinstance(x, dict):
                if x.get("hook_event_name") in ("PreToolUse", "PermissionRequest") and "tool_name" in x:
                    found.append((path, x))
                for k, v in x.items():
                    walk(v, path + [k])
            elif isinstance(x, list):
                for i, v in enumerate(x):
                    walk(v, path + [i])
        walk(e, [])
        for path, p in found:
            if p["tool_name"] not in ("AskUserQuestion", "ExitPlanMode"):
                continue
            raw = json.dumps(p, ensure_ascii=False)
            rows.append((sess, n, p["hook_event_name"], p["tool_name"], len(raw), path, sorted(p.keys()), p.get("tool_use_id"), e.get("ts"), e.get("kind") or e.get("type")))

mode = sys.argv[1] if len(sys.argv) > 1 else "list"
if mode == "list":
    for r in rows:
        print(r[0], "line", r[1], r[2], r[3], "bytes", r[4], "path", r[5], "tuid", r[7], "ts", r[8], "linekind", r[9])
    print("keys:")
    seen = set()
    for r in rows:
        k = (r[2], r[3], tuple(r[6]))
        if k not in seen:
            seen.add(k)
            print(" ", r[2], r[3], r[6])
elif mode == "line":
    sess, n = sys.argv[2], int(sys.argv[3])
    f = root + "/" + sess + "/events.ndjson"
    for i, line in enumerate(open(f, encoding="utf-8", errors="replace")):
        if i == n:
            e = json.loads(line)
            print(json.dumps(e, indent=1, ensure_ascii=False)[:6000])
elif mode == "context":
    # print kinds of lines around PermissionRequest AskUserQuestion rows (no content)
    for r in rows:
        if r[2] == "PermissionRequest":
            f = root + "/" + r[0] + "/events.ndjson"
            lines = open(f, encoding="utf-8", errors="replace").read().splitlines()
            print("==", r[0], r[1], r[3], "tuid", r[7])
            for i in range(max(0, r[1] - 4), min(len(lines), r[1] + 4)):
                try:
                    e = json.loads(lines[i])
                except Exception:
                    print("  ", i, "torn")
                    continue
                inner = {}
                def walk2(x):
                    if isinstance(x, dict):
                        if "hook_event_name" in x:
                            inner.update({"ev": x.get("hook_event_name"), "tool": x.get("tool_name"), "tuid": x.get("tool_use_id")})
                        for v in x.values():
                            walk2(v)
                walk2(e)
                print("  ", i, e.get("ts"), e.get("kind") or e.get("type"), inner)
