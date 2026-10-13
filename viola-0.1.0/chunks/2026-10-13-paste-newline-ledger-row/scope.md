# Scope — 2026-10-13-paste-newline-ledger-row · Paste newline ledger row

**Working entry** (`viola-0.1.0/working-route.md`, the first markerless entry, fifth under
`### Epoch 5 — Driver surface`, directly after "Interrupted verify cleanup"): Paste newline ledger row — an LF typed
inside a paste, which every multi-line send relies on, is a probed and stamped capability-ledger row (v1-34) — plus
eight CARRY blocks (folded below; completeness check `route.py pins --run-dir` → this run dir's trail: eight rows on
the entry, 1471 · 1176 · 1023 · 1538 · 861 · 1475 · 1648 · 403 chars, no abstention).

**The operator's direction at take-up** (`inputs#I1`, the invocation, verbatim in the snapshot):
- two things are **the founder's own live word, asked as P4 forks with their counts**: (a) a **fifth Run B paste** — a
  boundary widening, labelled by the operator "his R-L29" (no record of that label is on disk: the tree reads
  `R-L1`..`R-L4` only, so it is the label of the ruling to be asked, not a ruling made); (b) **the cap of live
  `claude` starts for this chunk** — "a re-stamp on the tmpfs home costs five; count every start the plan needs";
- **no live start beyond the cap he gives** is planned;
- **no mutation run, witness run or workflow dispatch**;
- every time written into a record comes from `date -u`.

**CI read at Setup 5a** (the last wrap's flip `1948311` = HEAD, `ci.py conclusion`, this run dir's trail):
- `1948311`: **verdict not yet available** — `ci#38463266088` (push) in progress at the read, checks 15/15 registered,
  6 running (oldest `lint (ubuntu-latest)` 127 s), no wall-clock yet. Not a green; nothing folded. P3 re-read it once:
  **green**, checks 15/15, wall 576 s (research.md §Measured facts 1); no red to disposition.

## What this chunk builds

The entry is the owner of every owed capability-ledger row the route has parked on it: seven shapes, each a row with
its `viola verify` probe, under one founder ruling on the verify child set. Today `LedgerRow::ALL` holds seventeen
rows (`crates/viola-agent-claude/src/ledger.rs:41`, re-read at P1), all gating `cli_verified`.

### 1. An LF typed inside a paste (the entry; CARRY 1, the founder's word 2026-10-09T20:58:38Z)
- The owed row of `v1-34`: an LF typed inside a bracketed paste reaches the CLI as one prompt holding that LF
  (submitted unchanged, not an early submit). Measured once on live `claude` 2.1.287 on the Linux dev host (chunk
  2026-10-09-inner-cr-and-crlf-in-a-sent-text, `evidence/live-run.md`, `evidence/live-readings.ndjson`,
  `rule-inner-lf`): the prompt equal to the 69-byte sent text, filed `driver`, no `wheel` record; two LFs in a row came
  back as two (`rule-inner-cr-cr`, `rule-inner-lf-cr`). Not measured: another CLI version, a long or wrapped
  multi-line text, Windows.
- What the row costs, as shown to the founder (CARRY 1, verbatim): "a fifth Run B paste, an eighteenth row, fixtures
  re-recorded, every stamped home re-stamped, live starts under a cap of their own."
