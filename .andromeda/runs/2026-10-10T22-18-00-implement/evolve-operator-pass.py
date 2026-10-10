import json

with open("viola-0.1.0/working-route.md", encoding="utf-8") as fh:
    header = fh.read().split("\n")[103]
assert header.startswith("### Epoch 4"), header

TS = "2026-10-10T22:34:51Z"
BASE = {
    "v": 1,
    "ts": TS,
    "version": "viola-0.1.0",
    "epoch": header[4:],
    "chunk": "2026-10-10-budget-governor",
    "skill": "andromeda-implement",
}


def rec(letter, step, **fields):
    out = dict(BASE)
    out["kind"] = "step"
    out["step"] = step
    out["id"] = TS + "-" + letter
    out.update(fields)
    return out


rs = [
    rec(
        "a",
        "fix-loop",
        outcome="ok",
        counts={"iterations": 0, "retries": 0, "soft_exit": 0, "deferred": 0},
        consumed=[
            {
                "artifact": "plan",
                "quality": "ok",
                "note": "re-entry at step 16: entry 13 through the tool, entry 14 as written twice, entry 15 through --operator, entry 16 as written; each ran as written and read green on its first firing",
            },
            {"artifact": "research", "quality": "ok"},
            {"artifact": "tests", "quality": "ok", "note": "CI on the pre-CI commit read green on attempt 1, 15 of 15 checks; no fix commit"},
            {"artifact": "source", "quality": "ok", "note": "no source file written in this run; none newer than the earlier run's last green trail"},
        ],
        produced=[
            {
                "artifact": "source",
                "signals": ["guard-refused:1"],
                "note": "the bash guard's heredoc-with-a-file-target arm refused one call while the operator-pass record was being extended; the append was re-issued once through the Edit tool and landed",
            },
            {"artifact": "matrix", "signals": [], "note": "the chunk claims no capability; no ref written"},
            {"artifact": "implement-outcome", "signals": ["green"], "note": "the operator pass complete on one commit and one CI run"},
        ],
        problem=[
            {
                "nature": "process",
                "solution": "overridden",
                "note": "the letter keeps a commit and a push out of implement; the operator's word with the invocation directed the pre-CI commit and the guarded push entry, so this run made both and recorded the word as an input",
            },
            {
                "nature": "process",
                "solution": "workaround",
                "note": "the whole block was not fired again at re-entry: its green reading is the earlier run's, on a source tree read unchanged since; only the pre-push entry and the three operator entries ran here",
            },
            {
                "nature": "environment",
                "solution": "workaround",
                "note": "two foreground waits on a backgrounded call's output file used a bounded read-timeout loop, the harness blocking a foreground sleep",
            },
        ],
    ),
    rec(
        "b",
        "smoke",
        outcome="ok",
        counts={"retries": 0, "deferred": 0},
        consumed=[
            {"artifact": "plan", "quality": "ok"},
        ],
        produced=[
            {
                "artifact": "conversation",
                "signals": ["recorded-not-rerun"],
                "note": "the smoke entries 7 to 9 and 12 read green in the earlier run of this chunk on the same source tree; this re-entry at step 16 did not drive them again",
            },
            {"artifact": "implement-outcome", "signals": ["green"]},
        ],
        problem=None,
    ),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
