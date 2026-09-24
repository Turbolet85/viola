# Curation — 2026-09-24-supply-chain-and-workflow-gates

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  + "Probe, baseline and gate a CI tool at the exact version CI pins …" (confidence 0.8)
  Proof: the P5 baselines and P3 probes ran on host cargo-deny 0.19.4 while CI pins 0.20.2. The plan's gate `cargo deny … check bans -c deny-sync.toml` failed under 0.20.2 with `error: unexpected argument '-c' found` (implement gate log 4). Signals: measurement +0.4, specific +0.2, no-other-home +0.2 (no master, route annotation or ledger note carries it).
  Tier 2 (.claude/rules/*):                   + host-win32.md: "`grep` over a glob that matches ONE file prints no `file:` prefix …" (confidence 0.8)
  Proof: P4 control, `.github/workflows/*.yml` matching only `ci.yml`: the unprefixed line `103: AGENT_RUN_CHUNK_BASE…` escaped the `^[^:]+:[0-9]+:` exemption, and `grep -H` fixed it (plan gate "event payload" grep; baseline/8.log, 8c.log). Signals: measurement +0.4, specific +0.2, no-other-home +0.2. `host-win32.md` has no `paths:`, so the entry is held to the Tier-1 bar (one sentence, ~220 B).
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 1 task-specific (probe projects under target/ need their own [workspace] — carried in the script comment) · 0 conflict · 0 deferred; 3 rejected at exactly 0.6 with a master home this wrap (cargo-deny --exclude vs feature unification → architecture §Build system; cargo-deny 0.20 global --config → test-plan §9; exit-only probes vacuous → architecture §Build system)
  No-other-home: "Probe, baseline and gate a CI tool at the exact version CI pins"; "grep over a one-file glob drops the file: prefix"
  CLAUDE.md size: 121/200 · T1 1.2 KB, 0 over 600 B
