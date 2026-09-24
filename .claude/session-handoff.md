# Session Handoff

**Last Updated:** 2026-09-24T16:41:30Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup
**Status:** clean
**Last Commit:** 2026-09-24-workspace-tree-and-code-graph-planes — feat: release-check and orphans gates, fuzz lockfile audit, arch tree lists every test/obs/a11y artifact, code-graph planes decided

## Position
- Done: 2026-09-24-workspace-tree-and-code-graph-planes. This closes Epoch 1 — Foundation.
  - New `scripts/release-check.sh`: a per-OS `release` job (target job 6) that allows only `viola` in the release build.
  - New `scripts/orphans-check.sh`: `cargo modules orphans --deny` per lib/bin target in `lint`.
  - The `fuzz/Cargo.lock` audit (advisories + sources) runs in `supply-chain`, and weekly in nightly.
  - The architecture tree lists all 45 test/obs/a11y artifacts (inventory gate 45 rows / 0 missing).
- Next: `/andromeda-phase` to promote and plan "Security prerequisites", the Epoch 2 head. Its PREREQ closes the Rust gate deferral from this chunk.
- **v1-23 verified flip is gated on this push** (overseer condition): the three `release (…)` legs must read `success` on the pushed sha. The inventory half read 0 at this wrap.
- **CI witness for 3f385dd (quality-gates):** run 36019646063 is 12/12 green, all 3 `file_mode` mutants are caught on ubuntu, the union is green, and the msrv log shows `rustc 1.96.1`.

## Work done
- Source: 2 workflows modified; 2 scripts and the inventory TSV are new. No `.rs` file changed (mutants `no-rust-delta`).
- Gates: 20 green, 1 recorded, 3 deferred (fmt, clippy, unit: zero Rust delta), 3 operator legs.

## Drift resolved
- 36 amendments: architecture 13, test-plan 15, security-plan 7, obs-plan 1.
  - 27 came from the fan-out, 6 I raised under check 5 (security's stale "owed" sites), and 3 from the cascade sweep (security `:161`; test-plan `:164`/`:1655` quoting the retired "No separate test crate").
- Leaves re-derived: CLAUDE.md overview, `docs/stack.md` (its Code quality row had lagged), `docs/commands.md`, `docs/obs-summary.md`, `rules/security.md`.
- 1 escalation, ratified by the overseer: the `supply-chain` artifact (`deny.json`, `deny-fuzz.json`, `zizmor.json`) is admissible by content (0 absolute paths measured). The guard stays binding.
- Disproved and amended:
  - cargo-modules 0.27.0 `--acyclic` fails by construction, so it is now an on-demand review only;
  - the rmcp `cargo tree` assertion is vacuous until viola-mcp;
  - a bare `cargo build --release` already yields `viola` only, and `--workspace` adds `viola-harness`.

## Notes
- **Operator decisions this chunk:**
  - wire orphans, and amend `--acyclic` and the rmcp check;
  - the ts code-graph plane is `e2e-web/` only, via a tracked `e2e-web/tsconfig.json`, and never over viola-ui;
  - the fuzz lock joins the weekly advisories;
  - the v1-23 witness condition above.
- **Route:**
  - "Security prerequisites" gets the Rust gate-deferral PREREQ;
  - "PTY wrapper on Windows" gets the cfg-gated-module orphans CARRY (hypothesis);
  - "MCP server for drivers" gets the rmcp `cargo tree` CARRY.
- **Host changes:** cargo-modules 0.26.0 → 0.27.0 (the CI pin). Created `target/release-probe`, `target/release-check`, `target/tool-build`, `target/modules-probe-research` and `target/orphans-probes`.
- **Deferred learnings:** `git check-ignore -q` on a not-yet-existing path under a trailing-slash dir pattern reads "not ignored" (confidence 0.8, from the prior wrap). It was not reproduced this session: `target/release-check` read ignored before it existed.
- The operator's viola-lab prototype (`viola.exe` 12172, 57904) was running; it is not this project's.
- Last failed command: none open.
