import json

TS = "2026-11-18T09:15:03Z"
BASE = {"v": 1, "ts": TS, "version": "viola-0.2.0",
        "epoch": "Epoch 4 — Driver reliability: ledger rows and bounds",
        "chunk": "2026-11-18-trust-record-row-read-live", "skill": "andromeda-phase"}

rs = [
    dict(BASE, kind="step", id=TS + "-a", step="validate", outcome="ok",
         counts={"dialogue_rounds": 0, "iterations": 0, "halted": 0},
         consumed=[{"artifact": "plan", "quality": "ok"},
                   {"artifact": "scope", "quality": "ok"},
                   {"artifact": "matrix", "quality": "ok"}],
         produced=[{"artifact": "plan", "signals": ["superseded-line-marked"],
                    "note": "planlint 12 (d) listed the Revision 1 seed acceptance line pinning the six-marker case step 27 renames; marked superseded at P5, the plan re-put and every check re-run; val-1 aligned; review prompted as text, a single yes; no cap claimed (0 coverage rows)"}],
         problem=None),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
