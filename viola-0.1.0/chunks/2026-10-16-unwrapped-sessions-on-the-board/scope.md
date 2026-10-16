# Scope — 2026-10-16-unwrapped-sessions-on-the-board · Unwrapped sessions on the board

**Working entry** (`viola-0.1.0/working-route.md`, the first markerless entry at take-up, ninth under
`### Epoch 5 — Driver surface`, directly after "Remove the browser leg"): Unwrapped sessions on the board —
`claude agents --json` read once, unwrapped read-only rows, idle/busy STATUS, revive refuses a session id live
elsewhere (v1-37) — plus five CARRY blocks (folded below; completeness check `route.py pins --run-dir` → this run
dir's trail: five rows on the entry, 776 · 494 · 527 · 1189 · 682 chars, no abstention, no `BLOCKED-ON`, no `WATCH`,
no `PREREQ`).

**The operator's direction at take-up:** none beyond the invocation (`/andromeda-phase`, no flags). No external
input is snapshotted at P1 or P3: every fact below is read from the entry, from a file of this repository (the
earlier chunks' `inputs/` and `evidence/`, `refs/session-memory-options.md` included), or off the installed CLI on
this host (`research.md` §Measured on the Linux dev host).

**CI read at Setup 5a** (the last wrap's flip `92264a7` = HEAD, `ci.py conclusion`, this run dir's trail):
- `92264a7f254c`: **in progress** at Setup — `CI 92264a7f254c: verdict not yet available`, checks 15/15 listed, run
  `ci#38950072826` (push) open, its oldest running job `lint (ubuntu-latest)` at 138 s when read. Not folded and not
  read as green there. Re-read at P3 (`ci.py conclusion`, this run dir's trail): **green**, checks 15/15, wall 542 s,
  `ci#38950072826` completed/success. No red or `not green` to disposition.

**Premise closure (P3):** every `[inferred]` bullet below was read against `research.md`. A verified bullet has
lost its tag, and a falsified one carries `[premise-corrected: …]`.

## What this chunk builds

Today `viola list` (`src/cmd/list.rs:35`, `crates/viola-state/src/board.rs`) reads `<home>/instances/` and
`budget.json` only and spawns no `claude agents --json`: every item is `wrapped: true` with `status: "unknown"`
(`board.rs:179`, `:232`), and no unwrapped row and no `-- UNWRAPPED - READ-ONLY --` separator is printed, on the human
board or in `list --json` (`.claude/rules/api.md` §Wrapper channel; `.claude/rules/frontend.md` §CLI surface). `viola
revive`'s preflight refusals are `no-session` · `cwd-missing` after the instance check, with `already-live` being
`run`'s own collision refusal (`src/cmd/revive.rs:45-93`); no refusal reads whether the id is live elsewhere.

### 1. The read: `claude agents --json`, once per `list` (entry; CARRY 1)
- `viola list` runs `claude agents --json` once and joins its rows to the wrapped instances.
- Measured at P3 on 2.1.296 and on 2.1.287 alike:
  - one JSON array whose rows each hold exactly `cwd, kind, name, pid, sessionId, startedAt, status`;
  - `pid` and `startedAt` (epoch ms) are integers;
  - `status` ∈ {`busy`, `idle`}, `kind` = `interactive`, and every `sessionId` is a lowercase UUID.

  This is the study's 2.1.283 field set unchanged (`refs/session-memory-options.md` §1 M5). `--all` adds a
  `background` row of another shape (no `pid`, no `status`). viola reads the bare `--json`. One read takes about
  125 ms (`research.md` §Measured).
- [premise-corrected: `run_bounded` (`src/run/version_gate.rs:41`) drains stdout through `take(MAX_FRAME)` and keeps
  no over-the-cap flag, so an over-`MAX_FRAME` output reads as a cap-length one] The spawn follows the security
  plan's child rules: a direct spawn of the resolved program (never a `.cmd` / `.bat` shim, never through a shell),
  under the R8 strip, killed at a time bound (`list` "works with no live wrapper" and must not hang on a stuck child).
  Its stdout is read through `take(MAX_FRAME + 1)`, so an over-cap output is told apart and read as a failure.
- The parse is tolerant and closed: an over-`MAX_FRAME`, malformed, absent or failed read leaves every wrapped row
  `unknown` and lists no unwrapped row, and `list` never fails or changes exit on it (architecture §Standard Contracts,
  Session liveness; test-plan §6; a row `name` never becomes a `ViolaName`, test-plan §4).

### 2. idle/busy STATUS on wrapped rows (entry; CARRY 1)
- A wrapped row's STATUS becomes `idle` or `busy` from the agents row it joins, else `unknown`.
- The mapping is one to one over the measured values (`status` ∈ {`busy`, `idle`}, P3). A value outside them reads
  `unknown`.

### 3. Unwrapped read-only rows (entry; CARRY 1)
- Every agents row that joins no wrapped instance is listed as an unwrapped read-only row: `wrapped: false`,
  `liveness: "live"`, `dialog_pending: false`, no `wheel` / `budget_paused` / `cli_version` / `cli_verified`
  (architecture §Standard Contracts, `list` payload). On the human board it sits under the
  `-- UNWRAPPED - READ-ONLY --` separator, after every wrapped row, with `n/a` in WHEEL, DIALOG and CLI (design-system
  §Surface: cli, pattern 1; layout-templates §Output structure — `viola list`).
- Rows escape every control character through the existing `escape_row` (`src/human/escape.rs:20`), over the call's
  own fields (CARRY 1: "the rows' escape on that call's fields"). As built, NAME is printed raw (`board_rows`) because
  every name is a `ViolaName`. An unwrapped name is not one.
- An unwrapped row is never a valid target (v1-37's requirement; test-plan §6, E1; security-plan §Security
  Anti-Patterns, Input). Its name is not a `ViolaName`.
- [premise-corrected: the CLI's `name` field is a free-text session title (9–35 chars, spaces and non-ASCII
  measured, never equal to `sessionId`), while architecture names the unwrapped `name` "the session identifier" and
  the design sample shows a hex id] Which field fills the unwrapped NAME cell and the `--json` `name` is a P4 fork.
  The design and layout sources fix the rest of the rack: no seventh column, no pid.

### 4. The join field: a capability-ledger row with its probe (CARRY 5; CARRY 1)
- CARRY 5, verbatim in substance: the `claude agents --json` join-field ledger row (child pid or session id) and its
  `viola verify` probe land here. It was moved from "Owed ledger rows" at that chunk's wrap on the founder's own answer
  to its P4 fork 2 («К потребителю»).
- CARRY 1 names the join field as coming "from 'Paste newline ledger row'". Re-verified at P1: that entry and "Owed
  ledger rows" are complete and neither landed the row. `LedgerRow::ALL` holds 22 rows
  (`crates/viola-agent-claude/src/ledger.rs:47`), none of them for `agents --json`. CARRY 5's later move to this entry
  is the standing owner, and the CARRY 1 phrase is a stale coordinate.
- Both candidate fields are exposed (P3: `pid` and `sessionId` on every row). On this host `claude` is a native ELF
  executable, so the PTY's `child_pid` is the `claude` process the agents row names. Known positive: this builder
  session's own pid read exactly one row, `busy`.
- [premise-corrected: `InstanceSnapshot` (`crates/viola-state/src/snapshot.rs:39-69`) has `child_pid` but no
  `agent_session_id`, although architecture §Standard Contracts (Instance snapshot) lists `agent_session_id?` "only to
  join `claude agents --json` rows"; the id is held only in the log (replay) and in the send slot's memory] Whether the
  join keys on `child_pid` alone or also on a snapshot `agent_session_id` the wrapper starts writing is a P4 fork.
- The study's P12 (does `sessionId` follow the new id after `/clear`?) decides whether a session-id join survives a
  `/clear`. The `list` join and the revive preflight both rely on it (`refs/session-memory-options.md` §5, P12). Run
  B already pastes `/clear` (`FRAMING_TURNS`, `ledger.rs:206-211`), so a second probe read after that paste could
  measure P12 inside Run B (`research.md`). Amended at P5 validation-1, intent-incomplete: the founder's answer
  (`inputs#I1` fork 1) stated one added verify child, spawn pairs 7 → 8. The probe therefore reads once, before
  `/clear`, and P12 is measured in the P4 leg with no added start (`plan.md` step 17 (b)).
- The cost frame of the previous ledger-row chunks stands (arch, security, tests history files of this run dir):
  - A new row joining the relied set (`LedgerRow::relied()`, `ledger.rs:149`) leaves every home stamped before it
    `cli_verified:false` until re-stamped.
  - Each new row states on which OS it is relied.
  - A probe that adds a `viola verify` child or a live start is a boundary widening of verify's child set, ruled only
    by the founder's own word, with a live-start cap of its own (the CARRY of "Resume and compact ledger rows", its
    `scope.md` §0; `.andromeda/playbook.md`, "Boundary widening").

  Whether the row is relied (gating `cli_verified`), or only a ledger record exempt under the ruled limit because no
  `send` relies on it, is a P4 fork.

### 5. The fake agent's `agents --json` stub (CARRY 2)
- Deferred to this entry by chunk 2026-09-24-fake-agent-and-test-data-fixtures: the fake agent answers
  `agents --json` with `FAKE_CLAUDE_AGENTS_MODE=recorded|oversize|malformed` (test-plan §7 seed table,
  `.andromeda/test-plan.md:926`), its shape pinned against a recorded `claude agents --json` output.
- [premise-corrected: architecture §Occupied Resources lists the fake agent's interface as argv options with no env,
  and the env seam list is closed at two founder-ratified seams; test-plan §7's env channel contradicts it (P2
  aggregate C)] The mode channel is a P4 fork. Today the fake agent reads no mode env and lets
  `claude agents --json` fall through to interactive mode (`src/bin/viola-fake-agent.rs:917-973`).
