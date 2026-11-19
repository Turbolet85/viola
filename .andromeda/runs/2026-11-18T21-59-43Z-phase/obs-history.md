@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (verify's outcome on stdout, the uninstrumented `hook --capture` arm) — set verify's agent-readable outcome as exit 0/1, one stdout step line per row and the last stdout line `stamped <version>  N pass  N fail`; refusals are the stderr `unable:` + `hint:` pair, a fault exactly `error: internal error`; the capture arm is uninstrumented by design. Why it matters: the new browser-tools-prompt line must land on a named stream without breaking this stdout/stderr split (contract/verify-stdout/step-line, summary-line; decision/hidden-capture-arms-uninstrumented).

@history_item n=2 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (panic backtrace as raw, never-symbolised frames) — panic detail lines carry up to 62 raw frames resolved offline against the pinned copy, never symbolised (the symbolising cost broke the 1.0 s spine bound). Why it matters: the re-pin replaces the pinned copy; offline resolution of any panic frame depends on keeping the pinned binary (and the rollback copy) that wrote it (decision/raw-unsymbolised-backtrace).

@history_item n=3 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — verify logs each spawn as a process-start/process-exit pair at the call site (`verify-probe` added to the subject enum by a Decisions Log entry); in CI verify runs only against the fake agent, and the real-`claude` verify runs only through the local live run, refused under `CI`. Why it matters: the live 24-row re-stamp is the local-only real-CLI verify; the new prompt line is proved on the fake agent only (decision/ci-verify-fake-agent-only, decision/real-cli-verify-never-in-ci, contract/verify-spawn-subject/*).

@history_item n=4 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — `parse-rejected{parser:"ledger-stamps"}` gained `strict-modes-failed` beside `unreadable`/`malformed`, read as unverified (`cli_verified:false`) at warn with no path. Why it matters: the rebuilt product's version gate reads the live home's stamps; an unreadable or refused stamps file reads unverified with this one line (contract/run-log-parse-rejected/ledger-stamps, decision/unreadable-ledger-means-unverified).

@history_item n=5 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — verify's interactive PTY runs got the `verify-pty-probe` subject, one pair per run, with no row text, prompt or path in any line. Why it matters: the live verify's per-run pairs, and a prompt line that must not reach the process log (contract/diag-subject/verify-pty-probe, decision/verify-lines-carry-no-content).

@history_item n=6 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — CI's fake-agent verify flag list gained `--framing`, bound to test-plan §3's boot step 4. Why it matters: precedent: a new fake-agent option driving verify (here `--start-screen` for the prompt line) moves the obs CI flag-list bullet with its test-plan twin.

@history_item n=7 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — an unconfirmable send (a listed command with no post-condition, or any listed command on an unverified CLI) opens no `run.confirm_window` and logs one `send-issued` plus `send-confirmed{confirmed:false}`. Why it matters: stamps a tree-built binary reads unverified make `/clear`, `/compact`, `/mcp` transport-only; `/context`, being no ledger-listed command, is an ordinary send whose post-condition is measured live (C6) (contract/send-line/unconfirmable).

@history_item n=8 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — `process-exit` detail `interrupted` for a verify stopped by SIGINT/SIGTERM: no further spawn, every opened pair closed, exit 1, no stamp; the probe dir removed on pass, fail and stop, while SIGKILL or a crash leaves it. Why it matters: a live verify stopped mid-run writes no stamp and may leave run folders behind (contract/verify-process-exit/interrupted, contract/process-exit-detail/interrupted).

@history_item n=9 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — verify's stdout became one `[NN/MM]` step line per relied row, MM being the relied count, and the summary sums to it. Why it matters: the 24-row live stamp's step and summary lines (contract/verify-stdout/step-line).

@history_item n=10 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — Run E added (seven spawns); a child of an interactive PTY run killed at a Ctrl-C/TERM stop carries `child_exit_status` and `duration_ms`, as measured. Why it matters: it is the basis of the nine-pair exit-status promise the live verify reads (contract/verify-process-exit/child-exit-status-and-duration).

@history_item n=11 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — verify's eighth spawn pair is one `agents-probe` around Run B's single agents read, with no row text, session id, cwd or path in a line. Why it matters: it is part of the live verify's spawn census (contract/verify-spawn-subject/agents-probe, contract/diag-subject/agents-probe).

@history_item n=12 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — `/compact`'s 40 s `COMPACT_CONFIRM_WINDOW` (PROVISIONAL) beside the 10 s window, with `window_ms` reading it; Run B's pastes add no spawn pair or subject. Why it matters: the live run's confirm windows; `/context` as an ordinary send takes the 10 s window (decision/compact-confirm-window, contract/send-span-attributes/window-ms, contract/confirm-window-close/compact).

@history_item n=13 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (verify's home-tree check; `update_stamps` strict-modes) — verify checks the home, `ledger/`, `bin/`, `plugin/` before its log opens, and a refusal writes no line; `update_stamps` runs `check_stamps` before and under its lock, a refusal exiting 1 through verify's own `process-exit`. Why it matters: the founder's real `~/.viola` must pass these checks for the live re-stamp to write (contract/viola-verify-refusal/home-tree-refused, contract/viola-verify-refusal/stamp-write-refused).

@history_item n=14 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (home-tree check of `run` and revive's start, before the wrapper log) — `checked_exe` plus `check_home_tree` over the home, `bin/<key>/` and the pinned copy run before the log; a refusal opens no log and writes no line. Why it matters: the rebuilt, re-pinned product's first start meets this check on the new `bin/<key>/` (decision/home-tree-check-before-start-log, decision/home-tree-check-before-log-init).

@history_item n=15 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the refusal-telemetry record) — strict-modes failures have lines only at the `ledger-stamps` read and verify's refused stamp write; every other refused check writes none. Why it matters: it bounds what the live verify and the re-pin may log on a refusal (decision/strict-modes-failure-lines, decision/strict-modes-refusal-writes-no-line).

@history_item n=16 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — an absent pinned copy is placed no-clobber, never replaced; a start losing the placement re-hashes the copy that stands (equal reused, different `pinned-hash-mismatch`, the file left as found), adding no line, field or detail. Why it matters: the re-pin and the live verify each pin their binary under `~/.viola/bin/<key>/`, and a stale copy at the same key refuses rather than being overwritten (decision/pinned-copy-never-replaced, decision/pinned-copy-race-adds-no-line, contract/run-span-attributes/run-pin-copy).

@history_item n=17 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`verify` writes its refusal's code) — under a parsed `VIOLA_NAME` verify's `process-exit` carries `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture` or `interrupted`; a failing row carries no code; verify takes no `--json`. Why it matters: the browser-tools-prompt line must add no refusal code (absent is not a refusal), and a RED live verify reads through these codes (contract/process-exit-detail/verify-refusal).

@history_item n=18 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the `human::refuse` site counts) — the census reads `run` 7 and `verify` 5, re-derived by `grep -rcE 'human::refuse\(' src`; the print-macro ban's output-module exception holds. Why it matters: a new fixed-words verify line written through the human writer moves this census, and it must stay a fixed string with no print macro outside the output modules (decision/no-print-macros, decision/fixed-stderr-messages).

@history_item n=19 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (Windows host arms retired) — the relied count reads all 24 rows of `ALL`; a verify stop is SIGINT/SIGTERM; SIGKILL and a crash leave the probe dir. Why it matters: the live home's 24-row stamp is counted against this one relied set.

@history_item n=20 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (diag-schema additive rule narrowed) — the operator's Q2 narrowed the schema rule: fields may be added through a Decisions Log entry, and only a field the product still emits is never renamed or removed. Why it matters: a new event or field for the prompt line would be an additive schema change (decision/diag-schema-additive-only, decision/exit-code-log-field).

@history_item n=21 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (verify lines and capture arm) — a verify log line carries no row text, prompt, session id, probe command or path; one hidden capture arm remains, uninstrumented. Why it matters: the prompt line must not carry prompt text into any log (decision/verify-lines-carry-no-content, decision/hook-capture-arms-unlogged).

@history_item n=22 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — the live round on 2.1.325 measured folder-trust keys writing `wheel{human-input}` before `session-start`, and a reused instance name keeps earlier starts' lines at the head of `events.ndjson`. Why it matters: it is the precedent for a live round under real config; read a start's own lines from where that start begins (decision/events-stream-never-rotated).

@history_item n=23 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — an ordinary send's window closes on the `prompt-submitted{origin:"driver"}` whose prompt equals the typed text, with trailing whitespace tolerated. Why it matters: `/context`, being no ledger-listed local command, confirms as an ordinary send under this rule, so its live post-condition is read against it (contract/confirm-window-close/ordinary-send).

@history_item n=24 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the founder ratified the trailing-only tolerance; a leading trim waits for a live reading. Why it matters: the `/context` live send stands on the ratified ordinary-send close.

@history_item n=25 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — a local command's echo confirmation counts the stdout tag in the session transcript past the paste-time end, keeping no byte; the echo read refuses symlinked or foreign transcripts with no line; `decision/transcript-echo-read-uninstrumented` was added. Why it matters: `/mcp disable viola-probe-absent` is the `local-command-echo` row (Run B's seventh paste) whose new row makes every older stamp unverified, so the live re-stamp is owed (decision/transcripts-not-a-contract, decision/deterministic-event-emission, contract/send-refused-detail/no-prompt-submitted).

@history_item n=26 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** — the founder ratified the local-command-delivery rows (transcript echo exceptions, event order, deterministic emission). Why it matters: the echo-confirm rows this chunk's live verify reads now stand as ratified.

@history_item n=27 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — verify gained Run F: six PTY runs and nine spawn pairs. Verify writes and removes its temp folder's trust record in the claude config, refusing link, non-regular or foreign files with no line and with the outcome on the stamps only. It reads the kept `CLAUDE_CONFIG_DIR` value and never logs it. `decision/trust-record-write-uninstrumented` was added. Why it matters: the live verify writes to the founder's real Claude config through this path, and no config path, key or value may reach a line, stamp or summary.

@history_item n=28 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — a verified run/revive start sets its folder's trust flag in the claude config and writes one `trust-record` line; `CLAUDE_CONFIG_DIR` is read by the start too. These rows are PROVISIONAL (the delegate's «Confirm all 26»). Why it matters: the re-pinned product's starts under the real config write this flag; key-name-only reads of `~/.claude.json` must account for it (decision/claude-boundary-only-instrumentation, decision/claude-internals-not-a-contract, decision/env-vars-not-configuration).

@history_item n=29 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (step, summary and exit 0 for an absent row) — the step line's verdict is `pass|absent|fail`, absent being the browser-tools-prompt row's word when no run met the prompt; the summary's pass count includes an absent row; exit 0 when every row counted. These are PROVISIONAL (delegate). Why it matters: these are the live 24-row verify's passing shapes (contract/verify-stdout/*, contract/verify-exit-codes/0).

@history_item n=30 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (`decision/browser-tools-prompt-reading-uninstrumented` added) — reading the prompt as a modal adds no log line, span, event or detail code; verify's reading of it reaches only the ledger stamp and verify's stdout. Why it matters (trap): this row bounds verify's new prompt line. On stdout it fits as is; on stderr or in the process log it amends this row.
