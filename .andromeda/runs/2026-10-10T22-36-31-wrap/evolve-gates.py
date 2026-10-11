import json

with open("viola-0.1.0/working-route.md", encoding="utf-8") as fh:
    header = fh.read().split("\n")[103]
assert header.startswith("### Epoch 4"), header

TS = "2026-10-10T23:21:06Z"
record = {
    "v": 1,
    "kind": "step",
    "ts": TS,
    "id": TS + "-a",
    "version": "viola-0.1.0",
    "epoch": header[4:],
    "chunk": "2026-10-10-budget-governor",
    "skill": "andromeda-wrap-session",
    "step": "gates",
    "outcome": "ok",
    "counts": {"halted": 0, "deferred": 0},
    "consumed": [
        {"artifact": "plan", "quality": "ok"},
        {"artifact": "matrix", "quality": "ok"},
        {"artifact": "master-route", "quality": "ok"},
    ],
    "produced": [
        {
            "artifact": "matrix",
            "signals": ["claimed:0", "gate-no-op"],
            "note": "the chunk claimed no capability; the version line stands at verified 16 of 53",
        },
        {
            "artifact": "master-route",
            "signals": ["flip:pending-complete", "flip-exit:0", "compacted:1", "pins-stripped:4", "pins-moved-before:2"],
            "note": "the dry run's four pin rows were all CARRYs this chunk consumed or the wrap had already moved",
        },
        {
            "artifact": "git-state",
            "signals": [
                "light-gate:green-13-of-16",
                "not-run-operator:3",
                "hygiene:clean",
                "scope:clean",
                "new-text:unchanged",
                "staged:113",
                "stamp-written",
                "pushed",
                "graph:done",
            ],
            "note": "the 13 run entries reproduced implement's verdict on the wrapped tree; the three operator legs "
            "were read from the recorded pass; the graph refresh was done at this wrap's Setup, rust 5215 nodes "
            "and 27952 edges, ts 7 and 1",
        },
    ],
    "problem": None,
}

print(json.dumps(record, ensure_ascii=False))
