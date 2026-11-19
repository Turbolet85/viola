import json

ts = "2026-11-19T06:19:51Z"
base = {"v": 1, "ts": ts, "version": "viola-0.2.0", "epoch": "Epoch 4 — Driver reliability: ledger rows and bounds",
        "chunk": "2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin", "skill": "andromeda-phase", "step": "validate"}
rs = [
    dict(base, kind="step", id=ts + "-a", outcome="ok",
         counts={"dialogue_rounds": 0, "iterations": 0, "halted": 0},
         consumed=[{"artifact": "plan", "quality": "ok", "note": "rev 11; scope sync PASS +0 -0 on first fire; planlint 0 hits; size warning (~540 prose lines) carried from rev 9"},
                   {"artifact": "scope", "quality": "ok"},
                   {"artifact": "matrix", "quality": "ok", "note": "no cap claimed; audit 0 claimed"}],
         produced=[{"artifact": "matrix", "signals": ["no-claim"], "note": "approval fact written by matrix.py audit --run-dir"}],
         problem=None),
    dict(base, kind="friction", id=ts + "-b", type="contract.mechanical-check",
         what="check 6 (size) warned: about 540 plan prose lines outside the fence against the ~150-400 band, as at rev 9",
         impact={},
         artifacts=["plan"]),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