- The recording lands under `fixtures/claude/<cli-version>/` beside the existing sets. 2.1.287, the stamped set's
  version, is still installed on this host (P3). It passes the `--record` scrub (no home path, absolute path, username
  or email): the rows' `cwd` is an absolute path, so it is rewritten or the recording is refused. It holds no session
  content, and it takes a fixture class and schema of its own (the `StatusLine.` precedent,
  `tests/contract_fixture_hygiene.rs:24-60`).

### 6. The parser's property and fuzz target (CARRY 3; CARRY 1)
- The `claude agents --json` parser over arbitrary JSON takes a proptest property (`cases: 512`, committed seeds),
  a `fuzz/fuzz_targets/` target and a seeded corpus (test-plan §6 Property suite).
  - The fuzz pipeline today holds `channel_frame`, `hook_stdin`, `paste_text`, `viola_name` and `vt100_feed`.
  - `viola-fuzz` already path-depends on `viola-agent-claude` (`fuzz/Cargo.toml`).
  - The replay and nightly jobs enumerate the targets from the directory (`research.md`).
- CARRY 1 names the property and fuzz target as `verification-matrix.json#v1-37`'s.

### 7. `viola revive` refuses a session id live elsewhere (entry; CARRY 4)
- From chunk 2026-10-10-viola-revive (the founder's answer to its review card, relayed; that chunk's snapshot
  `viola-0.1.0/chunks/2026-10-10-viola-revive/inputs/I5-relay-1.md.txt`, point 1): the `session-live` preflight refusal reads `claude agents --json` for a row whose session id is the one
  about to be resumed. Until it lands, viola does not refuse a revive of an id live elsewhere.
