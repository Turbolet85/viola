import json

with open("viola-0.1.0/working-route.md", encoding="utf-8") as fh:
    header = fh.read().split("\n")[103]
assert header.startswith("### Epoch 4"), header

TS = "2026-10-10T23:14:59Z"
BASE = {
    "v": 1,
    "ts": TS,
    "version": "viola-0.1.0",
    "epoch": header[4:],
    "chunk": "2026-10-10-budget-governor",
    "skill": "andromeda-wrap-session",
    "step": "curation",
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
        outcome="ok-degraded",
        counts={"dialogue_rounds": 0},
        consumed=[
            {
                "artifact": "conversation",
                "quality": "thin",
                "note": "the wrap ran in two sessions and the chunk in two implement runs; of four conversations "
                "only the resume's was in the window, the rest reached curation as ten candidates in "
                "resume-point.md",
            },
            {"artifact": "report", "quality": "ok"},
        ],
        produced=[
            {
                "artifact": "conversation",
                "signals": ["exact-hit:2", "applied:3", "dup:2", "below-threshold:3"],
                "note": "two candidates totalled exactly 0.6 and were carried by the no-other-home signal, its "
                "stated condition holding for each",
            }
        ],
        problem=[
            {
                "nature": "process",
                "solution": "workaround",
                "note": "the session-local scope could not hold: the conversations that held the corrections were "
                "cleared, so the candidates were taken from the resume point the earlier session wrote for this",
            }
        ],
    ),
    rec(
        "b",
        "friction",
        type="ambiguity.tier-routing",
        what="one survivor, a route rule the operator stated in a dialog answer, had no clean tier home: it is "
        "neither a safety rule nor path-scoped, and was written to Tier 1 on the precedent of the process rules "
        "already there",
        impact={"iterations": 1},
        artifacts=["CLAUDE.md"],
        evidence=".andromeda/runs/2026-10-10T22-36-31-wrap/curation.md",
    ),
]

print("\n".join(json.dumps(r, ensure_ascii=False) for r in records))
