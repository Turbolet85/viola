@history_item n=1 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** (the mutation row leaves CI) — moved obs-code mutation from CI to `agent-run run --mutants` at the epoch-boundary audit and on demand, and made a surviving obs mutant a failure there. This is where decision/obs-code-mutation-tested and decision/surviving-obs-mutant-fails come from. It is why this chunk runs no mutation itself and only makes the next boundary's baseline reachable.

@history_item n=2 marker=2026-10-10-windows-mutation-grade
**2026-10-10-windows-mutation-grade** ("missed" reads after the host exclusion) — set what is red at a `run --mutants`: a timed-out obs mutant, a missed mutant the harness does not leave out as host-excluded, or unviable outnumbering caught. "Surviving" is the harness document's `survived`. This is the source of the except clause on decision/surviving-obs-mutant-fails. The slow-case kill (M3) and the hypr-tap survivors are read against these words.

@history_item n=3 marker=2026-09-24-epoch-1-cleanup
**2026-09-24-epoch-1-cleanup** (§9 Mutation row names the per-leg unviable rule) — added the rule that unviable mutants outnumbering caught ones is red, which decision/obs-code-mutation-tested still carries. A timeout margin that turns the slow cases into timeouts or unviables would trip it.

@history_item n=4 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** (pre-push document and mutation scratch wording) — on the Linux dev host the mutation scratch is the operator's NOCOW `TMPDIR`, which no document prints, and `mutants.out/` stays at the repository root. This is the scratch path item 1's `sun_path` bound is computed under (`$TMPDIR/.tmpXXXXXX/…`). Any new assertion must not print that path.

@history_item n=5 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (one CI leg, two workflows, one deny target and one harness shim) — reduced CI to `ci.yml` on `[ubuntu-latest]` and dropped the `windows-mutants.yml` leg and the Windows mutation scratch from the Mutation row. The boundary mutation baseline this chunk prepares is therefore the Linux `run --mutants` alone. The OS-gated census now faults a `windows` or `target_os = "macos"` gate, which matters for any gated test this chunk adds.

@history_item n=6 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS target, Linux only) — set the source of decision/linux-only-os-target: the core on any current distribution, CI on `ubuntu-latest`. Item 1's `sun_path` limit of 107 B is the Linux one, and no other OS bound needs proving.

@history_item n=7 marker=2026-10-02-epoch-2b-cleanup
**2026-10-02-epoch-2b-cleanup** (CI keeps every home: deleter renamed) — a root test home now drops through `remove_owned`, and CI's `AGENT_RUN_KEEP_HOMES=1` keeps every home until G2, G4, the scan and the uploads. This explains decision/homes-kept-until-gates-run and decision/test-homes-kept-until-gates. It bears on item 5 if the naming rule lands in `tests/support/home.rs` (`TestHome::new`).

