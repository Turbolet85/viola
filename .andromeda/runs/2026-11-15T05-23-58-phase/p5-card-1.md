# P5 review card 1 — revision of 2026-11-14-status-line-interception-removed (plan rev 5)

The word (`inputs#I8`, the delegate overseer for the operator) approves the one change if every check passes, and asks
for this card only if one fails. One check does not pass as the word lets it run.

## The change (rev 4 → rev 5)

The five test names become neutral at all three of their sites each (the step, the nextest `contains` atom and the
acceptance criterion):

| was | now |
|---|---|
| `snapshot_of_a_status_line_era_writer_reads_whole_skipping_its_command` | `snapshot_holding_the_retired_command_reads_whole_skipping_it` |
| `snapshot_written_holds_no_status_line_key` | `snapshot_written_holds_no_retired_snapshot_key` |
| `verified_reads_a_stamp_holding_the_status_line_rows_on_the_relied_rows_alone` | `verified_reads_a_stamp_holding_the_two_retired_rows_on_the_relied_rows_alone` |
| `relied_rows_hold_no_status_line_row` | `relied_rows_hold_no_retired_row` |
| `hook_statusline_word_is_an_unknown_event_and_writes_nothing` | `hook_retired_word_is_an_unknown_event_and_writes_nothing` |

There is also one added Implementation-notes bullet, which SKILL.md §Revision item 4 requires of every revision: the
steps already done (1–11, part of 12), with their evidence (implement's outcome file), step 3's neutral keys
(`inputs#I6`), and the re-entry point (step 12, then 13). No other plan line moved.

The plan's render to open: `.andromeda/cache/docs/2026-11-14-status-line-interception-removed/plan.md`. No plan stands
in the chunk's folder. Writing in that file changes no record until a `put`, and
`python -X utf8 {tools_dir}/rec.py diff --root . 2026-11-14-status-line-interception-removed` diffs the chunk's
records.

## Checks

- 4 (0) fence parses, 29 entries · planlint 0 hits (12 (d) lists two pre-existing «unchanged» claims, lines 61 and 357,
  which the renames do not touch, read and holding) · 10 citations clean (I5 uncited, information) · research lists
  parsed 71 · 2: the deleted paths of `Files to modify` are the deletions research marks and the stopped run made,
  named in the revision bullet · 7: graph trace in the take-up dir · val-1: aligned (names only).
- **4 (9) — NOT RUN, owed.** Entries 11 and 12 (`cargo nextest run -p viola-state -p viola-agent-claude …` and
  `cargo nextest run -p viola --features fake-agent --bin viola --test hook_fail_open --test cli_instance_state …`)
  changed in their `contains` atoms. `gate.py baseline` reads them `new (key)`, and §Revision item 4 has every changed
  entry baseline-run on the tree as the stopped run left it. Each run is a cargo build and test run, and the word says
  «no build». Expected readings, not yet measured: 11 red, with the four renamed cases absent under their new names
  (the red the plan predicts); 12 red at compile, because `tests/support/mod.rs` still names the deleted module (this
  chunk's own unfinished step 12).

Options:
- **fire 11 and 12 now (recommended)** — two baseline runs through `gate.py run --entry`, their readings written as
  each entry's `baseline`, then the checks re-run whole. It is the check's own letter, and «no build» reads as no
  product work in this revision; the runs build the existing test targets and change nothing.
- **keep rev 4's baselines** — the take-up's readings stand, and this card records that 4 (9) was waived on the
  word. Implement's red-before-green run is then the only reading of the new names.

## Capability (matrix refs)

v2-05 stays claimed by this chunk. Its acceptance is re-claimed with the two renamed test names it cites, at step 4
after the word, with a dated note:

- `hook_statusline_word_is_an_unknown_event_and_writes_nothing` → `hook_retired_word_is_an_unknown_event_and_writes_nothing`
- `relied_rows_hold_no_status_line_row` → `relied_rows_hold_no_retired_row`

Every other word of the acceptance stays, the identifier probe included. This is not a weakening: the same assertions
hold under the new names.

## Beyond the plan

none

Apply? (yes / review / cancel) — with `yes`, name the 4 (9) option: fire 11 and 12, or keep rev 4's baselines.
