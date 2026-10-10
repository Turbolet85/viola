import json

ROUTE = "viola-0.1.0/working-route.md"
with open(ROUTE, encoding="utf-8", newline="") as f:
    lines = f.read().split("\n")
header = lines[103]
assert header.startswith("### Epoch 4 "), header
epoch = header[4:].rstrip("\r")

TS = "2026-10-10T22:12:51Z"
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
        "step": "plan",
        "outcome": "ok",
        "counts": {"dialogue_rounds": 0, "iterations": 0},
        "consumed": [
            {"artifact": "research", "quality": "ok"},
            {"artifact": "scope", "quality": "ok", "note": "the revision section written at P3 is what step 15 was synthesized from"},
        ],
        "produced": [
            {
                "artifact": "plan",
                "signals": ["revision-one-step", "authority-resolved:the-hold-yielded-to-the-operator-word"],
                "note": "step 15 rewritten, 11 lines to 23; the fence, the criteria and the touchpoints byte-unchanged; gate dry-run parsed 16 entries and planlint read 0 hits on the first fire; the extracts were not re-read",
            }
        ],
        "problem": [
            {
                "nature": "process",
                "solution": "overridden",
                "note": "the letter asks a fork whose option ships an escalate pattern (the budget.json widening landing with the operator pass no longer held; playbook 'Boundary widening - what ratifies it') as a P4 question naming the pattern; no round was asked because the invocation's word had already chosen the option, so the plan records the pattern, the answerer and that it is no ratification, and the review card names it",
            },
            {
                "nature": "process",
                "solution": "workaround",
                "note": "the word limits the edit to step 15, while the Goal's last paragraph, the Test Commands prose, three operator entries' notes and two Expected-amendments lines still word the hold or a founder ruling at the wrap; step 15 carries a clause that it governs those sentences in place of editing them, and the review card lists them",
            },
        ],
    }
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