- **gate (not a `BLOCKED-ON`; the operator's fork, `inputs#I1`):** the verify child set is founder-ratified
  (`.claude/rules/security.md`, the bullet on `viola verify`'s four interactive PTY children: Run B's four pastes are
  compiled literals — the probe prompt, a long text, a tag-like text, `/clear`); a fifth paste is a boundary widening
  that needs his own word, asked at P4 with its count. Without it no row of this section lands.
- Out of this entry (CARRY 1): `v1-34`'s harness-prefix row and its R8 identity-floor row — the founder,
  2026-10-07T09:43Z: no entry minted, each needs its own ruling.

### 2. `claude --resume <id>` reopens the session (CARRY 4, chunk 2026-10-10-viola-revive)
- A second owed row of `v1-34`: `viola revive` relies on `claude --resume <id>` reopening the same session and firing
  a SessionStart of source `resume` with that id (measured at that chunk on 2.1.287, Linux dev host,
  `evidence/live-revive.md`, three live starts; under the fake agent on CI). No ledger row, probe or recorded fixture
  covers it.
- Its probe needs a session to resume — "a new shape for the verify child set, under the founder ruling this entry
  already waits on". The operator's answer at that wrap (`viola-0.1.0/chunks/2026-10-10-viola-revive/inputs/I8-relay-3.md.txt`, answer 3): one ruling, one re-stamp and
  one count move for both rows, **and this phase proposes a cut if the entry no longer fits one window**.
- With the row, the recorded SessionStart of source `resume` replaces the fake agent's rewritten payload: the live
  payload holds 10 key names, the rewritten one 5 (`cwd`, `hook_event_name`, `session_id`, `source`,
  `transcript_path`; the five more: `context_tokens`, `estimated_cache_write_usd`, `prompt_cache_likely_expired`,
  `scratchpad_dir`, `seconds_since_last_response`).
- Not measured at that chunk: `--fork` on the real CLI, a resume of a killed session on the real CLI, whether a
  resumed session held its earlier turns, project settings on a resume from another directory.

### 3. `/compact` as a local command (CARRY 5, chunk 2026-10-10-viola-revive)
- `viola send` of `/compact` exits 13 `not-delivered` / `no-prompt-submitted` after 10 s while the command runs, and
  the log shows a `session-start` of cause `compact` with the same session id (measured at that chunk on 2.1.287,
  `evidence/live-revive.md`; the `compact` payload holds 8 key names). `/compact` is not on the local-command list.
- The CARRY's own marker, kept verbatim: "hypothesis: the send waited for a `prompt-submitted` that a local command
  never fires, as `/clear` fires none." Closed at P3 in its code half only: `/compact` is not on `LOCAL_COMMANDS`
  and `PostCondition` holds `NewSession` alone (research.md §Measured facts 2, 9), so `send` classifies it as a
  prompt and waits for `prompt-submitted`. The CLI half stays a hypothesis: no live start at research.
- Listing it with that post-condition is a ledger row and one more Run B paste; "a compaction inside `viola verify`
  costs a live turn". The size of this entry is read at its take-up (that wrap's answer 4).

### 4. The statusline settings override and the Unix shell (CARRY 6, chunk 2026-10-10-statusline-pass-through)
- Two owed rows, each with its `viola verify` probe:
  - the settings override: a `statusLine` given through `--settings <file>` takes the status line's place for the
    session, and its unquoted `<pinned path> hook statusline` string runs;
  - the Unix shell: the CLI runs a statusline command string through a shell it starts as `/bin/sh`.
- Both read by hand on 2.1.287 on the Linux dev host with a project-scope status line (that chunk's
  `evidence/live-statusline.md`); macOS not read. The recorder that reads which shell ran, by three quoted words, is
  that chunk's `evidence/live-recorder.sh`.
- Not measured, owed to the same live rig: a user-scope status line; which of two `--settings` flags wins (Runs C and
  D already pass one — `src/cmd/verify/typed.rs`, `SETTINGS_FLAG` since chunk 2026-10-13-interrupted-verify-cleanup);
  the flag the CLI hands its shell; how the CLI cancels a statusline script still running and whether the user's
  command outlives the hook; the 5 s bound on a real session; the status line row's appearance with an empty output.
- This phase "sizes the entry and proposes a cut" (that wrap's
  `viola-0.1.0/chunks/2026-10-10-statusline-pass-through/inputs/I6-relay-1.md.txt`).

### 5. The statusline payload's `rate_limits` (CARRY 7, chunk 2026-10-10-budget-governor)
- The `rate_limits` ledger row and its `viola verify` probe; the wrapper's budget gate refuses an automated `send` as
  `budget-paused` on this shape, outside the ruled limit "a shape no `send` relies on needs no probe". The founder's
  own answer (2026-10-13, relayed verbatim by the Viola overseer, «Строку к Paste newline ledger row»): the build
  stays as it is, and the row and its probe join this entry.
- The shape: `rate_limits` with `five_hour` and `seven_day`, each a whole-number `used_percentage` and a whole-number
  `resets_at` read as epoch seconds (read by hand on 2.1.287, chunk 2026-10-10-statusline-pass-through's
  `evidence/live-statusline.md`).
- The CARRY's own marker, kept verbatim: "hypothesis: `rate_limits` arrives from the second payload of a session on,
  not only after the first API response as the fetched documentation says (one live start: present 0.35 s after the
  first payload; cause not known)." Not re-derivable at HEAD (a CLI behaviour; no live start at research); carried as
  the hypothesis it is, and leaned on by no step of this chunk unless the row stays in it (research.md §Open
  questions, the cut).
- Not measured: an account without `rate_limits`, the `spend_limit` window, the real `claude` against the gate. A
  drift in `resets_at` opens the gate for that window (fail-open). The masters name this entry (architecture
  §Cross-cutting Patterns, Capability ledger).

### 6. The `claude agents --json` join field (CARRY 8, re-pinned from "The board: viola list")
- From chunk 2026-09-28-capability-ledger-and-viola-verify: the `claude agents --json` join-field ledger row (child
  pid or session id) and its `viola verify` probe land here.

### 7. Mutants the 10 s kill grades `caught` (CARRY 2, CARRY 3)
- CARRY 2 (chunk 2026-10-09-epoch-3-cleanup-ii): four mutants on lines that chunk did not edit read `caught` by the
  `mutants` profile's 10 s kill alone, with no failing assertion — `delete !` twice in `Agent::replay_dialogs`
  (`src/bin/viola-fake-agent.rs:450:16`, `:450:38` as of `632f6a7`), `delete match arm "--screens"` in `Opts::parse`
  (`:107:17`), and `ProbeDir::typed` replaced by `Default::default()` (`src/cmd/verify.rs:284:9`). Re-read at P1 by
  function name: `replay_dialogs` at `src/bin/viola-fake-agent.rs:446`, the `"--screens"` arm at `:107`,
  `ProbeDir::typed` at `src/cmd/verify.rs:318`. Under each the stamped-home fixture's `viola verify` does not finish.
  "Each takes a test that fails on an assertion, shown by this entry's own witness run over the two files."
- CARRY 3 (chunk 2026-10-10-windows-mutation-grade; "the take-up decides whether it is this entry's work"): three
  `viola-state` mutants in the retry loop, now `replace_private_retrying` (`crates/viola-state/src/fs.rs:281`,
  `attempts += 1` at `:295`, re-read at P1), read caught on `windows-2025` after 10 s (run 38036448183, job
  `mutants (viola-state)`). The CARRY's own marker, kept verbatim: "hypothesis: they are ended by the 10 s kill line
  and fail no assertion, the shape of the four above." A runner-only reading, closed against its run, not HEAD: the
  job log holds grade and time only and the per-mutant log was not kept, so it stays a hypothesis; the site is
  verified at HEAD (research.md §Measured facts 12) and two Linux cases already kill all six of its mutants by
  hand-applied reading (chunk 2026-10-13-epoch-4-survivors).
- The operator's direction excludes every witness and mutation run and every workflow dispatch (`inputs#I1`), so
  neither CARRY's "shown by a witness run" can run inside this chunk (verified: research.md §Measured facts 11, 12 —
  CARRY 2's three sites stand at HEAD, by name; CARRY 3's re-reading needs a `windows-mutants.yml` dispatch).
  Whether the assertion tests are written here in the project's hand-applied form (their grading owed to the Epoch 5
  boundary audit) or the CARRYs pass to an owner is P4's, raised on the card.

