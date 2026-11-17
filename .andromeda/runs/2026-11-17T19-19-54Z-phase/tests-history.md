@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (verify's lines pinned by literal asserts) — `tests/cli_verify.rs` pins `viola verify`'s `[NN/MM]` step lines, the `stamped` summary and the `unable:`/`hint:` pairs by literal asserts; the 23rd row moves every `[NN/MM]` literal and summary in that file (contract/verify-output/step-line, summary-line).

@history_item n=2 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (stamped root seam; `run --local-live` specified) — root `stamped_home` runs verify against the fake agent and is stamped only on exit 0 with a `… 0 fail` last line; `--local-live` checks each literal relied row id names one `pass` line, else `row-missing` — the `LEDGER_ROWS` literal must gain the 23rd id or a live home reads `row-missing`.

@history_item n=3 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (ten rows, verify's interactive runs, `--trusted-root`/`--screens`) — introduced verify's interactive runs and the fake agent's argv-only `--trusted-root` and `--screens` (no env); boot step 4 and supervise pass `--trusted-root`, `boot_untrusted` passes none; `contract_ledger_probes` stamped vs drift-only literal lists — the origin of the argv-only trust decision this chunk changes to also read the config record.

@history_item n=4 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** (fourteen rows; the verify-driven kill lines) — a verify a test drives has no test-side bound; the twelve verify-driven binaries take the 20 s `ci` kill and `verify_window_` its own class kill, raised because added runs make a designed floor — a sixth run lengthens that floor again.

@history_item n=5 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** (seventeen rows) — precedent for a row count move: `[NN/17]`, `LEDGER_ROWS` and the contract sweep; the 20 s kill held on a measured reading (longest verify-driven test 9.98 s); a stale "seven settles" comment was left in `.config/nextest.toml` and `tests/support/verify.rs` — the step-count sweep must cover comments too.

@history_item n=6 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** (kills follow the gate) — standing rule: a kill line moves only for a floor that is by design, with a planted-hang control recorded both ways; applies if the sixth run pushes the `verify_window_` no-screen case toward its kill.

@history_item n=7 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (the `mutants` override order and the `verify_window_` class) — the `verify_window_` override stands first under `[profile.mutants]`, pinned by a test; the class's floor is four gate waits — a verify run added grows every `verify_window_` case.

@history_item n=8 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** (verify's stop table) — `verify_stopped_by_a_signal_removes_only_its_own_dirs_and_exits_1` asserts a signalled verify removes only its own run dirs, foreign dirs standing byte for byte, exit 1, no `stamps.json`, log ending `interrupted` (contract/verify-stop/signal) — the sixth run's own trust record must be removed on the same exit paths, never another's.

@history_item n=9 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** (eighteen rows) — precedent for stale stamps: the tmpfs live home stamped at seventeen reads `row-missing` until its next live re-stamp; boot stamps every row; `cli_verify` and `contract_ledger_probes` literals move with the count (rule `LedgerRow::ALL`).

@history_item n=10 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** (the relied set) — `[NN/MM]` with MM = `LedgerRow::relied()`'s length; `STEPS_PASS`/`ROW_IDS`/`LEDGER_ROWS` literals; Run B carries a probe `--settings` (the founder's own answer) — the shape a new relied row and its probe take in the suites.

@history_item n=11 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** (Run E, the fifth interactive run) — the direct precedent for adding an interactive verify run: MM and summaries moved, spawn pairs counted, the stop case plants `.viola-verify-0-resume` and gains `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1`, boot's stamp runs the new run, the no-screen `verify_window_` case went 34.42 s → 42.98 s with no kill moved.

@history_item n=12 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (agents files beside the exe; agents-join row) — the founder ruled files beside the exe over a new env seam for the fake agent's inputs; the agents-join row moved summaries, the older-stamp oracle and the spawn-pair count (eighth pair) — precedent for both the fake agent reading a file and a row's suite sweep.

@history_item n=13 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (twenty-four rows) — another row-count sweep: `cli_verify`, `contract_ledger_probes`, `LEDGER_ROWS`/`LIVE_PASS`, the older-stamp oracle at the older ids, the no-framing and tag-turn summaries.

@history_item n=14 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the 2.1.302 set stamped) — `contract_ledger_probes` stamped sets are literal (2.1.287 and 2.1.302), drift-only sets apart; a new relied row must stamp on every stamped set the fake agent replays (decision/ledger-probe-contract-suite).

@history_item n=15 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the `verify_window_` kill at 60 s) — the no-screen `verify_window_` case reads ~43 s against a 60 s kill under `ci` and `mutants`, each kill moved with its planted-hang pair, never by raising the default — the headroom a sixth run's settle consumes.

