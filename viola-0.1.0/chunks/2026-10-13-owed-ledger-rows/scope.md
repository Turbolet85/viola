# Scope — 2026-10-13-owed-ledger-rows · Owed ledger rows

**Working entry** (`viola-0.1.0/working-route.md`, the first markerless entry, sixth under
`### Epoch 5 — Driver surface`, directly after "Paste newline ledger row"): Owed ledger rows — the relied-on CLI
shapes still without a ledger row become probed, stamped capability-ledger rows (v1-34) — plus six CARRY blocks
(folded below; completeness check `route.py pins --run-dir` → this run dir's trail: six rows on the entry, 1064 ·
1538 · 861 · 1475 · 1648 · 403 chars, no abstention).

**The operator's direction at take-up:** none beyond the invocation (`/andromeda-phase`, no flags). No external
input is snapshotted at P1: every fact below is read from the entry or from a file of this repository.

**CI read at Setup 5a** (the last wrap's flip `14e6232` = HEAD, `ci.py conclusion`, this run dir's trail):
- `14e62329f808`: **green**, checks 15/15, wall 555 s, `ci#38476922170` (push). No red or `not green` to disposition.

## What this chunk builds

The entry owns the six relied-on CLI shapes that "Paste newline ledger row" moved off itself by the founder's own
answer to its P4 fork 3 (`viola-0.1.0/chunks/2026-10-13-paste-newline-ledger-row/inputs/I2-relay-2.md.txt`): each a capability-ledger row with its `viola verify` probe. Today
`LedgerRow::ALL` holds eighteen rows (`crates/viola-agent-claude/src/ledger.rs:42`, re-read at P1), all gating
`cli_verified`; the last is `paste-newline`.

The entry's own CARRY (CARRY 1, minted at that wrap) states the cost frame, verbatim in substance:
- each shape needs a **widening ruling on the `viola verify` child set** and a **live-start cap of its own** at
  take-up;
- one `viola verify` round is five live starts, and a fifth Run B paste added none;
- the eighteenth row left every home stamped before it `cli_verified:false` until re-stamped;
- a sweep of the row count's literals missed five readers that pin the row set by shape (a verdict vector, a stamp's
  rows object, an older-stamp oracle, the dialog-less summary, the framing-variant list).

### 1. `claude --resume <id>` reopens the session (CARRY 2, chunk 2026-10-10-viola-revive)
- A second owed row of `v1-34`: `viola revive` relies on `claude --resume <id>` reopening the same session and firing a
  SessionStart of source `resume` with that id (measured at that chunk on 2.1.287 on the Linux dev host,
  `evidence/live-revive.md`, three live starts; under the fake agent on CI). No ledger row, probe or recorded fixture
  covers it.
- Its probe needs a session to resume — a new shape for the verify child set, under a founder ruling.
- The operator's answer at that wrap (`viola-0.1.0/chunks/2026-10-10-viola-revive/inputs/I8-relay-3.md.txt`, answer 3):
  one ruling, one re-stamp and one count move for both of that chunk's rows (§1, §2), **and this phase proposes a cut
  if the entry no longer fits one window**.
- With the row, the recorded SessionStart of source `resume` replaces the fake agent's rewritten payload: the live
  payload holds 10 key names, the rewritten one 5 (`cwd`, `hook_event_name`, `session_id`, `source`,
  `transcript_path`; the five more: `context_tokens`, `estimated_cache_write_usd`, `prompt_cache_likely_expired`,
  `scratchpad_dir`, `seconds_since_last_response`).
- Not measured at that chunk: `--fork` on the real CLI, a resume of a killed session on the real CLI, whether a
  resumed session held its earlier turns, project settings on a resume from another directory.

### 2. `/compact` as a local command (CARRY 3, chunk 2026-10-10-viola-revive)
- `viola send` of `/compact` exits 13 `not-delivered` / `no-prompt-submitted` after 10 s while the command runs, and
  the log shows a `session-start` of cause `compact` with the same session id (measured at that chunk on 2.1.287,
  `evidence/live-revive.md`; the `compact` payload holds 8 key names). `/compact` is not on the local-command list
  (re-read at P1: `LOCAL_COMMANDS` holds `/clear` with `PostCondition::NewSession` and `/remote-control` with none,
  `ledger.rs:157`; `PostCondition` holds `NewSession` alone, `ledger.rs:150`).
