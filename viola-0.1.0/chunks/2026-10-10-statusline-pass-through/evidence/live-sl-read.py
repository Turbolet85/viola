#!/usr/bin/env python3
"""The reader of the statusline readings (plan.md step 12).

It reads what the rig's recorder filed in the private directory and, when a home and an instance are given, the
home's `budget.json` and the instance's hook role file. It prints codes only: names, counts, JSON types and
equalities. No payload value, no prompt text and no path of the user's is printed; a number from the payload is
only ever compared, never shown.

  live-sl-read.py records <private dir> [<label>]           one line per recorder invocation, then a summary
  live-sl-read.py home <private dir> <home> <instance>      budget.json and the hook's role lines
  live-sl-read.py wait <private dir> <label> <what> <bound seconds>
        polls every 100 ms until a recorder invocation of <label> exists (`any`) or one whose payload holds
        `rate_limits` (`rate_limits`); prints how long it waited and what it settled on
  live-sl-read.py wait-home <private dir> <home> <instance> <what> <bound seconds>
        the same over the hook role file: `invoked` (a statusline hook-decision exists) or `written` (one with
        budget_written true)

Run from the repository root.
"""
import datetime
import glob
import json
import os
import sys
import time


def stamp():
    return datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%S.%f")[:-3] + "Z"


def json_type(value):
    if isinstance(value, bool):
        return "boolean"
    if isinstance(value, int):
        return "whole number"
    if isinstance(value, float):
        return "fraction"
    if isinstance(value, str):
        return "string"
    if value is None:
        return "null"
    return "array" if isinstance(value, list) else "object"


def window(value):
    if not isinstance(value, dict):
        return {"is": json_type(value)}
    used = value.get("used_percentage")
    return {
        "keys": sorted(value),
        "used_percentage": json_type(used) if "used_percentage" in value else "absent",
        "used_in_0_100": isinstance(used, (int, float)) and not isinstance(used, bool) and 0 <= used <= 100,
        "resets_at": json_type(value.get("resets_at")) if "resets_at" in value else "absent",
    }


def invocations(priv, label=None):
    out = []
    for meta in sorted(glob.glob(os.path.join(priv, "rec-*.meta"))):
        rec = {}
        with open(meta, encoding="utf-8") as f:
            for line in f:
                key, _, value = line.rstrip("\n").partition("=")
                rec[key] = value
        if label and rec.get("label") != label:
            continue
        try:
            with open(meta[: -len(".meta")] + ".stdin", "rb") as f:
                raw = f.read()
        except OSError:
            raw = None
        try:
            doc = json.loads(raw) if raw is not None else None
        except ValueError:
            doc = None
        rec["payload_object"] = isinstance(doc, dict)
        rec["_doc"] = doc if isinstance(doc, dict) else {}
        out.append(rec)
    return sorted(out, key=lambda r: r.get("at", ""))


def shown(rec):
    doc = rec["_doc"]
    limits = doc.get("rate_limits")
    line = {k: v for k, v in rec.items() if not k.startswith("_")}
    line["keys"] = sorted(doc)
    if isinstance(limits, dict):
        line["rate_limits"] = {name: window(value) for name, value in sorted(limits.items())}
    else:
        line["rate_limits"] = "absent" if "rate_limits" not in doc else json_type(limits)
    return line


def records(priv, label):
    recs = invocations(priv, label)
    for rec in recs:
        print(json.dumps(shown(rec), sort_keys=True))
    with_limits = [i for i, r in enumerate(recs) if isinstance(r["_doc"].get("rate_limits"), dict)]
    key_sets = sorted({tuple(sorted(r["_doc"])) for r in recs})
    summary = {
        "invocations": len(recs),
        "by_label": {},
        "with_rate_limits": len(with_limits),
        "first_with_rate_limits": with_limits[0] + 1 if with_limits else None,
        "distinct_key_sets": len(key_sets),
        "key_union": sorted({k for r in recs for k in r["_doc"]}),
        "launcher_arg0": sorted({r.get("launcher_arg0", "") for r in recs}),
        "launcher_bash": sorted({r.get("launcher_bash", "") for r in recs}),
        "launcher_zsh": sorted({r.get("launcher_zsh", "") for r in recs}),
        "ancestors": sorted(
            {" < ".join(r.get(f"p{n}_comm", "") + "(" + r.get(f"p{n}_exe", "") + ")" for n in (1, 2, 3)) for r in recs}
        ),
    }
    for r in recs:
        summary["by_label"][r.get("label")] = summary["by_label"].get(r.get("label"), 0) + 1
    print(json.dumps({"summary": summary}, sort_keys=True))


def role_lines(home, instance):
    path = os.path.join(home, "diagnostics", f"hook-{instance}.ndjson")
    try:
        with open(path, "rb") as f:
            data = f.read()
    except OSError:
        return []
    out = []
    for line in data[: data.rfind(b"\n") + 1].split(b"\n"):
        try:
            doc = json.loads(line)
        except ValueError:
            continue
        if isinstance(doc, dict):
            out.append(doc)
    return out


