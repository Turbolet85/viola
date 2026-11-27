# Outcome — 2026-11-27-epoch-5-cleanup-2

**Outcome:** green. The gate block ran green on its first firing: 17 green · 0 red · 3 recorded · 7 not run (the
operator entries 21–27). No fix iteration. This run stopped before the operator pass, as the invocation directed.
Next → the OPERATOR PASS (entries 21–27 by number: the smoke trio 21 boot · 22 status · 23 cleanup, 24 pre-push,
25 hygiene, 26 push, 27 CI), then `/andromeda-wrap-session`.

## Files changed
- Modified: `src/cmd/up.rs` · `crates/viola-pty/src/pump.rs` · `src/cmd/run.rs` · `src/run/window.rs` ·
  `src/trust_record.rs` · `src/cmd/mod.rs` (recorded companion).
- New: `crates/viola-pty/src/pump/restart.rs` · `crates/viola-pty/src/pump/test_cases.rs` ·
  `crates/viola-pty/src/pump/test_cases_paste.rs` · `crates/viola-pty/src/pump/test_cases_respawn.rs` ·
  `src/cmd/run/test_cases.rs` · `src/cmd/run/test_cases_start.rs`.

## Deviations, with their justifications
1. **Step 5: where the respawn helpers live.** The step puts `killable` … `pumped` (lines 1739–1880 at `509eeca1`)
   in `pump/test_cases.rs`. Built that way, the root file read 854 tokei code lines, against the step's own "each
   file at most 800" and the `sizes` acceptance (scratchpad dry runs `dry-plan` vs `dry-local`). Those seven helpers
   are used only by the cases of 1882–2316 and by each other (scratchpad `items.py` over the base module). So they
   stand in `pump/test_cases_respawn.rs`, written once, above those cases. Files now read 724 · 182 · 525.
2. **Step 5: the paste file holds one case named `pump_…`.** `pump_returns_with_the_writer_blocked_on_a_child_that_never_reads`
   lies inside 1547–1740, the range the step gives the paste file, so it moved with that range and kept its name. It
   reads `pump::tests::paste::pump_returns_with_the_writer_blocked_on_a_child_that_never_reads`.
3. **Step 7: `cmd::run` widened as well.** `WindowAsk` at `pub(crate)` is still unnameable from `crate::run::window`
   while `cmd`'s `mod run;` is private. `src/cmd/mod.rs:15` now reads `pub(crate) mod run;`. Scope-record line:
   ``- `src/cmd/mod.rs` · companion · serves step 7 · self``.
4. **Steps 7–8: the moved bodies reach themselves by a self-import.** `open_window` calls `run::resolve_program`
   and `run::window::window_step`, and `trust_word` calls `trust_record::…`. Their new homes import `crate::run`
   (`src/run/window.rs`) and `crate::trust_record` (`src/trust_record.rs`), so the bodies stay byte-for-byte.
5. **Step 11: a second comparison rule.** The record's rule 1 is the plan's (all whitespace removed; plus the
   visibility words of steps 3 and 7). It read 3 items as differing, each by one trailing comma that `cargo fmt`
   drops when the dedent lets a wrapped list fit one line: `pump_forwards_a_resize_that_lands_before_its_first_look`,
   `methods_wheel_params_they_cannot_take_are_invalid` and `first_snapshot_of`. Rule 2 (also drop a comma before
   `)`, `]` or `}`) applies to those three alone, and each is marked in the record. The `bodies` entry reads 0
   ` differs` lines because of that rule; without it, it would read 3.
6. **Step 11: what was not compared.** `upped`'s split (step 1) and `Pumping::next_request` (step 4) are
   extractions, not whole moves, so the record names them and compares neither.

## Symbols / APIs
- `viola_pty`'s public names are unchanged: `pump.rs` re-exports `RESPAWN_REAP_BOUND`, `Respawn` and
  `RespawnHandle` from the private module `pump::restart`.
