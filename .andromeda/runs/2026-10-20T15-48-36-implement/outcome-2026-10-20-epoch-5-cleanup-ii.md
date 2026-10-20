# Outcome — 2026-10-20-epoch-5-cleanup-ii (/andromeda-implement, run 2026-10-20T15-48-36)

**GREEN + SMOKE** — on the dialog-15 answers (`inputs#I5`, read 2026-10-20T18:24Z): Q1 the founder's own word «Ещё один,
промпт через stdin»; Q2 and Q3 the Viola overseer's technical answers. The block's one red, entry 20, is the plan's
wording that Q3's word covers. Next → the OPERATOR PASS: entries 28 (hygiene, as written), the pre-CI commit, 29 (the
push), 30 (`ci.py`). Then stop before the wrap (`inputs#I4`).

An earlier version of this file read STUCK: start 1 had yielded no recording, entry 7 rested on a falsified premise,
and entry 20 needed a word. It was rewritten whole after `inputs#I5`.

## The operator pass (run here on `inputs#I4`'s word; record `evidence/operator-pass.md`)
- Entry 28, hygiene: it first refused 2 phase-run P5 records (temp-dir paths). After they were respelled `$TMPDIR/…` it
  read clean, and it read clean again over the pass record.
- The pre-CI commit `2cdfb0dd`. Entry 29 pushed it: `origin/build/viola-0.1.0` 844de976→2cdfb0dd.
- Entry 30: `verdict: red` on ci#39477233397, 13/15 success.
  - `test (macos-latest)` and `release (macos-latest)` were cancelled, never given a hosted macOS arm64 runner (0 steps;
    "not acquired by Runner of type hosted"; capacity notices). Not this chunk's.
  - The macOS readings of the OS-sensitive cases are still owed. A re-run of the two jobs waits for the operator's word.
  - Stopped before the wrap.

## Files changed — `gate.py scope`: `clean — changed 30 · listed 29 · recorded 1 (companion 1)`
- **Item (5):**
  - `crates/viola-agent-claude/src/ledger.rs`: the test module out; `relied_here` gone.
  - `crates/viola-agent-claude/src/ledger/test_cases.rs` (recorded companion, `inputs#I5` Q2), in place of the listed
    `ledger/tests.rs`.
  - `src/cmd/verify/typed.rs` and `typed/{runs,turns,pty,run,test_support}.rs`.
  - `src/human/board.rs` and `board/{pairs,bay,cells,rows,test_support}.rs`.
  - `crates/viola-mcp/Cargo.toml` (the `serde` edge dropped) and `Cargo.lock`.
- **Item (4):** `statusline.rs`, `liveness.rs`, `scratch.rs` and `mutants.rs`, `session.rs`, `status.rs`, and
  viola-mcp's `lib.rs` and `tools.rs`.
- **Item (7):**
  - `crates/viola-e2e/src/bin/viola-mcp-tap.rs` and `crates/viola-e2e/Cargo.toml`;
  - `crates/viola-e2e/tests/mcp_recorded_exchange.rs`;
  - `schemas/claude-mcp-fixture.v1.json`;
  - `tests/contract_fixture_hygiene.rs`;
  - `fixtures/claude/2.1.302/Mcp.tools-list-1.json`.
- **Not changed:** `tests/contract_fake_agent_drift.rs` (green over the set that now holds the `Mcp.` file) and
  `.config/nextest.toml` (the replay case runs in under a second).

## Gate block — the last firing, trail `.andromeda/runs/2026-10-20T15-48-36-implement/gate-2026-10-20-epoch-5-cleanup-ii.json`
`entries 30 · green 25 · red 1 (20) · recorded 1 · timeout 0 · not-run 3`
- 1 recorded (`/tmp` 15G free).
- 2–8 green: fmt; clippy on the host and on the Windows target; G1; lint-probes; orphans 2/2 probes and every target
  clean; rmcp features.
- 9 green (the unit tier). 10 green: `9 tests run: 9 passed`.
- 11–14 green: no file of the two splits over 800 code lines; no inline ledger test module; the refuse census 14; no
  `relied_here`.
- 15 green: integration — verify, list, plain output, plugin install.
- 16 green: the E2E contracts. The Windows scope contract passes over the moved files; the hygiene walk passes with the
  `Mcp.` class; the drift and ledger-probe contracts pass; `cross_list_rows` and `mcp_tools` pass.
- 17 green: the replay case over the recorded fixture, its two planted negatives inside.
- 18 green (`cargo deny check`). 19 green (`cargo machete crates/viola-mcp`).
- 20 **red — the plan's wording** (`inputs#I5` Q3: "Yes, the plan's wording", the Viola overseer, 2026-10-20, relayed
  verbatim by the operator seat). The guard's line, copied whole from its log:
  `guard: allowed 19 — Cargo.lock, crates/viola-agent-claude/src/ledger.rs, crates/viola-agent-claude/src/statusline.rs,
  crates/viola-e2e/Cargo.toml, crates/viola-e2e/src/bin/viola-mcp-tap.rs, crates/viola-e2e/src/harness/run/mutants.rs,
  crates/viola-e2e/src/harness/run/mutants/scratch.rs, crates/viola-e2e/src/harness/session.rs,
  crates/viola-e2e/src/harness/status.rs, crates/viola-e2e/tests/mcp_recorded_exchange.rs, crates/viola-mcp/Cargo.toml,
  crates/viola-mcp/src/lib.rs, crates/viola-mcp/src/tools.rs, crates/viola-state/src/liveness.rs,
  fixtures/claude/2.1.302/Mcp.tools-list-1.json, schemas/claude-mcp-fixture.v1.json, src/cmd/verify/typed.rs,
  src/human/board.rs, tests/contract_fixture_hygiene.rs · companions excluded 1 —
  crates/viola-agent-claude/src/ledger/test_cases.rs (serves step 3) · changed 30 · excluded 0`, then
  `guard: outside 10 files` — the 5 `src/cmd/verify/typed/*.rs` and 5 `src/human/board/*.rs` files: research's two
  directory items, which the allow regex never matches whole.
