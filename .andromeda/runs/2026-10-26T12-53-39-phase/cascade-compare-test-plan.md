# Cascade compare — test-plan · phase · 2026-10-26-explicit-session-links
E items 169 · held 33 · lost at classify 2 · lost at strong 2 · beside a placed fact - · cascade only 8 · listed 9 · body 87 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### ci-integration#1.9 · lost at strong · .andromeda/test-plan-detail/ci-integration.md:43
- (.andromeda/test-plan-detail/ci-integration.md:43) The verbs and channel methods must compile and their unit cases pass on the 1.96 MSRV toolchain.
  > ubuntu `msrv`: `rustup toolchain install 1.96 --profile minimal`, then `RUSTUP_TOOLCHAIN=1.96` steps for `rustc --version`, `cargo check --workspace` and `agent-run run --unit`, then `gate --require nextest-unit`;

### quality-gates-coverage-targets#2.0 · lost at classify · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:16) Any `cfg(windows)` arm the verbs add that the Linux dev host cannot reach is recorded by coordinate as owed, never claimed caught or equivalent.
  > A mutant this host cannot compile or reach — inside a `cfg(windows)` item or block, or behind a const such as `HOST_SCRATCH = cfg!(windows)` — is recorded "not measured here; owed to {the route entry that measures it}" by coordinate, never "equivalent", and never counted as caught;

### quality-gates-coverage-targets#3.1 · lost at classify · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:26
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:26) Adding the two methods to the wrapper's dispatch must leave the timed hook rows (which run through that same wrapper and their server check) under the spine deadline in the per-OS `perf` job.
  > Every timed hook also makes its server check (the peer pid and its OS start time against the one snapshot read) before its first frame; with the check in the timed path the hook rows read `max` 2.2–2.4 ms over 30 runs each on the Linux dev host, as measured at `viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md` (Outcome, `run --perf`).

### test-scope-summary#6.1 · lost at strong · .andromeda/test-plan-detail/test-scope-summary.md:76
- (.andromeda/test-plan-detail/test-scope-summary.md:76) Scenario E3 needs two wrappers (`overseer`, `builder`), which `boot` must start in one home.
  > Starts one or more `viola run <name> -- <fake agent> <args>` wrappers.

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### ci-integration#1.6 · listed (lost at classify) · .andromeda/test-plan-detail/ci-integration.md:31
- (.andromeda/test-plan-detail/ci-integration.md:31) The chunk's new root cases (`tests/channel_links.rs`, the `link` / `unlink` cases of `tests/cli_links.rs`, the moved `HELP` literal) run inside each OS's one instrumented coverage run, which must end in a passing `gate --require coverage,doctest`.
  > Its one instrumented nextest run replaces the Unit, Integration and `run --e2e` rows, which are the local `agent-run` equivalents, and the doctest run follows it. Each job's last step is `viola-harness gate --require <suites>`: the per-OS test job uses `coverage,doctest`, and the Perf, Fuzz replay and MSRV jobs use `perf`, `fuzz-replay` and `nextest-unit`

### e2e-test-strategy#1.4 · listed (lost at strong) · .andromeda/test-plan-detail/e2e-test-strategy.md:13
- (.andromeda/test-plan-detail/e2e-test-strategy.md:13) The channel cases (`tests/channel_links.rs` and the raw-frame negative) must drive the wrapper through the sync client or a raw stream with jsonschema, the endpoint read only from `snapshot.json`.
  > | ipc-internal (wrapper channel) | viola-channel sync client + interprocess 2.4.4 raw stream + jsonschema 0.57.0 | direct | The endpoint is read only from `snapshot.json`. |

### e2e-test-strategy#13.1 · listed (lost at strong) · .andromeda/test-plan-detail/e2e-test-strategy.md:254
- (.andromeda/test-plan-detail/e2e-test-strategy.md:254) The `-32602` errors and every `--json` document of the new methods and verbs must pass the per-test sanitization: no absolute path, serde path or `Caused by`, and every `-32603` exact.
  > - **Error sanitization and secret scan:** after every E2E test, the `events` support helper checks:
  >   - CLI `--json`, MCP `isError` and channel `error.data` contain no absolute path (drive letter, `/home/`, `/Users/`, the temp home prefix), no serde path and no `Caused by`
  >   - every -32603 is exactly `"internal error"` / `null`

### integration-test-strategy#1.4 · listed (lost at classify) · .andromeda/test-plan-detail/integration-test-strategy.md:18
- (.andromeda/test-plan-detail/integration-test-strategy.md:18) The channel cases must run over a real endpoint bound in a temp home, with frames validated by jsonschema.
  > Wrapper channel: real endpoint bound by `viola-channel` server in a temp home, with frames validated by jsonschema.

### integration-test-strategy#2.0 · listed (lost at classify) · .andromeda/test-plan-detail/integration-test-strategy.md:32
- (.andromeda/test-plan-detail/integration-test-strategy.md:32) Each link / unlink test must start from its own fresh tempfile home.
  > there is a fresh tempfile home per test (the rstest `home` fixture).

### integration-test-strategy#2.0 · listed (lost at classify) · .andromeda/test-plan-detail/integration-test-strategy.md:33
- (.andromeda/test-plan-detail/integration-test-strategy.md:33) Under the instance-check form, a case that drives the check-guarded verb path on every OS must start from `stamped_home`, because a seeded home fails the instance check on Windows. The booted `overseer` / `builder` wrappers must sit in the kill-and-wait `Drop` guard.
  > Such a home fails `hook statusline`'s instance check on Windows (as measured at chunk 2026-10-10-statusline-pass-through, CI run 38080061631, three cases refused; the home's DACL not read), so a case that drives a check-guarded path on every OS starts from `stamped_home` (§7 Seed strategies).