## Sizing and the cut
- Seven shapes, each a ledger row with a probe, a possible fifth Run B paste, a new verify child for `--resume`, a
  live turn for `/compact`, statusline children, a `claude agents --json` start, fixtures re-recorded and every
  stamped home re-stamped do not fit one window as one chunk (verified: research.md §Measured facts 9 — five of the
  six other shapes each need a new child shape under the founder-ratified set); two CARRYs (4, 6) direct this phase
  to propose a cut. The cut is P4's; moving a row the founder placed here (CARRY 7, by his own answer; CARRY 8, by a
  card he approved) is his word, asked at P4.
- The live-start count is read per part the plan keeps, and the founder's cap bounds the plan (verified:
  research.md §Measured facts 7, 8 — one `viola verify` round is five starts, print plus Runs A–D, and a fifth Run B
  paste adds none; 2.1.287 is installed and named by path; a record round cannot use a `--local-live` home, so
  recording and re-stamping that tmpfs home are two rounds of five when both are wanted). With a cap below five no row
  can be stamped in this chunk: the fake agent replays a compiled paste only from a recorded variant (research.md
  §Measured facts 5).

## The cut, as answered at P4 (validation-1: intent-incomplete, amended at P5)
- The founder's own answers (`inputs#I2`): fork 1, a fifth Run B paste (R-L29); fork 2, a cap of five live `claude`
  starts; fork 3, the six shapes of §2–§6 (`--resume`, `/compact`, the statusline override, `/bin/sh`, `rate_limits`,
  `claude agents --json`) move to a new entry placed directly after this one. So this chunk builds §1 (the LF row) and
  §7's CARRY 2 in the hand-applied form; §2–§6 are carried, not built, and their CARRYs ride the new entry verbatim.
- §7's CARRY 3 is not this entry's work (its re-reading needs a dispatch `inputs#I1` excludes, and its site's Linux
  kills stand); its owner is proposed on the P5 card.

## Boundaries
- No mutation run, witness run or workflow dispatch (`inputs#I1`).
- No live `claude` start beyond the founder's cap (`inputs#I1`); no change to the verify child set without his word.
- Paste text, `send`'s typed text and its validation are unchanged; this chunk measures and stamps what `send` relies
  on, it does not change what it types.
- Behaviour on an unverified CLI build stays transport-only; a row added to `ALL` gates `cli_verified` like the
  seventeen.
- Never a key typed into a dialog of a verify child; viola writes nothing under `~/.claude`.
