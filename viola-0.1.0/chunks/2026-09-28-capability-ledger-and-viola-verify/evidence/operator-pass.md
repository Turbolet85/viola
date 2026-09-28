# Operator pass — 2026-09-28-capability-ledger-and-viola-verify

Driven by /implement on the operator's word ("At the operator pass, stop after the live verify --record and
before the pre-CI commit, so the overseer can review the scrubbed fixture files"). Entries 30-34 are NOT run:
the pass stops here for the overseer's review of the recorded set.

## Entry 28 — `claude --version` (probe, leg operator)
- run: `claude --version`, exactly as the plan writes it; `claude` resolves to the npm shim on PATH
- exit: 0
- atoms: `exit 0` ✓ · `contains (Claude Code)` ✓ — stdout `2.1.283 (Claude Code)`
- verdict: green

## Entry 29 — the live recording (e2e, leg operator, once)
- run: `cargo build --bin viola && target/debug/viola --home "$HOME/.viola-record" verify --record fixtures/claude`,
  exactly as the plan writes it (one print-mode Haiku probe)
- exit: 0
- atoms: `exit 0` ✓ · `contains 6 pass  0 fail` ✓
- stdout, whole:
  ```
  [01/06] shim-resolution claude resolves to a real executable  pass
  [02/06] spine-hooks spine hooks fire through the plugin dir  pass
  [03/06] session-start-fields SessionStart carries session_id and source  pass
  [04/06] prompt-verbatim UserPromptSubmit carries the prompt as sent  pass
  [05/06] stop-message Stop carries last_assistant_message  pass
  [06/06] largest-hook-payload every hook payload fits the frame cap  pass
  stamped 2.1.283  6 pass  0 fail
  ```
- stderr: empty
- artifact `fixtures/claude/`: fresh — every file written by this run, after the build (15:11:33 local); the probe
  dir under the record home's `ledger/probes/` is empty after the run
- recorded set, `fixtures/claude/2.1.283/`:

  | file | bytes | fields |
  |---|---|---|
  | `SessionStart.default.json` | 290 | session_id · transcript_path · cwd · hook_event_name · source |
  | `UserPromptSubmit.default.json` | 415 | session_id · transcript_path · cwd · prompt_id · permission_mode · hook_event_name · prompt |
  | `Stop.default.json` | 438 | session_id · transcript_path · cwd · prompt_id · permission_mode · hook_event_name · stop_hook_active · last_assistant_message · background_tasks · session_crons |
  | `SessionEnd.default.json` | 337 | session_id · transcript_path · cwd · prompt_id · hook_event_name · reason |

  Every `cwd` reads `~\.viola-record\ledger\probes\<pid>` and every `transcript_path`
  `~\.claude\projects\C--Users-<user>--viola-record-ledger-probes-<pid>\<session>.jsonl`: the home prefix scrubbed to
  `~`, the user word to `<user>`. The ids are the CLI's own UUIDs and the probe's pid.
- verdict: green

## Entry 30 — the claude-fixture hygiene walk over the recorded set (integration, leg operator)
- run: `bash scripts/agent-run.sh run --integration --filter 'binary(=contract_fixture_hygiene)'`
- exit: 0 · atoms: `exit 0` ✓ · `contains "cmd":"run","ok":true` ✓ — 26 passed, `claude_fixtures_pass_hygiene` PASS
- known positive (the walk read the set, not an empty dir): a planted
  `fixtures/claude/2.1.283/Planted.default.json` holding a drive path turned the same run red (exit 1, the one
  failure `claude_fixtures_pass_hygiene`); the plant was removed at once and the set is the four files again
- verdict: green

## Entry 32 — `bash scripts/agent-run.sh pre-push` on the uncommitted tree (integration, leg operator)
- exit: 0 · atoms: `exit 0` ✓ · `contains "cmd":"pre-push","ok":true` ✓ · `contains "stage":"union"` ✓
- linux coverage 942 passed · playwright 1 passed · gate no breaches; windows coverage 956 passed · gate no breaches;
  mutation legs ubuntu-latest and windows-2025 each `counted`, 222 tested, union gate no breaches
- verdict: green

## Entry 31 — `gate.py hygiene` (probe, leg operator), after the pre-push
- exit: 0 · atoms: `exit 0` ✓ · `contains hygiene: clean` ✓ — `hygiene: clean — read 30 (runs 29 · evidence 1) ·
  trails 12 not read · binary 0 not read by P1`, every predicate's control fired on its known positive
- verdict: green

## Overseer review — approved (2026-09-28 15:14, the Viola overseer)
Recorded verbatim from the overseer's word: "Overseer review of fixtures/claude/2.1.283 (4 files), 2026-09-28 15:14:
approved. Every path is scrubbed to ~ and <user>. A search for the user name, @, gmail, sk-ant, token and C:\Users
hits 0 of the 4 files; its control, the <user>/viola-record literal, hits 4 of 4. The only values are the CLI UUIDs,
the probe prompt and ok."
