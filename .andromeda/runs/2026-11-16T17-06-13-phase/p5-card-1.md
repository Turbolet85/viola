# P5 review card 1 — 2026-11-16-granular-api-error-cause «Granular API-error cause» (viola-0.2.0, Epoch 3)

The plan to read is `.andromeda/cache/docs/2026-11-16-granular-api-error-cause/plan.md` (rendered from
`doc:2026-11-16-granular-api-error-cause/plan` rev 4). No plan stands in a chunk's folder. Editing that file changes
no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-16-granular-api-error-cause`
diffs the chunk's records.

## Goal
`turn-ended`'s `cause` grows from the one word `api-error` to the founder's nine. The CLI's StopFailure `error` is
classified by the adapter's existing map, which has no product caller today (`api_error_class` @
`crates/viola-agent-claude/src/hook.rs:253`), and only the word crosses into `events.ndjson`, `wait --json` and MCP
`wait`. The error text crosses nothing. A later CLI's `error` value outside the 13 it declares today reads
`api-error` and leaves one codes-only `parse-rejected{hook-stdin, unknown-value}` line (`inputs#I2`, the overseer's
technical answer).

## Top acceptance criteria
- Each of the CLI's 13 declared `error` words gives its mapped word, and an outside or absent `error` gives
  `api-error`. Proven by the map table and a booted case over 15 synthetic StopFailures. matrix#v2-34
- The word reaches CLI `wait --json` (`server-error`, the path5 case) and MCP `wait` (`rate-limited`, a handler case).
  matrix#v2-34
- The wrapper admits only the nine words or none. The CLI's raw words (`server_error`) and any text are refused.
- No line, snapshot or file holds the error text, `error_details` or an outside value. Two file walks prove it.
  matrix#v2-34
- An outside value leaves exactly one `unknown-value` line, codes only, and every hook line passes the schema.
- All nine still clear the running turn and read STATUS `idle`. Human output is unchanged.

## Touchpoints (11 files, none new)
`crates/viola-core/src/lib.rs` · `crates/viola-agent-claude/src/hook.rs` · `src/cmd/hook.rs` ·
`schemas/diag-line.v1.json` · `src/cmd/run.rs` · `src/run/send/slot.rs` · `crates/viola-state/tests/state_events.rs` ·
`tests/hook_events.rs` · `tests/cli_wheel.rs` · `tests/cli_list.rs` · `crates/viola-mcp/tests/handlers.rs`

## Test commands (19 entries)
- 2 lint and 12 new test entries.
- The 12 new entries were baselined on the untouched tree. Green: 3, 4, 6, 9, 11, 13 and 14; entry 14's control is
  in `p5-controls.md`. Red as predicted: 5, 7, 10 and 12 (exit 4, no tests to run, their cases not written yet), and
  8 (6 of 15 table rows).
- 2 seen-green harness entries.
- 3 operator entries: hygiene, push, and CI on ubuntu-latest.
- Four one-shot controls, each on an entry filtered to its case:
  - `cause-map` on entry 5, 300 s
  - `cause-check` on entry 8, 300 s
  - `unknown-line` on entry 7, 300 s
  - `cause-crossing` on entry 10, 600 s
- No mutation run and no live start (0 asked).

## Concretization preview — matrix#v2-34 (method integration)
- **Now:** A driver waiting on a session whose turn ends on each kind of API error reads the matching cause word of
  the nine, an error outside them reads api-error, and no event, snapshot or log line holds the CLI's error text.
- **Proposed:** For each of the 13 `error` words the CLI's StopFailure schema declares, a wrapper over the fake agent
  appends a `turn-ended` whose cause is that word's mapped word, and an outside string or an absent `error` reads
  `api-error`. The words map as: `rate_limit` → rate-limited · `overloaded` → overloaded · the five auth words →
  auth-failed · `billing_error` → billing · `invalid_request` → invalid-request · `model_not_found` →
  model-not-found · `server_error` → server-error · `max_output_tokens` → output-limit · `unknown` → api-error.
  Proven by `hook_stop_failure_names_the_cause_of_each_cli_error_word`. A parked `viola wait --json` reads the word
  (`path5_an_api_error_turn_end_clears_the_turn_with_no_pause_or_release`, server-error). MCP `wait` carries it
  unchanged (`handlers_wait_carries_a_turn_end_cause_word_unchanged`, rate-limited). No file under the home holds a
  planted `error_details`, `last_assistant_message` or outside value.