@history_item n=8 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** (G2's start point descends a linked scope) — on the dev host `target/e2e-home` is a link to an owner-only tmpfs directory, and G2 must read `target/e2e-home/` with a trailing slash. Trap: `find` does not descend a link start point without the slash. This constrains item 5's home-naming helper, and any path-length reasoning about homes under that root.

@history_item n=9 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (the G2 question answered: the chaos home outside the scans, F4) — every integration home lives under `target/e2e-home/` except `tests/chaos_feed_panic.rs`'s `TestHome::outside_scan()` home. This is the except clause on decision/test-homes-under-e2e-home and decision/test-homes-under-scan-root. An item-5 naming rule in the home helper must keep both the scan root and this one named carve-out.

@history_item n=10 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (v1-47's secret-scan reading) — the secret scan reads after each job's end-to-end run, with `AGENT_RUN_KEEP_HOMES=1` set on the job and the `viola-chaos-*` home out. The 2026-11-01 batch later ratified this. It is the CI context in which the coverage suite this chunk greens keeps its homes, and it backs the test-homes rows.

@history_item n=11 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (G2's exact-path seam exemption, the perf rows built) — G2 (`scripts/g2-zero-panics.sh`) counts every panic line except the seam's exact path. This is why a hook panic, which exits 0, must sit under the scanned root (the why of the test-homes rows). It also states the hyperfine `max`-is-enforced budgets behind decision/hyperfine-max-is-enforced and decision/budgets-asserted-by-exit-code.

@history_item n=12 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (verify-probe joins the spawn subjects) — in CI, verify runs only against the fake agent (`tests/cli_verify.rs`, boot step 4, `stamped_home`), and the real-CLI verify runs only through the local live run, which refuses under CI. This is the source of decision/ci-verify-fake-agent-only and decision/real-cli-verify-never-in-ci. The `--record` callers (`tests/cli_verify/*`) that item 5 binds are fake-agent only, and this chunk has a live cap of 0.

@history_item n=13 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** (`detail:"interrupted"` on verify's process-exit) — a verify stopped by SIGINT or SIGTERM starts no further spawn, closes every pair it opened, ends `process-exit{detail:"interrupted"}` and removes its probe dir. This is the source of contract/verify-process-exit/interrupted, contract/process-exit-detail/interrupted and the stop pair. The M2 red case `cli_verify::verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1` asserts these, so a changed `WITHIN` bound must keep them.

@history_item n=14 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** (seven verify spawn pairs; a killed PTY child's exit status) — added Run E, the `--resume` start whose `start` the M2 test times out waiting for. It also measured that a PTY child killed at the stop carries `child_exit_status` and `duration_ms` (the resume-run stop case), which contract/verify-process-exit/child-exit-status-and-duration rests on. The M2 wait fix must not weaken that case's reading.

@history_item n=15 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (session-live, the agents-probe callers and verify's eight spawn pairs) — verify logs eight spawn pairs, and revive's preflight adds `session-live` with its `agents-probe` pair. This is the source of contract/revive-preflight-detail/session-live, contract/revive-process-log/agents-probe and the "eight" in the child-exit-status row. The slow case `chaos_revive::revive_after_a_killed_wrapper_…` runs this preflight.

@history_item n=16 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`verify` writes its refusal's code) — under a parsed `VIOLA_NAME`, verify writes its refusal code (including `unclean-fixture` and `strict-modes-failed` on a refused `update_stamps`), and a failing row carries none. This is the source of contract/process-exit-detail/verify-refusal and contract/diag-detail/strict-modes-failed. The `--record` homes of item 5 must not drift into an `unclean-fixture` refusal.

@history_item n=17 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`viola verify`'s home-tree check) — verify checks the home tree before its log opens (refused: exit 1, home pair, no line), and `update_stamps` checks stamps before writing. This is the source of contract/viola-verify/home-tree-refused, contract/viola-verify-refusal/home-tree-refused and decision/refused-home-tree-writes-no-line. A renamed `--record` home must still pass this check.

@history_item n=18 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree check of `run` and `viola revive`'s start) — run and revive's start check the home tree, including the pinned copy, before their log opens; a refusal opens no log, writes no line and exits 1. This is the source of decision/home-tree-check-before-start-log and decision/home-tree-check-before-log-init. It is the path the slow cases `cli_instance_state::run_refuses_a_tampered_pinned_copy` and `run_rewrites_the_plugin_folder_each_start` take.

@history_item n=19 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the hook event and dialog paths' instance check before obs init) — every hook arm but `--capture` checks the instance before obs init. A refused or absent instance exits 0 with no config, stdin, line, frame or SessionEnd append. This is the source of decision/hook-silent-without-trusted-instance, decision/hook-refused-instance-silent and decision/hook-silent-without-instance, which `tests/hook_fail_open.rs` exercises. The item-1 socket rename must keep those cases reaching the paths they test.

@history_item n=20 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the pre-init window and the panic exemptions) — each strict-modes check precedes the first open under the home, because that open narrows a widened home to 0700, and a refused check writes no line. This is the source of decision/strict-modes-before-diagnostics, decision/instance-check-before-log-init's why and decision/pre-log-strict-refusals-write-no-line. The negative tests assert that no diagnostics folder appears.

@history_item n=21 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the refusal-telemetry record: two strict-modes lines, every other refusal none) — set which strict-modes failures leave a line (the `ledger-stamps` read, and verify's refused stamp write) and that every other refused check leaves none. This is the source of decision/strict-modes-failures-lines, decision/strict-modes-refusal-writes-no-line and decision/pre-open-strict-modes-refusal-silent.

@history_item n=22 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the snapshot readers over a replayed snapshot) — a replayed snapshot carries no endpoint, pid or started_at. The hook then fails open `channel-unreachable`, and `run.collision_check` reads the heartbeat alone. This is the source of contract/replayed-snapshot/live-fields, decision/replayed-snapshot-unreachable and contract/run-span-attributes/run-collision-check. It is what the slow case `chaos_snapshot_replay::a_start_over_an_unreadable_snapshot_replays_the_log` reads.

@history_item n=23 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** (where `state-recovered` is written and how often) — `state-recovered{snapshot-replayed|snapshot-unsupported-v}` is written at WARN, one line per call, and none when the replay fails. This is the source of contract/state-recovered/snapshot-replayed and contract/state-recovered-detail/*, which the chaos_snapshot_replay slow case asserts.

@history_item n=24 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (`run.pin_copy` under a concurrent first start) — an absent pinned copy is placed no-clobber and never replaced; a lost placement re-hashes, and the outcome set stays `ok|pinned-hash-mismatch`. This is the source of decision/pinned-copy-never-replaced and contract/run-span-attributes/run-pin-copy. It sits behind the slow case `run_refuses_a_tampered_pinned_copy`, whose wall time item 2 must measure.

@history_item n=25 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (pinned-exe hash-mismatch refusal telemetry through a lost placement) — a lost placement re-hashing different also reads `pinned-hash-mismatch` and adds no line, field or code. This bears on decision/refusal-telemetry and contract/run-start-failure-detail/pinned-hash-mismatch, which the tampered-copy slow case asserts.

@history_item n=26 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (`server-verify-failed` written: the hook fail-open code) — the hook fails open with `hook-decision{detail:"server-verify-failed"}` at WARN and no frame, with `corr` null on the dialog path. No pid, start time, path or endpoint appears. This is the source of contract/hook-decision/server-verify-failed, contract/dialog-log-line/server-verify-failed and contract/hook-decision-detail/server-verify-failed, all exercised by `tests/hook_fail_open.rs` over the test channel whose socket name item 1 changes.

@history_item n=27 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the pre-frame server check's place in the span tree) — placed the server check inside the dispatch span and `hook.handle`, guarded by `request_refuses_an_unverified_server_inside_its_dispatch_span` and `hook_refuses_an_unverified_server_inside_hook_handle`, each red with the check removed. This is the source of decision/server-refusal-inside-root-span with its guard. It is also the precedent for item 4's remove-the-guard controls.

@history_item n=28 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at `viola hook`) — the hook's `connect` now runs `check_endpoint_dir` on the recorded endpoint's parent before `connect_by`; a refused directory is one `hook-decision{detail:"socket-dir-refused"}` at WARN with exit 0. This is the source of contract/hook-decision/socket-dir-refused and contract/hook-decision-detail/socket-dir-refused. Trap for item 1: a new test socket name or location must keep its parent directory passing this check (owner, 0700, not a symlink), or the fail-open cases read `socket-dir-refused` instead of their own detail.

@history_item n=29 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`socket-dir-refused` in the log catalogue) — added `socket-dir-refused` to the exit-1 and exit-21 lists and to the `hook-decision` fail-open codes. This is the source of contract/diag-detail/socket-dir-refused and contract/hook-decision-detail/fail-open-codes, which the hook fail-open tests assert.

@history_item n=30 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (Founder Direction 4: the socket-directory carve-out retired) — a refused check, a failed create and a lost race on the socket directory all read the one code `socket-dir-refused`. This is the source of decision/socket-dir-one-code, and of the except clause on decision/pre-open-strict-modes-refusal-silent.

@history_item n=31 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (3. Observability Harness Contract: 1 row) — `~ decision/instance-check-before-log-init`: the except now names one hook `--capture` arm, which runs ahead of the check. Hook fail-open tests and the item-5 helper see only that one capture arm.

@history_item n=32 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (1. Obs Scope Summary: 1 row) — `~ contract/replayed-snapshot/live-fields`: the promise lost «hook statusline prints nothing». It now reads only unreachable endpoint, hook `channel-unreachable` and run's heartbeat-only collision check, which is the current wording the hook and snapshot-replay tests hold.

@history_item n=33 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (6. Log Coverage: 5 rows) — added the `hyprctl-probe` subject and the `hyprctl-json` parser, with `oversize` past `MAX_FRAME`, for the Hyprland seam. `viola-hypr-tap.rs`, the recorder whose ten survivors item 4 kills, records that seam's replies (`record`, `record-dispatch`, `Reply::Version`, `Value::Array`). Any recorder test should keep fixtures consistent with these closed words.

@history_item n=34 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (1. Obs Scope Summary: 3 rows) — `~ decision/per-parser-parse-failure-counts`: the row now names the hyprctl replies, and a hyprctl parse failure reads the closed `oversize|malformed` with one `parse-rejected` line, never `unknown`. This is the obs side of the seam whose recorder item 4 hardens.

@history_item n=35 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** (fake agent's print-ban exemption) — only `viola-e2e` omits `[lints] workspace = true` and carries its own `[lints.clippy]`. Test-only bins elsewhere need a crate-level allow for print macros. `crates/viola-e2e/src/bin/viola-hypr-tap.rs` lives under that separate lint table, which bounds what item 4's `main` and its tests may do with stdout and stderr.

@history_item n=36 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** (plugin install's span, log shape and no-line refusal) — plugin install's refused tree check exits 21 with no log and no line. This is the source of contract/plugin-install-exit-codes/21; the `run_rewrites_the_plugin_folder_each_start` slow case reads this plugin tree.

@history_item n=37 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (strict-modes-failed on the stamps read) — added `parse-rejected{parser:"ledger-stamps", detail:"strict-modes-failed"}` from run's version gate, the first of the two strict-modes line sites in decision/strict-modes-failures-lines and contract/diag-detail/strict-modes-failed. It also moved CI verify to the recorded fake-agent version.
