import json

with open("viola-0.1.0/working-route.md", encoding="utf-8") as fh:
    header = fh.read().split("\n")[103]
assert header.startswith("### Epoch 4"), header

TS = "2026-10-11T00:58:46Z"
BASE = {
    "v": 1,
    "ts": TS,
    "version": "viola-0.1.0",
    "epoch": header[4:],
    "chunk": "2026-10-10-session-links",
    "skill": "andromeda-wrap-session",
    "step": "route-resolve",
}
RUN = ".andromeda/runs/2026-10-11T00-33-08-wrap/"


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
        counts={"dialogue_rounds": 0, "halted": 0, "extra_reads": 2},
        consumed=[
            {"artifact": "report", "quality": "ok"},
            {
                "artifact": "working-route",
                "quality": "ok",
                "note": "the frozen line's two CARRYs read whole, both discharged by the chunk; the head entry's ten "
                "blocks read whole before the eleventh was appended",
            },
        ],
        produced=[
            {
                "artifact": "working-route",
                "signals": ["tail-reshaped", "carry-pins:1", "citation-repoint:1", "trajectory-settled-by-direction:1"],
                "note": "one CARRY of 2935 chars on the head entry for the held part of the plan, placed by the "
                "operator's recorded direction naming the entry and the disposition, so no halt; one route citation's "
                "digits re-pointed after the cited line was read; two hunks, both markerless lines",
            }
        ],
        problem=None,
    ),
    rec(
        "b",
        "friction",
        type="contract.no-sanctioned-channel",
        what="a fact a detector surfaced for the operator's judgment (the link rule takes a logged driver and driven "
        "as strings and does not parse them as names again) has no owner entry and the directions name no disposition "
        "for it; it was written into the master as a fact and listed on the handoff for the operator's word, no CARRY "
        "minted",
        impact={"extra_reads": 0},
        artifacts=["viola-0.1.0/working-route.md", ".claude/session-handoff.md"],
        evidence=RUN + "route-record.md",
    ),
]

print("\n".join(json.dumps(r, ensure_ascii=False) for r in records))
