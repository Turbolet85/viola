import json

with open("viola-0.1.0/working-route.md", encoding="utf-8") as fh:
    header = fh.read().split("\n")[103]
assert header.startswith("### Epoch 4"), header

TS = "2026-10-11T01:03:03Z"
BASE = {
    "v": 1,
    "ts": TS,
    "version": "viola-0.1.0",
    "epoch": header[4:],
    "chunk": "2026-10-10-session-links",
    "skill": "andromeda-wrap-session",
    "step": "gates",
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
        outcome="ok",
        counts={"halted": 0, "deferred": 0},
        consumed=[
            {
                "artifact": "plan",
                "quality": "ok",
                "note": "17 entries: 13 green, 0 red, entry 6 not run under --skip with the operator's reason (the "
                "held part's test file does not exist), 3 operator legs read from the chunk's evidence",
            },
            {"artifact": "matrix", "quality": "ok", "note": "claimed 0, gate no-op"},
            {"artifact": "master-route", "quality": "ok"},
        ],
        produced=[
            {"artifact": "matrix", "signals": ["claimed-0-no-op"]},
            {
                "artifact": "master-route",
                "signals": ["flip-exit-0", "desc-rewritten", "compacted:1"],
                "note": "route.py flip exited 0 and rewrote the description from the desc file, on the operator's "
                "direction; compact archived and stripped the one line, both of its blocks consumed by the chunk",
            },
            {
                "artifact": "git-state",
                "signals": ["pushed", "stamp-written", "light-gate-reproduced", "graph-refresh-done"],
                "note": "81 files, no source file; the code-graph refresh of the wrap's first session stood (rust "
                "5311 nodes and 28724 edges, ts 7 and 1); the branch pushed fast-forward and reads 0 ahead",
            },
        ],
        problem=[
            {
                "nature": "process",
                "solution": "unresolved",
                "note": "the gates playbook was read while the light gate was still running, before the step had "
                "completed, against the reading-order rule; the gate call was already fired and nothing of the step "
                "was changed after the read",
            },
            {
                "nature": "process",
                "solution": "overridden",
                "note": "the plan's own red-by-construction entry for the held part was not run: the operator "
                "directed its skip with the reason, since none of the light gate's assert arms fits an entry that is "
                "red while a plan step holds",
            },
        ],
    ),
]

print("\n".join(json.dumps(r, ensure_ascii=False) for r in records))
