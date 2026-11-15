# Founder forks — 2026-11-14-status-line-interception-removed (phase P4, away form)

Chunk: «Status-line interception removed» (viola-0.2.0, Epoch 2). Run: `.andromeda/runs/2026-11-14T19-10-38-phase/`.
Raised under the operator's phase directive, item 7 (`inputs#I1`): «whether this removal takes the same exception is a
fork that is his — put it on the away form's file with the measured compatibility facts (new reader / old files; old
reader / new files), and stop there.» The phase stops at P4 on this file; the plan is written once the answer is
appended here.

## Q1 — Does this removal keep every own-format `v` at 1, as the budget removal did?

**What leaves an own format.** The instance snapshot's `data` loses `statusline_command` (`snapshot.json`, `v` 1). The
ledger stamps' `rows` map loses the two row ids `statusline-override` and `statusline-shell` for a version this build
stamps (`ledger/stamps.json`, `v` 1). No event kind leaves (`hook statusline` emits no event), no channel `params`
field and no `list` document field leaves. Process-log lines carry no `v` (their schema filename is their version).

**The rule as it stands** (architecture · decision/protocol-version-bump-rule): «Adding a field or event kind is
additive and needs no bump of `v`; removing a field or changing its meaning bumps `v`.» Its one except: «Removing the
budget gate's fields and event kind kept every own-format `v` at 1 (snapshot, channel params, event line, list
document); readers skip the removed keys and kind.» That except was your ruling for the budget removal only.

**Measured compatibility facts** (research of this chunk, HEAD 7416035f; `doc:2026-11-14-status-line-interception-removed/research` §Measured compatibility facts):
- New reader, old snapshot: `InstanceSnapshot` has no `deny_unknown_fields` (`crates/viola-state/src/snapshot.rs`, 0
  hits), so a snapshot holding `statusline_command` reads whole; the key is skipped.
- Old (0.1.0) reader, new snapshot: at `build/viola-0.1.0` `statusline_command` is optional, so its absence alone would
  parse; but that reader requires `budget_paused`, which this branch stopped writing at the budget removal, so it
  already replays every snapshot this branch writes. This removal changes nothing for it.
- New reader, old stamps: the verdict reads the relied rows alone, so a stamp still holding the two retired rows reads
  verified when every relied row passes (the pattern the budget removal proved for `statusline-rate-limits`).
- Old (0.1.0) reader, new stamps: a version re-stamped by this build holds only its 21 rows (`merge_stamp` writes the
  run's rows alone), so a 0.1.0 reader reads it unverified — already true since the budget removal (that build stamps
  without `statusline-rate-limits`).

**Options, recommendation first:**
1. **Keep every own-format `v` at 1 (Recommended).** The except of decision/protocol-version-bump-rule widens to name
   this removal too (the snapshot's `statusline_command` and the stamps' two row ids). Why: in both directions the
   facts are the budget removal's own, unchanged — new readers skip the removed key and rows, and old readers were
   already cut off by the budget removal. A bump would add a second supported `v` to the snapshot envelope and the
   stamps reader for no reader it protects. Trade-off: the rule's text gains a second named exception, and the
   «removing a field bumps `v`» rule now has two removals that did not bump it.
2. **Bump `v` by the rule as written** — the snapshot envelope to `v` 2 (and the stamps envelope to `v` 2). Every
   reader before the bump then treats a new snapshot as unsupported and replays the log (no endpoint, so an older
   client reads the instance unreachable), and treats new stamps as malformed (unverified). New readers must accept
   `v` 1 and `v` 2 alike (contract/snapshot-envelope/v: «above 1 is unsupported»; contract/ledger-stamps). Why not
   recommended: it costs a reader change and a contract change in two envelopes to protect no reader that is not
   already cut off.
3. **Bump only the snapshot envelope, keep stamps at `v` 1** — the stamps' rows map is keyed data, not a fixed field
   (one could hold the map's keys outside the rule). Same snapshot cost as option 2; stamps unchanged.

**The answer:** _(appended here, verbatim, with who gave it and when)_

### The operator's message (session, 2026-11-14)

> Answer to the fork in founder-forks.md: option 1, keep every own-format `v` at 1 — a delegate answer, PROVISIONAL,
> the rule's exception escalated at the wrap for his own word. Read
> ~/dev/projects/additional/viola-overseer/relay-status-line-q1.md whole, append it under the fork and snap it, then
> resume the phase at P4 and run it to the P5 card as the skill says. No cargo build or test run but what the plan
> cannot be written without (the host is shared); name each on the card.

### The relay, whole (`~/dev/projects/additional/viola-overseer/relay-status-line-q1.md`)

# Phase of 2026-11-14-status-line-interception-removed — the answer to founder-forks.md's fork (a delegate's answer, PROVISIONAL)

**Option 1 — keep every own-format `v` at 1.** PROVISIONAL: the driving overseer answers as the delegate while the
operator is away; his own word ratifies or changes it, and the question is on his list for his next look.

Why this one while he is away: it is the option that stays open (a `v` written as 2 cannot be taken back without a
second break; a bump added later is additive), and it is the answer he gave for the budget removal on the same
measured facts — but his ruling was for that removal only, so this one is not settled by it.

What it binds in the plan: the measured compatibility facts stated as measured, with a test for each direction the
tree can run (a new reader over a snapshot that still holds `statusline_command`; a stamp that still holds the two
retired rows), the old shapes in fixture files outside any identifier probe's paths; the exception to
`architecture · decision/protocol-version-bump-rule` an EXPECTED AMENDMENT FOR THE WRAP, PROVISIONAL, on the wrap's
card for his own word; the steps a bump would add listed on the P5 card, so a change of his word is a cheap revision.

The `workspace.repo` exception rows that die with the status-line fixtures: put their retirement on the P5 card as a
row of its own (the entry does not name them).
