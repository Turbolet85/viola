@history_item n=1 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — (1. Obs Scope Summary: 2 rows) added the excepts to `~ decision/claude-internals-not-a-contract` and `~ decision/refusal-telemetry`. A local command's stdout tag in the session transcript, counted past the end recorded before the paste, is that send's echo confirmation, and no transcript byte is kept. The echo read refuses a symlinked, non-regular or foreign transcript and writes no line. This is the exact mechanism `/context` joins (`CommandStdout`, as `/mcp`). Adding `/context` must stay inside this exception: no transcript byte kept and no new refusal line.

@history_item n=2 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — (11. Obs Anti-Patterns: 1 row) gave `~ decision/transcripts-not-a-contract` its one except: the stdout tag count is read as the echo confirmation. A second echo-confirmed command (`/context`) widens what the except covers in use but not what it says. The except is worded per local command, so the row needs no edit unless the tag shape differs.

@history_item n=3 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — (1. Obs Scope Summary: 2 rows) `~ contract/send-path-log-events/event-order`: no hook-invoked is awaited for a local command confirmed by its transcript echo. `~ decision/deterministic-event-emission` except: the echo confirmation keys on the stdout tag count. A `/context` send must log channel-request › send-issued › send-confirmed|send-refused › channel-response, with no hook line awaited.

@history_item n=4 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — (4. Span / Trace Coverage: 3 rows) `~ contract/send-log-fields/hook-invoked`: an echo-confirmed local command waits for no hook line. `~ contract/send-refused-detail/no-prompt-submitted`: window expiry for an echo-confirmed local command is `no-prompt-submitted`, with no new detail. `+ decision/transcript-echo-read-uninstrumented`: the echo read opens no span and writes no line or field of its own, and its result shows only as send-confirmed inside `run.confirm_window`. This is the precedent that a new echo-confirmed command adds no event, detail, span or field.

@history_item n=5 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — (10. SLO Invariants: 1 row) `~ decision/deterministic-emission` except: alongside the readiness gate's vt100 read, a local command's echo confirmation keys on its transcript's stdout tag count. `/context` rests on this same exception.

@history_item n=6 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** — the founder ratified the delegate's PROVISIONAL confirm of every local-command-delivery row: the claude-internals and refusal-telemetry excepts, event-order, deterministic-event-emission, deterministic-emission and transcripts-not-a-contract. The echo-confirmation rows this chunk extends stand on his own word. Widening them past a per-command echo would need a fresh word.

@history_item n=7 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — set the confirm window's per-send close and the unconfirmable send. A listed command with no post-condition, or any listed command on an unverified CLI, opens no `run.confirm_window`. Its chain ends at `pty.paste_write`, it logs one send-issued and one send-confirmed{confirmed:false} under the same corr, with no send-refused and no hook. `viola.send.confirm_ms` counts only confirmed lines. This is the `unconfirmable` branch the scope names for `/context` on a version where the post-condition is unmeasured, or a binary that reads the stamp unverified.

@history_item n=8 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — the precedent for a local-command post-condition ledger row. `/compact` gained a close condition with no new event, detail or field. verify's MM rose by one (the 24th row, `local-command-compact`). Run B's extra paste added no spawn pair or subject. `window_ms` reads `SendSlot::window()`: 40 000 for compact, else 10 000. A `/context` row should follow this shape: a Run B paste with no new spawn pair, 10 s window, MM +1.

@history_item n=9 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — the fake-agent verify flag list gained `--framing` (bound to test-plan §3 boot step 4). The six spawn pairs did not change, and no row text, prompt or path reaches a line. This is the precedent for the fake agent's rehearsal of a new local-command paste: a flag or rehearsal change moves the bound CI verify invocation, not the obs line set.

@history_item n=10 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — in CI, verify runs only against the fake agent (`tests/cli_verify.rs`, harness boot step 4, `stamped_home`). The real-`claude` verify runs only through the local `run --local-live`, which refuses under CI. Those rows are `decision/ci-verify-fake-agent-only` and `decision/real-cli-verify-never-in-ci`. The `/context` probe must be rehearsed by the fake agent in CI, and its live stamp on 2.1.328 stays an operator leg.

@history_item n=11 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the Windows host arms retired) verify's relied count reads all 24 rows of `ALL`: there is no per-OS MM split and `WRAPPED_HERE` is folded to true. A new RELIED `/context` row raises the one MM for every build, with no OS variant to state.

