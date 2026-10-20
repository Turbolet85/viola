# Scope — Epoch 5 cleanup II (`2026-10-20-epoch-5-cleanup-ii`)

**Working entry:** "Epoch 5 cleanup II — the three Epoch 5 boundary debts the founder's split moved out of "Epoch 5
cleanup", each closed so the next boundary's audit reads it closed" (`viola-0.1.0/working-route.md`, Epoch 6 — Security
hardening, directly after `[2026-10-20-epoch-5-cleanup]`).

**What it builds:** three closures of debts the Epoch 5 boundary named, and nothing else. Item (4) closes the audit's 16
Informational mutation survivors, item (5) closes its two size proposals, and item (7) adds the recorded-exchange contract
case the evolve diagnosis named. Each one is closed so that the NEXT boundary's audit reads it closed. That audit is the
reader, so a closure is worded by what that audit measures.

## Provenance and authority
- The placement is the founder's own word «Два чанка», at the P4 split fork of chunk 2026-10-20-epoch-5-cleanup
  (`viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup/inputs/I2-relay-1.md.txt`). It places this entry directly after that chunk,
  carrying items (4), (5) and (7).
- The freight is the Viola overseer's direction, given as the founder's delegate, and it stays **PROVISIONAL until his word**
  (`viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup/inputs/I1-phase-cleanup-directive.md.txt`).
