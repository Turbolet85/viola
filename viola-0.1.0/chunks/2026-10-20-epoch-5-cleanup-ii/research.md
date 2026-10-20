# Codebase Research — 2026-10-20-epoch-5-cleanup-ii

## Scope
- **Depth:** deep · **Reads:** 24 · **Globs/Greps:** 19
- **Harness rules consulted:** `.claude/rules/verification-harness.md` (read in full, 17 Session Additions; applied: the
  `test(/(^|::)tests::name(::|$)/)` filter form for each hand-applied mutant's red, `run --mutants` keeps no home
  (`AGENT_RUN_KEEP_HOMES=0`), the stamped-CLI `PATH` rule for a live rig) · `.claude/rules/testing.md` (read in full,
  54 Session Additions; applied: the remove-the-guard pair over a scratch clone of HEAD (`--project-root {clone}`) when
  the classifier refuses the in-tree edit, oracles as literals and never the product's own constant, no
  `std::env::set_var`, the mutants profile's 10 s default kill, a planted negative for an absence check)
- **Platform issues consulted:** none. No bullet is runner-only (Setup 5a read ci#39407580327 `in progress`, and nothing
  was folded from it).
- **External inputs:** `inputs#I1`, the overseer's phase directive: no mutation, witness or dispatch run in the chunk;
  item (4) shown by hand-applied mutants over a scratch clone; item (7)'s live starts are the founder's cap, asked at P4
  as a dialog with the starts counted; split at P4 if the items do not fit one window; `df -h /tmp` before stamped-home
  gates against the 9 GiB floor.

## Files inspected
- `.andromeda/runs/2026-10-19T21-08-30-code-audit/proposals.md` (lines 1–100, 251) — the 16 survivors (§Informational),
  M1 and M2, and the machete reading `viola-mcp` → `serde` ("a candidate, not a verdict").
- `crates/viola-agent-claude/src/ledger.rs` (130–160, 1600–1612; item map) — `relied_here` at 142 is `relied_on(WRAPPED_HERE)`
  and its only caller is `relied()` at 158. The test module starts at the column-0 `#[cfg(test)]`, line 1518. Its relied-count
  assert at 1606–1609 is two `#[cfg(unix)]` / `#[cfg(windows)]` statements.
- `crates/viola-agent-claude/src/statusline.rs` (100–135, 435–450) — `probe_document` at 117 is `probe_on(…, WRAPPED_HERE)`.
  The one test that calls it asserts only `.is_some() == cfg!(unix)`.
- `crates/viola-state/src/liveness.rs` (60–95, 140–200) — `live_endpoint` at 74. Its one test reads a dir with no snapshot.
  `touch_heartbeat` (`heartbeat.rs:16`), `write_snapshot` (`snapshot.rs:91`) and `own_start` (`liveness.rs:58`) are the writers
  a live case is built from.
- `crates/viola-e2e/src/harness/run/mutants/scratch.rs` (30–200) — `prepare(root)` at 47 returns `Ok(None)` when
  `!HOST_SCRATCH`, where `HOST_SCRATCH = cfg!(windows)` is a const at line 11. Its test
  `prepare_empties_the_scratch_and_reports_what_it_held` branches on `HOST_SCRATCH`.
- `crates/viola-e2e/src/harness/run/mutants.rs` (215–230) — `mutate()` calls `scratch::prepare(&ws.root)` at 223.
- `crates/viola-e2e/src/harness/session.rs` (1–160) — `Drop` at 110–124 does three things: it runs `cleanup` when
  `!self.cleaned`, reads `AGENT_RUN_KEEP_HOMES` / `AGENT_RUN_KEEP_FAILED` through a closure `v == "1"`, and removes
  the home's parent when `!keep`. The two existing tests cover `keep_home` and the options default only.
- `crates/viola-e2e/src/harness/status.rs` (55–80) — `read_list` reads stdout through `take(MAX_FRAME + 1)` at 71, then
  `within = len <= MAX_FRAME`, then parses.
- `crates/viola-mcp/src/lib.rs` (50–80) — `serve` at 61 builds a current-thread runtime and serves `rmcp::transport::stdio()`.
  stdin EOF before `initialize` is `Err(ServeError::Initialize)`.
- `crates/viola-mcp/src/tools.rs` (1–125, 155–215, 416–480) — `get_info` (195) and `list_tools` (200) are the
  `ServerHandler` impls. The unit tests reach `tools_result()` directly (`tools_result_carries_its_ttl_and_cache_scope`) and
  never reach the handler. The only derive besides thiserror is schemars' `JsonSchema`.
- `crates/viola-mcp/Cargo.toml` (full) — `serde.workspace = true` at 21. Dev-deps are rstest and tempfile. tokio is `rt`
  only, and rmcp `server` enables `transport-async-rw` (`tokio/io-util`) (`rmcp-3.4.1/Cargo.toml:119,138`).
- `crates/viola-e2e/tests/mcp_tools.rs` (75–135, 280–300) — the rmcp-client E2E asserts `ttl_ms == Some(0)` and
  `cache_scope == Private` against viola's own result. Its client is rmcp's, which accepts a result without them.
- `crates/viola-e2e/Cargo.toml` (full) — one `[[bin]]`, `viola-harness`. rmcp is `client`, tokio is `rt` + `process`, and
  there are no dev-deps.
- `viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/evidence/live-mcp.md` (1–60) — the live rig, its ledger
  (`live-ledger.py`, one row per start), the stamped-CLI `PATH`, and the CLI's `mcp-logs-plugin-viola-viola/` log that names
  the `ttlMs` / `cacheScope` issues. That log was read and never copied.
- `tests/contract_windows_mutation_scope.rs` (51–153) — `product_lines` keeps a file's lines BEFORE its first column-0
  `#[cfg(test)]`. A file with none is read whole, and any `cfg(windows` / `cfg!(windows)` token there marks it Windows-gated.
- `tests/contract_fixture_hygiene.rs` (1–60) — one schema per class (`StatusLine.` / `Agents.` / `Screen.` named, the rest
  `claude-fixture.v1.json`).
- `scripts/orphans-check.sh` (1–80) — `cargo modules orphans --deny` per target, an orphan only when every triple reports it.
- `src/run/send.rs` (40–46) — `#[cfg(test)] mod test_support;`, an out-of-line test-only module (`src/run/send/test_support.rs`).
- `src/cmd/verify/typed.rs` (item map) — 1428 raw / 1212 code lines. The product part runs from 1 to 894, with `Run` (726–894),
  the framing turns, the readiness waits and `SharedPty` / `Feed` / `Sink` the concerns. Its tests start at 895, and its one cfg
  token, `#[cfg(unix)]` at 1104, sits inside the tests.
- `src/human/board.rs` (item map) — 1065 raw / 949 code lines. The concerns are the refusal pairs (17–31), the BAY line
  (33–97), the cells and spans (99–224) and the rows and writer (226–312). Its tests start at 313. `src/human.rs:16` re-exports
  `BayBudget, home_state_pair, instance_state_pair, write_board`.
- `.github/workflows/windows-mutants.yml` (5–70) — no item names `typed.rs`, `board.rs` or `ledger.rs`. `viola-agent-claude`
  is `lib.rs` alone, and the `viola-e2e` item is `scratch.rs` + `cleanup.rs`.
- `.andromeda/test-plan.md:1095` — the Mutation gate. A mutant behind a const such as `HOST_SCRATCH = cfg!(windows)` is a
  record "not measured here; owed to {route entry}" by coordinate, never "equivalent". The Epoch 4 survivors were closed by
  behaviour-preserving restatements, each shown red under its hand-applied mutant.

## Graph impact (from the code-graph query; the trail `.andromeda/runs/2026-10-20T10-26-22-phase/tree-query-2026-10-20-epoch-5-cleanup-ii.json`)
- **relied_here** — 1 caller, `LedgerRow::relied()` at `crates/viola-agent-claude/src/ledger.rs:157`. A name grep over the
  tree finds the same two lines and nothing else. Inlining it as `relied_on(WRAPPED_HERE)` touches no other file.
- **prepare** (`scratch::prepare`) — 1 production caller, `mutate()` at `crates/viola-e2e/src/harness/run/mutants.rs:223`,
  plus 3 calls in its own test (`scratch.rs:163,172,179`). A `host: bool` parameter threads through those two files only.
- **live_endpoint** — 12 production calls across `src/cmd/{send,wait,last,answer,pause}.rs` and `crates/viola-mcp/src/tools.rs`,
  and 1 test call. Its signature does not change: the kill is a new case in `liveness.rs`'s own tests.
- **read_list** — 2 calls, `status()` (`status.rs:87`) and the test `status_derives_down_with_no_session_bin_to_run`
  (`status.rs:213`). A capped-parse helper split out of it changes no caller.
- **probe_document** — 2 calls, `ProbeDir::create` (`src/cmd/verify.rs:320`) and the one test (`statusline.rs:445`).
  Unchanged.
- **serve** (`viola_mcp::serve`) — 1 caller, `src/cmd/mcp.rs:35`. Every other `serve` row is `viola_channel`'s
  `Server::serve`, a different symbol. Unchanged.
- **get_info / list_tools** — 0 rows. These are trait methods rmcp calls at run time, and zero callers is no leaf evidence
  (cookbook §Reading the result). A grep over `crates/viola-mcp/` finds only their definitions.

## Patterns detected
- **Const-gated decision passed in by the caller** (test-plan.md:1095, `viola_state::fs`'s retry decision at chunk
  2026-10-13-epoch-4-survivors): a site that a const makes unreachable on Linux becomes reachable when the caller passes the
  const, with no behaviour change. This fits `prepare` (`HOST_SCRATCH`) and `relied_here` (`WRAPPED_HERE`).
- **Pure decision split out of an I/O body** (`ctrl_c_ending`, `ended_by`, `probe_until` in `typed.rs` at chunk
  2026-10-13-epoch-4-survivors-ii): the decision becomes a plain function with its own case table. This fits `Drop`'s flag
  read and `read_list`'s capped parse.
- **Module root plus children by concern** (`src/run/send.rs` + `src/run/send/*`, `src/human.rs` + `src/human/*`): the root
  keeps the public items and `pub(crate) use` re-exports them, so every caller keeps its path. Each child keeps its own
  inline `#[cfg(test)] mod tests` at its bottom (test-plan §4).
- **Recorded fixture class with its own schema** (`Agents.recorded-1.json` + `schemas/claude-agents-fixture.v1.json`,
  chunk 2026-10-16-unwrapped-sessions-on-the-board): scrubbed before the write, refused whole when unclean, hygiene-walked
  with planted cases, and compared by key-and-type shape (`fake_agent_agents_replay_matches_every_recorded_agents_set`).

## Conventions to follow
- **Test names**: `<subject>_<condition>_<expected>`. viola-e2e takes labelled case tables in one `#[test]` (no rstest), and
  viola-state and viola-agent-claude take rstest `#[case]` (test-plan §4; `.claude/rules/testing.md` Naming).
- **No env write in a test**: `std::env::set_var` is banned (edition 2024). A flag read is tested as a pure function of the
  read's result, never by setting the variable (`.claude/rules/testing.md` Determinism).
- **Hand-applied mutant**: each kill is read red with the mutant applied, by its own case's panic line under the
  `test(/(^|::)tests::name(::|$)/)` filter, and green once the mutant is reverted. Where the classifier refuses the in-tree
  edit, the mutant goes on a scratch clone of HEAD (`--project-root {clone}`) (`.claude/rules/testing.md` 2026-09-25 ext.
  2026-10-20; `.claude/rules/verification-harness.md` 2026-09-27).
- **Windows scope contract**: a moved test-only file must hold no `cfg(windows` / `cfg!(windows)` / `target_os = "windows"`
  token. Without a column-0 `#[cfg(test)]` the contract reads the whole file as product (`tests/contract_windows_mutation_scope.rs:59-66`).

## New files to create
- `crates/viola-agent-claude/src/ledger/tests.rs` — `ledger.rs`'s test module, moved whole
- `src/cmd/verify/typed/` — the children of `typed.rs` by concern, each with its own tests
- `src/human/board/` — the children of `board.rs` by concern, each with its own tests
- `crates/viola-e2e/tests/mcp_recorded_exchange.rs` — item (7)'s replay case over a `viola mcp` child
- `schemas/claude-mcp-fixture.v1.json` — the recorded exchange's class schema
- `fixtures/claude/2.1.302/Mcp.tools-list-1.json` — the recorded exchange, made by the one live start (inputs#I2)
- `crates/viola-e2e/src/bin/viola-mcp-tap.rs` — the test-only recording tap and its fixture-writing form

## Files to modify
- `crates/viola-agent-claude/src/ledger.rs` — `relied()` reads `relied_on(WRAPPED_HERE)` and `relied_here` goes; the test module is declared `#[cfg(test)] mod tests;`
- `crates/viola-agent-claude/src/statusline.rs` — the `probe_document` test asserts the whole document literal on a wrapped host
- `crates/viola-state/src/liveness.rs` — live and stale `live_endpoint` cases built through the writers
- `crates/viola-e2e/src/harness/run/mutants/scratch.rs` — `prepare(root, host)` and its cases on every host
- `crates/viola-e2e/src/harness/run/mutants.rs` — `mutate()` passes `HOST_SCRATCH`
- `crates/viola-e2e/src/harness/session.rs` — the flag read and the drop's two steps as plain functions with cases, and a constructed-session drop case
- `crates/viola-e2e/src/harness/status.rs` — the capped parse as a plain function with its two boundary cases
- `crates/viola-mcp/src/lib.rs` — a `serve` case over a re-executed test process with stdin closed
- `crates/viola-mcp/src/tools.rs` — `get_info` and `list_tools` cases through a served `Server` over an in-memory duplex
- `crates/viola-mcp/Cargo.toml` — the `serde` reading closed
- `Cargo.lock` — follows the manifest if a dependency edge goes
- `src/cmd/verify/typed.rs` — the module root, re-exporting its children
- `src/human/board.rs` — the module root, re-exporting its children
- `crates/viola-e2e/Cargo.toml` — the `viola-mcp-tap` `[[bin]]` and a `viola-agent-claude` edge for the record scrub
- `tests/contract_fixture_hygiene.rs` — the `Mcp.` class walked against its schema, with planted cases
- `tests/contract_fake_agent_drift.rs` — a set walker meeting a new class file, read and kept passing
- `.config/nextest.toml` — the replay binary beside `mcp_tools` on its kill lines, only if its wall needs that class

## Sweeps
- `relied_here` over the whole tree (`grep -rn --include=*.rs`): 2 hits · 2 changed · 0 no-change.
- `cfg(windows)\|cfg(unix)\|cfg(not` over `typed.rs`, `board.rs` and `ledger.rs`: 3 hits · 1 changed (`ledger.rs:1606-1609`,
  folded into one `cfg!(unix)` expression when the module moves) · 2 no-change (`typed.rs:1104` is `cfg(unix)` inside its
  tests, which the contract never reads as Windows, and `ledger.rs:1608` is the changed line's twin).
- `get_info\|list_tools\|viola_mcp::serve` over `crates/viola-mcp/`: 3 definition hits · 0 test hits. The freight's hypothesis
  holds: no package test reaches them.
- `fixtures/claude\|"fixtures"` over `tests/ crates/ src/` (`grep -rln`): 20 files. 2 are listed above (the hygiene walk, the
  drift contract). The other 18 read a named variant by event and are no-change: an `Mcp.` file names no hook event.

## Open questions
- Item (7)'s live starts: how many, on which stamped CLI version, and through which tap (a `viola-e2e` `[[bin]]` or an
  internal `viola-harness` arm, its command an absolute exec path in an `--mcp-config` file read by a print-mode
  `claude -p --strict-mcp-config` start) → blocks: plan-decision (the founder's cap, inputs#I1 item 2). Answered at P4
  by the founder himself (inputs#I2): one start, on 2.1.302. The tap is a `viola-e2e` `[[bin]]`, leaned at P4.
- Whether items (4), (5) and (7) fit one implement window → blocks: plan-decision (inputs#I1 item 3). Answered at P4 by
  the founder himself (inputs#I2): one chunk, (5) then (4) then (7).
- Whether `serde` is truly unused in `viola-mcp` (schemars' `JsonSchema` derive is the only derive) → blocks:
  implementation-scope (implement measures it: drop the edge, `cargo check -p viola-mcp --all-targets`; a failure keeps it,
  with the reading written to the plan's evidence).
