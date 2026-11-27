# Founder forks — 2026-11-27-epoch-5-cleanup (phase run 2026-11-27T19-38-11Z-phase)

Written 2026-11-27T20:23:41Z, the operator away; his answer comes through the overseer. The phase goes on meanwhile:
it plans the part that certainly fits first (items 1, 2, 4, 5, then 3 and 7) and holds item 6's steps on this fork.

## F1 — the split: item 6 (the three splits) in this chunk, or a second entry

**The question.** Item 6 — `upped` split in `src/cmd/up.rs`, the restart code out of `crates/viola-pty/src/pump.rs`,
the window and trust steps out of `src/cmd/run.rs` — stays in «Epoch 5 cleanup», or leaves it as a second entry
placed right after it, ahead of «Console scaffold»?

**Figures** (the research document, §Sizing; tokei code lines, the audit's own measure):
- The yardstick, the last corrective chunk `2026-11-22-epoch-4-cleanup`: 11 code files, about 311 changed lines, no
  structural move, one implement run.
- Items 1, 2, 4, 5 (tests and a script) and 3, 7 (two small moves): about 14 files, about 400–500 changed lines,
  8–10 new tests — the yardstick's order.
- Item 6: about 2,300 moved lines in about 7 new or moved files. The audit's size measure counts a file's inline test
  module, so the moves are larger than the product code alone: `pump.rs` 608 product + 1129 test lines, `run.rs`
  779 + 932. Each of the two test modules must leave its file and split into at least two files of at most 800 lines,
  or the next audit reads a new file over 800. Each move is closed by the whole layers green with no test edited
  beyond a re-pointed path.

**Options, in order:**
1. **(Recommended) Split: item 6 becomes a second entry**, «Epoch 5 cleanup — the three splits», placed right after
   this one with item 6's carry (`cry-cnhf6m4x`) moved to it word for word. *Cost:* one more phase and implement
   pass (about one more window), and the next boundary's audit still reads M2 and M3 only after that entry. *Why it
   wins:* this chunk then fits one window by the yardstick, and its one announced mutation run (item 1's addendum)
   scores the root crate's 372 in-diff mutants over code that this chunk has not moved. A move made first would shift
   the `--in-diff` sites the recorded form reads.
2. **Keep item 6 in this chunk.** *Cost:* about 7× the yardstick's volume in one implement window, at the risk of a
   stopped implement and a revision. The addendum run would come after the moves, or before them, with a stop of its
   own. *What it buys:* M2 and M3 read cleared at the next boundary with one entry fewer.

**Read-not-reached:** no boundary widening and no escalate pattern of the playbook is reached by either option (the
four `verdict: escalate` rows read at P3). The split moves no word of the carry.
