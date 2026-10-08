"""Known-verdict controls for the two reader entries the revision changed.

Reads the plan's own fence, takes the `run` of the ledger reader and of the run reader as written, and fires each
over hand-minted lines in p5-controls/: one pass file, and one file per property the reader must refuse.
"""
import copy
import json
import pathlib
import subprocess
import sys
import tomllib

plan = pathlib.Path(sys.argv[1])
out = pathlib.Path(sys.argv[2])
out.mkdir(exist_ok=True)
text = plan.read_text(encoding="utf-8")
sec = text.split("## Test Commands", 1)[1]
fence = sec.split("```toml", 1)[1].split("```", 1)[0]
gates = tomllib.loads(fence)["gate"]
EVID = "viola-0.1.0/chunks/2026-10-08-first-live-test-and-self-drive/evidence/"


def reader(name):
    hits = [g["run"] for g in gates if g["run"].startswith("jq -e -s") and g["run"].endswith(EVID + name)]
    assert len(hits) == 1, (name, len(hits))
    return hits[0]


def fire(run, name, path):
    cmd = run.replace(EVID + name, str(path)) if path else run
    return subprocess.run(["bash", "-o", "pipefail", "-c", cmd], capture_output=True, cwd=plan.parents[3]).returncode


def write(path, lines):
    path.write_text("".join(json.dumps(l, separators=(",", ":")) + "\n" for l in lines), encoding="utf-8")


LEDGER = [
    {"kind": "compositor", "event": "start", "n": 1, "own_instance": True, "desktop_lock_shell": True,
     "desktop_lock_compositor": True, "shell_first_answer": "no-answer", "shell_relaunched": True},
    {"kind": "key-probe", "typed_into": "probe", "compositor": "own", "own_instance": True, "guard": True,
     "wheel_cause": "human-input", "focus_wheel_delta": 0, "send_exit": 10, "live_starts_before": 0,
     "cols": 97, "rows": 49, "font_size": 8},
    {"kind": "census", "when": "before", "own_left": 0},
] + [{"kind": "start", "n": n, "session": "verify", "cli": "2.1.287", "build": "harness"} for n in range(1, 6)] + [
    {"kind": "round", "fired": 1, "home": "viola-live-1", "stamped": "2.1.287", "pass": 17, "fail": 0},
    {"kind": "census", "when": "after", "own_left": 0},
    {"kind": "start", "n": 6, "session": "live-run", "cli": "2.1.287", "build": "release-check"},
    {"kind": "census", "when": "after", "own_left": 0},
    {"kind": "compositor", "event": "end", "desktop_lock_shell": True, "desktop_lock_compositor": True,
     "shell_first_answer": "true", "shell_relaunched": False, "own_left": 0},
]

DIALOG = {"raised": True, "answer_exit": 0, "decision_emitted": True, "deadline_hit": False, "wheel_records": 0,
          "keys_typed": 0, "turn_ended": True}
RUN = [
    {"kind": "step", "id": "snapshot", "cli_version": "2.1.287", "cli_verified": True, "cols": 97, "rows": 49},
    {"kind": "step", "id": "skill-sent", "exit": 0, "confirmed": True},
    {"kind": "step", "id": "turn-ended", "exit": 0},
    {"kind": "step", "id": "last-read", "exit": 0, "non_empty": True},
    {"kind": "step", "id": "review-answered", "exit": 0, "confirmed": True},
    {"kind": "step", "id": "clear", "exit": 0, "confirmed": True, "new_session": True},
    {"kind": "step", "id": "skill-2", "exit": 0, "confirmed": True},
    dict({"kind": "step", "id": "dialog-question", "kind_raised": "question"}, **DIALOG),
    dict({"kind": "step", "id": "dialog-permission", "kind_raised": "permission", "behavior": "deny"}, **DIALOG),
    dict({"kind": "step", "id": "dialog-plan", "kind_raised": "plan"}, **DIALOG),
    {"kind": "step", "id": "takeover", "wheel_after_focus": 0, "compositor": "own", "guard_at_key": True,
     "holder": "human", "cause": "human-input", "send_exit": 10, "hint_names_release": False, "send_issued": False},
]


def mut(lines, pick, **change):
    new = copy.deepcopy(lines)
    for l in new:
        if pick(l):
            for k, v in change.items():
                if v is None:
                    l.pop(k, None)
                else:
                    l[k] = v
    return new


def drop(lines, pick):
    return [l for l in copy.deepcopy(lines) if not pick(l)]


kp = lambda l: l["kind"] == "key-probe"
cs = lambda l: l["kind"] == "compositor" and l["event"] == "start"
ce = lambda l: l["kind"] == "compositor" and l["event"] == "end"
st = lambda l: l["kind"] == "start"
rd = lambda l: l["kind"] == "round"
live = lambda l: l["kind"] == "start" and l["session"] == "live-run"
extra = [{"kind": "start", "n": n, "session": "readings", "cli": "2.1.287", "build": "release-check"} for n in (7, 8, 9)]

