import json

TS = "2026-10-20T19:39:19Z"
base = {"v": 1, "ts": TS, "version": "viola-0.1.0", "epoch": "Epoch 6 — Security hardening",
        "chunk": "2026-10-20-epoch-5-cleanup-ii", "skill": "andromeda-wrap-session", "step": "report"}
rs = [
    dict(base, kind="step", id=TS + "-a", outcome="ok",
         counts={"extra_reads": 9},
         consumed=[
             {"artifact": "conversation", "quality": "ok", "note": "this window held implement's run whole"},
             {"artifact": "implement-outcome", "quality": "ok",
              "note": "green after inputs#I5; the operator pass's only move after it was a CI re-run with no edit"},
             {"artifact": "operator-directive", "quality": "ok",
              "note": "inputs#I6 added the attempt-2 CI basis and the stop-the-loop word; it corrected no measurement"},
             {"artifact": "operator-pass", "quality": "thin", "signals": ["fixed", "commits:1", "red:1"],
              "note": "evidence/operator-pass.md records attempt 1 red (two macOS jobs never given a runner); the attempt-2 green was read by this wrap's own ci.py call, not from evidence/"},
             {"artifact": "git-state", "quality": "ok"},
             {"artifact": "plan", "quality": "wrong",
              "note": "step 15's class frames and step 18's argv were measured false; entry 20's allow regex cannot match its own directory items"},
         ],
         produced=[{"artifact": "report", "signals": ["new-text-generated", "new-text-rows:725"],
                    "note": "a fixture-set count (architecture.md:374, 29 -> 30 files) and a stale probe count (test-plan.md:1027, 1/1 vs 2/2) surfaced only through the site sweep's rows and window reads"}],
         problem=None),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