- 21–26 green: the smoke — cleanup, boot, status, G2, G4, cleanup (processes and endpoint gone).
- 27 green: `pre-push`, stage `linux-tests`, `"ok":true`.
- 28–30 not run (operator legs).

## Evidence (`viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup-ii/evidence/`)
- `survivors.ndjson`: 16 lines, 10 `killed` and 6 `restated`. Each case read red by its own panic line under the
  `mutants` profile with the mutant applied by hand, and green once the mutant was reverted byte-exact. #1's panic line
  names `ledger/tests.rs:79`, the file's name at that reading; it is `ledger/test_cases.rs` since `inputs#I5` Q2.
- `sizes-before.md`, `sizes-after.md`:
  - after the moves: 114 cases moved module segment, 0 removed;
  - in the final tree: 0 removed in every package, 198 `ledger::tests::…` cases kept;
  - the census reads 2/2 `obs_event!` sites and 14/14 `human::refuse(` calls.
- `serde.md`: green.
- `live-tap.md`, `live-sessions.ndjson` (4 rows: `start` n 1 at 16:28:32Z, its census, `start` n 2 cap 2 at 18:31:58Z,
  its census), `live-tap.py`, `live-ledger.py` (`cmp` 0).
- Inputs: `inputs#I4` (the invocation directive) and `inputs#I5` (the dialog-15 answers).

## Deviations from the plan
- **Step 3:** the test module file is `ledger/test_cases.rs` under `#[cfg(test)] #[path = "ledger/test_cases.rs"] mod
  tests;`, which keeps every `ledger::tests::…` path (`inputs#I5` Q2; recorded in `scope-record.md`).
  - Measured on probe crates: cargo-modules 0.27 skips an out-of-line `#[cfg(test)]` module file only when its stem
    begins `test_`. So scope.md's premise ("an out-of-line cfg(test) module reads clean") held for `test_support` alone.
  - The moved tests held two more `cfg!(windows)` oracles than research's sweep found; they are restated as
    `!cfg!(unix)`.
- **Step 13:** the served-duplex case speaks `2026-07-28` and ends its client with `shutdown()`. Dropping one half of a
  `tokio::io::split` never signals EOF (two hung runs, killed).
- **Step 14:** the child marker is `MCP_SERVE_TEST_CHILD`, and the child entry `serve_child_entry` is one more listed
  case.
- **Step 15:**
  - **the class is the measured handshake.** claude 2.1.302 on `2026-07-28` sends `server/discover`, then `tools/list`,
    and no `initialize` or `notifications/initialized`;
  - the document holds the client frames up to and including the first `tools/list` request, verbatim, with each
    request's result keyed by its method; the schema, the planted cases and the replay follow it;
  - the tap's refusal reuses `ledger::screen_fault` per string, the one public check that covers all four codes.
- **Step 18:**
  - **two starts, not one.** Start 1, in the plan's argv, exited 1 before any server ran. HYPOTHESIS: `--mcp-config` is
    variadic in 2.1.302 and took the prompt. Its stderr was not recorded.
  - Start 2, on the founder's own word (`inputs#I5` Q1), gave the prompt on stdin and recorded the exchange.
  - Both starts ran the ledgered build: before start 2 the two `target/debug` names were relinked by `ln -f` to cargo's
    own artifacts, and their hashes were read equal to the ledger's.
- **Staging:** a synthetic capture's fixture was placed at the fixture path for one read before start 1, then removed.
- **Matrix:** this chunk claims no capability (`matrix.py show`: 0), so no ref was written.

## Process hygiene
| process | started by | final state |
|---|---|---|
| two hung `viola-mcp` test binaries (the served-duplex case) | this run | terminated (TaskStop; then a `timeout 300` bound) |
| `claude -p` start 1 | this run (the live round) | exited 1 by itself; no tap or server started |
| `claude -p` start 2, its `viola-mcp-tap`, its `viola mcp` | this run (the live round) | exited 0 by themselves (`mcp.ndjson` `process-exit` 0) |
| the smoke session `p-e5c2-smoke`, both block firings | a gate entry | terminated (entry 26: processes and endpoint gone) |
| any process from `projects/viola/target`, or any `viola-mcp-tap` | — | `gate.py census` (18:41Z): none running |
The host's other sessions (the overseers, the builders, another project's `cargo-mutants`) were not started by this run
and were left alone.

## Decisions & corrections
- `inputs#I4` (the operator seat, 15:48Z): one live start on 2.1.302, ledgered before it; no mutation run and no
  dispatch; then the operator pass, stopping before the wrap. Each start was ledgered before it ran. No mutation run and
  no dispatch were made.
- `inputs#I5` (dialog 15; it expired at 17:41:53Z and the founder dismissed it here; read by the overseer at 18:24Z):
  - Q1, the founder's own word: one more start, in the stdin form;
  - Q2, the overseer's: the `#[path]` form, `test_cases.rs`;
  - Q3, the overseer's: entry 20 is the plan's wording.
- The operator rejected my question dialog in this window and pointed to the relay file instead.
- Bash guard refusals: 4 (a `cd` out of the root ×2, a cat heredoc to a file, a pipeline tool's output into a filter).
  Each was re-issued once in the guarded form.
- A standing learning recurred: testing.md 2026-09-25 (an out-of-line `#[cfg(test)]` module file reads as an orphan)
  holds on 0.27.0 except for a `test_` stem. scope.md had read it as stale.
