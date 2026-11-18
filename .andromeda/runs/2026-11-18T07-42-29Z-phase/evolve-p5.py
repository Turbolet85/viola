import json

TS = "2026-11-18T08:10:33Z"
BASE = {"v": 1, "ts": TS, "version": "viola-0.2.0",
        "epoch": "Epoch 4 — Driver reliability: ledger rows and bounds",
        "chunk": "2026-11-18-trust-record-row-read-live", "skill": "andromeda-phase", "step": "validate"}

rs = [
    dict(BASE, kind="step", id=TS + "-a", outcome="ok",
         counts={"dialogue_rounds": 2, "iterations": 1, "halted": 0},
         consumed=[{"artifact": "plan", "quality": "ok", "note": "dry-run 19 not-run, planlint hits 0, inputs 0 drifted, scope sync PASS, lists parsed, on both cards"},
                   {"artifact": "scope", "quality": "ok"},
                   {"artifact": "matrix", "quality": "ok", "note": "no cap claimed; audit 0 claimed"}],
         produced=[{"artifact": "scope", "signals": ["val1-aligned"], "note": "scope rev 7 names the founder's narrowing"}],
         problem=[{"nature": "process", "solution": "overridden",
                   "note": "card 1's whole-rows lean was narrowed at review by the founder's own answer to a rig-only hidden verify option; folded at P5, checks re-run, card 2 put and approved"},
                  {"nature": "process", "solution": "workaround",
                   "note": "an Edit of a plan bullet failed on a joined line; replaced by a scratch python fold of the working copy"}]),
    dict(BASE, kind="friction", id=TS + "-b", type="ambiguity.review-cycles",
         what="card 1's decisive lean (whole rows under --record) went to review; the founder narrowed it to a hidden rig-only option, costing one fold and a second card",
         impact={"dialogue_rounds": 1, "iterations": 1},
         artifacts=[".andromeda/runs/2026-11-18T07-42-29Z-phase/p5-card-1.md", ".andromeda/runs/2026-11-18T07-42-29Z-phase/p5-card-2.md"],
         evidence="inputs#I13"),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
