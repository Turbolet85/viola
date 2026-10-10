import json

with open("viola-0.1.0/working-route.md", encoding="utf-8") as fh:
    header = fh.read().split("\n")[103]
assert header.startswith("### Epoch 4"), header

TS = "2026-10-10T22:45:21Z"
BASE = {
    "v": 1,
    "ts": TS,
    "version": "viola-0.1.0",
    "epoch": header[4:],
    "chunk": "2026-10-10-budget-governor",
    "skill": "andromeda-wrap-session",
    "step": "report",
}


def rec(letter, kind, **fields):
    out = dict(BASE)
    out["kind"] = kind
    out["id"] = TS + "-" + letter
    out.update(fields)
    return out


rs = [
    rec(
        "a",
        "step",
        outcome="ok",
        counts={"extra_reads": 14},
        consumed=[
            {
                "artifact": "conversation",
                "quality": "thin",
                "note": "this window held the operator pass and its report; the run that wrote steps 1 to 14 ended before a clear, so none of its conversation was here",
            },
            {
                "artifact": "implement-outcome",
                "quality": "thin",
                "note": "the second implement run's report stood as given; the first run's P4 report was not in the window, and its 11 friction-ledger records with evidence/red-green.md stood in for its deviations and its one surfaced item, each re-read in the diff",
            },
            {
                "artifact": "operator-directive",
                "quality": "ok",
                "note": "the wrap invocation (inputs I9): Phase 1 only, then a resume point; it changed no fact of the report",
            },
            {
                "artifact": "operator-pass",
                "quality": "ok",
                "signals": ["as-left", "commits:1", "red:0"],
                "note": "the commit list and evidence/operator-pass.md carried every result",
            },
            {"artifact": "git-state", "quality": "ok"},
            {
                "artifact": "plan",
                "quality": "ok",
                "note": "16 expected-amendment entries dispositioned; one amendment the list does not name was raised (the start's must-trace span list)",
            },
        ],
        produced=[
            {
                "artifact": "report",
                "signals": ["reconstructed", "new-text-generated", "new-text-rows:265"],
                "note": "every Changes family was filled by reading the diff and the masters' cited lines, not from session knowledge of the code",
            }
        ],
        problem=[
            {
                "nature": "process",
                "solution": "overridden",
                "note": "the operator's word with the invocation ends this wrap session after Phase 1; the reconcile runs in a fresh context from a resume point",
            },
            {
                "nature": "process",
                "solution": "workaround",
                "note": "multi-KB master lines were read through scratch scripts written for this report (a per-master hit counter, a bounded window printer, a heading finder) where cascade.py window is the documented reader",
            },
        ],
    ),
    rec(
        "b",
        "friction",
        type="recall.change-reconstruction",
        what="the report's six Changes families were reconstructed from the 3191-line diff, the plan, research and scope, because the code was written by an earlier run whose conversation a clear had ended; three diff files, the new module and the masters' cited lines were read for it",
        impact={"extra_reads": 14},
        artifacts=["viola-0.1.0/chunks/2026-10-10-budget-governor/report.md"],
        evidence=".andromeda/runs/2026-10-10T22-36-31-wrap/new-text.md",
    ),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
