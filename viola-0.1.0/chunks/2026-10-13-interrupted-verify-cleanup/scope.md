# Scope — 2026-10-13-interrupted-verify-cleanup · Interrupted verify cleanup

**Working entry** (`viola-0.1.0/working-route.md`, the first markerless entry, fourth under
`### Epoch 5 — Driver surface`, directly after "Epoch 4 survivors II"): Interrupted verify cleanup — `viola verify`
stopped by Ctrl-C or TERM removes only what that run created; older, foreign and kill-or-crash leftovers stay by hand
— plus four CARRY blocks (folded below; completeness check `route.py pins --run-dir` → this run dir's trail: four rows
on the entry, 637 · 1038 · 430 · 528 chars, no abstention). The entry's closing sentence: architecture §Occupied
Resources (flags passed to the child) and security-plan §Data Protection name this entry.

**The operator's direction at take-up** (`inputs#I1`, the invocation, verbatim in the snapshot):
- **no mutation run, witness run or workflow dispatch inside the chunk**;
- **no live `claude` start** without the founder's cap: if one seems needed, it is raised as a P4 fork with its count;
- the chunk is **sized against one window at P5**;
- every time written into a record comes from `date -u` in the same call.

**CI read at Setup 5a** (the last wrap's flip `d9f01b1` = HEAD, `ci.py conclusion`, this run dir's trail):
- `d9f01b1`: **verdict not yet available** — `ci#38446140379` (push) in progress at the read, checks 15/15 registered,
  6 running, no wall-clock yet. Not a green; nothing folded. P3 re-read it once: still `in progress` (wall 706 s, 15/15
  checks), no red to disposition.

## What this chunk builds

### 1. Ctrl-C and TERM end a verify run cleanly (the entry; CARRY, the founder's word)
- The founder's narrow entry (CARRY, live in the overseer's dialog 2026-10-10T10:26:30Z, «Узкая запись», relayed as
  `viola-0.1.0/chunks/2026-10-10-self-healing-state/inputs/I4-relay-1.md.txt`): `viola verify` handles Ctrl-C and TERM and removes only what it
  created in that run, nothing older and nothing foreign; what a SIGKILL or a crash leaves stays by hand. **No owner
  record and no remover of an earlier run's dirs is built here.**
