@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (verify's outcome on stdout, the uninstrumented `hook --capture` arm) set verify's agent-readable outcome as exit 0/1 plus one stdout step line per row and the last line `stamped <version>  N pass  N fail`, refusals the stderr `unable:`/`hint:` pair; made the hidden capture arm uninstrumented by founder ratification. R4's live `viola verify --record` reading is read through exactly these step/summary lines (contract/verify-stdout/step-line, summary-line) and must not expect hook lines from the capture arm (decision/hidden-capture-arms-uninstrumented).

@history_item n=2 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — added `verify-probe` to the spawn subjects and fixed that in CI verify runs only against the fake agent, the real-`claude` verify only through the local `run --local-live`, refused under `CI`. R4 is exactly that real-CLI verify: it must run on the local rig, never as a CI step (decision/real-cli-verify-never-in-ci, decision/ci-verify-fake-agent-only).

@history_item n=3 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — added `verify-pty-probe` and verify's interactive PTY runs logged as spawn pairs with `child_exit_status` and `duration_ms`, no row text, prompt or path in any line. A live R4 run's `cli-<name>.ndjson` is read against these subjects (contract/verify-spawn-subject/verify-pty-probe, contract/diag-subject/verify-pty-probe).

@history_item n=4 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — verify's seven spawn pairs (Runs A–E) and the measured rule that an interactive PTY child killed at a Ctrl-C/TERM stop still carries `child_exit_status`; print-mode/`--version` children killed at a stop not measured. Bears on R4 (a live verify stopped mid-run) and on contract/verify-process-exit/child-exit-status-and-duration; any new reading of a killed print probe would be the first measurement.

@history_item n=5 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — verify's eighth spawn pair (`agents-probe` around Run B's single agents read) and the `claude-agents-json` parse-rejected `oversize` detail. R4's live verify writes eight pairs; an agents read on 2.1.326 that fails its parse surfaces as `unknown` plus a counter (decision/agents-probe-parse-failure-unknown, decision/agents-probe-output-untrusted, contract/verify-spawn-subject/agents-probe).

@history_item n=6 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — gave a stopped verify `process-exit{exit_code:1, detail:"interrupted"}` after every opened spawn pair is closed, a stopped run starting no further spawn, the probe dir removed on pass/fail/stop but left by SIGKILL or a crash. The rig's stop paths for R4 (and the nextest kill line's terminate) meet this contract (contract/process-exit-detail/interrupted); a SIGKILL leaves the probe dir, a trap for the rig's cleanup.

@history_item n=7 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — verify's stdout prints one `[NN/MM]` step line per relied row with MM the relied count (not per ledger row). R4's live record reading counts against MM, not the ledger's full row set (contract/verify-stdout/step-line).

@history_item n=8 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — the 24th relied row `local-command-compact` and Run B's sixth paste with no new spawn pair; `MM` 24. Sets the count a live R4 summary must sum to (contract/verify-stdout/summary-line).

@history_item n=9 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the Windows host arms of the panic capture, the env strip, verify and the statusline retired) verify's relied count reads all 24 rows of `ALL`, a verify stop reads `SIGINT`/`SIGTERM`, SIGKILL and a crash leave the probe dir; the strip matches `CLAUDE*` exactly. Current MM and stop semantics for R4 on this Linux-only host; the strip rule binds the rig's private config/credentials handling (decision/r8-strip-logged-as-names-only, decision/stripped-env-never-logged).

@history_item n=10 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (`viola verify`'s home-tree check and the catch site's narrowed reach) verify checks the home tree before its `cli` log opens, a refusal exit 1 with the home pair and no line; `update_stamps` runs `check_stamps` before writing, mapping a refusal to the home pair through its own `process-exit`. R4's record home must pass this check (owner-only modes) or the live record reads a no-line refusal (contract/viola-verify-refusal/home-tree-refused, decision/refused-home-tree-writes-no-line). The same chunk put the hook's instance check inside the timed path (decision/hook-latency-budget).