ledger_controls = {
    "five-starts": drop(LEDGER, live),
    "nine-starts": LEDGER + extra,
    "a-2.1.289-start": mut(LEDGER, lambda l: st(l) and l["n"] == 3, cli="2.1.289"),
    "round-red": mut(LEDGER, rd, **{"pass": 16, "fail": 1}),
    "round-on-2.1.289": mut(LEDGER, rd, stamped="2.1.289"),
    "round-fired-twice": mut(LEDGER, rd, fired=2),
    "no-key-probe": drop(LEDGER, kp),
    "key-into-another-window": mut(LEDGER, kp, typed_into="other"),
    "key-probe-after-a-start": mut(LEDGER, kp, live_starts_before=1),
    "focus-moved-the-wheel": mut(LEDGER, kp, focus_wheel_delta=1),
    "key-in-the-desktop-compositor": mut(LEDGER, kp, compositor="desktop"),
    "key-probe-not-on-the-own-instance": mut(LEDGER, kp, own_instance=False),
    "key-with-the-guard-failed": mut(LEDGER, kp, guard=False),
    "terminal-26-by-14": mut(LEDGER, kp, cols=26, rows=14),
    "terminal-size-as-text": mut(LEDGER, kp, cols="97", rows="49"),
    "terminal-size-absent": mut(LEDGER, kp, cols=None, rows=None),
    "no-compositor-start-line": drop(LEDGER, cs),
    "two-compositor-starts": LEDGER + [l for l in LEDGER if cs(l)],
    "no-compositor-end-line": drop(LEDGER, ce),
    "shell-reads-unlocked-after-the-start": mut(LEDGER, cs, desktop_lock_shell=False),
    "desktop-instance-reads-unlocked-after-the-start": mut(LEDGER, cs, desktop_lock_compositor=False),
    "shell-reads-unlocked-at-the-end": mut(LEDGER, ce, desktop_lock_shell=False),
    "desktop-instance-reads-unlocked-at-the-end": mut(LEDGER, ce, desktop_lock_compositor=False),
    "lock-reading-absent-at-the-end": mut(LEDGER, ce, desktop_lock_shell=None),
    "something-of-the-chunk-left-at-the-end": mut(LEDGER, ce, own_left=1),
    "live-run-on-the-harness-build": mut(LEDGER, live, build="harness"),
    "a-probe-dir-left": mut(LEDGER, lambda l: l["kind"] == "census" and l["when"] == "after", own_left=1),
}

sid = lambda i: (lambda l: l["id"] == i)
dlg = lambda l: l["id"].startswith("dialog-")
run_controls = {
    "no-takeover": drop(RUN, sid("takeover")),
    "no-clear": drop(RUN, sid("clear")),
    "unverified": mut(RUN, sid("snapshot"), cli_verified=False),
    "skill-not-confirmed": mut(RUN, sid("skill-sent"), confirmed=False),
    "clear-without-a-new-session": mut(RUN, sid("clear"), new_session=False),
    "no-plan-dialog-step": drop(RUN, sid("dialog-plan")),
    "plan-dialog-not-raised": mut(RUN, sid("dialog-plan"), raised=False),
    "permission-answered-allow": mut(RUN, sid("dialog-permission"), behavior="allow"),
    "question-step-of-another-kind": mut(RUN, sid("dialog-question"), kind_raised="permission"),
    "decision-past-the-deadline": mut(RUN, sid("dialog-question"), deadline_hit=True),
    "a-wheel-record-during-a-dialog": mut(RUN, sid("dialog-permission"), wheel_records=1),
    "a-key-during-a-dialog": mut(RUN, sid("dialog-plan"), keys_typed=1),
    "focus-took-the-wheel": mut(RUN, sid("takeover"), wheel_after_focus=1),
    "takeover-key-in-the-desktop-compositor": mut(RUN, sid("takeover"), compositor="desktop"),
    "takeover-with-the-guard-failed": mut(RUN, sid("takeover"), guard_at_key=False),
    "takeover-guard-absent": mut(RUN, sid("takeover"), guard_at_key=None),
    "send-not-refused": mut(RUN, sid("takeover"), send_exit=0),
    "hint-names-release": mut(RUN, sid("takeover"), hint_names_release=True),
    "takeover-by-manual-pause": mut(RUN, sid("takeover"), cause="manual-pause"),
}

rows = []
for label, name, pass_lines, controls in (
    ("live-sessions", "live-sessions.ndjson", LEDGER, ledger_controls),
    ("live-run", "live-run.ndjson", RUN, run_controls),
):
    run = reader(name)
    rows.append(f"{label}: baseline (as written) exit {fire(run, name, None)}")
    p = out / f"{label}-pass.ndjson"
    write(p, pass_lines)
    rows.append(f"{label}: pass file exit {fire(run, name, p)}")
    for c, lines in controls.items():
        p = out / f"{label}-{c}.ndjson"
        write(p, lines)
        rows.append(f"{label}: {c} exit {fire(run, name, p)}")
(out / "verdicts.txt").write_text("\n".join(rows) + "\n", encoding="utf-8")
print("\n".join(rows))
bad = [r for r in rows if ("pass file" in r and not r.endswith("exit 0")) or ("baseline" not in r and "pass file" not in r and not r.endswith("exit 1"))]
print("controls:", len(rows) - 4, "refusals expected exit 1 ·", "unexpected", len(bad))
for b in bad:
    print("UNEXPECTED", b)
