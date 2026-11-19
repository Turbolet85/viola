import json

TS = "2026-11-18T23:17:19Z"
BASE = {"v": 1, "ts": TS, "version": "viola-0.2.0",
        "epoch": "Epoch 4 — Driver reliability: ledger rows and bounds",
        "chunk": "2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin",
        "skill": "andromeda-phase", "step": "validate"}

rs = [
    dict(BASE, kind="step", id=TS + "-a", outcome="ok",
         counts={"dialogue_rounds": 2, "iterations": 1, "halted": 0},
         consumed=[
             {"artifact": "plan", "quality": "ok",
              "note": "every mechanical check passed on rev 6 and rev 9; the review folded three items (rev 7-9)"},
             {"artifact": "scope", "quality": "ok", "note": "val-1 aligned; the R1 fold a review-ordered scope expansion"},
             {"artifact": "matrix", "quality": "ok", "note": "no cap claimed: the 18 unclaimed caps are other entries' surfaces"},
         ],
         produced=[
             {"artifact": "plan", "signals": ["baseline-found-existing-red", "count-baseline-read-vacuous-then-resized"],
              "note": "check 4 (9)'s baseline of entry 6 read an intermittent red on an existing rehearsal, owner named at review; the first 10-count of the folded entry read 10 of 10 green on the untouched tree, so it was resized to 30, whose baseline read the red at firing 25"},
         ],
         problem=[{"nature": "process", "solution": "workaround",
                   "note": "inputs.py snap refused the P5 word file p5-word.md (it lies under .andromeda/, the pipeline's own tree); the word stands in the run dir and is cited by its path"}]),
    dict(BASE, kind="friction", id=TS + "-b", type="ambiguity.review-cycles",
         what="one review round before the yes: the overseer's relay corrected the VH widening's citation to the founder's own later word, confirmed the token-window lean, and folded the intermittent rehearsal with bounds and a count",
         impact={"dialogue_rounds": 1, "iterations": 3},
         artifacts=[".andromeda/runs/2026-11-18T21-59-43Z-phase/p5-card-1.md", ".andromeda/runs/2026-11-18T21-59-43Z-phase/p5-card-2.md"]),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
