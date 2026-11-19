# Outcome — 2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin

**Run:** 2026-11-19T07-06-34Z-implement · plan rev 13 · normal (pending) mode · re-entry at step 19

**Outcome:** green. Every gate this run fired read green. Step 19a held. Next is the operator pass (entries 19–27),
then the wrap in a fresh window.

## Gates
- Block firing 1, entries 1–8, all green on the first firing: `entries 27 · green 8 · red 0 · recorded 0 · timeout 0 ·
  not-run 19`. The R1 case ran green inside entries 6 and 7. That closes nothing: the red stays open (CLAUDE.md).
- Step 19a: entry 14 (`L1`) was read from its own log, `14.log` of run `2026-11-19T06-49-43Z-implement` (1881 B, sha
  `a4eeb096`, the same as the trail). It was not re-fired. All 34 content atoms of the mended expect held, plus `exit
  0`. The table is in `evidence/live-starts.md`, «Entry 14 against the mended expect».
- The host was read before the round at 2026-11-19T07:09:42Z: `QUIET - 0 s stalled on IO`.
- The round, `--live-legs --entry 17` (`evidence/round-071000Z.txt`):
  - Entry 15: `census: 0`.
  - Entry 16: the debug build, `998fe6ff80a30578`.
  - Entry 17 (`L2`): green.
  - `round: COMPLETE · legs fired 1/1 · entries 27 · green 3 · red 0 · recorded 0 · timeout 0 · indeterminate 0 ·
    not-run 24`.
- Entry 18, fired once by `--operator 18`: `green · exit 0 · 0.31s · 10 B → 18.log · history: unmoved`. Its log reads
  `census: 0`.
- The `bt-line` control was not fired again (plan step 19). It held at `evidence/control-1.txt`.
- No matrix capability is owned by this chunk: the coverage view holds 0 rows.

## L2: what it read (the summary is in `evidence/live-starts.md`)
- The send: `send-1-reply: not-delivered:no-prompt-submitted`, `context-submitted: no`.
- The transcript: `context-transcript: absent`, so `context-tags: unread` and `context-post: none`.
- The stop and teardown: `keys: none`, `stop: ctrl-c 3`, `census: 0`, `credentials: removed`, `scratch: absent`,
  `user-config: changed-by-others`.
- **Named gap, not a red:** the acceptance says `L2` reads `context-post` «from a readable transcript».
  - That wording is not met: the transcript was absent, so the tag half was not read.
  - The entry's expect admits `absent`; it refuses only `unread`.
  - So `context-post: none` holds only the prompt-submitted half (no). The tag half is UNREAD, not zero.
  - This is recorded as read, never as verification, and it goes to the wrap.
- The cap: `L1` spent at 06:54:18Z and `L2` spent at 07:09:57Z, both ledgered before each start. `L1-2` is unspent.

## Files changed
None this run. The tree is the one the stopped run `2026-11-19T06-21-00Z-implement` wrote, unchanged since.
`gate.py scope`: `scope: clean — changed 10 · listed 9 · recorded 1 (companion 0 · mechanical 0 · in-intent 1 ·
widening 0) · absorbed 0 · excluded 280`.

## Deviations
- The scope record's line: ``- `crates/viola-agent-claude/src/ledger/test_cases_stamps.rs` · in-intent · serves
  step 1 · self``.
- Carried from run `2026-11-19T06-21-00Z-implement`:
  - `others_word` takes the two maps that `stamp_others` reads.
  - The `settings-keys` and `needle-settings` words were settled there.
  - `CX`'s tag read polls within `SUBMIT_BOUND`.

## Guard pass lines
None: the block holds no `gate.py guard` entry.

## Process hygiene
| process | started by | final state |
|---|---|---|
| gate block entries 1–8 (cargo, nextest, the harness, fake agents, rig rehearsals) | this run | terminated (the census below finds none) |
| round entries 15–17; `L2`'s `viola run` and its one `claude` 2.1.328 | a leg | terminated (`census: 0` in L2's summary; entry 18 `census: 0`) |
| 7 `claude` sessions (the overseers and builders, `up.sh`) | NOT DERIVED: not this run's | left running: not this run's |

The census (`gate.py census --comm viola-live-rig --comm viola-fake-agent --comm claude --args
projects/viola/target/debug/viola --args projects/viola/target/release/viola`) read `census: 7 running`. All 7 are
the host's `claude` sessions; no rig, fake agent or viola build of this tree was running.

## Decisions & corrections
- The operator's word for this run: «Read the host before L2: Viola own gate stall is no neighbour — wait for a
  QUIET reading, then fire». The first read was QUIET (07:09:42Z), so no wait was needed.
- The directive (`inputs#I18`) is superseded by plan rev 13 where it speaks of the fold or of `L1` as ahead (the
  operator's word). `L1` was not re-run.
