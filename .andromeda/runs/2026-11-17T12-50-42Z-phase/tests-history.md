@history_item n=1 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (§1 Test Scope Summary) — `~ decision/no-advisory-ignored` gained its one `except`: an advisory sitting only in a banned or disabled rmcp feature is ignored with a reason naming that feature, until rmcp leaves <3.5; ratified by the founder's own answer, the card confirm PROVISIONAL. This is the exact cell part A widens: the new chrono ignore (RUSTSEC-2026-0543, an `unmaintained` advisory brought in by upstream crates) is a second ground beside the rmcp-feature ground, and the row edit is the WRAP's, not this chunk's.

@history_item n=2 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (§9 CI Integration) — `~ decision/no-ignored-advisories` gained the same rmcp-feature `except` (guard: CI job supply-chain). The twin row of n=1 in §9: both cells name a single ground today, so the widened unmaintained ground would make both read false unless the wrap amends both together.

@history_item n=3 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the supply-chain advisories with no ignore) — the supply-chain trigger moved to «no advisory ignored, `[advisories] ignore = []`», RUSTSEC-2026-0441 closed by moving `portable-pty` to `=0.9.0` (no `pin-utils` in the graph), and the RUSTSEC-2017-0008 ignore left with the `serial` crate. Precedent: an advisory closed by a dependency move rather than an ignore. It also records the narrowing this chunk's widening runs against.

@history_item n=4 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the recorded supply-chain red, RUSTSEC-2026-0441) — a red `supply-chain` on an `unmaintained` advisory (`pin-utils`, transitive) was recorded with a CARRY owner and «the ignore list is not widened». Direct precedent for an unmaintained transitive advisory: then the remedy was an upgrade, and now no safe upgrade exists and the founder has widened the rule. Plan P3 should state why this case differs.

@history_item n=5 marker=2026-10-26-tokio-rustsec-2026-0386-lock-bump
**2026-10-26-tokio-rustsec-2026-0386-lock-bump** — an advisory (RUSTSEC-2026-0386) was closed by an exact-pin lock bump of tokio, and the test-plan's pin text was moved at three sites. Precedent for the «same id once a safe upgrade exists» re-red condition: the expected remedy then is the upgrade, not a standing ignore.

@history_item n=6 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — created the Supply-chain stage: the ubuntu job `supply-chain` running cargo deny, the sole-root `deny-sync.toml` tokio ban, `scripts/deny-probes.sh`, zizmor and the JSON artifact, plus the weekly `nightly.yml` advisories job. These are the configs and jobs P3 must re-measure for RUSTSEC-2026-0543 (rows `decision/supply-chain-ci-gates`, `decision/supply-chain-fail-closed`, `decision/sync-crates-without-tokio`, `decision/tokio-banned-per-sync-crate`).

@history_item n=7 marker=2026-09-24-workspace-tree-and-code-graph-planes
**2026-09-24-workspace-tree-and-code-graph-planes** — added the `fuzz/Cargo.lock` advisories + sources step (into `deny-fuzz.json`), made the weekly advisories cover both lockfiles, and made `lint`, `supply-chain` and `release` plain exit-code jobs with no gate step. The fuzz lockfile audit and the nightly run are separate advisory readers. Each may read the chrono id independently of `deny.toml` (rows `decision/cargo-deny-gate`, `decision/weekly-advisories-run`, `decision/weekly-advisory-scan`, `decision/ci-gate-last-step`).

@history_item n=8 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the serde_json `unbounded_depth` ban and deny-probes 14/14) — added a `[[bans.features]]` ban to `deny.toml`, so `deny-probes.sh` ends `14/14 banned, control clean`. The count's rule is the `probes=(` rows plus the planted channel probe. This is the current `deny.toml` shape the ignore edit sits beside: the ignore list and its comment must not disturb the bans or the probe count (rows `decision/cargo-deny-tokio-and-c-crate-bans`, `decision/cargo-deny-supply-chain-gate`).

