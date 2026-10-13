# Outcome — 2026-10-13-epoch-4-cleanup

**green + smoke** — the chunk's own gates green (firing 2: 21 green · 0 red · 3 `leg = 'operator'` not run), the
smoke green. The operator pass has run (`evidence/operator.md`): `pre-push` re-fired green, hygiene clean after two
phase-run-dir lines were rewritten, the pre-CI commit `3b75dc7` pushed, and the CI read `verdict: green` on
ci#38375200217 (15/15), where the Windows witness
`wipe_scratch_takes_an_instance_tree_the_check_accepts_where_a_plain_dir_does_not` passed on `test (windows-2025)`
(job 115221715737). Next → `/andromeda-wrap-session`.

## Plan and scope agree on the split
Read from disk before any edit (the phase window was auto-compacted once): plan.md's Goal takes the entry's three
parts (the Windows baseline, the two splits, the Windows-target clippy) and states the fourth (the 47 survivors)
leaves for an entry of its own; scope.md §1 (rewritten after the founder's "Делить на две", the phase run dir's
`relay-2.md`) says it "leaves this chunk with the split, owed to the entry #1 mints". Only residue: scope.md's
"Surfaces touched" still lists the survivor files, as the survey the new entry carries; no file of it was edited.

## Files changed
- Modified: `crates/viola-e2e/src/harness/run/mutants/scratch.rs` (`wipe` re-creates the scratch through
  `viola_state::fs::create_private_dir`; the `#[cfg(windows)]` witness
  `wipe_scratch_takes_an_instance_tree_the_check_accepts_where_a_plain_dir_does_not`), `crates/viola-e2e/Cargo.toml`
  (the `viola-state` path dependency), `Cargo.lock` (the edge, no crate), `src/human.rs` (root: shared writers, `mod`
  lines, re-exports), `src/run/send.rs` (root: `Ctx`, `send`, the refusal-order, text-refusal, gate-wait,
  failed-paste, internal and link cases).
- New: `src/human/{mirror,escape,board,wait,verbs,hints}.rs`; `src/run/send/{classify,slot,params,budget,
  test_support,relabel}.rs`.
- Evidence: `viola-0.1.0/chunks/2026-10-13-epoch-4-cleanup/evidence/split-test-names.txt` (223 lines, step 1, on the
  untouched tree) and `evidence/sizes-before.md` (`src/run/send.rs` 2088, `src/human.rs` 1064 code lines).
- Sizes after (tokei code): `src/run/send.rs` 714, `slot.rs` 381, `test_support.rs` 362, `classify.rs` 267,
  `budget.rs` 200, `relabel.rs` 185, `params.rs` 45; `src/human.rs` 135, `board.rs` 450, `hints.rs` 147,
  `mirror.rs` 144, `wait.rs` 101, `verbs.rs` 94, `escape.rs` 56.

## Deviations from the plan, with their justifications
1. **One more phase file, `src/run/send/relabel.rs`** (scope record: `- src/run/send/relabel.rs · in-intent · serves
   step 5 · self`). Step 5's own clause: after the planned split the root read 889 code lines; the prompt-match and
   relabel cases (nine tests: the match, the harness relabel, the trailing-newline and inner-CR confirmations and
   claim-nothing cases, the different and unsent prompts, the no-flight prompt) moved there; the root reads 714.
2. **`mod relabel;` is declared without `#[cfg(test)]`**; the file holds only a `#[cfg(test)] mod tests`. Firing 1
   read entry 6 red (`orphaned module relabel`): `cargo modules orphans` analyses without `cfg(test)`, so a
   cfg-gated declaration is absent to it. `test_support.rs`, declared `#[cfg(test)]` as step 5 says, was not
   flagged by the same tool; why was not measured.
3. **`PasteFn` is not re-exported at `run::send`**, though step 5 names it: no file outside the module names it
   (`grep` over `src`), and an unused `pub(crate) use` fails `-D warnings`. Its tests reach it through
   `slot.rs`.
4. **The human tests' shared writers** (`Recorder`, `Broken`, `one_write`), which step 4 does not place, stay in
   the root's `human::tests` at `pub(super)` (fields of `Recorder` too), and each sibling's tests import them through
   `super::super::tests` — the precedent `crates/viola-e2e/src/harness/run/mutants/scratch.rs` already uses.
