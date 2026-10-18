# Outcome — 2026-10-18-compact-post-condition-ledger-row

**Outcome: surfaced.** Every gate of the plan's block is green except entry 15, the preservation guard. It reads
red on exactly the two files `scope-record.md` records (`tests/support/home.rs` companion, `tests/tui_channel_fds.rs`
in-intent), and the acceptance line "no file outside the modify set changes (the preservation guard)" is therefore
not met as written. Nothing was authored in any spec or plan. **The word (`inputs#I6`, the Viola overseer):**
«Accept, run the pass» — "both edits are recorded companion/in-intent per scope-record.md; entry 15 red on exactly
those two files is the wrap's to reconcile against the modify set (the operator accepts it at the light gate). Name
both in the report." Next: the operator pass (entries 24, 25, 26), then `/andromeda-wrap-session`.

## Gates (run dir `.andromeda/runs/2026-10-18T21-07-38-implement/`, trail `gate-2026-10-18-compact-post-condition-ledger-row.json`)
- Firing 1 (`--only 1,2,3,4,5,6,14,15,16`): `entries 26 · green 9 · red 0` — the unit gates green before any start;
  entry 6 `nextest-unit 16 passed 0 failed`.
- Live round (`--live-legs`, 2026-10-18T21:24:14Z → 21:25:47Z): `round: COMPLETE · legs fired 1/1 · red 0`;
  `stamped 2.1.287  24 pass  0 fail`. Starts spent: **6 of the founder's 12**; no second round (no red); no plugin
  start. Record: `chunks/2026-10-18-compact-post-condition-ledger-row/evidence/live-round.md`, `round-212414Z.txt`.
- Firing 3 (whole block): `green 17 · red 1 (23)` — `pre-push` red on `cli_answer
  path4_a_stamp_failing_the_dialog_rows_leaves_dialogs_to_the_human` and `tui_channel_fds
  tui_channel_fds_the_child_holds_nothing_of_violas` (both fixed, below).
- Firing 4 (whole block): `entries 26 · green 17 · red 1 (15) · recorded 0 · timeout 0 · not-run 8` — entry 13
  (integration) green in 46.58 s, entry 23 (`pre-push`) green in 84.43 s, entry 15 red as surfaced. The eight
  not-run are the five round/live entries (fired in the round) and the three operator entries.