@history_item n=12 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — `~ contract/verify-stdout/step-line`, `~ contract/verify-stdout/summary-line` and `~ contract/verify-exit-codes/0`: a step verdict is pass|absent|fail, and absent counts toward pass only for the browser-tools-prompt row. A new `/context` row's step line must be pass|fail (absent is not its word), and the summary's counts must still sum to MM. `+ decision/browser-tools-prompt-reading-uninstrumented` is the precedent for a verify-time reading that adds no telemetry.

@history_item n=13 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** — `~ decision/browser-tools-prompt-reading-uninstrumented`: verify's one fixed stdout line on a met prompt is that prompt's only record. This is the latest re-stamp chunk, the one the scope cites for the RELIED or re-stamp question. Its rows show that a stamping change reaches only stamps and stdout, never new log lines.

@history_item n=14 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — (4. Span / Trace Coverage, 6. Log Coverage) verify's interactive PTY runs went from five to six (Run A–F) and its spawn pairs from eight to nine. This moved `contract/verify-spawn-subject/verify-pty-probe`, `contract/diag-subject/verify-pty-probe` and `contract/verify-process-exit/child-exit-status-and-duration`. These are the counts a `/context` probe would move if it took a new interactive run instead of a paste inside an existing run (the compact precedent).

@history_item n=15 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — (1. Obs Scope Summary) the claude-internals and refusal-telemetry excepts gained verify's trust-record write. `+ decision/trust-record-write-uninstrumented`: verify's trust-record write and removal emit no line and no span, and the outcome rides the stamps. A `/context` probe in an interactive verify run inherits this trusted-temp-folder path and must not add instrumentation to it.

@history_item n=16 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — gave `decision/claude-internals-not-a-contract` and `decision/refusal-telemetry` their current excepts (echo tag count, verify trust record, a verified start's trust flag only). It widened `decision/claude-boundary-only-instrumentation` to the start's one trust-record line, and `~ decision/trust-record-write-uninstrumented`. These are the exact texts the picked rows hold. Any `/context` wording change must edit this current form, which stands PROVISIONAL on the delegate's «Confirm all 26».

@history_item n=17 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — `parse-rejected{parser:"ledger-stamps"}` gained `strict-modes-failed` (run's version gate refused the stamps file, read as unverified, `cli_verified:false`, at warn, with no path). This is the unverified-ledger path a RELIED `/context` row triggers: every binary built after it reads earlier stamps unverified until re-stamped, and the sends degrade to unconfirmable.

@history_item n=18 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (`verify` writes its refusal's code) a refused `update_stamps` leaves verify's `process-exit` exit 1 `detail:"strict-modes-failed"`, and a failing row carries no code. These are the rows `contract/process-exit-detail/strict-modes-failed`, `contract/process-exit-detail/verify-refusal` and `decision/strict-modes-failure-lines`. The live stamp of the `/context` row writes through this stamps path: a failing `/context` row is no refusal and writes no detail code.

@history_item n=19 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — `text_bytes` is the typed text's length: CR LF and lone CR each count as one LF, and trailing CR/LF are removed. This holds on `pty.paste_write` and send-issued (`contract/send-span-attributes/text-bytes`). A `/context` send logs its typed length by this rule and never its text.

@history_item n=20 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `send-confirmed.duration_ms` runs from the send's arrival and includes the readiness gate's wait (up to `GATE_MAX_WAIT`). This is the row `contract/send-log-fields/send-confirmed`. An echo-confirmed `/context` send's duration reads the same way, which matters if the readiness-gate reading (cry-kutofxml) rides this entry's rig start.

@history_item n=21 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — `~ contract/confirm-window-close/ordinary-send` (prompt equality with a trailing-whitespace tolerance) and `~ contract/send-refused-detail/turn-running`: the turn read, link record, send-issued and paste run under one flight lock, so a turn the wrapper knows of before the paste refuses turn-running. The ordinary-send close does not apply to `/context`, which closes on its tag count. The flight-lock bound does apply to it.

@history_item n=22 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the founder ratified the trailing-only tolerance of `contract/confirm-window-close/ordinary-send`, and a leading trim waits on the live rig. This is a trap: do not route `/context` through the prompt-equality close.

@history_item n=23 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — a vt100 panic is caught on run's feed thread and gives one `parse-rejected{vt100-feed}` and an input-not-ready refusal. The gate reads screen state only for its verdict (`decision/no-telemetry-from-screen`, `contract/send-span-attributes/run-readiness-gate`). The folded carry cry-kutofxml reads the placeholder hint's cell attributes at this entry's rig start. That reading must stay attribute-only and add no telemetry derived from screen content.
