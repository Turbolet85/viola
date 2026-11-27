# P5 review card 1 — 2026-11-27-epoch-5-cleanup «Epoch 5 cleanup» (written 2026-11-27T20:40:04Z)

**The plan to open:** `.andromeda/cache/docs/2026-11-27-epoch-5-cleanup/plan.md` (its render, rev 3, 608 lines). No plan
stands in a chunk's folder. Writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-27-epoch-5-cleanup` diffs the
chunk's records.

## Goal
The Epoch 5 audit's ranked defects mended so that the next boundary's audit reads each one cleared:
- (1) the root crate's mutation baseline green, then its 372 in-diff mutants scored ONCE as an addendum, behind an
  announced stop;
- (2) and (4) the 9 + 3 survivors killed in their own crates;
- (5) `r2-open.sh`'s two preconditions;
- (3) one start prologue; (7) one `refused()`.

Item 6 (the three splits) is held on F1, the operator's. Items 8 and 9 are expected wrap amendments, at discretion.
No product behaviour, flag, event kind, refusal code or master rule is added.

## Top acceptance criteria
1. The verify class's filter line, one line in both nextest profiles, takes the census's six tests by name.
   - Under `NEXTEST_PROFILE=mutants` they read no TIMEOUT; the 15 s default is unchanged.
   - The planted-hang pair (TIMEOUT ~80 s / PASS) goes to `evidence/kill-lines.md`.
   - **F2, the overseer's answer (`inputs#I2`): ALL SIX move**: `path4_a_stamp_failing_the_dialog_rows_leaves_dialogs_to_the_human`
     and `tests/cli_version_gate.rs`'s `run_with_a_verified_version_records_cli_verified`,
     `run_without_a_verified_stamp_degrades`, `run_with_world_writable_stamps_is_unverified_whatever_they_hold`,
     `run_writes_nothing_of_its_own_on_any_gate_outcome`, `run_with_a_child_that_is_not_the_cli_records_no_version`.
     One margin, one cause.
   - **The pinned filter line is re-pointed:** `tests/contract_lints.rs:130` `VERIFY_WINDOW_FILTER` takes the new line,
     the one test edit of item 1.
2. The addendum, step 8, the one mutation run.
   - Step 6's listing must read 372 first.
   - It runs once, on the overseer's «go» at step 7's stop. That stop also asks his word on where the dated addendum
     line lands: the audit run is sealed.
   - The record goes to `evidence/c-mutation-viola.late.json` and `evidence/addendum.md`.
3. The 5 `pump.rs` cases and the 5 `viola-state` cases pass, and all 12 controls read `holds`. F3: the drain bound goes
   through `Seams`, and `walk` runs over an injected parent lookup, so no mutant is left standing as equivalent.
4. `r2-scratch` and `r2-locked` read exit 1 with their precondition line before anything starts.
5. One `trusted()` and one `refused()` stand under `src/cmd`. The whole layers read `"ok":true`, with no test edited
   beyond the item 1 pin and four re-pointed `contract_error_bodies.rs` rows. The smoke trio reads ready, and CI reads
   green.

## Touchpoints
- **Files to modify (13):** `.config/nextest.toml` · `tests/contract_lints.rs` · `crates/viola-pty/src/pump.rs` ·
  `crates/viola-state/src/{liveness,board,replay}.rs` · `scripts/r2-open.sh` · `src/cmd/{run,revive,adopt,open,up}.rs`
  · `tests/contract_error_bodies.rs`. No new file.
- Research's lists also hold item 6's files (`src/run/window.rs`, `src/trust_record.rs`,
  `tests/contract_bounded_readers.rs`, the derived `pump/` and `run/` test files), each marked *held on F1*. Implement
  touches none of them before F1's answer.

## Test commands
33 entries:
- fmt and clippy;
- the three crate unit runs;
- 8 control entries, one per guard group;
- the class listing and the class-line count;
- the build;
- the mutants-profile read of `cli_answer`, `cli_version_gate` and `contract_lints`;
- `r2-scratch` and `r2-locked`;
- two dedupe counts;
- the host read;
- the three harness layers;
- the operator legs: the addendum probe, the smoke trio, pre-push, hygiene, push and `ci.py`.

12 one-shot `gate.py control`s, ids of 16 characters at most, each `--timeout 600`, each on its own filtered entry.

## Concretization preview
None. No capability is claimable by this chunk: the pool's v2-14, v2-15, v2-28, v2-30 and v2-31 are owned by
«Console scaffold», «Console actions» and «The fleet on the founder's host». This chunk links nothing.

## The block the directive asks (item 8), verbatim where a tool printed it

