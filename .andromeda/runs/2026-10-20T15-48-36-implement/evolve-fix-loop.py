import json

TS = "2026-10-20T16:41:26Z"
base = {"v": 1, "ts": TS, "version": "viola-0.1.0", "epoch": "Epoch 6 — Security hardening",
        "chunk": "2026-10-20-epoch-5-cleanup-ii", "skill": "andromeda-implement", "step": "fix-loop"}
rs = [
    dict(base, kind="step", id=TS + "-a", outcome="soft-exit",
         counts={"iterations": 1, "retries": 0, "soft_exit": 1, "deferred": 0},
         consumed=[
             {"artifact": "plan", "quality": "wrong",
              "note": "entry 20's --allow regex holds two directory items that never match a path whole; entry 7 rests on a premise (an out-of-line cfg(test) module reads clean) that holds only for a test_-prefixed stem"},
             {"artifact": "research", "quality": "thin",
              "note": "the cfg sweep missed the cfg!(windows) macro form; the orphans premise came from one module name"},
             {"artifact": "tests", "quality": "ok"},
             {"artifact": "source", "quality": "ok"},
         ],
         produced=[
             {"artifact": "source", "signals": ["mutant-controls-16-of-16"],
              "note": "every hand-applied survivor mutant read red by its own panic line and green reverted"},
             {"artifact": "implement-outcome", "signals": ["stuck"],
              "note": "three words owed: a second live start (the founder's), the ledger test-file form, the guard's plan wording"},
         ],
         problem=[
             {"nature": "process", "solution": "unresolved",
              "note": "entries 16, 17, 27 red only on the empty Mcp. walk: the one capped live start yielded no recording"},
             {"nature": "product-logic", "solution": "unresolved",
              "note": "entry 7: cargo-modules 0.27 flags ledger/tests.rs; a #[path] test_cases.rs form measured clean on a probe crate, held for the operator's word"},
         ]),
    dict(base, kind="friction", id=TS + "-b", type="contract.spec-reality-gap",
         what="scope.md read testing.md 2026-09-25 as stale on cargo-modules 0.27 from send/test_support.rs alone; probe crates show only a test_-prefixed stem is skipped (tests, testing, support, foo, unit flagged)",
         impact={"soft_exit": 1}, artifacts=["viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup-ii/scope.md", "scripts/orphans-check.sh"],
         evidence=".andromeda/runs/2026-10-20T15-48-36-implement/outcome-2026-10-20-epoch-5-cleanup-ii.md"),
    dict(base, kind="friction", id=TS + "-c", type="contract.vacuous-check-found",
         what="the guard entry's P5 controls changed a plain listed file and an outside file, never a file under a listed directory item, so the allow regex's whole-match miss on src/cmd/verify/typed/ and src/human/board/ went unseen until implement",
         impact={"halted": 1}, artifacts=["viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup-ii/plan.md"],
         evidence=".andromeda/runs/2026-10-20T15-48-36-implement/gate-2026-10-20-epoch-5-cleanup-ii.json"),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
