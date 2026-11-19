import json

ts = "2026-11-19T06:49:18Z"
base = {"v": 1, "ts": ts, "version": "viola-0.2.0", "epoch": "Epoch 4 — Driver reliability: ledger rows and bounds",
        "chunk": "2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin", "skill": "andromeda-phase"}
rs = [
    dict(base, kind="step", step="validate", id=ts + "-a", outcome="ok",
         counts={"dialogue_rounds": 1, "iterations": 0, "halted": 0},
         consumed=[{"artifact": "plan", "quality": "ok", "note": "sync PASS +0 -0, lists parsed, dry-run 27 entries, planlint 0 hits, inputs 10 unchanged/n-a"},
                   {"artifact": "scope", "quality": "ok", "note": "val-1 aligned with scope rev 6"},
                   {"artifact": "matrix", "quality": "ok", "note": "no claim; audit 0 claimed"}],
         produced=[{"artifact": "matrix", "signals": ["no-claim"], "note": "approval trail written by matrix.py audit with --run-dir"}],
         problem=[{"nature": "process", "solution": "removed-cause",
                   "note": "gate.py hygiene flagged a temp-root path in the stopped implement run's outcome file (line 66), cleared on the operator's word at the P5 yes; then this run's own card quoting the same path twice was respelled after the word, a quote change only"}]),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