@history_item n=16 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (verify's home-tree check before any log) — `verify` refuses a widened home, `ledger/`, `bin/`, `plugin/` before any step, `stamps.json` byte for byte (decision/widened-home-tree-refused-at-start, contract/strict-modes-refusal/verify).

@history_item n=17 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (stamps and pin checks at the unit tier) — `update_stamps_refuses_a_widened_ledger_and_leaves_it`: checked before the create and again under the lock (decision/stamps-refuse-widened-ledger); the guards a widened stamp write keeps.

@history_item n=18 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (remove-the-guard witnesses bounded by `WITHIN`) — a control whose neutralised guard lets a product process start needs a bounded witness (an unbounded one hung 27 min under nextest's default profile) — binds the probe's remove-the-guard control (decision/bounded-remove-the-guard-witness).

@history_item n=19 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader enumeration) — `tests/contract_bounded_readers.rs`: every product reader of external input is bounded in-statement or named in `BOUNDARIES` with a witness; the census fails an unbounded read — the writer's read of `.claude.json` must be capped and named (decision/bounded-readers).

@history_item n=20 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded compare of `replace_private_shared`) — a compare reads at most one byte past the compared bytes (decision/bounded-compare); the read-back of the written record falls under it.

@history_item n=21 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (no-clobber placement and its forced-window witness) — `fs::place_private_new` and a forced-window witness through a test parameter (no env, feature or seam), red with the guard removed — the precedent for testing the writer's lock/atomic-replace race.

@history_item n=22 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error-body enumeration) — `tests/contract_error_bodies.rs` census: every product error writer named, `thiserror_messages_interpolate_nothing` — a new writer error type or message must join the enumeration and interpolate no value (decision/error-writers-each-named, thiserror-messages-interpolate-nothing).

