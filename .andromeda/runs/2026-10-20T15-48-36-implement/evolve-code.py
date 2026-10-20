import json

TS = "2026-10-20T16:31:34Z"
base = {"v": 1, "ts": TS, "version": "viola-0.1.0", "epoch": "Epoch 6 — Security hardening",
        "chunk": "2026-10-20-epoch-5-cleanup-ii", "skill": "andromeda-implement", "step": "code"}
rs = [
    dict(base, kind="step", id=TS + "-a", outcome="ok-degraded",
         counts={"extra_reads": 14, "soft_exit": 1},
         consumed=[
             {"artifact": "plan", "quality": "wrong",
              "note": "step 18's start argv put the prompt after --mcp-config, which 2.1.302 reads as variadic <configs...>; the one start yielded no recording"},
             {"artifact": "research", "quality": "thin",
              "note": "the cfg sweep over ledger.rs named only the 1606-1609 pair; the moved tests held two cfg!(windows) oracles the Windows scope contract reads"},
             {"artifact": "scope", "quality": "ok"},
             {"artifact": "source", "quality": "ok"},
         ],
         produced=[{"artifact": "source", "signals": ["style-inferred", "guard-refused:3"],
                    "note": "bash guard arms: cd out of the root (2), a cat heredoc to a file (1); each re-issued once"}],
         problem=[
             {"nature": "product-logic", "solution": "workaround",
              "note": "two cfg!(windows) test oracles in the moved ledger tests restated as !cfg!(unix), same truth on the three CI OSes, so the Windows scope contract does not read ledger/tests.rs as Windows-gated"},
             {"nature": "process", "solution": "unresolved",
              "note": "the founder's one live start exited 1 in 252 ms before any MCP server ran; stderr not recorded by the rig; a second start needs his new word"},
         ]),
    dict(base, kind="friction", id=TS + "-b", type="contract.narrow-basis-claim",
         what="research's sweep pattern cfg(windows)|cfg(unix)|cfg(not missed the cfg!(windows) macro form the Windows scope contract also reads; two hits surfaced only at the move",
         impact={"iterations": 1}, artifacts=["crates/viola-agent-claude/src/ledger/tests.rs"],
         evidence="tests/contract_windows_mutation_scope.rs is_windows_gate"),
    dict(base, kind="friction", id=TS + "-c", type="input.plan-step-ambiguous",
         what="plan step 18 fixed the live argv `claude -p --strict-mcp-config --mcp-config <file> '<prompt>'`; 2.1.302's --help reads --mcp-config <configs...> variadic, and the capped start exited 1 with no server spawned (cause a hypothesis: stderr unrecorded)",
         impact={"halted": 1}, artifacts=["viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup-ii/plan.md"],
         evidence="viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup-ii/evidence/live-tap.md"),
    dict(base, kind="friction", id=TS + "-d", type=None, untyped=True,
         what="the rig's start captured the CLI's stderr and printed only stdout's count, so the one capped start's failure text was lost",
         impact={"retries": 0}, evidence="viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup-ii/evidence/live-tap.md"),
    dict(base, kind="friction", id=TS + "-e", type=None, untyped=True,
         what="the served-duplex unit case hung twice (600 s, then 300 s bound): dropping one half of tokio::io::split never signals EOF to the peer; shutdown() on the write half fixed it",
         impact={"retries": 2}, artifacts=["crates/viola-mcp/src/tools.rs"]),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
