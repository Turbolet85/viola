# Implement outcome — 2026-11-20-permissionrequest-question-answered

**Run:** 2026-11-20T12-23-17Z-implement · **Outcome: stuck** — stopped before `S1` on the implement directive's stop
rule and plan step 18. No live start was made; `PQ` and `PQ-2` are unspent. Nothing was committed, landed or
re-stamped beyond the chunk's working files.

## Why it stopped

Plan step 18 (`inputs#I5` item 4) names three pre-round conditions that the gate block does not read, «each the rig's own to refuse».
It says: «If the rig does NOT refuse one of them itself, implement stops before `S1` and says so: the condition becomes a plan entry
through the revision door, not a start». The directive adds: «If the rig does not refuse one of them itself, stop before `S1` and
say so.»

Read from the rig's code on the Phase B tree, for the `VR` arm that `PQ` takes, **the rig refuses none of the three**:
- **A standing rig scratch home.** Not refused. `Scratch::create` makes a fresh `<base>/<run>` for every run id. The only check
  for anything standing is `procs::census(Scope::Orphans)` (`refused: leftover`), and it reads processes, not directories.
- **A `.viola-verify-*` dir in the probe cwd.** Not refused. Nothing reads it before the start. Only `VH`'s `HomeNames` reads
  it, and `record_left` after the start.
- **An `XDG_RUNTIME_DIR` that is not owner-only.** Not refused for `VR`. Only the contained `SF` arm's `sandbox_probe` reads
  it (`refused: sandbox`).

All three are clear on the host as read at 2026-11-20T12:36:58Z: the scratch base `/tmp/viola-rig-<uid>` is 0700 with
0 entries, `XDG_RUNTIME_DIR` is 0700 and owned by the operator, and there is no `.viola-verify-*`. Details are in
`evidence/live-round.md`.

**Next:** the overseer's word through the phase's revision door. Each condition becomes a plan entry (a `round` probe
before `S1`, or a rig refusal written in code), then the round, `S1`, the after-reads, the landing and every layer
again. Nothing is committed between Phase B and the landing.

## What was done (Phases A and B, green)

**Phase A (26 rows, the product change):**
- `crates/viola-agent-claude/src/dialog.rs`: `decision_body` maps `(PermissionRequest, Question)` to `allow` +
  `updatedInput`. One helper, `answered_input`, now serves both question arms. `permission_decision` takes an
  optional `updatedInput`. The doc bullet is rewritten. Cases:
  `decision_body_question_on_permission_request_is_allow_with_updated_input` (literal + new snapshot
  `question_answered_on_permission_request`) and `decision_body_question_on_permission_request_carries_annotations`.
- `src/run/dialog.rs`: `unanswerable` is removed, so an arm-less question is held like any dialog. Cases:
  `continuation_raised_as_a_question_is_held_and_answered`, plus the rstest
  `continuation_raised_as_a_question_is_left_to_the_human_when` (`human_wheel` · `unverified_cli` ·
  `another_dialog_pending`).