- This chunk's phase directive is the overseer's (inputs#I1).
- The directive's item 1 is the founder's standing direction: **no mutation run, no witness run and no workflow dispatch
  inside this chunk** (inputs#I1; verified against the directive's own text, and against `.claude/rules/testing.md`
  2026-10-04: "A chunk's `[[gate]]` block holds no mutation entry"). No `cargo mutants` run, `witness` gate or
  `gh workflow run` appears in the plan's gates.

## Item (4) — the 16 survivors, each killed or recorded
Source: `.andromeda/runs/2026-10-19T21-08-30-code-audit/proposals.md` §Informational "Survivors, complete" (16 = Σ
counts.missed). Each row below ends either **killed** or **recorded**:
- **Killed:** a test in the place the audit's run reaches fails under that mutant. The kill is shown by a hand-applied
  mutant (inputs#I1).
- **Recorded:** a written reading of why no test can or should kill it, for example an equivalent mutant or a harness
  teardown that has no observable output.

| # | unit | site | mutation |
|---|---|---|---|
| 1 | viola-agent-claude | `crates/viola-agent-claude/src/ledger.rs:143:9` | `replace LedgerRow::relied_here -> bool with true` |
| 2 | viola-agent-claude | `crates/viola-agent-claude/src/statusline.rs:118:5` | `replace probe_document -> Option<String> with Some("xyzzy".into())` |
| 3 | viola-agent-claude | `crates/viola-agent-claude/src/statusline.rs:118:5` | `replace probe_document -> Option<String> with Some(String::new())` |
| 4 | viola-e2e | `crates/viola-e2e/src/harness/run/mutants/scratch.rs:48:5` | `replace prepare -> Result<Option<(PathBuf, u64)>, String> with Ok(None)` |
| 5 | viola-e2e | `crates/viola-e2e/src/harness/run/mutants/scratch.rs:54:8` | `delete ! in prepare` |
| 6 | viola-e2e | `crates/viola-e2e/src/harness/session.rs:112:12` | `delete ! in <impl Drop for HarnessSession>::drop` |
| 7 | viola-e2e | `crates/viola-e2e/src/harness/session.rs:112:9` | `replace <impl Drop for HarnessSession>::drop with ()` |
| 8 | viola-e2e | `crates/viola-e2e/src/harness/session.rs:115:69` | `replace == with != in <impl Drop for HarnessSession>::drop` |
| 9 | viola-e2e | `crates/viola-e2e/src/harness/session.rs:121:12` | `delete ! in <impl Drop for HarnessSession>::drop` |
| 10 | viola-e2e | `crates/viola-e2e/src/harness/status.rs:71:36` | `replace + with * in read_list` |
| 11 | viola-e2e | `crates/viola-e2e/src/harness/status.rs:71:36` | `replace + with - in read_list` |
| 12 | viola-mcp | `crates/viola-mcp/src/lib.rs:66:5` | `replace serve -> Result<(), ServeError> with Ok(())` |
| 13 | viola-mcp | `crates/viola-mcp/src/tools.rs:196:9` | `replace <impl ServerHandler for Server>::get_info -> ServerConfig with Default::default()` |
| 14 | viola-mcp | `crates/viola-mcp/src/tools.rs:205:9` | `replace <impl ServerHandler for Server>::list_tools -> Result<ListToolsResult, ErrorData> with Ok(Default::default())` |
| 15 | viola-state | `crates/viola-state/src/liveness.rs:75:5` | `replace live_endpoint -> Option<String> with None` |
| 16 | viola-state | `crates/viola-state/src/liveness.rs:77:47` | `replace != with == in live_endpoint` |

- [premise-corrected: the sites shift by at most one line at HEAD 844de976 — `relied_here` 142, `probe_document` 117,
  `prepare` 47 (its `!` at 48), `live_endpoint` 74 (its `!=` at 76), `get_info` 195, `list_tools` 200, `serve` 61; `Drop`
  110–124 and `read_list`'s `take` at 71 are unchanged] The sites are the audit's coordinates at HEAD b6ddc19856. Each was
  re-read at HEAD 844de976, and those lines are what the kills aim at.
- [premise-corrected: no `viola-mcp` package test reaches `serve`, `get_info` or `list_tools` (a grep over
  `crates/viola-mcp/` finds only their definitions). Whether `mcp_tools.rs` would kill them is left unmeasured, and it
  does not matter: the audit's package run never executes another package's tests (test-plan §3 `run` step 4)]
  **The freight's hypothesis, kept verbatim:** "the three `viola-mcp` ones are killed only by
  `crates/viola-e2e/tests/mcp_tools.rs`, outside the package run — not measured". Its half that matters stands: a kill that
  the next audit reads sits inside `viola-mcp`'s own package tests.
- The audit runs per unit with the nextest `mutants` profile, and each mutant meets only the tests of the package that owns
  its file (test-plan §3 `run` step 4 "Test scope"; proposals.md §Informational). A kill therefore lives in that package's
  own tests.
- **Witness form** (inputs#I1, item 1; `.claude/rules/testing.md` 2026-09-25, extended 2026-10-20): each kill is shown by a
  hand-applied mutant, as a remove-the-guard reading. Where the classifier refuses the in-tree edit, it goes over a scratch
  clone of HEAD (`--project-root {clone}`). The case reads red by its own panic line with the mutant applied, and green once
  it is reverted. It is never shown by a `cargo mutants` run.
- **A const-gated site is restated, never recorded as "equivalent"** (verified at P3: `.github/workflows/windows-mutants.yml`
  holds no `ledger.rs` line; `HOST_SCRATCH` and `WRAPPED_HERE` are consts, `scratch.rs:11`, `statusline.rs:21`) (test-plan.md:1095: "A mutant behind a const
  … is recorded 'not measured here; owed to {the route entry that measures it}' by coordinate, never 'equivalent'").
  `prepare` (`HOST_SCRATCH`) and `relied_here` (`WRAPPED_HERE`) are const-gated on Linux. The Epoch 4 precedent passes the
  const in from the caller (a behaviour-preserving restatement), which makes the site reachable and killable on Linux.
  `ledger.rs` is on no Windows matrix line, so a "not measured here" record for `relied_here` would be owed to no leg.

## Item (5) — the sizes
Source: the same proposals.md, §Proposals M1 and M2.
- `crates/viola-agent-claude/src/ledger.rs` (4275 lines at d35644a5, with its in-file test module starting at line 1518;
  4275 raw lines and `#[cfg(test)]` at line 1518 read at HEAD 844de976) must stand without that module. M1 proposes
  `ledger/tests.rs`.
- `src/cmd/verify/typed.rs` (1428 raw lines at HEAD) and `src/human/board.rs` (1065 raw lines at HEAD) must each stand split
  by concern. M2 proposes a module directory, in the same way `src/run/send.rs` was split into `src/run/send/*`.
- `viola-mcp` → `serde` must not be an unused-dependency candidate (`crates/viola-mcp/Cargo.toml:21` `serde.workspace =
  true`).
- [premise-corrected: tokei 14.0.0 at HEAD reads `ledger.rs` 3773 code lines, 1214 of them above its test module, so
  `ledger.rs` without the module still stands over 800; `typed.rs` reads 1212 and `board.rs` 949] The audit measures sizes
  in tokei code lines (M1/M2 evidence), and the closures are read in those units. Moving `ledger.rs`'s module out takes
  `ledger.rs` off `sizes.file_max`, but not off `sizes.over_800`. Splitting `typed.rs` and `board.rs` so that each new file
  stands under 800 takes both off `over_800`.
- [premise-corrected: the reading is cargo-machete's ("`viola-mcp` → `serde` (cargo-machete; `serde` derives may arrive
  through `schemars` / `rmcp` re-exports — a candidate, not a verdict)", proposals.md:251). No `serde::` path and no serde
  derive appear in `crates/viola-mcp/src/`; schemars' `JsonSchema` derive is the only derive] Whether `serde` is used is
  measured at implement: drop the edge and run `cargo check -p viola-mcp --all-targets`. A green check removes the edge, a
  removal that contradicts architecture §Infrastructure Patterns → Crate dependency direction, which is a wrap amendment. A
  red check keeps it, with the reading. A machete `ignored` entry is not chosen.
- [premise-corrected: `cargo-modules` 0.27.0 at HEAD does not report `src/run/send/test_support.rs`, an out-of-line
  `#[cfg(test)] mod`, as an orphan (`cargo modules orphans -p viola --bin viola --deny --target x86_64-unknown-linux-gnu`
  names only the `cfg(windows)` `conpty`), so `.claude/rules/testing.md`'s 2026-09-25 rule "moving tests out of a file
  never shrinks it" is stale, which is wrap drift] The splits are pure moves. They change no behaviour, no public path
  and no test name or count, and the same tests pass before and after.
- **Trap** (`tests/contract_windows_mutation_scope.rs:59-66`): a moved test-only file has no column-0 `#[cfg(test)]` line,
  so the contract reads all of it as product code. `ledger.rs`'s relied-count assert (`#[cfg(unix)]` / `#[cfg(windows)]`,
  lines 1606–1609) is folded into one `cfg!(unix)` expression as it moves. Otherwise `ledger/tests.rs` reads as a
  Windows-gated file that no matrix line holds.

## Item (7) — a recorded live `tools/list` exchange replayed by an E2E contract case
- An E2E contract case replays a recorded live `tools/list` exchange, so that the gate sees the class only a live start saw
  (`ttlMs` / `cacheScope`). The source is the evolve diagnosis, `.andromeda/runs/2026-10-19T20-57-02-evolve-diagnose/proposals.md:207`:
  "the E2E rmcp client accepts a tools/list result without ttlMs/cacheScope, so every gate read green while claude 2.1.287
  (protocol 2026-07-28) dropped every tool; only the one live start could see it".
- No recording of it exists, so the case needs a new live `claude` start.
- **The live starts are the founder's cap for this chunk** (inputs#I1, item 2; security-plan history
  2026-10-19-newer-claude-cli-rows: "a subprocess start of the user's CLI whose output reaches the repository is a
  boundary widening the founder answers himself"). The cap is asked at P4 as a dialog fork with the starts counted, because
  he is at his keyboard. **No live start runs before his word.**
- [premise-corrected: `viola verify --record` has no MCP leg, so the recording is a test-side capture. It lives under
  `fixtures/claude/<cli-version>/` as a class of its own, `Mcp.*.json`, with its own schema, as the `Agents` class did at
  chunk 2026-10-16-unwrapped-sessions-on-the-board] A recorded exchange committed as a fixture passes the NEVER-log floor
  and an equal scrub and refusal (`home-path` · `absolute-path` · `username` · `email`): no R8-stripped `CLAUDE*` value, home
  path, username or email. `tests/contract_fixture_hygiene.rs` walks it against its schema. The replay compares the recorded,
  CLI-accepted `tools/list` result with viola's by key and type shape, which is the Agents class's form
  (`fake_agent_agents_replay_matches_every_recorded_agents_set`). The class the live start saw is a key the CLI requires and
  rmcp's client does not.

## Sizing
- Items (4), (5) and (7) are taken as one chunk. If they do not fit one window, the split is proposed at P4 (inputs#I1,
  item 3, as the directive states it). Whether they fit is P4's fork.

## Host
- Read `df -h /tmp` before any gate that boots stamped homes. The headroom floor is 9 GiB (`HEADROOM_FLOOR_MIB`, the
  keepers of chunk 2026-10-20-epoch-5-cleanup; inputs#I1, item 4). `/tmp` read 13 G free of 32 G at this session's start
  (`df -h /tmp`, 2026-10-20T10:25Z).

## Watch
- watch: the macOS readiness stall, `wrapper <name> not ready`, where the wrapper wrote `process-start{subject:"self"}` and
  nothing more, inside `run.start` before the version gate, its phase not named. Its witness is the stall report that
  `tests/support/home.rs` `wait_ready` writes into the panic. It was closed UNMET on the founder's own word «Закрыть +
  WATCH» (`viola-0.1.0/chunks/2026-10-19-newer-claude-cli-rows/inputs/I9-relay-1.md.txt`). It retires on a recurrence or on
  3 green runs (1/3; since 2026-10-19-newer-claude-cli-rows).

## CI read at Setup 5a
- `844de97682c8` (the last wrap's flip, HEAD): **verdict not yet available**. ci#39407580327 (push) was `in progress` with
  15 of 15 checks, 8 of them running, the oldest `lint (ubuntu-latest)` at 186 s. Its wall-clock is not known yet. It is
  not read as green, and nothing is folded from it.

## Boundaries
- No new product behaviour. Item (7)'s case is test-side, and item (5)'s moves are behaviour-neutral.
- No other route entry's freight is taken.
- No spec-source edit. Drift goes to the wrap.
