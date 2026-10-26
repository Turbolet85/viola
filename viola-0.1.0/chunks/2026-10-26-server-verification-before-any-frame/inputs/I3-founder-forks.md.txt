# Founder forks — phase P4 of 2026-10-26-server-verification-before-any-frame

The founder is AWAY (the overseer's directive, `inputs#I1` item 5: «He is asleep now, so a widening fork is held as
such and never answered from a recorded direction»). Per gate-contract.md §Scope *Whose word*, his forks take the away
form: written here, the step stopped. A delegate's answer is PROVISIONAL until his own word, and a fork on a boundary
widening is answered by its most conservative option. P4 resumes on the answer appended below.

## Fork 1 — what a verified client does with an unreadable or newer `snapshot.json` (CARRY 7), and whether the snapshot readers move to the replay (CARRY 8)

**The freight, as the working entry holds it.**
- CARRY 7 (chunk 2026-10-10-self-healing-state, the operator's direction at its wrap,
  `viola-0.1.0/chunks/2026-10-10-self-healing-state/inputs/I4-relay-1.md.txt`): «what a client's pre-check does with an
  unreadable or newer `snapshot.json` is undecided and unchanged. `read_snapshot` reads either as absent, so
  `live_endpoint` finds no endpoint and the verb exits 21; the classified read and the log replay stand as library code
  that no client calls. A client that connected on a replayed endpoint and a heartbeat alone would change the dated
  pre-check's outcome: a boundary matter, the founder's to rule before this entry plans it.»
- CARRY 8 (chunk 2026-10-11-the-board-viola-list, the review card the founder approved): the other snapshot readers
  «still call `read_snapshot` and take no replay, which keeps `verification-matrix.json#v1-41` open and E5's steps 1–3
  unbuilt». P3 corrected the list at HEAD: `src/cmd/client.rs` holds no call any more; the CLI verbs' read is
  `viola_state::liveness::live_endpoint`, and under `src/` four product readers remain (`hook.rs` ×2, `statusline.rs`,
  `run.rs`'s collision check) — `chunks/2026-10-26-server-verification-before-any-frame/research.md` Sweeps.

**What P3 established that bears on it.** A replayed snapshot carries no `endpoint`, `pid` or `started_at`
(architecture §Standard Contracts, Snapshot envelope). The server verification this chunk builds compares the connected
server with the snapshot's own `pid` and `started_at`, so a client holding a replayed snapshot has nothing to verify
against: under this chunk it can never write a frame on a replayed or rebuilt endpoint, whichever option is taken.

**Options, in order:**

1. **Keep today's outcome (Recommended — the most conservative option).** Every reader keeps `read_snapshot`: an
   unreadable or newer snapshot reads as absent, so the CLI verbs exit 21 (`instance-dead`, the not-running hint), the
   MCP tools return `instance-unreachable`, the hook fails open — exactly as today. No client takes the classified read
   or the replay, and none writes a `state-recovered` line. CARRY 8's reader move (`v1-41`, test-plan §6 E5 steps 1–3)
   is NOT built here: it leaves this entry on the P5 card's `## Beyond the plan` as a re-pin, held for your own word
   (its destination proposed on the card). Why it wins: it changes nothing across the boundary, and the replay's two
   product readers (`viola revive`, `viola list`) stay its only ones, as your board ruling left them. Its cost: `v1-41`
   stays open past this chunk.
2. **Move the four readers and `live_endpoint` onto `read_snapshot_or_replay` now.** A replayed snapshot then reaches
   each client as a snapshot with no endpoint and no identity, so every client still reports unreachable or fails open
   (no frame is possible without a recorded `pid` / `started_at`), but each such read writes one
   `state-recovered{snapshot-replayed|snapshot-unsupported-v}` line into the client's role file (`cli-<name>`, `hook-<name>`,
   `mcp`), and `run`'s collision check would classify a replayed instance by its heartbeat alone (live or gone, never
   stale). It closes CARRY 8 and lets `v1-41` be claimed with E5's steps 1–3 built here. Its cost: a new log crossing
   in three client roles and a changed collision outcome on a replayed snapshot — a boundary matter by CARRY 7's own
   words, which only your live word ratifies.

**What counts as his word:** the founder's own answer naming option 1 or 2 (or another form), given live or relayed
verbatim. A delegate's answer is provisional and, the fork touching a boundary, may only be option 1.

## Leans P4 takes without a question (each settled by an artifact; shown so the answer covers the whole picture)
- The strict-modes half of the dated gaps stays with "Home and code-bearing file integrity": that entry's own freight
  names every verb of this chunk («this entry adds `send` to the strict-modes entry points …»). This chunk closes the
  server-identity half only; the gaps' strict-modes half stays dated until that entry.
- macOS: `peer_creds()` gives no pid there, and the euid and socket-directory checks macOS's verification rests on are
  "Unix endpoint and home hardening"'s by its own freight («the directory verification … and the peer euid check stay
  this entry's»). This chunk's pid + start-time check is Linux and Windows; macOS keeps the recorded gap.
- CARRY 2 (a hook reaching an exiting wrapper's own Windows pipe): the exiting wrapper IS the recorded process, so a
  passing check is the right verdict there and the check is not built to close the window; it is named unclosed, and
  its re-pin rides the P5 card.

## Answer
_(appended here when it arrives, then snapped with `--message-file`, `--kind relay` and an `--origin` naming who
answered and when)_

The answer, verbatim, in the session's message of 2026-10-26 (authority: delegate the Viola overseer, 2026-10-26, 08:08Z —
PROVISIONAL until the founder's own word; gate-contract.md §Scope *Whose word*):

Please resume the phase per the skill with option 1 on the fork, keep today's outcome, as the overseer's PROVISIONAL delegate answer (08:08Z; the founder is asleep, away form): it stands only until his own word, and the move to the replay (v1-41, E5 steps 1-3) is re-pinned on the P5 card for his word. Append this answer to founder-forks.md, then resume P4.
