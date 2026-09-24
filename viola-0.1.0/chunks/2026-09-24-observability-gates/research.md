# Codebase Research — 2026-09-24-observability-gates

## Scope
- **Depth:** deep on CI, lint config and the root test chain. The gates are new, and their subjects already exist. · **Reads:** 19 · **Globs/Greps:** 16
- **Harness rules consulted:** `.claude/rules/verification-harness.md`, read in full with 1 Session Addition applied (the `--in-diff` note: a later chunk never regenerates an earlier chunk's missed mutant). `.claude/rules/testing.md`, read in full with 5 Session Additions applied: the observable-effect rule, wait-on-the-asserted-line, one test per global `OnceLock`, and the per-OS I/O-error rule.
- **Platform issues consulted** (P5 check 9; the runner-only bullet, scope item 7):
  - `gh search issues --repo sourcefrog/cargo-mutants "Auto-set test timeout"` → 5 closed issues (#544, #406, #365, #327, #295), none about the auto timeout's 20 s floor grading a hung test.
  - `… "minimum timeout 20 seconds"` → #20, "Test fails in cargo-mutants (unmodified) but works outside it", unrelated.
  - `gh search issues --repo actions/runner-images "nextest test threads 2 vCPU"` → #2, unrelated.

  No platform defect matches. The recorded per-mutant logs name the cause, a test-side wait, and no instrumentation is planned.

## Files inspected
- `src/main.rs` (1–80): `main` (`:42`) opens with `std::panic::set_hook(Box::new(viola_panic_hook))` at `:43`, then `catch_unwind(cmd::dispatch(..))`. The hook writes one `write_all` line to the role file and nothing to stderr. The `:43:5` mutant position is the body of `main`.
- `src/cmd/mod.rs` (`:27` `enum Command { Run }`, `:39` the dispatch body): **`run` is the only verb at HEAD.** There is no `hook`, `mcp`, `ui`, `list` or `--json`.
- `src/obs.rs`: doc lines `:2`, `:261` and `:327` say nothing there writes stdout or stderr.
- `tests/support/home.rs` (full): `READY_WITHIN = 20 s` (`:16`). `TestHome::new` puts every root-chain home under `target/e2e-home/viola-test-*/home` (`:37-47`) and keeps it on `AGENT_RUN_KEEP_HOMES=1` (`:60-70`). `Wrapper::boot` (`:117-148`) spawns `viola run` and calls `wait_ready`. **`wait_ready` (`:151-172`) polls only the role file and never consults `self.child`.** A wrapper that has exited is waited on until the 20 s deadline, in a `yield_now` busy loop.
- `tests/run_cli.rs` (`:189-215`): `run_self_exit_carries_duration_ms` has the same shape. It waits up to 20 s for `"subject":"claude-child"` in the role file and never calls `child.try_wait()`. Canary constants sit at `:21` and `:365-367`, and the canary-hit reporter at `:376-409` (as `<where> <variable>`, never the bytes).
- `tests/cli_fake_agent.rs` (`:523-580`, `:590`, `:606`): `run_and_watch_stdout` plus `fake_agent_exit_no_eof_exits_while_stdout_is_held`, which asserts at `:558`. The fixture consumers are `chain_boots_…` (`:583`) and `booted_wrapper_fixture_is_ready_and_receipting` (`:606`). `:575` has a 5 s EOF deadline.
- `tests/support/fake.rs` (`:12`): `WAIT_WITHIN = 10 s`, a receipt wait that is reached only after a successful boot.
- `.config/nextest.toml` (full): `[profile.mutants]` sets `fail-fast = { max-fail = 1, terminate = "immediate" }` and `slow-timeout = { period = "15s", terminate-after = 2 }`, which is a 30 s kill. `[profile.ci]` has `retries = 0` and junit at `junit.xml`.
- `crates/viola-e2e/src/harness/run.rs` (`:400-460`): `run --mutants` runs `cargo mutants --workspace --features fake-agent --in-diff <chunk.diff> --test-tool=nextest --copy-target=true` with `NEXTEST_PROFILE=mutants`. It passes no `--timeout`, so the timeout is cargo-mutants' auto value.
- `crates/viola-e2e/src/harness/boot.rs` (`:21-22`, `:118-128`, `:220-278`): harness homes live at `target/e2e-home/viola-session-*/home`. `INSTANCE_DEADLINE = 20 s`. Harness readiness sees an exit only through a `process-exit{self}` line (`:207`, `:229`), so a wrapper that exits without writing any line waits the full deadline. This code is not reached by a root-package mutant run: the mutants baseline runs only root-package binaries.
- `crates/viola-e2e/src/bin/viola-harness.rs` (`:20-67`): subcommands `boot · run · status · cleanup · logs · supervise`. **No `gate` subcommand exists at HEAD.** The harness prints at `:71`, `:80` and `:146`, which is exempt by design.
- `crates/viola-e2e/Cargo.toml`: `[lints.rust] unused_must_use = "deny"` only, with no `[lints.clippy]` table.
- `Cargo.toml` (`:50-95`): the root `[lints] workspace = true`. `[workspace.lints.rust]` has only `unused_must_use`, and **no `[workspace.lints.clippy]` exists**. Members are `crates/*`. Both profiles are `panic = "unwind"`.
- `crates/viola-core/Cargo.toml`: `[lints] workspace = true`. Its only dependency is `nutype`, with no `tracing`. `crates/viola-core/src/obs.rs:152-158`: `obs_event!` expands to `::tracing::event!` under `#[allow(clippy::disallowed_macros)]` (`:155`). The arch placement question in the arch extract resolves to no amendment.
- `clippy.toml`: **absent.**
- `.github/workflows/ci.yml` (full): `permissions: {}`. Jobs:
  - `test`: 3 OSes, with `AGENT_RUN_KEEP_HOMES: "1"` at job level (`:19-20`), unit and integration through the shims, and the harness lifecycle. `:74-82` uploads `agent-run-${{ matrix.os }}` from `target/agent-run/artifacts/` under a bare `if: always()`, with **no secret scan before it**.
  - `mutants`: ubuntu.
  - `lint`: 3 OSes, running **only** the Tokio-free `cargo check`, with **no `cargo fmt` and no `cargo clippy` step**.
  - `supply-chain`: ubuntu.

  There are no G1–G4 steps, no secret scan and no junit/diag upload.
- `rust-toolchain.toml`: 1.98.1 with components `rustfmt` and `clippy`.
- `scripts/deny-probes.sh` (`:1-40`): the both-ways proof pattern. It builds throwaway Cargo projects under `target/deny-probes/run-*` with an empty `[workspace]` table, asserts one expected diagnostic per probe, and runs a clean control.
- `.gitignore:8-9`: `target/` covers `target/e2e-home/` and `target/agent-run/`.
- `.andromeda/test-plan.md:1230-1236` (§6 Error sanitization and secret scan; Schema conformance) and `:1412` (§9 tooling): jaq 3.1.1 via `cargo install --locked` in the jobs that invoke it, and each tool has one version source. `.andromeda/obs-plan.md:1196-1209` (§8 item 6, the `target/secret-scan/` hit report) and `:1223-1283` (§9).

## Graph impact (`.andromeda/runs/2026-09-24T11-55-47-phase/tree-query-2026-09-24-observability-gates.json`)
- **`wait_ready`**: 5 callers. The only root-chain caller is `Wrapper::boot` @ `tests/support/home.rs:145`. The other four are the viola-e2e harness's own `wait_ready` (`boot.rs:145`, `:394`, `:404`, `:423`), a distinct symbol that shares the name. Changing the root `wait_ready` changes no signature.
- **`boot`** (callee_file `home.rs`): 2 callers, `chain_boots_the_fake_agent_under_viola_with_a_gated_script` @ `tests/cli_fake_agent.rs:591` and `booted_wrapper` @ `tests/support/home.rs:213`. `booted_wrapper` is consumed once, at `tests/cli_fake_agent.rs:607` (grep `booted_wrapper\|Wrapper::boot` over `tests/`: 4 hits). These are exactly the two tests SIGTERM'd in CI.

## Measured facts (the scope item 7 witness: run `35995290314`, sha `2834e4d42bb61fe610ed160f7d1a89fb4815cbdf`, job `mutants`, artifact `mutants` downloaded with `gh run download 35995290314 -n mutants`)
- `log/baseline.log`: `Starting 125 tests across 6 binaries`, `Summary [1.346s] 125 tests run: 125 passed`. The binaries are root-package only: `bin/viola` 84 lines, `bin/viola-fake-agent` 18, `cli_fake_agent` 64, `contract_diag_schema` 18, `contract_fixture_hygiene` 30, `run_cli` 36 (grep of the `) <binary>` prefix). The mutation gate's step log shows `Auto-set test timeout to 20s` and `Unmutated baseline in 44s build + 1s test`: the 1 s baseline puts cargo-mutants at its 20 s floor.
- `log/src__main.rs_line_43_col_5.log:591-620` and `log/src__cmd__mod.rs_line_39_col_5.log:368-397`: in both mutants, only **2 of 125 tests started**, both `wait_ready` consumers:
  - `booted_wrapper_fixture_is_ready_and_receipting`
  - `chain_boots_the_fake_agent_under_viola_with_a_gated_script`

  Each went `SLOW [> 15.000s]` and was killed by `SIGTERM [19.82s]` when cargo-mutants hit its 20 s timeout (`*** result: Timeout`). Two tests running at once means the runner gave nextest 2 test threads.
- **Mechanism, re-derived.** The overseer's claim was marked as a hypothesis and is verified here:
  - Both mutants make `viola` exit 0 at once and write no role line. `main → Default::default()` never runs `dispatch`. `dispatch → Ok(Default::default())` returns before `run` opens a sink.
  - `Wrapper::wait_ready` never looks at the child, so it cannot tell "exited" from "not ready yet". It spins to `READY_WITHIN = 20 s`.
  - 20 s ≥ cargo-mutants' 20 s timeout, so the run is graded Timeout before either test can fail.
  - `[profile.mutants]`'s 30 s nextest kill (15 s × 2) is also ≥ 20 s, so nextest's own termination can never pre-empt cargo-mutants on a hang. The profile comment "a hung test is killed at 30 s" never takes effect under the 20 s floor.
- **Why Windows caught the same mutants** (local `mutants.out/`, `outcomes.json`): the same 20 s auto timeout applied (`debug.log`: `Auto-set test timeout to 20s`), and each mutant was caught in 0.66 s / 0.91 s. The first FAIL was `fake_agent_exit_no_eof_exits_while_stdout_is_held` (`tests/cli_fake_agent.rs:558`, `stdout reached EOF`), which the Windows host scheduled alongside the waiters (more threads). The kill depends on scheduling: on a 2-thread runner that happens to start the two waiters first, no fast-failing test ever starts.
- **In-test deadlines a root-package mutant can reach** (grep `Duration::from_(secs|millis)\([0-9]+\)|READY_WITHIN|deadline` over `tests src crates`; the viola-e2e rows are outside the root mutants run):

  | Site | Deadline | Reachable? |
  |---|---|---|
  | `tests/support/home.rs:16` | 20 s | yes, it hangs both mutants |
  | `tests/run_cli.rs:202` | 20 s | yes, same shape |
  | `tests/support/fake.rs:12` | 10 s | only after a successful boot |
  | `tests/cli_fake_agent.rs:575` | 5 s | yes |
  | `.config/nextest.toml` `[profile.mutants]` | 30 s | on any hang |
- **Readiness in practice:** `booted_wrapper_fixture_is_ready_and_receipting` passes in 0.010 s on ubuntu (ci run log, `(1/86)`), so a readiness deadline far below 20 s keeps a large margin.

## Patterns detected
- **Exit-aware readiness (harness)** (`crates/viola-e2e/src/harness/boot.rs:229-231`, `:264-266`): readiness returns `Exited(code)` → `run-exited` as soon as the wrapper's exit is observable. The root chain lacks the equivalent. It holds the `Child`, so `try_wait()` is the direct probe (test-plan §3 Readiness: a `process-exit` for `self` arriving first means `run-exited`).
- **Throwaway-project probes** (`scripts/deny-probes.sh:12-40`): prove each ban both ways with a clean control, fail closed on a missing tool (`tool-missing`), and keep scratch under `target/<probes>/run-*`.
- **Canary reporting without bytes** (`tests/run_cli.rs:376-409`): hits are reported as `<where> <variable>`.
- **Job-level env, SHA pins, `persist-credentials: false`, per-job `contents: read`** (`ci.yml` throughout).

## Conventions to follow
- **Waits are bounded file-state or pid probes, never `sleep`** (testing.md Determinism; `tests/support/fake.rs:2`).
- **Tests named `<subject>_<condition>_<expected>`**, with root sync suites at `tests/{cli,hook,…,contract}_<topic>.rs` (testing.md Naming).
- **Tool versions: one source each.** Cargo tools come from taiki-e/install-action `@7623a79…` or `cargo install --locked` (test-plan `:1412`). Actions are SHA-pinned (`ci.yml:22,27,28,76`).
- **Host shell:** `rg` exists in the Bash tool only as a shell function that wraps the Claude binary's embedded ripgrep 14.1.1 with PCRE2 10.45. A clean `bash --noprofile --norc` finds no `rg` (measured), so gate-shell commands must not rely on it. The official ripgrep release is 15.2.0 (`gh release view -R BurntSushi/ripgrep`, 2026-07-15). `jq` is on the host through scoop; `jaq` is absent.

## New files to create
- `clippy.toml`: `disallowed-macros` for `tracing::{event,info,warn,error,debug,trace}` (obs §3).
- `tests/contract_lints.rs`: the member-list assertion (every workspace member except `viola-e2e` carries `[lints] workspace = true`), plus the panic-hook-first source-order witness. `[inferred: the placement is P4's call]`
- `scripts/lint-probes.sh`: throwaway-project both-ways proofs for `print_stdout`, `print_stderr`, `dbg_macro` and `disallowed-macros`, with clean controls, following `deny-probes.sh`. `[inferred: P4's call]`
- The G4 schema-conformance and secret-scan check bodies (tests-owned; jsonschema 0.57.0 is already a root dev-dependency). The vehicle is an open question below.

## Files to modify
- `Cargo.toml`: add `[workspace.lints.clippy]` `print_stdout` / `print_stderr` / `dbg_macro` = `deny`.
- `crates/viola-e2e/Cargo.toml`: add its own `[lints.clippy]` table without the print lints (obs §3).
- `src/bin/viola-fake-agent.rs`: crate-level `#![allow(clippy::print_stdout, clippy::print_stderr)]` (CARRY).
- `tests/support/home.rs`: `wait_ready` fails fast when `self.child.try_wait()` reports an exit, plus the deadline value (open question).
- `tests/run_cli.rs:202`: the same exit-aware wait.
- `.config/nextest.toml`: `[profile.mutants]` slow-timeout below cargo-mutants' 20 s floor (open question).
- `.github/workflows/ci.yml`:
  - `lint` job: add `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`, the ripgrep install, `rg --pcre2-version`, G1, G3 and the lint probes.
  - `test` job: add G2, G4, the `if: failure()` harness capture, the `secret-scan`, and the scan-gated `diag-`/`harness-`/`junit-<os>` uploads plus the `secret-scan-<os>` upload.
  - Replace the unscanned `agent-run-<os>` upload (`:74-82`).
- Companion sweep: `READY_WITHIN` has 2 hits, both in `tests/support/home.rs` (def + use). `wait_ready` in `tests/` has 2 hits, both in `home.rs`. No test pins the deadline value.

## Open questions
- **The vehicle for the G4 and secret-scan check bodies** (tests-owned Rust with jsonschema 0.57.0) → blocks: plan-decision. Options: (a) internal `viola-harness` subcommands beside `supervise` (not agent commands, so the 5-command surface is unchanged); (b) a root integration test that walks `target/e2e-home/**`, which nextest orders unpredictably; (c) a new bin.
- **G2's `jq` versus test-plan §9's jaq/single-version-source rule** → blocks: plan-decision. obs §9 copies G2 verbatim with `jq -R -n -e`, which is preinstalled on runner images. test-plan `:1412` installs jaq 3.1.1 in the jobs that invoke it.
- **How far below 20 s the reachable deadlines go** (`READY_WITHIN`, `run_cli.rs:202`, the `[profile.mutants]` kill) → blocks: plan-decision. test-plan §3 states the readiness bound as 20 s, so lowering it is a wrap amendment. With exit-aware readiness alone, these two mutants no longer reach a deadline. A mutant that keeps `viola` alive without writing `process-start` still would.
