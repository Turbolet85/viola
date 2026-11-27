@history_item n=1 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** (amd-f7kltdky, 1 of 2) — set `contract/claude-child-process-log/handle-wait-exit` (process-start for each child, the respawned one included; process-exit once that child is reaped or the pump returns; never on reader EOF), `contract/run-process-log/process-exit-self` (self process-exit follows only the child the pump ends on, none after a replaced child) and `decision/launch-line-untraced` (a switch appends one launch line, untraced). Why it matters: this is the switch respawn that move 2 takes out of `crates/viola-pty/src/pump.rs`. The moved restart module must keep this lifecycle order and these lines exactly, and add no span or line.

@history_item n=2 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** (amd-n7rc6pgf, 2 of 2) — set `decision/window-line-untraced` (a switch writes the window line again after its own launch line), `contract/send-refused-detail/turn-running` (a standing switch refuses a send turn-running until the new child's session-start) and added `contract/cli-log-file/switch`. Why it matters: the switch path reaches both the pump's respawn (move 2) and the window line (move 3). A pure move must keep both the re-written window line and the turn-running window.

@history_item n=3 marker=2026-11-27T19-07-11Z-wrap
**2026-11-27T19-07-11Z-wrap** (amd-lpaiygi4, amd-f5tk4tde, amd-ef3endhz, amd-e5tbi5n2) — `decision/launch-line-untraced`, `decision/lineage-line-untraced` and `decision/window-line-untraced` went from provisional to standing. The switch rows and adopt's not-resumed exception were ratified. Why it matters: these are the standing decisions the moved window step and the pump's respawn must keep word for word. They are no longer provisional, so a move cannot reopen them.

@history_item n=4 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** (amd-luw5ev7l, amd-luw5ev7l's sibling amd-tvkckso6, amd-2kukexyk) — set `decision/run-threads-inside-catch-unwind`, `decision/panic-caught-at-every-role` and `decision/run-worker-panic-ends-run` (rule and why) to three run PTY threads under catch_unwind: the two pump copy threads and the paste writer thread (`spawn_worker`, `Worker::Panicked`, child killed, exit 1 internal-error). Why it matters: moving the restart code out of `pump.rs` must keep every worker spawned through the same catch_unwind path. Otherwise a worker panic could leave main running with no process-exit, which is forbidden.

@history_item n=5 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (amd-yriuve77, amd-y6qadfpo) — `viola-pty` was cut to portable-pty with openpty its one backend, and the Windows `TerminateProcess` fallback was removed. `exit_source:"kill-fallback"` and `ExitSource::KillFallback` were kept in `viola-pty`, and handle-detected child exit lost its ConPTY qualifiers. Why it matters: these define the pump/seam surface that move 2 splits. `ExitSource` and the handle-wait exit path stay as they are, and `contract/pty-seam-log/kill-fallback` and `decision/child-exit-from-process-handle` stay true across the move.

@history_item n=6 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (amd-uo5ljwnp, amd-nh5gs6li) — re-derived the `human::refuse(` census per file (`run` 7, `verify` 5) with `grep -rcE 'human::refuse\(' src`. The OS-gated test census (`tests/contract_os_gated_tests.rs`) names each gated test function and file directly under `tests/` and `crates/*/tests/`. Why it matters (trap): the refuse census is counted per file. If the trust or window step that moves out of `src/cmd/run.rs` carries a `human::refuse(` site, the per-verb counts in §3/§11 move, an expected wrap amendment. Any gated test that is moved or re-pointed must still match its `GATES` row.

@history_item n=7 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (amd-yebnagzb) — the `viola-pty` entity and the logging-stack bullet read portable-pty `=0.9.0`: its `log` target is OFF and its failures are captured at the seam (D-11). Why it matters: the new restart module in `viola-pty` stays behind the same seam. It must enable no `log` bridge (`decision/no-log-crate-bridge`, `decision/portable-pty-log-target-off`) and capture failures from returned values.

@history_item n=8 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** (amd-betgscce) — a child of an interactive PTY run killed at a stop carries `child_exit_status`, which is the PTY pump's exit code (portable-pty reports a code for a signalled child). Why it matters: the pump's exit-status read is one of the measured behaviours the `pump.rs` split must preserve unedited.

@history_item n=9 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** (amd-c47zgdbt) — the vt100 feed thread (`src/run/gate.rs`) is a contained worker: a caught panic poisons the model, writes one `parse-rejected{vt100-feed}` and sends nothing to main. The no-process-exit ban covers the PTY pump threads. Why it matters: when the pump's threads are re-homed, the split between contained workers (feed) and session-ending workers (pump/paste) must not change.

@history_item n=10 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (amd-qne6ykzo) — set `contract/run-start-spans/span-chain`: under `--origin`, `run.window` follows the trust step, before `state.snapshot_write`. Added `contract/run-span-attributes/run-window` (field `outcome` alone), `contract/cli-log-file/open` and `decision/window-line-untraced`. Why it matters: move 3 takes `open_window` and the trust step out of `src/cmd/run.rs`. The span's static name, its `skip_all` field allow-list, its place in the chain (after trust, before snapshot_write) and the untraced window line must survive the move unchanged.

@history_item n=11 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (amd-lmjqnkhr, amd-4hcnuf6k) — added `contract/run-start-events/window-line` (one window line `{address, workspace, placement, origin}`, source wrapper, after launch) and extended `contract/strict-modes-refusal-exit/1` to `open`. Why it matters: the window line's place in the start's product order is a contract the moved window step must keep writing at the same point.

@history_item n=12 marker=2026-11-25-placing-without-moving-the-view
**2026-11-25-placing-without-moving-the-view** (amd-iqxvshpu) — added `decision/window-view-restore-uninstrumented`: the window step's before/after active-workspace reads and its restore add no placement word, span field or log line. Their only instrumentation is the seam's hyprctl-probe pairs. Why it matters: this is the latest change to the window step that move 3 relocates. The scope bars any change to the window code's logic, so this behaviour moves as it is.

@history_item n=13 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (amd-meyxwweu, amd-b6657mge, amd-vflsty3z, amd-km4rcxjc) — added the hyprctl spawn to `decision/os-apis-at-call-sites` and `decision/instrument-call-sites-not-os-internals`, and `hyprland` to `decision/span-naming`. Added the hyprctl-probe subject and the hyprctl-json parser with its exception to `decision/parse-failures-unknown-counter`. Why it matters: the window step calls this seam. Moving it must keep the hyprctl spawn instrumented at viola's own call site and keep the span names static.

@history_item n=14 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (amd-ye23emtv, amd-qxgopvmr, amd-v67jn6xb, amd-es35y2ig, amd-yjk3figz, amd-mt35652l) — introduced the start's trust step. `decision/claude-boundary-only-instrumentation` names its one trust-record line. `decision/trust-record-write-uninstrumented` says it opens no span and writes only that line. `contract/start-trust-log/trust-record` and `contract/diag-line-fields/trust-record` were added. `decision/refusal-telemetry` and `decision/claude-internals-not-a-contract` gained the start's trust-record write (flag only, guarded writer, no refusal line). The `CLAUDE_CONFIG_DIR` read went into `decision/r8-strip-logged-as-names-only` and `decision/env-vars-not-configuration`. Why it matters: this is the trust step move 3 takes out of `src/cmd/run.rs`. After the move it must still write exactly one INFO trust-record line with one of the 16 words, open no span, and log no path, key or value.

@history_item n=15 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (amd-kvyldec3, amd-klhmhcby, amd-mwp5uama, amd-u2v2pm5c) — ratified the trust-step rows of 2026-11-18. Set `contract/run-terminal-output/child-output`: before the child, a keyed start writes its automatic-verify lines and the verify child's output, none under revive --json, and nothing while the child runs. Why it matters: the trust rows are now standing, and the run-terminal promise binds the boot path that `run.rs`'s split touches. That is why the plan holds the harness smoke trio.

@history_item n=16 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (amd-6pluk4bc, amd-2whzt7ul, amd-m3r652po, amd-somqwxfd, amd-idjn364c, amd-p3wvgazh) — adopt's start runs run's own start and trust step. `contract/start-trust-log/trust-record`, `decision/r8-strip-logged-as-names-only` and `decision/env-vars-not-configuration` name adopt. `decision/adopt-start-logs-as-run` (no event, detail, field, subject or span of its own) was added, and the launch and lineage untraced rows name adopt. Why it matters: `run.rs` is the shared start of run, revive and adopt. The moved trust and window steps must stay on the one shared path so adopt still adds nothing of its own (ratified W5: amd-zuilxaw5, amd-wkkfty34, amd-nrkjlvjr).

@history_item n=17 marker=2026-11-24-adopt-s-resume-folder
**2026-11-24-adopt-s-resume-folder** (amd-nfvgg26d, amd-uoop6ciz) — gave `decision/adopt-start-logs-as-run` its one exception, the detail not-resumed on adopt's self process-exit exit 1 written after the claude-child process-exit. Why it matters: this is a start/pump-return ordering the run.rs and pump.rs moves must keep.

@history_item n=18 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (amd-nohbky3m) — set `contract/diag-subject/terminal` (one process-start, no process-exit, for each terminal open or up spawns), `contract/diag-detail/strict-modes-failed` (written on up's exit-1 process-exit under a parsed VIOLA_NAME) and the exception on `decision/symlink-skip-logged-as-count` (up writes no state-entry line). Added `contract/process-exit-detail/boot-time-unread`. Why it matters: these are the lines the `upped` loop's act half (spawn, wait, line) writes. Splitting `upped` into plan and act must keep one terminal process-start per spawn and these detail words.

@history_item n=19 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (amd-4ohqryia, amd-zigefsdw, amd-dntl63mx, amd-ilggz2ka, amd-szmte3sh, amd-otyhke6a) — set `contract/process-log/child-and-probe-lifecycle`, `decision/strict-modes-failure-lines` and `contract/process-exit-detail/strict-modes-failed` (up's home tree refused after its board scan), and `contract/strict-modes-refusal-exit/21` and `/1` (up's board scan, home-tree check). Gave `decision/strict-modes-before-diagnostics` its up exception, and added `contract/cli-log-file/up` and `contract/process-exit-detail/exit-1-up`. Why it matters: these are up's refusal order (scan, then home tree before the first spawn) and its exit-1 detail rule (not-live, else none). The split's plan half (refusal, order, skip) must keep them exactly. Ratified by amd-o3ixp7o4, amd-nnehuuuq and amd-hmlr6dgt.

@history_item n=20 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (amd-hb4z2fp7) — set `contract/run-start-spans/span-chain` to the chain without `run.budget_eval` (state.snapshot_recover › run.collision_check › run.pin_copy › run.version_gate › channel.bind › state.snapshot_write › state.heartbeat_start › pty.spawn). Why it matters: this is the current base of the start chain that move 3 must leave intact. `run.start` still closes when `pty.spawn` returns.

@history_item n=21 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (amd-yfhlqjyk, amd-477gj7qu, amd-7l763bjy) — `checked_exe` / `refuse_tree` run before `open_wrapper_log` in run's and revive's start, and `state.snapshot_recover` is placed under `run.start` before `run.collision_check`. The `human::refuse` census was counted per file. Why it matters: these are prologue orders inside `src/cmd/run.rs` that the move must not reorder (the scope keeps the start prologue out), plus the per-file refuse census as a trap.

@history_item n=22 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (amd-j7mnbkll, amd-y4dp2cj4) — the start's socket-directory step (`socket_dir_ready`) carries no span and writes a line only on refusal (`socket-dir-refused`), and the run refuse census moved 7 to 8. Why it matters: this is a precedent for unspanned start steps inside `run.rs`. Its refusal site is counted by file, so moving it would move the census.

@history_item n=23 marker=2026-10-10-session-links
**2026-10-10-session-links** (amd-rzzmd2hs) — recorded which start steps carry no span and no line (the link set's restore among them), and kept the span chain and the start-record order unchanged. Why it matters: this is a precedent that the start's unspanned steps are a closed, recorded set. Moving the trust and window steps must neither add a span nor turn a spanned step into an unspanned one.

@history_item n=24 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (amd-lyzkv6h4, amd-nen7xeyv) — named `refused_with` (`src/cmd/run.rs`) as the --json writer of the start refusals (before `spawn_child`; a `pump_child` error prints no document) and `collision_check` (`src/cmd/run.rs`) as the writer of `stale-heartbeat`. Why it matters: the master cites these by path. If move 3 relocates them or their callers, the citations are expected wrap amendments, and the no-document-after-pump rule holds.

@history_item n=25 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** (amd-5psnrxhp) — added `contract/run-process-log/auto-verify-stop`: a stop in the start's verify window ends the start by the signal with no process-exit of its own. Why it matters: this is a start-path exception that lives in the boot path `run.rs`'s split touches, so it must survive the move.

@history_item n=26 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (amd-wurmqixn) — counted the snapshot readers by rule (`read_snapshot(` under `src/`, outside test modules, `src/cmd/run.rs` among them), with the standing lesson that a count carried from a plan is re-measured at the wrap, never copied. Why it matters (trap): the scope's code-line figures are «inferred» from the earlier take-up. Every count that names a moved file (rules that walk `src/` and exclude test modules) must be re-measured after the move.

@history_item n=27 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (amd-u35ojlie) — standing rule: a master names a planted literal by its home, never as `path:N`, so the citation sweep does not re-point it. Why it matters: a pure move shifts line numbers in `run.rs`, `pump.rs` and `up.rs`. Plan steps that cite symbol and line, and any master `path:N` citation of a moved function, need re-pointing at the wrap. Planted literals do not.

@history_item n=28 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** (amd-vxj66te3) — obs code meets cargo-mutants only at `run --mutants` (the epoch-boundary audit or on demand), not in CI. Why it matters: this backs `decision/obs-code-mutation-tested` and `decision/surviving-obs-mutant-fails`. The chunk plans no mutation run, but moved obs sites (span declarations, `obs_event!` calls) change what the next audit's diff reads, so their tests must move with them.

@history_item n=29 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** (amd-trr6pu6x) — tracing-subscriber is a product dependency of the root bin only. `viola-channel` and `viola-state` take it as a dev-dependency for their unit tests' line capture. Why it matters: if the moved `pump.rs` test module in `viola-pty` captures lines, it must use what `viola-pty` already has. A pure move adds no dependency (`decision/sync-crates-without-tokio`, `decision/third-party-targets-off`).
