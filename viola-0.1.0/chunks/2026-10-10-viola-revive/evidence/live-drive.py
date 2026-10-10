#!/usr/bin/env python3
"""The driver of the live sessions (plan.md steps 6 to 9).

It runs the product's own verbs, `target/release-check/release/viola --home <home> <verb> <name>`, with the text
on stdin and stdout / stderr in files of a private directory outside the tree, and keeps a journal of codes:
verb, exit, times, the byte size of the instance's event log before and after, a cursor, a dialog id and kind,
the rule a dialog was answered by. No prompt text, assistant text or event payload text is printed or journalled.

A dialog must be answered by its id inside the wrapper's 60 s deadline, so a turn is one call: send, wait, answer
each dialog as it is raised, wait again, until the turn ends.

  live-drive.py <private dir> <home> <name> send <label> <text file>
  live-drive.py <private dir> <home> <name> turn <label> <text file> [--timeout-ms N]
        [--question <asked text> <answer>] [--deny-true] [--plan approve]
        [--then <label> <text file>]
  live-drive.py <private dir> <home> <name> last <label> [<expected single word>]
  live-drive.py selftest

Run from the repository root.
"""
import datetime
import json
import os
import re
import subprocess
import sys

VIOLA = "target/release-check/release/viola"
HOME = os.path.expanduser("~")
SKILLS = HOME + "/.claude/skills/"
DENY = {"behavior": "deny", "message": "not part of this test"}
DIALOG_KINDS = ("question", "permission", "plan")
GIT_READS = ("status", "log", "rev-list", "rev-parse", "diff", "show", "ls-files")


def stamp():
    return datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%S.%f")[:-3] + "Z"


class Drive:
    def __init__(self, priv, home, name):
        self.priv = priv
        self.home = home
        self.name = name
        self.events = os.path.join(home, "instances", name, "events.ndjson")
        self.journal = os.path.join(priv, "journal.ndjson")

    def size(self):
        try:
            return os.path.getsize(self.events)
        except OSError:
            return 0

    def count(self):
        try:
            with open(self.journal, encoding="utf-8") as f:
                return sum(1 for _ in f)
        except OSError:
            return 0

    def note(self, rec):
        with open(self.journal, "a", encoding="utf-8") as f:
            f.write(json.dumps(rec, sort_keys=True) + "\n")

    def verb(self, label, verb, args, stdin):
        n = self.count() + 1
        base = "%03d-%s-%s" % (n, label, verb)
        out = os.path.join(self.priv, base + ".out")
        err = os.path.join(self.priv, base + ".err")
        before = self.size()
        t0 = stamp()
        with open(out, "wb") as o, open(err, "wb") as e:
            p = subprocess.run(
                [VIOLA, "--home", self.home, verb, self.name, *args],
                input=stdin if stdin is not None else b"",
                stdout=o,
                stderr=e,
                check=False,
            )
        rec = {
            "n": n,
            "label": label,
            "verb": verb,
            "args": [a for a in args],
            "exit": p.returncode,
            "t0": t0,
            "t1": stamp(),
            "events_before": before,
            "events_after": self.size(),
            "out": base + ".out",
            "err": base + ".err",
            "stdin_bytes": len(stdin) if stdin is not None else 0,
        }
        return rec, out, err


def expand(token, variables):
    t = token.strip("\"'")
    t = t.replace("${HOME}", HOME).replace("$HOME", HOME)
    if t == "~" or t.startswith("~/"):
        t = HOME + t[1:]
    for k, v in variables.items():
        t = t.replace("${%s}" % k, v).replace("$%s" % k, v)
    if "$" in t or "~" in t:
        return None
    return os.path.normpath(t)


