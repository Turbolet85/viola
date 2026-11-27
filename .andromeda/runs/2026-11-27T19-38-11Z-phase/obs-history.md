@history_item n=1 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — the mutation row left CI: obs code meets cargo-mutants only at `agent-run run --mutants` at the epoch boundary or on demand, and a surviving obs-code mutant is a §10 failure at such a run (decision/surviving-obs-mutant-fails, decision/obs-code-mutation-tested). Item 1's one announced addendum run is exactly this form; item 2 and 4 survivors in `viola-pty` / `viola-state` are the reading the next audit re-runs.

@history_item n=2 marker=2026-10-10-windows-mutation-grade
**2026-10-10-windows-mutation-grade** — red at a mutation run is a timed-out obs-code mutant, a missed one not host-excluded, or unviable outnumbering caught; "surviving" is the harness document's `survived`, a host-never-compiled miss left out. Item 1's addendum must report caught / missed / timeout / unviable counts against this rule; a TIMEOUT is red, which is why the baseline's 15 s kill of `path4_…` matters.

@history_item n=3 marker=2026-09-24-epoch-1-cleanup
**2026-09-24-epoch-1-cleanup** — the Mutation row names the per-run unviable rule (unviable outnumbering caught is red). Bears on item 1's addendum verdict for the root crate's 372 in-diff mutants.

