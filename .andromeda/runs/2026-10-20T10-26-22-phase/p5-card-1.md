# P5 review — 2026-10-20-epoch-5-cleanup-ii · "Epoch 5 cleanup II"

**Plan:** `viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup-ii/plan.md`, with `scope.md` and `research.md` beside it.
**Authority:** the founder's own answers at P4 (inputs#I2): «Один старт», «2.1.302», «Один чанк». The freight is the
overseer's direction as his delegate (inputs#I1), PROVISIONAL until his word.

## Goal
This chunk closes the three Epoch 5 boundary debts the founder split off, so that the next boundary's audit reads each
closed. It runs in his order: (5), then (4), then (7). No mutation run, witness run or workflow dispatch is made.

## What it does
- **(5) the sizes**, as pure moves:
  - `ledger.rs` keeps only `#[cfg(test)] mod tests;`, its module moving to `ledger/tests.rs`. The relied-count
    `#[cfg(windows)]` statement is folded into one `cfg!(unix)` expression, because the Windows scope contract would
    otherwise read the moved file as Windows-gated product code.
  - `src/cmd/verify/typed.rs` (1212 code lines) and `src/human/board.rs` (949) are split by concern into module
    directories, every new file under 800 lines, every caller's path and every test's name unchanged.
  - `viola-mcp` → `serde` is measured: dropped when `cargo check -p viola-mcp --all-targets` stays green, otherwise kept
    with its reading. No machete `ignored` entry is written.
- **(4) the 16 survivors**, each closed in the package that owns the file and shown red by a hand-applied mutant over a
  scratch clone:
  - The two const-gated sites are restated by the Epoch 4 precedent, never recorded "equivalent" (test-plan.md:1095):
    `prepare` takes `host` from its one caller, and `relied_here` goes, `relied()` reading `relied_on(WRAPPED_HERE)`.
  - `Drop`'s flag read and two decisions, and `read_list`'s capped parse, become plain functions with labelled cases.
  - `get_info` / `list_tools` are killed through a `Server` served over an in-memory duplex, and `serve` through a
    re-executed test process with stdin closed (the `viola-pty` child-entry precedent).
  - `live_endpoint` gets a live and a stale case built through the writers, and `probe_document` a whole-document literal
    case.
- **(7) the recorded exchange:**
  - A test-only `viola-mcp-tap` `[[bin]]` in `viola-e2e`.
  - A new `Mcp.` fixture class and `schemas/claude-mcp-fixture.v1.json`, walked by the hygiene contract with planted
    cases.
  - An E2E case, `mcp_recorded_exchange`, that replays the recorded client frames and holds viola's `tools/list` result to
    the recorded, CLI-accepted result's key and type shape, with two planted negatives (`ttlMs` removed, `cacheScope`
    removed).
  - **One live start** on 2.1.302 (`claude -p --strict-mcp-config --mcp-config <tap config>`, a synthetic prompt),
    ledgered before it runs, fired only after the tap, the class and the replay are green on a planted capture. A failed
    start stops and asks again.

## Top acceptance criteria
- All 16 survivors end `killed` or `restated` in `evidence/survivors.ndjson`, each with its red and green readings. None
  ends "equivalent" or "recorded", and the nine new cases read `9 tests run: 9 passed`.
- No file of the two splits stands over 800 code lines (tokei), `ledger.rs` holds no inline test module, the orphans and
  Windows scope contracts pass, and the board and verify output is byte-identical (the `cli_list`, `cli_verify`,
  `cli_output_plain`, `cli_plugin_install` and `cross_list_rows` binaries pass unchanged).
- `cargo machete crates/viola-mcp` is clean, with no `ignored` entry. If `serde` turns out to be needed, this is recorded
  unmet with its reading.
- `fixtures/claude/2.1.302/Mcp.tools-list-1.json` exists, made by the one start, scrubbed and hygiene-clean, and
  `mcp_recorded_exchange` passes with both planted negatives reading red.
- `pre-push` is green and the final HEAD's CI is green.

## Touchpoints
- **New:** `ledger/tests.rs`, `src/cmd/verify/typed/`, `src/human/board/`, `viola-mcp-tap.rs`,
  `mcp_recorded_exchange.rs`, `claude-mcp-fixture.v1.json`, the `Mcp.` fixture.
- **Modified:** `ledger.rs`, `statusline.rs`, `liveness.rs`, `scratch.rs`, `mutants.rs`, `session.rs`, `status.rs`,
  `viola-mcp/{lib,tools}.rs`, both manifests, `Cargo.lock`, the two module roots, `contract_fixture_hygiene.rs`,
  `contract_fake_agent_drift.rs`, and `.config/nextest.toml` only if needed.

## Test commands (30 entries)
- The lints: fmt, clippy, the Windows-target clippy, G1, lint probes, the orphans gate and the rmcp feature check.
- `run --unit`, then the nine kill cases by name.
- The size and census reads: tokei, the inline module, the refuse census, `relied_here` gone.
- The integration and E2E binaries, then the replay case.
- `cargo deny check`, then `cargo machete crates/viola-mcp`.
- The preservation guard.
- The smoke boot / status / cleanup, then G2 / G4, then `pre-push`.
- The operator trio: hygiene, push, `ci.py`.
- Every new entry was baseline-run on the untouched tree (HEAD 844de976). Each red is the chunk's own subject, with a
  must-pass or known-positive control over a scratch clone (`.andromeda/runs/2026-10-20T10-26-22-phase/p5-controls.md`),
  except `pre-push` below.

## Read before the word
- **`pre-push` read red at baseline on the host, not on the tree** (`.andromeda/runs/2026-10-20T10-26-22-phase/p5-prepush-red.md`).
  78 cases were refused by the last chunk's headroom keeper (`8939 MiB free, below the 9216 MiB floor`) while the coverage
  run filled the `/tmp` tmpfs from 13G free. That tmpfs holds 16G of Claude Code's own `$TMPDIR/claude-1000` and 1.9G of
  earlier chunks' kept live homes. Step 1 now stops below 14 GiB free.
- **Size:** the plan runs 630 lines, about 430 of prose outside the fence, against the template's ~400. That is within
  the warning band.
- **CI for the last wrap's flip** (844de976, ci#39407580327) was `in progress` at Setup and has not been read since. The
  operator pass reads this chunk's own push.
- **No capability is claimed:** none of the 24 unclaimed caps is a surface this chunk builds.
- **The boundary widening:** item (7)'s start, the CLI's MCP client through the test-only tap with the scrubbed frames
  committed as a fixture, is the founder's own answer to the P4 fork (inputs#I2 Q1). The wrap halts on it as one answer
  card under the playbook's "Boundary widening the founder answered himself to a P4 fork".

## Beyond the plan
| # | proposal | owner |
|---|---|---|
| 1 | Free the `/tmp` tmpfs to at least 14 GiB before `/andromeda-implement`'s `pre-push`, for example by removing old `$TMPDIR/claude-1000` session dirs or earlier chunks' kept `viola-live-*` homes under `$TMPDIR/viola-e2e-home-1000`. Implement stops at step 1 below that. | the founder, at his desk (nothing outside the work tree is removed by this session) |
