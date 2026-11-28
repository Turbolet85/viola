import json
ts = "2026-11-28T15:13:51Z"
env = {"v": 1, "ts": ts, "version": "viola-0.2.0", "epoch": "Epoch 6 — The fleet console",
       "chunk": "2026-11-28-console-actions", "skill": "andromeda-implement", "step": "fix-loop"}
rs = [
    dict(env, kind="step", id=ts + "-a", outcome="ok",
         counts={"iterations": 4, "retries": 0, "soft_exit": 0, "deferred": 0},
         consumed=[
             {"artifact": "plan", "quality": "ok", "note": "the Part-1 entries ran as written; Part-2 entries 6, 9, 11 left unfired by the directive's Part-1 stop"},
             {"artifact": "research", "quality": "ok"},
             {"artifact": "tests", "quality": "ok"},
             {"artifact": "source", "quality": "ok"}],
         produced=[
             {"artifact": "source", "signals": ["controls-hold:4"],
              "note": "env-strip, one-child, out-escape and confirm each read red on its own case with the guard removed, green in place (evidence/control-1.txt)"},
             {"artifact": "implement-outcome", "signals": ["green"]}],
         problem=None),
    dict(env, kind="friction", id=ts + "-b", type="contract.test-expectation",
         what="two new unit cases asserted the wrong thing before the first block run: a lone ESC fed right before 'v' decodes as one Alt sequence, and the 2-column gutter cut the LAST cell off an 80-column frame-log case; both cases were corrected, the product unchanged",
         impact={"iterations": 2}, evidence="src/console/mod.rs tests"),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