@history_item n=4 marker=2026-09-26-ci-chunk-base-and-union-verdict
**2026-09-26-ci-chunk-base-and-union-verdict** — `target/agent-run/chunk.diff` (the mutation run's diff) is excluded from the secret scan and the harness upload (decision/mutation-diff-never-uploaded). Item 1's `--in-diff` run produces this file; it must stay at that exact path, never uploaded.

@history_item n=5 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — `mutants.out/` and `target/run-archive/<n>/` (JUnit and `outcomes.json` with absolute argv paths) are never uploaded (decision/local-run-outputs-never-uploaded, decision/mutants-out-never-uploaded). Item 1's addendum run writes both; the ledger addendum may carry counts only, no path.

@history_item n=6 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** (pre-push document and mutation scratch wording) — on the Linux dev host the NOCOW `TMPDIR` scratch is printed by no document and `mutants.out/` stays at the repo root; the pre-push document carries no absolute path. Item 1 sets `TMPDIR` under `~/dev/projects/viola-mutants-scratch/`: no record of the addendum may name it beyond the repo-relative / `~/…` form.

@history_item n=7 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (one CI leg; ConPTY sideload and Windows kill fallback retired) — the Windows mutation leg and `windows-mutants.yml` are gone, openpty is the one PTY backend, yet `exit_source:"kill-fallback"` (`ExitSource::KillFallback`) stands in `viola-pty` (contract/pty-seam-log/kill-fallback). Item 2's killing tests on `reap_within` / `drain` must keep handle-wait vs kill-fallback exits as logged; item 6's pump split must not drop the kill-fallback path.

@history_item n=8 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** (4. Span / Trace Coverage, 1 of 2) — the pump now owns a respawned child: process-start once each child is spawned (a switch's respawn included), process-exit once that child is reaped or the pump returns, never on reader EOF; run's self process-exit follows only the child the pump ends on (contract/claude-child-process-log/handle-wait-exit, contract/run-process-log/process-exit-self). This is the restart path of `pump.rs` whose nine survivors item 2 kills and whose restart code item 6 moves out: the generation guard and `reap_within` / `drain` limits carry these log promises.

@history_item n=9 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** (4. Span / Trace Coverage, 2 of 2) — a switch writes the window line again after its own launch line (decision/window-line-untraced) and `viola switch` joined the instance-check-before-log-init list. Item 6's move of the window step out of `run.rs` must keep the switch's re-write of the window line.

@history_item n=10 marker=2026-11-27T19-07-11Z-wrap
**2026-11-27T19-07-11Z-wrap** — the previous wrap ratified as standing decision/launch-line-untraced, decision/lineage-line-untraced, decision/window-line-untraced, decision/lineage-names-confined and the switch / adopt / up / verify-refusal rows (every PROVISIONAL mark at `a80df0e5`). This chunk's pure moves rest on those rows as standing, with no new obs row expected.

@history_item n=11 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** — the three run PTY threads (two pump copy threads, the paste writer) run under `catch_unwind`, a worker panic goes to main which kills the child and exits 1 `internal-error`, the child exit polled on main (decision/run-threads-inside-catch-unwind, decision/run-worker-panic-ends-run, decision/panic-caught-at-every-role). Item 6's move of restart code out of `pump.rs` must keep every worker inside `spawn_worker` / `catch_unwind`; item 2's guard tests run beside these.

@history_item n=12 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — the vt100 feed thread is a contained worker (caught panic poisons the model, one `parse-rejected`), and the "no process-exit" ban covers the PTY pump and the handle-wait path. Constrains the `pump.rs` split: the feed stays contained, the pump threads stay fatal.

@history_item n=13 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — a child of an interactive PTY run killed at a stop carries `child_exit_status` (the pump's exit code) and `duration_ms`, measured. Bears on item 2's `reap_within` / `drain` time-limit tests: the exit status reaching the pair is a promise those mutants touch.

@history_item n=14 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the portable-pty pin at =0.9.0) — portable-pty `=0.9.0`'s `log` target is OFF and PTY failures are captured at the seam (decision/portable-pty-log-target-off, decision/no-log-crate-bridge). The `pump.rs` split moves seam code; failures must stay captured from returned values, never through a log bridge.

@history_item n=15 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — revive's start arm is role `run`, its log opened by the setup it shares with `run` before the preflight; the preflight reads strict-modes, holder, session, directory in order (decision/revive-start-logs-as-run, decision/revive-preflight-order). Item 3's shared prologue is that shared setup: order and role must not move.

@history_item n=16 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (a refused instance check writes no line, on both arms) — revive's start arm runs its instance check before the log init; a refusal is the stderr pair and exit 1 with no log, no process-start, no process-exit, because opening the log narrows the home the check would refuse. This is the trap item 3's prologue must keep: the check stays ahead of `open_wrapper_log` in the one function.

@history_item n=17 marker=2026-10-20T06-58-31-wrap
**2026-10-20T06-58-31-wrap** — the founder accepted as built the no-line shape of revive's refused instance check (and the other no-line refusals). Item 3's move keeps this shape; no line may appear on a refused check after the merge.

@history_item n=18 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the link verbs' instance check before their log) — the order trap is held by a test that reads the mode after the refusal and reads red with the check moved (`link_verbs_refuse_a_widened_instance_before_any_log`). Precedent for item 3: the prologue's check-before-log order is guarded by a mode-after-refusal test, which must stay green unedited.

@history_item n=19 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree check of `run` and revive's start, before the wrapper log; the pre-init window) — `checked_exe` / `refuse_tree` run before `open_wrapper_log` in both starts (revive after its `check_instance`); a refusal opens no log, writes no line, starts no child (decision/home-tree-check-before-start-log, decision/home-tree-check-before-log-init, decision/pre-log-strict-refusals-write-no-line). Item 3's single prologue is this call sequence; the `run` start stays outside it but shares the order.

@history_item n=20 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (E5: the replay's product readers; `state.snapshot_recover` placed) — revive's preflight makes one snapshot read for its collision check and its directory reading, one replay, one `state-recovered` line. Item 3's prologue merge must not add a second read for adopt or revive.

@history_item n=21 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — revive's preflight reads holder, session, `session-live`, directory; the `session-live` reading (skipped under `--fork`, fail open) writes one `agents-probe` pair into `run-<name>.ndjson`. adopt later shares it (`live_elsewhere`). Item 3: the shared prologue carries this probe pair for both verbs.

@history_item n=22 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (every refusal of `revive --json`'s start prints its document; a stale holder is `stale-heartbeat`) — `refused_with` (`src/cmd/run.rs`) prints the exit-1 document for start refusals under revive's `--json`, an anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` prints the `internal-error` document. Item 3's prologue sits on these call sites; item 6's split of `run.rs` may move `refused_with`, which `revive` and `adopt` call.

@history_item n=23 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (exit 21's closed causes; the start's socket-directory step) — `refused_with` also carries `socket-dir-refused`, shared by revive's start; the socket-directory step has no span and a line only on refusal. Constrains the `run.rs` split and the shared prologue: refusal writers and their order stay.

@history_item n=24 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `revive <name> --list` is role `cli` (role_of), logging into the caller's `cli-<VIOLA_NAME>.ndjson` only after its strict-modes check (contract/cli-log-file/revive-list). Item 3 must keep the `--list` arm outside the start prologue that logs as `run`.

@history_item n=25 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (2. Telemetry Strategy; 3. Observability Harness Contract; 4. Span / Trace Coverage) — adopt's start runs its strict-modes and instance checks before `open_wrapper_log`, exits 1 with no line, logs as `run` through run's own lines and spans adding no event, detail, field or span of its own (decision/adopt-start-logs-as-run, decision/home-tree-check-before-log-init, decision/instance-check-before-log-init, contract/strict-modes-refusal-exit/1). Item 3's prologue unification is the copy these rows were written against; the merged function must keep every row true for both verbs.

@history_item n=26 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (1. Obs Scope Summary; 6. Log Coverage; 7. Error Capture & Reporting; 8. PII Scrubbing) — adopt writes `session-live` on its own exit-1 process-exit, its `strict-modes-failed` refusal writes no line, its `agents-probe` pair comes from the shared `live_elsewhere`, its start is silent at the internal-error catch site, and its typed session id is never logged (contract/process-exit-detail/exit-1-adopt, contract/cli-internal-error/human-line). Item 3: the shared prologue must not carry adopt's session id into any line.

@history_item n=27 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (trust-record, R8 value read, env vars, socket-dir rows naming adopt's start) — adopt's start runs run's trust step and its socket-directory refusal, so the trust-record line, the `CLAUDE_CONFIG_DIR` value read and `socket-dir-refused` exit 1 name run, revive and adopt (contract/start-trust-log/trust-record, decision/r8-strip-logged-as-names-only, ratified W5). Item 6's move of the trust step out of `run.rs` serves all three starts.

@history_item n=28 marker=2026-11-24-adopt-s-resume-folder
**2026-11-24-adopt-s-resume-folder** — adopt's own self process-exit exit 1 `not-resumed`, after the child's process-exit, when the resumed child ends before its session starts (the except of decision/adopt-start-logs-as-run). The one adopt-only line the shared prologue (item 3) must not fold into revive.

@history_item n=29 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — the start's trust step writes the claude config's trust flag only, opens no span and writes exactly one INFO trust-record line (16 closed start words, no path, key or value); its guarded writer refuses a symlinked, non-regular or foreign file with no refusal line (decision/trust-record-write-uninstrumented, contract/start-trust-log/trust-record, decision/claude-boundary-only-instrumentation, decision/refusal-telemetry). Item 6 moves the trust step out of `run.rs`: one line, no span, no new refusal line after the move.

@history_item n=30 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — verify's trust-record write and removal emit no line and no span, the outcome on the stamps (decision/trust-record-write-uninstrumented). Shares the guarded writer with the start's trust step that item 6 relocates.

@history_item n=31 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (4. Span / Trace Coverage) — under `--origin`, `run.window` follows the trust step and precedes `state.snapshot_write`, carrying `outcome` alone; the window line is untraced (contract/run-start-spans/span-chain, contract/run-span-attributes/run-window, decision/window-line-untraced). Item 6 moves the window and trust steps out of `run.rs`: the span name, its field allow-list and its place in the chain must not move.

@history_item n=32 marker=2026-11-25-placing-without-moving-the-view
**2026-11-25-placing-without-moving-the-view** — the window step's before/after workspace reads and conditional restore add no placement word, span field or line (decision/window-view-restore-uninstrumented); verify gained `untrusted-folder`. The just-landed window code item 6 moves; the scope's boundary forbids its rework.

@history_item n=33 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (run.budget_eval in the start chain) — the unit case that pins the start's span chain was restated when the chain changed (since retired with the gate). Precedent for item 6's `run.rs` split: the chain-pinning captured-span unit case must stay green with no edit beyond a re-pointed path.

@history_item n=34 marker=2026-11-24-viola-up
**2026-11-24-viola-up** — `viola up` logs as `cli` only under a parsed `VIOLA_NAME` after its strict board scan, inside `cli.up`; a refused scan writes no line and exits 21; its home-tree refusal before the first spawn ends the opened log with `strict-modes-failed`; one `process-start{subject:"terminal"}` per spawn, no exit; exit-1 detail rules and `boot-time-unread` (contract/cli-log-file/up, contract/process-exit-detail/exit-1-up, decision/strict-modes-before-diagnostics except, decision/strict-modes-failure-lines). Item 6 splits `upped` in `src/cmd/up.rs`: every line and its order stay.

@history_item n=35 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (2. Telemetry Strategy) — up's board scan exits 21 and its home-tree check exits 1 on a strict-modes refusal, under `--json` the strict-modes-failed documents (contract/strict-modes-refusal-exit/21, contract/strict-modes-refusal-exit/1). Item 7's one `refused()` helper serves these `--json` refusal shapes for `open` and `up`; they must print unchanged.

@history_item n=36 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (2. Telemetry Strategy; 6. Log Coverage) — `viola open` exits 1 on a strict-modes refusal, under `--json` the strict-modes-failed document; its exit-1 details `terminal-not-found` and `not-live`; the terminal subject. Item 7: the merged `refused()` must keep `open`'s document and pair byte-identical.

@history_item n=37 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the `.cmd`/`.bat` start refusal retired; the refusal writer's site counts) — the `human::refuse(` census, re-derived by `grep -rcE 'human::refuse\(' src`, reads `run` 7, `verify` 5 (§3, §11); earlier counts in «2026-11-01-unix-endpoint-and-home-hardening — `run`'s start refusals and the `human::refuse` census». Items 3, 6 and 7 move refusal call sites across files: the per-file census stated in the master may drift and the wrap re-measures it.

@history_item n=38 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — every start writes a lineage line, log-only and untraced, its parent and role confined to product surfaces (decision/lineage-line-untraced, decision/lineage-names-confined). Item 4's `ancestors(pid, max)` bound in `crates/viola-state/src/liveness.rs` sits near this path; its killing tests add no line or span.

@history_item n=39 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — standing rule: a master names a planted literal by its home, never as `path:N`. Trap for any wrap amendment item 6's splits bring: line citations (`pump.rs`, `run.rs`, `up.rs`) re-point by symbol, not by line.
