# Report — 2026-10-13-epoch-4-cleanup

**Chunk:** Epoch 4 cleanup — the Windows mutation leg's root-package baseline unblocked at its cause, `src/run/send.rs` and `src/human.rs` split by phase, a Windows-target clippy gate entry (the 47 survivors leave for "Epoch 4 survivors", the founder's split, `inputs#I2`)
**Date:** 2026-10-13
**Commits:** `3b75dc7 chore(2026-10-13-epoch-4-cleanup): operator pre-CI commit, for the run this chunk's verdict reads` (since `last_wrap` 2026-10-13T07:04:22Z; basis `git log --format='%h %s' d9ad9f1..HEAD`)

## Changes (structured — detectors read this)
- **Files:** modified `crates/viola-e2e/src/harness/run/mutants/scratch.rs`, `crates/viola-e2e/Cargo.toml`, `Cargo.lock`,
  `src/human.rs`, `src/run/send.rs`; new `src/human/{mirror,escape,board,wait,verbs,hints}.rs`,
  `src/run/send/{classify,slot,params,budget,test_support,relabel}.rs` (basis: `gate.py scope` — changed 17 · listed 16 ·
  recorded 1; the New text listing below).
- **Symbols / APIs:**
  - `scratch.rs` `wipe`: after removing the scratch it re-creates it through `viola_state::fs::create_private_dir`
    (was `fs::create_dir_all`); `prepare`, `scratch_allowed`, `scratch_dir`, `dir_bytes`, `HOST_SCRATCH` unchanged, and
    `prepare`'s lines 48 and 54 do not move. On Windows `create_private_dir` gives the topmost folder it creates outside
    the profile the protected `D:P(A;OICI;FA;;;<user>)(A;OICI;FA;;;SY)` DACL, so the wiped scratch is that folder and
    every tempdir laid inside it inherits owner-only; on Linux it sets 0700 (production never reaches `wipe` there:
    `prepare` returns at `HOST_SCRATCH`). New `#[cfg(windows)]` test
    `wipe_scratch_takes_an_instance_tree_the_check_accepts_where_a_plain_dir_does_not` (scratch.rs 252-281 @258): a
    `vhome/instances/builder` tree laid inside a `wipe`d scratch under `target/e2e-home` reads
    `check_instance` → `Ok(())`, the same tree inside a plain-created sibling → `Err(Refused::Writable)`.
  - `src/human.rs` is the module root: the shared writers (`write_refusal`, `refuse`, `write_internal_error`,
    `internal_error`, `write_result`, `result`) stay; `pub(crate) use` re-exports every name the callers use, so no
    caller file changed. Moved, bodies byte-identical: `src/human/mirror.rs` (`MIRROR_WORD`, `mirror_line`,
    `write_send_open`, `write_read_back`, `write_send_unconfirmable`, `write_send_unable`, `write_wrapper_fault`,
    `write_unable`), `src/human/escape.rs` (`escape_message`, `escape_row`), `src/human/board.rs` (`instance_state_pair`,
    `home_state_pair`, `BayBudget`, the cells, `bay_line`, `board_rows`, `write_board`, `EMPTY_BOARD`),
    `src/human/wait.rs` (the `wait` / `last` writers), `src/human/verbs.rs` (`write_answered`, `write_paused`,
    `write_released`, `override_end`, `write_budget_lifted`, `write_budget_open`), `src/human/hints.rs`
    (`HUMAN_TYPING_HINT`, `answer_hint`, `send_hint`).
  - `src/run/send.rs` is the module root: `Ctx` with its methods but `budget_paused`, `send`, and the refusal-order,
    text-refusal, gate-wait, failed-paste, internal and link cases; re-exports `parse_from`, `SendSlot`,
    `append_hook_event` (`PasteFn` is NOT re-exported: no caller outside the module names it). Moved:
    `src/run/send/classify.rs` (`Match`, `Confirms`, `classify` — the local-command decision), `src/run/send/slot.rs`
    (`STEP`, `PasteFn`, `InFlight`, `Flight`, `SendSlot` and its methods incl. `claim` / `settle` / `confirm`,
    `append_hook_event`, `close`, `window_ms`, `Reserved`), `src/run/send/params.rs` (`parse`, `parse_from`),
    `src/run/send/budget.rs` (`impl Ctx { budget_paused }`), `src/run/send/test_support.rs` (`#[cfg(test)]`, the shared
    fixtures), `src/run/send/relabel.rs` (only a `#[cfg(test)] mod tests`: the nine prompt-match and relabel cases).
    Visibility widened only to `pub(super)`; nothing past `pub(crate)`.
  - Where the cited tests now sit (basis: `grep -n 'fn …' src/run/send.rs src/run/send/*.rs src/human.rs
    src/human/*.rs` at the work tree): `send_budget_paused_*` (8) → `src/run/send/budget.rs`; `send_local_command_*`
    (8 functions) → `src/run/send/classify.rs`; the two `inner_cr_is_one_lf` tests → `src/run/send/relabel.rs`; the six
    `trailing_newline` tests of the send module → `src/run/send/classify.rs` 3 (`send_local_command_clear_with_a_trailing_newline_…`,
    `…_decision_reads_the_text_without_its_trailing_newline`, `…_without_a_post_condition_and_a_trailing_newline_…`),
    `src/run/send/relabel.rs` 2 (`send_trailing_newlines_are_not_typed_and_the_send_is_confirmed`,
    `send_a_prompt_that_keeps_the_trailing_newline_claims_nothing`), `src/run/send.rs` 1
    (`send_a_refused_character_before_a_trailing_newline_is_control_character`); the three `send_empty_text_*` cases stay
    in `src/run/send.rs`; `bay_line_*`, `board_row_*` (incl. `board_row_caption_and_default_row_are_the_literals`) and
    `instance_and_home_state_pairs_are_the_fixed_texts` → `src/human/board.rs`; `escape_row_shows_every_control_character_as_hex_text`
    and `escape_message_shows_controls_as_hex_text` → `src/human/escape.rs`; `send_hint` (product) → `src/human/hints.rs`.
    The audit survivor `src/run/send.rs:271:5` (`close`) now sits in `src/run/send/slot.rs`.
