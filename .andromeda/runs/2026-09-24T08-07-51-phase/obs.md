# obs extract

## Relevance
Partial. This chunk is tests-owned harness infrastructure. obs's stake is narrow: where the fixture chain's homes live and how long they are kept (so the CI diagnostics gates can see them), the fake agent's print-ban exemption, the synthetic-input and canary rule for fixture and script data, and the no-vacuous-pass principle the obs gates share with item 7.

## Constraints
- Every integration and E2E viola home must be under `target/e2e-home/`. A home anywhere else is missed by G2, G4, the secret scan and the `diag-<os>` upload, and a `hook` panic exits 0, so it would go unseen. This applies to the `home` fixture and to the `scratch()` migration in item 4 (per obs-plan §9 Pipeline integration "Integration tests" row; §10 "Obs overhead on `hook`" row).
- Under `AGENT_RUN_KEEP_HOMES=1` (set by `ci.yml`), an rstest `TempDir` drop must not delete a home before G2, G4, the secret scan and the scan-gated uploads have read it. If homes are deleted, a green run leaves `target/e2e-home/` empty and G2's non-empty check fails. Item 4's "always `keep()` under `AGENT_RUN_KEEP_HOMES=1`" has to hold for every fixture in the chain (per obs-plan §9 Step order and conditions, step 1 "Homes are kept until the gate steps have run"; §12 Decisions Log B1).
- The `home` fixture hands viola a not-yet-existing path. obs's init contract assumes viola creates that home itself (0700 on Unix, the protected DACL on Windows) and passes strict-modes before any `diagnostics/` file is created. The fixture must not pre-create the home or its `diagnostics/` directory with other permissions (per obs-plan §3 OTel SDK init, init step 4).
- The fake agent may print to stdout and stderr only through the crate-level `#![allow(clippy::print_stdout, clippy::print_stderr)]` in `src/bin/viola-fake-agent.rs`. It is a root-package `[[bin]]`, so it inherits the root `[lints]`, and the exemption must not spread to product code paths (per obs-plan §3 Bootstrap phases, obs-ci-gate-wire; §11 Logs, the print-ban entry).
- Every integration and E2E input must be synthetic and embed the fixed, tests-owned canary. The scan asserts the canary never shows up in a home-level `diagnostics/*.ndjson`. Scenario scripts in `fixtures/fake-scripts/*.json` and receipt-driven prompts fall under this. Uploading detail files is allowed only because inputs are synthetic (per obs-plan §8 Integration points, item 6 "Verification").
- Recorded fixtures are Medium class and must be scrubbed. That rule is owned by security and tests. obs adds no fixture content and does not treat fixtures as a telemetry source, so the scrub-and-schema walk in item 5 is not an obs gate (per obs-plan §8 data classification table, "Recorded fixtures" row; §1 Build and CI infrastructure entity).
- The fake agent's receipt lines are a harness test format, not viola diagnostics. They must not be written into `<home>/diagnostics/*.ndjson`. A receipt there would fail G4 against `schemas/diag-line.v1.json`, which uses `additionalProperties: false` and a closed `event` enum. Whether the `--receipt` path defaults could ever land inside `diagnostics/` is a question for research (per obs-plan §8 Default-deny posture; §9 G4).

## Patterns to follow
- Guard against vacuous passes the way G2 does: it first asserts that its file scope is non-empty, because an empty scope would pass vacuously. The same shape fits item 5 (a nonzero match count per glob) and item 7 (an explicit no-rust-delta verdict that names the diff) (per obs-plan §9 Gate commands, G2 comment).
- Every CI verdict must be an assertion that sets an exit code and names what it checked, never human review. This applies to the item 7 `mutants` verdict and the item 5 walk (per obs-plan §11 CI, "NEVER use human-review-gated log analysis").
- When a check fails, report `file`, line and the failing schema keyword or pattern class, never the offending content. This is a good model for the item 5 walk's failure output on planted bad inputs (per obs-plan §9 G4; §8 Scan failure hit report).
- Set env per child with `Command::env`, never with process-global env. This matches obs's rule that env vars are not a configuration channel for the product (per obs-plan §11 Universal and Logs, the `RUST_LOG` / env-filter ban).

