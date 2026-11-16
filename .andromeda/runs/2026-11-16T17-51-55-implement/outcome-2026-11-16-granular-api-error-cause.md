# Outcome — 2026-11-16-granular-api-error-cause

**Outcome: green** · run `2026-11-16T17-51-55-implement` · 0 fix iterations (the first block firing read 16 green, 0 red)

## Files changed (11, all on research's modify list; 0 new)
- `crates/viola-core/src/lib.rs` — `TurnEndCause` nine members, `ALL: [Self; 9]`, `as_str`, doc; `turn_end_cause_is_each_its_own_code_word` pins the nine words and refuses `server_error`, `rate_limit`, `completed`, `API Error`, ``
- `crates/viola-agent-claude/src/hook.rs` — `ApiErrorClass` / `api_error_class` retired; `pub fn api_error_cause(Option<&str>) -> (TurnEndCause, bool)`; `data_of`'s StopFailure arm writes the classified word; `Normalised.unknown_value: bool`; tables `stop_failure_turn_end_carries_its_cause_and_no_error_text` (4 rows) and `api_error_cause_maps_every_cli_error_word` (15 rows)
- `src/cmd/hook.rs` — `reject_stdin("unknown-value")` after `normalise` when marked; unit case `handle_logs_an_unknown_error_value_by_its_code_only`
- `schemas/diag-line.v1.json` — `parse-rejected` `detail` enum gains `unknown-value`
- `src/cmd/run.rs` — the appended-line table: 15 rows (nine words + absent appended; `server_error`, `rate_limit`, a number, `null`, an error text refused)
- `src/run/send/slot.rs` — row `rate_limited_turn_end`
- `crates/viola-state/tests/state_events.rs` — an `output-limit` `turn-ended` round-trip line
- `tests/hook_events.rs` — two expectations `server-error`; new booted case `hook_stop_failure_names_the_cause_of_each_cli_error_word`
- `tests/cli_wheel.rs` — `server-error`; `tests/cli_list.rs` — `overloaded`
- `crates/viola-mcp/tests/handlers.rs` — `handlers_wait_carries_a_turn_end_cause_word_unchanged`

## Deviations (with justification)
- Step 6's unit case reads the hook's lines through the module's scoped `Recorded` capture (`crate::test_support::with_scoped_default`), not a role file: a root-bin unit case opens no role file. The role-file read and its `diag-line.v1.json` validation stand in step 11's booted case (exactly one `parse-rejected` `hook-stdin` `unknown-value` line, `assert_hook_logs_valid`). The case is one `#[test]` holding two labelled literal rows (outside word → 1 line; `overloaded` → 0), so entry 7's `contains 1 passed` reads one test.
- Step 2/4: the map's "unknown, marked" verdict is the second member of `api_error_cause`'s return tuple; `normalise` sets `Normalised.unknown_value` from it for a StopFailure only. The map table carries the mark as a third column (true only for `Server_Error`).
- Scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 11 · listed 11 · recorded 0 … · excluded 227`).

## Gates (block firing 1, `gate.py run`, summary `entries 19 · green 16 · red 0 · recorded 0 · timeout 0 · not-run 3`)
1 `cargo fmt --all --check` green · 2 clippy green · 3 viola-core green · 4 viola-agent-claude green · 5 map table by name green (15 passed) · 6 run/hook/slot modules green · 7 unknown-value case green (1 passed) · 8 appended-line table green (15 passed) · 9 state_events green · 10 booted case green (1 passed) · 11 MCP handlers green · 12 MCP pass-through case green (1 passed) · 13 root integration set green · 14 fixtures probe green (last line 0) · 15 `agent-run.sh run --unit` green · 16 `agent-run.sh pre-push` green (91.57 s) · 17–19 `not run — leg operator` (the operator pass's: hygiene, push, `ci.py conclusion`).
No deferral: rust delta 10 — every gate ran.

## Controls (`gate.py control`, the tool's clone at HEAD bde97ba8; evidence `control-1.txt`, `control-2.txt`)
- `cause-map` · entry 5 · holds (removed red, exit 100) — `--timeout 300`
- `cause-check` · entry 8 · holds (removed red, exit 100) — `--timeout 300`
- `unknown-line` · entry 7 · holds (removed red, exit 100) — `--timeout 300`
- `cause-crossing` · entry 10 · holds (removed red, exit 100; all 15 causes read `api-error`) — `--timeout 600`
`control: 3 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed` · `control: 1 holds · … · clone removed`

## Smoke
skipped — no boot-path change (the plan lists no smoke); the changed hook arm ran as real `viola hook` processes under booted wrappers in entries 10 and 13.

## Matrix
`v2-34` planned → implemented, ref `tests/hook_events.rs::hook_stop_failure_names_the_cause_of_each_cli_error_word · tests/cli_wheel.rs::path5_an_api_error_turn_end_clears_the_turn_with_no_pause_or_release · crates/viola-mcp/tests/handlers.rs::handlers_wait_carries_a_turn_end_cause_word_unchanged · tests/hook_events.rs::hook_stop_failure_error_text_reaches_no_event_line_or_snapshot`.

## Surfaced / stuck
none. No red moved a plan entry.

## Process hygiene (`gate.py census --comm viola --comm viola-fake-agent --comm cargo-nextest --args projects/viola/target/debug/viola`, `census: 7 running`)
| process | started by | final state |
|---|---|---|
| 4 × `viola-lab/prototype/target/debug/viola run …` (pulse-, conductor-, escher-builder, andromeda-worker) | NOT DERIVED — another tree's build (the bridge), not this run | left running: not this run's — the operator's |
| `~/.viola/bin/0.1.0-…/viola mcp` | NOT DERIVED — the pinned product's MCP server | left running: not this run's — the operator's |
| `~/.local/viola-product/bin/viola wait viola-builder …` | NOT DERIVED — the overseer's wait on this builder | left running: not this run's — the operator's |
| `viola-lab/prototype/target/debug/viola wait escher-builder …` | NOT DERIVED — another tree's | left running: not this run's — the operator's |
| any `projects/viola/target/debug/viola` | — | none running |
This run started: the gate block's and the controls' entries, each terminated by its own entry (no survivor line printed).

## Decisions & corrections
- The Bash guard refused one `cd` into the skill's references dir (it would move the session cwd); re-issued with absolute paths.
- The phase's directive item 6 («a red that moves a plan entry stops implement BEFORE the operator pass») and the invocation's: no red met, no plan entry moved.
