import json

ts = "2026-11-19T06:16:49Z"
base = {"v": 1, "ts": ts, "version": "viola-0.2.0", "epoch": "Epoch 4 — Driver reliability: ledger rows and bounds",
        "chunk": "2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin", "skill": "andromeda-phase"}
rs = [
    dict(base, kind="step", step="research", id=ts + "-a", outcome="ok",
         counts={"reformulations": 0, "extra_reads": 1},
         consumed=[{"artifact": "source", "quality": "ok",
                    "note": "the 2.1.325 and 2.1.328 bundles re-read by byte search (no start), the tree's diff, df and target/"}],
         produced=[{"artifact": "scope", "signals": ["graph-not-applicable"],
                    "note": "revision completion: one verified bullet added for the stopped implement run (inputs#I7); every rev-4 reading re-derived and held"},
                   {"artifact": "research", "signals": ["revision-section-landed"],
                    "note": "research rev 3 adds ## Revision 1, which the plan had cited since rev 10 though the earlier run never landed it"}],
         problem=[{"nature": "process", "solution": "workaround",
                   "note": "the earlier revision run's release-build duration, which plan rev 10 cited to research, was never written to disk; the plan line was re-worded to the build's mtime and the duration dropped, not re-measured"}]),
    dict(base, kind="step", step="plan", id=ts + "-b", outcome="ok",
         counts={"dialogue_rounds": 0, "iterations": 0},
         consumed=[{"artifact": "scope", "quality": "ok"}, {"artifact": "research", "quality": "ok"}],
         produced=[{"artifact": "plan", "signals": ["revision-1-completed-rev-11"],
                    "note": "four point edits over rev 10 (the stopped run, inputs#I7, the release-build line); scope sync PASS delta +0 -0 on the first fire"}],
         problem=None),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
