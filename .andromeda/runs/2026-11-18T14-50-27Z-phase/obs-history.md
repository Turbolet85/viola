@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (verify's outcome on stdout) set verify's agent-readable outcome: exit 0/1, one stdout step line per row and the last line `stamped <version>  N pass  N fail`; refusals are the stderr `unable:` + `hint:` pair. This is the precedent behind contract/verify-stdout/step-line and summary-line, which the 24th relied row changes (MM 23 → 24).

@history_item n=2 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — added `verify-probe` to the closed `subject` enum (a new closed value needs a Decisions Log entry), logged each verify spawn at its call site, and limited CI verify to the fake agent, with the real-CLI verify only through the local live run, which refuses under CI. This bears on decision/ci-verify-fake-agent-only and decision/real-cli-verify-never-in-ci: the new browser-tools probe is proved in CI only against fake-agent screens.

@history_item n=3 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — put the vt100 feed on its own thread with a per-call `catch_unwind`: a poisoning writes one `parse-rejected{vt100-feed, panicked}`, and confirmed `send` then reads `input-not-ready`. This is the gate whose modal reading this chunk extends; its outcome is `ready|input-not-ready` only (contract/send-span-attributes/run-readiness-gate).

@history_item n=4 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — vt100-feed `oversize` covers a dropped copy, and `tests/chaos_feed_panic.rs` witnesses the panic line followed by `send` refused `input-not-ready` (exit 13) while typed keys still pass through. It is the standing proof form for a gate refusal, and the precedent that a refused gate keeps the human's passthrough.

@history_item n=5 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — added `strict-modes-failed` to the `parse-rejected{parser:"ledger-stamps"}` details (read as unverified, `cli_verified:false`, at WARN, no path), and moved CI verify to the recorded CLI version. This is the run version-gate line (contract/run-log-parse-rejected/ledger-stamps) that a home stamped at 23 rows meets when it reads a 24-row ledger.

@history_item n=6 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — added `verify-pty-probe` to the subject enum, one pair per interactive PTY run, and made CI's fake-agent verify pass `--screens --turn-stop --trusted-root`. This is the precedent for a probe that needs an interactive run, and for the fake agent learning a screen behind a flag.

@history_item n=7 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — took verify from four spawn pairs to six (Runs C and D) with the subject enum unchanged, and CI added `--dialogs` to the fake agent. Precedent: dialog/modal screens in the fake agent are switched on by a flag, and new runs reuse `verify-pty-probe`.

@history_item n=8 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — CI's fake-agent verify flags gained `--framing`, bound to test-plan §3's boot step 4. Nothing else moved, and no row text, prompt or path reaches a line. If a browser-tools screen flag is added, the obs flag list moves with test-plan in the same pass.

@history_item n=9 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `send-confirmed.duration_ms` runs from the send's arrival, so it includes the readiness gate's wait up to `GATE_MAX_WAIT`. Today a standing browser-tools prompt makes the gate wait out that bound. Reading it as a modal changes the outcome to a refusal, with no new field.

@history_item n=10 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — added `detail:"interrupted"` on verify's process-exit after every opened spawn pair is closed. A stopped run starts no further spawn, prints the fixed stop pair and writes no stamp. This bears on the scope's "the stop with the modal standing included": a stop during the browser-tools probe ends this way.

@history_item n=11 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — changed verify's step lines to one `[NN/MM]` per relied row (was per ledger row), with the summary summing to MM. The 24th relied row moves MM; no line carries the probe command, a payload or a path.

@history_item n=12 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — added Run E (seven pairs, subject enum unchanged), and set by measurement that a PTY child killed at a stop carries `child_exit_status` and `duration_ms`. Precedent for a ledger row that adds a run, and the measured fact behind contract/verify-process-exit/child-exit-status-and-duration.

@history_item n=13 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added verify's eighth spawn pair (Run B's agents read, reusing `agents-probe`) and moved MM. Precedent: a new probe step inside an existing run reuses a subject and adds no enum word.

@history_item n=14 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — added a 24th row via a sixth paste in Run B with no new spawn pair or subject. This is the precedent for measuring a row inside an existing run rather than a new one. The P3 probe-shape choice decides the spawn-pair count (nine today) and contract/verify-spawn-subject/verify-pty-probe's "six".

