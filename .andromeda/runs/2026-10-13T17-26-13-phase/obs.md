# obs extract

## Relevance
partial — the chunk adds tests and possibly a behaviour-preserving restatement (a `cfg(test)` Pty double seam, named framing waits) inside `viola verify`'s Run B driver (`src/cmd/verify/typed.rs`). It adds no event, span or metric. Obs applies only as a must-not-change guard on verify's spawn logging and as the print/instrument bans on any new code.

## Constraints
- obs-plan §4 Span / Trace Coverage (`verify` / `plugin install` bullet) requires `verify`, under a `VIOLA_NAME`, to log its six child spawns as `process-start{subject}` / `process-exit{subject, child_exit_status, duration_ms}` pairs, with one `verify-pty-probe` pair for each of the four interactive PTY runs in the order Run A, B, C, D, and with no row text, prompt or path in any line. A restatement of Run B's driver must leave Run B's pair, its place in that order and its fields unchanged. Whether `typed.rs` or its caller writes that pair today is research's question.
- obs-plan §6 Log Coverage (Boundary-call wrappers, Child / shell spawns) requires verify to log its six spawns at the call site, with `run_bounded` itself unlogged. Moving the PTY behind a test double must not move the `verify-pty-probe` logging into the seam, and must not add a second pair or drop one. `schemas/diag-line.v1.json` closes `subject` at seven values (D-35).
- obs-plan §4 Span / Trace Coverage (`verify` / `plugin install` bullet) names the dispatch spans `cli.verify`, `cli.verify_step` per step and `state.ledger_write`. The chunk's "no behaviour change" boundary keeps these spans as they are. Under the same bullet, no obs gate depends on verify's own log lines, so the tests that kill the 16 survivors assert on behaviour and not on log lines.
- obs-plan §11 Obs Anti-Patterns (Logs) bans `print!` / `println!` / `eprint!` / `eprintln!` / `dbg!` in product code paths, enforced by `[workspace.lints.clippy]` `print_stdout` / `print_stderr` / `dbg_macro` = `deny` under the §9 clippy step. It also bans calling the `tracing` level macros directly outside `viola_core::obs`. A restated driver or test seam in `typed.rs` emits through neither.
- obs-plan §11 Obs Anti-Patterns (Spans / Traces) bans a bare `#[instrument]` and `#[instrument(err)]` / `#[instrument(ret)]`. Any restated function that carries a span uses `#[instrument(skip_all, fields(..))]`, because Run B's pastes are probe text (content).
- obs-plan §11 Obs Anti-Patterns (Telemetry Strategy) bans deriving telemetry from screen content. If Run B's framing waits are restated as named waits, they gate on hook-capture counts and the deadline. They emit no event and no per-chunk log.
- obs-plan §11 Obs Anti-Patterns (Logs) bans adding an `event` value without a Decisions Log entry. A test-only seam, or a restated `end_by_ctrl_c` / `wait_pump` / `finish`, adds no `event` value and no field outside the §6 catalog. obs-plan §11 Obs Anti-Patterns (PII Scrubbing) makes that catalog default-deny.

## Patterns to follow
- Spawn logging at the call site with the closed `subject` enum (`verify-pty-probe`), carrying `child_exit_status` and `duration_ms` on exit (per obs-plan §6 Log Coverage, Child / shell spawns).
- Codes-only fields: `detail` is a closed kebab-case code and never a path or pid (per obs-plan §6 Log Coverage, `detail` code catalog).
- The only output modules allowed a local `#[allow]` for a print macro are the ones that own `--json`, human CLI stderr, the hook decision body and the one-time `ui` launch line. Verify's step lines are owned output, and a test seam is not one of these modules (per obs-plan §11 Obs Anti-Patterns, Logs).

## Anti-patterns to avoid
- A test double that writes telemetry or diagnostics to stdout or stderr. stdout is reserved, and the default tracing-subscriber writer is stdout (per obs-plan §11 Obs Anti-Patterns, Logs).
- A test or test seam that echoes probe text or a home path into its captured output. Nextest stores a failing test's captured output in `target/nextest/ci/junit.xml`, which the secret scan reads (per obs-plan §9 CI Integration, step 3).

## Contract bindings
- obs ↔ tests §10 Mutation gate: obs-plan §9 CI Integration (Mutation row) and §10 SLO Invariants & Telemetry Budgets make a surviving mutant in obs code red at a `run --mutants`. The 16 `typed.rs` sites are not obs code under those rows; whether any of them sits on an `obs_event!` call site is research's question. The kills are confirmed by the Epoch 5 boundary audit's reading, not inside the chunk.
- obs ↔ tests harness: obs-plan §9 CI Integration step 3 scans `target/nextest/ci/junit.xml` because nextest stores failing tests' captured output there, so the new `typed.rs` tests fall under the secret scan.
- obs ↔ security: obs-plan §11 Obs Anti-Patterns (PII Scrubbing) applies the NEVER-log floor to any probe text a restated Run B driver handles (content, never in a home-level file).

## Acceptance criteria contributions
- With `VIOLA_NAME` set, `viola verify` over the fake agent still writes exactly six `process-start` / `process-exit` spawn pairs, with Run B's `verify-pty-probe` pair third of the four PTY pairs and carrying `child_exit_status` and `duration_ms`. Whether an existing test asserts this is research's question (per obs-plan §4 Span / Trace Coverage, `verify` / `plugin install` bullet).
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` passes with no new `#[allow]` for `print_stdout` / `print_stderr` / `dbg_macro` and no direct `tracing` level macro in `src/cmd/verify/` (per obs-plan §11 Obs Anti-Patterns, Logs).
- Any function added or restated in `src/cmd/verify/typed.rs` that carries `#[instrument]` uses `skip_all`, with neither `err` nor `ret` (per obs-plan §11 Obs Anti-Patterns, Spans / Traces).
- The chunk adds no `event` value, no `subject` value and no additive field: `schemas/diag-line.v1.json` is unchanged in the chunk's diff (per obs-plan §6 Log Coverage, Additive field catalog).
