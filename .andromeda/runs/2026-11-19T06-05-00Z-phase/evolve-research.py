import json

ts = "2026-11-19T06:08:15Z"
base = {"v": 1, "ts": ts, "version": "viola-0.2.0", "epoch": "Epoch 4 — Driver reliability: ledger rows and bounds",
        "chunk": "2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin", "skill": "andromeda-phase", "step": "research"}
rs = [
    dict(base, kind="step", id=ts + "-a", outcome="ok",
         counts={"reformulations": 0, "extra_reads": 2},
         consumed=[{"artifact": "source", "quality": "ok", "note": "the 2.1.325 and 2.1.328 bundles by byte search, the rig's CLI resolution and seeded markers"}],
         produced=[{"artifact": "scope", "signals": ["premise-corrected", "graph-not-applicable"],
                    "note": "revision 1: the version moved to 2.1.328 by the founder's word; four bundle readings plus the seeded markers re-read, all hold"}],
         problem=None),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