- No gate deferred. No mutation run (the founder's cap). No matrix ref: the chunk claims no capability
  (`matrix.py show --chunk`: 0).

## Files changed
- Modified (research's list, 17): `crates/viola-agent-claude/src/screen.rs`, `crates/viola-agent-claude/src/ledger.rs`,
  `src/run/send/slot.rs`, `src/run/send/classify.rs`, `src/run/send.rs`, `src/cmd/verify/typed.rs`, `src/cmd/verify.rs`,
  `src/bin/viola-fake-agent.rs`, `src/run/version_gate.rs`, `tests/cli_verify.rs`, `tests/contract_ledger_probes.rs`,
  `tests/support/verify.rs`, `tests/cli_send.rs`, `tests/cli_fake_agent.rs`, `tests/contract_fake_agent_drift.rs`,
  `tests/contract_fixture_hygiene.rs`, `crates/viola-e2e/src/harness/run.rs`.
- New: `fixtures/claude/2.1.287/SessionStart.compact-1.json` (recorded live, copied alone), `evidence/live-round.md`,
  `evidence/round-212414Z.txt`.
- Recorded out of the lists (`scope-record.md`):
  - `tests/support/home.rs` · companion · serves step 10 · self — `dialogless_home` pinned `19 pass  4 fail` /
    `16 pass  4 fail`; the 24-row set reads `20 pass  4 fail` / `17 pass  4 fail`.
  - `tests/tui_channel_fds.rs` · in-intent · serves step 13 · self — a red met in this chunk's `pre-push`, folded
    per the standing rule; its fd-table read now retries until a consistent table (`yield_now`, a 2 s bound, no sleep).
- Scope read: `scope: clean — changed 20 · listed 18 · recorded 2 (companion 1 · mechanical 0 · in-intent 1 ·
  widening 0) · absorbed 1 · excluded 66`.

## Deviations from the plan, with justifications
1. `crates/viola-e2e/src/harness/run.rs` (step 10): the plan says to bound the `LEDGER_ROWS[18..]` statusline slice
   "to the rows it read". It was kept unbounded, so its missing-row cases now also cover the compact row (a superset;
   on Unix the slice already read five rows, on Windows two, so "the same three rows" named no slice).
2. `src/run/send/classify.rs`: besides the plan's cases, the trailing-newline unconfirmable table
   (`send_local_command_without_a_post_condition_and_a_trailing_newline_is_unconfirmable`) lost its three `/compact`
   rows — on a verified CLI `/compact` is no longer unconfirmable, and the new trailing-newline confirmed case covers it.
3. `tests/cli_verify.rs`: summaries the plan did not predict moved by the measured run — without Stop `5 pass  19 fail`
   / `5 pass  16 fail`; without screens `7 pass  17 fail` / `7 pass  14 fail`; the identity-strip hook count 38 → 39;
   the first-turn modal case's compact row now fails with the paste rows; the `--record` case lists and checks
   `compact-1`. The plan's predicted no-dialog, no-framing and tag-modal summaries held as written.
4. `src/run/send.rs`: besides the import, one doc line ("inside the same window" → "inside its window").
5. `src/bin/viola-fake-agent.rs`: its unit case caught a bug in the first cut — a `/compact` with no recorded variant
   cleared the remembered session; a SessionStart with no fixture now leaves it as it was.
6. The two recorded out-of-list files above, which turn the preservation guard red (surfaced).

## Surfaced
- **The preservation guard (entry 15) red** — the plan's modify set (and its guard's exclusions) misses two files this
  chunk needed: `tests/support/home.rs` (a shared helper whose count every `cli_answer` case reads; the literal probe's
  five files do not hold it either) and `tests/tui_channel_fds.rs` (a pre-existing race the loaded `pre-push` run met:
  `fd target: NotFound`; it passed 15/15 standalone before the fix and 5/5 after; a harness fd sampler showed
  descriptors that vanish between listing and `readlink` while the fake agent works). The race window could not be
  forced open to read red-before-green under a hold: the plan rules out a new fake-agent seam. Decision for the
  operator: accept the two additions (the guard then stands red on them for the wrap) or re-plan.
- **The transcript delta** — 5, not the "two per real-CLI verify" of security-plan §Data Protection (Accepted risk):
  a `-backup-<n>.jsonl` the CLI writes at the compaction, two session transcripts in Run B's directory, Run C, Run D
  (evidence/live-round.md). The plan names this as an expected amendment for the wrap.
- **Unmeasured:** whether the CLI fires a SessionEnd for `/compact` (the recorded set cannot show it).
- **A watch, not this chunk's red:** `run::send::budget::tests::send_budget_paused_is_refused_with_the_window_word_and_no_cursor::case_4_both_name_five_hour`
  went red once under a plain `cargo test` (libtest threads; "a send-refused line" not captured), green on three
  re-runs and in every nextest gate run (process per test). Cause not established; recorded for the wrap.

## Process hygiene
| process | started by | final state |
|---|---|---|
| `claude` 2.1.287 × 6 (print probe, Runs A–E) | a leg (the round's entry 9) | terminated — entry 10's census `green · no output`; no `.viola-verify-*` under the repo root or `/tmp` |
| harness sessions `p-cpc-smoke` (×2 firings) | a leg (entries 17–22) | terminated — cleanup `processes_gone:true`, `endpoint_gone:true` |
| harness sessions `p-cpc-fdprobe`, `p-cpc-fdprobe2` (the fd sampler) | this run | terminated — cleanup `processes_gone:true`, `endpoint_gone:true`, `home_removed:true` |
| the 18 processes `gate.py census` lists (`viola-lab` prototype bridges and their `claude` children, the overseer sessions, the desktop app's `claude`) | NOT DERIVED — none is this run's (other projects' paths and session names) | left running: not this run's — their owners |

## Decisions & corrections
- The founder's cap, relayed by the operator with this run's direction (`inputs#I5`): twelve starts, two rounds, the
  second only after a red, no plugin start, no mutation run — spent 6, one round.
- Guard refusal: the Bash guard refused one `cd` into the scratchpad (it moves the session cwd); re-issued with
  absolute paths.
- Standing rule applied: "a red met during a chunk folds into that chunk even outside its diff" (CLAUDE.md Session
  Learnings) — `tests/tui_channel_fds.rs`.
- Standing rule applied: testing.md "never `sleep` for synchronization" — the first retry used a 10 ms sleep and was
  rewritten to spin on `yield_now` against a deadline.
- Measurement correction: a 15-run `agent-run.sh run --integration --filter 'binary(tui_channel_fds)'` loop read
  `nextest-exit-4` (nothing selected) and was discarded; the binary was re-measured with direct `cargo nextest`.
