# Scope — 2026-10-18-compact-post-condition-ledger-row

**Working entry (verbatim title + hint):** Compact post-condition ledger row — `/compact` confirmed by its SessionStart of
cause `compact`, a probed, stamped ledger row with its own confirm window (v1-34)

**Capability named by the entry:** `verification-matrix.json#v1-34` (Capability ledger, `viola verify`, and
transport-only degrade; `planned`, unclaimed at take-up). This chunk advances it and cannot complete it: its acceptance
also needs the harness-prefix row and the R8 identity-floor row, owed with no entry minted (the founder,
2026-10-07T09:43Z; verified at P3 against the cap's own `notes`, `matrix.py show --id v1-34`). P4 reads the claim lean.

**Take-up direction:** the founder is asleep; this run plans through P4 and raises the forks that are his (a boundary
widening, a live `claude` start cap, a new keystroke or a write to `~/.claude.json` for the trust dialog) as one
dialog, which the Viola overseer answers «hold for the founder» (`inputs#I1`, the operator's direction). A fork so
answered is held, never ratified, and the plan stops at it.

## What it builds
- **A `/compact` post-condition** — a `send` of `/compact` on a verified CLI is confirmed by the session's next
  SessionStart of cause `compact` carrying the SAME session id, instead of reading `unconfirmable` at once as it does
  since chunk 2026-10-14-resume-and-compact-ledger-rows (`/compact` is on `LOCAL_COMMANDS` with `None`,
  `crates/viola-agent-claude/src/ledger.rs:201`–`:205`). Verified at P3: `PostCondition` holds `NewSession` alone
  (`ledger.rs:194`–`:198`) and `session_started` claims only cause `clear` with a new id (`src/run/send/slot.rs:103`–`:119`),
  so it needs a second `PostCondition` member and a second claim arm; the arm compares with the id remembered BEFORE
  the line, since `session_started` overwrites it first (research §Measured facts 1).
- **A confirm window of its own for `/compact`** — the one shared deadline `opened + CONFIRM_WINDOW_FALLBACK` (10 s,
  `crates/viola-agent-claude/src/screen.rs:20`, used at `src/run/send/slot.rs:144`) is shorter than the one measured
  compaction. [premise-corrected: the bound is not free — `GATE_MAX_WAIT` 8.5 s + paste + window must stay under the MCP
  client's tool-call timeout, known only to exceed 50 s on 2.1.287, `architecture.md:46`, `:58`; research §Measured
  facts 2] A new compiled bound in `viola_agent_claude::screen` for the compact post-condition alone, at most about
  41.5 s, its value and PROVISIONAL status set at P4.
- **A probed, stamped ledger row** — a 24th `LedgerRow` (`LedgerRow::ALL: [Self; 23]` at `ledger.rs:48`) with a `viola
  verify` probe and a recorded 2.1.287 row, gating the `/compact` post-condition on the stamp like the `/clear` row
  gates `NewSession`. Relied on every OS (only the three statusline rows are OS-conditional, `ledger.rs:137`), which
  moves the relied set (Unix 23 → 24, Windows 20 → 21): every home stamped before this chunk reads `cli_verified:false`
  until its next `viola verify`; its Windows reading owed on "Windows-only live measurements". Verified at P3.
- **The fake agent's compact replay** (found at P3, research §Measured facts 4): under `--framing` the recorded
  compact SessionStart is fired with `session_id` set to the fake agent's current session (the `--resume` precedent,
  `src/bin/viola-fake-agent.rs:175`–`:184`), since a byte-unchanged recorded id can never equal the fake session's.

## Folded freight and premises
- CARRY 1 (minted at the 2026-10-14-resume-and-compact-ledger-rows wrap on the founder's own word «В конец Эпохи 5»,
  `chunks/2026-10-14-resume-and-compact-ledger-rows/inputs/I6-relay-2.md.txt` Q3, from its P5 card's row 1, approved
  by the operator, that chunk's `inputs/I2-relay-2.md.txt`): `/compact` is on the local-command list with no
  post-condition, so a `send` of it reads `unconfirmable` at once on a verified and an unverified CLI. Verified at P3:
  `classify` (`src/run/send/classify.rs:26`–`:32`) and the cases `compact_on_a_verified_version` /
  `compact_on_an_unverified_version`.
- Measured-marked, kept verbatim from CARRY 1: "one compaction was measured at 18.8 s, past the 10 s
  `CONFIRM_WINDOW_FALLBACK` (a 2-turn session, that chunk's `research.md` §Measured facts 3)". Verified at P3 against
  its evidence: `chunks/2026-10-10-viola-revive/evidence/live-revive.md:99`–`:101` (issued 14:49:11.194, SessionStart
  14:49:29.973).
- Measured-marked, kept verbatim from CARRY 1: "its SessionStart of cause `compact` kept the session id (measured on
  2.1.287 at chunk 2026-10-10-viola-revive, `evidence/live-revive.md`)". Verified at P3: `live-revive.md:101`, `:103`.
- Hypothesis-marked, kept verbatim from CARRY 1: "hypothesis: the compaction time grows with the session
  (unmeasured)". Verified unmeasured at P3: `live-revive.md:147` lists "`/compact` on a long session" as not measured,
  and no other compaction reading stands under `viola-0.1.0/chunks/*/evidence/`. It bears on the window: a compaction
  past the bound reads `not-delivered` / `no-prompt-submitted` though it ran.
- From CARRY 1, verbatim: "A probe that compacts inside `viola verify` costs a live turn, so it needs a widening ruling
  on the verify child set and a live-start cap of its own at take-up". Verified at P3 against architecture §Established
  Decisions [CLI Version Compatibility] and security-plan §Threat Model Summary (the enumerated child set, Run B's five
  pastes). Both are the founder's word (P4 forks), held under `inputs#I1`.
- CARRY 2 (chunk 2026-10-17-optional-plugin-install, the Viola overseer's answer at its wrap,
  `chunks/2026-10-17-optional-plugin-install/inputs/I8-relay-1.md.txt` — a delegate's, PROVISIONAL, the founder's own
  word owed at the Epoch 5 boundary batch): two readings that chunk's live round on claude 2.1.287 did not take ride
  here —
  - which `viola` MCP server or servers a wrapped session holding both the `--plugin-dir` plugin and the installed
    `viola@viola` starts, and which answers its `list` call;
  - whether a registered `viola mcp` that exits 1 (its home at 0770) blocks an unwrapped start.
  Verified at P3: these are `verification-matrix.json#v1-42`'s owed readings (`matrix.py show --id v1-42`, its
  2026-10-18 note); whether this chunk takes them and claims v1-42 is a P4 lean behind the founder's forks.
  [intent-amended at P4: the founder's own answers to forks 3 and 4 (`inputs#I3`, «Отдельный новый вход» · «Здесь
  ноль») — CARRY 2 leaves this chunk whole: the two readings, the trust-dialog method and v1-42 move to a NEW entry
  "Plugin precedence live readings" placed directly after this one in Epoch 5, and this chunk makes no plugin start.
  The CARRY 2 bullets below stand as the record of what moves, not as this chunk's work.]
- Measured-marked, kept verbatim from CARRY 2 ("measured at that chunk, `evidence/live-round.md`"): "`--scope local`
  run from a rig under `target/` writes the repository's own `.claude/settings.local.json` (the CLI takes the git work
  tree root), a rig made its own git repo is an untrusted project root whose start the trust dialog holds before
  SessionStart, and that dialog's confirm row is not its selected row, so one Enter cannot accept it (2 starts of a
  3-start cap red, start 3 unspent)". Verified at P3 against `chunks/2026-10-17-optional-plugin-install/evidence/live-round.md`
  (rig 1's guard table; rig 2 starts 1 and 2; start 2's rig read `unselected` at every quiet read). The record itself
  marks the dialog's identity at start 1 as an inference, and the selected row's text was never kept.
