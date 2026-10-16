# Codebase Research — 2026-10-14-resume-and-compact-ledger-rows

## Scope
- **Depth:** deep · **Reads:** 14 · **Globs/Greps:** 12
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 18 Session Additions (2026-10-10:
  a live rig that needs the stamped version puts that version's install dir first on its `PATH` and reads
  `claude --version` by path first; 2026-10-07 extension: the tmpfs backing, a stalled-start red reported and never
  re-run; 2026-10-14: a fake-agent side effect goes after the turn's screen write; `agents --json` lands with its
  consumer).
- **Platform issues consulted:** none — no runner-only bullet: the one CI run Setup read is green on its re-read
  (`880b056377f4`, `ci#38526582400`, 15/15, wall 579 s; this run dir's `ci.py` trail).
- **External inputs:** `inputs#I1` — the founder's answers to the three P4 forks, relayed verbatim by the operator (P4);
  `inputs#I2` — the P5 word on the take-up's card (P5); `inputs#I3` — the overseer's answer at implement's stop on
  entry 15 (implement); `inputs#I4` — the operator's word for this revision: the preservation guard excludes
  `tests/support/home.rs`, implement re-enters at entry 15, no live start remains (the revision's P3). The take-up's P3
  snapped none: the three CARRY blocks cite this repository's earlier chunk folders, re-read at P1.

## Measured facts
1. **The row set** (`crates/viola-agent-claude/src/ledger.rs@880b056`): `LedgerRow::ALL: [Self; 21]` at `:46`, last
   `StatusLineRateLimits`; `relied_on` (`:134`) drops only the three statusline rows where the status line is not
   wrapped; `relied()` `:145`. `check(row, &Probes)` `:850`, one arm per row (`:853`–`:894`). Re-derived:
   `command grep -n` over the file at HEAD.
2. **`/compact` today, code half — re-derived at HEAD.** `classify` (`src/run/send/classify.rs:27`–`:33`) finds no
   `/compact` in `LOCAL_COMMANDS` (`ledger.rs:193`, two entries: `/clear` → `Some(NewSession)`, `/remote-control` →
   `None`) and returns `Confirms::Prompt`, so `send` waits for a matching `prompt-submitted`. `SendSlot::session_started`
   (`src/run/send/slot.rs:98`–`:118`) claims only a `session-start` whose `cause` is `clear` AND whose id differs from
   the one remembered, for `Confirms::Post(PostCondition::NewSession)`. `PostCondition` holds `NewSession` alone
   (`ledger.rs:186`). So a `/compact` post-condition (cause `compact`, the SAME id) needs a second `PostCondition`
   member and a second claim arm. The CARRY's hypothesis ("the send waited for a `prompt-submitted` that a local
   command never fires") is **verified in its code half**: `Confirms::Prompt` is the only path for an unlisted text.
   Its CLI half stays the one live reading of chunk 2026-10-10-viola-revive (`evidence/live-revive.md`, "Start 2,
   continued": the refused send, then the `compact` session-start 18.8 s after issue), never a probed row.
3. **The confirm window is shorter than the measured compaction** (the decisive new fact). `SendSlot::confirm`
   (`slot.rs:136`–`:159`) gives every send the one deadline `opened + CONFIRM_WINDOW_FALLBACK`, and
   `CONFIRM_WINDOW_FALLBACK` is 10 s (`crates/viola-agent-claude/src/screen.rs:20`, pinned by the unit at `:190` and by
   `send_confirm_window_is_the_fallback`, `slot.rs:477`). The one live `/compact` reading put its `session-start`
   18.8 s after the send was issued, on a session of two short turns (`live-revive.md`, "Start 2, continued", and its
   §What was not measured: "`/compact` on a long session"). The equality the design needs — "a listed `/compact`
   on a verified CLI is confirmed by its post-condition" — therefore does NOT hold for that reading under today's
   window: the claim arm alone would still end exit 13 `no-prompt-submitted` at 10 s. A `/compact` post-condition
   needs a window of its own, a new bound, and the compaction's length grows with the session (unmeasured).
4. **`/clear`'s probe and why a `/compact` paste must come before it** (`src/cmd/verify/typed.rs@880b056`).
   `framing_turns` (`:320`–`:354`) pastes the long, tag-like and newline texts, then `PROBE_LOCAL_COMMAND` last, and
   waits on `local_command_answered` (a new SessionStart or a new UserPromptSubmit, `:364`). After `/clear` the session
   is a new empty one; the only measured compaction ran on a session that held turns. So a `/compact` paste placed
   after `/clear` would compact an empty session (unmeasured). `PROBE_LOCAL_COMMAND` is `LOCAL_COMMANDS[0].0` with the
   doc line "the list's entry that has a post-condition" (`ledger.rs:197`–`:198`): a second entry with a post-condition
   makes that doc line false. `clear_start` (`ledger.rs:787`) searches the first SessionStart of source `clear` after
   the tag-like paste, and `local_command_cleared` (`:799`) compares its id with Run B's FIRST SessionStart id — both
   stay true with a `compact` SessionStart (same id) between them.
5. **The verify child set and a resume child** (`src/cmd/verify.rs@880b056`, `typed.rs`). `verify` logs six spawn
   pairs: `version-probe` (`verify.rs:172`), `verify-probe` (`:197`, the print probe, `--no-session-persistence`
   `:196`), then `typed::measure` (`typed.rs:74`) runs Runs A–D in order, each one `verify-pty-probe` pair (`SUBJECT`,
   `typed.rs:41`). Run B (`trusted_run`, `:113`) passes no `--no-session-persistence`, so its session is persisted
   (inferred from the flag's absence, as at chunk 2026-10-13-owed-ledger-rows; the revive reading resumed a session
   stopped by Ctrl-C, `live-revive.md` start 2). Run B's FIRST SessionStart capture carries the session id that held
   the probe turn and the pastes (`local_command_cleared` reads it, `ledger.rs:800`–`:803`); the id after `/clear` is
   a new, empty session. A resume probe that reads the session's turns must resume the FIRST id.
6. **`--resume` homes** (`crates/viola-agent-claude/src/lib.rs@880b056`): `is_session_id` `:127`, `resume_args(id,
   fork)` `:140`; product callers only `src/cmd/revive.rs` (`:10` import, `:165` `preflight`) — graph query 1, this
   run dir's `tree-query-2026-10-14-resume-and-compact-ledger-rows.json`, `resume_args` 4 rows (2 in lib.rs tests).
   A verify resume child takes its id from Run B's own capture: a new id source onto a child's argv (security
   history 2026-10-10-viola-revive / -statusline-pass-through: the ratified crossing covers a logged id or a typed
   `--id` only).
7. **The fake agent's resume** (`src/bin/viola-fake-agent.rs@880b056`): `resume_fields` (`:157`–`:168`) sets `source`
   `resume` and `session_id` (the given id, or `FORK_SESSION_ID` under `--fork-session`) on the recorded
   `SessionStart.default`; 7 call sites, all in the file (graph query 1). `tests/cli_revive.rs` and
   `tests/chaos_revive.rs` read the id the revive passed, so a recorded `SessionStart.resume-1.json` can replace the
   BASE payload but the fake agent must still write the given id into it: "replaces the rewritten payload" holds for
   the five extra keys, never for `session_id`.
8. **Fixtures** (`ls fixtures/claude/2.1.287/ | wc -l` → 31, `RELAYED.md` included): framing variants
   `UserPromptSubmit.paste-1..3`, `SessionEnd.clear-1`, `SessionStart.clear-1`; status line variants
   `StatusLine.default-1`, `StatusLine.rate-limits-1`. No `resume` or `compact` variant. `schemas/claude-fixture.v1.json`
   enumerates event names, so `SessionStart.compact-1` / `SessionStart.resume-1` are variants inside the enum (no
   schema change). Whether `/compact` also fires a SessionEnd before its SessionStart was not read live (the revive
   rig recorded SessionStart key names only).
9. **The live-start unit** (re-derived, not copied): one `viola verify` round starts `claude` five times (the print
   probe and Runs A–D; `version-probe` is `--version` only). A Run B paste adds no start but a `/compact` paste adds a
   live compaction (a model call) inside Run B. A resume child adds one start per round. A record round and a
   `run --local-live` re-stamp of the tmpfs home are two rounds when both are wanted. Host CLI: the 2.1.287 install
   is named by path (harness rule 2026-10-10).
10. **The older-stamp oracles** (`src/run/version_gate.rs@880b056`): `:301` "the fourteen rows of an older stamp",
   `:307` "all but the newline row", `:314` "the eighteen rows of an older stamp", `:320` "the eighteen rows are every
   row relied on here" (Windows); `ledger.rs:1993` `verified_reads_a_fourteen_row_stamp_as_unverified`, `:2027` "the
   eighteen rows before the statusline rows". Each row this chunk lands adds an older id set (the 21-row stamp).
11. **Relied on every OS — verified.** `src/cmd/revive.rs` carries no top-level `cfg` (its `cfg` lines are `:246`
   `#[cfg(test)]` and `:337` `#[cfg(unix)]` inside that test module; `command grep -n 'cfg(' src/cmd/revive.rs`), and
   `classify` / `LOCAL_COMMANDS` carry none. Both new rows are relied wherever `revive` and `send` run: the relied
   set would move 21 → 23 on Unix and 18 → 20 on Windows, the Windows count waiting on a Windows stamp ("Windows-only
   live measurements", `BLOCKED-ON` standing, `route.py pins`).
12. **`v1-34` stays unclaimable** (`matrix.py show --id v1-34`, notes through 2026-10-13): its acceptance also needs
   the harness-prefix row and the R8 identity-floor row, owed with no entry minted (the founder, 2026-10-07T09:43Z).

## Files inspected
- `crates/viola-agent-claude/src/ledger.rs` (`:40`–`:240`, `:580`–`:1020`) — rows, words, relied set, local-command
  list, framing turns, `check`, `framing_variants`, `merge_stamp`.
- `src/run/send/classify.rs` (`:1`–`:45`) — `Confirms`, `classify`.
- `src/run/send/slot.rs` (`:60`–`:180`) — `claim`, `session_started`, `settle`, `confirm`, `append_hook_event`.
- `crates/viola-agent-claude/src/screen.rs` (`:20`) — `CONFIRM_WINDOW_FALLBACK`.
- `src/cmd/verify/typed.rs` (`:20`–`:370`) — Runs A–D, `trusted_args`, `framing_turns`, `local_command_answered`.
- `src/cmd/verify.rs` (`:150`–`:300`) — the spawn pairs, the print probe, `typed::measure`, `Probes`, `merge_stamp`,
  `record`, `ProbeDir`.
- `src/bin/viola-fake-agent.rs` (`:140`–`:185`) — `resume_fields`, `screen`.
- `crates/viola-agent-claude/src/lib.rs` (`:127`, `:140`) — `is_session_id`, `resume_args`.
- `viola-0.1.0/chunks/2026-10-10-viola-revive/evidence/live-revive.md` (full) — the three live starts.
- `viola-0.1.0/chunks/2026-10-13-owed-ledger-rows/research.md` (`:1`–`:74`) — the previous measured facts.
- `.claude/rules/verification-harness.md` (full).
- `scripts/code-graph-cookbook.md` (full).

## Graph impact (rust plane, this run dir's `tree-query-2026-10-14-resume-and-compact-ledger-rows.json`, query 1)
- **session_started** — 1 caller, `src/run/send/slot.rs:184` (`append_hook_event`): the second claim arm lands inside
  it; no signature change needed for a new `PostCondition` member.
- **framing_turns** — 1 caller, `src/cmd/verify/typed.rs:141` (`trusted_run`).
- **trusted_run** — 1 caller, `src/cmd/verify/typed.rs:84` (`measure`'s run list).
- **framing_variants** — 6 rows: `src/cmd/verify.rs:520` (`record`) and 5 unit sites in `ledger.rs` (`:3049`–`:3093`).
- **local_command_cleared** — 1 caller, `ledger.rs:881` (`check`).
- **merge_stamp** — 11 rows: `src/cmd/verify.rs` 1, `src/run/version_gate.rs` 3, `ledger.rs` 7 (tests).
- **relied** — 5 rows: `src/cmd/verify.rs:445`, `:677`; `ledger.rs:1105`, `:1446`, `:1976`.
- **resume_args** — 4 rows: `src/cmd/revive.rs:11`, `:166`; `lib.rs:676`, `:680` (tests).
- **resume_fields** — 7 rows, all `src/bin/viola-fake-agent.rs`.
- **framing_stem** — 10 rows: `ledger.rs` 5, `src/bin/viola-fake-agent.rs` 5.
- **classify** — 17 rows, name-collided across crates (`dialog.rs`, `board.rs`, `liveness.rs`, `client.rs`,
  `hook.rs`, `run.rs`); the send one's callers are the 2 rows in `src/run/send.rs`.

## Patterns detected
- **A row = an enum member + id + words + `relied_on` arm + `check` arm** (`ledger.rs:46`–`:149`, `:850`): the
  statusline rows' landing is the newest instance.
- **A probed local command** (`ledger.rs:186`–`:206`, `typed.rs:340`–`:354`, `slot.rs:98`): a `LOCAL_COMMANDS` entry
  with its `PostCondition`, one compiled Run B paste waited on by `local_command_answered`, a claim arm in the slot,
  recorded `<Event>.<stem>-<n>` variants replayed by the fake agent's `--framing`.
- **Per-OS literal oracles** split by `cfg(unix)` / `cfg(not(unix))` (`tests/cli_verify.rs:28`/`:52`/`:73`/`:97`/`:119`/`:121`,
  `tests/contract_ledger_probes.rs:17`/`:41`/`:130`/`:132`, `crates/viola-e2e/src/harness/run.rs:335`/`:359`).
- **A verify run is a `RunStep` in `measure`'s array** (`typed.rs:81`–`:86`), each with its own `TrustedDir` guard and
  the stop check between runs.

## Conventions to follow
- **The test oracle is a literal, never `LedgerRow::ALL`** (test-plan §11; every file of the Sweeps below).
- **Per-OS reliance stated beside every count** (`relied()`, the 2026-10-13 amendments).
- **No sleep in a test; waits on `events.ndjson` offsets** (test-plan §11).

## New files to create
- `fixtures/claude/2.1.287/SessionStart.resume-1.json` — the recorded resume variant (`viola verify --record`, plan step 13)

## Files to modify
- `crates/viola-agent-claude/src/ledger.rs` — the resume row, words, `relied_on`, `check` arm, the resume helpers, the `/compact` local-command entry, unit cases
- `src/run/send/classify.rs` — unit rows for `/compact`
- `src/cmd/verify/typed.rs` — Run E
- `src/cmd/verify.rs` — the resume dirs and captures, `Probes` wiring, `record` of the resume variant
- `src/bin/viola-fake-agent.rs` — `--resume` from the recorded `resume-1` base
- `src/run/version_gate.rs` — the 21-row older-stamp oracle
- `crates/viola-e2e/src/harness/run.rs` — `LEDGER_ROWS` per OS
- `tests/cli_verify.rs` — `STEPS_PASS`, `ROW_IDS`, `STAMPED_PASS`, the partial-replay counts, the stop table, the run-count readers
- `tests/contract_ledger_probes.rs` — `ROW_IDS` per OS and the stamped line
- `tests/contract_fixture_hygiene.rs` — the named resume variant list
- `tests/contract_fake_agent_drift.rs` — the resume variant's replay
- `tests/cli_fake_agent.rs` — the fake agent's resume base
- `tests/cli_send.rs` — the `/compact` cross-process case
- `tests/support/verify.rs` — the shaped session id, the `after` parameter of `verify_started`
- `tests/support/home.rs` — `dialogless_home`'s summary literal, 17/14 → 18/15 (the revision, `inputs#I4`)
<!-- Rewritten at the plan revision (run dir .andromeda/runs/2026-10-16T07-29-18-phase/) to the write set the stopped
implement run made (outcome-…md §Files changed) plus its recorded companion. The take-up's provisional branches not
built — crates/viola-agent-claude/src/screen.rs, src/run/send/slot.rs, tests/cli_revive.rs — are dropped: fork 1 left
/compact without a post-condition, and the revive cases read the given id unchanged. -->

## Sweeps
- `"statusline-rate-limits"` over the whole tree (`command grep -rln … --include=*.rs .`): 4 files ·
  4 changed (`ledger.rs`, `crates/viola-e2e/src/harness/run.rs`, `tests/cli_verify.rs`,
  `tests/contract_ledger_probes.rs`) · 0 no-change.
- `"paste-newline"` over the whole tree: 4 hits, the same 4 files · 4 changed · 0 no-change.
- `clear-1` over the whole tree: 6 files · 6 changed if `/compact` is built (`ledger.rs`, `tests/cli_fake_agent.rs`,
  `tests/cli_verify.rs`, `tests/contract_fake_agent_drift.rs`, `tests/contract_fixture_hygiene.rs`,
  `tests/support/verify.rs`) · 0 no-change.
- `;\s*(21|18)\]` over `src crates tests`: 9 hits · 8 changed (`ledger.rs:46`, `:1712`; `harness/run.rs:335`, `:359`;
  `cli_verify.rs:28`, `:52`, `:73`, `:97`; `contract_ledger_probes.rs:17`, `:41`) · 1 no-change
  (`crates/viola-e2e/src/harness/run/mutants/host.rs:618`, a 21-case cfg table unrelated to the ledger).
- `"…(16–21) pass` over `src crates tests`: 9 hits in `tests/cli_verify.rs` (`:119`, `:121`, `:1131`, `:1188`, `:1190`,
  `:1266`, `:1268`) and `tests/contract_ledger_probes.rs` (`:130`, `:132`) · 9 changed · 0 no-change.
- Shape readers listed by the function the row set decides (the deferred learning): `LedgerRow::ALL` / `relied()`
  (membership: `ledger.rs` unit `:1712`, `:1446`, `:1976`); `check` (the verdict vector); `merge_stamp` (the stamp's
  rows object: `ledger.rs` 7 sites, `version_gate.rs` 3); the version gate's older-stamp oracle (fact 10);
  `framing_variants` / `FRAMING_TURNS` (the framing-variant list: 6 sites); the dialog-less summary
  (`cli_verify.rs:1131`/`:1188`/`:1190`); the harness `LEDGER_ROWS`. Each is a plan step's named case list.
- The revision's re-read (the stopped run's tree against `880b056377f4`): `git diff --name-only 880b056377f4` with the
  preservation guard's own exclusions prints 1 path, `tests/support/home.rs` · 1 changed (the `dialogless_home`
  summary, `17 pass  4 fail` → `18 pass  4 fail` Unix and 14 → 15 Windows, a dialog-less summary reader the sweep
  above missed: it was keyed on `… pass` literals over the summary sites in `tests/cli_verify.rs`, and this reader
  builds its line from a `passed` integer) · 0 no-change.

## Open questions
- Does the chunk build both rows, and how is `/compact` confirmed given fact 3 (a window of its own, a listed
  command left `unconfirmable`, or deferred)? → blocks: plan-decision (P4 fork, the founder's word)
- The verify child-set widening for `--resume` (a sixth start resuming Run B's FIRST id, a new id source) and for a
  `/compact` paste in Run B before `/clear`, and this chunk's live-start cap → blocks: plan-decision (P4 forks, the
  founder's own word, never a decisive lean)
- Whether `/compact` fires a SessionEnd before its SessionStart, and whether Run B's resumed FIRST id is found from
  the resume run's dir → blocks: implementation-scope (read at the record round; the variant list follows it)