def bash_is_safe(cmd, root):
    """A Bash call allowed without a human: the skills' own tool scripts, a git read, an echo, a cd to the root."""
    if any(x in cmd for x in ("`", "$(", "<", "\\")):
        return False
    c = cmd.replace("2>/dev/null", " ").replace("2>&1", " ")
    if ">" in c:
        return False
    segs = re.split(r";|&&|\|\||\n", c)
    if any("|" in s or "&" in s for s in segs):
        return False
    variables = {}
    seen = 0
    for seg in segs:
        s = seg.strip()
        if not s:
            continue
        seen += 1
        m = re.fullmatch(r"([A-Za-z_][A-Za-z0-9_]*)=(\S+)", s)
        if m:
            p = expand(m.group(2), variables)
            if p and (p + "/").startswith(SKILLS):
                variables[m.group(1)] = p
                continue
            return False
        toks = s.split()
        head = toks[0]
        if head == "true" and seen > 1 and len(toks) == 1:
            continue
        if head == "cd":
            if len(toks) == 2 and expand(toks[1], variables) == root:
                continue
            return False
        if head == "echo":
            rest = s[4:].replace("$?", "")
            if "$" in rest:
                return False
            continue
        if head == "git":
            if len(toks) >= 2 and toks[1] == "branch" and toks[2:] == ["--show-current"]:
                continue
            if len(toks) >= 2 and toks[1] in GIT_READS:
                if any(t.startswith("--output") or t.startswith("--ext-diff") or t == "-o" for t in toks[2:]):
                    return False
                if "$" in s:
                    return False
                continue
            return False
        if head in ("python", "python3") and toks[1:3] == ["-X", "utf8"] and len(toks) >= 4:
            p = expand(toks[3], variables)
            if not (p and p.startswith(SKILLS) and p.endswith(".py")):
                return False
            for t in toks[4:]:
                if "$" in t and expand(t, variables) is None:
                    return False
            continue
        return False
    return seen > 0


def decide(kind, data, planned, root):
    """The response to a raised dialog and the rule it came from. None: no answer here, control returns."""
    if kind == "question" and "question" in planned:
        asked, answer = planned.pop("question")
        qs = data.get("questions") or []
        text = qs[0].get("question") if qs and isinstance(qs[0], dict) else None
        if not isinstance(text, str):
            return None, "question-unreadable", {}
        return {"answers": {text: answer}}, "planned", {"asked_as_planned": text == asked, "questions": len(qs)}
    if kind == "permission":
        tool = data.get("tool")
        inp = data.get("input") if isinstance(data.get("input"), dict) else {}
        cmd = inp.get("command") if isinstance(inp.get("command"), str) else ""
        extra = {"tool": tool if isinstance(tool, str) else None}
        if planned.get("deny-true") and tool == "Bash" and cmd.strip() == "true":
            planned.pop("deny-true")
            return dict(DENY), "planned", dict(extra, behavior="deny", command_is_true=True)
        if tool in ("Read", "Glob", "Grep"):
            return {"behavior": "allow"}, "auto-read-tool", dict(extra, behavior="allow")
        if tool == "Bash" and bash_is_safe(cmd, root):
            return {"behavior": "allow"}, "auto-skill-script-or-read", dict(extra, behavior="allow")
        return dict(DENY), "auto-deny", dict(extra, behavior="deny")
    if kind == "plan" and "plan" in planned:
        behavior = planned.pop("plan")
        return {"behavior": behavior}, "planned", {"behavior": behavior}
    if kind == "question":
        qs = data.get("questions") or []
        answers = {q.get("question"): "Not now, thank you." for q in qs if isinstance(q, dict) and isinstance(q.get("question"), str)}
        if not answers:
            return None, "question-unreadable", {}
        return {"answers": answers}, "auto-decline", {"questions": len(qs)}
    return None, "unplanned-" + kind, {}


def do_send(d, label, path):
    with open(path, "rb") as f:
        text = f.read()
    rec, out, err = d.verb(label, "send", [], text)
    with open(out, "rb") as f:
        first = f.read()
    m = re.search(rb"^\[RB\] read back\s+\S+\s+\S+\s+cursor (\d+)", first)
    if m:
        rec["result"] = "read-back"
        rec["cursor"] = int(m.group(1))
    elif first.startswith(b"[  ] unconfirmable"):
        rec["result"] = "unconfirmable"
    else:
        rec["result"] = "none"
    with open(err, "rb") as f:
        words = f.read().decode("utf-8", "replace").split("\n")
    # a refusal's mirror line holds codes only: the box, `unable`, the name, the reason and its detail
    rec["stderr_codes"] = [w.split() for w in words if w.startswith("[/ ]")][:1]
    rec["stderr_hint_lines"] = sum(1 for w in words if w.startswith("hint: "))
    rec["hint_names_release"] = any("release" in w for w in words if w.startswith("hint: "))
    d.note(rec)
    return rec