- From CARRY 2: the method past the folder-trust dialog was not found. Each candidate — a new keystroke into the rig's
  own pty (beyond the founder's one Enter, `chunks/2026-10-17-optional-plugin-install/inputs/I6-relay-3.md.txt`), a
  write to `~/.claude.json` (a trust record for the rig's root), or a rig placed under an already-trusted root — is the
  founder's word (P4 fork), held under `inputs#I1`. The open question carried in the handoff: which row 2.1.287's
  trust dialog selects by default.
- From CARRY 2, verbatim: "a round rebuilds `target/debug/viola` first, since no gate builds it". Verified at P3:
  `live-round.md` Setup (rig 1 found the build stale and rebuilt it).
- Every live `claude` start this chunk makes (a verify record round for the new row; a plugin round for CARRY 2) counts
  against a cap that is the founder's own word (as at every chunk since 2026-10-13), held under `inputs#I1`.
  [intent-amended at P4: his own answers (`inputs#I3`) — fork 1 «Шестая вставка в Run B»: the probe is a sixth
  compiled Run B paste (the widening his); fork 2 «12 стартов — два verify»: at most 12 live starts, one record round
  of `viola verify --record` (six starts) plus one re-run after a red, never beyond; fork 4: no plugin start here.]

## CI read at take-up (Setup 5a)
- `7f35c8e` (the last wrap's flip, HEAD) — `ci#39092978434`, green, checks 15/15, wall 615 s. Nothing to disposition.

## Boundaries
- `/clear`'s row, its probe and its `NewSession` post-condition are unchanged except where a `/compact` paste placed
  before `/clear` touches Run B's paste order. Verified at P3: `clear_start`, `local_command_cleared` and
  `framing_variants` stay true with a compact SessionStart (source `compact`, the same id) between the newline paste and
  `/clear` (`ledger.rs:807`–`:848`), unless the CLI fires a SessionEnd for `/compact` (unmeasured; the record round reads
  it).
- No keystroke into any CLI dialog by `viola verify` (security-plan: never a byte typed into any dialog); no write
  under `~/.claude` by viola; no live start and no new verify child beyond the founder's word.
- The harness-prefix row and the R8 identity-floor row stay owed with no entry minted (the founder, 2026-10-07T09:43Z);
  not this chunk's.
- Windows legs of every live reading are owed on "Windows-only live measurements" (no interactive Windows host).