- The CARRY's own marker, kept verbatim: "hypothesis: the send waited for a `prompt-submitted` that a local command
  never fires, as `/clear` fires none." Closed at P3 in its code half (research.md §Measured facts 2: `classify`
  returns `Confirms::Prompt` for `/compact`); its CLI half is that chunk's one reading on 2.1.287 (the refused send and
  the `compact` session-start 18.8 s later), not a probed row. Listing it needs a new `PostCondition` member (cause
  `compact`, the SAME id) and a second claim arm in `SendSlot::session_started`, which today claims only a new id
  (research.md §Measured facts 2).
- Listing it with a post-condition is a ledger row and one more Run B paste; "a compaction inside `viola verify` costs
  a live turn". The size of this entry is read at its take-up (that wrap's answer 4).

### 3. The statusline settings override (CARRY 4, chunk 2026-10-10-statusline-pass-through)
- An owed row with its `viola verify` probe: a `statusLine` given through `--settings <file>` takes the status line's
  place for the session, and its unquoted `<pinned path> hook statusline` string runs.
- Read by hand on 2.1.287 on the Linux dev host with a project-scope status line (that chunk's
  `evidence/live-statusline.md`); macOS not read.
- Not measured, owed to the same live rig: a user-scope status line; which of two `--settings` flags wins (Runs C and
  D already pass one since chunk 2026-10-13-interrupted-verify-cleanup); how the CLI cancels a statusline script
  still running and whether the user's command then outlives the hook; the 5 s bound on a real session; the status
  line row's appearance with an empty output.
- The founder's word on that chunk (live, 2026-10-10, relayed verbatim): no row then; the operator's answer at its
  wrap (`viola-0.1.0/chunks/2026-10-10-statusline-pass-through/inputs/I6-relay-1.md.txt`): this entry's phase **sizes the entry and proposes a cut**.

### 4. The Unix shell `/bin/sh` (CARRY 4, the same block)
- An owed row with its `viola verify` probe: the CLI runs a statusline command string through a shell it starts as
  `/bin/sh`. Read by hand on 2.1.287 on the Linux dev host (same evidence). The recorder that reads which shell ran,
  by three quoted words, is that chunk's `evidence/live-recorder.sh`.
- Not measured: the flag the CLI hands its shell. Windows has no `/bin/sh` arm (the override is not written there).

### 5. The statusline payload's `rate_limits` (CARRY 5, chunk 2026-10-10-budget-governor)
- The `rate_limits` ledger row and its `viola verify` probe. The wrapper's budget gate refuses an automated `send` as
  `budget-paused` on this shape, outside the ruled limit "a shape no `send` relies on needs no probe". The founder's
  own answer (Epoch 4 boundary batch, 2026-10-13, relayed verbatim by the Viola overseer, «Строку к Paste newline
  ledger row»): the build stays as it is, and the row and its probe join this entry.
- The shape: `rate_limits` with `five_hour` and `seven_day`, each a whole-number `used_percentage` and a whole-number
  `resets_at` read as epoch seconds (read by hand on 2.1.287, chunk 2026-10-10-statusline-pass-through's
  `evidence/live-statusline.md`).
- The CARRY's own marker, kept verbatim: "hypothesis: `rate_limits` arrives from the second payload of a session on,
  not only after the first API response as the fetched documentation says (one live start: present 0.35 s after the
  first payload; cause not known)." Not re-derivable at HEAD (a CLI behaviour; no live start at research): it stays the
  hypothesis it is. What a probe may lean on is narrower and was read twice: absent on a session's first payload,
  present after a turn (research.md §Measured facts 7).
- Not measured: an account without `rate_limits`, the `spend_limit` window, the real `claude` against the gate. A
  drift in `resets_at` opens the gate for that window (fail-open). The masters name this entry (architecture
  §Cross-cutting Patterns, Capability ledger).

### 6. The `claude agents --json` join field (CARRY 6, re-pinned from "The board: viola list")
- From chunk 2026-09-28-capability-ledger-and-viola-verify: the `claude agents --json` join-field ledger row (child
  pid or session id) and its `viola verify` probe land here.
- No shipped code reads `claude agents --json` today; its consumer is the next markerless entry, "Unwrapped sessions
  on the board" (verified: research.md §Measured facts 11). The shape is relied on by no shipped surface yet, and the
  ruled limit "a shape no `send` relies on needs no probe" bears on whether its row belongs before its consumer lands;
  the test plan's consumer-first rule keeps a fake-agent `agents --json` mode out until then.