@history_item n=9 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the CI gates: the channel tree form, rmcp features, deny-probes 13/13) — added `scripts/rmcp-features.sh` (exactly server + transport-io) and `channel-tokio-free.sh`, and dropped `viola-channel` from the sole-root list. It sets up rows `decision/rmcp-feature-limit` and `decision/rmcp-features-server-transport-io`, and with them the «rmcp feature banned or off» premise the existing ignore ground rests on. chrono reaches the graph through `rmcp 3.4.1`/`schemars` via `viola-mcp`.

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the CI matrix [ubuntu-latest] and two workflows) — the workflows are `ci.yml` and `nightly.yml` only, on the one `ubuntu-latest` leg. This bounds where advisories are read: the `supply-chain` job and the nightly advisories job. Acceptance is the pushed sha's run green on that one leg (rows `decision/tests-run-on-linux-ubuntu-latest`, `decision/agent-reads-ci-through-gh`).

@history_item n=11 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** (§5: 3 rows) — `~ decision/wrapper-drop-guard` now ties every long-lived root-test viola child to its spawning thread by a parent-death kill. `+ decision/no-untied-viola-spawn` adds the census over every test source, proven on a planted positive. `+ decision/tied-child-spawn-thread` requires that a tied child is spawned only on a thread that outlives it. Part B extends exactly this tie and census to viola-e2e's `harness_lifecycle.rs` and `cli.rs` boots. Trap from the census row's why: a census over one spelling of a constant missed a spawn site.

@history_item n=12 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** (§3: 1 row) — `~ decision/supervisor-ordinary-child`: the agent-run CLI boot leaves the supervisor untied, while an in-test harness session ties it and its stamping `viola verify` to the test thread. Part B's `tied: false` boots in `harness_lifecycle.rs`/`cli.rs` are the in-test sessions this row now says must be tied. The tied stamping verify built here has no kill witness yet, which this chunk owes.

@history_item n=13 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** (interim supervisor, readiness, status and cleanup) — the supervisor became an ordinary child (detach flags removed). The cleanup report gained `processes_gone`, its assertion «no field false», and the cleanup document's shape. This is the origin of `decision/supervisor-ordinary-child` and of `contract/cleanup-document/processes-gone` / `contract/cleanup-exit-codes/1`, which a killed-test proof should read.

