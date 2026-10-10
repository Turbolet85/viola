import json

ROUTE = "viola-0.1.0/working-route.md"
with open(ROUTE, encoding="utf-8", newline="") as f:
    lines = f.read().split("\n")
header = lines[103]
assert header.startswith("### Epoch 4 "), header
epoch = header[4:].rstrip("\r")

TS = "2026-10-10T22:14:46Z"
rs = [
    {
        "v": 1,
        "kind": "step",
        "ts": TS,
        "id": TS + "-a",
        "version": "viola-0.1.0",
        "epoch": epoch,
        "chunk": "2026-10-10-budget-governor",
        "skill": "andromeda-phase",
        "step": "validate",
        "outcome": "ok",
        "counts": {"dialogue_rounds": 0, "iterations": 0, "halted": 0},
        "consumed": [
            {
                "artifact": "plan",
                "quality": "ok",
                "note": "no check failed or warned: the fence parsed 16 entries, planlint 0 hits, 28 modify paths and 3 new files present; the whole set ran after the last plan edit and before the prompt",
            },
            {"artifact": "scope", "quality": "ok", "note": "val-1 aligned: step 15 against the revision section P3 wrote"},
            {"artifact": "matrix", "quality": "ok", "note": "0 claimed by the marker before and after; nothing to release"},
        ],
        "produced": [
            {
                "artifact": "matrix",
                "signals": [],
                "note": "coverage and audit fired after the word as the approval fact; no claim, no note written",
            },
            {
                "artifact": "scope",
                "signals": ["approval-recorded"],
                "note": "one bullet citing the approval word (inputs#I7), written after the word; not a val-1 amendment",
            },
        ],
        "problem": None,
        "note": "a revision's P5: the prompt was issued as text; one yes whose three remarks changed no plan line; no baseline run, every gate entry carried unchanged",
    }
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