## Gates (not a `BLOCKED-ON`; the founder's own word, asked at P4)
- **The verify child set is founder-ratified** (`.claude/rules/security.md`, the bullet on `viola verify`'s four
  interactive PTY children: Run B's five pastes are compiled literals — the probe prompt, a long text, a tag-like
  text, a newline text, `/clear`; Runs C and D answer dialogs only through the hidden `--capture --answers` arm).
  Every shape above that needs a new child shape, a new paste, a resume start, a compaction turn, a statusline child
  or a `claude agents` invocation is a boundary widening needing his own word, asked at P4 with its count. Without it
  no row of that shape lands. Verified, and widened by one finding: `hook statusline --capture` files nothing today,
  so a statusline capture is also a new capture arm (research.md §Measured facts 5).
- **The cap of live `claude` starts** for this chunk is his word too (CARRY 1: "a live-start cap of its own at
  take-up"). No live start beyond the cap he gives is planned.
- A planned step that departs from a founder ruling is never a decisive lean or a card-confirm item: the plan goes up
  to it and holds it behind a stop rule naming what counts as his word (CLAUDE.md Session Learnings).

## Sizing and the cut
- Six shapes, each a ledger row with a probe — a resume child, a compaction turn, statusline children, a
  `claude agents --json` start — plus fixtures re-recorded and every stamped home re-stamped, do not fit one window as
  one chunk (verified: research.md §Measured facts 2, 4, 5, 8 — `/compact` adds a product change to `send`'s claim,
  the resume row a sixth child and a new id source, the statusline rows a new capture arm and, for `rate_limits`, a
  new fixture class). Two CARRYs (2, 4) direct this phase to propose a cut. The cut is P4's; moving a row off this
  entry that the founder placed here (CARRY 5, by his own answer; the entry itself, by his fork-3 answer) is his word,
  asked at P4.
- The live-start count is read per part the plan keeps: one `viola verify` round is five starts (print plus Runs A–D),
  six with a resume child; a record round and a re-stamp of the tmpfs `--local-live` home are two rounds when both are
  wanted (verified: research.md §Measured facts 9).
- Every reader that pins the row set by shape moves with each row added, not only the count literals (verified:
  research.md §Sweeps — six files of count literals, the dialog-less summary in `tests/support/home.rs`, the
  older-stamp oracle in `src/run/version_gate.rs`).

## The cut, as answered at P4 (validation-1: intent-incomplete, amended at P5)
- The founder's own answers (`inputs#I1`, relayed by the operator from the overseer's dialog, 2026-10-13T22:52:37Z):
  - fork 1: this chunk builds the statusline trio (§3, §4, §5); `--resume` (§1) and `/compact` (§2) move to a new
    entry placed directly after this one, their CARRYs verbatim;
  - fork 2: the `claude agents --json` row (§6) moves onto its consumer, "Unwrapped sessions on the board";
  - fork 3: Run B gains a probe `--settings`, its `statusLine` command the pinned `viola` with a new hidden `hook
    statusline --capture --shell "$0"` arm (a ratified widening of the verify child set; no added start);
  - fork 4: a cap of ten live starts (one record round and one `run --local-live` round).
- Found at planning, decided by the artifacts (plan.md §Provenance, leaned): the three rows are relied on only where
  the status line is wrapped (Unix, `statusline::WRAPPED_HERE`). A Unix host checks and stamps twenty-one rows, a
  Windows host eighteen, and the step counter's MM is the relied count.

## Out of this entry
- `v1-34`'s harness-prefix row and its R8 identity-floor row — the founder, 2026-10-07T09:43Z: no entry minted, each
  needs its own ruling. So `v1-34` is not completed by this chunk whatever it lands.
- The `paste-newline` row's Windows reading — owned by a CARRY on "Windows-only live measurements" (`BLOCKED-ON` an
  interactive Windows host).

## Boundaries
- No live `claude` start beyond the founder's cap; no change to the verify child set without his word.
- Paste text, `send`'s typed text and its validation are unchanged; this chunk measures and stamps what viola relies
  on, it does not change what it types.
- Behaviour on an unverified CLI build stays transport-only; a row added to `ALL` gates `cli_verified` like the
  eighteen.
- Never a key typed into a dialog of a verify child; viola writes nothing under `~/.claude`.