@history_item n=14 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** (boot's two flags, harness_session, status over viola list) — `harness_session(test, label, opts)` became a plain function and drop guard that keeps the spawned `supervise`, so `cleanup`'s pid + start-time proof stays exact. The `Drop` cleans up and keeps the home under `AGENT_RUN_KEEP_HOMES` / `AGENT_RUN_KEEP_FAILED`. This is the `BootOptions`/session path part B ties (rows `contract/harness-session/*`, `decision/e2e-boot-model-by-cleanup`, `decision/cleanup-removes-home-record-and-supervisor`).

@history_item n=15 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified `harness_session` keeping the spawned `supervise`, along with the E2E regex and Path 1 at the root tier, so their PROVISIONAL marks are gone. The spawned-supervisor model part B ties is standing, not provisional. A tie must keep the pid + start-time cleanup proof intact.

@history_item n=16 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (harness keys and the time mechanism) — `harness_session` takes `stamp: !opts.unstamped`, and `SessionOptions::unstamped` is the in-test `boot --unstamped`. This explains the scope's gap: the last chunk's harness victim boots unstamped, so no stamping `viola verify` ran and its tie went unwitnessed. The new kill test needs a stamped session, or a direct tied verify spawn.

@history_item n=17 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (Path 7's MCP leg on the default pair, unstamped) — `mcp_tools_path7_unverified_cli_answer` boots the default pair through `SessionOptions { unstamped: true, .. }`. This is another in-test `harness_session` boot a site sweep over viola-e2e's `BootOptions`/`SessionOptions` spellings must find, so it is a lead for the «every spelling» sweep.

@history_item n=18 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — viola-e2e's booted lifecycle tests got a `Booted` drop guard that keeps a failing test's home under `AGENT_RUN_KEEP_FAILED=1`. viola-e2e (no dev-dependencies) uses a labelled case table instead of rstest `#[case]`. That guard sits in `harness_lifecycle.rs`, the file part B edits, and constrains how its new cases are written.

@history_item n=19 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`cleanup`'s `endpoint_gone` on Unix) — after a force-killed wrapper the leftover socket file stays, cleanup removes nothing of it, and `endpoint_gone` reads «connect refused or path absent». Witness `cleanup_after_a_force_killed_wrapper_reads_the_endpoint_gone` is in `harness_lifecycle.rs`. A killed-test proof must use this reading, never the file's absence (rows `contract/harness-cleanup/endpoint-gone`, `contract/cleanup-document/endpoint-gone`, `decision/killed-wrapper-endpoint-wait`).

@history_item n=20 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the kill path's endpoint wait) — `Wrapper::kill` waits for the exit, then for the recorded child to be gone by pid and start time, then on `holder_gone`. The plan's `wait_endpoint_gone` after a kill was disproved because a killed wrapper leaves its socket file. Trap behind `decision/killed-wrapper-endpoint-wait` for any «nothing is left» check after a kill.

@history_item n=21 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (stamped root seam, stop waits for the endpoint) — the root `stamped_home` runs verify and is stamped only on exit 0 and a last line `stamped <ver>  <n> pass  0 fail`. `Wrapper::stop` waits for the endpoint to be unconnectable, because «an exit code is not the endpoint gone». Rows `contract/viola-verify/stamped-line`, `decision/verify-sole-stamp-writer` and the stop-vs-kill distinction the killed-verify test must respect.

@history_item n=22 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — `verify_stopped_by_a_signal_removes_only_its_own_dirs_and_exits_1` signals a verify it started (`verify_started`, `libc::kill` at its own pid once the receipt shows progress). It asserts only its own run dirs removed, the exact plain pair, exit 1, no `stamps.json` and `process-exit{detail:"interrupted"}`. This is the closest precedent for killing the tied stamping verify (rows `contract/verify-stop/signal`, `contract/verify-exit-detail/interrupted`). A SIGKILL from the parent-death tie is not this SIGINT/SIGTERM path, so the oracle must say which one it reads.

@history_item n=23 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — added `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1`, its killed child asserted on `duration_ms` only. This is a second precedent for asserting a verify killed mid-run, and for how a killed child's fields are read.

@history_item n=24 marker=2026-10-07-a-send-ending-in-a-newline-is-confirmed
**2026-10-07-a-send-ending-in-a-newline-is-confirmed** — measured that an instrumented child killed during its exit-time profile write leaves a short profile that llvm-profdata refuses (3 of 4 800). The fix was an uninstrumented `whoami` child, plus the `scripts/profraw-census.sh` witness. This is the trap behind `decision/killed-test-child-uninstrumented`: a new test that kills the instrumented `viola verify` / fake agent risks a coverage-merge red.

@history_item n=25 marker=2026-09-29-h2-conpty-resize-probe
**2026-09-29-h2-conpty-resize-probe** (coverage: the throwaway crate writes no profile) — named the one deliberate exception to child-process coverage: the harness self-tests' nested cargo removes cargo-llvm-cov's four variables through `test_support::uninstrumented`. This is the `except` of rows `decision/child-process-coverage` / `decision/coverage-includes-child-processes`, and the mechanism a killed-child test could reuse.

@history_item n=26 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the headroom floor, the hook hangup named) — every command the fake agent spawns runs in its own process group, so a hangup when a killed wrapper closes the PTY no longer cuts a hook's profile. This is the origin of `decision/fake-agent-children-own-process-group`, and it bears on what a SIGKILLed test's fake agent and hooks leave behind.

@history_item n=27 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the fake agent's exit ignores SIGHUP) — the fake agent's exit ignores SIGHUP from its first step, while a hangup landing before the exit still ends it. This came from a cut-short fake-agent profile when a killed wrapper hung up its PTY (row `decision/fake-agent-exit-ignores-sighup`). A tied kill of a wrapper sends exactly such a hangup.

@history_item n=28 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** (the fake agent quiesces its hooks before it exits) — the fake agent exits on `\x03` or stdin EOF and waits for a running hook first. This exit was measured to hang up an in-flight `viola hook` (a `.profraw` WATCH). It is the origin of `contract/fake-agent-exit/exit-triggers`, which says what «nothing of it is left» means for the fake agent.

@history_item n=29 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (remove-the-guard witnesses bounded by `WITHIN`) — a control whose neutralised guard lets a product process start needs a bounded witness, because nextest's default profile has no kill line (an unbounded witness hung 27 minutes). This is row `decision/bounded-remove-the-guard-witness`: the new tie or census controls must be bounded.

@history_item n=30 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (root-wait-bound, within-7s-wait-bound, mutants kill) — `WITHIN` moved 7 s → 12 s, the root waits are bounded at 12 s below the mutants 15 s kill and cargo-mutants' 20 s floor, and the mutants default kill moved 10 s → 15 s. These are the current bounds every new wait in part B takes, under the project's 20 s lint (rows `decision/root-wait-bound`, `decision/within-7s-wait-bound`, `decision/mutants-kill-below-cargo-mutants-timeout`).

@history_item n=31 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** (one verify stamp per build; the 20 s kill's why) — `+ decision/one-verify-stamp-per-build`: stamped homes copy one stamp per build, taken under an exclusive lock. `~ decision/verify-driven-tests-20s-kill`'s why now rests on that once-per-build stamp, and the stamp dir has its own sweep rule. A test that kills the stamping verify must not leave the shared stamp or its lock half-taken.

@history_item n=32 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (§3: stamping-verify-passes-transcript) — `+ decision/stamping-verify-passes-transcript`: every stamping verify in the harness boot and the stamped-home helpers passes the fake agent `--transcript`. A new test that drives its own stamping verify, to kill it, must keep this or its stamp lacks the row.

@history_item n=33 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the bootstrap phases without the Windows arms) — `[profile.ci]`'s 20 s kill line lists twenty-six verify-driven binaries, its rule the `binary(…)` names on the filter line, and the root waits read 31 sites in 23 files. A new stamped-home test binary joins that line and moves the count by its rule (rows `decision/ci-stamped-home-kill-line`, `decision/verify-driven-tests-20s-kill`).

@history_item n=34 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS-gate census reads another OS as a fault) — the `contract_os_gated_tests` census reads any gate naming windows/macos as a fault, using a parse that catches a multi-line `cfg_attr` a line grep missed. This is row `decision/linux-only`'s guard. Precedent for part B's census: sweep every spelling, not a line grep, and the tie is Linux-only (`prctl` parent-death).

