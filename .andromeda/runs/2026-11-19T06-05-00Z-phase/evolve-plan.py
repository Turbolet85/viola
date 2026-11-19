import json

ts = "2026-11-19T06:10:36Z"
base = {"v": 1, "ts": ts, "version": "viola-0.2.0", "epoch": "Epoch 4 — Driver reliability: ledger rows and bounds",
        "chunk": "2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin", "skill": "andromeda-phase", "step": "plan"}
rs = [
    dict(base, kind="step", id=ts + "-a", outcome="halted-resolved",
         counts={"dialogue_rounds": 0, "iterations": 0, "halted": 1},
         consumed=[{"artifact": "scope", "quality": "ok"}, {"artifact": "research", "quality": "ok"}],
         produced=[{"artifact": "plan", "signals": ["revision-1-put-rev-10"],
                    "note": "the 2.1.328 move written as plan rev 10; the list sync after it refused (gate.py scope --sync exit 3), so P5 was not reached"}],
         problem=[{"nature": "process", "solution": "unresolved",
                   "note": "the revision ran on the operator word over an approved plan no implement run had executed; gate.py scope --sync refuses a revision with no stopped implement run, and the overseer relay (inputs#I6 item 7) directs a stop with the printed line"}]),
    dict(base, kind="friction", id=ts + "-b", type=None, untyped=True,
         what="gate.py scope --sync exit 3: no *-implement run before 2026-11-19T06-05-00Z-phase holds outcome-{marker}.md — a revision stands on a stopped implement run; the revision door has no form for re-planning an approved plan that never ran",
         impact={"halted": 1},
         artifacts=["plan"],
         evidence=".andromeda/runs/2026-11-19T06-05-00Z-phase/gate-2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin.json"),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
