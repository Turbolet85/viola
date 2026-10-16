# P5 review card 1 — 2026-10-16-unwrapped-sessions-on-the-board · Unwrapped sessions on the board

Plan: `viola-0.1.0/chunks/2026-10-16-unwrapped-sessions-on-the-board/plan.md` (17 steps · 12 acceptance criteria ·
24 gate entries). Research and scope: the same chunk folder. The founder's answers: `inputs#I1`.

## Goal
`viola list` reads `claude agents --json` once (5 s bound, R8 strip, `MAX_FRAME + 1`). It joins each row to a wrapped
instance by `child_pid` or `agent_session_id` and gives that row STATUS `idle` or `busy`. Every unjoined row prints as
a read-only row under `-- UNWRAPPED - READ-ONLY --` (NAME = its `sessionId`, `n/a` in WHEEL, DIALOG and CLI) and as a
`wrapped:false` item in `--json`. A failed, oversize or malformed read changes nothing `list` prints today.

`viola revive` gains `session-live` (holder → session → **session-live** → directory). `--fork` skips it, a failed
read fails open, and the wording is the founder's.

The join field becomes ledger row 23, `agents-join`, relied on every OS (23 Unix / 20 Windows). Its probe is ONE agents
read inside Run B before `/clear`, so verify spawn pairs go 7 → 8 and no start is added. The fake agent answers
`agents --json` from files beside its exe (`agents-mode`, `agents-rows.json`, a live `sessions/` registry), with no
env. There is one 512-case property, one fuzz target, and one recorded row (the probe's own) at
`fixtures/claude/2.1.287/Agents.recorded-1.json`.

## Decided at P4
- **The founder's own words** (`inputs#I1`, live in the overseer's dialog, 2026-10-16T19:54:09Z, relayed verbatim):
  - fork 1: relied, probe in Run B (a verify-child widening, ratified);
  - fork 2: files beside the exe;
  - fork 3: refusal now plus a P4 leg, cap **8 live starts** (6 + 2);
  - fork 4: the "open elsewhere / --fork" pair.
- **Narrowed within fork 1, for your confirm:** the probe makes ONE read before `/clear`. That is the 7 → 8 pairs
  the fork's text stated, though its preview drew a second read. P12 moves to the P4 leg, at no added start.
- **Leaned on artifacts (confirm-class):**
  - NAME = `sessionId` (architecture "session identifier"; the cli bans upstream text, and the CLI's `name` is a
    free-text title);
  - the snapshot gains `agent_session_id` as architecture lists it (it is absent today);
  - `oversize` is logged for the agents parser (the schema already admits it);
  - the recording is made on 2.1.287;
  - an empty wrapped rack keeps the founder-ruled empty-board line, with the unwrapped rack after it;
  - no truncation or colour ("CLI output discipline").

## Top acceptance criteria
- `verification-matrix.json#v1-37` (below) under the fake agent on three OSes.
- A failed, oversize or malformed read gives the same exit and stdout as the no-`claude` case.
- `LedgerRow::ALL` = 23, last `agents-join`, `[NN/23]` on Unix and `[NN/20]` on Windows. The record round prints
  `stamped 2.1.287  23 pass  0 fail`.
- `session-live` prints exactly:
  - `unable: <name>'s session is open in another claude`
  - `hint: close it there first, or start a copy: viola revive <name> --fork`

  It exits 1 with no child. `--fork` passes, and a failed read passes.
- Every test-started `viola list` / `revive` runs under a test-scoped `PATH`. On this host the tests would otherwise
  read your 11 live sessions.
- At most 8 `claude` starts across the record round and the P4 / P12 leg.

## Touchpoints
- **New:** `crates/viola-agent-claude/src/agents.rs` · `src/run/agents.rs` · the fuzz target and corpus ·
  `schemas/claude-agents-fixture.v1.json` · `fixtures/claude/2.1.287/Agents.recorded-1.json` ·
  `tests/cross_list_rows.rs` · the proptest regressions file.
- **Modified (26):**
  - ledger, board, snapshot, the slot's session-start write;
  - list, the human board, revive, verify and its Run B, the fake agent;
  - the diag-line schema, fuzz manifest and lock, `.gitattributes`, nextest's kill line;
  - eight test files and supports.

