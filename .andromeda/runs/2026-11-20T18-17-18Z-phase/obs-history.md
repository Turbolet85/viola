@history_item n=1 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** (fake agent's print-ban exemption) — The fake agent `src/bin/viola-fake-agent.rs` is exempt from the print-macro ban through a crate-level `#![allow(clippy::print_stdout, clippy::print_stderr)]`. This chunk extends the fake agent's replay with the two new probes, so any output it adds goes through that exemption and nowhere else (decision/no-print-macros).

@history_item n=2 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** (env_kept, D-34 supersedes D-14) — `process-start{subject:"claude-child"}` gained `env_kept`. D-34 retired the 14-name compile-time list: every stripped and kept name is logged and values are never read. This is the origin of the names-only env rows (env_stripped_count/known/kept, decision/env-names-only). The identity-floor probe's "env NAMES, never values, captured" has to keep the same names-only form.

@history_item n=3 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (verify's outcome on stdout, the uninstrumented `hook --capture` arm) — verify's agent-readable outcome is the exit code plus one stdout step line per row and the `stamped <version>  N pass  N fail` summary. The hidden `hook --capture` arm is uninstrumented by design (no `VIOLA_*`, no obs init, no hook lines; §10 bounded exemption 5), as the founder ratified. A new probe that captures through this arm must keep that shape.

@history_item n=4 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (verify-probe joins the spawn subjects) — Every verify child spawn is a `process-start`/`process-exit` pair logged at the call site, with `child_exit_status` and `duration_ms`. A new closed `subject` value takes a Decisions Log entry, and `run_bounded` stays unlogged. In CI verify runs only against the fake agent; the real-claude verify runs only through the local live run, which is refused under CI. Any new spawn from the probes (for example a second session) follows this rule.

@history_item n=5 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** (registry migration, E2 lift) — The R8 strip's removed set and persistent set, and the 11-name `IDENTITY_FLOOR`, were lifted into §4 E2 from D-34: both name lists comma-joined, no value read. This chunk turns that floor into a relied ledger row.

@history_item n=6 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (strict-modes-failed on the stamps read) — `parse-rejected{parser:"ledger-stamps"}` gained `strict-modes-failed`: run's version gate refusing the stamps file is read as unverified (`cli_verified:false`) at warn, with no path. New relied ids in the stamp land on this read (contract/run-log-parse-rejected/ledger-stamps, decision/unreadable-ledger-is-unverified).

@history_item n=7 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (verify-pty-probe) — Added the `verify-pty-probe` subject, one pair per interactive PTY run, instrumented at the call site. No line holds row text, a prompt or a path. This is the template for a new interactive run, such as a verify child that a second session sends into.

@history_item n=8 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** (six verify spawn pairs, the capture arm's answer body) — Runs C and D raised the pair count to six with the subject enum unchanged. The capture arm stays uninstrumented, but with `--answers` it prints one product-built decision body. This is the precedent for widening the capture arm's output without instrumenting it.

@history_item n=9 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** (fake-agent verify flag list gains --framing) — New ledger rows add a fake-agent replay flag to CI's verify run (`--screens --turn-stop --trusted-root … --dialogs --framing`), bound to test-plan §3 boot step 4. The new probes' fake-agent replay will move this list again.

@history_item n=10 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** (E2's floor sentence) — Corrected E2's verb: the strip always removes the 11 floor names whatever the persistent set says (was "keeps"), and "beyond the floor" there is no compile-time list of known names. It also records that a Linux dev-host reading on 2.1.287 differed from the Windows-measured floor and moved nothing. This is the measured basis and the trap for the identity-floor row (decision/identity-floor-always-stripped).

@history_item n=11 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** (`detail:"interrupted"` on verify's process-exit) — A verify stopped by SIGINT or SIGTERM starts no further spawn, closes every pair it opened and ends with `process-exit` exit 1 `interrupted`. The probe dir is removed on a pass, a fail and a stop. A new probe run or second session must honour the stop path (contract/verify-process-exit/interrupted).

@history_item n=12 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** (the statusline capture arm, uninstrumented, and verify's relied step lines) — `[NN/MM]` became one line per relied row, with MM the relied count and the summary summing to it. A second hidden capture arm was added and kept uninstrumented. This is the precedent for owed rows of existing relied behaviour turned into ledger rows, which is what this chunk does.

@history_item n=13 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** (seven verify spawn pairs; a killed PTY child's exit status) — Run E raised verify's spawns to seven and moved MM. A PTY child killed at a stop carries `child_exit_status`, as measured. Each added run moves the pair counts and MM in the rows (contract/verify-process-exit/child-exit-status-and-duration).

@history_item n=14 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (verify's eight spawn pairs) — Run B's agents read added an eighth pair, reusing `agents-probe` with the subject enum unchanged, and moved MM. No row text, session id, cwd or path goes in a line. This is the precedent for reusing an existing subject for a new verify spawn.

@history_item n=15 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** (plugin install's span, log shape and no-line refusal) — verify and plugin install log their self pair into the caller's `cli-<name>.ndjson` only under a parsed `VIOLA_NAME` and after the check passes; with no name there is no log (decision/verify-logs-only-with-instance).

@history_item n=16 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (MM 24, the sixth Run B paste) — Moved MM to 24 with the 24th row `local-command-compact`. Run B's sixth paste added no spawn pair or subject. This shows that a probe can be a paste inside an existing run rather than a new spawn, which bears on how the harness-prefix probe is shaped.

@history_item n=17 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`viola verify`'s home-tree check; the refusal-telemetry record) — verify checks the home tree before its log opens, and a refusal writes no line. `update_stamps` runs `check_stamps` before and under its lock, and its refusal is exit 1 through verify's own process-exit. A strict-modes failure has a line only on the ledger-stamps read and verify's stamp write (decision/strict-modes-failure-lines, contract/viola-verify-refusal/*).

@history_item n=18 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`verify` writes its refusal's code) — Under a parsed `VIOLA_NAME`, verify's process-exit carries its refusal code: `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture` or `interrupted`. A failing row carries no code. A fixture's `<file>` never enters a code, which matters for the `--record` scrub of new captures (contract/process-exit-detail/verify-refusal).

@history_item n=19 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the Windows host arms of the env strip and verify retired; `batch-script-child` retired) — The strip matches `CLAUDE*` names exactly, with the persistent set taken from `config.json` `claude_env_keep` (the Windows case-insensitive and registry arms are gone). The relied count is all rows of `ALL`. `batch-script-child` left verify's exit-1 causes. The identity-floor row is read on Linux only.

@history_item n=20 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the closed words that left `diag-line.v1.json` with their emitter) — §6 gained the rule that a closed schema word no build can emit leaves the schema with its emitter. Any new subject or detail for the probes is added together with its emitter and leaves together with it.

@history_item n=21 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** (secret-scan classes; Cookie clauses dropped) — These amendments edited decision/secret-scan-before-upload, decision/secrets-never-logged, decision/no-secrets-in-log-lines, decision/claude-env-and-cookie-never-logged and decision/content-only-in-instance-detail-file. The secret scan's classes are now CLAUDE* values, the content canary, mode 0600 and an unreadable file. Captured env names in fixtures or stamps must never let a CLAUDE* value through.

@history_item n=22 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** (decision/payload-types-derive-redact) — Every payload type that carries content derives veil Redact. A type with no content field derives none, because veil 0.3.0 refuses that derive. A new capture payload type for the probes falls under this row.

@history_item n=23 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (rows decision/os-apis-at-call-sites, decision/instrument-call-sites-not-os-internals, decision/parse-failures-*; process-start subjects) — Added the hyprctl spawn to the call-site-instrumented operations, `hyprctl-probe` to the subject list and the hyprctl parse exception. This is the precedent for how a new spawn and parser extend these picked rows.

@history_item n=24 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (decision/exit-code-log-field) — Exit 11 is retired and never reused. A new verify outcome cannot take 11.

@history_item n=25 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (capture arms two to one; verify lines; process-start/exit fields; child-spawn subject) — The changed rows are decision/capture-arms-uninstrumented, decision/hidden-capture-arms-uninstrumented, decision/hook-capture-arms-unlogged, contract/hook-stdout/capture-arms, decision/hook-instance-check-before-obs-init, decision/instance-check-before-log-init, decision/verify-lines-carry-no-content, contract/diag-line-fields/process-start and process-exit, contract/child-spawn-log/subject, decision/sensitive-content-detail-file-only and contract/stdout-by-role/hook. Only `hook <event> --capture` remains, so a new probe capture (env names, a harness prompt) either rides that one arm or adds an arm the rows must then name again.

@history_item n=26 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (StopFailure with no ledger row) — StopFailure was registered with no ledger row and no verify probe, with the P4 answer "no ledger row". This is the counter-precedent to this chunk: here a relied behaviour is made a ledger row with a probe.

@history_item n=27 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (the StopFailure no-ledger-row answer ratified) — The founder ratified "no ledger row" for StopFailure, its live reading owned by the recorded fixture set. This shows how a decision about whether a behaviour gets a ledger row is ruled and ratified.

@history_item n=28 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (decision/claude-internals-not-a-contract, decision/transcripts-not-a-contract, contract/send-path-log-events/event-order, decision/deterministic-emission) — Added the first named exception to "claude internals / transcripts are not a contract" (the stdout tag count, no byte kept) and the matching event-order and emission exceptions. Reading a prompt's raw prefix in a verify child is another internal read that these rows' exceptions would have to name.

@history_item n=29 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** (local-command-delivery rows ratified) — The founder ratified the claude-internals, transcripts, event-order and deterministic-emission exceptions. Those rows stand on his word, and widening them takes the same ruling path.

@history_item n=30 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** (decision/parse-failures-unknown-counter, decision/parse-failures-surface-as-unknown) — Added the second exception: a StopFailure error word outside the CLI's set reads cause api-error with one parse-rejected line. This is how a picked row's except list grows when a new reading avoids `unknown`.

@history_item n=31 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Run F; nine spawn pairs; claude-internals and refusal-telemetry exceptions; r8-strip names only; env-vars-not-configuration; trust-record-write-uninstrumented) — This is the closest precedent ledger-row chunk. Run F moved the PTY runs to six and the spawn pairs to nine. verify reading the kept `CLAUDE_CONFIG_DIR` value broke "values never read" and was bounded in decision/r8-strip-logged-as-names-only and decision/env-vars-not-configuration. The config write went uninstrumented, with its outcome riding the stamps. An identity-floor probe that reads env must stay inside "no value is read, bar CLAUDE_CONFIG_DIR", or the rows must be amended again.

@history_item n=32 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (r8-strip names only, claude boundary-only, env-vars-not-configuration, claude-internals except, trust-record-write-uninstrumented) — Widened the CLAUDE_CONFIG_DIR value read to the run and revive start and added the start's one trust-record line to the claude boundary list. These are the current texts of the picked env and boundary rows this chunk's identity-floor probe rests on.

@history_item n=33 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (contract/verify-stdout/step-line, summary-line, verify-exit-codes/0; decision/browser-tools-prompt-reading-uninstrumented) — Added the `absent` verdict for the browser-tools-prompt row, counted in the pass total and in exit 0. A screen reading for a ledger row stays uninstrumented, with its outcome in the stamp and stdout only. New rows join this step-line, summary and exit shape.

@history_item n=34 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** (decision/browser-tools-prompt-reading-uninstrumented) — verify's fixed stdout line on a met prompt is the prompt's only record. The live home's re-stamp is the re-pin step's, which this chunk's scope keeps untouched.

@history_item n=35 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** (decision/wheel-typed-mark-uninstrumented) — In-memory wheel state (the typed mark, the held wakes) is uninstrumented by plan. Harness-prefixed prompts are the turns that never move the wheel, so the harness-prefix row rests on this same uninstrumented wheel logic.

@history_item n=36 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** (turn-running's two causes) — A running turn is in-memory wrapper state, marked by a `prompt-submitted` of any origin, that writes no event, span or line of its own. This bears on how a harness-origin prompt filed by its raw prefix is observed: only through existing lines.

@history_item n=37 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** (contract/diag-subject/auto-verify, contract/run-process-log/auto-verify-stop, contract/config-json/known-keys) — The start's automatic verify is a subject `auto-verify` child, and a stop during it ends the start by the signal. `config.json` has five known keys, `claude_env_keep` (the env keep-list) among them. New verify runs execute inside the start's auto-verify too, so the added probes lengthen that window and its stop path. This entry is PROVISIONAL until the founder's word.