- It joins revive's preflight order (`already-live` · `no-session` · `cwd-missing`, `src/cmd/revive.rs`). The
  strict-modes check runs ahead of the preflight and of the wrapper log (since chunk 2026-10-10-budget-governor).
- The design and layout rules forbid the pid that the study's wording names (study §3 V2: "refuse `session-live`
  (naming the pid)"). The refusal's `unable:` / `hint:` pair names no pid.
- Two of the study's measurements travel with it (`refs/session-memory-options.md` §5):
  - **P4**: is an interactive `claude --resume <id>` of a live id refused, copied, or two writers on one transcript?
    The answer decides whether `session-live` is a refusal or a warning.
  - **P12**: the `/clear` id follow (§4 above).
- `--fork` (→ `--fork-session`, `resume_args(&id, args.fork)`, `src/cmd/revive.rs:165`) is the study's opt-in escape
  (§3 V2), so a forked revive skips the `session-live` reading.
- A failed or unparseable agents read fails open and revive proceeds as today (architecture §Standard Contracts: the
  read's failure "is never an error"; the human always wins). Whether that is right for a refusal that guards two
  writers on one transcript is a P4 question if P4's measurement reads "two writers".
- The new refusal is a closed `process-exit` detail: the 13th member of `schemas/diag-line.v1.json`'s
  `process-exit.detail`, which is closed at 12. Exit 1, a revive preflight refusal (`.claude/rules/api.md` §CLI).

### 8. v1-37 (entry)
- `verification-matrix.json#v1-37` is unclaimed. Its notes record the board chunk's advance and what is still owed:
  idle/busy status, the unwrapped rows, and the GUI-envelope equality. The 2026-10-16 note took the GUI-envelope
  clause out of 0.1.0 by the founder's ruling. The CLI half stands unweakened. This chunk is the cap's last named
  owner. Every clause left once the GUI clause is out is built here or already built by the board chunk. Whether the
  concretized acceptance is provable at this chunk's wrap is P5 step 4's.

## Boundaries (not this chunk)
- [premise-corrected: no `viola-mcp` crate exists (`ls crates/`), so there is no MCP `list` reader to extend] The MCP
  `list` tool is "MCP server for drivers"'s. The colour layer and width-known truncation on `list` are "CLI output
  discipline"'s. `--json` on `viola revive` is "CLI machine contract"'s. The harness's `status` running `viola list`
  (and a `boot --agents-mode` flag with it) is "End-to-end path binary and harness legs"'s.
- Windows readings of `claude agents --json` and of the join row are owed to "Windows-only live measurements", like
  every earlier row's Windows leg. This chunk measures on the Linux dev host.
- No kill of any session: viola never kills a session it did not spawn in this process (study §3, failure-mode table).
- The GUI sessions envelope is 0.2.0's (the founder's ruling, 2026-10-16, on v1-37's notes).
