import json

ts = "2026-11-19T06:46:06Z"
base = {"v": 1, "ts": ts, "version": "viola-0.2.0", "epoch": "Epoch 4 — Driver reliability: ledger rows and bounds",
        "chunk": "2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin", "skill": "andromeda-phase"}
rs = [
    dict(base, kind="step", step="research", id=ts + "-a", outcome="ok",
         counts={"reformulations": 0, "extra_reads": 0},
         consumed=[{"artifact": "source", "quality": "ok",
                    "note": "the stopped run's outcome file and evidence/r1-red.md; two greps of the rig (arm_r1/r1_keys callers, kept_frame); git status of the tree"}],
         produced=[{"artifact": "scope", "signals": ["graph-not-applicable"],
                    "note": "revision 2: three verified bullets (the stop, the fold out of the gates as a carry, the round unchanged) citing inputs#I9/I10"},
                   {"artifact": "research", "signals": ["revision-section-landed"],
                    "note": "research rev 4 adds ## Revision 2; lists unchanged, delta +0 -0"}],
         problem=None),
    dict(base, kind="step", step="plan", id=ts + "-b", outcome="ok",
         counts={"dialogue_rounds": 0, "iterations": 0},
         consumed=[{"artifact": "scope", "quality": "ok"}, {"artifact": "research", "quality": "ok"}],
         produced=[{"artifact": "plan", "signals": ["revision-2-rev-12"],
                    "note": "18b-18d and the 30-run entry removed, entries after it renumbered (round 9-18, operator 19-27), 18a kept, re-entry at step 19; scope sync PASS on the first fire"}],
         problem=None),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