**The union** (`rec.py rows-union`):
```
architecture: rows 1049 · packs 11 · picked 157 (68 by every draw) · standing 31 · 50064 B
security-plan: rows 667 · packs 7 · picked 103 (61 by every draw) · standing 13 · 34823 B
design-system: rows 361 · packs 4 · picked 105 (68 by every draw) · standing 0 · 27312 B
layout-templates: rows 238 · packs 3 · picked 54 (33 by every draw) · standing 0 · 12856 B
test-plan: rows 1387 · packs 14 · picked 259 (132 by every draw) · standing 65 · 67847 B
obs-plan: rows 1056 · packs 11 · picked 185 (104 by every draw) · standing 28 · 51881 B
a11y-plan: rows 244 · packs 3 · picked 77 (49 by every draw) · standing 11 · 22138 B
rows-union: 7 master(s) · picked 940 of 5002 · rule any
```
The rows files' sum is 266,921 B, past the 250,000 B whole-read bound. P4 read them by search, never whole.

**Every cited row in the union or its standing list: yes.** All 30 handles the plan cites were checked by a grep of
the rows files and standing lists. 25 stand in a rows file; `decision/test-seams-compiled-out`,
`decision/no-seam-for-foreign-peer` and `decision/real-cli-local-only` stand in `security-standing.md`;
`decision/real-claude-local-only`, `decision/no-orphan-modules` and `decision/mutation-at-epoch-boundary` stand in two
rows files each. 0 were not found.

**planlint, whole** (rev 3):
```
planlint v1.25 · a61c2794
fence: 33 entries · plan: doc:2026-11-27-epoch-5-cleanup/plan · research: doc:2026-11-27-epoch-5-cleanup/research
check 9 · CI-outcome reads: 1 · leg operator exempt: 1 · Platform issues consulted: none · `none — no runner-only bull…`
check 9 · runner-only scope bullet: not read — the letter's
check 4 (5) · leg = 'live' entries: 0 · Harness rules consulted: filled · ``.claude/rules/verification-harness.md` and…`
check 4 (5) · leg entries with a content atom: 5 · note without `atom from`: 0
check 4 (13) · operator CI-outcome reads: 1 · not ci.py conclusion: 0
check 4 (13) · job-log reads by pattern: 0
check 4 (14) · base-less git change reads outside the operator and round legs: 0
check 4 (15) · grep → wc -l counts asserting exit 0 and last line 0: 0
check 4 (16) · guards whose exclusions leave a new file in: 0 of 0 with an exclusion
check 4 (18) · entries reading a master's or a base's path the store now holds: 0
check 4 (19) · absence probes against the test names their own plan adds: 0 · probes read 0
check 4 (17) · entries reading a path the wrap writes: 0 of 24 asserting — listed, P5's read
check 4 (17) · a path the wrap writes, read from inside a program an entry runs (a suite, the ledger's tool): not read
check 2 · ledger-note entries (matrix#): 0
check 2 · entries naming no master and no matrix#: 0
check 5 · placeholders outside code: 0
check 12 (c) · step references the plan's steps lack: 0 of 30
check 12 (d) · «unchanged» claims on a name another part of the plan names: 12 of 16 — listed, P5's read
  line 97 · «unchanged» · pump.rs (step 11) · drain (step 11)
  line 131 · «unchanged» · src/cmd/open.rs (step 19)
  line 158 · «untouched» · PATH (step 8) · hyprctl (step 16) · true (step 11) · foot (step 16)
  line 164 · «byte-equal» · src/cmd/revive.rs (Files to modify) · src/cmd/adopt.rs (Files to modify)
  line 171 · «unchanged» · revive.rs (Files to modify) · adopt.rs (Files to modify) · started (step 16)
      start (step 13)
  line 174 · «unchanged» · revive.rs (Files to modify) · --list (step 6) · trusted (Files to modify)
  line 179 · «byte-identical» · src/cmd/open.rs (step 13) · refused (step 5) · pub(super) (step 18)
      src/cmd/up.rs (Files to modify)
  line 181 · «unchanged» · up.rs (Files to modify) · tests/contract_error_bodies.rs (Files to modify)
      UP_REFUSED (Files to modify)
  line 183 · «unchanged» · UP_REFUSED (Files to modify)
  line 538 · «unchanged» · NEXTEST_PROFILE=mutants (step 3) · tests/contract_lints.rs (step 2)
  line 548 · «unchanged» · viola-state (step 12) · pid1-stop (Test Commands) · scan-list (Test Commands)
      end-kind (Test Commands) · holds (step 11)
  line 550 · «untouched» · r2-scratch (step 17) · r2-locked (step 17) · placement= (Test Commands)
  a claim on a name no other step, Files to modify or Test Commands names is not listed — P5's read
check 4 (4) · criterion evidence paths the plan names nowhere else: 0 of 3
hits: 0 — resolved by its check (validation.md 2 · 5 · 9 · 4 (4), (5), (13)–(16), (18)–(19) · 12 (c)); 0 is not a pass
```
I read each 12 (d) claim against its steps. Each is a pure-move or unchanged-behaviour claim that holds after the
chunk; none is contradicted.

