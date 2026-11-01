# tests · Quality Gates & Coverage Targets — category read

## Carried
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) Every test this chunk adds that clears a spawned `viola`'s environment must re-add the profile name, so the refusals it asserts stay in the coverage number.
  > Child `viola` processes spawned by E2E tests are included, because `LLVM_PROFILE_FILE` propagates. Tests that call `env_clear()` must re-add it.
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) A case this chunk adds that kills an instrumented child (the leftover-file takeover, the force-killed wrapper) must not land the kill in the child's exit-time profile write, or the merge of the coverage gate goes red.
  > One mechanism for such a profile is measured (as measured at chunk 2026-10-07-a-send-ending-in-a-newline-is-confirmed, `evidence/profraw-red-green.md`): an instrumented child that exits by itself and is killed during its exit-time profile write leaves a short profile, which `llvm-profdata` refuses.
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:9) A kill-shaped case this chunk adds must run its fake agent's commands in process groups of their own, so a wrapper kill's hangup cuts no `viola hook` profile.
  > A second mechanism is measured: a test that kills the wrapper closes its PTY, and the hangup reaches the hooks the fake agent ran in that terminal's foreground process group, so a `viola hook` cut off in its exit-time profile write leaves a page-cut profile that `llvm-profdata` refuses, or none; with every command the fake agent spawns in a process group of its own (the fake agent's exit bullet, §7) the census over the kill-shaped `chaos_wait_parked_wrapper_killed_exits_21` reads 0 refused profiles of 800 in 200 runs (as measured at `viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup/evidence/profraw-cause.md`).
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:9) The refused-profile count of a new kill-shaped case is read with this dev-host witness, and the chunk must wire it into no CI job, `pre-push` stage or harness command.
  > The witness is `scripts/profraw-census.sh <runs> <workers> [<name>]`, with a second form `--case <binary stem> <test name> <runs> <workers> [<name>]` over one integration case that offers each profile alone to `llvm-profdata merge` and counts the refused ones, a dev-host script (architecture §Occupied Resources → Repository): no CI job, `pre-push` stage or harness command runs it, and it has no unit test, only its recorded one-shot controls (must-fail on the untouched test, must-pass, and two planted readings).
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:15) This chunk opens Epoch 7 and must add no mutation gate of its own; mutation stays out of its gate set.
  > **Mutation gate (Founder Direction 1):** judged at the epoch boundary by `/andromeda-code-audit` and on demand by `run --mutants`; no chunk, pre-push or CI mutation gate exists since 2026-09-28 (the founder's ruling, 17:59).
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:16) A mutant in this chunk's macOS-only or otherwise host-unreachable code, on any `run --mutants`, must be recorded as owed by coordinate, never read as equivalent or caught.
  > A mutant this host cannot compile or reach — inside a `cfg(windows)` item or block, or behind a const such as `HOST_SCRATCH = cfg!(windows)` — is recorded "not measured here; owed to {the route entry that measures it}" by coordinate, never "equivalent", and never counted as caught; `missed == 0` then reads over the measurable set (the overseer's ruling, 2026-10-04).
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:25) The directory verification and the client's peer euid check must leave the perf session's hooks on the real channel path, so no timed row reads a refusal's fail-open as its timing.
  > hyperfine runs with the repo root as cwd and `VIOLA_NAME=builder` / `VIOLA_DIR=<perf home>/instances/builder` set, so every timed hook takes the real logging and channel path, never the no-instance no-op (obs-plan §10 obs-overhead row)
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:25) Whatever the chunk finds of harness `cleanup` after a force-killed Unix wrapper, a perf session's `cleanup` must still report `ok:true`, and a failure must turn suite `perf` red by name.
  > Then `cleanup` (`keep_homes` = `AGENT_RUN_KEEP_HOMES` exactly `1`) must report `ok:true`. A failed build (`build`), boot (`boot: <reason>`), row (the hook's name), panic check or cleanup turns suite `perf` red with the failure named, never a missing or skipped row.
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:26) The hook is a Unix client, so the peer euid check before its first frame and the socket directory verification before its connect join this timed path and must keep the five rows under `SPINE_DEADLINE`; the chunk's own `run --perf` is the new reading.
  > Every timed hook also runs its instance strict-modes check (`check_instance`) before any `config.json` read or obs init, and makes its server check (the peer pid and its OS start time against the one snapshot read) before its first frame; with both checks in the timed path the five hook rows read `max` 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms on the Linux dev host, under `SPINE_DEADLINE`, as measured at `viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md` (Outcome, `run --perf`).
- (.andromeda/test-plan.md:818) The code this chunk adds must hold the Comprehensive tier's thresholds.
  > | **Comprehensive (viola)** | **≥ 85%** | **≥ 80%** (enforced as region coverage `--fail-under-regions 80`) | **≥ 95%** |