- **Crates / modules:** root bin: modules `human::{mirror,escape,board,wait,verbs,hints}` and
  `run::send::{classify,slot,params,budget,relabel}` + `#[cfg(test)] run::send::test_support` added; no crate added.
- **Dependencies:** `viola-e2e` gains the path dependency `viola-state = { path = "../viola-state" }`; `Cargo.lock` gains
  the edge, no crate (gate `git diff -U0 d9ad9f1f1d96 -- Cargo.lock | grep -c '^+name = '` read `0`).
- **Schema / config:** none.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:** `src/run/send.rs` 2088 → 714 code lines, `src/human.rs` 1064 → 135 (basis: `tokei
  --files`, `evidence/sizes-before.md` and implement's outcome); after: `slot.rs` 381, `test_support.rs` 362, `classify.rs`
  267, `budget.rs` 200, `relabel.rs` 185, `params.rs` 45; `board.rs` 450, `hints.rs` 147, `mirror.rs` 144, `wait.rs` 101,
  `verbs.rs` 94, `escape.rs` 56 — no file of either split over 800 (the sizes gate read `last line 0`). The audit's
  `sizes.over_800` 19 → 17 is a FORECAST for the next boundary audit, not measured. Test count of both modules unchanged:
  223 lines (`evidence/split-test-names.txt`, the split-names diff empty).
- **Dev-tool versions:** none — `tokei` re-read at 14.0.0 (the plan's basis); the `x86_64-pc-windows-msvc` std was
  already installed on the dev host (research §4), not a gate's install.
- **Harness / gate surface:** a chunk gate entry, the Windows-target check-only clippy
  `CARGO_TARGET_DIR=target/wincheck cargo clippy --workspace --all-targets --features fake-agent --target
  x86_64-pc-windows-msvc -- -D warnings` (dev host, separate target dir, nothing linked) — not a `pre-push` stage and not
  a CI step; no harness verb, status shape or CI workflow changed. The mutation scratch's lay-down changed (above): on a
  Windows host `run --mutants` now re-creates `<repo parent>/viola-mutants-scratch` owner-only.
- **Cross-project / external claims:**
  - CI run ci#38375200217 (push of `3b75dc729a10`), `verdict: green`, checks 15/15, wall 806 s
    (`evidence/operator.md`, entry 24). In its `test (windows-2025)` job (115221715737) the witness
    `wipe_scratch_takes_an_instance_tree_the_check_accepts_where_a_plain_dir_does_not` PASSED (2403 tests run, 2403
    passed). This wrap's commit adds bookkeeping only to that tree.
  - Run 38215104120 (`windows-mutants.yml`, the Epoch 4 audit, at `dffad3e`) — the red this chunk removed the cause of;
    read at phase P3 (research §2), not re-read here.
  - Inputs (`inputs.py verify`, wrap P1): `I1 · message (the /andromeda-phase invocation, 2026-10-13T07:07Z) · copy · n/a`
    — cited 13×; `I2 · message (the operator, 2026-10-13 08:55Z; the split the founder's own answer «Делить на две»,
    relayed) · copy · n/a` — cited here (`verify` printed it `UNCITED` before this report); `I3 · message (the operator,
    the /andromeda-wrap-session invocation, 2026-10-13T10:47Z) · copy · cited here` (snapped at this wrap). No drift.
  - `inputs#I2` names the P5 card `.andromeda/runs/2026-10-13T07-07-28-phase/p5-card-1.md`; its `## Beyond the plan`
    rows, verbatim, with the word's authority — the operator's own word 2026-10-13, the split itself the founder's own
    dialog answer relayed verbatim by the operator («Делить на две», `I2`); `inputs#I3` (the operator) directs the mint
    of row 1 at this wrap "as the founder approved":

    | # | proposal | kind |
    |---|---|---|
    | 1 | A new working-route entry **"Epoch 4 survivors"**, placed directly after "Epoch 4 cleanup" at the head of `### Epoch 5 — Driver surface`. It carries this entry's CARRY 1 verbatim (the audit's 47 survivors) and research.md §1's survey: the `fs.rs` six reachable only on Windows; `scratch.rs` `prepare`'s two already recorded "not measured here"; `PortablePty::resize`'s miss with its cause unknown; the disposal forms test-plan holds. It inherits `inputs#I1`'s direction: no mutation run, witness run or dispatch; no live `claude` start. | route insertion (the wrap mints it) |
    | 2 | Scope §1 of this chunk leaves with the split: recorded in `scope.md` as "leaves this chunk with the split, owed to the entry #1 mints". | deferral of a part, its owner #1 |
- **Reverted / negative API facts:** a `#[cfg(test)] mod relabel;` declaration was written and replaced by an
  unconditional `mod relabel;` (the file holds only a `#[cfg(test)] mod tests`): `cargo modules orphans` analyses
  without `cfg(test)` and read the cfg-gated file orphaned (gate firing 1, entry orphans red). `NumberFields` and `Lines`
  (test_support) were widened by the move and narrowed back to private. A `pub(crate) use … PasteFn` re-export was not
  written (no caller; an unused re-export fails `-D warnings`).
- **Insufficient fixes (written, kept, not the remedy):** none measured. The fix removes the measured cause of run
  38215104120's red (the scratch's DACL); whether the root package's Windows mutation baseline now reads green is NOT
  measured in this chunk (no dispatch, `inputs#I1`): it is read at the next `windows-mutants.yml` dispatch (the next
  boundary audit). The witness proves the lay-down on CI's ordinary leg, not the mutants run.