**[RETYPED] lines and re-spawns** (`fanout.py check`):
```
security-pick-p4 · [RETYPED] line 92: decision/plugin-files-rewritten-every-start is no row of the pack — read as decision/plugin-files-rewritten-each-start, the pack's row 92
security-pick-p4.d2 · [RETYPED] line 28: decision/live-rig-mb-arm-keeps-only-closed-words is no row of the pack — read as decision/live-rig-mb-arm-keeps-no-output, the pack's row 28
tests-pick-p13.d3 · [RETYPED] line 16: contract/coverage-failure/llvm-cov-exit-N is no row of the pack — read as contract/coverage-failure/llvm-cov-exit, the pack's row 16
tests-pick-p12 · [EVERY_ROW] line 46: decision/internal-harness-subcommands-off-agent-surface — the pack's order holds decision/host-excluded-mutants here
```
`tests-pick-p12` got one re-spawn (`pick-p2-r1`), which passed: `VERDICT: 1 of 1 pass`. Its first file is
`.raw-tests-pick-p12.md`. `tests-history` printed 51 `not a marker` rows (it bolds `amd-…` entry ids, not markers) and
passed `[CITES]`.

**Non-zero exits, refusals, UNPARSED / INDETERMINATE / NOT DERIVED:**
- **Bash guard refusals, each re-issued once in the passing form:**
  - `cd` into `references/` (P2), refused: `Blocked: cd (…/andromeda-phase/references) moves the session cwd…`;
  - `cd` into the run dir (P4), refused the same;
  - a pipe of `sidecar.py --help` into `head` (P3), refused: `Blocked: a pipeline tool's output piped into a filter…`.
- **Baseline reds, each the red this chunk builds against:**
  - 8 control entries exit 4 (`no tests to run`);
  - the class-line count exit 1 (last line 0);
  - `r2-scratch` and `r2-locked` exit 1 on `first=parent-not-live` (no precondition yet);
  - the two dedupe counts at last line 2.
- **`ls`, `find` and `grep` reads exiting non-zero on an absence:** the `outcomes.json` look-up, the census grep of
  `dialogless_home` in `cross_killed_test.rs`.
- **The fanout-contract read** was capped at lines 1–401 of 487, `PARTIAL view`; §The batch, §Render, §Check and
  §Re-spawn were read whole.
- **The recursive verify-caller graph query** over-matched through a name collision (184.7 KB of fake-agent symbols).
  Its trace row stands; the census was taken by a scripted scan instead.
- **The first census pass** read 13 functions (it missed bare imported `verify(` calls); the refined pass read 35.
- **The research draft's census counts** were 8 in the class and 27 at the default, corrected to 7 and 28 before the
  `put`.
- No UNPARSED, no INDETERMINATE and no NOT DERIVED line appeared from any tool.

**Where a letter was wrong or silent:**
- **The directive and `gate-contract.md` §Control disagree on control ids.** The directive asks ids of 1–16 characters,
  and §Control allows 1–48. The plan keeps ≤ 16.
- **§Control is silent on a non-runner entry.** It reads a case only from a test runner's result line, so item 5's
  script entries cannot carry a control. Their P5 baseline red, plus the must-pass green over a clone
  (`controls-p5.md`), stands as their remove-the-guard reading.
- **Check 4 (9) and a leg entry with no recorded output.** It asks a must-pass reading for a literal copied from another
  representation. The addendum probe's atom has no recorded output on the host, so its first firing after step 8 is
  named as that reading.
- **The code-audit letter's addendum channel** (`audit-pass.md:32-38`) is the audit run's own `proposals.md` and a
  `.late.json` beside it. That run is sealed, and no letter says how a later chunk lands an addendum into it. It is
  asked at step 7's stop.
- **The audit's `epoch.diff` was not sealed** with its run. Step 6 regenerates it from the sealed `population.txt` and
  checks the 372.

## Beyond the plan
| row | proposal |
|---|---|
| 1 | F1 (the operator's, `founder-forks.md`): on «split», a new entry «Epoch 5 cleanup — the three splits», placed right after «Epoch 5 cleanup» and ahead of «Console scaffold», carrying item 6's carry `cry-cnhf6m4x` word for word; the wrap's route adaptation writes it. On «keep», no row: a plan revision plans item 6 here. |
| 2 | The addendum's landing place: the overseer's word at step 7's stop (the sealed audit run's `proposals.md` and `c-mutation-viola.late.json`, or where he names). |

## Card notes
- **Size:** 608 lines with the fence (about 235 of them), so about 370 of prose, inside the 150–400 band.
- **Aggregate B:** 0 masters read `picked 0`.
- **Validation-1:** aligned with `doc:2026-11-27-epoch-5-cleanup/scope` rev 3 and the working entry.
- **Scratch left, named, none removed:**
  - the r2 control clone `$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/9ef63c91-382d-4fd3-9e83-88fd34b22892/scratchpad/r2clone`;
  - the `mktemp -d` dirs of the r2 baseline and control firings, under `/tmp`;
  - `scratchpad/split/`, the tokei measurement copies.

Apply? (yes / review / cancel)