## Test commands
- **Lint and unit:** fmt, clippy, G1, lint-probes, `run --unit`, plus the new cases by name (baseline red: 0
  selected).
- **The live round, fired once:** the 2.1.287 probe, then the **record round (6 starts)**, then the census.
- **The fixture guard:** baseline red on an empty input. Clone controls: the variant alone green; a modified fixture
  red; two additions red.
- **The integration filter (11 binaries):** baseline red, because `cross_list_rows` doesn't exist yet.
- **The rest:** fuzz replay green 6.13 s; both `cargo deny` reads green; the smoke cleanup / boot / status / cleanup
  green; G2, G4, `pre-push`; then the operator's hygiene, push and `ci.py conclusion`.
- **Not an entry:** the P4 / P12 leg (step 17, **2 starts**, the committed headless rig, attended). Its record is
  `evidence/live-p4.md`, counts and codes only.

## Concretization preview — `v1-37` (the one capability claimed)
- **Current acceptance:** list works with no live wrapper and shows each wrapped session's liveness (live or stale,
  gone omitted), idle or busy status, wheel holder with any budget pause, pending dialog kind and CLI verdict, plus
  unwrapped sessions as read-only rows that are never valid targets; human output is the fixed six-column board with
  its BAY line, every row single-line with controls escaped; the machine-readable output equals the GUI sessions
  envelope.
- **Proposed concrete form:**
  - With no live wrapper, `viola list` exits 0 and prints the board.
  - Each wrapped instance shows liveness `live` / `stale` (gone omitted) and STATUS `idle` / `busy` from the
    `claude agents --json` row it joins by `child_pid` or `agent_session_id`, else `unknown`. It also shows the wheel
    holder with any budget pause, the pending dialog kind and the CLI verdict.
  - Every unjoined agents row prints under `-- UNWRAPPED - READ-ONLY --` with `n/a` in WHEEL, DIALOG and CLI and as
    `wrapped:false` in `--json`, and a `send` naming one exits 21.
  - Human output is the six-column board with its BAY line. Every row is one line, every control (`\n` and `\t`
    included) escaped as `\xHH` (`cross_list_rows`: piped, `NO_COLOR` / `TERM=dumb`, outer PTY).
  - `--json` is `{"v":1,"ok":{"items","budget","skipped"}}`.
  - Proved under the fake agent by `binary(cli_list) | binary(cross_list_rows)` on the three CI OSes.
  - The GUI-envelope clause is out of 0.1.0 by the founder's ruling of 2026-10-16 (v1-37's own notes). The CLI half
    is unweakened.
- **Method:** `e2e`, unchanged.

## Flags
- Plan prose is 457 lines (fence excluded), against the template's ~400. That is within the warn band, but the chunk
  is large: ten surfaces, two live legs.
- Every home stamped before this chunk reads `cli_verified:false` until its next `viola verify` (23 relied rows).
- If P4 reads `refused` or `copied`, `session-live` stays a refusal in this chunk. Its conversion to a warning is the
  route row below.

## Beyond the plan
- `boot --agents-mode recorded|oversize|malformed` (test-plan §3, not built): owned by the existing entry
  **"End-to-end path binary and harness legs"**, with `status` running `viola list`. Under fork 2 the flag writes the
  fake's `agents-mode` file rather than an env var.
- The unwrapped-name `instance-unreachable` hint (`<name> is not a wrapped session; only sessions started with viola
  run take commands`, design-system cli pattern 2, not built). This chunk proves the exit 21 only. Owned by the
  existing entry **"Exit-cause code catalogue"** (`v1-18`).
- The Windows readings of `claude agents --json`, of the `agents-join` row and of `session-live`: appended to the
  existing entry **"Windows-only live measurements"**.
- **Conditional route insertion**, only if the P4 leg reads `refused` or `copied`: a new working-route entry
  **"Session-live as a warning"**, placed directly after this entry in Epoch 5. It turns `session-live` into a warning
  on the founder's word. Not minted on a `two writers` reading.

Apply? (yes / review / cancel)