5. **Visibility** widened only to `pub(super)`, only where a sibling reads the item: in `send` the `SendSlot` fields
   the root's `send` reads, `flight`, `confirm`, `InFlight` and its fields, `Flight` and its `send`, `Match`,
   `Confirms`, `classify`, `parse`, `Reserved` and its field, `budget_paused`; the fixtures in `test_support.rs` the
   phase tests call (two the regex widened and nothing outside reads, `NumberFields` and `Lines`, were narrowed back
   before firing 2). Nothing went past `pub(crate)`.
6. **Bodies moved by line slices, byte-identical**, with two mechanical exceptions: `test_support.rs` sits one
   indent level shallower than the test module it came from, and rustfmt re-wrapped four lines there
   (`verified_gate`'s and `poisoned_gate`'s `let`, `HARNESS_TURN`, `confirmed`'s signature); unused imports the
   glob scaffolding left were removed by `cargo fix`. A multiset read of trimmed lines, HEAD against the split files,
   shows only visibility, imports, `mod` scaffolding and those re-wraps.
7. **The root `src/run/send.rs` keeps the original import block**; the phase files read it through `use super::*`
   rather than repeating it.
8. **Test placement where step 5 left a choice**: the slot file takes the closed-send, window-expiry,
   second-in-flight, wait-feed signal, turn-start, turn-end, after-a-confirmed-send and confirm-window cases; the
   `Setup` enum stays in the root beside its one user, `send_refusal_order`; `send_writers_return_the_writer_error`
   (it also asserts `write_answered`) sits in `human/mirror.rs`.

## Gates
- Firing 1: 20 green · 1 red (6, the orphan gate, deviation 2) · 3 operator not run.
- Firing 2, after the fix: 21 green · 0 red · 3 operator not run — fmt; workspace clippy; the Windows-target clippy;
  G1; the lint probes; the orphan gate; `run --unit` (nextest-unit 1972 passed); the scratch cases (`package(viola-e2e)
  & test(/wipe_|prepare_|scratch_/)`); the split-names diff against step 1's record (no line lost or gained); sizes
  (`last line 0`); no print macro or `#[allow]` in the human siblings; the preservation guard; the lockfile (no
  `+name =`); `cargo deny check`; the smoke session (cleanup · boot · status · G2 · G4 · cleanup); `pre-push`
  (`ok:true`, stage `linux-tests`, coverage 2406 passed, playwright 1, gate no breaches).
- No deferral. No `watch:` subject. No matrix write: the chunk claims no capability (`matrix.py show`: 0).
- The Windows witness is compiled out on this host (`#[cfg(windows)]`); only entry 24's CI read runs it.

## Smoke
Fired (boot path: the wrapper's `send` handler moved). Driven by hand once after P2: `cleanup` → `boot --session
p-e4c-p3 --instance builder` (ok, wrapper and child pids) → `status` (`state:"ready"`) → `cleanup`
(`processes_gone` and `endpoint_gone` true, home removed).

## Surfaced
None.

## Process hygiene
| process | started by | final state |
|---|---|---|
| harness session `p-e4c-smoke` (supervisor, wrapper, fake agent), ×2 | this run (gate entries 16, both firings) | terminated (entry 20: `processes_gone`, `endpoint_gone` true) |
| harness session `p-e4c-p3` (supervisor, wrapper, fake agent) | this run (P3, by hand) | terminated (`processes_gone`, `endpoint_gone` true) |
| `pre-push` children (coverage, playwright) | this run (gate entry 21, both firings) | terminated (exited with the entry) |

Measured after P3 against the host's process list by `/proc/<pid>/exe` under the repository root: no process.

## Decisions & corrections
- The operator's direction at this run's invocation: read plan.md and scope.md from disk and confirm the split before
  any edit (done, above); no live `claude` start, no mutation run, witness run or workflow dispatch (none made);
  journal each long gate run in the run dir before waiting on it (`journal.md`, firings 1 and 2); run the operator
  pass with the `ci.py` conclusion read.
- Guard or hook refusals: none.