def wait_loop(d, label, cursor, timeout_ms, planned, root):
    answered = []
    while True:
        rec, out, _ = d.verb(label, "wait", ["--after", str(cursor), "--timeout-ms", str(timeout_ms), "--json"], None)
        try:
            with open(out, encoding="utf-8") as f:
                ok = json.load(f).get("ok") or {}
        except (OSError, ValueError):
            ok = {}
        if rec["exit"] != 0 or not ok:
            rec["woke"] = "no-document"
            d.note(rec)
            return "no-document", cursor, answered
        if ok.get("timed_out") is True:
            rec["woke"] = "timed-out"
            d.note(rec)
            return "timed-out", cursor, answered
        event = ok.get("event") or {}
        kind = event.get("kind")
        cursor = ok.get("cursor", cursor)
        rec["woke"] = kind
        rec["cursor"] = cursor
        rec["event_ts"] = event.get("ts")
        data = event.get("data") if isinstance(event.get("data"), dict) else {}
        if kind in DIALOG_KINDS:
            rec["dialog_id"] = data.get("dialog_id")
            d.note(rec)
            response, rule, extra = decide(kind, data, planned, root)
            if response is None:
                d.note({"n": d.count() + 1, "label": label, "verb": "none", "dialog_id": data.get("dialog_id"), "kind": kind, "rule": rule, "t0": stamp()})
                return "dialog-unanswered:" + kind + ":" + str(data.get("dialog_id")), cursor, answered
            arec, _, _ = d.verb(label, "answer", [str(data.get("dialog_id"))], json.dumps(response).encode())
            arec.update({"dialog_id": data.get("dialog_id"), "kind": kind, "rule": rule})
            arec.update(extra)
            d.note(arec)
            answered.append({"dialog_id": data.get("dialog_id"), "kind": kind, "rule": rule, "answer_exit": arec["exit"]})
            continue
        d.note(rec)
        return kind, cursor, answered


def op_turn(d, argv):
    label, path = argv[0], argv[1]
    rest = argv[2:]
    timeout_ms = 900000
    planned = {}
    then = None
    i = 0
    while i < len(rest):
        if rest[i] == "--timeout-ms":
            timeout_ms = int(rest[i + 1]); i += 2
        elif rest[i] == "--question":
            planned["question"] = (rest[i + 1], rest[i + 2]); i += 3
        elif rest[i] == "--deny-true":
            planned["deny-true"] = True; i += 1
        elif rest[i] == "--plan":
            planned["plan"] = rest[i + 1]; i += 2
        elif rest[i] == "--then":
            then = (rest[i + 1], rest[i + 2]); i += 3
        else:
            sys.exit("usage: unknown option " + rest[i])
    root = os.getcwd()
    s = do_send(d, label, path)
    summary = {"label": label, "send_exit": s["exit"], "send_result": s["result"], "cursor": s.get("cursor"), "stderr_codes": s["stderr_codes"]}
    if s["exit"] == 0 and "cursor" in s:
        woke, cursor, answered = wait_loop(d, label, s["cursor"], timeout_ms, planned, root)
        summary.update({"woke": woke, "end_cursor": cursor, "dialogs": answered, "planned_left": sorted(planned.keys())})
        if then and woke == "turn-ended":
            s2 = do_send(d, then[0], then[1])
            second = {"label": then[0], "send_exit": s2["exit"], "send_result": s2["result"], "cursor": s2.get("cursor"), "stderr_codes": s2["stderr_codes"]}
            if s2["exit"] == 0 and "cursor" in s2:
                woke2, cursor2, answered2 = wait_loop(d, then[0], s2["cursor"], timeout_ms, {}, root)
                second.update({"woke": woke2, "end_cursor": cursor2, "dialogs": answered2})
            summary["then"] = second
    print(json.dumps(summary, sort_keys=True))