- **Spec claims disproved by measurement:** none.
- **Expected amendments (from plan):** sites by `cascade.py sweep --pre-pass` (`report-patterns.toml` in this run dir):
  pattern `split` (`\b(run/)?send\.rs\b|\bhuman\.rs\b`) standing 17 — architecture 5 (`architecture.md:49` ×3, `:91`,
  `registries/contracts/architecture/build-system.md:4`, `…/project-directory-structure.md:21`, `:31`), security-plan 1
  (`:228` ×5), test-plan 6 (`:566`, `:582`, `:583`, `:590`, `:655`, `:1123`), obs-plan 3 (`:556`, `:767`, `:1157`), a11y-plan 2
  (`:543`, `:853`); pattern `scratch` (`mutants-scratch|host scratch|scratch.{0,40}(DACL|owner-only|create_dir_all|wipe)`,
  case-folded) standing 11 — architecture 1 (`:452`), security-plan 1 (`:607`), test-plan 8 (`:441`, `:1226`,
  `registries/contracts/test-plan/5-command-implementation.md:45`, `:46`, `:49`, `:57`, `:142`, `:163`), obs-plan 1 (`:973`);
  pattern `winclippy` (`x86_64-pc-windows-msvc|wincheck|Windows-target (check-only )?clippy`) — control UNFIRED over the
  seven masters at `3b75dc72` (0 sites; the tool's refusal line is the basis).
  - architecture §Infrastructure Patterns → Project directory structure (the two splits) — carried: **Symbols / APIs**
    (the module roots and their children); sites `project-directory-structure.md:21` (`human.rs` row), `:31` (`send.rs` row).
  - architecture §Occupied Resources, the mutation scratch row (owner-only on a Windows host) — carried: **Symbols /
    APIs** (`wipe`) and **Harness / gate surface**; site `architecture.md:452`.
  - test-plan §3 `run` step 4, the Windows host scratch arm (the scratch's DACL and its witness) — carried: **Symbols /
    APIs**, **Cross-project** (the CI witness); site `5-command-implementation.md:45` ("removes and recreates it").
  - test-plan §4, the file citations of moved tests — carried: **Symbols / APIs** (where the cited tests now sit); sites
    `test-plan.md:566` (product caller `send`, still `src/run/send.rs` — no change), `:582` (`send_budget_paused_*`),
    `:583` (six `trailing_newline`, two `inner_cr_is_one_lf`), `:590` (`send_empty_text_*` — still `src/run/send.rs`, no
    change); `:655` (§5, the board units) is the same fact one section on.
  - test-plan §9 Lint row (the Windows-target clippy as a chunk gate entry) — carried: **Harness / gate surface**; site
    the Lint row `test-plan.md:1157` (no `winclippy` hit; located by its row header).
  - security-plan §Security Anti-Patterns › Code Patterns, the `run --mutants` row (the wiped scratch re-created
    owner-only) — carried: **Symbols / APIs**; site `security-plan.md:607`.
  - a11y-plan §4, P6's units now in `src/human/board.rs` and `src/human/escape.rs` — carried: **Symbols / APIs**; sites
    `a11y-plan.md:543` (§4, `board_row_caption_and_default_row_are_the_literals`), `:853` (§8, the board units).
  - Beyond the list, sites the sweep found stating a moved fact: `architecture.md:49` @c9591 (the local-command decision
    "as landed … `src/run/send.rs`", now `classify.rs`), `:49` @c8015 (the claim / relabel / settle, now `slot.rs` and the
    root's `send`), `:91` (`send_hint`, `src/human.rs` → `src/human/hints.rs`), `build-system.md:4` ("its only human
    writer is `src/human.rs`" — the module now spans `src/human/`). Standing true: `architecture.md:49` @c3828 and
    `security-plan.md:228` (`src/cmd/send.rs`, another file; the wrapper's `send` still in `src/run/send.rs`), `obs-plan.md:556`,
    `:1157` (`refuse` → `write_refusal` stay in the root `src/human.rs`), `test-plan.md:1123`, `obs-plan.md:767` ("as read
    in … at chunk …", dated readings).
- **Coverage of new surfaces:**
  - `run --mutants` scratch lay-down (harness, Windows host) → validation n/a (the guard unchanged) · instrumentation n/a
    (harness, no product span) · PII n/a (the path is still never printed) · tests unit (Linux: the scratch cases;
    Windows: the witness on CI) · a11y n/a · tokens n/a
  - the two module splits → no new external surface (every caller path kept by re-export; no output, refusal, span or
    event moved) · tests unit (the split-names diff, `pre-push` coverage 2406 passed) · the rest n/a

## Deviations from intent
- **The survivors and the baseline's green reading leave this chunk.** The working-route line names "Epoch 4's mutation
  survivors disposed, the Windows mutation baseline green on the root package": the 47 survivors leave with the split the
  founder approved (`inputs#I2`, P5 card row 2) to the entry row 1 mints ("Epoch 4 survivors", `inputs#I3`); the
  baseline's cause is removed and witnessed on CI, but its green reading needs a `windows-mutants.yml` dispatch, which
  the founder's standing direction holds to the epoch boundary (`inputs#I1`) — owed to the next boundary audit, and named
  in the new entry's CARRY.
- `src/run/send/relabel.rs`, one phase file beyond the plan's list: step 5's own clause (the root read 889 code lines
  after the planned split; the nine prompt-match and relabel cases moved, root 714).
- `mod relabel;` declared without `#[cfg(test)]` (orphan gate, above); `test_support.rs` keeps `#[cfg(test)]` as step 5
  says and the gate did not flag it — why was not measured.
- `PasteFn` not re-exported though step 5 names it (no caller; `-D warnings`).
- The human tests' shared writers (`Recorder`, `Broken`, `one_write`) stay in the root `human::tests` at `pub(super)`;
  siblings import them through `super::super::tests` (the `scratch.rs` precedent).
- Bodies moved by line slices byte-identical but for the one-level dedent of `test_support.rs` and four rustfmt
  re-wraps there; `cargo fix` removed the unused imports the glob scaffolding left; the root `src/run/send.rs` keeps its
  original import block and the children read it through `use super::*`.
- Test placement where step 5 left a choice: the slot file takes the closed-send, window-expiry, second-in-flight,
  wait-feed signal, turn-start, turn-end, after-a-confirmed-send and confirm-window cases; `Setup` stays in the root
  beside `send_refusal_order`; `send_writers_return_the_writer_error` sits in `src/human/mirror.rs`.
- Scope record (`gate.py scope` at wrap P1: `scope: clean — changed 17 · listed 16 · recorded 1 (companion 0 ·
  mechanical 0 · in-intent 1 · widening 0)`):
  - in-intent: `src/run/send/relabel.rs` · serves step 5 · self.

## Decisions & corrections
- `cargo modules orphans` analyses without `cfg(test)`: a `#[cfg(test)] mod x;` declaration reads its file as an
  orphan (gate firing 1, `orphaned module relabel`); fixed by declaring the module unconditionally, the file holding only
  a `#[cfg(test)] mod tests`. Evidence: implement journal firing 1, outcome deviation 2.
- A pure-move split was proven by three reads: a test-name diff against a before-record written on the untouched tree
  (normalised by sed dropping the module path between the root and `tests::`), a tokei size probe, and a line-multiset
  comparison of HEAD against the split files (only visibility, imports, `mod` scaffolding and four re-wraps differed).
  Evidence: `evidence/split-test-names.txt`, outcome deviation 6.
- Slicing scripts were re-read before running and two defects were caught (a missing blank line after the root doc
  comment; a `parse` range overlapping the root's `Ctx` slice). Evidence: the implement window (no artifact).
- Gate hygiene refused two phase run-dir files that quoted the runner's scratch as a Windows drive path; rewritten as
  `<repo parent>/viola-mutants-scratch` on the runner (`evidence/operator.md`, entry 22). A run-dir note quoting a CI log
  path carries a drive path the hygiene read refuses.
- An `#[cfg(windows)]` witness whose refused control runs beside the accepted reading keeps a Windows-only assertion from
  passing vacuously; it runs on CI's ordinary `test (windows-2025)` leg with no dispatch. Evidence: plan step 3,
  `evidence/operator.md` (the PASS line).
- The operator's directions at implement: confirm plan.md and scope.md agree on the split from disk before any edit
  (done: outcome §"Plan and scope agree"); no live `claude` start, no mutation run, witness run or workflow dispatch (none
  made); journal each long gate run before waiting (`journal.md`); run the operator pass with the `ci.py` read (done).
- The operator's direction at this wrap (`inputs#I3`): mint "Epoch 4 survivors" directly after this entry from the P5
  card's row 1, as the founder approved (`inputs#I2`).
- Guard or hook refusals: implement none; this wrap one (`cd` into a skill dir refused by the Bash guard; re-issued as
  a subshell).

## Outcome
- Acceptance, re-asserted against the diff:
  - The Windows witness passes on `test (windows-2025)` in the run the CI read reads green for HEAD — MET: ci#38375200217,
    job 115221715737, `PASS [0.035s] (1993/2403)`.
  - `crates/viola-state` unchanged, `check_instance` included — MET: no file under `crates/viola-state` in the diff (the
    preservation guard green).
  - The scratch guard and its no-fallback / no-path rules hold — MET: `prepare` and `scratch_allowed` unchanged; the
    scratch cases green (7).
  - Every test of `human` and `run::send` keeps its name and cases — MET: the split-names diff empty against 223 lines.
  - Every human-output, refusal and tui case passes unchanged — MET: no file under `tests/` changed; `pre-push`
    `"ok":true` at `linux-tests` (coverage 2406 passed, playwright 1).
  - No file of either split over 800 code lines; orphans reach every new file — MET (sizes `last line 0`; orphans green
    after the `mod relabel;` fix).
  - G1 clean; the human siblings carry no print macro and no `#[allow]` — MET.
  - The Windows-target clippy exits 0 on the final tree — MET (firing 2).
  - `cargo deny` passes and `Cargo.lock` gains no crate — MET.
  - `run --unit` `"ok":true` (nextest-unit 1972 passed) and CI `verdict: green` across the three OSes — MET.
- Gates (implement firing 2, the final source tree; `pre-push` re-fired before the pre-CI commit):
  `cargo fmt --all --check` green · `cargo clippy --workspace …` green · `CARGO_TARGET_DIR=target/wincheck cargo clippy …
  --target x86_64-pc-windows-msvc …` green · G1 `rg … instrument …` green (exit 1, no output) · `bash scripts/lint-probes.sh`
  green · `bash scripts/orphans-check.sh --probe && …` green (red at firing 1, fixed) · `agent-run.sh run --unit` green ·
  `run --unit --filter 'package(viola-e2e) & …'` green · the split-names `nextest list … | diff -` green · the `tokei`
  sizes probe green (`last line 0`) · the human print/allow `rg` green (exit 1, no output) · the preservation `git diff
  --quiet d9ad9f1f1d96 …` green · the lockfile `grep -c '^+name = '` green (`0`) · `cargo deny check` green · smoke
  `cleanup` · `boot` · `status` green · `g2-zero-panics.sh` green · `schema-check` green · `cleanup` green
  (`processes_gone`, `endpoint_gone`) · `pre-push` green (67.92 s; re-fired 67.13 s). `leg = 'operator'`: hygiene green
  (`hygiene: clean` after two rewrites) · push green (`d9ad9f1f→3b75dc72`) · `ci.py conclusion` green (ci#38375200217,
  15/15) — `evidence/operator.md`. Smoke (boot path changed: the wrapper's `send` handler moved): green.
- Watches: none folded.
- Outcome basis: the operator pass ran (pre-CI commit `3b75dc7`; no fix commit); the gate verdicts rest on its final
  state, the CI run recorded in `evidence/operator.md`; implement's report (`.andromeda/runs/2026-10-13T08-58-46-implement/outcome-2026-10-13-epoch-4-cleanup.md`,
  read whole at this wrap — the window was compacted) is the basis for deviations and the census.
- Process hygiene (implement's census): the harness sessions `p-e4c-smoke` ×2 and `p-e4c-p3`, terminated
  (`processes_gone`, `endpoint_gone` true); `pre-push` children exited with the entry; measured after P3 by `/proc/<pid>/exe`
  under the repository root: no process. Re-measured at this wrap before the light gate (P7).
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: d9ad9f1f (the parent of the oldest pre-CI commit 3b75dc72) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### Cargo.lock — added 1 line(s) in 1 range(s)
added: 1683
### crates/viola-e2e/Cargo.toml — added 1 line(s) in 1 range(s)
added: 23
### crates/viola-e2e/src/harness/run/mutants/scratch.rs — added 35 line(s) in 3 range(s)
added: 62-64 · 70 · 252-282
  - 252-281 @258 «fn wipe_scratch_takes_an_instance_tree_the_check_accepts_where_a_plain_dir_does_not() {»
### src/human.rs — added 27 line(s) in 4 range(s)
added: 7-28 · 70-72 · 93 · 165
- 17-20 «pub(crate) use mirror::{»
- 21-24 «pub(crate) use verbs::{»
- 25-28 «pub(crate) use wait::{»
### src/human/board.rs — new file · 505 line(s)
- 13-19 @14 «pub(crate) fn instance_state_pair(name: &str) -> (String, String) {»
  - 15-18 «(»
- 21-27 @22 «pub(crate) fn home_state_pair() -> (&'static str, &'static str) {»
  - 23-26 «(»
- 29-33 @30 «pub(crate) struct BayBudget<'a> {»
- 35-49 @37 «fn window_cell(label: &str, window: &Reading<BudgetWindow>, now: DateTime<Utc>) -> String {»
  - 38-41 «let Reading::Known(BudgetWindow {»
  - 42-44 «else {»
  - 45-46 «let expired = matches!(resets_at, Reading::Known(at)»
- 51-62 @53 «fn read_cell(read_at: &Reading<String>, now: DateTime<Utc>) -> String {»
  - 54-57 «let at = match read_at {»
  - 58-61 «match at.filter(|at| *at <= now) {»
- 64-93 @66 «pub(crate) fn bay_line(»
  - 71-88 «let (five_hour, seven_day, read, gate) = match budget {»
  - 89-92 «format!(»
- 101-108 «fn wheel_cell(item: &Item) -> String {»
  - 103-107 «if item.budget_paused == Some(true) {»
- 110-118 «fn dialog_cell(item: &Item) -> String {»
  - 111-113 «if !item.dialog_pending {»
  - 114-117 «match item.dialog_kind.as_deref() {»
- 120-132 «fn cli_cell(item: &Item) -> String {»
  - 121-131 «match &item.cli_version {»
- 134-140 «fn board_line(width: usize, cells: [&str; 6]) -> String {»
  - 136-139 «format!(»
- 142-168 @144 «pub(crate) fn board_rows(items: &[Item]) -> Vec<String> {»
  - 145-148 «let width = items»
  - 149-152 «let mut rows = vec![board_line(»
  - 153-166 «for item in items {»
- 172-187 @173 «pub(crate) fn write_board(»
  - 181-183 «if board.items.is_empty() {»
- 189-505 @190 «mod tests {»
  - 195-211 @196 «fn instance_and_home_state_pairs_are_the_fixed_texts() {»
  - 213-217 «fn at(text: &str) -> DateTime<Utc> {»
  - 221-226 «fn window(used: Reading<f64>, resets_at: &str) -> Reading<BudgetWindow> {»
  - 228-240 «fn stamped(»
  - 242-248 «fn counts(unknown_kinds: u64, unknown_fields: u64, torn_lines: u64) -> SkippedTotal {»
  - 250-260 @251 «fn bay_line_without_a_reading_is_the_unknown_line() {»
  - 262-285 @263 «fn bay_line_with_a_reading_shows_the_figures_the_age_and_the_gate() {»
  - 301-313 @303 «fn bay_line_window_cell(#[case] five_hour: Reading<BudgetWindow>, #[case] cell: &str) {»
  - 315-327 @322 «fn bay_line_read_cell(#[case] read_at: &str, #[case] cell: &str) {»
  - 329-332 @330 «fn bay_line_read_cell_of_an_unknown_instant_is_unknown() {»
  - 334-347 «fn item(name: &str) -> Item {»
  - 349-350 «const CAPTION: &str =»
  - 351-352 «const DEFAULT_ROW: &str =»
  - 354-361 @355 «fn board_row_caption_and_default_row_are_the_literals() {»
  - 363-388 @364 «fn board_row_every_value_starts_in_its_captions_column() {»
  - 390-405 @396 «fn board_row_wheel_cell(»
  - 407-423 @414 «fn board_row_dialog_cell(»
  - 435-444 «fn board_row_cli_cell(»
  - 446-457 @447 «fn board_row_a_32_character_name_prints_whole_and_moves_the_columns_right() {»
  - 459-469 @460 «fn board_row_rows_keep_the_boards_name_order_whatever_their_state() {»
  - 471-483 @472 «fn board_row_the_empty_board_is_three_lines_in_one_write() {»
  - 485-504 @486 «fn board_row_a_board_is_one_write_of_printable_ascii_and_lf() {»
### src/human/escape.rs — new file · 67 line(s)
- 3-15 @5 «pub(crate) fn escape_message(text: &str) -> String {»
  - 7-13 «for c in text.chars() {»
- 17-30 @20 «pub(crate) fn escape_row(text: &str) -> String {»
  - 22-28 «for c in text.chars() {»
- 32-67 @33 «mod tests {»
  - 37-50 @48 «fn escape_message_shows_controls_as_hex_text(#[case] text: &str, #[case] shown: &str) {»
  - 52-66 @61 «fn escape_row_shows_every_control_character_as_hex_text(»
### src/human/hints.rs — new file · 165 line(s)
- 7-21 @9 «pub(crate) fn answer_hint(reason: &str, detail: Option<&str>) -> Option<&'static str> {»
  - 10-20 «match (reason, detail) {»
- 23-52 @26 «pub(crate) fn send_hint(name: &str, cause: &str) -> Option<String> {»
  - 27-51 «Some(match cause {»
- 54-165 @55 «mod tests {»
  - 89-91 «fn send_hint_is_the_design_string(#[case] cause: &str, #[case] hint: &str) {»
  - 93-97 @94 «fn send_hint_has_none_for_an_opaque_cause() {»
  - 99-132 @101 «fn no_hint_names_release() {»
  - 158-164 «fn answer_hint_is_the_fixed_table(»
### src/human/mirror.rs — new file · 174 line(s)
- 8-10 «fn mirror_line(boxed: &str, word: &str, name: &str, rest: &str) -> String {»
- 12-16 @14 «pub(crate) fn write_send_open(out: &mut impl Write, name: &str, issued: &str) -> io::Result<()> {»
- 18-27 @19 «pub(crate) fn write_read_back(»
- 29-34 @31 «pub(crate) fn write_send_unconfirmable(out: &mut impl Write, name: &str) -> io::Result<()> {»
- 36-49 @38 «pub(crate) fn write_send_unable(»
  - 45-47 «if let Some(hint) = hint {»
- 51-54 @52 «pub(crate) fn write_wrapper_fault(out: &mut impl Write, code: i64) -> io::Result<()> {»
- 56-69 @58 «pub(crate) fn write_unable(»
  - 65-67 «if let Some(hint) = hint {»
- 71-174 @72 «mod tests {»
  - 77-83 @78 «fn write_send_open_is_the_open_box_in_one_write() {»
  - 85-91 @86 «fn write_read_back_is_the_filled_box_in_one_write() {»
  - 93-99 @94 «fn write_send_unconfirmable_is_the_open_box_and_the_fixed_note_in_one_write() {»
  - 101-129 @102 «fn write_send_unable_puts_the_hint_last_in_one_write() {»
  - 131-141 @132 «fn write_send_mirror_words_line_up() {»
  - 143-149 @144 «fn write_wrapper_fault_is_the_code_and_no_hint() {»
  - 151-163 @152 «fn write_unable_puts_the_hint_last_in_one_write() {»
  - 165-173 @166 «fn send_writers_return_the_writer_error() {»
### src/human/verbs.rs — new file · 120 line(s)
- 9-12 @10 «pub(crate) fn write_answered(out: &mut impl Write, name: &str, dialog_id: u64) -> io::Result<()> {»
- 14-17 @15 «pub(crate) fn write_paused(out: &mut impl Write, name: &str) -> io::Result<()> {»
- 19-22 @20 «pub(crate) fn write_released(out: &mut impl Write, name: &str, holder: &str) -> io::Result<()> {»
- 24-38 @27 «pub(crate) fn override_end(override_until: &str, today: NaiveDate) -> String {»
  - 28-30 «let Ok(end) = DateTime::parse_from_rfc3339(override_until) else {»
  - 32-36 «let format = if end.date_naive() == today {»
- 40-49 @42 «pub(crate) fn write_budget_lifted(»
- 51-54 @52 «pub(crate) fn write_budget_open(out: &mut impl Write, name: &str, holder: &str) -> io::Result<()> {»
- 56-120 @57 «mod tests {»
  - 62-74 @63 «fn wheel_verb_lines_are_the_design_lines_in_one_write() {»
  - 76-88 @77 «fn budget_verb_lines_are_the_design_lines_in_one_write() {»
  - 90-92 «fn day(year: i32, month: u32, day: u32) -> NaiveDate {»
  - 105-111 @106 «fn override_end_is_the_utc_time_with_its_date_on_another_day(»
  - 113-119 @114 «fn write_answered_is_one_line_naming_the_dialog() {»
### src/human/wait.rs — new file · 124 line(s)
- 7-10 @8 «pub(crate) fn write_waiting(out: &mut impl Write, name: &str) -> io::Result<()> {»
- 12-22 @13 «pub(crate) fn write_woken(»
- 24-34 @25 «pub(crate) fn write_woken_dialog(»
- 36-39 @37 «pub(crate) fn write_timed_out(out: &mut impl Write, name: &str, ms: u64) -> io::Result<()> {»
- 41-45 @42 «pub(crate) fn write_last_turn(out: &mut impl Write, name: &str, at: &str) -> io::Result<()> {»
- 47-52 @48 «pub(crate) fn write_message(out: &mut impl Write, message: &str) -> io::Result<()> {»
- 54-57 @55 «pub(crate) fn write_no_message(out: &mut impl Write) -> io::Result<()> {»
- 59-124 @60 «mod tests {»
  - 65-87 @66 «fn wait_writers_are_the_design_lines_in_one_write() {»
  - 89-100 @90 «fn last_writers_are_the_design_lines_in_one_write() {»
  - 102-111 @104 «fn woken_lines_escape_what_they_are_handed() {»
  - 113-123 @114 «fn wait_and_last_writers_return_the_writer_error() {»
### src/run/send.rs — added 14 line(s) in 5 range(s)
added: 39-45 · 47-48 · 50-52 · 303 · 308
### src/run/send/budget.rs — new file · 233 line(s)
- 5-16 «impl Ctx<'_> {»
  - 6-15 @8 «pub(super) fn budget_paused(&self, crossed: &Crossed) -> Result<Value, ProtocolError> {»
- 18-233 @19 «mod tests {»
  - 27-84 @35 «fn send_budget_paused_is_refused_with_the_window_word_and_no_cursor(»
  - 86-105 @88 «fn send_budget_paused_fields_are_on_no_other_refusal() {»
  - 107-119 @110 «fn send_budget_paused_comes_after_human_typing() {»
  - 121-132 @125 «fn send_budget_paused_comes_after_the_text_refusals(#[case] text: &str, #[case] detail: &str) {»
  - 134-164 @139 «fn send_budget_paused_comes_before_turn_running(#[case] in_flight: bool) {»
  - 166-180 @169 «fn send_budget_paused_does_not_refuse_under_the_thresholds() {»
  - 182-218 @185 «fn send_budget_paused_is_not_read_again_after_the_gate_wait() {»
  - 220-232 @222 «fn send_budget_paused_whose_record_cannot_land_is_internal() {»
### src/run/send/classify.rs — new file · 309 line(s)
- 5-12 «pub(super) enum Match {»
- 14-23 @16 «pub(super) enum Confirms {»
- 25-33 @27 «pub(super) fn classify(text: &str, cli_verified: bool) -> Confirms {»
  - 28-32 «match LOCAL_COMMANDS.iter().find(|(command, _)| *command == text) {»
- 35-309 @36 «mod tests {»
  - 44-72 @50 «fn send_local_command_without_a_measured_post_condition_is_unconfirmable(»
  - 74-106 @77 «fn send_local_command_clear_is_confirmed_by_a_new_session() {»
  - 108-166 @114 «fn send_local_command_clear_is_not_confirmed_by_another_session_start(»
  - 168-188 @169 «fn send_local_command_window_expiry_is_no_prompt_submitted_with_the_cursor() {»
  - 190-217 @198 «fn send_local_command_decision_is_exact_text_never_a_leading_slash(#[case] text: &str) {»
  - 219-244 @226 «fn send_local_command_decision_reads_the_text_without_its_trailing_newline(#[case] tail: &str) {»
  - 246-268 @252 «fn send_local_command_without_a_post_condition_and_a_trailing_newline_is_unconfirmable(»
  - 270-308 @278 «fn send_local_command_clear_with_a_trailing_newline_is_confirmed_by_a_new_session(»
### src/run/send/params.rs — new file · 56 line(s)
- 5-11 @6 «pub(super) fn parse(params: &Value) -> Result<(&str, Option<ViolaName>), ProtocolError> {»
  - 7-9 «let text = params["text"]»
- 13-22 @14 «pub(crate) fn parse_from(params: &Value) -> Result<Option<ViolaName>, ProtocolError> {»
  - 15-21 «match params.get("from") {»
- 24-56 @25 «mod tests {»
  - 33-46 @38 «fn send_params_it_cannot_take_are_invalid_params(#[case] params: Value) {»
  - 48-55 @49 «fn send_from_null_is_no_from() {»
### src/run/send/relabel.rs — new file · 214 line(s)
- 4-214 @5 «mod tests {»
  - 14-17 @15 «fn send_match_is_ok_with_the_prompt_ts_and_the_cursor() {»
  - 19-22 @20 «fn send_harness_filed_prompt_with_the_in_flight_text_is_relabelled_driver() {»
  - 24-36 @34 «fn send_trailing_newlines_are_not_typed_and_the_send_is_confirmed(#[case] tail: &str) {»
  - 62-67 «fn send_text_whose_inner_cr_is_one_lf_is_typed_and_confirmed(»
  - 69-115 @76 «fn send_a_prompt_that_keeps_the_cr_while_its_inner_cr_is_one_lf_claims_nothing(»
  - 117-162 @124 «fn send_a_prompt_that_keeps_the_trailing_newline_claims_nothing(#[case] tail: &str) {»
  - 164-184 @165 «fn send_a_different_prompt_is_appended_unchanged_and_claims_nothing() {»
  - 186-199 @190 «fn send_an_unsent_human_prompt_takes_the_wheel_before_its_line_and_a_harness_one_does_not() {»
  - 201-213 @202 «fn send_no_prompt_while_nothing_is_in_flight_is_appended_unchanged() {»
### src/run/send/slot.rs — new file · 458 line(s)
- 10-14 «pub(super) struct InFlight {»
- 16-23 @18 «pub(super) struct Flight {»
- 25-37 @28 «pub(crate) struct SendSlot {»
- 39-160 «impl SendSlot {»
  - 40-55 «pub(crate) fn new(»
  - 57-60 @58 «pub(crate) fn attach(&self, paste: PasteFn, gate: Gate) {»
  - 62-65 @63 «pub(crate) fn budget_with(&self, gate: Arc<BudgetGate>) {»
  - 67-70 @68 «pub(crate) fn links_with(&self, links: Arc<LinkSet>) {»
  - 72-74 «pub(super) fn flight(&self) -> MutexGuard<'_, Flight> {»
  - 76-91 @79 «pub(crate) fn claim(&self, text: &str) -> bool {»
  - 93-117 @98 «fn session_started(&self, data: &Value) -> bool {»
  - 119-130 @120 «pub(crate) fn settle(&self, ts: Option<String>) {»
  - 132-159 @136 «pub(super) fn confirm(&self, opened: Instant) -> Option<String> {»
- 162-204 @169 «pub(crate) fn append_hook_event(»
  - 175-178 «let prompt = line.kind == EventKind::PromptSubmitted»
  - 179-181 «if prompt {»
  - 182-183 «let claimed =»
  - 184-193 @187 «match line.kind {»
  - 194-198 @196 «if line.kind == EventKind::PromptSubmitted && line.data["origin"] == "human" {»
  - 200-202 «if claimed {»
- 206-210 «fn close(flight: &mut Option<InFlight>) {»
  - 207-209 «if let Some(f) = flight.as_mut() {»
- 212-214 «fn window_ms() -> u64 {»
- 219-223 «impl Drop for Reserved<'_> {»
  - 220-222 «fn drop(&mut self) {»
- 225-458 @226 «mod tests {»
  - 236-259 @239 «fn send_closed_send_keeps_the_slot_until_its_guard_drops() {»
  - 261-282 @262 «fn send_window_expiry_is_no_prompt_submitted_with_the_cursor() {»
  - 284-315 @285 «fn send_second_while_one_is_in_flight_is_turn_running_and_types_nothing() {»
  - 317-344 @319 «fn send_append_hook_event_signals_the_wait_feed_after_the_append() {»
  - 346-367 @351 «fn send_a_prompt_of_any_origin_starts_a_turn(»
  - 369-401 @375 «fn send_a_turn_end_lets_the_next_send_past_the_rung(»
  - 403-452 @406 «fn send_after_a_confirmed_send_is_turn_running_until_turn_ended() {»
  - 454-457 @455 «fn send_confirm_window_is_the_fallback() {»
### src/run/send/test_support.rs — new file · 442 line(s)
- 26-30 «impl Clock for FixedClock {»
  - 27-29 «fn now(&self) -> Instant {»
- 35-41 «impl Clock for JumpClock {»
  - 36-40 «fn now(&self) -> Instant {»
- 43-48 @45 «pub(super) struct SwitchClock {»
- 50-58 «impl Clock for SwitchClock {»
  - 51-57 «fn now(&self) -> Instant {»
- 60-70 «impl SwitchClock {»
  - 61-69 @62 «pub(super) fn at(base: Instant) -> (Self, Arc<std::sync::atomic::AtomicBool>) {»
- 72-74 «pub(super) fn name() -> ViolaName {»
- 76-86 @77 «pub(super) fn quiet_gate(base: Instant) -> Gate {»
  - 78-82 «let (feeder, gate, thread) = gate::start(»
- 88-96 @89 «pub(super) fn verified_gate(fed_at: Instant, rows: &[&str]) -> Gate {»
- 98-104 @100 «pub(super) struct WaitClock {»
- 106-117 «impl Clock for WaitClock {»
  - 107-116 «fn now(&self) -> Instant {»
- 119-127 @120 «pub(super) fn poisoned_gate(base: Instant) -> Gate {»
- 133-151 «impl Pastes {»
  - 134-137 «pub(super) fn new() -> (Self, mpsc::Receiver<()>) {»
  - 139-146 «pub(super) fn paste_fn(&self) -> PasteFn {»
  - 148-150 «pub(super) fn all(&self) -> Vec<String> {»
- 153-161 «pub(super) fn call(params: &Value, id: u64) -> Call<'_> {»
  - 154-160 «Call {»
- 163-169 «pub(super) fn events(dir: &Path) -> Vec<Value> {»
  - 164-168 «std::fs::read_to_string(dir.join("events.ndjson"))»
- 171-176 «pub(super) fn kinds(dir: &Path) -> Vec<String> {»
  - 172-175 «events(dir)»
- 178-186 «pub(super) fn prompt(text: &str, origin: &str) -> EventLine {»
  - 179-185 «EventLine::new(»
- 188-190 «pub(super) fn feed() -> WaitFeed {»
- 192-198 «pub(super) fn occupy(slot: &SendSlot) {»
  - 193-197 «slot.flight().send = Some(InFlight {»
- 200-204 @201 «pub(super) fn paused(wheel: &WheelSlot) {»
- 206-209 @207 «pub(super) fn confirmed_with(origin: &str) {»
- 211-253 @214 «pub(super) fn confirmed_as(sent: &str, typed: &str, origin: &str) {»
  - 221-224 «std::fs::write(»
  - 227-233 «let reply = std::thread::scope(|s| {»
  - 237-240 «assert_eq!(»
  - 242-245 «assert_eq!(»
  - 247-250 «assert_eq!(»
- 255-256 «pub(super) const HARNESS_TURN: &str =»
- 258-260 «pub(super) fn hook_line(kind: EventKind, data: Value) -> EventLine {»
- 262-270 @264 «pub(super) fn mute_child() -> (SendSlot, Pastes) {»
- 272-279 @273 «pub(super) fn assert_refused_without_a_cursor(dir: &Path, detail: Option<&str>) {»
- 281-296 @282 «pub(super) fn confirmed(»
  - 289-294 «std::thread::scope(|s| {»
- 298-331 @302 «pub(super) fn sent_after_the_gate_wait(»
  - 310-314 «let clock = WaitClock {»
  - 317-320 «let gate = verified_gate(»
  - 324-328 «assert_eq!(»
- 336-341 «pub(super) fn used(percentage: f64) -> Reading<BudgetWindow> {»
  - 337-340 «Reading::Known(BudgetWindow {»
- 343-356 @344 «pub(super) fn budget_rig(root: &Path) -> (std::path::PathBuf, std::path::PathBuf, Arc<BudgetGate>) {»
  - 348-354 «let gate = Arc::new(BudgetGate::new(»
- 358-364 «pub(super) fn feed_budget(home: &Path, five_hour: f64, seven_day: f64) {»
  - 359-362 «let reading = BudgetReading {»
- 366-378 @368 «pub(super) fn budgeted(»
- 384-403 «impl tracing::field::Visit for NumberFields {»
  - 385-387 «fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {»
  - 388-390 «fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {»
  - 391-393 «fn record_str(&mut self, field: &tracing::field::Field, value: &str) {»
  - 394-396 «fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {»
  - 397-399 «fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {»
  - 400-402 «fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {»
- 409-416 «impl<S: tracing::Subscriber> Layer<S> for Lines {»
  - 410-415 «fn on_event(&self, event: &tracing::Event<'_>, _: Context<'_, S>) {»
- 418-424 «pub(super) fn capture_lines<R>(f: impl FnOnce() -> R) -> (R, Vec<Value>) {»
- 426-433 @427 «pub(super) fn linking(links_dir: &Path) -> (SendSlot, Pastes) {»
- 435-442 @436 «pub(super) fn line_starts(dir: &Path) -> Vec<usize> {»
  - 439-441 «std::iter::once(0)»