### integration-test-strategy#2.0 · listed (lost at classify) · .andromeda/test-plan-detail/integration-test-strategy.md:33
- (.andromeda/test-plan-detail/integration-test-strategy.md:33) No wrapper or state may be shared between the link cases.
  > Wrappers started in a test are held in a `Drop` guard that calls `Child::kill()` then `wait()`. Nothing is shared between tests.

### quality-gates-coverage-targets#1.1 · listed (lost at classify) · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:8
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) The `viola link` / `viola unlink` children the root cases (`tests/channel_links.rs`, Scenario E3) spawn are counted toward coverage only if their profile env survives; any case that clears the env must re-add it.
  > Child `viola` processes spawned by E2E tests are included, because `LLVM_PROFILE_FILE` propagates. Tests that call `env_clear()` must re-add it.

### quality-gates-coverage-targets#1.1 · listed (lost at classify) · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:8
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) A new case that starts an instrumented child and kills it straight after must not leave a short profile that fails the coverage merge; such a case takes an uninstrumented child or lets the instrumented one finish its exit.
  > One mechanism for such a profile is measured (as measured at chunk 2026-10-07-a-send-ending-in-a-newline-is-confirmed, `evidence/profraw-red-green.md`): an instrumented child that exits by itself and is killed during its exit-time profile write leaves a short profile, which `llvm-profdata` refuses.

### test-scope-summary#7.1 · listed (lost at classify) · .andromeda/test-plan-detail/test-scope-summary.md:106
- (.andromeda/test-plan-detail/test-scope-summary.md:106) Every link / unlink test must seed its own state in a fresh home.
  > Self-bootstrapping per test: a fresh `--home` temp dir.

### test-strategy#2.1 · listed (lost at classify) · .andromeda/test-plan-detail/test-strategy.md:42
- (.andromeda/test-plan-detail/test-strategy.md:42) `tests/channel_links.rs` and any `cli_` link file must take the root sync-suite naming and use the shared `tests/support/` reader, wait, runner and complete-lines reader, never a copy of their own.
  > **Sync (tokio-free) E2E suites:** root `tests/{cli,hook,tui,channel,chaos,contract,security_negatives}_<topic>.rs`, sharing helpers in `tests/support/{home.rs,outer_pty.rs,fake.rs,events.rs,cli.rs,ndjson.rs}`. `events.rs` holds the one `events.ndjson` reader and its wait (`events`, `wait_events`) and the `boot` that waits for the session-start record; `cli.rs` holds the one started-child guard and runner (`Running`, `Ran`, `spawn`, `viola`), with stdout and stderr as two captures beside the exit code, and `spawn_on` / `viola_on`, which put a planted bin dir on the child's `PATH`; a plain `spawn` of `list` runs with `PATH` set to an empty bin dir, so no test reads the host's `claude` sessions. `fake.rs` holds `empty_bin`, `claude_bin` and `claude_in`, the one place a test plants the fake agent as `claude` (chunk 2026-10-16-unwrapped-sessions-on-the-board). No root test file outside `tests/support/` defines its own copy of the reader, the wait, the guard or the result struct. `ndjson.rs` is the one complete-lines reader: every reader of a live-appended file parses only newline-terminated lines, because a Linux reader can observe an append part-way.

### test-strategy#2.1 · listed (lost at classify) · .andromeda/test-plan-detail/test-strategy.md:48
- (.andromeda/test-plan-detail/test-strategy.md:48) A trycmd case for the three human result lines must live under the trycmd home.
  > **trycmd cases:** `tests/cmd/*.toml`.

### test-strategy#3.18 · listed (lost at classify) · .andromeda/test-plan-detail/test-strategy.md:75
- (.andromeda/test-plan-detail/test-strategy.md:75) Every link test must keep the discipline trigger: no screen parsing, no flakes.
  > | discipline (no screen parsing / no blanket approval / no flakes) | all | §6 Scenario 4 negative; §11 |

## Held

- ci-integration#1.1
- e2e-test-strategy#1.1
- e2e-test-strategy#10.1
- e2e-test-strategy#11.1
- e2e-test-strategy#14.1
- e2e-test-strategy#15.1
- integration-test-strategy#3.1
- integration-test-strategy#3.2
- integration-test-strategy#4.1
- mocking-stubbing-discipline#1.1
- mocking-stubbing-discipline#2.2
- mocking-stubbing-discipline#2.3
- mocking-stubbing-discipline#2.4
- test-data-fixtures#1.1
- test-data-fixtures#1.8
- test-data-fixtures#2.1
- test-data-fixtures#6.1
- test-scope-summary#1.1
- test-scope-summary#10.1
- test-scope-summary#3.1
- test-scope-summary#4.1
- test-scope-summary#5.2
- test-scope-summary#5.3
- test-strategy#1.1
- test-strategy#1.2
- test-strategy#1.3
- test-strategy#3.1
- test-strategy#3.12
- test-strategy#3.16
- test-strategy#3.5
- unit-test-strategy#1.1
- unit-test-strategy#1.2
- unit-test-strategy#2.1

## Cascade only

- ci-integration#1.5
- e2e-test-strategy#1.5
- e2e-test-strategy#12.1
- e2e-test-strategy#3.2
- e2e-test-strategy#6.3
- test-data-fixtures#1.7
- test-strategy#3.13
- test-strategy#3.6