def op_send(d, argv):
    s = do_send(d, argv[0], argv[1])
    print(json.dumps({k: s.get(k) for k in ("label", "exit", "result", "cursor", "stderr_codes", "stderr_hint_lines", "hint_names_release", "events_before", "events_after")}, sort_keys=True))


def op_last(d, argv):
    label = argv[0]
    word = argv[1] if len(argv) > 1 else None
    rec, out, _ = d.verb(label, "last", [], None)
    with open(out, "rb") as f:
        text = f.read().decode("utf-8", "replace")
    stripped = text.strip()
    rec["text_bytes"] = len(text.encode("utf-8"))
    rec["non_empty"] = bool(stripped)
    rec["holds_closing_question"] = "Ready to continue?" in text
    rec["words"] = len(stripped.split())
    if word is not None:
        rec["is_the_word"] = stripped.strip(".!\"'` ").lower() == word.lower()
    d.note(rec)
    print(json.dumps({k: rec.get(k) for k in ("label", "exit", "text_bytes", "non_empty", "holds_closing_question", "words", "is_the_word")}, sort_keys=True))


def selftest():
    root = os.getcwd()
    t = SKILLS + "andromeda-new-session/../andromeda-tools/scripts"
    cases = [
        ("python -X utf8 %s/health.py check --root . --stack rust --style agent-driven" % t, True),
        ("T=%s; python -X utf8 $T/route.py cursor --root .; echo \"=== exit $?\"" % t, True),
        ("cd %s; git status --short; git log --oneline -10" % root, True),
        ("git branch --show-current; echo ---; git rev-list --count @{u}..HEAD 2>/dev/null || true", True),
        ("true", False),
        ("git branch -D x", False),
        ("python -X utf8 /usr/lib/x.py", False),
        ("python -X utf8 %s/evolve.py append --root . --count 1 --records-file - <<'NDJSON'\n{}\nNDJSON" % t, False),
        ("git status > out.txt", False),
        ("git log | head", False),
        ("rm -rf x", False),
        ("cd /; git status", False),
        ("echo $(id)", False),
        ("python -X utf8 %s/../../../../x.py" % t, False),
    ]
    bad = 0
    for cmd, want in cases:
        got = bash_is_safe(cmd, root)
        if got != want:
            bad += 1
            print("MISMATCH want %s got %s: case %d" % (want, got, cases.index((cmd, want))))
    r, rule, _ = decide("permission", {"tool": "Bash", "input": {"command": "true"}}, {"deny-true": True}, root)
    if not (r == DENY and rule == "planned"):
        bad += 1
        print("MISMATCH the planned deny")
    r, rule, _ = decide("permission", {"tool": "Write", "input": {}}, {}, root)
    if not (r == DENY and rule == "auto-deny"):
        bad += 1
        print("MISMATCH a write tool")
    r, rule, x = decide("question", {"questions": [{"question": "Which colour?", "options": ["red", "blue"]}]}, {"question": ("Which colour?", "blue")}, root)
    if not (r == {"answers": {"Which colour?": "blue"}} and x.get("asked_as_planned") is True):
        bad += 1
        print("MISMATCH the planned question")
    r, rule, _ = decide("plan", {"plan": "x"}, {}, root)
    if r is not None:
        bad += 1
        print("MISMATCH an unplanned plan")
    print("selftest: %d cases, %d mismatches" % (len(cases) + 4, bad))
    sys.exit(1 if bad else 0)


def main():
    if len(sys.argv) == 2 and sys.argv[1] == "selftest":
        selftest()
    if len(sys.argv) < 6:
        sys.exit(__doc__)
    priv, home, name, op = sys.argv[1:5]
    if not os.path.isdir(priv):
        sys.exit("no private directory")
    d = Drive(priv, home, name)
    if op == "send":
        op_send(d, sys.argv[5:])
    elif op == "turn":
        op_turn(d, sys.argv[5:])
    elif op == "last":
        op_last(d, sys.argv[5:])
    else:
        sys.exit(__doc__)


if __name__ == "__main__":
    main()
