"""Relays the dialog-tier hook fixtures from the viola-lab prototype's live captures (founder ruling, live,
2026-10-04 ~11:55Z): print mode raises no dialog hook on claude 2.1.287, so `viola verify --record` cannot.

Reads every `~/.viola/sessions/*/events.ndjson` line's `data` (the hook payload as the prototype stored it,
keys sorted) and keeps, per tool (AskUserQuestion, ExitPlanMode), PreToolUse -> PermissionRequest PAIRS from one
session whose `tool_input` is equal (the PermissionRequest repeats its PreToolUse; measured 8 of 8). Per tool it
takes the pair with the smallest combined size, replaces an ExitPlanMode pair's `tool_input.plan` with one neutral
string (the overseer's review: viola is public, a captured plan is private text), scrubs both payloads with `ledger::scrub`'s rule, refuses the whole
run if any string still holds a path or the user word (`ledger::is_clean`'s checks), and only then writes
`fixtures/claude/2.1.287/<Event>.<kebab tool>.json`, one compact sorted-key object plus a newline (the form
`viola verify --record` writes).

Prints one line per written fixture: its file name, the source session, the line number, the byte size and the
sha256 of the written file. Never prints payload content. `--exclude <session>` (repeatable) drops a session the
overseer's review rejected. Exit 1 writes nothing.
"""

import argparse
import glob
import hashlib
import json
import os
import sys
from pathlib import Path

VERSION = "2.1.287"
TOOLS = {"AskUserQuestion": "ask-user-question", "ExitPlanMode": "exit-plan-mode"}
# viola is a public repository and a captured plan is another (private) project's text: the overseer's review
# (2026-10-04) has the relay replace `tool_input.plan` with one neutral string in both payloads of a pair, every
# key kept and the two `tool_input` values still equal.
NEUTRAL_PLAN = "# Probe plan\n\nCreate the file probe.txt in the working directory.\nThen stop.\n"
ROOT = Path(__file__).resolve().parents[4]
OUT = ROOT / "fixtures" / "claude" / VERSION


def compact(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def fold(text, case_insensitive):
    return text.lower() if case_insensitive else text


def replace_all(text, needle, with_, case_insensitive):
    folded, needle = fold(text, case_insensitive), fold(needle, case_insensitive)
    out, last, at = [], 0, folded.find(needle)
    while at != -1:
        out.append(text[last:at])
        out.append(with_)
        last = at + len(needle)
        at = folded.find(needle, last)
    out.append(text[last:])
    return "".join(out)


def is_word(c):
    return c.isalnum() or c == "_"


def replace_word(text, word, with_):
    folded, word = text.lower(), word.lower()
    out, last, at = [], 0, folded.find(word)
    while at != -1:
        end = at + len(word)
        before = at == 0 or not is_word(text[at - 1])
        after = end == len(text) or not is_word(text[end])
        if before and after:
            out.append(text[last:at])
            out.append(with_)
            last = end
        at = folded.find(word, at + 1)
    out.append(text[last:])
    return "".join(out)


def scrub_text(text, home, user, case_insensitive):
    if home:
        for spelling in (home.replace("\\", "/"), home.replace("/", "\\")):
            text = replace_all(text, spelling, "~", case_insensitive)
    if user:
        text = replace_word(text, user, "<user>")
    return text


def scrub(value, home, user, case_insensitive):
    if isinstance(value, str):
        return scrub_text(value, home, user, case_insensitive)
    if isinstance(value, list):
        return [scrub(v, home, user, case_insensitive) for v in value]
    if isinstance(value, dict):
        return {
            scrub_text(k, home, user, case_insensitive): scrub(v, home, user, case_insensitive)
            for k, v in value.items()
        }
    return value


def strings(value, out):
    if isinstance(value, str):
        out.append(value)
    elif isinstance(value, list):
        for v in value:
            strings(v, out)
    elif isinstance(value, dict):
        for k, v in value.items():
            out.append(k)
            strings(v, out)


def has_absolute_path(text):
    drive = len(text) >= 3 and text[0].isascii() and text[0].isalpha() and text[1] == ":" and text[2] in "\\/"
    return drive or "/home/" in text or "/Users/" in text or "\\Users\\" in text


def has_user_word(text, user):
    if not user or user == "<user>":
        return False
    return replace_word(text, user, "\0") != text


def is_clean(value, user):
    found = []
    strings(value, found)
    return all(not has_absolute_path(s) and not has_user_word(s, user) for s in found)


def redact_plan(payload):
    """A copy with `tool_input.plan` (and only it) replaced by NEUTRAL_PLAN; refuses a payload without one."""
    tool_input = payload.get("tool_input")
    if not isinstance(tool_input, dict) or not isinstance(tool_input.get("plan"), str):
        raise ValueError("an ExitPlanMode payload without a string tool_input.plan")
    return {**payload, "tool_input": {**tool_input, "plan": NEUTRAL_PLAN}}


def pairs(store, excluded):
    """Every (tool, combined size, session, (pre line, pre data), (pr line, pr data)) in the store."""
    found = []
    for path in sorted(glob.glob(os.path.join(store, "*", "events.ndjson"))):
        session = os.path.basename(os.path.dirname(path))
        if session in excluded:
            continue
        last_pre = {}
        with open(path, encoding="utf-8", errors="strict") as f:
            for n, line in enumerate(f):
                try:
                    data = json.loads(line).get("data")
                except (ValueError, AttributeError):
                    continue
                if not isinstance(data, dict) or data.get("tool_name") not in TOOLS:
                    continue
                tool, event = data["tool_name"], data.get("hook_event_name")
                if event == "PreToolUse":
                    last_pre[tool] = (n, data)
                elif event == "PermissionRequest" and tool in last_pre:
                    pre_n, pre = last_pre.pop(tool)
                    if pre.get("tool_input") == data.get("tool_input"):
                        size = len(compact(pre).encode()) + len(compact(data).encode())
                        found.append((tool, size, session, (pre_n, pre), (n, data)))
    return found


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--exclude", action="append", default=[], metavar="SESSION")
    args = parser.parse_args()
    home = os.path.expanduser("~")
    user = os.path.basename(home.rstrip("/\\"))
    case_insensitive = os.name == "nt"
    store = os.path.join(home, ".viola", "sessions")

    candidates = pairs(store, set(args.exclude))
    files = []
    for tool, kebab in TOOLS.items():
        mine = sorted((c for c in candidates if c[0] == tool), key=lambda c: (c[1], c[2], c[3][0]))
        if not mine:
            print(f"refused: no PreToolUse -> PermissionRequest pair for {tool}", file=sys.stderr)
            return 1
        _, _, session, (pre_n, pre), (pr_n, pr) = mine[0]
        for event, n, payload in (("PreToolUse", pre_n, pre), ("PermissionRequest", pr_n, pr)):
            if tool == "ExitPlanMode":
                payload = redact_plan(payload)
            scrubbed = scrub(payload, home, user, case_insensitive)
            if not is_clean(scrubbed, user):
                print(f"refused: {event} {tool} from {session} line {n} still holds a path or the user word",
                      file=sys.stderr)
                return 1
            files.append((f"{event}.{kebab}.json", session, n, (compact(scrubbed) + "\n").encode()))

    OUT.mkdir(parents=True, exist_ok=True)
    for name, session, n, body in files:
        (OUT / name).write_bytes(body)
        digest = hashlib.sha256(body).hexdigest()
        print(f"{name}  session {session}  line {n}  bytes {len(body)}  sha256 {digest}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
