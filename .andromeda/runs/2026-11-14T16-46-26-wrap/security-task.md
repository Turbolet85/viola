@task_prose part=session
The session removed the budget gate whole: no rate-limits reading, no budget.json file, no send budget rung, no budget-paused refusal, no budget fields on the board, the log or the snapshot, no Path 6 suite.
The driver release guard (release-from-driver) stays; a channel release now refuses any budget key with -32602 invalid-params and moves no wheel; viola release has no --budget flag.
CLI exit 11 is retired, never renumbered or reused; nothing maps to it, and an older wrapper's budget-paused reads unknown (14).
config.json loses its five_hour and seven_day threshold keys (config keys now v, diagnostics_level, claude_env_keep, dialog_hold_s); a file still carrying one logs one parse-rejected unknown-keys at start and nothing refuses.
viola hook statusline keeps its instance check, capped stdin, snapshot read, malformed-json refusal and pass-through, but reads no rate_limits and writes no file; every own-format v stays 1.
Readers of old disk state stay: a budget-era snapshot reads whole skipping its budget fields, a replay skips the budget-gate lines; the 0.1.0 shapes and retired words live in new viola-format fixtures walked by fixture hygiene.
@task_file n=1 path=src/run/budget.rs
what: the budget gate and governor (thresholds, decide, the lift) inside the wrapper, deleted
@task_file n=2 path=src/run/send/budget.rs
what: the send method's budget rung (budget-paused refusal), deleted
@task_file n=3 path=crates/viola-state/src/budget.rs
what: the budget.json writer and its readers (the budget read, the stamped reading), deleted
@task_file n=4 path=crates/viola-state/src/lib.rs
what: the state crate's module list, losing the budget module
@task_file n=5 path=crates/viola-agent-claude/src/statusline.rs
what: the statusline payload parser, losing the rate_limits reading
@task_file n=6 path=src/cmd/hook/statusline.rs
what: viola hook statusline: instance check, capped hook stdin, malformed-json refusal, statusline command pass-through; no budget write
@task_file n=7 path=src/run/wheel.rs
what: the wrapper's wheel and the channel release method with the driver-originated release guard and its params check
@task_file n=8 path=src/cmd/release.rs
what: the CLI viola release verb and its arguments (from VIOLA_NAME), its --budget flag gone
@task_file n=9 path=src/run/send.rs
what: the channel send method and its refusal order (control-character, empty-text, human-typing, turn-running, readiness)
@task_file n=10 path=src/run/send/slot.rs
what: the send slot that applies the send refusals
@task_file n=11 path=src/cmd/send.rs
what: the CLI exit-code map of the send refusals
@task_file n=12 path=src/cmd/list.rs
what: viola list, the board scan and its --json document, no longer reading budget or thresholds
@task_file n=13 path=src/human/board/bay.rs
what: the board's BAY line in human output
@task_file n=14 path=src/human/hints.rs
what: the CLI stderr hints, losing the budget-paused hint
@task_file n=15 path=src/cmd/run.rs
what: the wrapper start (start records) and the channel method dispatch, shared by run and revive
@task_file n=16 path=src/cmd/verify.rs
what: viola verify's steps and the statusline fixture it records
@task_file n=17 path=crates/viola-core/src/lib.rs
what: the shared core types: event kinds, refusal reasons (closed enums), losing the budget-gate kind and budget-paused
@task_file n=18 path=crates/viola-state/src/snapshot.rs
what: the instance snapshot written only by its wrapper, losing budget_paused and budget_override_until
@task_file n=19 path=crates/viola-state/src/replay.rs
what: the snapshot replayed from the event log when the snapshot is unreadable
@task_file n=20 path=crates/viola-state/src/board.rs
what: the board items read by viola list from each instance
@task_file n=21 path=crates/viola-agent-claude/src/ledger.rs
what: the relied ledger rows and stamps viola verify checks, losing statusline-rate-limits
@task_file n=22 path=src/obs.rs
what: the config.json key list and its tolerant parse logging unknown keys
@task_file n=23 path=schemas/config.v1.json
what: the config.json schema, losing five_hour and seven_day
@task_file n=24 path=schemas/diag-line.v1.json
what: the diagnostics log line schema (hook-decision and send lines), losing the budget fields
@task_file n=25 path=schemas/claude-statusline-fixture.v1.json
what: the recorded statusline fixture schema, losing its rate_limits pin
@task_file n=26 path=src/bin/viola-fake-agent.rs
what: the fake agent test binary under the fake-agent feature, losing its rate-limits statusline replay
@task_file n=27 path=crates/viola-agent-claude/Cargo.toml
what: the agent crate's dependency list, chrono removed
@task_file n=28 path=Cargo.lock
what: the root dependency lockfile in the cargo-deny graph
@task_file n=29 path=fuzz/Cargo.lock
what: the fuzz workspace lockfile, audited on its own
@task_file n=30 path=Cargo.toml
what: the workspace manifest and its test table, losing cli_budget
@task_file n=31 path=fixtures/claude/2.1.287/StatusLine.rate-limits-1.json
what: a recorded statusline fixture with rate_limits, deleted
@task_file n=32 path=fixtures/claude/2.1.302/StatusLine.rate-limits-1.json
what: a recorded statusline fixture with rate_limits, deleted
@task_file n=33 path=fixtures/viola-0.1.0/snapshot.json
what: a committed viola-format fixture, a 0.1.0 snapshot with its budget fields, read by compat tests
@task_file n=34 path=fixtures/viola-0.1.0/events.ndjson
what: a committed viola-format fixture, a 0.1.0 event log with a budget-gate line, read by replay tests
@task_file n=35 path=fixtures/viola-0.1.0/retired-words.json
what: a committed viola-format fixture holding the retired words, read by tests
@task_file n=36 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene check, its walk now covering the viola-format fixtures
@task_file n=37 path=tests/cli_budget.rs
what: the Path 6 budget gate test suite, deleted
@task_file n=38 path=crates/viola-e2e/tests/path6_budget_harness.rs
what: the e2e Path 6 budget harness test, deleted
@task_file n=39 path=tests/cli_controls_not_disableable.rs
what: the census that no config, env var or flag switches off a control, losing its budget gate case
@task_file n=40 path=tests/contract_bounded_readers.rs
what: the census of bounded product readers, losing the budget readers
@task_item n=1 part=touches
The budget gate's fail-open reading of an untrusted file (decision/budget-gate-fails-open) is retired with the gate itself.
@task_item n=2 part=touches
The single-writer decisions for budget.json (decision/budget-file-single-writer, decision/budget-json-single-writer) and the failed-budget-read rule (decision/failed-budget-read-no-reading): no writer and no reader of budget.json remains.
@task_item n=3 part=touches
The budget.json contract items (contract/budget-json/resets-at, used-percentage, window) retire whole.
@task_item n=4 part=touches
The config.json threshold items (contract/config-json/five-hour, seven-day) retire; config keys are four, a stale threshold key logs parse-rejected unknown-keys and nothing refuses, which reaches the tolerant-config and bad-config-value rows.
@task_item n=5 part=touches
The home-check row naming the budget read among its checkers (decision/home-write-rule-checkers): the budget read is gone, mcp start and plugin install stay.
@task_item n=6 part=touches
The statusline hook rows (statusline sources read untrusted, payload fails silent, source read only): the hook reads no rate_limits and writes no budget file; the malformed-json refusal, capped stdin, instance check and pass-through stay.
@task_item n=7 part=touches
The channel release params and wrapper channel release method (contract/channel-frames/release-params, contract/channel-release/params, contract/wrapper-channel-methods/release): params are {from?}, any budget key is -32602 invalid-params with no wheel moved and no record; the reply is {wheel}.
@task_item n=8 part=touches
The driver release guard rows (decision/release-from-driver-guard, decision/driver-release-refused, contract/channel-release/string-from, non-string-from, contract/release-errors/release-from-driver): the guard stays first, then a non-string from, then the new budget-key refusal; their guard tests stay.
@task_item n=9 part=touches
The CLI viola release rows (contract/cli-release/from, decision/pause-release-from-viola-name): the --budget flag is gone, clap refuses it with exit 2.
@task_item n=10 part=touches
The send refusal order and its contract items (contract/send-refusal/*, contract/wrapper-channel-methods/send, contract/mcp-tools/send): the budget-paused rung leaves the order; send-refused lines carry no budget fields.
@task_item n=11 part=touches
The CLI exit-code table rows (contract/cli-exit-codes/12, contract/cli-exit-codes/21 and any row listing 11 as budget-paused): 11 is retired and never reused; an older wrapper's budget-paused reads unknown 14.
@task_item n=12 part=touches
viola list and the board rows (contract/mcp-tools/list, the board read rules): the list payload is {items, skipped} with no budget member and no budget read; the BAY line has no budget cells.
@task_item n=13 part=touches
The rule that no config switches off a control (decision/config-cannot-disable-controls, decision/controls-not-switchable and their twins): its budget gate case is retired, and its guard census lost that case.
@task_item n=14 part=touches
The bounded readers census (decision/bounded-readers-census, bounded external readers): the budget readers leave the census.
@task_item n=15 part=touches
The replayed snapshot rows (decision/replayed-snapshot-fallbacks, decision/replayed-snapshot-inert): the replay recovers links, agent_session_id and wheel alone and skips budget-gate lines of an old log.
@task_item n=16 part=touches
The fixture hygiene rows (decision/fixture-hygiene, decision/fixture-recording-scrubbed, decision/fixtures-reviewed-before-commit): the rate-limits statusline fixtures are deleted and the hygiene walk now covers the committed viola-format fixtures.
@task_item n=17 part=touches
The channel frame version and closed-type rows (contract/channel-frames/version, decision/channel-closed-types, decision/hook-security-fields-closed-enums): the budget-gate event kind and budget-paused refusal reason leave the closed enums; v stays 1.
@task_item n=18 part=touches
The fuzz lockfile rows (decision/fuzz-lockfile-audited-separately, decision/fuzz-lockfile-own-audit): fuzz/Cargo.lock lost chrono and its dependents, still audited green under cargo-deny advisories and sources.
@task_item n=19 part=touches
The channel-response invalid-params detail (contract/channel-response-detail/invalid-params, contract/channel-errors/32602): a release carrying a budget key now lands there.
@task_item n=20 part=touches
The data classes row (decision/data-classes) and any row naming budget.json as viola-home state: that file is no longer written; a stale one in an old home is read by nothing.
@task_item n=21 part=touches
The relied verify ledger rows: statusline-rate-limits is retired (relied rows 23, verify steps 23); a stamp holding the retired row is still read on the relied rows alone.
@task_fact n=1 slug=authentication-authorization
fact: The channel release method refuses any budget key (true, false, a string or null) with -32602 invalid-params and data null, after the release-from-driver guard and the non-string from check; no wheel moves and no record is written.
@task_fact n=2 slug=authentication-authorization
fact: viola release takes no --budget flag; the argument parser refuses it with exit 2.
@task_fact n=3 slug=threat-model-summary
fact: CLI exit 11 is retired (was budget-paused) and never renumbered or reused; an older wrapper's budget-paused refusal reads as unknown (14).
@task_fact n=4 slug=input-validation
fact: A config.json still carrying a retired threshold key (five_hour, seven_day) logs one parse-rejected config-json unknown-keys at its process start and nothing refuses.
@task_fact n=5 slug=input-validation
fact: Readers of budget-era state stay tolerant: a 0.1.0 snapshot reads whole skipping its budget fields and a replay skips budget-gate log lines; every own-format v stays 1 through the removal.
@task_fact n=6 slug=input-validation
fact: viola hook statusline reads no rate_limits and writes no file: it is a pass-through after its instance check, capped stdin and malformed-json refusal.
@task_fact n=7 slug=threat-model-summary
fact: viola has no usage budget gate: it reads no Claude rate limits, keeps no budget file and refuses no send for usage.
@task_fact n=8 slug=data-protection
fact: The committed viola-format fixtures (the 0.1.0 shapes and retired words) fall under the fixture hygiene walk like recorded fixtures.
@task_item n=1 part=untouched
Peer euid checks, socket directory checks, socket modes and the server verification before any frame: no change to the channel transport or its refusals.
@task_item n=2 part=untouched
Secret management: the CLAUDE credential strip, the canary artifact scan, the identity floor and no stored secrets.
@task_item n=3 part=untouched
Pinned binary placement, re-hash and plugin install rules, and nothing written under the Claude home.
@task_item n=4 part=untouched
CI workflow rules: action SHA pins, zizmor, workflow permissions, toolchain pins and the root cargo-deny policy (no crate added or bumped).
@task_item n=5 part=untouched
MCP transport over stdio, rmcp feature bans and the MCP stdin line cap.
@task_item n=6 part=untouched
verify's dialog-answering, PTY and screen-literal rules and its unclean-fixture refusal codes.
@task_item n=7 part=untouched
link and unlink, revive codes and the collision check, Hyprland recorder rules and the event log rotation and growth rows.
