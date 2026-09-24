# Session Handoff

**Last Updated:** 2026-09-24T09:52:10Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup
**Status:** clean
**Last Commit:** 2026-09-24-supply-chain-and-workflow-gates — feat: deny.toml four-family policy, sole-root tokio ban, permanent ban probes, supply-chain + lint CI jobs, nightly advisories

## Position
- Done: 2026-09-24-supply-chain-and-workflow-gates.
  - `deny.toml` covers advisories, licences, sources and bans (C, telemetry and feature bans).
  - `deny-sync.toml` holds the tokio ban, run once per crate in `scripts/sync-crates.txt` as the sole root.
  - `scripts/deny-probes.sh` proves all 13 bans fire (13/13 banned, control clean).
  - `ci.yml` gained `lint` (3 OSes) and `supply-chain` (ubuntu). `nightly.yml` runs the weekly advisories check.
- Next: /andromeda-phase to promote and plan "Diagnostics plane". It carries the PREREQ `close rust gate deferral`.
- **CI witness owed (plan operator entries):** this wrap pushes the branch. Then:
  1. Read `check-runs` for the pushed sha. Expect `success` on test ×3, lint ×3, supply-chain and mutants.
  2. Run `gh workflow run nightly.yml --ref build/viola-0.1.0`.
  3. Read that dispatched run for the same sha. Expect `success`.

## Work done
- 5 new files, 1 workflow modified. Local gates: 9 green, 1 deferred (Rust unit suite, zero `.rs` delta), 4 left to the operator. Mutants verdict `no-rust-delta`.
- `plan.md` was edited after implement, on the operator's direction. The gate text now uses cargo-deny 0.20's global `--config` form, and the mutants gate lost a `mutants.out/` artifact key that could never be fresh.

## Drift resolved
- 21 amendments across 5 masters (arch, test-plan, security-plan, obs-plan, a11y-plan). 14 came from the detectors (one applied in part: the config files went to the tree, not the registry). The orchestrator raised 7 more. There were 0 escalations.
- Retired: the "graph with the async crates excluded" tokio-ban mechanism (measured false-fail under feature unification), the viola-e2e "tokio wrappers list", and "one workflow `ci.yml`".
- 9 leaf sites re-derived (CLAUDE.md workflow block, commands, stack, workflow, rules/security, services viola-core and viola-mcp).

## Notes
- Operator decisions this chunk:
  - The weekly run lives in `nightly.yml`; test-plan owns CI layout.
  - The ban probes are permanent in CI.
  - The two plan gate edits after implement.
- New route pins: CARRY on "Wrapper channel" (join `sync-crates.txt`), "MCP server for drivers" (the `viola-channel` own-root false-fail limit) and "Quality gates" (zizmor pedantic `concurrency-limits` left open).
- Host tools: cargo-deny 0.19.4 → 0.20.2 and zizmor 1.30.1 installed on the dev host (`cargo install --locked`).
- The operator's viola-lab prototype (`viola.exe` 12172, 58624) was running on the host; it is not this project's.
- Last failed command: none open.