@history_item n=23 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola verify`'s refusal codes) — verify writes its refusal code on `process-exit` from a closed `detail` enum in `schemas/diag-line.v1.json`; a failing row is no refusal and carries no code; verify takes no `--json` — the probe's closed failure words stay on the row line, not a refusal (decision/verify-failing-row-no-refusal).

@history_item n=24 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the census of OS-gated tests) — `tests/contract_os_gated_tests.rs` `GATES`: every OS-gated test needs a row with its reason; an unnamed or stale gate fails (decision/os-gated-tests-registered).

@history_item n=25 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS-gate census reads another OS as a fault) — a gate naming `windows` or `macos` is a fault even when a row names it (decision/linux-only); no Windows/macOS arm for the record path.

@history_item n=26 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set on every build) — `LedgerRow::relied()` is every row on every build, `STEPS_PASS`/`ROW_IDS`/`LEDGER_ROWS` one literal size, the older-stamp oracle reading older sets unverified — the one count the 23rd row moves.

@history_item n=27 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the R8 strip by exact name) — the strip removes `CLAUDE*` names by exact match and keeps only `config.json` `claude_env_keep` names, the eleven-name floor always stripped — the S0 run's `CLAUDE_CONFIG_DIR` reaches the child only through `claude_env_keep` (decision/r8-claude-env-strip, claude-env-strip).

@history_item n=28 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the statusline-rate-limits arm's live red) — a live verify read a relied row red on account state, not CLI behaviour, and the row was later retired — precedent that a live probe must measure the CLI's own behaviour and its failure reads `fail`.

@history_item n=29 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the state-file ban scoped) — a reader's own unit cases may plant a malformed state file only by writing through the writer and then altering its bytes (decision/state-files-never-hand-written) — the shape the writer's malformed/oversize/link cases may take.

@history_item n=30 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** (OS-gated cases stay in a split binary's root file) — the `cli_verify` split kept its `cfg(unix)` cases in the root file; binds any new `cli_verify` case placement.

@history_item n=31 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (gate-max-wait) — the readiness gate reads the input box up from its rule and prompt rows; a quiet screen without it waits 8 500 ms (contract/readiness-gate/gate-max-wait) — the sixth run's "first settled frame" reading and its timing.

@history_item n=32 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (fake agent reading files beside its exe) — `decision/fake-agent-argv-only` gained an except and `contract/fake-agent-cli/argv-only-options` a clause for the hyprctl personality reading its mode from a file beside its exe — precedent for the named exception the fake agent's config-record read needs, since it reads a file outside argv.

@history_item n=33 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (fake agent on a test-scoped PATH) — `decision/fake-agent-on-test-path` and `decision/fake-agent-as-claude` reworded when the fake agent gained a new input source; the same rows bind how it now finds the config file.

@history_item n=34 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (the version gate reads relied rows only) — `decision/version-gate-needs-all-relied-rows`: verified only when the stamp holds every relied row as pass, a retired row changing nothing — why every 22-row stamp reads unverified once the 23rd is relied.

@history_item n=35 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (`--settings` kept, no settings layer) — the fake agent keeps `--settings`, which verify's Runs C and D pass, and a start adds no settings layer (contract/fake-agent-cli/settings, viola-run-start/settings) — the existing config-handling surface the sixth run's record sits beside.

@history_item n=36 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (folder-trust keys at a start) — the live start measured trust keys passing through the wrapper as a `wheel{human, human-input}` record before `session-start` — the trust dialog behaviour the record is meant to skip.

@history_item n=37 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (wait bounds and the record-home rule) — `WITHIN` = 12 s, mutants default kill 15 s, under cargo-mutants' 20 s floor (decision/root-wait-bound, within-7s-wait-bound, mutants-kill-below-cargo-mutants-timeout); a home given to `verify --record` is never named `home` (decision/record-home-names-no-home).

@history_item n=38 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** (one verify stamp per build) — stamped homes copy one verify stamp per build under a lock, keyed by binary hashes (decision/one-verify-stamp-per-build, fresh-home-per-test), which keeps each stamped-home test under the 20 s kill (decision/verify-driven-tests-20s-kill) — how the tests' shared stamp stays green once the 23rd row is relied.

@history_item n=39 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (stamping verifies give the fake agent a transcript) — the 22nd row required every stamping verify (harness boot and the stamped-home helpers) to pass the fake agent `--transcript`, else the row fails (decision/stamping-verify-passes-transcript) — the direct precedent for a new row's probe needing every stamping path updated.

@history_item n=40 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (`--transcript` option and its set field) — a new fake-agent option added a set field to the replayed payload and its own contract row (contract/fake-agent-cli/transcript, fake-agent-hooks/stdin-payload) — the pattern for a new fake-agent input.

@history_item n=41 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (one standing committed live rig) — `decision/live-readings-not-a-test-layer` retired for `decision/standing-live-rig`: live starts go through `viola-live-rig` under the founder's cap, each ledgered with its clock time, rehearsals in CI, live arm refused under CI; `decision/real-claude-local-only` names the rig — S0 is one such start.

@history_item n=42 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (the fake agent's trust dialog) — added `contract/fake-agent-screens/trust-dialog`, SessionStart held behind the trust row, exit 1 on No — the fake trust form decided by argv `--trusted-root`, which this chunk extends to read the config record.

@history_item n=43 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (rig exceptions to the discipline rows) — `no-blanket-approval-tests`, `discipline-at-every-layer`, `never-parse-rendered-child-screen`, `no-child-screen-assertions` and `no-screen-parsing` each gained an except for the rig's compiled-literal screen reads and its granted trust key — the scope's "no keys into any dialog by the rig" stays inside them.

@history_item n=44 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** (tied children) — every long-lived viola child of a root test is tied to its thread by a parent-death kill, and an in-test harness session ties its stamping verify (decision/supervisor-ordinary-child, wrapper-drop-guard) — a SIGKILLed verify's leftover record is the case the plan must state.

@history_item n=45 marker=2026-11-17-supply-chain-green-and-booted-tests-end
**2026-11-17-supply-chain-green-and-booted-tests-end** (stamping verify by SIGTERM; tied-verify kill witness) — the in-test stamping verify takes SIGTERM at its thread's end (supervisor SIGKILL), and its kill witness must assert an owned run dir at the kill and the stamp's absence (decision/tied-verify-kill-witness) — the stop path on which verify must remove its own trust record.
