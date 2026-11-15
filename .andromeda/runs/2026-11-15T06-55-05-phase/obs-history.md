@history_item n=1 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — the status-line interception was retired whole. Removed rows: the statusline_command rows, `contract/hook-stdout/statusline`, the statusline spans, `contract/diag-subject/statusline-shell` and the statusline decision-detail rows. Changed rows: `~ contract/diag-line-fields/process-start` and `process-exit` (no statusline-shell, no shell_exit_status), `~ contract/diag-line-fields/hook-events` (nine hook names), `~ decision/capture-arms-uninstrumented`, `~ decision/instance-check-before-log-init` (one `--capture` arm), `~ decision/span-naming`, `~ contract/child-spawn-log/subject` and `~ decision/verify-lines-carry-no-content` (Run B's status-line probe retired). Why it matters: CARRY 3 reads the user's own status line live beside an unwrapped one, and CARRY 2's box-only screen fixture exists only because these rows were retired. The live round must find no `--settings`, no settings override and no statusline line or span in any log.

@history_item n=2 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — run.budget_eval left the start chain. Changed rows: `~ contract/run-start-spans/span-chain`, `~ contract/run-start-events/order` (wheel then session-start, no budget-gate), `~ decision/product-order-from-events-log`, `~ decision/exit-code-log-field` (11 retired, never reused), `~ contract/config-json/known-keys` (four keys, five_hour/seven_day counted as unknown) and `~ decision/gauges-from-harness-snapshots`. The additive-only rules were narrowed to "a field the product still emits": `~ decision/diag-schema-additive-only`, `decision/log-fields-additive-only`, `decision/log-schema-additive-only` and `decision/obs-aligns-to-tests-log-format`. Why it matters: v2-03 proves "the documented start order with no budget step", and these rows are what that start is read against. A live home with a stale `config.json` carrying the threshold keys logs one parse-rejected config-json unknown-keys line and does not refuse.

@history_item n=3 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the dialog-hold expiry became the wrapper's. At the hold's end the wrapper writes `dialog-expired` and `dialog-answered{deadline_hit:true}`, and the hook's `hook-decision` has `decision_emitted:false, deadline_hit:false`. The hook writes `deadline_hit:true` only at its backstop, `DIALOG_HOLD_MAX + 5 s` (3605 s). `run.dialog_await` `deadline_ms` = `dialog_hold_s` (60 000–3 600 000 ms). The start reads `config.json` for `dialog_hold_s`, and a bad value gets one `parse-rejected{config-json, malformed}`. Rows: `contract/config-json/dialog-hold-s`, `contract/events-log/dialog-expired`, `contract/hook-decision/deadline-hit`, `decision/expired-dialog-renders-for-human`, `contract/wait-span-attributes/run-wait-dispatch`. Why it matters: v2-02's near-3600 s live read must set `dialog_hold_s` near 3600 and check exactly these lines. Do not expect `deadline_hit:true` on the hook line unless the read passes 3605 s. The hold was a boundary widening the founder ruled himself.

@history_item n=4 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — this was measured false in a live run: `dialog-answered` carries `from`/`from_trust` only on an answer that carried a `from`. A `viola answer` from a process with no `VIOLA_NAME` writes neither field (row `contract/dialog-log-fields/dialog-answered`). Why it matters: it is a live-reading precedent and a trap for the dialog-hold step's assertions about how the rig answers.

@history_item n=5 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's Epoch 5 batch superseded the PROVISIONAL delegate marks on several entries, the dialog-hold-for-the-driver expiry entry among them. Their claims stand. Why it matters: it is the precedent for how a PROVISIONAL mark is settled by the founder's own word, which is what fork (4) on the 3600 s hold's PROVISIONAL mark must do.

@history_item n=6 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — verify-probe joined the spawn subjects. In CI, verify runs only against the fake agent, and the real-`claude` verify runs only through the local `run --local-live`, which refuses under `CI`. The harness `boot` readiness stage `start_records` checks `events.ndjson` lines 1-3. Rows: `decision/ci-verify-fake-agent-only`, `decision/real-cli-verify-never-in-ci`. Why it matters: this chunk's operator steps are local live starts that the rigs rehearse under the fake agent first. That split is exactly this rule, and the live verify that stamps the ledger for `cli_verified:true` must run outside CI.

@history_item n=7 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — verify's agent-readable outcome is the exit code plus stdout step lines and the last `stamped <version>  N pass  N fail` line. The hidden `hook --capture` arm is uninstrumented by design. Rows: `decision/capture-arms-uninstrumented`, `decision/hidden-capture-arms-uninstrumented`. Why it matters: the round stamps the ledger for the CLI it runs and reads verify's outcome. The capture arm stays lineless while a `--record` runs.

@history_item n=8 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — `verify-pty-probe` joined the subject enum, and verify's interactive PTY runs are instrumented at the call site with no row text, prompt or path in any line. Why it matters: the `--record` run that records CARRY 2's screen fixture is one of these PTY probes.

@history_item n=9 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — verify logs six spawn pairs. The capture arm's `--answers` prints one product-built decision body. CI drives the fake agent's four interactive runs with `--dialogs`. Why it matters: it is a precedent for verify's spawn-pair count and its fake-agent flags in the rehearsals.

@history_item n=10 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — CI's fake-agent verify flags are `--screens --turn-stop --trusted-root <root> --dialogs --framing`. Why it matters: the fake rehearsal of a new `Screen.*` box-only fixture replays through these screen flags.

@history_item n=11 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — this added the `hook statusline --capture` arm (Run B's probe settings), uninstrumented, now retired. Verify prints one `[NN/MM]` step line per relied row. Why it matters: it is history of the Run B status-line probe, which the status-line removal retired. A clean `--record` no longer meets those rows (CARRY 2).

@history_item n=12 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — this made seven verify spawn pairs (Run E `--resume`). A PTY child killed at a Ctrl-C/TERM stop carries `child_exit_status` (measured), while a killed print or `--version` probe child was not measured. Why it matters: every live spawn here has a kill deadline, and this is what its exit line should read.

@history_item n=13 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a stopped verify ends with `process-exit{exit_code:1, detail:"interrupted"}` after closing every spawn pair. The probe dir is removed on pass, fail or stop, but not on SIGKILL or a crash. `run`'s version gate observes no stop. Why it matters: a live verify killed at a rig deadline must read this way, and a SIGKILL leaves the probe dir behind.

@history_item n=14 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — verify has eight spawn pairs, the eighth an `agents-probe` around Run B's agents read. `parse-rejected{claude-agents-json}` covers an oversize or malformed answer, and `run::agents::read_logged` writes the pair for list, revive and verify. Row: `decision/agents-probe-parse-failure-unknown`. Why it matters: the live CLI's `claude agents --json` answer feeds this parser and pair.

@history_item n=15 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — verify's `MM` became 24 (row `local-command-compact`), and Run B's six pastes add no spawn pair. Why it matters: it is the relied-row count that a green ledger for the round's CLI is read against (later set to all 24 by the Linux-only chunk).

@history_item n=16 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — `parse-rejected{parser:"ledger-stamps"}` gained `strict-modes-failed`: run's version gate reads a refused stamps file as unverified (`cli_verified:false`). Rows: `contract/run-log-parse-rejected/ledger-stamps`, `contract/parse-rejected-detail/strict-modes-failed`, `decision/unreadable-ledger-means-unverified`. Why it matters: v2-03 needs `cli_verified:true` on a green ledger. Any unreadable, malformed or refused stamps file silently reads unverified, with this line as the only witness.

@history_item n=17 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — `process-start{subject:"claude-child"}` gained `env_kept`. D-34 superseded D-14: every stripped and kept name is logged, the values are never read, and the compile-time list is gone. Rows: `contract/claude-child-process-start/env-kept`, `env-stripped-count` and `env-stripped-known`, `decision/r8-strip-logged-as-names-only`. Why it matters: the live wrapped start's claude-child line carries these fields, and no value may appear in them.

@history_item n=18 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — E2's floor sentence was corrected: the strip always removes the 11-name `IDENTITY_FLOOR`. A differing Linux dev-host reading on 2.1.287 moves nothing. Row: `decision/identity-floor-always-stripped`. Why it matters: it is a live dev-host precedent, and the live start's `env_stripped_known` is read against this floor.

@history_item n=19 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — on the dev host, `target/e2e-home` is a link to an owner-only tmpfs dir, so a kept home's diagnostics end at reboot and `/tmp` ages them out after 10 days. Trap: G2's `find` needs `target/e2e-home/` with a trailing slash to descend the link. Why it matters: this chunk's live homes are `target/e2e-home/viola-live-*` on tmpfs (`df -h /tmp` read first). Evidence read from those homes must be captured before they age out, and the gates scanning them depend on the slash.

@history_item n=20 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (F4) — the one test home outside the scan root is `tests/chaos_feed_panic.rs`'s `TestHome::outside_scan()`. Every other integration home lives under `target/e2e-home/`. Rows: `decision/test-homes-under-scan-root`, `decision/test-homes-under-e2e-home` (except). Why it matters: the new live-round rigs' homes must sit under the scan root, or G2, G4 and the secret scan miss them. A second carve-out would need a founder ruling.

@history_item n=21 marker=2026-10-02-epoch-2b-cleanup
**2026-10-02-epoch-2b-cleanup** — CI keeps every home: neither harness `cleanup` nor a test home's drop deletes one before G2, G4, the scan and the uploads run (`AGENT_RUN_KEEP_HOMES=1`). Rows: `decision/test-homes-kept-until-gates`, `decision/homes-kept-until-gates-run`. Why it matters: it is the home-retention rule the fake rehearsals of the live rigs run under in CI.

@history_item n=22 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (v1-47's secret-scan reading; the mcp role writes no self process-exit) — the secret scan runs after each job's end-to-end run with `AGENT_RUN_KEEP_HOMES=1` and `if: always()`. The `mcp` role writes no self `process-exit` line. A `viola mcp` stdin line past `MAX_FRAME` before `initialize` exits 1 with one `parse-rejected` line. Why it matters: the live `claude` start through `viola-mcp-tap` (CARRY 1) runs a `viola mcp` session, and its log is read against these shapes.

@history_item n=23 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp` always logs to `<home>/diagnostics/mcp.ndjson` (D-09), with no stderr fallback in the current rows. The `list` tool writes its `agents-probe` pair and board lines into `mcp.ndjson`. A refused home check exits 1 with nothing printed. Rows: `decision/mcp-log-sink`, `decision/mcp-log-file-sink`, `contract/diagnostics-files/mcp`, `contract/viola-mcp-tools/list`. Why it matters: CARRY 1 re-records `Mcp.tools-list-1.json`, the `list` tool's description, through a live session that writes these lines.

@history_item n=24 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — `viola mcp` stdin is read in lines of at most `MAX_FRAME`, and a longer line ends the session with one `parse-rejected{mcp-stdin, oversize}`. The canary case `mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line` runs on its own fresh home. Row: `decision/canary-only-in-detail-files` (guard). Why it matters: the MCP tap's live session goes through this cap, and the canary guard is a row this chunk rests on.

@history_item n=25 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — `run` and revive's start run `checked_exe` and `check_home_tree` before the log opens, and a refusal opens no log, writes no line and starts no child. Verify checks the home tree before its log, and `update_stamps` checks the stamps. Over a replayed snapshot, `run.collision_check` reads the heartbeat alone. `state.snapshot_recover` sits under `run.start`. Rows: `decision/home-tree-check-before-start-log`, `decision/refused-home-tree-writes-no-line`, `contract/run-span-attributes/run-collision-check`, `decision/pre-log-strict-refusals-write-no-line`. Why it matters: the live test homes must pass the 0700/0600 tree checks, or the wrapped start refuses with no log line to read.

@history_item n=26 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — a stale holder's exit-1 code is `stale-heartbeat`. `already-live` means a live holder or a fresh replayed beat, while the span outcome stays `already-live` for both. Verify writes its refusal code on `process-exit` (`program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`, `interrupted`). Rows: `contract/run-start-failure-detail/already-live`, `contract/run-start-failure-detail/stale-heartbeat`, `contract/run-exit-1-detail/*`, `contract/process-exit-detail/verify-refusal`. Why it matters: v2-03's "a live or stale name refused" is read on these two distinct codes, and an `unclean-fixture` refusal is what a dirty `--record` gives.

@history_item n=27 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — `already-live` was first logged for both a live and a stale holder, and a gone holder is taken over with no exit-1 line. Why it matters: it is the precedent for the live/stale refusal, later split by the exit-cause catalogue. A gone holder is not refused.

@history_item n=28 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — an absent pinned copy is placed no-clobber and never replaced. A start that loses the placement re-hashes the copy that stands: an equal copy is reused, a different one gives `pinned-hash-mismatch` with no new line. This followed a measured `ETXTBSY` (4 of 24 boots). Rows: `decision/pinned-copy-never-replaced`, `decision/pinned-copy-race-adds-no-line`, `contract/run-span-attributes/run-pin-copy`. Why it matters: v2-03's "pinned copy run through a freshly written plugin by absolute path" happens at this step on a fresh test home.

@history_item n=29 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — the start gained a socket-directory step with no span that writes one line only on a refusal (`process-exit{exit_code:1, detail:"socket-dir-refused"}` before any snapshot, child or bind). Run's pre-spawn refusals gained it too. Row: `contract/process-exit-detail/exit-1`. Why it matters: it is part of the documented start sequence that the live wrapped start proves, with one more exit-1 cause to rule out.

@history_item n=30 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — `run`'s one pre-spawn terminal exception is a start refusal: two fixed stderr lines through `src/human.rs`. Otherwise run writes nothing to the terminal. Rows: `contract/run-terminal-output/start-refusal`, `contract/run-terminal-output/child-output`, `decision/run-silent-on-terminal`. Why it matters: v2-03 requires "no byte of viola's own on the rig's pty". The only allowed viola bytes are a refusal's fixed pair before spawn.

@history_item n=31 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — Viola became Linux only: openpty is the one `pty_backend`, the ConPTY sideload, Windows kill-fallback and `.cmd`/`.bat` refusal are retired, `os` is closed to `linux`, verify's relied count is all 24 rows, and the probe `--settings` lost its OS qualifiers. A closed schema word no build can emit leaves the schema with its emitter. Rows: `decision/linux-only-os-target`, `contract/run-span-attributes/pty-spawn`, `contract/process-start/os`. Why it matters: the live start is "on Linux", and its process-start and pty.spawn values are read against these.

@history_item n=32 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — portable-pty is pinned at `=0.9.0` (its `log` target OFF, failures captured at the seam), and `process-start` reads `pty_backend` `openpty` alone with no `sideload_fallback`. Rows: `decision/portable-pty-log-target-off`, `contract/run-process-log/process-start-claude-child`. Why it matters: these are the claude-child process-start fields the live start must show.

@history_item n=33 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — this added `hyprctl-probe` to the subject enum and `hyprctl-json` to the parse-rejected parsers. Changed rows: `~ decision/os-apis-at-call-sites` (the hyprctl spawn at call sites) and `~ decision/span-naming` (+ hyprland). Why it matters: these are picked rows. The round runs headless with no compositor step, so no hyprctl line should appear in the live evidence.

@history_item n=34 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — no process logs a liveness transition. `run` touches the heartbeat every 1 s, and liveness flips at more than 5 s, read through `list --json` / `agent-run status`. The process roles are `run|hook|mcp|cli`. Rows: `decision/liveness-transitions-not-logged`, `decision/heartbeat-file-not-logged`, `decision/heartbeat-staleness-flip`. Why it matters: the hour-long hold's liveness, and the stale-name refusal setup, are read from the heartbeat, not from log lines.

@history_item n=35 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** — `agent-run status` reads `state: degraded` when a booted instance's item is stale or absent. Row: `contract/agent-run-status/state-degraded`. Why it matters: during the near-ceiling hold, the rig reads instance health through this status.

@history_item n=36 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — a vt100 feed panic is caught on run's feed thread and gives one `parse-rejected{vt100-feed, panicked}`, and run continues. vt100 serves gate outcomes only. Rows: `decision/no-telemetry-from-screen`, `decision/manual-readiness-gate-span`. Why it matters: the live wrapped start runs the readiness gate on real `claude` screens, and no telemetry may come from screen content.

@history_item n=37 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** — `obs_event!` attaches `event`, `process` and `instance`, while `corr` is caller-supplied. The schema enforces default-deny with a top-level `unevaluatedProperties: false`. Row: `decision/default-deny-log-fields`. Why it matters: any field a live-round reading adds to a line fails G4, so the round must add no field.

@history_item n=38 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** — the secret scan's cookie class is retired, and the classes are now CLAUDE* values, the content canary, mode 0600 and an unreadable file. Changed rows: `~ decision/secret-scan-before-upload`, `~ decision/secrets-never-logged`, `~ decision/no-secrets-in-log-lines`, `~ decision/content-only-in-instance-detail-file`, `~ decision/claude-env-and-cookie-never-logged`. Why it matters: a live home holding real `claude` env is scanned for these classes, and these rows are picked.

@history_item n=39 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master's text became rows, each verified against the code at 8834195b0683. Why it matters: the picked rows are this conversion's output. A divergence "as the code shows" is read against the tree, not the old prose.
