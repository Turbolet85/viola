# P5 review card 1 — 2026-11-15-api-error-turn-end «API-error turn end» (viola-0.2.0, Epoch 3)

Written 2026-11-15T22:16:01Z. The plan's render is `.andromeda/cache/docs/2026-11-15-api-error-turn-end/plan.md`
(rev 4); open it in an editor. No plan stands in a chunk's folder: writing in that file changes no record until a
`put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-15-api-error-turn-end` is a
diff of the chunk's records.

## Goal
On an API error the CLI fires `StopFailure`, never `Stop`, and viola never registered it. So no `turn-ended` is
written, the running turn stands, every `send` reads 13 `turn-running`, and `wait` never returns. The chunk registers
`viola hook stop-failure`, maps it to the existing `turn-ended` kind carrying `cause: "api-error"`, and drops the
error text at the parse. The turn then ends exactly as a Stop's does. It is proven on the fake agent with synthetic
payloads; no live start remains.

## Top acceptance criteria
- One hook `turn-ended` line, `data` exactly `{"last_assistant_message": null, "background_tasks": null, "cause":
  "api-error"}` (entries 7, 8).
- With no `pause` or `release`: a parked `wait` returns `turn-ended`, the next `send` exits 0, and the wheel reads only
  its start (entry 8, `path5_an_api_error_turn_end_clears_the_turn_with_no_pause_or_release`). The board reads `idle`
  (entry 10).
- That case reads red with the `StopFailure` registration removed: the `registration` control, cap v2-16's «a test
  fails before the fix».
- A planted error text in `last_assistant_message` and `error_details` reaches no event, snapshot or diagnostics file
  (entry 7, `hook_stop_failure_error_text_reaches_no_event_line_or_snapshot`, and its `text-drop-e2e` control).
- The wrapper appends a `turn-ended` only with no `cause` or a closed word (entry 5, the `revalidation` control).

## Touchpoints (18 files modified, none new)
`crates/viola-core/src/lib.rs` · `crates/viola-agent-claude/src/{hook,ledger,lib}.rs` ·
`crates/viola-agent-claude/src/ledger/test_cases_stamps.rs` · `plugin/hooks/hooks.json` · `src/cmd/run.rs` ·
`src/run/send/slot.rs` · `src/bin/viola-fake-agent.rs` · `schemas/{diag-line,claude-fixture,fake-script}.v1.json` ·
`tests/{hook_events,cli_wheel,cli_list,hook_fail_open,security_negatives_strict_modes}.rs` ·
`crates/viola-state/tests/state_events.rs`.

## Test commands (21 entries)
fmt · clippy · check · crates unit (4) · root unit by name (5) · hook_events (6) · absence probe by name (7) ·
e2e by name (8) · cli_wheel (9) · cli_list (10) · hook_fail_open (11) · security negatives + fake-agent drift (12) ·
no recorded fixture changed (13) · the smoke trio (14–16) · `agent-run.sh run --unit` (17) · pre-push (18) · operator:
hygiene, push, `ci.py` (19–21). No mutation run. Step 14's five one-shot `gate.py control` runs follow green.
Baselines: 10 green, and 2 red as the plan predicts on an absent input (7 and 8: exit 4, no test written yet). Entry
13's both-sided controls are in `p5-controls.md`.

## Concretization preview — matrix#v2-16 (claimed after the word)
- Now: «A turn ended by an API error is logged as a turn end with its cause, the instance reads idle, and the driver's
  next send is accepted with no human step; a test fails before the fix.»
- Proposed: «On the fake agent firing a synthetic `StopFailure` payload (the CLI's API-error turn-end hook, registered
  as `viola hook stop-failure`): one hook `turn-ended` line whose `data` is exactly `{"last_assistant_message": null,
  "background_tasks": null, "cause": "api-error"}`; a parked `viola wait` returns `turn-ended` exit 0 and the next
  `send` exits 0 with no `pause` or `release`, the wheel holding only its start record
  (`path5_an_api_error_turn_end_clears_the_turn_with_no_pause_or_release`); `viola list` reads `idle`
  (`list_status_reads_idle_after_an_api_error_turn_end`); the `registration` control (the hooks.json `StopFailure`
  entry removed) reads that end-to-end case failed. The live payload on claude 2.1.325 is not read: a named exception
  owned by «The 2.1.325 recorded fixture set».»
- Never weakened: every clause of the outcome stands. «Its cause» is the one closed word `api-error` the delegate
  chose (`inputs#I2`). The proof is the fake agent's, the project's e2e layer; the live CLI's firing is the named
  exception, said in the acceptance itself.