- (.andromeda/test-plan.md:821) The thresholds must hold on each OS job on its own, so the chunk's `cfg(unix)` and macOS-only branches must be exercised on the runner that compiles them.
  > Gates are enforced per OS job with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`. Per-OS gating is deliberate: `#[cfg(windows)]` / `#[cfg(unix)]` code compiles only on its own runner.
- (.andromeda/test-plan.md:755) The chunk's per-OS test job is this one instrumented run, ending in the gate.
  > **Coverage report:** `agent-run run --coverage` is the per-OS test job, on `windows-2025` after the ConPTY vendor verification; its one instrumented nextest run replaces the unit, integration and E2E rows, and each job ends in `viola-harness gate --require <suites>`.
- (.andromeda/test-plan.md:759) The chunk's hook-path change must pass the perf job per OS, its verdict from the gate.
  > **Perf:** `agent-run run --perf` in its own per-OS job, its verdict from `viola-harness gate`, never from a `jq` binary.
- (.andromeda/test-plan.md:767) Any §10 breach this chunk causes must fail the gate.
  > **Quality gates:** `viola-harness gate` reads the JUnit, run-summary, coverage and hyperfine outputs and exits non-zero on any §10 breach.
- (.andromeda/test-plan.md:841) A perf breach from the new checks must be fixed in the product or escalated, never rerun away.
  > **Verdict:** `viola-harness gate` evaluates the exported JSON once; the `jq -e` expressions below are equivalent. There is no rerun, no outlier trimming and no second attempt. A breach is fixed in the product or escalated through a Decisions Log entry, never retried away.
- (.andromeda/test-plan.md:842) The hook's connect, with the directory and euid checks added to its client, must stay bounded by this gate and the deadline below it.
  > **Spine deadline:** the gate reads the product constant `viola_core::SPINE_DEADLINE` (1.0 s), imported by `viola-harness` (`gate.rs` `perf()`), never an edited literal in `ci.yml` or this table. The hook's own connect deadline is the crate-private, provisional `CONNECT_DEADLINE` = 750 ms in the root bin's `src/cmd/hook.rs`, below the gate and asserted below it by `connect_deadline_is_below_the_spine_deadline`.
- (.andromeda/test-plan.md:843) A hook fail-open case this chunk adds for a refused socket directory or a peer euid mismatch must hold the same bound at unit speed.
  > The fail-open matrix's `Instant` bound (`hook_fails_open_silently_within_the_spine_bound`, each case `< 1.0 s`) still asserts the spine bound at unit speed as well.
- (.andromeda/test-plan.md:847) These rows gate the hook path the chunk's checks join; each must hold on `max`, per OS.
  > | `viola hook session-end` (synthetic payload via `--input`, live wrapper), per OS | mean, median, stddev | `max < 1.0 s` (`jq -e '.results[0].max < 1.0'`) |
  > | Spine hooks `session-start`, `user-prompt-submit`, `stop` (same method, `VIOLA_NAME`/`VIOLA_DIR` exported to hyperfine) | mean, median, stddev | `max <` `viola_core::SPINE_DEADLINE` (1.0 s) |
  > | `viola hook pre-tool-use` on an unverified CLI (`hook.dialog` → `null` at once), timed by `run --perf` and required by `perf::ROWS` | mean, median, stddev | `max <` `viola_core::SPINE_DEADLINE` (1.0 s) |
- (.andromeda/test-plan.md:859) A new negative that flakes, the kill and leftover-file cases included, must keep the chunk red until its root cause is fixed in the chunk, with no retry and no `#[ignore]`.
  > **Zero-flakiness budget:** flaky tests are NOT tolerated. The nextest `retries = 0`. If a test flakes once, it gets quarantined immediately and fixed at the root cause, never with a retry-once budget. For viola, quarantine means the chunk stays red until the root cause is fixed within that chunk. `#[ignore]` is not allowed as a parking place. Agent-driven dev cannot distinguish a flake from a real bug; retry policies mask actual failures.
- (.andromeda/test-plan.md:862) The chunk must clear every one of these build failure conditions.
  > - Test failure (any level, any OS)
  > - Coverage below threshold for tier (per OS)
  > - Performance budget regression (hyperfine `max` gate)
  > - At a `run --mutants` (the epoch-boundary audit, or on demand): any missed mutant the harness does not leave out as host-excluded (§3 `run` step 4, Verdict), any timed-out mutant, or unviable mutants outnumbering caught ones (`unviable-exceeds-caught`)
  > - Lint / clippy / cargo-deny / zizmor / cargo-modules orphans failure, or a release build carrying a test-only binary
- (.andromeda/test-plan.md:929) Code the chunk cannot reach on a host, the macOS carve-out included, must not be passed through a wider coverage exclusion.
  > NEVER widen the coverage `--ignore-filename-regex` (`viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\]`) to pass a gate. The separator class that makes it match Windows paths is not a widening, because the four trees are unchanged. It is a back door to lowering the §10 thresholds and needs a Decisions Log entry first.
- (.andromeda/test-plan.md:938) Neither threshold nor gate may move for this chunk's sake.
  > NEVER lower coverage threshold to pass build (raise quality, not bar)
  > - NEVER skip quality gates "just this once"
