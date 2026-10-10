import json

TS = "2026-10-10T22:01:41Z"
BASE = {
    "v": 1,
    "ts": TS,
    "version": "viola-0.1.0",
    "epoch": "Epoch 4 — Session state & governance",
    "chunk": "2026-10-10-budget-governor",
    "skill": "andromeda-implement",
    "step": "fix-loop",
}


def rec(letter, kind, **fields):
    out = dict(BASE)
    out["kind"] = kind
    out["id"] = TS + "-" + letter
    out.update(fields)
    return out


rs = [
    rec(
        "a",
        "step",
        outcome="ok",
        counts={"iterations": 1, "retries": 1, "soft_exit": 0, "deferred": 0},
        consumed=[
            {"artifact": "plan", "quality": "ok", "note": "the block ran as written: 13 entries fired, 3 operator entries not run, on both calls"},
            {"artifact": "research", "quality": "ok"},
            {"artifact": "tests", "quality": "ok"},
            {"artifact": "source", "quality": "ok"},
        ],
        produced=[
            {"artifact": "source", "signals": []},
            {"artifact": "matrix", "signals": [], "note": "matrix.py show reads 0 capabilities claimed by this chunk, so no ref was written"},
            {"artifact": "implement-outcome", "signals": ["green"], "note": "two whole-block calls, each green 13, red 0; the second on the final tree after one test-only restatement"},
        ],
        problem=[
            {"nature": "process", "solution": "workaround", "note": "cargo fmt --check, host clippy and the Windows-target clippy were run by hand ahead of the first block call, so the block would not carry a lint red through its hour-bounded pre-push entry; fmt and host clippy then ran through the tool twice"},
        ],
    ),
    rec(
        "b",
        "friction",
        type="retry.fix-iterations",
        what="the hand clippy pre-flight read two cloned_ref_to_slice_refs errors in the new gate_eval case of src/run/budget.rs; fixed before the block; after the first green block one self-found test restatement in src/cmd/run.rs (the from-before-budget order with a non-bool budget) cost a second whole-block run",
        impact={"iterations": 1, "retries": 1},
        artifacts=["src/run/budget.rs", "src/cmd/run.rs"],
        evidence=".andromeda/runs/2026-10-10T21-29-10-implement (the gate trail, two run records)",
    ),
    rec(
        "c",
        "friction",
        type="contract.spec-reality-gap",
        what="surfaced for the wrap, pointer only: the start now opens run.budget_eval under run.start, which the start-order span list of obs-plan section 4 Scenario 1 does not hold and the plan's expected amendments do not name; the unit case that pins the list was restated",
        impact={"iterations": 0},
        artifacts=["src/cmd/run.rs", ".andromeda/obs-plan.md"],
    ),
    rec(
        "d",
        "friction",
        type=None,
        untyped=True,
        what="the pre-push document and its log carry no coverage figure; the three numbers the acceptance names were read from target/agent-run/artifacts/llvm-cov-summary.json after two greps of the log and one of the harness source",
        impact={"extra_reads": 3},
        artifacts=["scripts/agent-run.sh pre-push", "target/agent-run/artifacts/llvm-cov-summary.json"],
    ),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
