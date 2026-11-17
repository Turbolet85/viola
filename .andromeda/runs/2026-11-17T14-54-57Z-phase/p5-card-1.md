# P5 review card 1 — 2026-11-17-first-readings-on-the-standing-rig «First readings on the standing rig»

The plan's render to open in an editor: `.andromeda/cache/docs/2026-11-17-first-readings-on-the-standing-rig/plan.md`.
No plan stands in a chunk's folder; writing in that file changes no record until a `put`. A diff of the chunk's
records: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-17-first-readings-on-the-standing-rig`.

## Goal
The first live readings on the committed `viola-live-rig`, as closed words in this chunk's evidence:
- **R3b**, «a real wake reads 1»;
- **R3a**, a fired wake's prompt-submit `source`;
- **R3c**, the hook-latency window;
- **the external-imports modal's shape**;
- **R2's `holds-text`**, explained from its kept frame with no start.

Plus one CI step: viola-e2e's `harness_lifecycle`, `cli` and `harness_tied` join the `ci` 20 s kill line, the
never-ready case on a shortened 2 s wait carried on `BootOptions`. Everything rests on the founder's own answers
(inputs#I2):
- R4 leaves for «The 2.1.325 recorded fixture set»;
- the cap is 4 starts + 3 spares;
- the rig's own capture is ratified;
- no key is typed.

No product crate changes.

## Steps (11)
1–3 the nextest step (`BootOptions.ready_deadline`, the never-ready case at 2 s with a wall bound below 10 s, the
three binaries on the 20 s line) · 4 the hidden `capture` verb (filtered key names + a closed `source` word + its
own clock) · 5 the capture hook in the private config and the imports seed · 6 the wake arms `R3B` and `R3`
(`/loop` interval or dynamic, read from the 2.1.325 bundle) · 7 the imports arm and the no-key group-kill stop ·
8 labels and verbs · 9 three rehearsals over the fake agent · 10 `holds-text.md` from the kept frames · 11 the live
round `R3B` → `R3` → `R3-326` → `IM`, stopping before `R3` on a `wake-request: none`.

## Top acceptance criteria (10 in the plan)
- **The nextest step:**
  - `cargo nextest run -p viola-e2e --profile ci -E 'binary(harness_lifecycle) | binary(cli) | binary(harness_tied)'` exits 0 with no TIMEOUT line;
  - the three names are on the filter;
  - the never-ready case's wall is measured. Predicted 20.459 s → about 2–3 s; the report records what the run measures.
- **The controls:**
  - the `ready-deadline` control reads `holds`;
  - so do the `capture-keys` and `capture-source` controls.
- **The live starts:**
  - `R3B` is fired first and names `wake-request`, both sends' wake counts and a whole teardown;
  - `R3` / `R3-326` record `wake`, `wake-origin`, `capture-source`, `capture-keys`, `latency-ms` and `screen-ms`;
  - `IM` records its `modal` reading, `keys: none` and `stop: killed-no-key`;
  - at most 4 base starts and 3 spares are ledgered, and each spent spare's lost shape is named.

## Touchpoints
- **New:**
  - `crates/viola-e2e/src/rig/capture.rs`
  - `crates/viola-e2e/src/rig/wake.rs`
- **Modify:**
  - `crates/viola-e2e/src/bin/viola-live-rig.rs`
  - `crates/viola-e2e/src/rig/mod.rs`
  - `crates/viola-e2e/src/rig/isolate.rs`
  - `crates/viola-e2e/tests/rig_rehearsal.rs`
  - `crates/viola-e2e/src/harness/boot.rs`
  - `crates/viola-e2e/src/harness/session.rs`
  - `crates/viola-e2e/src/harness/run/perf.rs`
  - `crates/viola-e2e/src/bin/viola-harness.rs`
  - `crates/viola-e2e/tests/cli.rs`
  - `crates/viola-e2e/tests/harness_lifecycle.rs`
  - `.config/nextest.toml`

## Test commands (20 entries)
| Entries | What | Baseline |
|---|---|---|
| 1–2 | fmt, clippy | seen green |
| 3 | unit `rig::` | green, 29 passed |
| 4 | `rig_rehearsal` | green, 9 passed |
| 5 | the three binaries under `ci` | green; the case 20.459 s under the 30 s default |
| 6 | the control entry, default profile | green, 20.356 s |
| 7 | the filter count | red, exit 1 — the predicted red; must-pass control green in a scratch clone, `controls-entry7.md` |
| 8 | release-check | green |
| 9–12 | round probes and the plain `viola` build | — |
| 13–16 | live `R3B` · `R3` (`--id` the request word) · `R3-326` · `IM` | — |
| 17–20 | operator: pre-push · hygiene · push · `ci.py` | — |

**Controls** (implement's one-shot `gate.py control`):

| Control | Entry | Kill |
|---|---|---|
| `ready-deadline` | 6 | `--timeout 120` |
| `capture-keys` | 3 | `--timeout 300` |
| `capture-source` | 3 | `--timeout 300` |

With its guard removed, each control's case is predicted to fail (not to time out). This is unmeasured until implement fires them.

## Capabilities
None claimed. v2-36 is «A fired scheduled wake keeps the wheel»'s. v2-19, v2-32 and v2-35 are verified, and their readings ride
as Expected amendments (ledger notes). No concretization preview applies.

## Expected amendments (wrap), in the plan's notes
- **test-plan:**
  - ~ `decision/verify-driven-tests-20s-kill` and ~ `decision/ci-stamped-home-kill-line` (the filter goes from 25 to 28 binaries);
  - ~ `contract/harness-boot/timeout`;
  - ~ `decision/standing-live-rig`.
- **security-plan:**
  - + the rig's own capture row, pattern rul-bznresq5 (the founder's own answer, inputs#I2 item 3, relayed verbatim). This makes the wrap halt once on its answer card;
  - ~ `decision/live-round-rig-keys-own-pty` (the no-key group kill).
- **Ledger notes:** `matrix#v2-35` · `matrix#v2-19` · `matrix#v2-32`.

