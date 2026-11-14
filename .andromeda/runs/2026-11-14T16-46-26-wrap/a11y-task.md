@task_prose part=session
The session removed the budget gate whole on the founder's ruling: no rate-limits reading, no budget.json, no budget rung on send, no budget-paused refusal, no budget cells on the board, no budget fields in the log, the snapshot or the list document, and the Path 6 budget suite deleted.
Exit 11 (was budget-paused) is retired, never renumbered or reused; nothing maps to 11, and an older wrapper's budget-paused code reads as the unknown refusal, exit 14.
The send refusal order is now control-character, empty-text, human-typing, turn-running, the readiness gate, then human-typing and turn-running again, then no-prompt-submitted; send-refused lines carry no window, used_percentage, resets_at or paused.
release keeps its driver guard (release-from-driver refused first); viola release --budget is now a clap usage error (exit 2) and a channel release carrying any budget key is invalid-params, no wheel moved; a plain release answers only the wheel holder.
viola list prints the BAY line as skipped names alone (the 5H, 7D, read and gate cells gone), its document is items and skipped, the WHEEL cell has no budget-paused form and keeps width 23; viola verify counts 23 steps (was 24); config.json loses five_hour and seven_day, a file still carrying them is logged as unknown keys and never refused.
The start writes two records (wheel start, then session-start); the status-line hook passes the agent's output through unchanged, with no reading and no budget write; every own-format v stays 1.
@task_file n=1 path=src/run/budget.rs
what: the budget gate and governor on automated sends, its thresholds and the lift (deleted)
@task_file n=2 path=src/run/send/budget.rs
what: the send-path budget rung that refused a send as budget-paused (deleted)
@task_file n=3 path=crates/viola-state/src/budget.rs
what: the budget.json writer and readers (deleted)
@task_file n=4 path=tests/cli_budget.rs
what: the Path 6 budget CLI suite, guard of the budget gate rows (deleted)
@task_file n=5 path=crates/viola-e2e/tests/path6_budget_harness.rs
what: the Path 6 budget end-to-end harness case, guard of the budget gate rows (deleted)
@task_file n=6 path=fixtures/claude/2.1.302/StatusLine.rate-limits-1.json
what: the recorded status-line payload carrying rate_limits, the live reading the gate read (deleted)
@task_file n=7 path=fixtures/claude/2.1.287/StatusLine.rate-limits-1.json
what: the older recorded status-line payload carrying rate_limits (deleted)
@task_file n=8 path=crates/viola-core/src/lib.rs
what: the code-word vocabulary — refusal reasons (budget-paused removed) and event kinds (budget-gate removed)
@task_file n=9 path=src/cmd/send.rs
what: the CLI exit-code map of the send refusals — 10 human-typing, 12, 13, 14 unknown; 11 now unused
@task_file n=10 path=src/run/send.rs
what: the send refusal order and the send-refused diagnostic line
@task_file n=11 path=src/run/send/slot.rs
what: the send slot the channel send passes through
@task_file n=12 path=src/human/hints.rs
what: the one-hint-per-code table of refusal hints (budget-paused hint removed)
@task_file n=13 path=src/human.rs
what: the human output writers — the budget lifted and open lines and the override end removed
@task_file n=14 path=src/human/verbs.rs
what: the human result lines of the wheel verbs
@task_file n=15 path=src/cmd/release.rs
what: the viola release verb — its --budget flag removed, now a usage error
@task_file n=16 path=src/run/wheel.rs
what: the wheel and the release channel method — driver guard, budget key refused as invalid-params
@task_file n=17 path=src/cmd/list.rs
what: viola list — the board and its --json document, items and skipped
@task_file n=18 path=src/human/board/bay.rs
what: the board's BAY line, now the skipped names alone
@task_file n=19 path=src/human/board/cells.rs
what: the board's WHEEL cell and its words, width 23, no budget-paused form
@task_file n=20 path=src/human/board/rows.rs
what: the viola list board rows and their order
@task_file n=21 path=crates/viola-state/src/board.rs
what: the board item state the list reads (budget_paused removed)
@task_file n=22 path=src/cmd/verify.rs
what: viola verify — its step lines, now 23 steps
@task_file n=23 path=src/cmd/hook/statusline.rs
what: viola hook statusline — the status-line pass-through, its exit status and the malformed-payload refusal
@task_file n=24 path=crates/viola-agent-claude/src/statusline.rs
what: the status-line payload parsing — the rate_limits reading removed
@task_file n=25 path=crates/viola-agent-claude/src/ledger.rs
what: the relied ledger rows verify checks, now 23, statusline-rate-limits retired
@task_file n=26 path=src/cmd/run.rs
what: the run and revive start — two start records, no budget evaluation at start or turn end
@task_file n=27 path=src/obs.rs
what: the config.json keys read — v, diagnostics_level, claude_env_keep, dialog_hold_s
@task_file n=28 path=schemas/config.v1.json
what: the config.json schema — five_hour and seven_day removed
@task_file n=29 path=schemas/diag-line.v1.json
what: the diagnostic line schema — send-refused budget fields and budget_written removed
@task_file n=30 path=crates/viola-state/src/snapshot.rs
what: the instance snapshot — budget_paused and budget_override_until removed
@task_file n=31 path=crates/viola-state/src/replay.rs
what: the log replay that rebuilds the snapshot, skipping old budget lines
@task_file n=32 path=crates/viola-mcp/src/result.rs
what: the MCP tool results and their refusal bodies
@task_file n=33 path=crates/viola-mcp/src/tools.rs
what: the MCP tool descriptions
@task_file n=34 path=tests/cli_list.rs
what: the viola list literal pins — board caption, BAY line and document
@task_file n=35 path=tests/cli_verify.rs
what: the viola verify literal pins of the step lines and the pass summary
@task_file n=36 path=tests/hook_statusline.rs
what: the status-line pass-through byte-for-byte checks
@task_file n=37 path=tests/cli_wheel.rs
what: the release driver-guard checks
@task_file n=38 path=src/bin/viola-fake-agent.rs
what: the fake agent the CI legs run against — its rate-limits status-line replay removed
@task_file n=39 path=fixtures/viola-0.1.0/retired-words.json
what: the retired words (budget-paused, budget-gate and the old snapshot keys) the compatibility cases read (new)
@task_item n=1 part=touches
decision/budget-gate and decision/budget-gate-only-on-send: the budget gate on automated sends is retired whole; no send reads a budget gate.
@task_item n=2 part=touches
contract/cli-exit-codes/11: exit 11 (budget-paused) is retired, never renumbered or reused; the row is set to say so.
@task_item n=3 part=touches
contract/cli-exit-codes/14 and contract/cli-refusal/unknown-refusal: an older wrapper's budget-paused code now reads as unknown, exit 14.
@task_item n=4 part=touches
contract/cli-exit-codes/10, 13, 2 and contract/cli-refusal/exit-code: the exit table loses its 11 row; any row listing the codes in order must drop budget-paused.
@task_item n=5 part=touches
contract/config-json/five-hour and contract/config-json/seven-day: both threshold keys removed from config.json and its schema (defaults 90 and 85 gone); a file still carrying them is logged as unknown keys, not refused.
@task_item n=6 part=touches
decision/send-refused-while-human-typing, contract/send-refusal/input-not-ready, contract/send-refusal/no-prompt-submitted, decision/critical-path-send-readback: the send refusal order loses its budget rung.
@task_item n=7 part=touches
decision/one-hint-per-code, decision/no-hint-names-release, decision/unlisted-code-words-stay-visible, contract/code-word/format: the budget-paused code word and its hint are gone from the vocabulary.
@task_item n=8 part=touches
contract/viola-list-board/caption, contract/viola-list-board/order, contract/viola-list-caption/order, decision/list-captions-fixed, decision/critical-path-cli-board-output: the BAY line is now skipped names alone, the 5H, 7D, read and gate cells removed.
@task_item n=9 part=touches
decision/state-carried-by-printed-word, decision/stale-row-dim, contract/viola-list-board/colour: the WHEEL cell no longer prints a budget-paused word; width stays 23.
@task_item n=10 part=touches
contract/viola-verify/step-line and contract/viola-verify/interrupted-pair: verify now counts 23 steps (was 24), the statusline-rate-limits row retired.
@task_item n=11 part=touches
contract/viola-hook-statusline/exit, decision/statusline-output-passed-byte-for-byte, decision/live-readings-never-gate: the status-line hook reads no rate_limits and writes no budget file; the pass-through and its exit stay.
@task_item n=12 part=touches
decision/time-bounds-fall-on-the-driver, decision/time-bounds-never-on-human, decision/no-unwarned-timeouts: the budget override window (override_until) and the release --budget lift are retired.
@task_item n=13 part=touches
decision/critical-path-human-takes-wheel, decision/human-key-takes-the-wheel: release --budget gone; the release-from-driver guard stays; a release carrying any budget key is invalid-params and moves no wheel.
@task_item n=14 part=touches
contract/cli-usage-error/exit-2, contract/cli-exit-codes/2: viola release --budget is now an unknown argument, exit 2.
@task_item n=15 part=touches
contract/cli-json-failure/document, contract/start-json-refusal/document, contract/cli-refusal/json: the list --json document is items and skipped, no budget member; the release reply is the wheel holder alone.
@task_item n=16 part=touches
Every row whose guard names a deleted Path 6 case (path6_ or cli_budget or mcp_tools_path6_budget_paused_send or setting_does_not_disable_the_budget_gate): retire with its decision or re-point.
@task_item n=17 part=touches
decision/a11y-inside-existing-suites, decision/a11y-runs-in-existing-test-suites: the Path 6 budget suite left the suites the a11y assertions run in.
@task_item n=18 part=touches
contract/structured-violation-json/cli-tui-a11y-rows: any a11y row listed for the budget refusal or the BAY budget cells is retired.
@task_fact n=1 slug=cognitive-accessibility
fact: Exit 11 is retired (was budget-paused) and never reused; an older wrapper's budget-paused code reads as unknown, exit 14.
@task_fact n=2 slug=a11y-scope-summary
fact: viola release takes no --budget flag; passing it is a usage error, exit 2, and a channel release carrying any budget key is refused invalid-params with no wheel moved.
@task_fact n=3 slug=cognitive-accessibility
fact: A config.json still carrying five_hour or seven_day is logged once as unknown keys at start and never refused.
@task_item n=1 part=untouched
The other budgets and thresholds stand: the a11y performance budget, CI wall and zero-flakiness rows of section 10, and the threshold row there, are not the retired gate.
@task_item n=2 part=untouched
The TUI passthrough contracts (entry, keystrokes, focus, mouse and resize sequences, vt100 feed panic, viola terminal bytes, run diagnostics) are not reached.
@task_item n=3 part=untouched
The colour and SGR rules (no SGR off a terminal, word beside every SGR span, amber only on DIALOG, bold only on NAME, failure words never coloured) are not moved.
@task_item n=4 part=untouched
The revive refusals and revive --list, link and unlink results, plugin install, and the strict-modes, socket-directory and server-verify refusals of the frame verbs, hook calls and MCP frame tools are not reached.
@task_item n=5 part=untouched
The screen-reader targets and setups, the manual keyboard and Orca passes, the scope tier and the absences (web frontend, touch surface, internationalization, rendered page, user time limit) are not reached.
@task_item n=6 part=untouched
The dialog rows (dialog hold bounds the driver, dialog_hold_s, dialog-expired event, critical path dialog answer) stand; dialog_hold_s stays in config.json.