## The directive's block (inputs#I1 point 7), verbatim where a tool printed it
- The union, per master, and its line:
  ```
  architecture: rows 939 · packs 10 · picked 279 (164 by every draw) · standing 38 · 74275 B
  security-plan: rows 597 · packs 6 · picked 156 (100 by every draw) · standing 26 · 42181 B
  design-system: rows 313 · packs 4 · picked 67 (37 by every draw) · standing 5 · 15263 B
  layout-templates: rows 221 · packs 3 · picked 39 (25 by every draw) · standing 0 · 7477 B
  test-plan: rows 1327 · packs 14 · picked 355 (218 by every draw) · standing 63 · 84064 B
  obs-plan: rows 1021 · packs 11 · picked 303 (160 by every draw) · standing 62 · 71737 B
  a11y-plan: rows 235 · packs 3 · picked 37 (23 by every draw) · standing 21 · 11164 B
  rows-union: 7 master(s) · picked 1236 of 4653 · rule any
  ```
- The rows files were read BY SEARCH, not whole: their sum is 306,161 B, past the 250,000 B bound. Every step and
  criterion's terms were searched (grep over `{specialty}-rows.md`). The seven standing lists and the seven history
  files were read whole.
- Every row the plan cites is in the union or its standing list: 19 single-line cites checked by a script (18 in
  rows files, 1, `decision/hyprland-reply-text-dropped`, in security's standing list; 0 missing), and the 3 cites
  wrapped across lines found by the earlier search (arch, security and layouts rows).
- `[RETYPED]` lines, both resolved by `check`; no re-spawn in any batch (task 7/7, pick 153/153 in 3 parts, history
  7/7, all first try):
  ```
  security-pick-p6.d3 · [RETYPED] line 58: decision/server-verified-before-any-channel-frame is no row of the pack — read as decision/server-verified-before-any-frame, the pack's row 58
  tests-pick-p13.d3 · [RETYPED] line 97: contract/coverage-failure/llvm-cov-exit-N is no row of the pack — read as contract/coverage-failure/llvm-cov-exit, the pack's row 97
  ```
- Non-zero exits and refusals, every one:
  - the Bash guard refused 4 calls (3 × a `cd` into a non-root dir, 1 × a pipeline tool's output piped into a
    filter); each was re-issued in the form the guard passes;
  - the stamp-ahead hook refused the plan's estimated `Generated:` stamp; it was re-set from `date -u`;
  - a bounded-repetition grep over the 150 MB CLI bundle passed the Bash time limit; it was stopped and replaced by a
    scratchpad python read;
  - the scope-premise splice script exited 1, fired before its input file existed; it was re-run after the Write;
  - `git -C ~/dev/projects/additional/viola-overseer` exited 128 (not a repository), so the directive was snapped
    as a no-repo copy (`inputs#I1`);
  - `gate.py show --chunk … --n 5` exited 2 (usage: `show` takes no `--chunk`); the entry's log was read instead;
  - baselines 7 and 8: exit 4, «no tests to run», the predicted red;
  - live starts 1 and 2: `claude` exited 1 each (below).
  No `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line was printed by any tool this run.
- Where a letter was wrong or silent:
  - validation.md check 4 (9) names `gate.py show … --n {n}` and is silent on its arguments: it rejects `--chunk`;
  - the take-up read `claude` on PATH as a mise shim (`/usr/bin/mise`): the letters name `claude --version` but no
    resolution step for a shimmed CLI;
  - the live-start grant's condition «nothing of his … config … changed» could not be held by start 1: the CLI
    itself rewrote `~/.claude.json` (mtime and size changed; content not read). Start 2 used a private
    `CLAUDE_CONFIG_DIR` and left it unchanged.

## Live starts (cap 2, both spent; journal `live-starts.md`)
- Start 1 (`--model viola-probe-no-such-model`): exit 1 in 1.82 s, «There's an issue with the selected model…», no
  hook fired (`SessionStart` none).
- Start 2 (a closed local port as the API base, a private config dir, a dummy key, retries 0): exit 1 in 1.12 s, «API
  Error: Connection refused…», no hook fired.
- Neither reached a started session's turn, so StopFailure's live payload is unread. This is not evidence it does not
  fire.

## Forks owed to the founder (founder-forks.md), PROVISIONAL on the delegate's word (inputs#I2)
- F1: the hook surface admits `StopFailure`, with no ledger row.
- F2: the `cause` word crossing into `events.ndjson`, `wait --json` and MCP `wait` (one word now; the granular list
  is his to give).
Both are pattern «Boundary widening» (`rul-r74hshif`, `rul-p2zl2kgl`): the wrap's full escalation.

## Beyond the plan
- CARRY onto «The 2.1.325 recorded fixture set»: record a live `StopFailure` payload inside a started session's turn
  (the connection-lost class if reachable). That entry owns the named exception: StopFailure registered with no
  ledger row and no verify probe (`inputs#I2`).
- The founder's own word owed on F1 and F2 (`founder-forks.md`), carried on the handoff for his return. The granular
  cause list is the overseer's lean for him.
- The operator's desk list gains `target/live-stopfailure-20261115/` and `target/live-stopfailure-20261115-s2/`
  (gitignored scratch dirs of the two live starts), and the note that start 1's CLI rewrote `~/.claude.json`.

Apply? (yes / review / cancel)
