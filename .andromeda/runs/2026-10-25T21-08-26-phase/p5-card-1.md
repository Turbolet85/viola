# P5 review — 2026-10-25-windows-endpoint-admission · "Windows endpoint admission"

Plan: `viola-0.1.0/chunks/2026-10-25-windows-endpoint-admission/plan.md`. Base HEAD `b6740d59`. The four forks were
the founder's own answers, live (`inputs#I5`).

## Goal
Assert on CI's `test (windows-2025)` leg the two Windows admission controls that nothing pins today:
- **remote pipe clients rejected** — the bound listener's pipe type is read back through `NtQueryInformationFile` and
  must carry `FILE_PIPE_REJECT_REMOTE_CLIENTS`, with an accept-remote and a reject-remote control;
- **the DACL's effect** — an anonymous-token open of the listener is denied, while the same open of a
  default-descriptor pipe succeeds.

The DACL text read-back, non-inheritance, the first-instance `BindTaken` cases and the `squatted-name` refusal are
already asserted; P3 corrected scope for them, and they stay unedited.

The CARRY's case_4 is closed at its root cause. tracing-core 0.1.36 caches a callsite as `never` when a subscriber-less
thread registers it first while one dispatcher is registered. The fix is a shared root-bin test helper that keeps a
second, always-interested dispatcher registered, with all 9 scoped captures routed through it, and a witness case
red before and green after.

No product file changes. The touchpoints are the negatives file, the root `Cargo.toml` (one Windows dev-dependency
feature) and `#[cfg(test)]` code in 8 `src/` files.

## Top acceptance criteria
- `security_negative_viola_listener_rejects_remote_clients` and `…_denies_an_anonymous_open` green on
  `test (windows-2025)`, each with its control(s). A control that cannot tell the two apart there moves that claim to
  "Windows-only live measurements".
- The capture witness reads red on bare `with_default` and green on the helper (`evidence/capture-witness.md`). Under
  nextest it reads `1 test run: 1 passed`.
- `cargo test --bin viola` shows case_4 and the witness `ok`. Its three known-cause reds are recorded, with no owner.
- The census `grep -rn 'subscriber::with_default' src | wc -l` reads `1`. `run --unit`, `pre-push`, `deny bans licenses
  sources`, the tokio-free tree, G2 and G4 all read green.
- `ci.py --name 'test (windows-2025)'` reads `verdict: green`. The full CI row is recorded, and `supply-chain` is its
  only red.

## Test commands (24 entries; baselines on the untouched tree)
**Seen green at the last wrap (8):** fmt, clippy, the Windows-target clippy, G1, lint-probes, `run --unit`, G2, G4.

**New, green (9):**

| # | Entry | Baseline |
|---|---|---|
| 1 | `/tmp` reading | 26G free of 32G |
| 11 | deny bans licenses sources | green |
| 12 | tokio-free tree | green |
| 13 | guard | clean; its control over a clone reads red on an outside edit plus an untracked file |
| 14, 15, 16, 19 | smoke | green |
| 20 | `pre-push` | green, 99.7 s |

**New, red on the subject the chunk builds (3):**

| # | Entry | Baseline | Control |
|---|---|---|---|
| 8 | witness filter | 0 selected | the literal's must-pass reads `1 test run: 1 passed` |
| 9 | census | 9 | a clone with the sites rerouted reads 1 |
| 10 | plain run | only the witness line is absent; case_4 reads ok | — |

**Operator (4):** hygiene, push, the Windows-leg CI read (asserted) and the full CI read (report-only).

## Concretization preview
No capability is claimed. `v1-44` («The IPC endpoint admits only the same OS user and verifies its server») covers
every OS, client-side server verification and the macOS gap. Those are owned by "Server verification before any
frame", "Unix endpoint and home hardening" and "Linux and macOS parity", so this chunk can't make it fully verifiable.
It stays pooled, and after the word one dated `notes` line records why.

## Sampler (forced, `inputs#I1`)
- architecture: `sampler: E carried 40 · the cascade lost 6 (established-decisions#14.1, established-decisions#8.1, stack-and-technologies#1.1, stack-and-technologies#1.23, stack-and-technologies#1.4, standard-contracts#2.0)`
- security-plan: `sampler: E carried 34 · the cascade lost 4 (bootstrap-phases#2.1, error-handling#1.1, input-validation#1.3, security-anti-patterns#1.1)`
- design-system: `sampler: E carried 8 · the cascade lost 0`
- test-plan: `sampler: E carried 60 · the cascade lost 8 (ci-integration#1.7, integration-test-strategy#3.1, quality-gates-coverage-targets#2.0, quality-gates-coverage-targets#3.1, test-data-fixtures#1.3, test-scope-summary#6.1, test-strategy#1.7, unit-test-strategy#1.2)`
- obs-plan: `sampler: E carried 56 · the cascade lost 5 (ci-integration#1.3, error-capture-reporting#2.1, slo-invariants-telemetry-budgets#2.2, span-trace-coverage#8.2, telemetry-strategy#1.4)`
- a11y-plan: `sampler: E carried 9 · the cascade lost 0`

Every lost item is taken in the plan's Provenance, marked `(sampler)`. One of them (test-scope-summary.md:80, «`cargo
test`») is corrected there against the harness source, `crates/viola-e2e/src/harness/run.rs:238-271`, which runs
nextest.

## Notes for the reviewer
- **Remove-the-guard.** The witness's red reading is deterministic only in its own process (nextest). Implement takes
  it there before step 3.
- **Mechanism provenance.** The case_4 mechanism is read from the pinned tracing-core 0.1.36 source (research.md, P4
  additions), not reproduced live. The witness is what reproduces it.
- **Expected reds.** CI's `supply-chain` stays red while the lock-bump entry is pending (`inputs#I2`). The plain-run
  reds stay, by the founder's ruling (`inputs#I5`).

## Beyond the plan
- **Route insertion (wrap P5, route-contract §Write row 17).** A new working-route entry, "tokio RUSTSEC-2026-0386 lock
  bump", placed right after "Windows endpoint admission" so it runs next. It owns the `supply-chain` red: a lock bump
  of `tokio` to `>=1.53.3`, no ignore-list widening. This is the overseer's founder-delegated answer at this P1
  (`inputs#I2`).
- **Same-class captures left out, no owner.** The crate-level scoped captures `crates/viola-state/src/lib.rs:87`,
  `crates/viola-channel/src/test_support.rs:184` and `crates/viola-channel/src/lib.rs:199` share the mechanism. They
  have never been observed red, so no route entry is proposed. They are recorded in the plan and in the wrap's report.
