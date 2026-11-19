import json

TS = "2026-11-18T22:52:42Z"
BASE = {"v": 1, "ts": TS, "version": "viola-0.2.0", "epoch": "Epoch 4 — Driver reliability: ledger rows and bounds",
        "chunk": "2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin",
        "skill": "andromeda-phase", "step": "plan"}

rs = [
    dict(BASE, kind="step", id=TS + "-a", outcome="ok",
         counts={"dialogue_rounds": 0, "iterations": 3, "extra_reads": 1},
         consumed=[
             {"artifact": "rows", "quality": "ok",
              "note": "294 475 B past the 250 000 B bound: read by search; every cited handle found in a rows file"},
             {"artifact": "research", "quality": "thin",
              "note": "its Files to modify missed ledger.rs, tests/support/fake.rs and tests/cli_send.rs, which the verify line's public predicate and the moved synthetic screen need; added at P4 and research re-put"},
             {"artifact": "scope", "quality": "ok"},
         ],
         produced=[
             {"artifact": "plan", "signals": [
                 "authority-resolved:founder-answer-3-over-overseer-item-13-token-window",
                 "guard-refused:1"],
              "note": "relay item 3 (the founder's: credentials read by nothing but L2's token check) against item 13 (the overseer's: VH keeps the token window) resolved by authority at no cost, named on the P5 card; the bash guard refused one cd into the run dir, re-issued with absolute paths"},
         ],
         problem=None),
    dict(BASE, kind="friction", id=TS + "-b", type="input.research-thin",
         what="research's Files to modify named six files; the plan needed three more (the agent crate's public browser-tools predicate, the moved synthetic screen helper and its old caller), found by reading the code at P4",
         impact={"extra_reads": 1, "iterations": 1},
         artifacts=["doc:2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin/research"]),
    dict(BASE, kind="friction", id=TS + "-c", type="retry.synthesis-rework",
         what="three re-puts after the first: a TOML literal with doubled single quotes read UNPARSED by the put, entry numbers in the steps off by one against the parsed fence, and three leg notes without their atom source (planlint HITs)",
         impact={"iterations": 3},
         artifacts=["doc:2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin/plan"]),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
