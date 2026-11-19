import json

ts = "2026-11-19T07:06:10Z"
base = {"v": 1, "ts": ts, "version": "viola-0.2.0", "epoch": "Epoch 4 — Driver reliability: ledger rows and bounds",
        "chunk": "2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin", "skill": "andromeda-phase"}
rs = [
    dict(base, kind="step", step="validate", id=ts + "-a", outcome="ok",
         counts={"dialogue_rounds": 0, "iterations": 0, "halted": 0},
         consumed=[{"artifact": "plan", "quality": "ok", "note": "scope sync PASS +0 -0, lists parsed, 27 entries parse, planlint 0 hits, inputs 16/0 drifted/0 uncited; val-1 aligned with scope rev 7"},
                   {"artifact": "scope", "quality": "ok"},
                   {"artifact": "matrix", "quality": "ok", "note": "no capability claimed; audit claimed 0"}],
         produced=[{"artifact": "plan", "signals": ["guard-refused:1", "mended-expect-verified-on-log"],
                    "note": "entry 14's mended atoms checked by script on 14.log: 34/34 content atoms held, 4 synthetic negatives red; one bash-guard refusal of a pipe into grep, re-issued bare once; review prompt issued as text, a single yes"}],
         problem=None),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
