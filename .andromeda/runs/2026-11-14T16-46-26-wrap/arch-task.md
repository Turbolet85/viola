@task_prose part=session
The session removed the usage-budget gate whole, on the founder's ruling: no rate-limits reading, no budget.json, no send budget rung, no budget-paused state, no turn-end or start budget evaluation, no release --budget, no board or list budget fields, no Path 6 suite.
It retired the budget-gate event kind, the budget-paused refusal reason and its CLI exit 11 (retired, never renumbered or reused), the five_hour and seven_day config thresholds, the statusline-rate-limits ledger row and the release budget param and its override_until reply.
What stays: the driver release guard (release-from-driver), the statusline pass-through and its instance check, config.json with dialog_hold_s, and the readers of old disk state, which now skip budget-era fields and lines.
A channel release carrying any budget key is now refused -32602 invalid-params with no wheel moved; a plain release answers ok with the wheel holder alone.
Every own-format v stays 1 (snapshot, channel params, event line, list document) by the founder's word, as an exception to the protocol version bump rule.
Counts moved: event kinds 16 to 15, relied ledger rows 24 to 23 (verify prints 23 steps), config keys 6 to 4, start records 3 to 2; chrono left the Claude agent crate.
@task_file n=1 path=src/run/budget.rs
what: deleted — the budget gate and governor: thresholds, the decide step, the trigger, the pause lift
@task_file n=2 path=src/run/send/budget.rs
what: deleted — the send path's budget rung (the budget-paused check in the send refusal order)
@task_file n=3 path=crates/viola-state/src/budget.rs
what: deleted — the budget.json writer and readers in the state crate
@task_file n=4 path=crates/viola-core/src/lib.rs
what: the core vocabulary — normalised event kinds (EventKind::ALL), RefusalReason, the domain newtypes; lost budget-gate, budget-paused and the budget reading types
@task_file n=5 path=crates/viola-state/src/snapshot.rs
what: the instance snapshot and its atomic writer; no longer holds budget_paused or budget_override_until
@task_file n=6 path=crates/viola-state/src/replay.rs
what: snapshot replay from the events log; recovers links, agent_session_id and wheel alone, skipping budget-gate lines
@task_file n=7 path=crates/viola-state/src/board.rs
what: the board read model behind list (list payload items); lost the budget_paused item field
@task_file n=8 path=crates/viola-state/src/lib.rs
what: the state crate root; the budget module removed
@task_file n=9 path=crates/viola-agent-claude/src/statusline.rs
what: the Claude statusline payload shapes; the rate_limits reading removed
@task_file n=10 path=crates/viola-agent-claude/src/ledger.rs
what: the per-behaviour CLI capability ledger and its relied rows; the statusline-rate-limits row removed, 23 relied rows
@task_file n=11 path=crates/viola-agent-claude/Cargo.toml
what: the Claude agent crate's dependencies; chrono removed
@task_file n=12 path=src/run/wheel.rs
what: the wheel in wrapper memory and the channel release method; release takes no gate, refuses any budget key as invalid-params
@task_file n=13 path=src/run/send.rs
what: the channel send method and its refusal order; the budget rung gone, send-refused lines carry no budget fields
@task_file n=14 path=src/run/send/slot.rs
what: the send slot of the wrapper; its budget constructor removed
@task_file n=15 path=src/run/mod.rs
what: the wrapper run module tree; the budget module removed
@task_file n=16 path=src/cmd/run.rs
what: the run start path and the wrapper's channel method dispatch; one start wheel record then session-start, no budget evaluation at start or turn end
@task_file n=17 path=src/cmd/revive.rs
what: viola revive, which shares the start path
@task_file n=18 path=src/cmd/release.rs
what: the viola release CLI verb; the --budget flag removed (clap refuses it, exit 2)
@task_file n=19 path=src/cmd/send.rs
what: the viola send CLI verb and its exit-code map; nothing maps to 11, an older budget-paused reads unknown (14)
@task_file n=20 path=src/cmd/list.rs
what: viola list, the caller-side board read; no budget or thresholds read, ok payload items and skipped only
@task_file n=21 path=src/cmd/hook/statusline.rs
what: viola hook statusline, the statusline wrapper; no rate_limits reading, no budget-file write, no budget_written on hook-decision
@task_file n=22 path=src/cmd/verify.rs
what: viola verify, the sole stamps writer and fixture recorder; 23 steps, records StatusLine.default-1.json alone
@task_file n=23 path=src/human.rs
what: the one human writer of the root bin; the budget lifted and open lines and the override end removed
@task_file n=24 path=src/human/board/bay.rs
what: the board's BAY line in the human list output; its budget cells (5H, 7D, read, gate) gone
@task_file n=25 path=src/human/board/rows.rs
what: the board rows' WHEEL cell; no budget-paused form, width stays 23
@task_file n=26 path=src/human/hints.rs
what: human hint lines; the budget-paused hint removed
@task_file n=27 path=src/obs.rs
what: observability and the config keys list (CONFIG_KEYS); now v, diagnostics_level, claude_env_keep, dialog_hold_s; budget span removed
@task_file n=28 path=src/bin/viola-fake-agent.rs
what: the fake agent CI runs against; no rate-limits statusline replay after the first Stop
@task_file n=29 path=schemas/config.v1.json
what: the config.json schema; five_hour and seven_day dropped
@task_file n=30 path=schemas/diag-line.v1.json
what: the diagnostics line schema; budget_written and the send lines' budget fields dropped
@task_file n=31 path=schemas/claude-statusline-fixture.v1.json
what: the recorded statusline fixture schema; its rate_limits pin dropped
@task_file n=32 path=fixtures/viola-0.1.0/retired-words.json
what: new — the retired 0.1.0 own-format words, read by compatibility cases and walked by the fixture hygiene check
@task_file n=33 path=fixtures/viola-0.1.0/snapshot.json
what: new — a budget-era 0.1.0 instance snapshot, read by the snapshot compatibility case
@task_file n=34 path=fixtures/viola-0.1.0/events.ndjson
what: new — a budget-era 0.1.0 events log, read by the replay compatibility case
@task_file n=35 path=fixtures/claude/2.1.302/StatusLine.rate-limits-1.json
what: deleted — the recorded rate-limits statusline fixture
@task_file n=36 path=tests/cli_budget.rs
what: deleted — the Path 6 budget CLI suite (a guard of the budget and replay decisions)
@task_file n=37 path=crates/viola-e2e/tests/path6_budget_harness.rs
what: deleted — the Path 6 end-to-end budget suite
@task_file n=38 path=crates/viola-e2e/src/harness/boot.rs
what: the e2e harness readiness; reads two start records
@task_file n=39 path=tests/contract_fixture_hygiene.rs
what: the recorded fixtures hygiene check; now walks the viola-0.1.0 fixtures too
@task_file n=40 path=fuzz/Cargo.lock
what: the fuzz workspace lockfile audited separately; chrono and its dependents dropped
@task_item n=1 part=touches
Every budget gate and governor decision is retired: default thresholds, fail-open, never-polls, soft stop, pause gates only send, pause lift, readings last-writer-wins, only send reads the gate, budget release per instance.
@task_item n=2 part=touches
budget.json and its single writer are retired: the budget-json contract rows (five_hour, seven_day, read_at, v), the single-writer and shared-reading decisions.
@task_item n=3 part=touches
The budget-gate event kind is retired: its normalised-event-kinds row, its event-data row and its wrapper-event-appends row; the event kinds count is 15.
@task_item n=4 part=touches
RefusalReason budget-paused is retired; an older wrapper's budget-paused reads unknown through the catch-all.
@task_item n=5 part=touches
CLI exit code 11 is retired, never reused or renumbered: the exit-codes 11 row is set to say so; 10, 12, 13, 14 unchanged.
@task_item n=6 part=touches
The send refusal order loses its budget rung: control-character, empty-text, human-typing, turn-running, the readiness gate, then human-typing and turn-running again, then no-prompt-submitted; the send-budget-paused channel rows retire.
@task_item n=7 part=touches
send-refused event data carries no window, used_percentage, resets_at or paused.
@task_item n=8 part=touches
The channel release method: params from only, reply the wheel holder alone; any budget key is -32602 invalid-params, no wheel moved, no record; release budget true and budget release durability rows retire, the release type check and plain release rows are set.
@task_item n=9 part=touches
The release-from-driver guard stays and is still checked first; the driver-release-deterred decision stands but its rows that pair it with the budget param are set.
@task_item n=10 part=touches
The release CLI verb loses --budget; clap refuses it.
@task_item n=11 part=touches
The list payload is items and skipped only: the list-payload budget row and the list-item budget_paused row retire; the human board's BAY line has no budget cells.
@task_item n=12 part=touches
The instance snapshot holds no budget_paused or budget_override_until; snapshot replay recovers links, agent_session_id and wheel alone (the replay-recovers-only-log-fields decision).
@task_item n=13 part=touches
config.json loses five_hour and seven_day: their contract rows retire; config keys equal the schema at four keys; a config still holding a threshold key logs unknown-keys and nothing refuses.
@task_item n=14 part=touches
The protocol version bump rule gains an exception: retiring the budget fields, kind and refusal reason bumps no own-format v; every v stays 1.
@task_item n=15 part=touches
The statusline hook reads no rate_limits and writes no file; the statusline-rate-limits ledger row retires, the relied rows count is 23 and verify prints 23 steps; the version-verified-all-relied-rows decision now reads 23 rows.
@task_item n=16 part=touches
The tolerant resets_at decision for external payloads has no remaining reader of rate-limits resets_at.
@task_item n=17 part=touches
The start writes two records, the start wheel record then the hook's session-start; no budget evaluation at start or at turn end (run start order rows).
@task_item n=18 part=touches
chrono is no longer a dependency of the Claude agent crate; the chrono timestamps decisions stand for the crates that still use it.
@task_item n=19 part=touches
The fake agent no longer replays a rate-limits statusline payload after the first Stop.
@task_item n=20 part=touches
Guards naming the deleted Path 6 suite and the budget CLI test file must retire with their decisions or be re-pointed (the snapshot replay from the log decision's guard moves to the unreadable-snapshot replay case and the links case).
@task_item n=21 part=touches
The MCP tool descriptions and samples drop budget-paused; the recorded MCP tools/list exchange still mentions the usage budget until re-recorded live.
@task_item n=22 part=touches
The recorded fixtures hygiene decision now covers the committed viola-0.1.0 own-format fixtures.
@task_item n=23 part=touches
Diagnostics: hook-decision carries no budget_written; the run.budget_eval span and the budget span attributes are gone.
@task_fact n=1 slug=design-philosophy
fact: viola has no usage-budget gate: it reads no subscription rate limits, keeps no budget state and never pauses or refuses a send on usage (retired in 0.2.0).
@task_fact n=2 slug=standard-contracts
fact: A channel release carrying a budget key, of any value, is refused -32602 invalid-params with no wheel moved and no record; a string from is still refused release-from-driver first.
@task_fact n=3 slug=conventions
fact: A retired exit code is never reused or renumbered: exit 11, once budget-paused, stays unassigned.
@task_fact n=4 slug=cross-cutting-patterns
fact: Readers of own formats read a budget-era writer's state whole: a snapshot holding budget_paused or budget_override_until reads with those keys skipped, a replay over a log holding budget-gate lines skips those lines, a stamp holding the retired ledger row verifies on the relied rows alone.
@task_fact n=5 slug=occupied-resources
fact: Older-release own-format shapes and retired words live as committed fixtures under a per-version fixtures folder, outside the identifier-probed source trees, and the fixture hygiene walk covers them.
@task_item n=1 part=untouched
Other budgets named budget: the SessionEnd 1.5 s budget, the dirty-page budget, performance, retry, error and hook latency budgets stand.
@task_item n=2 part=untouched
Coverage floors, mutation thresholds and the heartbeat liveness thresholds are other thresholds; they stand.
@task_item n=3 part=untouched
The dialog hold, the dialog hooks, answer and the pending-dialog contracts are not reached.
@task_item n=4 part=untouched
Link and unlink recording, the links in the log and their channel methods are not reached.
@task_item n=5 part=untouched
The PTY seam, the vt100 readiness gate and its timings, and the Hyprland seam are not reached.
@task_item n=6 part=untouched
Endpoint naming, socket directory checks, peer euid admission and server identity checks are not reached.
@task_item n=7 part=untouched
The pinned copy, plugin install and plugin embedding, and revive's own refusals and error codes are not reached.
@task_item n=8 part=untouched
CI workflow, cargo-deny policy, toolchain pins and release tooling are not reached beyond the dropped dependency edge.
