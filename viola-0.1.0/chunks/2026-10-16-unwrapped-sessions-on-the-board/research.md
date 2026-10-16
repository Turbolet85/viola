# Codebase Research — 2026-10-16-unwrapped-sessions-on-the-board

## Scope
- **Depth:** deep · **Reads:** 24 · **Globs/Greps:** 21
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read whole (71 lines, 19 Session Additions). Two
  apply here. 2026-10-10: on the Linux dev host the bare `claude` can resolve to a newer CLI than a home is stamped
  for, so a live rig puts the stamped version's install dir first on `PATH`. 2026-10-11: `schema-check` (G4) reads
  kept homes only, so a plan note never claims it validates the new cases' lines.
- **Platform issues consulted:** none — no runner-only bullet. CI on the scope's one sha re-read green (below), and no
  CI-reading entry falls outside the operator leg.
- **External inputs:** none — every fact this chunk turns on lives in this repository, or was read off the installed
  CLI on this host (the CLI's behaviour, recorded below as a measurement, not a snapshot).

**CI re-read at P3** (`ci.py conclusion`, this run dir's trail): `92264a7f254c` **green**, checks 15/15, wall 542 s,
`ci#38950072826` completed/success. No red and no `not green` to disposition.

## Measured on the Linux dev host (P3, 2026-10-16)
Each reading printed only shapes, never session content (scripts in the session scratchpad: a key/type/closed-value
census, a pid lookup, a `name`-field form census).
- `claude --version` → `2.1.296 (Claude Code)` (mise `latest`). Also installed: `2.1.287` (the stamped set's
  version), `2.1.288`, `2.1.289` (`ls ~/.local/share/mise/installs/claude/`). Both `2.1.296/claude` and
  `2.1.287/claude` are native ELF x86-64 executables (`file -L`).
- `claude agents --json` on 2.1.296: one JSON array of 11 rows (2 911 bytes).
  - Every row's key set is exactly `cwd, kind, name, pid, sessionId, startedAt, status`.
  - Types: `pid` and `startedAt` are integers (`startedAt` 13 digits, epoch ms); the other five are strings.
  - `status` ∈ {`busy`, `idle`}, `kind` ∈ {`interactive`}, and all 11 `sessionId` values are lowercase 8-4-4-4-12
    UUIDs.
  - On 2.1.287 the same census gives the same 7 keys, types and closed values.
  - The study's 2.1.283 field set (`refs/session-memory-options.md` §1 M5) therefore still holds on both builds.
- `claude agents --json --all` adds one `background` row of another shape: `cwd, id, kind, name, sessionId, startedAt,
  state`, with no `pid` and no `status`. viola reads no `--all` (the scope's read is the bare `--json`).
- **Known positive** (this builder session's own `claude`, pid 3231528, the shell's parent): exactly 1 row whose
  `pid` equals it, `status: busy`, `kind: interactive`, read while a turn was running. **Control:** pid 1 → 0 rows.
- The `name` field is a free-text session title, not an id. Across the 11 rows it was 9–35 characters, one row held
  spaces and non-ASCII, and none equalled or prefixed its `sessionId`. It is user content.
- Wall time of one read: 124 / 126 / 127 ms (2.1.296), 115 ms (2.1.287) (`date +%s%N` around each call).
- Not measured here, on purpose:
  - What the R8 strip changes in this output. Listing this session's `CLAUDE*` names would cross the credential
    boundary; that answer is spent, per the 2026-10-07 security sidecar entry.
  - P4 (an interactive `--resume` of a live id).
  - P12 (`sessionId` after `/clear`).
  - Each needs a live start, which is the plan's to ask for.

## Files inspected
- `src/cmd/list.rs` (full) — `list` = `board::scan` (strict-modes, before any log) → `client::start` under a name →
  `read` (`board::read`, `read_budget_stamped`, `read_thresholds`) → one `--json` document or `human::write_board`.
  No `claude` resolution and no child spawn today. `document` serialises `board.items` whole.
- `crates/viola-state/src/board.rs` (1–260) — `SessionStatus` has one variant, `Unknown`. `Item` has `wrapped: bool`,
  `status`, optional `wheel` / `budget_paused` / `cli_version` / `cli_verified`, and a non-serialised `dialog_kind`.
  `snapshot_item` and the replayed arm set `wrapped: true, status: Unknown`. `read` takes `(scan, now)` and has no join
  input. It already runs one whole `log_pass` per snapshot-read instance for the skip counts.
- `crates/viola-state/src/snapshot.rs` (30–90) — `InstanceSnapshot` holds `child_pid: Option<u32>` but **no
  `agent_session_id`** (see Graph impact and the closure).
- `src/human/board.rs` (1–240) — `board_rows` pads NAME to `max(4, longest name)` and lays out the five fixed widths
  (`LIVE_WIDTH` 5, `STATUS_WIDTH` 7, `WHEEL_WIDTH` 23, `DIALOG_WIDTH` 17, CLI unpadded). `escape_row` is applied only
  in `cli_cell`. NAME is printed raw, which is safe today because every name is a `ViolaName`. `write_board` makes one
  write of BAY, caption, rows, or `EMPTY_BOARD`.
- `src/human/escape.rs` (`escape_row`, line 20) — escapes every control character, `\n` / `\t` / DEL / C1 included.
- `src/cmd/revive.rs` (40–230) — `Refusal { StrictModes, NoSession, UnknownId, CwdMissing }` with `detail()` / `pair()`.
  The `trusted` check runs before `open_wrapper_log`. `preflight_order(holder, session, recorded_dir)` is generic over
  three closures. `preflight` builds `resume_args(&id, args.fork)`. `ReviveArgs` has `--id`, `--fork`, `--list` and
  trailing words.
- `src/run/version_gate.rs` (1–140) — `run_bounded(program, args, cwd, &StripPlan, deadline, stopped)`: a direct
  spawn, stdin null, `env_remove` per the strip plan, both pipes drained on threads through `take(MAX_FRAME)`, killed
  and reaped at the deadline. `VERSION_DEADLINE` = 5 s. Its drain **truncates at `MAX_FRAME`** and keeps no
  over-the-cap flag, so an over-cap read looks like a cap-length one.
- `crates/viola-agent-claude/src/ledger.rs` (1–300) — `LedgerRow` holds 22 variants. `ALL: [Self; 22]` comes first,
  then `id()` / `words()`, then `relied_on(wrapped)` (only the three statusline rows depend on the OS), then
  `relied()`. Run B's compiled pastes are `FRAMING_TURNS`, whose fourth turn is `/clear` (`PROBE_LOCAL_COMMAND`).
- `crates/viola-agent-claude/Cargo.toml` — its dependencies are `viola-core`, chrono, serde, serde_json,
  serde_path_to_error, thiserror and vt100, with **no `viola-state` edge**. Its dev-dependencies include proptest.
- `src/bin/viola-fake-agent.rs` (370–410, 910–1020) — `main` takes a first-word mode (`statusline-echo`), then
  `Opts::parse`; unknown words are ignored. Today `claude agents --json` falls through to interactive mode and
  waits on stdin. No env var is read for a mode.
- `fuzz/Cargo.toml` — `viola-fuzz` already path-depends on `viola-agent-claude`. Five `[[bin]]` targets.
- `schemas/diag-line.v1.json` — `$defs.subject.enum` already holds `agents-probe`. `parse-rejected.parser` already
  holds `claude-agents-json`. `parse-rejected.detail` is ONE flat enum that already holds `oversize` and `malformed`,
  so the schema admits `{parser:"claude-agents-json", detail:"oversize"}`; only obs-plan §6's per-parser catalog line
  names `malformed` alone. `process-exit.detail` is closed at 12 members (the 2026-10-13 obs sidecar entry), so a
  `session-live` code is a 13th.
- `tests/contract_fixture_hygiene.rs` (1–100) — walks `fixtures/claude/*/*.json` as hook payloads against
  `schemas/claude-fixture.v1.json`, except files named `Screen.*` / `StatusLine.*`, which each have their own walker and
  schema. A recorded agents file needs its own class, or the hook walker reads it.
- `fixtures/claude/2.1.287/` — 32 files by stem (`ls | sed 's/\..*//' | sort | uniq -c`): PermissionRequest 4 ·
  PostToolUse 5 · PreToolUse 7 · RELAYED 1 · Screen 3 · SessionEnd 2 · SessionStart 3 · StatusLine 2 · Stop 1 ·
  UserPromptSubmit 4.
- `tests/cli_revive.rs` (index) — preflight cases (`revive_on_a_live_name_…`, `…_is_no_session`, `…_is_cwd_missing`,
  strict-modes cases, `revive_with_fork_reports_the_fork_session`) through `assert_refused(home, before, detail)`.
- `.config/nextest.toml:24` — the 20 s kill filter lists 18 `binary(…)` names, `cli_list` among them.
- `crates/viola-e2e/src/harness/run/fuzz.rs:22-24` — `run --fuzz-replay` reads every `fuzz/fuzz_targets/<target>.rs`
  from the directory. `.github/workflows/nightly.yml:53` loops `fuzz/fuzz_targets/*.rs`. Neither keeps a list to edit.
- `.gitattributes:6` — `fuzz/corpus/paste_text/** binary`, the byte-exact precedent for a new corpus.
- `src/run/send/slot.rs` (index) — `append_hook_event` is the hook tap, and the send slot remembers the last
  `session-start`'s `agent_session_id` in memory only. The snapshot handle writes `child_pid` once at spawn
  (`src/cmd/run.rs:604`).
- `crates/viola-e2e/src/**` — no `--agents-mode` and no `FAKE_CLAUDE_AGENTS_MODE` anywhere (grep over `src`,
  `crates/viola-e2e/src`, `scripts`, `tests`: 0 hits).
- `crates/` — `viola-agent-claude viola-channel viola-core viola-e2e viola-pty viola-state` (`ls crates/`). There is
  **no `viola-mcp` crate**, so no MCP `list` reader exists to extend.

## Graph impact (from the code-graph query; trace `tree-query-2026-10-16-unwrapped-sessions-on-the-board.json`, rust plane)
Line numbers below are editor lines (the graph's 0-indexed `line` + 1).
- **board::read** — 1 product caller, `src/cmd/list.rs:86` (`cmd/list/read()`), plus its unit tests in
  `crates/viola-state/src/board.rs`. A signature change threads through `list` alone.
- **snapshot_item** — 1 caller, `crates/viola-state/src/board.rs:222` (`board::read`).
- **board_rows** — `write_board` (`src/human/board.rs:180`) plus 5 unit tests in the same file.
- **write_board** — `src/cmd/list.rs:57` and `src/human.rs:14` (the re-export), plus 2 unit tests.
- **escape_row** — `cli_cell` (`src/human/board.rs:128`) plus its own unit test.
- **preflight_order** — `preflight` (`src/cmd/revive.rs:150`) plus 2 unit tests. **preflight** — `revive`
  (`src/cmd/revive.rs:106`).
- **resume_args** — `revive::preflight` (`src/cmd/revive.rs:165`) and `verify/typed::resume_run`
  (`src/cmd/verify/typed.rs:251`).
- **run_bounded** — `verify::run_logged` (`src/cmd/verify.rs:106`) and `version_gate` (`src/run/version_gate.rs:136`).
- **resolve_program** (agent crate) — `run::resolve_program` (`src/run/mod.rs:151`), `cmd/run/start`
  (`src/cmd/run.rs:223`) and `verify::measure` (`src/cmd/verify.rs:132`).
- **relied** — `ledger::verified` (`crates/viola-agent-claude/src/ledger.rs:1144`) and `verify::check_rows`
  (`src/cmd/verify.rs:455`). **relied_on** — `relied_here` (`ledger.rs:135`).

## Patterns detected
- **Bounded child read** (`src/run/version_gate.rs:41-83`): a direct spawn under the strip plan, both pipes drained
  through the frame cap, killed and reaped at a deadline. The agents read is this shape. It has to tell an over-cap
  output apart (`take(MAX_FRAME + 1)`), which `run_bounded` cannot today.
- **Spawn pair at the call site** (`src/run/version_gate.rs:133-140`): `obs_event!(INFO, ProcessStart,
  subject="version-probe")` before `run_bounded`, then a `process-exit` with `child_exit_status` and `duration_ms`.
  `run_bounded` itself stays unlogged.
- **First-word fake-agent mode** (`src/bin/viola-fake-agent.rs:917-923`): `statusline-echo` is dispatched before
  `Opts::parse`. An `agents` first word fits the same slot.
- **Ordered preflight with a generic chain** (`src/cmd/revive.rs:126-139`): each reading is a closure, and the first
  refusal ends the chain. A fourth reading means changing the chain's arity and its two unit tests.
- **Per-class fixture walker** (`tests/contract_fixture_hygiene.rs:24-60`): a recorded non-hook class gets its own name
  prefix, schema and walker (the `StatusLine.` precedent).

## Conventions to follow
- **Claude shapes only in `viola-agent-claude`, which stays pure** (`crates/viola-agent-claude/src/ledger.rs:1-5`): the
  row type, the parse, the status mapping and the join predicate go there. The spawn goes in the root bin, beside
  `run_bounded`. `viola-state`'s `board` takes no agent type (no manifest edge).
- **Every count carries its rule**: `LedgerRow::ALL.len()`, `LedgerRow::relied()` per OS, the fixture count by `ls … |
  wc -l`, the nextest kill line by its `binary(…)` names.
- **Closed values land in schema and catalog together** (`schemas/diag-line.v1.json`, obs-plan §6).

## New files to create
- `crates/viola-agent-claude/src/agents.rs` — the `claude agents --json` reading: a tolerant row type over the measured 7 keys, the bounded parse to rows or a closed failure, the `status` → idle/busy mapping, the join predicate
- `crates/viola-agent-claude/proptest-regressions/agents.txt` — the parser property's committed regressions file
- `src/run/agents.rs` — the bounded `claude agents --json` read: resolution, the spawn under the strip plan, the over-the-cap flag, the `agents-probe` spawn pair at its callers
- `fuzz/fuzz_targets/claude_agents_json.rs` — the parser's fuzz target
- `fuzz/corpus/claude_agents_json/` — its synthetic seeds
- `schemas/claude-agents-fixture.v1.json` — the recorded agents output class's schema
- `fixtures/claude/2.1.287/Agents.*` — the recorded and scrubbed `claude agents --json` output (name per the plan)
- `tests/cross_list_rows.rs` — the `list` row-integrity suite (test-plan §6)

## Files to modify
- `crates/viola-agent-claude/src/lib.rs` — `mod agents` and its re-exports
- `crates/viola-agent-claude/src/ledger.rs` — the join-field row: variant, `ALL`, `id`, `words`, `relied_on`, its post-condition check, the probe fields
- `crates/viola-state/src/board.rs` — `SessionStatus` idle / busy, the unwrapped item, the join's inputs (`child_pid`, the session id) carried out of `read`
- `crates/viola-state/src/snapshot.rs` — `agent_session_id` per architecture §Standard Contracts (Instance snapshot), if P4 keeps the session-id key
- `src/run/send/slot.rs` — the snapshot's `agent_session_id` rewritten on each `session-start` the tap takes, same condition
- `src/run/mod.rs` — `mod agents`
- `src/run/version_gate.rs` — `run_bounded`'s drain gains the over-the-cap reading, or the agents read gets its own drain
- `src/cmd/list.rs` — the agents read, the join, the unwrapped items, the `--json` items
- `src/human/board.rs` — STATUS words, the separator, the unwrapped rows with `n/a`, NAME through `escape_row`
- `src/cmd/revive.rs` — the `session-live` reading, its refusal pair and detail, `--fork` skipping it
- `src/cmd/verify.rs` — the join-field probe's spawn pair and its capture
- `src/cmd/verify/typed.rs` — the probe's read while Run B's session is live (before and after its `/clear`)
- `src/bin/viola-fake-agent.rs` — the `agents --json` mode: recorded, oversize and malformed, and the live-session rows
- `schemas/diag-line.v1.json` — the 13th `process-exit` `detail`
- `fuzz/Cargo.toml` — the new `[[bin]]`
- `fuzz/Cargo.lock` — the lockfile follows the manifest
- `.gitattributes` — the new corpus byte-exact
- `.config/nextest.toml` — the kill line gains `binary(cross_list_rows)` if the suite boots a verify-stamped wrapper
- `tests/cli_list.rs` — the unwrapped rack and STATUS literals
- `tests/cli_revive.rs` — the `session-live` refusal case and the `--fork` pass
- `tests/cli_verify.rs` — the `[NN/MM]` and summary literals if the row is relied
- `tests/contract_ledger_probes.rs` — the row id and stamped lists
- `tests/contract_fixture_hygiene.rs` — the `Agents.` class walker
- `tests/contract_fake_agent_drift.rs` — the recorded mode byte-equal to its recording
- `tests/support/fake.rs` — the fake agent's agents mode wiring for the root tests

## Sweeps
- `agents --json|agents_json|FAKE_CLAUDE_AGENTS|escape_row|session-live|SessionLive|UNWRAPPED` over `*.rs *.md
  *.toml *.json` of the repository (re-derived: `grep -rln … | grep -v '^./target\|^./.andromeda/runs\|mutants.out' |
  grep -v 2026-10-16-unwrapped-sessions-on-the-board | wc -l`): 84 files hit · 0 changed at P3 · 84 no-change
  (planning documents, chunk records, the masters and their rules, `src/human/escape.rs` and `src/human/board.rs`,
  both on the modify list above).
- `agents-mode|agents_mode|AGENTS_MODE` over `crates/viola-e2e/src scripts src tests`: 0 hits — the harness has no
  agents mode.
- `agent_session_id` over `src crates` (`*.rs`): no hit in `crates/viola-state/src/snapshot.rs`. The replay
  (`replay.rs:22,108`), the send slot (in memory) and the hook mapping hold it; the snapshot does not.

## Open questions
- Join keys: `child_pid` alone (measured exposed, survives `/clear` because the pid does not change, and needs no
  snapshot change), or `child_pid` plus a snapshot `agent_session_id` as architecture §Standard Contracts states
  (needs a write-side change in the run pump). → blocks: plan-decision
- The unwrapped NAME cell and `--json` `name`: the row's `sessionId` (an id, the architecture's "session identifier",
  and the design sample's hex form) or its `name` (the CLI's free-text title, user content; test-plan §6's
  `cross_list_rows` stub gives names holding `\n` / `\t`). → blocks: plan-decision
- The fake agent's mode channel (`FAKE_CLAUDE_AGENTS_MODE`, test-plan §7, against architecture's "argv options, no
  env"), and how its `agents` mode learns the live fake sessions' pids for a joined row. → blocks: plan-decision
