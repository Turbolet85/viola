@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (verify's outcome on stdout, the uninstrumented `hook --capture` arm) set verify's agent-readable outcome as exit 0/1 plus one stdout step line per row and the last line `stamped <version>  N pass  N fail`, and made the hidden capture arm uninstrumented by design (founder ratification). The `viola verify --record` this chunk runs reads through exactly this outcome surface, and its captures come through that uninstrumented arm, so the recording adds no lines (decision/capture-arms-uninstrumented, decision/hidden-capture-arms-uninstrumented).

@history_item n=2 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — added `verify-probe` to the spawn subjects. It also fixed that CI runs verify only against the fake agent (then at 2.1.283) and that the real-`claude` verify runs only through the local `run --local-live`, which refuses under `CI`. This is the precedent behind decision/ci-verify-fake-agent-only and decision/real-cli-verify-never-in-ci: the clean 2.1.325 recording is a local live run, and CI only replays the new set through the fake agent.

@history_item n=3 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — added `strict-modes-failed` to `parse-rejected{parser:"ledger-stamps"}` (read as unverified, `cli_verified:false`). It also moved the version CI's `viola verify` runs at to the recorded 2.1.287 (was 2.1.283). That is the precedent for moving the fake agent's replay version when a new fixture set lands, and the ledger-stamps rows (contract/run-log-parse-rejected/ledger-stamps, decision/unreadable-ledger-is-unverified) govern how an older 22- or 23-row stamp reads unverified (C3, C8).

@history_item n=4 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — added `verify-pty-probe` and verify's interactive PTY runs, instrumented at the call site with no row text, prompt or path in any line, and set CI to drive the fake agent's interactive runs (`--screens --turn-stop --trusted-root`). This is the subject every live Run A–F of the recording logs under (contract/verify-spawn-subject/verify-pty-probe).

@history_item n=5 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — took verify to six spawn pairs (Runs A–D) and let the capture arm print one product-built decision body under `--answers` (contract/hook-stdout/capture-arms). CI added `--dialogs`. The fake-agent flag list is the replay surface the new 2.1.325 set must still satisfy.

@history_item n=6 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — CI's fake-agent verify flags gained `--framing`, with boot step 4 and the stamped-home fixture stamping through the framing replay, bound to test-plan §3's boot step 4. A new recorded set feeds that replay, so the flag list and the binding are what the drift suites read against.

@history_item n=7 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — an unconfirmable send (a listed command with no post-condition, or any command on an unverified CLI) opens no `run.confirm_window` and logs `send-issued` plus `send-confirmed{confirmed:false}`. This bears on C4: measuring `/context`'s post-condition live decides whether it stays unconfirmable or gets a confirmation shape, and on C3/C8, where an unverified stamp turns local commands unconfirmable.

@history_item n=8 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — added `process-exit` detail `interrupted` for a verify stopped by SIGINT/SIGTERM, with no stamp written and the probe dir removed on pass, fail or stop (SIGKILL or a crash leaves it). A live `--record` that is stopped writes no stamp and only the pairs it started (contract/verify-process-exit/interrupted, contract/verify-stop-stderr/unable-hint-pair).

@history_item n=9 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — set verify's stdout to one `[NN/MM]` step line per relied row, MM being the relied count, with the summary summing to it (was per ledger row). The 24-row re-stamp (C3, C8) reads by this relied-count rule (contract/verify-stdout/step-line).

@history_item n=10 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — added Run E, the `--resume <id>` start of Run B's first session id, with `resume-session` relied. It also measured that a PTY child killed at a stop carries `child_exit_status`. This is C10's subject: Run E spawned nothing on the browser-tools host because Run B left no resumable SessionStart, and a clean `--record` here should show whether Run E starts.

@history_item n=11 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added verify's eighth spawn pair, one `agents-probe` around Run B's single `claude agents --json` read (no row text, session id, `cwd` or path), relied `agents-join`, and revive's `session-live` reading with `parse-rejected{claude-agents-json}` on oversize or malformed input. The recording captures that agents read, and its rows are untrusted and never logged verbatim (contract/verify-spawn-subject/agents-probe, decision/agents-probe-output-untrusted).

@history_item n=12 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (the mcp list tool's lines) gave `viola mcp`'s `list` tool an `agents-probe` pair into `mcp.ndjson`, with codes only and no row text. C1 re-records `Mcp.tools-list-1.json` through `viola-mcp-tap` against this tool (contract/viola-mcp-tools/list), and the MCP stdout carries frames only.

@history_item n=13 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — added `local-command-compact` and the 40 s compact window (decision/compact-confirm-window, PROVISIONAL), with no new spawn pair. This is the earlier precedent of a new relied row forcing a re-stamp. The live recording on 2.1.325 is also where that window's post-condition is read again.

@history_item n=14 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (`viola verify`'s home-tree check) made verify check the home tree before its log opens, so a refusal is exit 1 with the home pair and no line. `update_stamps` runs `check_stamps` before and under its lock, and a refused stamp write exits 1 through verify's own `process-exit`. The live `--record`'s record home must pass this check, and the record-home rule (C6) must not trip it (contract/viola-verify-refusal/*, decision/refused-home-tree-writes-no-line).

@history_item n=15 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (`verify` writes its refusal's code) made verify write its refusal code on `process-exit` under a parsed `VIOLA_NAME` (`program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`, `interrupted`). A failing row carries no code, and the fixture's `<file>` enters no code. `unclean-fixture` is the refusal the recorder's scrub raises: a payload holding `/home/` is refused, never hand-scrubbed (contract/process-exit-detail/verify-refusal).

@history_item n=16 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the Windows host arms … retired) set verify's relied count to all 24 rows of `ALL`, the stop to SIGINT/SIGTERM, and the env strip to exact `CLAUDE*` names with the persistent set from `claude_env_keep`. This sets the 24-row Linux re-stamp this chunk owes, and the strip that keeps `CLAUDE_CONFIG_DIR` for the rig's private config (decision/claude-env-strip).

@history_item n=17 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — corrected E2's floor sentence: the strip always removes the 11 `IDENTITY_FLOOR` names, and a differing Linux dev-host reading on 2.1.287 stands beside it. Logged fields stay names only (`env_stripped_count`, `env_stripped_known`, `env_kept`). A live 2.1.325 recording may give a new floor reading, and no value may reach a fixture or a line (contract/claude-child-process-start/env-*).

@history_item n=18 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** — retired the secret scan's Cookie class and dropped the Cookie clause from decision/secrets-never-logged, decision/no-secrets-in-log-lines, decision/content-only-in-instance-detail-file and decision/claude-env-and-cookie-never-logged, leaving `CLAUDE*` values, the canary, 0600 and unreadable files as scan classes. These are the secret floors the recorder hygiene and the recorded fixtures must hold to.

@history_item n=19 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — removed the statusline capture arm (and Run B's status-line probe), leaving one hidden `hook <event> --capture` arm. It also dropped "a status line payload" from decision/verify-lines-carry-no-content. A 2.1.325 recording therefore writes no `StatusLine.*` captures, and the rows decision/hidden-capture-arms-uninstrumented, decision/hook-capture-arms-unlogged, decision/capture-arms-uninstrumented and contract/hook-stdout/capture-arms name the single arm.

@history_item n=20 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (1. Obs Scope Summary: 6 rows) retired exit 11 and set the run-start events order to `wheel{start}` then `session-start` (decision/exit-code-log-field, contract/run-start-events/order). A recording on 2.1.325 must show no `budget-gate` record and no `budget_written` field.

@history_item n=21 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — the live round on 2.1.325 measured folder-trust keys writing `wheel{human-input}` before `session-start`, and a reused instance name keeping earlier starts' lines. This amended contract/run-start-events/order, decision/product-order-from-events-log and decision/events-stream-never-rotated. It is the source of C1's carries (`inputs/I37`, `I7`, `I13`, `I14`, `I23`, `I53`), and a recorded start must be read from its own lines.

@history_item n=22 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — took the `hook_event` enum to ten with `stop-failure`, registered with no ledger row (the P4 answer to F1). This is the PROVISIONAL exception C2 owns: StopFailure stands with no ledger row and no verify probe.

@history_item n=23 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — (the StopFailure no-ledger-row answer ratified by the founder) ratified that StopFailure stands registered with no ledger row and no verify probe, and named «The 2.1.325 recorded fixture set» as the owner of its live reading. This is C2's direct mandate: record a live StopFailure payload and settle the exception (a row with a probe, or kept on the founder's word).

@history_item n=24 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** — added contract/parse-rejected-detail/unknown-value and contract/parse-rejected/hook-stdin-unknown-value (one WARN line for a StopFailure error word outside the CLI's 13, still delivered as `api-error`). It also added that exception to decision/parse-failures-unknown-counter and decision/parse-failures-surface-as-unknown. C2 checks the live payload's `error` word against this: one of the 13 writes no `unknown-value` line.

@history_item n=25 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — added `hyprctl-probe` and `hyprctl-json` to the subject and parser enums, and added the hyprctl exception to decision/parse-failures-unknown-counter, decision/parse-failures-surface-as-unknown and decision/per-parser-parse-failure-counts. These are the first exceptions on the parse-failure rows C2's StopFailure reading touches, and the closed subject and parser lists a recorded set must not widen (contract/diag-line-fields/process-start, contract/diag-line-fields/parse-rejected).

@history_item n=26 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — added the transcript stdout-tag echo confirmation as an exception to decision/claude-internals-not-a-contract, decision/transcripts-not-a-contract, decision/deterministic-event-emission and decision/deterministic-emission. It also added the echo read's no-line refusal to decision/refusal-telemetry, updated contract/send-path-log-events/event-order, and added decision/transcript-echo-read-uninstrumented. This is the chunk behind C3 (the 22nd relied row `local-command-echo` making older stamps unverified) and C4 (`/context` never sent live).

@history_item n=27 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** — ratified on the founder's own word the delegate's confirm of the 2026-11-16-local-command-delivery rows (the claude-internals and refusal-telemetry exceptions, event-order, the deterministic-emission exceptions, transcripts-not-a-contract). The echo-confirmation rows C3 and C4 rest on are founder-ratified, not provisional.

@history_item n=28 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — added Run F, the trust-record run, so verify has six interactive runs and nine spawn pairs (contract/verify-spawn-subject/verify-pty-probe, contract/diag-subject/verify-pty-probe, contract/verify-process-exit/child-exit-status-and-duration). It added verify's trust-record write and removal to the claude-internals and refusal-telemetry exceptions, verify's read of `CLAUDE_CONFIG_DIR` (decision/r8-strip-logged-as-names-only, decision/env-vars-not-configuration), and decision/trust-record-write-uninstrumented. C8's trust-record row read is under these rows, and the delegate marks are PROVISIONAL.

@history_item n=29 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — extended `CLAUDE_CONFIG_DIR` reading to the run and revive start, added the start's one trust-record line to decision/claude-boundary-only-instrumentation, widened the claude-internals and refusal-telemetry exceptions to the start's flag-only write, and added contract/start-trust-log/trust-record and contract/diag-line-fields/trust-record (16 start words). It also amended decision/trust-record-write-uninstrumented. The delegate's «Confirm all 26» is PROVISIONAL until the founder's word, and the live starts of the recording write these lines.

@history_item n=30 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — added `absent` to the step line, an absent browser-tools-prompt row counting toward pass and exit 0 (contract/verify-stdout/step-line, contract/verify-stdout/summary-line, contract/verify-exit-codes/0, delegate «Confirm all 26», PROVISIONAL). It also added decision/browser-tools-prompt-reading-uninstrumented. This is the 24th relied row of C8 and the subject of C9 (8/24 on an unanswered host) and C11 (the `bt-absent` control, `verified_reads_the_browser_tools_row_absent_as_counting`).
