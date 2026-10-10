import json

with open("viola-0.1.0/working-route.md", encoding="utf-8") as fh:
    header = fh.read().split("\n")[103]
assert header.startswith("### Epoch 4"), header

TS = "2026-10-10T23:17:00Z"
BASE = {
    "v": 1,
    "ts": TS,
    "version": "viola-0.1.0",
    "epoch": header[4:],
    "chunk": "2026-10-10-budget-governor",
    "skill": "andromeda-wrap-session",
    "step": "route-resolve",
}


def rec(letter, kind, **fields):
    out = dict(BASE)
    out["kind"] = kind
    out["id"] = TS + "-" + letter
    out.update(fields)
    return out


records = [
    rec(
        "a",
        "step",
        outcome="halted-resolved",
        counts={"dialogue_rounds": 1, "halted": 1, "extra_reads": 2},
        consumed=[
            {"artifact": "report", "quality": "ok"},
            {
                "artifact": "working-route",
                "quality": "ok",
                "note": "the frozen line's four CARRY blocks named their own next owners or left one open; three "
                "markerless blocks restated facts this chunk changed (a preflight order, a row's owner, a cited line)",
            },
        ],
        produced=[
            {
                "artifact": "working-route",
                "signals": ["tail-reshaped", "carry-pins:6", "prereq:0", "reorders:0", "stale-text-fixed:3"],
                "note": "the one trajectory question (a test owner) was asked together with the Phase 2 escalation, "
                "in one dialog; two placements were applied on a recorded direction naming entry and disposition",
            }
        ],
        problem=[
            {
                "nature": "process",
                "solution": "overridden",
                "note": "the wrap recommended the entry that takes the rest of Path 6 as the test's owner; the "
                "operator chose the head, on the rule that a missing test is folded into the next chunk",
            }
        ],
    ),
    rec(
        "b",
        "friction",
        type="contract.carry-no-owner",
        what="one of the frozen entry's four CARRYs (an owed ledger row and its probe) was left with no next owner "
        "by the plan and by the word that lifted its hold; the owner arrived with the wrap's resume directions",
        impact={"dialogue_rounds": 0},
        artifacts=["viola-0.1.0/working-route.md:128"],
        evidence=".andromeda/runs/2026-10-10T22-36-31-wrap/route-record.md",
    ),
]

print("\n".join(json.dumps(r, ensure_ascii=False) for r in records))
