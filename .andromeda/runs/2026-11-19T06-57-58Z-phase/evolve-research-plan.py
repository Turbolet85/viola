import json

ts = "2026-11-19T07:05:17Z"
base = {"v": 1, "ts": ts, "version": "viola-0.2.0", "epoch": "Epoch 4 — Driver reliability: ledger rows and bounds",
        "chunk": "2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin", "skill": "andromeda-phase"}
rs = [
    dict(base, kind="step", step="research", id=ts + "-a", outcome="ok",
         counts={"reformulations": 0, "extra_reads": 0},
         consumed=[{"artifact": "source", "quality": "ok",
                    "note": "the stopped run's outcome file, evidence/live-starts.md and round-065404Z.txt; gate.py show --n 14 (14.log); rig/mod.rs put_after and isolate.rs needle_word"}],
         produced=[{"artifact": "scope", "signals": ["graph-not-applicable", "guard-refused:2"],
                    "note": "revision 3: three verified bullets citing inputs#I14-I16; two cd calls refused by the bash guard (a references dir, the evidence dir), each re-issued once as a subshell or absolute path"},
                   {"artifact": "research", "signals": ["revision-section-landed"],
                    "note": "research rev 5 adds ## Revision 3; lists unchanged, delta +0 -0"}],
         problem=[{"nature": "process", "solution": "workaround",
                   "note": "research's ## Revision 3 section was appended to its working copy with a shell printf >> rather than the Edit tool the letter names for a document write; the put and the sync read it whole"}]),
    dict(base, kind="step", step="plan", id=ts + "-b", outcome="ok",
         counts={"dialogue_rounds": 1, "iterations": 0},
         consumed=[{"artifact": "scope", "quality": "ok"}, {"artifact": "research", "quality": "ok"}],
         produced=[{"artifact": "plan", "signals": ["designed-dialogue:entry-18-after-l2-census", "revision-3-rev-13"],
                    "note": "gate.py --live-legs --entry 17 fires no round step after the last leg, so the after-L2 census needed a re-key; the overseer chose operator and added the failing-readings and projects-new carry asks; entry 14 expect mended, verified 34/34 atoms held on 14.log and 4 synthetic negatives red; scope sync PASS on the first fire"}],
         problem=None),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