- On either stop the run ends its PTY child (no key typed into any dialog — security-plan's standing rule for verify's
  children), lets its directory guards remove what this run made, and exits. The order is child ended and reaped
  first, then the removal: a dir is not removed under a live child writing into it (verified, research.md §Measured
  facts 4: every wait is a `POLL` loop a stop flag ends within one `POLL`; the run's `kill` + `finish` precede the
  guards' drop on the return).
- [premise-corrected: research.md §Measured facts 3 — every OS-level reach was read; the Windows event set is not
  settled by any artifact] "Ctrl-C and TERM" per OS: Unix `SIGINT` and `SIGTERM` (a terminal Ctrl-C reaches verify and
  its `run_bounded` child, never a PTY child; a `kill <pid>` reaches verify alone). Windows: `CTRL_C_EVENT` and
  `CTRL_BREAK_EVENT` reach verify and its `run_bounded` child, never a ConPTY child; whether `CTRL_CLOSE_EVENT` also
  counts as TERM is a P4 fork. In every case verify itself ends the child it is running.
- [premise-corrected: research.md §Measured facts 4, 9 — architecture §Conventions, design-system and layout-templates
  hold no form for an interrupted run] An interrupted run writes no stamp and prints no step line and no `stamped`
  line: the stop returns from `measure` before `check_rows` and `update_stamps` (verified, by construction at
  `verify.rs:209`–`:214`). Its exit status (inside the defined set or a new code), its stderr lines and its
  `process-exit` `detail` (none, or a twelfth closed member) are a P4 fork; none is invented here.
- [premise-corrected: research.md §Measured facts 6 — the root's `windows-sys` inherits `Win32_System_Console` from the
  workspace feature list] No new dependency, no manifest line and no `Cargo.lock` package: `libc` (root, Unix) and
  `windows-sys` (root, Windows, the workspace features) carry the handler. Architecture's crate-direction text, which
  names the root's `windows-sys` roles without the console, is a wrap amendment.

### 2. What one run creates — three classes (CARRY, chunk 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host)
- The CARRY's measured claim (verified at HEAD `d9f01b1`, research.md §Measured facts 1, 2), kept verbatim: "Its dirs are held by a drop guard and verify handles no
  signal (read at that chunk's research M8 and again at chunk 2026-10-10-self-healing-state's research at HEAD
  `2f1efe3`: `ProbeDir` and `TrustedDir` remove their dir in `Drop`, Run A's dir is a `tempfile` guard, and no
  `signal`, `ctrlc`, `SIGTERM` or `SIGINT` stands in `src/cmd/verify.rs`, `src/cmd/verify/typed.rs` or
  `src/main.rs`)." Re-read at P1 by a grep over `src/` and `crates/` (no `sigterm|sigint|ctrlc|signal_hook|
  SetConsoleCtrlHandler|sigaction` anywhere; `ProbeDir` `src/cmd/verify.rs:244` with `Drop` at `:304`; `TrustedDir`
  `src/cmd/verify/typed.rs:351` with `Drop` at `:363`; Run A `tempfile::Builder::new().prefix("viola-verify-")` at
  `typed.rs:68`) — coordinates only; P3 closes the mechanism.
- "A kill leaves three classes of dir, not one": `<cwd>/.viola-verify-<pid>/` with its `-dialogs` and `-plan`
  siblings; `<home>/ledger/probes/<pid>/`, which holds raw hook captures; and Run A's `viola-verify-*` dir in the OS
  temp root. Verified, research.md §Measured facts 2: each class has one guard today (`TrustedDir`, `ProbeDir`, a
  `tempfile::TempDir`), so no guard is built; the stop makes them drop.
- The same-pid pre-clean in `ProbeDir::create` and `TrustedDir::create` (research.md §Measured facts 8) predates the
  founder's ruling and stands unchanged; this chunk adds no removal.
- "Only what that run created" is decided by the run's own guards (the paths this process made), never by a pattern
  over a directory listing: no glob of `viola-verify-*`, no `<pid>` scan, nothing removed that this run did not
  create.
- Standing leftovers are the operator's (CARRY): at that research the repository root held none and
  `crates/viola-e2e/` held `.viola-verify-227786-plan` — the handoff's operator desk lists it; this chunk leaves it.

### 3. The child-program name and the `--settings` flag through their one homes (CARRY ×2)
- CARRY (chunk 2026-10-10-viola-revive): the literal `claude` stands in `src/cmd/verify.rs` as verify's default
  program (re-read at P1: `:105`, `None => (OsString::from("claude"), Vec::new())`), beside
  `viola_agent_claude::PROGRAM`, the name's one home. "The take-up decides whether verify reads the constant." Lean
  (decided at P4, stated here as the hypothesis): it does — Claude-specific shapes live only in `viola-agent-claude`.
- CARRY (chunk 2026-10-10-statusline-pass-through, that wrap's
  `viola-0.1.0/chunks/2026-10-10-statusline-pass-through/inputs/I5-wrap-statusline-directions.md.txt` item 4): `src/cmd/verify/typed.rs` holds
  the literal `--settings` at two sites (Runs C and D; re-read at P1: `:132` and `:173`); both move onto
  `viola_agent_claude::statusline::SETTINGS_FLAG`. Owed here.
- Both are behaviour-preserving: the argv each run builds is byte-identical before and after (verified, research.md
  §Measured facts 7: both constants are `pub`, hold exactly `claude` and `--settings`, and are pinned by a literal case
  in their crate).

### 4. The masters that name this entry
- architecture §Occupied Resources (flags passed to the child) and security-plan §Data Protection name this entry
  (the entry's own sentence); security.md already states that Run A's OS-temp dir and `<cwd>/.viola-verify-<pid>/`,
  `-dialogs/`, `-plan/` are "removed on every exit path" — this chunk makes Ctrl-C and TERM two such exit paths, and
  SIGKILL / a crash the stated exception (research.md §Measured facts 1). Any master wording that changes is a wrap
  amendment, never a phase edit.
- The handoff carries, for the operator's word: "Interrupted verify cleanup" has no `requirements.md` line and no
  ledger entry. Verified at P3: no unclaimed matrix cap names an interrupted verify (`matrix.py show --unclaimed`, 37
  rows), so the chunk is expected to claim none; P4 reads each candidate's requirement before deciding.

## Boundaries
- No mutation run, witness run or workflow dispatch inside the chunk (`inputs#I1`).
- No live `claude` start (`inputs#I1`): the stop is proven against the fake agent or a child the test owns; a live
  start that seems needed is a P4 fork with its count, never planned silently.
- No owner record, no cleanup of an earlier run's or another process's dirs, no SIGKILL or crash recovery (CARRY).
- `viola verify`'s probes, texts, deadlines, order, stamps and ledger rows are unchanged on a run that is not
  interrupted; the human always wins (no key typed into a child's dialog on the stop path).
- `viola run` / `revive` and other verbs' signal behaviour are out of reach.
- Sizing (`inputs#I1`): one verb's stop path plus two literal moves, sized against one window at P5.
