import json

TS = "2026-11-18T09:10:37Z"
BASE = {"v": 1, "ts": TS, "version": "viola-0.2.0",
        "epoch": "Epoch 4 — Driver reliability: ledger rows and bounds",
        "chunk": "2026-11-18-trust-record-row-read-live", "skill": "andromeda-phase"}

rs = [
    dict(BASE, kind="step", id=TS + "-a", step="research", outcome="ok",
         counts={"reformulations": 0, "extra_reads": 9},
         consumed=[{"artifact": "source", "quality": "ok"}],
         produced=[
             {"artifact": "research", "signals": ["graph-not-applicable", "guard-refused:1"],
              "note": "revision 4 closure by the bundle read and targeted reads of rig/isolate.rs and the V arm only (the operator's word, inputs#I17 item 7); list sync delta +0 -0, one-line Revision 4 section; guard-refused: a cd into the prior phase run dir, re-issued with absolute paths"},
             {"artifact": "scope", "signals": ["premise-closed"],
              "note": "an [inferred] Revision 4 bullet closed in the same pass against the 2.1.325 bundle (ChromeAutoEnableDialog, MIs/lTo/SD, the offer step's decision-recorded skip)"}],
         problem=[
             {"nature": "environment", "solution": "workaround",
              "note": "a /usr/bin/grep -aoE with bounded context over the 365 MB claude binary ran past the Bash tool's 120 s limit and was stopped; replaced by a scratch mmap reader of the binary run by path"},
             {"nature": "process", "solution": "workaround",
              "note": "the P3 checkpoint was fired after P4's plan put rather than before P4 began; both checkpoints appended together"}]),
    dict(BASE, kind="step", id=TS + "-b", step="plan", outcome="ok",
         counts={"dialogue_rounds": 0, "iterations": 0},
         consumed=[{"artifact": "research", "quality": "ok"},
                   {"artifact": "scope", "quality": "ok"}],
         produced=[{"artifact": "plan",
                    "signals": ["authority-resolved:relay-item-3-key-over-environment"],
                    "note": "the bundle holds a key the keep-off answer writes (claudeInChromeDefaultEnabled false), so the environment arm of the relay was not taken; two of the six Revision-1 markers dropped on the bundle's reasons; no escalate pattern met (one key more in the rig's own private config)"}],
         problem=None),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