@history_item n=11 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (`verify` writes its refusal's code) verify writes `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture` or `interrupted` on its `process-exit` under a parsed `VIOLA_NAME`; a refused `update_stamps` reads `strict-modes-failed`; a failing row carries no code; verify takes no `--json`. A live R4 on an unstamped 2.1.326 or an unreadable `--version` lands on these codes (contract/process-exit-detail/verify-refusal, contract/verify-process-exit/strict-modes-failed, decision/strict-modes-failure-lines).

@history_item n=12 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — added `parse-rejected{parser:"ledger-stamps", detail:"strict-modes-failed"}` on `run`'s version gate, read as unverified (`cli_verified:false`). A start on PATH `claude` 2.1.326, which no stamp covers, reads `cli_verified:false` on `process-start{claude-child}` (decision/unreadable-ledger-is-unverified, contract/run-log/cli-verified, contract/diag-detail/strict-modes-failed): the scope's count-apart of 2.1.326 readings rests on it.

@history_item n=13 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — CI's fake-agent verify flag list gains `--framing`, bound to test-plan §3's boot step 4. The nextest kill-line carry concerns binaries (`harness_lifecycle`, `cli`, `harness_tied`) that drive this stamping verify through `boot`; the flag list and six/eight spawn pairs must stay as they are when the kill override changes.

@history_item n=14 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** — `turn-running` covers a running turn (in-memory wrapper state, marked by a `prompt-submitted` of any origin) or another send in flight, under one closed detail with no new field and no line of its own. R3c measures the window in which a CLI-started turn (including a fired wake) is not yet marked; any finding must stay inside this no-new-field shape (contract/send-log-line/detail).

@history_item n=15 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — bounded `contract/send-refused-detail/turn-running` to a turn the wrapper knows of before the paste, and recorded that a turn whose `UserPromptSubmit` arrives after the paste stays undetected (fct-izz7vw4n), with the flight lock covering only known turns. This is the exact residual R3c measures; also set the ordinary-send confirm-window close (contract/confirm-window-close/ordinary-send).

@history_item n=16 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — (the send trailing-only tolerance ratified) the founder ratified the trailing-only whitespace tolerance; a leading trim waits for the live reading carried on «Standing live rig with a screen model». A live reading taken on this rig may be the one that carry names. The same wrap ratified StopFailure with no ledger row, its live reading owned by «The 2.1.325 recorded fixture set», which this chunk must not re-record.

@history_item n=17 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `send-confirmed.duration_ms` runs from the send's arrival at the wrapper and includes the readiness gate's wait (up to `GATE_MAX_WAIT`), so it is no `open`→`read back` interval. A hook-latency window read for R3c from send lines must not take `duration_ms` as the paste-to-hook interval (contract/process-log-line/duration-ms, contract/send-path-log-events/event-order).

@history_item n=18 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — added the one named exception to «claude internals / transcripts are not a contract»: a local command's stdout tag in the session transcript, counted, no byte kept; `decision/deterministic-event-emission` gained the same exception and the send event order its echo-confirmed case. R3a reads a fired wake's raw prompt-submit payload: any reading beyond hook stdin must stay inside boundary-only instrumentation, with no transcript byte kept (decision/claude-internals-not-a-contract, decision/transcripts-not-a-contract).

@history_item n=19 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** — the founder ratified the local-command-delivery rows (claude-internals except, deterministic-event-emission except, the event order, transcripts-not-a-contract except, deterministic-emission except). These rows stand on his own word, so R3a/R3c readings may not widen them without a new ruling.

@history_item n=20 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — a live round on 2.1.325 measured folder-trust keys writing `wheel{human-input}` before `session-start`; the start order is read from that start's own lines, since a reused instance name keeps earlier starts' lines at the head of `events.ndjson`. The precedent for writing live readings as rows: R3b's `turn-ended` and R3a's wake reading on a reused rig instance name must be read from the start's own lines (decision/events-stream-never-rotated).

@history_item n=21 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — a differing Linux dev-host live reading on `claude` 2.1.287 was set beside the origin and moved nothing in the logged fields (`env_stripped_count`, `env_stripped_known`, `env_kept`, names only). Precedent: a live reading on a new CLI version (2.1.326) is recorded as a reading, not a field change (decision/r8-strip-logged-as-names-only).

@history_item n=22 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** — added `parse-rejected{parser:"hook-stdin", detail:"unknown-value"}` and the StopFailure exception to `decision/parse-failures-unknown-counter` / `decision/parse-failures-surface-as-unknown`. R3a's `source` field on a wake payload is today dropped by the hook; if a later chunk reads it, an outside value takes this closed-word pattern, and the reading here adds no parser or line.

@history_item n=23 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (1. Obs Scope Summary: 3 row(s)) added the hyprctl replies to `decision/per-parser-parse-failure-counts` and the hyprctl exception to the parse-failures-as-unknown rows. These are the current wording of the picked parse-failure rows that any rig-read payload parse rests on.

@history_item n=24 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — the `hook_event` enum went to ten closed names with `stop-failure` (contract/diag-line-fields/hook-events), no ledger row. Hook lines read live on the rig (R3a, R3c) carry one of these ten; no new hook name is admitted here.

@history_item n=25 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — removed the statusline arm: one hidden `--capture` arm left (decision/hidden-capture-arms-uninstrumented, decision/hook-capture-arms-unlogged, contract/hook-stdout/capture-arms), `decision/hook-latency-budget`'s statusline except dropped, `run` no longer passes `--settings` (decision/child-settings-never-logged keeps it out of logs). The live rig's sessions and R4's probes run against this trimmed hook surface.

@history_item n=26 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (1. Obs Scope Summary) retired exit 11 (decision/exit-code-log-field), dropped `budget-gate` from the start order and narrowed `decision/log-fields-additive-only` to fields the product still emits. A live reading must not expect a budget record or field; any field a reading adds goes through the additive-only rule.

@history_item n=27 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — set where a vt100 panic is caught on `run`'s feed thread and what it writes (one `parse-rejected{vt100-feed}`, model poisoned until resize). The R2 `box-text: holds-text` and external-imports modal readings are screen-model readings of this gate: they inform gate outcomes only, never telemetry (decision/no-telemetry-from-screen, contract/send-span-attributes/run-readiness-gate, decision/manual-readiness-gate-span).
