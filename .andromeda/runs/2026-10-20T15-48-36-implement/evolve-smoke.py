import json

TS = "2026-10-20T18:41:29Z"
base = {"v": 1, "ts": TS, "version": "viola-0.1.0", "epoch": "Epoch 6 — Security hardening",
        "chunk": "2026-10-20-epoch-5-cleanup-ii", "skill": "andromeda-implement", "step": "smoke"}
rs = [
    dict(base, kind="step", id=TS + "-a", outcome="ok",
         counts={"retries": 0, "deferred": 0},
         consumed=[{"artifact": "agent-harness", "quality": "ok"}, {"artifact": "plan", "quality": "ok"}],
         produced=[
             {"artifact": "conversation", "signals": ["recorded-not-rerun"],
              "note": "the plan's smoke entries 21-26 ran as P2 gates, green in both block firings"},
             {"artifact": "implement-outcome", "signals": ["green"],
              "note": "after inputs#I5: start 2 recorded the exchange; the block reads 25 green, 1 red (20, the plan's wording the overseer's word covers), 1 recorded"},
         ],
         problem=[
             {"nature": "process", "solution": "overridden",
              "note": "the cap of one start rose to two on the founder's own word (inputs#I5 Q1) after start 1 yielded no recording"},
             {"nature": "product-logic", "solution": "workaround",
              "note": "the ledger test module file is ledger/test_cases.rs under #[path], keeping ledger::tests, because cargo-modules 0.27 flags any out-of-line test module stem not beginning test_ (inputs#I5 Q2)"},
         ]),
    dict(base, kind="friction", id=TS + "-b", type="contract.premise-falsified",
         what="plan step 15 named the class frames initialize, notifications/initialized and tools/list; the live 2.1.302 client on protocol 2026-07-28 sent server/discover then tools/list, each with its own _meta; the class document, schema and planted cases were adapted to the measured frames",
         impact={"iterations": 1}, artifacts=["viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup-ii/plan.md", "schemas/claude-mcp-fixture.v1.json"],
         evidence="viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup-ii/evidence/live-tap.md"),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
