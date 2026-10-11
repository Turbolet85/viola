import json

with open("viola-0.1.0/working-route.md", encoding="utf-8") as fh:
    header = fh.read().split("\n")[103]
assert header.startswith("### Epoch 4"), header

TS = "2026-10-11T00:55:45Z"
BASE = {
    "v": 1,
    "ts": TS,
    "version": "viola-0.1.0",
    "epoch": header[4:],
    "chunk": "2026-10-10-session-links",
    "skill": "andromeda-wrap-session",
    "step": "reconcile",
}
RUN = ".andromeda/runs/2026-10-11T00-33-08-wrap/"


def rec(letter, kind, **fields):
    out = dict(BASE)
    out["kind"] = kind
    out["id"] = TS + "-" + letter
    out.update(fields)
    return out


DOCS = ["architecture", "security-plan", "design-system", "layout-templates", "test-plan", "obs-plan", "a11y-plan"]
AMENDED = ["architecture", "security-plan", "test-plan", "obs-plan", "a11y-plan"]

step = rec(
    "a",
    "step",
    outcome="ok",
    counts={"retries": 0, "dialogue_rounds": 0, "halted": 0, "extra_reads": 3},
    consumed=[
        {
            "artifact": "report",
            "quality": "thin",
            "note": "27 proposals, every source coordinate found in the report's bullets or its New text section; three "
            "detectors named a fact the report does not carry, and the orchestrator read source for each (whether the "
            "link append signals the wait feed; what the folded budget case reads; where the restore stands against "
            "the strict-modes check, left unclaimed)",
        }
    ]
    + [{"artifact": "spec:" + d, "quality": "ok"} for d in DOCS]
    + [{"artifact": "drift-base", "quality": "ok"}, {"artifact": "wrap-playbook", "quality": "ok"}],
    produced=[{"artifact": "spec:" + d, "signals": ["cascade-wide"]} for d in AMENDED]
    + [{"artifact": "sidecar:" + d, "signals": []} for d in AMENDED]
    + [
        {
            "artifact": "fanout-results",
            "signals": ["rejected-for-source:0", "rejected-for-coordinate:0", "escalation-settled-by-direction:1"],
            "note": "27 proposals applied and 2 orchestrator-raised; the one escalate-by-rule class (held items landed "
            "on the operator's word) was settled by the resume directions with no halt; proposals per doc: "
            "architecture 12, security-plan 2, design-system 0, layout-templates 0, test-plan 6, obs-plan 5, "
            "a11y-plan 2; ten leaves re-derived",
        }
    ],
    problem=[
        {
            "nature": "process",
            "solution": "workaround",
            "note": "master lines of 2 000 to 12 000 chars were read through a scratch script that prints one line or "
            "a slice of it, in place of cascade.py window, whose wrapped output cannot feed the Edit tool an exact "
            "string",
        }
    ],
)

gap = rec(
    "b",
    "friction",
    type="input.report-insufficient",
    what="three detectors flagged a fact the report does not state (the link append bumping no wait-feed generation; "
    "whether the folded closed-gate case reads the terminal stream; the restore's place against the strict-modes "
    "check); two were settled by a source read at validate and the third was left unclaimed in the body",
    impact={"extra_reads": 3},
    artifacts=["report.md", "fanout-results.md"],
    evidence=RUN + "fanout-results.md",
)

miss = rec(
    "c",
    "friction",
    type="contract.cascade-miss",
    what="the leaf services/viola-state.md said the snapshot replay is a function no reader calls yet, stale since the "
    "revive chunk gave it a product caller two wraps ago; found on the line this pass re-derived for another claim",
    impact={"extra_reads": 0},
    artifacts=[".claude/docs/services/viola-state.md:28"],
    evidence=RUN + "cascade-dispositions.md",
)

fix = rec(
    "d",
    "friction",
    type="contract.in-pass-correction",
    what="two first writes corrected before the commit: test-plan Path 6 said an eighth root case where the count of "
    "path6 cases was not taken (re-read, now one more root case), and fanout-results said 19 hits read where the "
    "listing held 18 (a recount)",
    impact={"iterations": 2},
    artifacts=[".andromeda/test-plan.md", "fanout-results.md"],
    evidence=RUN + "fanout-results.md",
)

print("\n".join(json.dumps(r, ensure_ascii=False) for r in [step, gap, miss, fix]))