@history_item n=15 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (verify's home-tree check; refusal-telemetry record) verify checks the home tree before its log (a refusal writes no line), and `update_stamps` checks strict modes before writing, its refusal riding verify's own process-exit. A strict-modes line exists only at the ledger-stamps read and the refused stamp write. This underlies contract/viola-verify-refusal/* and decision/strict-modes-failure-lines, which a stamp of a 24-row ledger goes through.

@history_item n=16 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (verify writes its refusal's code; Founder Direction 4) verify writes its refusal code on process-exit under a parsed VIOLA_NAME; a failing row is no refusal and carries no code. Each exit-1/21 cause gets a distinct code, except a strict-modes check refused before any log opens. So a 24th row failing (browser-tools prompt unmeasured) is a fail line, not a new detail code.

@history_item n=17 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (closed words; relied count; strict-modes lines) the relied count read all rows of `ALL` on Linux. A strict-modes failure has a line in exactly two places. A closed diag-line word no build can emit leaves the schema with its emitter (decision/unemittable-schema-words-removed). The relied-count wording and the schema rules bound any "verify record word" this chunk adds.

@history_item n=18 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — turned the master's text into rows (decision/contract/absence), each verified against the code. This is the form every amendment of this chunk takes: `+`/`~`/`−` by handle.

@history_item n=19 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — narrowed decision/diag-schema-additive-only, decision/log-fields-additive-only, decision/log-schema-additive-only and decision/obs-aligns-to-tests-log-format to "must not rename or remove a field the product still emits". It also retired exit 11 (decision/exit-code-log-field). These are the schema rules any new verify or gate word must follow: additive, with a Decisions Log record.

@history_item n=20 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — dropped "a status line payload" from decision/verify-lines-carry-no-content (Run B's status-line probe retired), left one hidden capture arm, and removed the statusline-shell subject. The relied-row set and verify's run contents changed here before 23 → 24, so the count is re-measured, never copied.

@history_item n=21 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — measured on 2.1.325 that folder-trust keys passing through the wrapper write `wheel{human-input}` before `session-start`. It is a nearby start-screen precedent: a start modal the human answers moves the wheel. The browser-tools prompt is never typed into by viola, so only a human key would do this.

@history_item n=22 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — set decision/deterministic-emission's exception (the readiness gate reads the vt100 screen before a paste and can refuse `input-not-ready`) and its twin deterministic-event-emission, and first gave decision/claude-internals-not-a-contract an exception. The modal literal must fit this gate-outcome exception and decision/no-telemetry-from-screen-content: no new telemetry from screen content.

@history_item n=23 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** — the founder ratified the local-command-delivery rows (claude-internals-not-a-contract and refusal-telemetry excepts, deterministic-emission excepts, transcripts-not-a-contract), so those exceptions stand as his word that this chunk reads against.

@history_item n=24 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — the last ledger-row precedent: Run F took the interactive runs to six and the spawn pairs to nine (contract/verify-spawn-subject/verify-pty-probe, contract/diag-subject/verify-pty-probe, contract/verify-process-exit/child-exit-status-and-duration). It added decision/trust-record-write-uninstrumented and the claude-internals and refusal-telemetry excepts. A new browser-tools run or probe moves the same counts.

@history_item n=25 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — the chunk this one follows. It widened decision/claude-boundary-only-instrumentation (the start's one trust-record line), decision/claude-internals-not-a-contract (a verified start sets its folder's trust flag only), decision/trust-record-write-uninstrumented, decision/r8-strip-logged-as-names-only and decision/env-vars-not-configuration. It added the trust-record event and its rows, all PROVISIONAL on the delegate's "Confirm all 26". The scope's boundary "viola writes nothing to turn the prompt off" keeps these exceptions unwidened.

@history_item n=26 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — added the hyprctl parser and spawn to decision/per-parser-parse-failure-counts and decision/os-apis-at-call-sites / instrument-call-sites-not-os-internals, plus the first except on parse-failures-surface-as-unknown. These are the per-parser rows a vt100 modal reading must not extend, since a modal is a gate outcome, not a parse failure.

@history_item n=27 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** — added the StopFailure exception to decision/parse-failures-unknown-counter and parse-failures-surface-as-unknown, the current wording of those picked rows.