- **For the never-weaken check:** "each kind of API error" becomes each `error` word the CLI declares, driven by
  synthetic payloads. The 13 words were read from the CLI's source, identical in all 15 installed bundles from
  2.1.287 to 2.1.325, and none was measured. The reading that the real CLI fires StopFailure with them stays
  «The 2.1.325 recorded fixture set»'s (F1, ratified), as the cap's acceptance always rested.

## The directive's block (inputs#I1 item 9), verbatim where a tool printed it
- The union's per-master lines:
  - `architecture: rows 947 · packs 10 · picked 135 (64 by every draw) · standing 17 · 38523 B`
  - `security-plan: rows 603 · packs 7 · picked 93 (57 by every draw) · standing 13 · 27287 B`
  - `design-system: rows 315 · packs 4 · picked 58 (27 by every draw) · standing 28 · 15110 B`
  - `layout-templates: rows 222 · packs 3 · picked 33 (17 by every draw) · standing 0 · 6675 B`
  - `test-plan: rows 1331 · packs 14 · picked 224 (131 by every draw) · standing 56 · 52895 B`
  - `obs-plan: rows 1022 · packs 11 · picked 134 (75 by every draw) · standing 67 · 33099 B`
  - `a11y-plan: rows 235 · packs 3 · picked 33 (19 by every draw) · standing 15 · 10488 B`
  - `rows-union: 7 master(s) · picked 710 of 4675 · rule any`
- Rows read whole: every rows file was read whole, once. The sum is 184 077 B, under 250 000.
- Every cited row is in the union: yes, every one of the plan's handles is in its rows file. One handle was
  mis-sheeted at P4 (`test-plan · 5-integration-test-strategy/…` for a row standing under `integration-test-strategy`)
  and was corrected in plan rev 4.
- `[RETYPED]` lines: none. Re-spawns: none, every batch passed first try (task 7/7, pick 156/156 in 3 parts,
  history 7/7). Dead draws: 0.
- Non-zero exits, refusals and verdict lines:
  - The bash guard refused 3 calls: a `cd` out of the cwd, twice, and a `rec.py render` piped into `head`. Each was
    re-issued in the guard's form.
  - I piped `code-graph.py`'s stdout through `tail -60` against the no-filter rule. The whole result was re-read
    from its trace: 38 rows.
  - The first bundle-scan pattern read NOT FOUND on 9 of 15 versions; the pattern was blind to an older
    minification. Re-scanned, all 15 read the same 13 values. This is recorded as friction.
  - The 5 red baselines above.
  - No `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line appeared anywhere.
- Places a letter was wrong or silent:
  - The letters keep research's file lists in step with the plan only on a revision (`gate.py scope --sync`). They
    are silent for a take-up whose P4 fork adds touchpoints, as this one did: `src/cmd/hook.rs` and
    `schemas/diag-line.v1.json`. I put research rev 2 to follow the plan, and `scope lists: parsed 11 items`.
  - The P4 answer (`inputs#I2`) asked whether the notice needs more than one enum member, its emitter and its test.
    It needs one more thing: a boolean mark on `Normalised` (plan step 4) that carries the adapter's verdict to the
    hook.

## Forks and words
- P4 fork, the notice of an unmapped value: answered «One closed log word», the overseer's technical answer
  (`inputs#I2`, not the founder's).
- The nine words crossing out of the adapter is the founder's own ruling A (the CARRY). The plan's
  Expected amendments name the escalate pattern (rul-bznresq5, else the plain «Boundary widening») for the wrap.

## Beyond the plan
none

Apply? (yes / review / cancel)
