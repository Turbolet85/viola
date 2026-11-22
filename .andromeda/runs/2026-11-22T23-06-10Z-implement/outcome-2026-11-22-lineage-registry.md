# Outcome — 2026-11-22-lineage-registry

**Run:** 2026-11-22T23-06-10Z-implement · **Outcome:** green · **Next:** the operator pass (entries 16–22: the
smoke boot / status / cleanup, `pre-push`, hygiene, the push, the `ci.py conclusion` read), then the wrap in a fresh
window.

## Files changed
34 (`gate.py scope`: `scope: clean — changed 34 · listed 28 · recorded 6 (companion 3 · mechanical 0 · in-intent 3 ·
widening 0) · absorbed 0 · excluded 237`). New: `tests/contract_lineage_label.rs`.

## Gates (the block, last whole firing)
`entries 22 · green 14 · red 0 · recorded 1 · timeout 0 · not-run 7` — entries 1–11 and 13–15 green, 12 (the
hostwatch probe) recorded, 16–22 `not run — leg operator`. Earlier reds, each recorded before its re-fire: entry 1
`cargo fmt --all --check` red once (a blank line a sed deletion left in `tests/cli_list.rs`); entries 14
(`run --integration`) and 15 (`run --e2e`) red once — four start-record counters outside the plan's named readers
(`tests/hook_events.rs` `hook_prompts_arrive_normalised_with_their_origin`,
`hook_every_registered_event_lands_as_one_line_of_its_kind`,
`hook_session_end_appends_directly_only_while_the_log_is_free`; `tests/path1_run_start.rs`
`path1_run_start_one_home_in_order`), each an assertion on a value, fixed as companion edits. After a later test-only
edit (`src/human/board/rows.rs`, the tree's piped/terminal witness) entries 1, 2, 5, 7 re-ran green.

Controls (`evidence/remove-the-guard.md`): `control: 3 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone
removed` (last-wins, name-check, revive-keep) and `control: 1 holds …` (census-bite: a reader planted in
`src/run/links.rs` read red, `"src/run/links.rs:23: snapshot.lineage.is_some()"`).

No `gate.py guard` entry in the block.

## Host
hostwatch `--last 15` before the layers: CONTENDED (escher's build alive in the stalled seconds) — stopped per
`inputs#I4`; resumed on the overseer's quiet-window word (`inputs#I5`, neighbour builds stopped 23:28:02Z): `read
--from 2026-11-22T23:28:02Z` QUIET before the layers (0 s stalled) and before the re-runs.

## Deviations (with justification)
- The tree keeps the board's own order (name order from `board::read`) rather than re-sorting in `rows.rs`: a sort
  reddened the existing `board_row_style_a_live_row_bolds_its_name_and_ambers_the_dialog_word` (input `[builder,
  ab]`), which step 11 keeps unchanged; the `board_tree_*` inputs are given in name order as the board gives them.
- The census reads `.parent` as a field access only (not `.parent(`): the plan's token matches `path.parent()` in
  `crates/viola-channel/src/endpoint.rs`; it also cuts `#[cfg(test)]` items, whose snapshot literals now hold
  `lineage: None` (the struct sweep), and leaves `test_`-stemmed files out.
- A fourth control (census-bite) beyond the plan's three, for the directive's «the census is shown to bite».
- The self-parent usage error is built by `run::checked` (a `clap::Error` of kind `ArgumentConflict`, exit 2, before
  anything is read); the parse test chains it after clap.
- Scope record: `src/human/board/test_support.rs` · companion · serves `src/human/board/rows.rs` · self;
  `tests/support/home.rs` · in-intent · serves step 18 · self (`boot_flagged`, `beside`: two wrappers in one home and
  `viola run` flags before `--`); `tests/hook_events.rs` · companion · serves step 13 · self;
  `tests/path1_run_start.rs` · companion · serves step 13 · self.

- Harness fix in the operator pass: the first smoke status read both harness items with `"parent":"viola-builder"`
  (this session's inherited name); `crates/viola-e2e/src/harness/supervise.rs` now removes `VIOLA_NAME` / `VIOLA_DIR`
  / `VIOLA_BIN` on its wrapper spawn (in-intent · serves step 16 · self), witnessed by
  `harness_session_boots_reports_logs_and_tears_down` (`crates/viola-e2e/tests/harness_lifecycle.rs`, in-intent ·
  serves step 16 · self) and the `harness-unwrapped` control (holds). Entries 1, 2, 11 re-ran green; smoke 16–18
  re-fired green with no `parent`.

## Surfaced / stuck
none.

## Process hygiene
| process | started by | final state |
|---|---|---|
| gate entries' cargo / nextest / viola / fake-agent trees | this run | terminated — `gate.py census --comm viola-fake-agen --comm cargo-nextest --args projects/viola/target/debug/viola --args andromeda-control`: `census: none running` |
| control clones `~/.cache/andromeda-control/e9a84550`, `568a5f20` | this run | removed by the tool (`clone removed`) |

## Decisions & corrections
- Host rule fired: hostwatch CONTENDED with escher alive in the stalled seconds → stopped before the whole layers;
  the overseer's word `inputs#I5` set the quiet window and the `--from` read form (never `--last 15`).
- Recurrence of testing.md 2026-10-04 (start-record count move): four counters outside the plan's readers read red only
  at the whole layers — the plan's sweep named two files.
- Guard refusals: the Bash guard refused a `cat` heredoc to a file (re-issued as a Write-tool script), a `cd` into the
  skill's references dir (read by the Read tool) and a pipe of `rec.py find` into grep (re-run bare).
