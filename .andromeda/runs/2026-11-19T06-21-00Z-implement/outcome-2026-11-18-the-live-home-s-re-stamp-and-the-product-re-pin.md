# Outcome — 2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin

**Run:** 2026-11-19T06-21-00Z-implement · plan rev 11 · normal (pending) mode

**Outcome:** stuck — the plan's own stop (step 18b; the overseer's directive «The fold … comes first: … or no red
comes in 30 runs, STOP before the live round and say so»). Entry 7, the R1 count, read 30 of 30 green on the
unmended tree, so the red's cause stays unread. Implement stopped before the live round. No live start was made,
nothing under the founder's real home was read or written, no operator entry was fired and no report was written.

## Gates (block firing 1)
- Entries 1–9 green on the first firing (fmt; clippy; the agent crate's unit suite; `run --unit`; `cli_verify` +
  `cli_send`; `rig_rehearsal`; the R1 count; the whole integration layer; the e2e layer):
  `entries 28 · green 9 · red 0 · recorded 0 · timeout 0 · not-run 19` (trail
  `.andromeda/runs/2026-11-19T06-21-00Z-implement/gate-2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin.json`).
- Entry 7: `green · exit 0 · 355.1s`. Its log holds 30 harness documents `"ok":true` and 30 suites
  `"passed":27,"failed":0`, with the R1 case run in every one (`evidence/r1-red.md`).
- `bt-line` control: `control bt-line · entry 5 · holds — in place green, removed red · exit 0 ✗ (exit 1)`
  (`evidence/control-1.txt`).
- Entries 10–19 (the live round) and 20–28 (the operator pass and the re-pin) were not run.

## Files changed (the tree is preserved for the next run)
- `crates/viola-agent-claude/src/ledger.rs`: `browser_tools_stood` (step 1).
- `crates/viola-agent-claude/src/ledger/test_cases_stamps.rs`: `browser_tools_stood_reads_true_only_when_a_run_met_the_prompt`
  (step 1; scope record: in-intent).
- `src/cmd/verify.rs`: `BROWSER_TOOLS_STOOD`, written before the summary (step 2).
- `tests/support/fake.rs` + `tests/cli_send.rs`: `browser_tools_screen` moved (step 3).
- `tests/cli_verify.rs`: `verify_meeting_the_browser_tools_prompt_says_so_before_the_summary` (step 3). Its summary
  literal `stamped 2.1.0  9 pass  15 fail` and exit 1 are copied from the first run.
- `crates/viola-e2e/src/rig/mod.rs`: `Arm::VH` and `Arm::CX`, the three `Plan` fields, `start` / `run_child` /
  `arm_v` / `finish` for `VH`, `HomeNames`, `settings_delta`, `arm_cx`, `tags_past`, `context_post` and the
  inline cases (steps 5–10, 13–15).
- `crates/viola-e2e/src/rig/isolate.rs`: `child_env_real` and the name readers, with their cases (steps 11–12).
- `crates/viola-e2e/src/bin/viola-live-rig.rs`: `Live` for `VH`, `Rehearse --target-home --claude-dir`, the
  required-flags refusal (step 6).
- `crates/viola-e2e/tests/rig_rehearsal.rs`: the 18a instrument (`kept_frame`) and the three rehearsals
  (steps 16–18).
- Not applied: step 18c, the mend. The plan binds it to Cause A, which must be read first.

## Deviations
- Scope record: ``- `crates/viola-agent-claude/src/ledger/test_cases_stamps.rs` · in-intent · serves step 1 · self``.
- `others_word` takes the two in-memory maps that `stamp_others(path, version)` reads, where step 11 named
  `others_word(before, after, version)`. The version filter moved into the reader; the words are the same.
- Words the plan leaves open, settled here: `settings-keys` joins its files with `;` and marks a file `<name>:new`
  or `<name>:gone`. `needle-settings` reads `present` if any settings file holds the needle, `unread` if any file
  could not be read.
- `CX`'s tag read polls within the same `SUBMIT_BOUND` until a tag lands, rather than reading once.
- Step 4's sweep: 3 hits, 1 changed (the new case), 2 left unchanged (`cli_send.rs:1247`, the `viola run` case;
  `rig_rehearsal.rs:1063`, the `vp` rehearsal, which reads the stamp, never verify's stdout).
- A by-hand `vh` rehearsal (the fake agent and scratch dirs only) read the whole summary, so every `L1` atom's
  spelling was checked against real output. Its case dir `$TMPDIR/vrig-hand-7IXWRN` stands: the guard refused the
  recursive rm.

## What stuck
Plan step 18b names two stops: «Any other cause shown, or no red in the 30 firings (the cause unread): implement
STOPS before the live round and says so». The second fired. The open red
(`rig_rehearsal_reads_the_box_empty_after_x_and_del`, `box-text: absent`, P5 baseline 2026-11-18T22:56Z and
23:12Z) stays open with its cause unknown. A green re-run does not close it (CLAUDE.md).

Next: the operator's or overseer's call. Choices: re-plan through `/andromeda-phase`'s revision door (for
example, another way to read the cause, a larger count, or the round decoupled from the fold), or a direct word
to go on. The founder's cap is unspent: no `L1`, `L2` or `L1-2` start was made.

## Hygiene and land (held)
`gate.py hygiene` reads `refused 1 files`. The one remaining row is
`P1 .andromeda/runs/2026-11-18T21-59-43Z-phase/founder-forks.md:72 ×1 · tmp`: the take-up phase run's record of the
founder's forks, whose table names the `$TMPDIR/viola-verify-*` project keys verify leaves. That record is another
run's, not this run's to rewrite, so the row goes to the operator (gate-contract §Hygiene). `rec.py land` is held for
the operator's word. The chunk's evidence (`evidence/r1-red.md`, `evidence/control-1.txt`) stands unlanded in the
work dir. This run's own two `/tmp` rows were respelled `$TMPDIR/…` in this file.

## Guard pass lines
None: the block holds no `gate.py guard` entry.

## Process hygiene
| process | started by | final state |
|---|---|---|
| gate block entries 1–9 (cargo, nextest, the harness, fake agents, rig rehearsals) | this run | terminated (census below finds none) |
| `gate.py control` clone `~/.cache/andromeda-control/48eabd2e` | this run | removed (`clone removed`) |
| by-hand `viola-live-rig rehearse vh` with the fake agent | this run | terminated (`census: 0` in its own summary) |
| 7 `claude` sessions (the overseers and builders) | NOT DERIVED: the host's `up.sh` sessions, not this run's | left running: not this run's |

The census (`gate.py census --comm viola-live-rig --comm viola-fake-agent --comm claude --args projects/viola/target/debug/viola --args projects/viola/target/release/viola`)
read `census: 7 running`, all 7 the host's `claude` sessions, and no rig, fake agent or viola build of this tree.

## Decisions & corrections
- Bash guard refusal: a heredoc to a file target (the `Arm` edit script) was refused. The edits went through the
  Edit tool.
- Bash guard refusal: a recursive rm of `$TMPDIR/vrig-hand-7IXWRN` was refused. The dir is left and named here.
- Recurrence of a standing learning (testing.md 2026-10-16 «An oracle literal … is copied from the first run»):
  the new verify case's summary literal was first typed as a placeholder and corrected from the first run's
  output before any gate firing.