def statusline_lines(home, instance):
    lines = role_lines(home, instance)
    return [
        l for l in lines if l.get("hook_event") == "statusline" or l.get("subject") == "statusline-shell"
    ]


def rfc3339_of(seconds):
    at = datetime.datetime.fromtimestamp(seconds, datetime.timezone.utc)
    return at.strftime("%Y-%m-%dT%H:%M:%S.000Z")


def home(priv, home_dir, instance):
    lines = statusline_lines(home_dir, instance)
    counts = {}
    for l in lines:
        key = l.get("event", "")
        for field in ("subject", "detail", "budget_written", "shell_exit_status", "deadline_hit", "level"):
            if field in l:
                key += f" {field}={json.dumps(l[field])}"
        counts[key] = counts.get(key, 0) + 1
    durations = [l["duration_ms"] for l in lines if l.get("event") == "hook-decision" and "duration_ms" in l]
    shells = [l["duration_ms"] for l in lines if l.get("event") == "process-exit" and "duration_ms" in l]
    out = {
        "statusline_lines": len(lines),
        "by_shape": dict(sorted(counts.items())),
        "decision_ms": {"n": len(durations), "min": min(durations, default=None), "max": max(durations, default=None)},
        "shell_ms": {"n": len(shells), "min": min(shells, default=None), "max": max(shells, default=None)},
        "any_corr": any("corr" in l for l in lines),
        "field_names": sorted({k for l in lines for k in l}),
    }
    try:
        with open(os.path.join(home_dir, "budget.json"), encoding="utf-8") as f:
            budget = json.load(f)
    except (OSError, ValueError):
        budget = None
    if isinstance(budget, dict):
        forms = {}
        for name in ("five_hour", "seven_day"):
            w = budget.get(name)
            forms[name] = (
                {k: json_type(v) if v != "unknown" else "the word unknown" for k, v in sorted(w.items())}
                if isinstance(w, dict)
                else ("the word unknown" if w == "unknown" else json_type(w))
            )
        out["budget"] = {"keys": sorted(budget), "v": budget.get("v"), "windows": forms, "read_at_chars": len(str(budget.get("read_at", "")))}
        # the newest payload a wrapped recorder was handed that holds a reading, compared field by field
        wrapped = [r for r in invocations(priv, "wrapped") if isinstance(r["_doc"].get("rate_limits"), dict)]
        if wrapped:
            limits = wrapped[-1]["_doc"]["rate_limits"]
            same = {}
            for name in ("five_hour", "seven_day"):
                src, got = limits.get(name), budget.get(name)
                if not isinstance(src, dict):
                    same[name] = got == "unknown"
                    continue
                reset = src.get("resets_at")
                want_reset = rfc3339_of(reset) if isinstance(reset, int) and not isinstance(reset, bool) else None
                same[name] = (
                    isinstance(got, dict)
                    and got.get("used_percentage") == src.get("used_percentage")
                    and (want_reset is None or got.get("resets_at") == want_reset)
                )
            out["budget"]["equals_the_newest_wrapped_payload_s_reading"] = same
    else:
        out["budget"] = None
    try:
        mode = oct(os.stat(os.path.join(home_dir, "budget.json")).st_mode & 0o777)
    except OSError:
        mode = None
    out["budget_mode"] = mode
    print(json.dumps(out, sort_keys=True))


def wait(priv, label, what, bound, probe):
    began, began_at = time.monotonic(), stamp()
    while True:
        settled = probe()
        waited = time.monotonic() - began
        if settled is not None or waited >= bound:
            break
        time.sleep(0.1)
    doc = {
        "label": label,
        "what": what,
        "bound_s": bound,
        "began_at": began_at,
        "found": settled is not None,
        "waited_s": round(waited, 3),
        "settled_on": settled,
    }
    line = json.dumps(doc, sort_keys=True)
    with open(os.path.join(priv, "waits.ndjson"), "a", encoding="utf-8") as f:
        f.write(line + "\n")
    print(line)


def main():
    args = sys.argv[1:]
    if len(args) >= 2 and args[0] == "records":
        records(args[1], args[2] if len(args) > 2 else None)
    elif len(args) == 4 and args[0] == "home":
        home(args[1], args[2], args[3])
    elif len(args) == 5 and args[0] == "wait":
        priv, label, what, bound = args[1], args[2], args[3], float(args[4])

        def probe():
            recs = invocations(priv, label)
            if what == "rate_limits":
                recs = [r for r in recs if isinstance(r["_doc"].get("rate_limits"), dict)]
            return {"invocations": len(recs), "first_at": recs[0].get("at")} if recs else None

        wait(priv, label, what, bound, probe)
    elif len(args) == 6 and args[0] == "wait-home":
        priv, home_dir, instance, what, bound = args[1], args[2], args[3], args[4], float(args[5])

        def probe():
            lines = [l for l in statusline_lines(home_dir, instance) if l.get("event") == "hook-decision"]
            if what == "written":
                lines = [l for l in lines if l.get("budget_written") is True]
            return {"decisions": len(lines), "first_at": lines[0].get("timestamp")} if lines else None

        wait(priv, instance, what, bound, probe)
    else:
        sys.exit(__doc__)


if __name__ == "__main__":
    main()