## The directive's item 12 block (verbatim where a tool printed it)
**The union, per master:**
```
architecture: rows 948 · packs 10 · picked 210 (116 by every draw) · standing 9 · 61124 B
security-plan: rows 607 · packs 7 · picked 159 (102 by every draw) · standing 10 · 44796 B
design-system: rows 315 · packs 4 · picked 61 (36 by every draw) · standing 0 · 15062 B
layout-templates: rows 222 · packs 3 · picked 60 (31 by every draw) · standing 0 · 12459 B
test-plan: rows 1338 · packs 14 · picked 261 (133 by every draw) · standing 19 · 65622 B
obs-plan: rows 1024 · packs 11 · picked 277 (152 by every draw) · standing 19 · 66716 B
a11y-plan: rows 235 · packs 3 · picked 47 (22 by every draw) · standing 14 · 13739 B
rows-union: 7 master(s) · picked 1075 of 4689 · rule any
```

**How the inputs were read:**
- **Rows:** read by search, not whole. The union's sizes sum to 279 518 B, past the 250 000 B line.
- **Cited rows:** EVERY row the plan cites is in the union, all 13 handles, each found once in its master's rows file. Two were also looked up small by `rec.py find` (`decision/ci-stamped-home-kill-line`, `contract/harness-boot/timeout`) and stand in the union too.
- **Standing lists:** all seven read whole.
- **History files:**
  - tests, security, arch and obs read whole;
  - design, layouts and a11y read by search for this chunk's terms (rig · nextest · wake · imports · kill · boot · capture · placeholder · holds-text). The letter asks each history file to be read, so this is a departure from it, named here.
  - Tests and security were read at P4 before the plan was written. Arch and obs were read after the plan's first `put`, and nothing in them moved the plan.

**`[RETYPED]` lines (3), no re-spawn (0 of 170 agents):**
```
arch-pick-p4.d3 · [RETYPED] line 80: decision/fake-agent-test-seam-variables is no row of the pack — read as decision/fake-agent-test-seams, the pack's row 80
tests-pick-p9.d2 · [RETYPED] line 71: decision/synthetic-fixture-text-only is no row of the pack — read as decision/synthetic-fixture-text, the pack's row 71
tests-pick-p12 · [RETYPED] line 96: contract/coverage-failure/llvm-cov-exit-N is no row of the pack — read as contract/coverage-failure/llvm-cov-exit, the pack's row 96
```

**Non-zero exits and refusals:**
- **Gate entry 7's baseline:** red, `exit 0 ✗ (exit 1)`, in the tree and in clone `ctl7u`; the predicted red.
- **Bash guard, three refusals, each re-issued in the form it passes:**
  - twice: `Blocked: cd (…) moves the session cwd for every later call`;
  - once: `Blocked: a pipeline tool's output piped into a filter`.
- **The stamp-ahead hook:** `Stamp ahead of the clock: 2026-11-17T16:20:00Z (+6 min) written to …/plan.md`. The plan's `Generated:` was an estimate; it was re-read from `date -u` and corrected. This is a recurrence of the deferred learning «A timestamp written into any record … is read from date -u».
- **An expected absence:** `ls` of the two new files, exit 2.

**`UNPARSED` / `INDETERMINATE` / `NOT DERIVED` lines:** none printed.

**Where a letter was wrong or silent:**
1. **The history read:** no whole-read bound is stated for history files beside the rows' 250 000 B line, so the partial read above was my own call.
2. **The evolve system reference:** this phase used the copy read at new-session (it is byte-synced ×4, stamp `evolve v1.4`), not a fresh read of phase's own file.
3. **The live legs' `env`:** the plan template asks `env` on a `leg = 'live'` entry. The rig reads `HOME`, but these entries declare `env = []`, as the previous chunk's did.
4. **R3 depends on `R3B`:** the round's `--id` substitution is the only way a later leg can read an earlier leg's word, so `R3` and `R3-326` are fired by `--entry` with `--id`, never by one `--live-legs` call.

## Beyond the plan
- **Re-pin R4:** R4, the live `viola verify --record`, and the record-home rule's live half (cry-ksqqok4x) leave this chunk. They are re-pinned on the existing entry «The 2.1.325 recorded fixture set» as a carry, at the wrap's route-resolve, on the founder's own words (inputs#I2 item 1).
- **An owner for R2's open question:** whether the CLI's placeholder hint is told apart by a cell attribute is unmeasured. Proposed owner: «The 2.1.325 recorded fixture set» (the next entry to read 2.1.325 screens), carried by the wrap.
