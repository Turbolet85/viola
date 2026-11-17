@history_item n=1 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** (fake agent's print-ban exemption) — the fake agent is a `[[bin]]` of the root `viola` package (`src/bin/viola-fake-agent.rs`), so it inherits the root `[lints]` and is exempted only by a crate-level print allow; only `viola-e2e` carries its own lint table. Any fake-agent change this chunk makes (for example, ending once its PTY or parent is gone) lives under that lint regime.

@history_item n=2 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** — `obs_event!` attaches `event`, `process` and `instance` from `ProcessCtx`, with `corr` supplied by the caller. Default-deny is a top-level `unevaluatedProperties: false` in `schemas/diag-line.v1.json`. This is the basis of the attached-fields and schema rows: any line a changed `viola run` exit path writes must validate under G4.

@history_item n=3 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — `already-live` is logged for a live or stale holder, and a `gone` holder is taken over with no exit-1 line (`run.collision_check` `free`). A wrapper left alive by a killed test keeps its name held, so a later boot on the same name meets this refusal. That is the trap behind the collision rows.

@history_item n=4 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** — D-28: the concurrent-append check landed for the hook files (8 processes); `shared-log-files-concurrent-append` rests on it. A census or kill harness that reads role files while wrappers are still writing must tolerate torn lines.

@history_item n=5 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (G2's exact-path seam exemption) — G2 runs as `scripts/g2-zero-panics.sh` (`--probe` first, in the test and perf jobs) and leaves out only a panic located exactly at `src/cmd/hook/seam.rs:<digits>`, the fake agent's forced panic. A fake-agent or fixture change must not add a second exemption.

@history_item n=6 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — `run`'s one pre-spawn terminal exception is a start refusal, written as two fixed stderr lines through `src/human.rs`. Otherwise `run` writes nothing to the terminal (`run-silent-on-terminal`). Any shutdown path this chunk adds to `viola run` must write no terminal byte.

@history_item n=7 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (verify's outcome, the `boot` readiness owner) — `verify` in CI was `tests/cli_verify.rs`. `boot` step 4 and `stamped_home` were handed to «Verify-stamped test homes and harness», which also owns `boot`'s readiness check. This is the origin of the shared stamped fixture this chunk repairs.

@history_item n=8 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — mutation testing left CI and runs at `agent-run run --mutants` at the epoch-boundary audit or on demand. That kind of run killed tests at their timeout and left five wrappers spinning, which is why this chunk exists. Its boundary says no mutation run here.

@history_item n=9 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — in CI verify runs only against the fake agent (`tests/cli_verify.rs`, harness `boot` step 4 unless `--unstamped`, `stamped_home`). The real-CLI verify runs only through `run --local-live`, refused under `CI`. `boot`'s `start_records` stage checks `events.ndjson` lines 1-3. These are the fixtures (`stamped_home`, `harness/boot.rs`) whose validity on a cache-restored `target/` is this chunk's first step. The `ci-verify-fake-agent-only` and `real-cli-verify-never-in-ci` rows rest here.

@history_item n=10 marker=2026-10-02-epoch-2b-cleanup
**2026-10-02-epoch-2b-cleanup** — neither harness `cleanup` nor a test home's drop (now `remove_owned`) deletes a home before G2, G4, the secret scan and the uploads. CI's `AGENT_RUN_KEEP_HOMES=1` keeps every home. Ending a booted wrapper is a different job from removing its home: process teardown must not delete a kept home (`test-homes-kept-until-gates`).

@history_item n=11 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — the vt100 feed thread is detached and a contained worker. The "no `process-exit`" ban covers the PTY pump and the handle-wait thread. This is the thread model to read when P3 asks why an orphaned wrapper SPUN instead of idling. The `run-threads-inside-catch-unwind` and `run-worker-panic-ends-run` rows bound any loop change.

@history_item n=12 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (the G2 question answered: the chaos home outside the scans) — every integration home lives under `target/e2e-home/` except `tests/chaos_feed_panic.rs`'s `TestHome::outside_scan()` home, which is the founder's one named carve-out. A new kill or census test home must sit under the scan root and must not become a second carve-out.

@history_item n=13 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — `target/e2e-home` is a link to an owner-only tmpfs directory, kept homes end at a reboot, and `/tmp` ages them out. G2's `find` start point needs a trailing slash to descend a linked scope. This bears directly on the CI-red hypothesis: a `target/` cache restores the link and a stamp marker while the `/tmp` home behind them is gone.

@history_item n=14 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — revive's start arm is role `run` and logs as the wrapper into `run-<name>.ndjson` (`contract/cli-log-file/revive-start`). It shares `run`'s start setup, so any exit-path change to the wrapper reaches revive's start too.

@history_item n=15 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a verify stopped by Ctrl-C or TERM ends `process-exit{detail:"interrupted"}` after closing every spawn pair it opened. The probe dir is removed on a pass, a fail and a stop, but SIGKILL and a crash leave it. This is the precedent for what a SIGKILL'd process leaves behind, here inside stamping.

@history_item n=16 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — a child of an interactive PTY run killed at a stop carries `child_exit_status` (the PTY pump's exit code) and `duration_ms`. A print-mode or `--version` probe child killed there is not measured. `verify-process-exit/child-exit-status-and-duration` rests on this; a killed fake-agent child should read the same way.

@history_item n=17 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (the reserved web vocabulary leaves) — no process logs a liveness transition: `run` touches `heartbeat` every 1 s, and staleness is over 5 s, read through `list --json` / `agent-run status`. A leftover wrapper shows only through its heartbeat and liveness reads, never through a log line. This bounds what a census may read.

@history_item n=18 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — verify logs eight child spawn pairs (including `agents-probe`) and `run_bounded` / `run_capped` stay unlogged. Stamping the shared home spawns all of these fake-agent children, and a killed test's census has to account for them.

@history_item n=19 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** — `agent-run status` reads `state: degraded` when a booted instance is `stale` or absent, deriving its state from `viola list`. This is the harness-side reading of a wrapper that is gone or lingering.

@history_item n=20 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the `diagnostics/` opens behind `check_diagnostics`) — a refused diagnostics folder or file is exactly a failed open (`refused-diagnostics-path-is-a-failed-open`). A test home rebuilt with widened modes writes no role line.

@history_item n=21 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree check of `run` and revive's start) — `checked_exe` and `check_home_tree` run before the wrapper log. A refusal exits 1 with no log, no line and no child (`home-tree-check-before-start-log`). A rebuilt stamped home must pass this tree check, or a booted wrapper refuses before it logs anything.

@history_item n=22 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`viola verify`'s home-tree check) — verify checks the home tree before its log, and `update_stamps` runs `check_stamps` before and under its lock. A refusal is exit 1 with the home pair. This is the stamp-write path the shared fixture's rebuild goes through.

@history_item n=23 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the snapshot readers over a replayed snapshot) — over a replayed snapshot `run.collision_check` decides by the heartbeat alone: a beat no older than `STALE_AFTER` is `already-live`, an older one `free`. A killed wrapper's fresh heartbeat therefore blocks a reboot of its name (`contract/run-span-attributes/run-collision-check`).

@history_item n=24 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (E5: the replay's product readers, `state.snapshot_recover` placed) — `run`'s start makes one snapshot read, which writes one `state-recovered` line on an unreadable snapshot, before `run.collision_check`.

@history_item n=25 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the refusal-telemetry record) — a strict-modes check refused before any log opens writes no line and has no logged code. This is the `except` of `distinct-detail-code-per-exit-cause`.

@history_item n=26 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (v1-47's secret-scan reading) — the scan runs after each job's end-to-end run with `AGENT_RUN_KEEP_HOMES=1`, `if: always()`, and the `viola-chaos-*` home left out. The `secret-scan-scope` and `homes-kept-until-gates-run` rows rest on this.

@history_item n=27 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (a stale holder's code is `stale-heartbeat`) — a `stale` holder ends exit 1 `stale-heartbeat`, and `already-live` covers a live holder or a fresh replayed beat (`run-exit-1-detail/*`, `run-start-failure-detail/*`). These are the refusals a leftover wrapper causes on a later boot of its name.

@history_item n=28 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`verify` writes its refusal's code; the `process-exit` `detail` enum's count) — verify writes `unclean-fixture`, `strict-modes-failed` and the other refusal codes on its `process-exit`, and the enum gained `unclean-fixture`. This is the fixture-side refusal code nearest to a stale or partial stamped home.

@history_item n=29 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (Founder Direction 4's codes and named carve-outs) — each exit-21 and exit-1 cause gets a distinct code. A strict-modes check refused before any log writes none. `distinct-detail-code-per-exit-cause` rests here: any new wrapper exit cause needs its own code.

@history_item n=30 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory is no strict-modes failure; Founder Direction 4's carve-out retired; the start's socket-directory step) — `socket-dir-refused` is exit 1 at `run` / revive's start before any snapshot, child or bind, and exit 21 at the frame verbs. It is no strict-modes failure. Test sockets go under the short `/tmp` root (directive), and their parent directory must pass this check.

@history_item n=31 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the OS-gated test census) — `tests/contract_os_gated_tests.rs` requires every OS-gated test function or file to be named in `GATES` with a reason. A new SIGKILL or census test gated on Linux-only APIs must be named there (`os-gated-test-census`).

@history_item n=32 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (ConPTY sideload and the Windows kill fallback retired; openpty the one PTY backend) — child exit is handle-detected over openpty. `exit_source` stays `handle-wait|kill-fallback`, with `ExitSource::KillFallback` still in `viola-pty` (the perf-job entry's Kept). The `child-exit-from-handle-wait` and `pty-seam-log/kill-fallback` rows rest here and bound how a wrapper whose test is gone may end its child.

@history_item n=33 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (one CI leg; the OS target, Linux only; the closed words that left `diag-line.v1.json`) — CI is `ubuntu-latest` alone, the census fails any gate naming `windows` or `macos`, `os` is closed to `linux`, and a closed word with no emitter leaves the schema. A Linux-only process-tree census is admissible.

@history_item n=34 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (portable-pty `=0.9.0`; process-start's fields on Linux only) — the `viola-pty` seam is over portable-pty `=0.9.0`, whose `log` target is OFF with failures captured at the seam. `pty_backend` reads `openpty` alone. Any PTY-gone detection this chunk adds is captured at the seam, never through the log bridge.

@history_item n=35 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** (8. PII Scrubbing & Compliance; 3. Observability Harness Contract) — `~ decision/secret-scan-before-upload`: its classes are now `CLAUDE*` values, the content canary, mode 0600 and an unreadable file, and the Cookie class is retired. `~ decision/content-only-in-instance-detail-file` lost its cookie. A rebuilt or killed home must still pass the 0600 and unreadable-file classes.

@history_item n=36 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — `~ decision/os-apis-at-call-sites` and `~ decision/instrument-call-sites-not-os-internals` add the hyprctl spawn. `~ contract/diag-line-fields/process-start` and `+ contract/diag-subject/hyprctl-probe` add a closed subject. These are the current forms of the call-site rows that any new spawn or kill instrumentation follows.

@history_item n=37 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `~ decision/exit-code-log-field`: 11 is retired and never reused. `~ decision/diag-schema-additive-only`, `~ decision/log-fields-additive-only`, `~ decision/log-schema-additive-only` and `~ decision/obs-aligns-to-tests-log-format` were narrowed to "a field the product still emits" on the operator's Q2. These are the current additive-only rules that any new wrapper exit field must meet.

@history_item n=38 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (6. Log Coverage) — `~ contract/diag-line-fields/process-exit` dropped `shell_exit_status`. `~ contract/child-spawn-log/subject`: no shell spawns, so spawn lines are child spawns only. Viola runs no shell, so a booted wrapper's process tree is the wrapper and its fake-agent child.

@history_item n=39 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — a reused instance name keeps the earlier starts' lines at the head of `events.ndjson`, and the start order is read from that start's own lines. This bears on boot readiness (`start_records` lines 1-3) when a test reboots a name after an earlier wrapper.

@history_item n=40 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — `~ decision/claude-internals-not-a-contract` gains the transcript stdout-tag echo exception. `~ decision/refusal-telemetry` gains the transcript echo read's no-line refusal. These are both rows in the scope summary area as they stand now.

@history_item n=41 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** — the founder ratified the 2026-11-16-local-command-delivery excepts on `decision/claude-internals-not-a-contract` and `decision/refusal-telemetry`. They now stand on his own word, unchanged.
