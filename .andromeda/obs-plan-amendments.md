# obs-plan — amendments

## 2026-09-24-three-os-ci-headless-harness-skeleton — fake agent's print-ban exemption
**Section:** §3 Bootstrap phases → obs-ci-gate-wire · §11 Obs Anti-Patterns → Logs
**Change:**
- Only `viola-e2e` omits `[lints] workspace = true` and carries its own `[lints.clippy]`.
- The fake agent is a `[[bin]]` of the root `viola` package. Lints are per package, so it inherits the root `[lints]` and is exempted by a crate-level `#![allow(clippy::print_stdout, clippy::print_stderr)]` in `src/bin/viola-fake-agent.rs`.
- The CI member-list assertion names only `viola-e2e`.
- The §11 carve-out lists that bin-level allow.

**Why:** report Changes > Files (`src/bin/viola-fake-agent.rs` is a root-package bin) and Deviation 7: a separate lint table is impossible for it. Sweep `only these two lack|these two members omit|own \[lints\.clippy\]` over all 7 masters:
- obs-plan.md:790 and :1373 amended;
- obs-plan.md:1748 no change (§12 Decisions Log history, B5);
- 0 in the other masters.

The leaf `.claude/rules/verification-harness.md` §Exemptions was re-derived.
