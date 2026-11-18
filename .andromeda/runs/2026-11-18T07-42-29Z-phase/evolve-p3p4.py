import json

TS = "2026-11-18T07:53:11Z"
BASE = {"v": 1, "ts": TS, "version": "viola-0.2.0",
        "epoch": "Epoch 4 — Driver reliability: ledger rows and bounds",
        "chunk": "2026-11-18-trust-record-row-read-live", "skill": "andromeda-phase"}

rs = [
    dict(BASE, kind="step", id=TS + "-a", step="research", outcome="ok",
         counts={"reformulations": 0, "extra_reads": 6},
         consumed=[{"artifact": "source", "quality": "ok"}],
         produced=[
             {"artifact": "research", "signals": ["graph-not-applicable", "guard-refused:1"],
              "note": "revision 3 premise closure by targeted reads, no row read (the operator's word, inputs#I11 item 8); guard-refused: a cd into the prior phase run dir, re-issued with absolute paths"},
             {"artifact": "scope", "signals": ["premise-corrected"],
              "note": "an [inferred] Revision 3 bullet closed in the same pass against src/cmd/verify.rs and ledger.rs"}],
         problem=[{"nature": "process", "solution": "workaround",
                   "note": "the P3 checkpoint was fired after P4's plan put rather than before P4 began; both checkpoints appended together"}]),
    dict(BASE, kind="friction", id=TS + "-b", step="research", type=None, untyped=True,
         what="research's recorded TypedRun-literal sweep reads 35 sites; a re-count at this revision read 31 lines matching 'TypedRun {' under crates/viola-agent-claude/src/ledger/, left for implement's recorded re-run",
         impact={"extra_reads": 2}, artifacts=["doc:2026-11-18-trust-record-row-read-live/research ## Sweeps"]),
    dict(BASE, kind="step", id=TS + "-c", step="plan", outcome="ok",
         counts={"dialogue_rounds": 0, "iterations": 0},
         consumed=[{"artifact": "research", "quality": "ok"},
                   {"artifact": "scope", "quality": "ok"}],
         produced=[{"artifact": "plan",
                    "signals": ["authority-resolved:relay-scrub-wording-yielded-to-relay-items-2-4-intent",
                                "guard-refused:2"],
                    "note": "the relay asks the failed screen 'scrubbed as --record scrubs', but the screen recording cuts rows to signature literals, which would keep only B-F's rule row; leaned whole rows through ledger::scrub + screen_fault, stated on the card; guard-refused: a cd into the skill references (re-issued by absolute path) and a cat heredoc append (re-issued through Edit)"}],
         problem=None),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