@history_item n=35 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader enumeration in the Contract suite) — `contract_bounded_readers.rs` built a tree-walking census (the `andromeda:walks-tree` token) with literal tables, a planted self-check and a red shown at an exact line. This is the house pattern part B's site sweep and census proof should follow.

@history_item n=36 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the root `cross_` binary and the shared start helpers; a wrapper lost under an open call) — `tests/cross_exit_causes.rs` is a root `cross_` binary in the E2E layer. `tests/support/start.rs` holds `kill_wrapper` and the shared start setups, and a wrapper lost is a real kill (`kill_wrapper`, `kill_by_pid`), never a double. These are the shared kill helpers and placement rules for a new kill test (rows `decision/real-process-faults`, `decision/cli-tests-as-subprocess`).

@history_item n=37 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (a stale holder reads `stale-heartbeat`) — a stale holder is a real wrapper under the `Frozen` guard (`SIGSTOP`), with no double. This is the other half of `decision/real-process-faults`. Any kill/freeze of a test's own process goes through these guards, and no test signals a process it does not own.

@history_item n=38 marker=2026-10-04-windows-boundary-mutation-workflow
**2026-10-04-windows-boundary-mutation-workflow** (the mutants profile waits for running tests) — `[profile.mutants]` moved to `terminate = "wait"` so the running tests' temp dirs and session guards drop. It measured 170 vs 0 leftover temp dirs, and 17 kill-path leftovers by design. This is a prior measurement of what a killed test leaves behind, the class part B closes for the supervisor and verify.

@history_item n=39 marker=2026-10-02-epoch-2b-cleanup
**2026-10-02-epoch-2b-cleanup** (test-data cleanup: owner record last) — a root `TestHome` removes its home through `remove_owned`, `owner.json` last, so a part-way removal stays reclaimable by the gone-owner sweep. This explains what a SIGKILLed test's home looks like and what sweeps it (rows `decision/session-home-retention`, `decision/cleanup-between-integration-tests`).

@history_item n=40 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — on the dev host `target/e2e-home` is a link to tmpfs, and harness cleanup, a `TestHome`'s drop and the owner sweep delete through it, but only dirs the harness or test created. This is the ratified widening behind `decision/e2e-home-deletion-through-link`. Any cleanup a killed-test proof checks stays within it.