- (.andromeda/test-plan.md:881) The chunk's macOS legs (the loosened-directory exit cause, the server check's carve-out) must be read on the macOS runner; the Linux dev run proves none of them.
  > NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan.md:610) Each socket-directory and peer euid refusal this chunk codes must take its own negative row in the matrix, asserting exit, document and a hint of its own.
  > **Exit-cause matrix** (`tests/cross_exit_causes.rs`, a root `cross_` binary): one negative test per cause of exit 21 and of exit 1 it holds, each asserting the exit code, the `--json` document and a last `hint:` line that names its cause — a hint shared across causes fails the table (`no_two_codes_share_a_hint`, its control `shared_hints_reads_a_planted_duplicate`);
- (.andromeda/test-plan.md:610) This entry owes the macOS exit-cause case by name, and must land it or carry the owing on.
  > beside the table, the server-verification cause is a case of `tests/security_negatives_server_identity.rs` on Linux and Windows (its macOS case owed to the route entry "Unix endpoint and home hardening")
- (.andromeda/test-plan.md:610) The hints of this chunk's new codes are held to the same ban.
  > driver-facing hints never contain `release` (`no_hint_names_release`).
- (.andromeda/test-plan.md:612) The socket directory refusal must stand as a security negative in the `security_negatives_*.rs` suite.
  > **Security control negatives** (`security_negatives_*.rs`): one negative test per security-plan control that had none — the Unix socket directory refusal,
- (.andromeda/test-plan.md:612) Making the board's home check one rule must leave these board cases holding.
  > `viola list`'s whole-board refusal exit 21 opening no log and `viola plugin install`'s exit 21 creating nothing (`tests/cross_exit_causes.rs` `strict_modes_on_the_board_is_exit_21_and_opens_no_log`, `strict_modes_on_plugin_install_is_exit_21_and_creates_nothing`)
- (.andromeda/test-plan.md:186) The socket directory and file modes must be asserted as this vector's required negatives.
  > Unix socket dir 0700 and file 0600 asserted
- (.andromeda/test-plan.md:254) The chunk's OS-branch tests must cover the macOS server verification named here, as a euid and directory check and never a pid comparison.
  > OS-branch tests cover named pipe vs Unix socket, ConPTY vs openpty, Windows DACL / strict-modes vs Unix modes, and macOS euid + dir-only server verification.
- (.andromeda/test-plan.md:775) The macOS leg of this chunk runs on this matrix row and must be read there.
  > `macos-latest` (macOS 26 arm64: openpty, Unix socket, euid + dir-only server verification; pin `macos-26` if the label moves)
- (.andromeda/test-plan.md:46) A peer of another euid cannot be driven on hosted runners, so the chunk's cross-user case must be asserted at the check boundary, with injected euids.
  > only the cross-user part (a peer of another euid) cannot be driven on hosted runners and is asserted at the peer-credential check boundary.
- (.andromeda/test-plan.md:444) The exit mapping of a home strict-modes refusal per surface must hold through the board's one rule, and frames the reading of each new refusal's exit.
  > strict-modes refusals give exit 1 for `run` / `mcp` / `viola revive` / `viola verify`, hook 0 with no output, `instance-unreachable` from the MCP frame tools, and CLI 21 for every other verb (the six frame verbs printing the instance pair);
- (.andromeda/test-plan.md:289) A macOS test the chunk compiles out must stay inside this named exception's form, and the unit tier otherwise runs on all three OSes.
  > on all three OSes except the two real-cargo-mutants harness tests and the server check's captured-span cases, compiled out on macOS.

## Bindings
- **Timed hook path ↔ the perf gate in the body**: the checks added to the hook's client sit in the timed path of the detail and under the body's `SPINE_DEADLINE` gate and `CONNECT_DEADLINE`, (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:26) with (.andromeda/test-plan.md:842).
- **Perf session ↔ obs-plan §10 obs-overhead row**: each timed hook taking the real channel path binds the detail to that obs row, (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:25).
- **Perf session's `cleanup` ↔ the harness 5-command contract**: `cleanup`'s `ok:true` in the detail binds with the body's idempotent `cleanup` requirement, where the force-killed wrapper hypothesis is measured, (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:25) with (.andromeda/test-plan.md:118).
- **Kill-shaped profile cause ↔ the fake agent contract (§7)**: the process-group rule the coverage detail leans on lives in the fake agent's exit bullet, (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:9) with (.andromeda/test-plan.md:678).
- **Host-unreachable mutants ↔ the body's mutation gate**: the record by coordinate in the detail is the body's owed mutant, (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:16) with (.andromeda/test-plan.md:829).
- **macOS owed case ↔ the CI matrix (§9)**: the owed exit-cause case proves only on the macOS row, and the per-OS coverage gate reads it there, (.andromeda/test-plan.md:610) with (.andromeda/test-plan.md:775) and (.andromeda/test-plan.md:821).
