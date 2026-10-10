import json

with open("viola-0.1.0/working-route.md", encoding="utf-8") as fh:
    header = fh.read().split("\n")[103]
assert header.startswith("### Epoch 4"), header

TS = "2026-10-10T23:13:01Z"
BASE = {
    "v": 1,
    "ts": TS,
    "version": "viola-0.1.0",
    "epoch": header[4:],
    "chunk": "2026-10-10-budget-governor",
    "skill": "andromeda-wrap-session",
    "step": "reconcile",
}
RUN = ".andromeda/runs/2026-10-10T22-36-31-wrap/"


def rec(letter, kind, **fields):
    out = dict(BASE)
    out["kind"] = kind
    out["id"] = TS + "-" + letter
    out.update(fields)
    return out


DOCS = ["architecture", "security-plan", "design-system", "layout-templates", "test-plan", "obs-plan", "a11y-plan"]

step = rec(
    "a",
    "step",
    outcome="halted-resolved",
    counts={"retries": 0, "dialogue_rounds": 1, "halted": 1, "extra_reads": 4},
    consumed=[
        {
            "artifact": "report",
            "quality": "ok",
            "note": "76 proposals, every source coordinate found in the report's bullets or its New text section; the "
            "report's fixed-string searches under-ran the masters at 4 phrasings the detectors or the sweep reached "
            "(four preflight readings, its third reading twice, reuses strict-modes-failed at exit 1)",
        }
    ]
    + [{"artifact": "spec:" + d, "quality": "ok"} for d in DOCS]
    + [{"artifact": "drift-base", "quality": "ok"}, {"artifact": "wrap-playbook", "quality": "ok"}],
    produced=[
        {
            "artifact": "spec:" + d,
            "signals": ["cascade-wide"] if d != "security-plan" else ["cascade-wide", "escalation-shaped"],
        }
        for d in DOCS
    ]
    + [
        {
            "artifact": "sidecar:all",
            "signals": ["entries:13", "docs:7", "on-form:13", "lazy-creates:0", "read-back:13"],
        },
        {
            "artifact": "fanout-results",
            "signals": [
                "proposals:76",
                "applied:76",
                "rejected-for-source:0",
                "rejected-for-coordinate:0",
                "escalations-resolved:3",
                "playbook-rule-appended:1",
                "respawns:0",
            ],
            "note": "per doc: architecture 22, obs-plan 23, test-plan 21, security-plan 6, layout-templates 2, "
            "a11y-plan 2, design-system 0; routine 70, escalate-by-rule 3 (the budget.json widening twice, the "
            "ledger row's owner), escalate-by-unease 3 (one dependent group), cross-contradiction 0",
        },
        {
            "artifact": "cascade-dispositions",
            "signals": ["patterns:28", "rows:94", "controls-fired:28", "sweeps:2", "leaves-rederived:16"],
            "note": "the tool's control check passed for all 28 patterns on the pre-pass text; 7 patterns read 0 "
            "rows after the pass",
        },
    ],
    problem=[
        {
            "nature": "process",
            "solution": "workaround",
            "note": "four masters (design-system, layout-templates, a11y-plan, security-plan) were amended while "
            "other detectors were still running and before fanout-results.md held any disposition: the wait on the "
            "background batch was filled with the apply of the docs whose own detector had returned",
        },
        {
            "nature": "process",
            "solution": "workaround",
            "note": "exact text of multi-KB master lines for the Edit tool was read through a scratch script that "
            "prints unwrapped windows; cascade.py window wraps its output and cannot feed an Edit's old_string",
        },
        {
            "nature": "process",
            "solution": "workaround",
            "note": "one leaf edit (docs/stack.md) went through an inline python heredoc where the Edit tool is the "
            "project's documented path",
        },
    ],
)

frictions = [
    rec(
        "b",
        "friction",
        type="contract.cascade-miss",
        what="four standing master sentences restating a retired claim were proposed by no detector and found only "
        "by the step-2 sweep: obs-plan (the preflight's third reading), design-system (the revive refusal reusing "
        "strict-modes-failed at exit 1), test-plan (the release --budget result; two cfg(unix) root cases)",
        impact={"extra_reads": 4},
        artifacts=["spec:obs-plan", "spec:design-system", "spec:test-plan"],
        evidence=RUN + "cascade-dispositions.md",
    ),
    rec(
        "c",
        "friction",
        type="ambiguity.playbook-no-match",
        what="three test-plan proposals narrowing a NEVER ban (a hand-written budget.json) matched no playbook rule: "
        "the detector returned them as escalate, main was uneasy, and the operator scoped the rule in one answer",
        impact={"dialogue_rounds": 1, "halted": 1},
        artifacts=["spec:test-plan", "wrap-playbook"],
        evidence=RUN + "fanout-results.md",
    ),
    rec(
        "d",
        "friction",
        type="contract.in-pass-correction",
        what="one site: the layout-templates verb block's first write carried an example override end with a "
        "calendar date ahead of the clock; a re-read against the operator memory on quoted example times replaced it "
        "with the YYYY-MM-DD form before any other step read it",
        impact={"iterations": 1},
        artifacts=["spec:layout-templates"],
    ),
    rec(
        "e",
        "friction",
        type="recall.corpus-recurrence",
        what="host-linux Paths (cd only inside a subshell) and Transports (no inline heredoc for an edit) both "
        "state the rule; most Bash calls of this step opened with a bare cd, and one edit used a python heredoc. "
        "No guard refused either",
        impact={"iterations": 1},
        artifacts=[".claude/rules/host-linux.md"],
    ),
    rec(
        "f",
        "friction",
        type=None,
        untyped=True,
        what="an input snap's --origin text carried a clock time typed without a read of the clock (the minutes "
        "written as a placeholder); the clock read at the snap is in the same call's output, 23:03:18Z",
        impact={"iterations": 0},
        artifacts=["inputs#I11"],
        evidence="viola-0.1.0/chunks/2026-10-10-budget-governor/inputs/andromeda-inputs.json",
    ),
]

print("\n".join(json.dumps(r, ensure_ascii=False) for r in [step] + frictions))