@history_item n=41 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a test's endpoint directory is made private; the socket directory's fixtures) — a test placing an endpoint makes its directory through `create_private_dir`, never a bare `tempfile::tempdir()` (0755, refused by the socket-dir check). The test socket root is `SOCKET_ROOT_VAR` under `/tmp`. These back the scope's «a test socket goes under the short `/tmp` root» (rows `decision/private-dir-for-test-endpoints`, `decision/test-endpoint-dirs-made-private`).

@history_item n=42 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket directory's security negatives) — the socket-dir refusal (`socket-dir-refused`): exit 1 at start, 21 at frame verbs, 0 at the hook, never chmods, left as found. This is the source of the picked socket-dir rows (`decision/socket-directory-refusal`, `decision/socket-dir-check-never-chmods`, `contract/start-exit-1/socket-dir-refused` and siblings). A booted test or kill proof must not trip them.

@history_item n=43 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (the test-scoped PATH) — every test-started `list`/`revive` runs under a test-scoped `PATH` (`claude_bin` / `empty_bin`), so a test never reads the host's live sessions. This is the origin of `decision/tests-never-read-host-sessions`, which a new test spawning viola must respect.

@history_item n=44 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — a test-started `viola` drops `VIOLA_NAME` / `VIOLA_DIR` / `VIOLA_BIN`, and a path oracle compares `resolved(home)`. This is the origin of rows `decision/test-started-viola-drops-env` and `decision/test-viola-drops-session-vars`, binding any new tied spawn helper.

@history_item n=45 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (fake-agent copies planted one after another, ETXTBSY) — a test planting several fake-agent copies plants them all before any thread starts a child, because a child forked on one thread inherits another's open write handle. This is a trap for part B's thread-spawned tied children.

@history_item n=46 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the MCP tests as landed) — the MCP stdio tests own their `viola mcp` child, closing stdin and awaiting its exit, never rmcp's `TokioChildProcess`, which killed the server 3 s after a close and truncated a coverage profile. This is the origin of `decision/mcp-stdio-test-owns-child` and `decision/viola-e2e-rmcp-client-only`. A viola-e2e child in `cli.rs`/`mcp_tools.rs` that the tie or the census must cover.

@history_item n=47 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (§3 harness) — the cleanup report is `{cleaned, processes_gone, endpoint_gone, home_removed, killed}` and the session record `{v, session, home, instances, supervisor_pid, supervisor_started_at}`. These are the fields a «nothing left» proof reads (rows `contract/harness-session-record/*`, `contract/cleanup-document/*`, `contract/harness-cleanup/idempotent`).

@history_item n=48 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** (run --e2e, the E2E regex widened) — the E2E regex is `^(path[0-9]*|tui|mcp|cross|chaos|contract)_` with integration as its complement. An unmatched `binary()` regex is a parse error. This decides which layer a new kill/census test binary lands in, and backs the scope's «every filter checked to select at least one test».

@history_item n=49 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** (the one-Rust-test example selects through --integration) — the plan's own gate written with `--e2e` exited 2 (it selected nothing) and was corrected to `--integration`. This is a precedent trap for the scope's rule that every control filter is checked to select at least one test.

@history_item n=50 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS scope: Linux only, one CI leg) — Linux only, with CI on `ubuntu-latest`. The rule «NEVER treat a Linux-only local run as proof for Windows or macOS» is retired. This is the source of rows `decision/linux-only` and `decision/tests-run-on-linux-ubuntu-latest`, under which the Linux parent-death tie is the whole mechanism.

@history_item n=51 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (§1: real-claude-local-only) — `~ decision/real-claude-local-only`: the real claude is reached only locally, through `viola verify` and the standing live rig's live starts, never in CI (PROVISIONAL delegate, ratification the founder's). This bounds the scope's «no live `claude` start». The kill test must stay on the fake agent (rows `decision/real-claude-never-in-ci`, `decision/ci-never-real-claude`).

@history_item n=52 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (run --local-live specified) — `--local-live` under `CI` refuses with `live-in-ci`, exit 2, before any build or spawn. This is the guard of `decision/no-real-cli-in-ci`, and it confirms no CI path in this chunk may reach the live verify.