- New, private: `cmd::up::{select, plan, act}`, `Pumping::next_request` (returns `ControlFlow<PumpEnd,
  Option<RespawnRequest>>`). Moved: `run::window::open_window` (`pub(crate)`) and `trust_record::trust_folder`
  (`pub(crate)`); `trust_record::trust_word` stays private. `cmd::run::start` is the sole caller of each moved
  function (graph impact, research).
- Widened: `cmd::run::WindowAsk` and its two fields `pub(super)` → `pub(crate)`; `cmd`'s `mod run` → `pub(crate)`;
  in `pump::restart`, `RespawnRequest`, `RespawnHandle::lock`, `Pumping::respawn` and `drain` take `pub(super)`.
- The `trust-record` log line's `target` reads `viola::trust_record` (was `viola::cmd::run`); its event and detail
  word are unchanged. `tests/run_cli.rs:137-139` reads only the prefix `viola`.

## Schema / config
none

## Gate block (`gate-2026-11-27-epoch-5-cleanup-2.json`)
1 fmt green · 2 clippy green · 3 orphans green · 4 root bins units green · 5 viola-pty green · 6 pty-cases green
(33) · 7 pty-total green (49) · 8 run-cases green (84) · 9 up-cases green (23) · 10 root-total green (1359) · 11
profraw-name green (1) · 12 sizes green (0) · 13 cx green (0) · 14 up-cx recorded «upped 5.0 · select 10.0 · plan
6.0 · act 9.0» · 15 pty-cx recorded «write_paste 12.0 · <anonymous> 8.0 · run 8.0 · next_request 10.0» · 16 bodies
green (exit 1, last line 0) · 17 host read recorded («verdict QUIET - 0 s stalled on IO»; /tmp 15 % used) · 18
`run --unit` green · 19 `run --integration` green · 20 `run --e2e` green · 21–27 not run — leg operator.

No `gate.py guard` entry is in the block. No deferral (every changed file is Rust, and the plan carries no
`defer`).

## Smoke
Not driven here. The boot path changed (the start shared by `run`, `revive` and `adopt`), and its smoke is the
plan's operator trio, entries 21–23, held for the operator pass.

## Surfaced / stuck
none

## Process hygiene
| process | started by | final state |
|---|---|---|
| the gate entries' cargo / nextest / harness trees | this run | terminated (entries exited; census after the block shows no `projects/viola/target/debug/viola` and no `cargo`) |
| 8 census rows (`foot` ×5, the prototype's `viola run escher-builder`, `~/.viola/bin/…/viola mcp`, the product's `viola wait viola-builder`) | NOT DERIVED — none started by this run (the overseer pair's sessions and the pinned product) | left running: not this run's — their owners' |

Census: `gate.py census --comm viola --comm viola-fake-agent --comm cargo --comm foot --args
projects/viola/target/debug/viola` → `census: 8 running` (the trail `census-2026-11-27-epoch-5-cleanup-2.json`).

## Decisions & corrections
- The invocation's directives, given while the operator is away: three ordered groups, each closed by its own crate
  layer; the moved-bodies record names every moved function and test and ends on its count line, never empty; a
  red with no failed assertion is read against hostwatch first, never re-run for green; every time from `date -u`;
  no scheduled wake; stop before the operator pass. All were kept: each group's layer ran green before the next
  group began, the record ends `moved: 159 equal · 0 differ`, no red occurred, and no wake was set.
- Guard refusals, each re-issued once in the form the guard passes: a `cd` into the skill's references dir
  (re-read with Read), and a `cat` heredoc to a scratch file (re-done with the Edit tool).
- Sweep hazard: a whitespace-free text comparison of code that `cargo fmt` reflowed still differs by the trailing
  commas it drops. The measure needs that rule named beside it (deviation 5).
- The invocation's directive is snapped as `inputs#I4` (step implement).