- `tests/cli_answer.rs` Path 4: the question is logged once, wakes a parked `wait` and is held. `answer` by id carries
  `answers` (each question's first label) and an `annotations` note. The hook prints the `allow` + `updatedInput` body
  by literal, and a second `answer` is refused `unknown-dialog`, exit 13.
- Gate entries 1–6 are green. Entry 4 went green on its second firing; see Deviations.
- Control `pr-question-arm`: **holds**. Its own case and its annotations sibling are the two failing in the removed run
  (`evidence/control-1.txt`).

**Phase B (the 27th row):**
- `ledger.rs`: `LedgerRow::PermissionRequestQuestion` / `permission-request-question` (27th) with its words, and
  `DIALOG_PROMPT_PERMISSION_QUESTION`. `DIALOG_TURNS` is now `[_; 5]` (stem `question-permission`).
  `ProbeAnswer::PermissionQuestionFirstOption` / `permission-question-first-option` makes `ALL` `[_; 6]`. `PROBE_ANSWERS`
  is `[_; 7]`, `(Questions, PermissionRequest, 2)`, with PreToolUse 4 unmapped. `probe_body` maps the new id.
  `question_permission_answered` / `first_labels_answered` are added. `question_answered`'s PermissionRequest clause
  is scoped to its own turn.
- `src/cmd/verify/typed/runs.rs`: Run C pastes the fourth prompt last.
- Ledger unit cases (step 13): `test_cases.rs`, `test_cases_framing.rs`, `test_cases_checks.rs`
  (`check_permission_request_question_passes_on_the_recorded_shape`, the 5-row rstest
  `check_permission_request_question_fails_when`) and `test_cases_stamps.rs`.
- Every relied-count literal is at 27 (step 14): `src/cmd/verify.rs`, `tests/contract_ledger_probes.rs` (`ROW_IDS`, the
  summary; `STAMPED`/`DRIFT_ONLY` untouched, step 23 is Phase D), `tests/cli_verify.rs`, `tests/cli_verify/runs.rs`,
  `tests/cli_verify/stamps.rs`, `crates/viola-e2e/src/harness/run.rs` and `crates/viola-e2e/tests/rig_rehearsal.rs`.
- The rig (step 15): `PQ` / `PQ-2` on `Arm::VR`, `QUESTION_STAMP` = `question-stamp.json`, a
  `permission-request-question` line in `stamp_lines` (both arms), and the label-table cases.
- Gate entries 7–11 are green. Entry 10 went green on its second firing; see Deviations.
- Control `pq-row-answers`: **holds**. Its own case, `check_permission_request_question_fails_when::case_4_answers_differ`,
  is the only failure in the removed run (`evidence/control-2.txt`).

**Not run (by the plan's order or this stop):**
- The round and `S1` (entries 12–17) and the after-reads (18–21).
- The landing (Phase D, steps 22–26) and the after-landing layers and smoke (22–29).
- The operator pass (30–34).
- The stamped-home suites have not run since Phase B; the plan says they cannot pass until `S1`'s variants land.

## Deviations (each with its scope-record line)

- `src/cmd/hook.rs` · companion. `cmd::hook::tests::answer_prints_nothing_without_a_mapped_answer::case_6_wrong_event_for_the_answer`
  borrowed `question-first-option` on PermissionRequest as an unmappable pair. Step 1 maps that pair, so the case read red
  at entry 4's first firing. It now uses `plan-approve` on PermissionRequest, still unmappable, label kept (testing.md
  2026-11-15: a borrowed use is re-pointed, never deleted).
- `src/run/version_gate.rs` · companion. `stamps_verdict_needs_the_dialog_rows` pinned verified at the 26th row. It now
  steps to the 27th, and the new row joins its failing loop.
- `tests/support/verify.rs` · companion. The test-built synthetic `2.1.0` dialog set (`dialog_set`) gains three
  `question-permission-1` shapes (PreToolUse, PermissionRequest with the same input, PostToolUse with the first label).
  Without them every `cli_verify` all-pass case reads the new row `fail`. **This is a test-owned scratch set written
  at run time. It is not a committed recorded set, and the founder's Q3 option 4 (no synthetic variants in 2.1.287 / 2.1.302 / 2.1.325)
  is not crossed.** The overseer should confirm that reading.
- The six plan-touchpoint files outside research's lists: `test_cases_checks.rs` (step 13), `harness/run.rs` (step 14),
  `rig/mod.rs` (step 15), `rig_rehearsal.rs` (step 14), `src/cmd/verify/typed/runs.rs` (step 9) and
  `tests/cli_verify.rs` (step 14). Each is recorded as in-intent or companion.
- `src/cmd/verify/typed/runs.rs`: the permission prompt's `settle` stays where it was, as step 9 says. **A settle after the
  fourth (last) prompt's Stop is added too**, so the function's own invariant still holds: «once the last Stop is
  captured the screen settles, so the kill never hangs up a Stop hook still exiting». Step 9 named only the
  permission settle.
- `crates/viola-e2e/src/rig/mod.rs` `FLOOR_ROWS` (the `S1`/TS arm's `step1-floor`): the new row joins (22 → 23). The floor
  is defined as «every relied row but four» (`the_floor_rows_are_every_relied_row_but_four`), the new row needs no
  transcript, and that test read red at entry 10's first firing. Plan step 15 did not name the floor.
- Step 13's case name: the `pq-row-answers` control's case is named as nextest prints it,
  `check_permission_request_question_fails_when::case_4_answers_differ` (an rstest row). The plan's table wrote
  `…_fails_when_the_answers_differ`.
- Summary literals swept by their sums (testing.md 2026-11-19/20), each moved by whether Run C reached the new turn in
  that case. `cli_verify/runs.rs`: 8/19 · 22/5 · 23/4 · 22/5 · 10/17 · 20/7 and the Stop count 9 → 10.
  `cli_verify/stamps.rs`: 7/20 and the hook count 39 → 44. `cli_verify.rs`: 26 pass 1 fail. These are untested until the landing.

## Process hygiene

| process | started by | final state |
|---|---|---|
| gate entries 1–11 and their cargo/nextest trees, the two controls' clone firings | this run (through `gate.py`) | terminated (each call exited; both clones removed) |
| the 15 processes `census` lists (`claude` sessions, the overseer sessions, the prototype's and the product's `viola`, `~/.viola`'s `viola mcp`) | NOT DERIVED — none is this run's: other trees' builds, the host's own sessions and the pinned product | left running: not this run's — their owners' |

No rig, no `viola-live-rig` and no fake agent was left by this run. Census trail: the `census` record in
`.andromeda/runs/2026-11-20T12-23-17Z-implement/gate-2026-11-20-permissionrequest-question-answered.json`.

## Decisions & corrections

- The directive's stop rule fired: «If the rig does not refuse one of them itself, stop before `S1` and say so». Evidence:
  `evidence/live-round.md`, read from `rig/mod.rs`, `rig/isolate.rs` and `rig/procs.rs`.
- Bash guard refusals: three (a `cd` into a subdirectory twice, a `cat` heredoc with a file target once). Each was re-issued
  once with absolute paths or the Edit tool.
