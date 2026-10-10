import json

ROUTE = "viola-0.1.0/working-route.md"
with open(ROUTE, encoding="utf-8", newline="") as f:
    lines = f.read().split("\n")
header = lines[103]
assert header.startswith("### Epoch 4 "), header
epoch = header[4:].rstrip("\r")

TS = "2026-10-10T22:11:38Z"
MARKER = "2026-10-10-budget-governor"
base = {
    "v": 1,
    "ts": TS,
    "version": "viola-0.1.0",
    "epoch": epoch,
    "chunk": MARKER,
    "skill": "andromeda-phase",
    "step": "research",
}
rs = [
    dict(
        base,
        kind="step",
        id=TS + "-a",
        outcome="ok",
        counts={"reformulations": 0, "extra_reads": 2},
        consumed=[
            {
                "artifact": "source",
                "quality": "ok",
                "note": "a revision: P3 ran as premise closure only; the four held items' built forms read at their sites, no graph query, no extract re-read",
            }
        ],
        produced=[
            {
                "artifact": "research",
                "signals": ["revision-closure", "lists-unchanged"],
                "note": "one section added, the External inputs slot gained I6; gate.py scope --lists-only parsed 33 items on the first fire",
            },
            {
                "artifact": "scope",
                "signals": ["premise-corrected"],
                "note": "one section added for the operator's word (inputs#I6) and one bullet corrected: the last clause of the earlier approval word is superseded",
            },
        ],
        problem=None,
        note="P1 and P2 did not run (skill body, Revision); open questions carried to P4: 0",
    ),
    dict(
        base,
        kind="friction",
        id=TS + "-b",
        type="contract.narrow-basis-claim",
        what="a grep hit on check_instance inside the helper trusted (src/cmd/revive.rs:123) was written into research.md as the site where revive reads its check ahead of open_wrapper_log at :108; a read of the function body gave the call site :104, and the line was corrected before the lists check",
        impact={"extra_reads": 2},
        artifacts=["research.md"],
        evidence="viola-0.1.0/chunks/2026-10-10-budget-governor/research.md, section Revision",
    ),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