## Anti-patterns to avoid
- Do not add retry-once policies or sleeps to fake-agent control-file sequencing or fixture assertions. Emission and waiting must be deterministic (per obs-plan §10 Standard+ invariants "Deterministic emission"; §11 SLO, the retry-once ban).
- Do not let the fake agent's `#![allow]`, or a copy of it, reach `hook`, `run` or `mcp` product code paths (per obs-plan §11 Logs, the print-ban entry).
- Do not put real `CLAUDE*` values, real usernames or absolute paths in scripts, receipts or fixtures (per obs-plan §11 PII Scrubbing, the `CLAUDE*` never-log entry; §8 table, the "Absolute paths" row).

## Contract bindings
- obs §9 Step order, step 1 ↔ tests §3 `cleanup` step 6: the rstest `TempDir` keep semantics under `AGENT_RUN_KEEP_HOMES=1` / `AGENT_RUN_KEEP_FAILED=1` are what keep G2, G4, the secret scan and the `diag-<os>` upload working.
- obs §9 Integration tests row ↔ tests fixture layout: `target/e2e-home/` is the root that the gates' `find` scope depends on.
- obs §4 `verify` note ↔ tests `stamped_home`: obs expects `stamped_home` to be the CI writer of `ledger/stamps.json` through the fake agent. This chunk ships only an interim seam. No obs gate depends on verify's log lines, but once real stamping lands, its home-level lines fall under G2, G4 and the secret scan.
- obs §8 item 6 ↔ tests secret-scan test and canary (tests-owned): fake-agent scripts and prompts are the synthetic inputs that must carry the canary.
- obs §3 obs-ci-gate-wire ↔ the CI member-list assertion: the fake agent's exemption is a bin-level allow, not a separate lint table.

## Acceptance criteria contributions
- (obs) Every home created by the `home` → `booted_wrapper` chain, and by the migrated `scratch()` users, resolves under `<workspace>/target/e2e-home/`. None is created in the system temp dir (per obs-plan §9 Pipeline integration, Integration tests row).
- (obs) With `AGENT_RUN_KEEP_HOMES=1`, a passing test's home still exists after the test process exits. A test proves the keep path (per obs-plan §9 Step order and conditions, step 1, B1).
- (obs) `cargo clippy -D warnings` stays green. The only print allowance in `src/bin/viola-fake-agent.rs` is the crate-level `#![allow(clippy::print_stdout, clippy::print_stderr)]`, and no new `#[allow(clippy::print_*)]` appears in product crates (per obs-plan §11 Logs; §3 obs-ci-gate-wire).
- (obs) Checked-in `fixtures/fake-scripts/*.json` prompt text is synthetic, containing no absolute paths, real usernames or `CLAUDE*` values, and is ready to carry the tests-owned canary (per obs-plan §8 Integration points, item 6).

## Relevant amendment history
- 2026-09-24-three-os-ci-headless-harness-skeleton, "fake agent's print-ban exemption" (§3 obs-ci-gate-wire, §11 Logs). The fake agent is a root-package `[[bin]]`, and lints are per package, so it cannot have its own lint table. It is exempted by a crate-level `#![allow(clippy::print_stdout, clippy::print_stderr)]` in `src/bin/viola-fake-agent.rs`, and the CI member-list assertion names only `viola-e2e`. Why: the previous chunk's report found that a separate lint table is impossible for a root-package bin. This chunk grows that same file, so the exemption and its scope carry over as-is. Widening the print exemption itself remains the "Observability gates" chunk's CARRY, per the scope's Boundaries.
