# /implement outcome — 2026-10-17-mcp-server-for-drivers

> Superseded on its live criterion by the re-entry at step 15 (`.andromeda/runs/2026-10-17T14-14-49-implement/`): the
> second live start, on the founder's word (`inputs#I4`), read green, and `v1-38` is `implemented`.

**SURFACED.** The block is green (fifth firing: green 23 · red 0 · the three `operator` entries not run; trail
`.andromeda/runs/2026-10-17T11-33-11-implement/gate-2026-10-17-mcp-server-for-drivers.json`). The one live `claude`
2.1.287 start (the founder's cap) was made and read negative: Claude Code refused the server's `tools/list` result
on the negotiated protocol `2026-07-28` (no `ttlMs`, no `cacheScope`), so no tool reached the model. That is fixed
and pinned after the leg, but the start was not re-fired, so the plan's live criterion and `v1-38`'s acceptance
(a default `wait` returning between 50 and 60 s on the live CLI) stay unproven. `v1-38` keeps `planned` and no ref
(the acceptance is not met as written). A second live start needs the founder's word.

**The operator pass ran after this report** (`chunks/2026-10-17-mcp-server-for-drivers/evidence/operator-pass.md`):
pre-push and the Windows-target lint green on the final tree, `hygiene: clean`, pre-CI commit `a902e6e4`, pushed,
CI green 15/15 on it (`ci#39033242394`, attempt 1). Next → `/andromeda-wrap-session`, with the live criterion and
`v1-38` for the founder's word.

## Files changed

New (11): `crates/viola-mcp/{Cargo.toml, src/lib.rs, src/tools.rs, src/result.rs, tests/handlers.rs}`,
`crates/viola-channel/src/tokio_client.rs`, `src/cmd/mcp.rs`, `crates/viola-e2e/tests/mcp_tools.rs`,
`scripts/channel-tokio-free.sh`, `scripts/rmcp-features.sh`, and `evidence/` (the rig copies, the ledger,
`live-mcp.md`, `instrument-controls.md`, `unstamped-boot-race.md`).

Modified (research's list): `Cargo.toml`, `Cargo.lock`, `crates/viola-channel/{Cargo.toml, src/lib.rs}`,
`crates/viola-state/src/liveness.rs`, `src/cmd/{client.rs, list.rs, mod.rs}`, `src/main.rs`, `plugin/.mcp.json`,
`crates/viola-agent-claude/src/lib.rs`, `crates/viola-e2e/Cargo.toml`, `.github/workflows/ci.yml`,
`scripts/sync-crates.txt`, `scripts/deny-probes.sh`, `tests/contract_lints.rs`, `tests/cli_output_plain.rs`,
`.config/nextest.toml`.

The scope record (`chunks/2026-10-17-mcp-server-for-drivers/scope-record.md`, `gate.py scope` → `clean`):
- `crates/viola-channel/src/client.rs` · in-intent — the envelope, the conn counter and the SQOS open shared with
  the Tokio client (`request_frame`, `reply_from`, `next_conn`, `open_handle`).
- `crates/viola-channel/src/frame.rs` · in-intent — `framed`, the one line verdict both readers use.
- `crates/viola-e2e/src/harness/session.rs` · in-intent — `SessionOptions::unstamped` for Path 7.
- `.github/workflows/windows-mutants.yml` · companion — `tokio_client.rs` joins the `viola-channel` scope
  (`contract_windows_mutation_scope` requires every Windows-gated source there).
- `src/cmd/{answer, last, pause, send, wait}.rs` · companion — callers of the moved `live_endpoint`.

## Deviations from the plan, with their justifications

1. **Step 7, the board provider.** `list::board_document` (scan → read → document) is what the `mcp` provider calls
   whole; CLI `list` keeps its order — scan, its own log opened, read, document — and calls the same `read` and
   `document` pieces and the shared `refused_document`. One function cannot hold CLI `list`'s log open between the
   scan and the read without a role parameter; CLI `list`'s output, exits and lines are unchanged (its cases green).
2. **Step 3, the tree form's reading.** Research measured `cargo tree … -i tokio` exiting 101 `did not match`;
   once rmcp and tokio are in `Cargo.lock` it exits 0 `nothing to print`. The script reads presence by a `tokio v`
   line and takes both absent forms; its known-verdict controls read both ways (`evidence/instrument-controls.md`).
3. **Step 6, the list result.** `tools_result()` sets `ttlMs` 0 and `cacheScope` `private`: required by
   protocol `2026-07-28` clients (the live leg's finding); unit-pinned with its remove-the-guard reading.
4. **Step 12, the handler tests' entry.** `viola_mcp::Tools` (`new`, `call`) is the transport-free handle the
   integration tests drive; `serve` answers each `tools/call` through the same `Server::call`. The test wrapper's
   snapshot is written through `viola_state::snapshot::write_snapshot` (the writer, never hand-written bytes).
5. **Step 13, the E2E client.** The rmcp client runs over a `viola mcp` child the test owns (features `client`
   only), not rmcp's `transport-child-process`: that transport kills the server 3 s after a close, and a killed
   instrumented server left a truncated coverage profile (114688 B against 7361 siblings of 141984 B) that
   `llvm-profdata` refused, reddening pre-push. The fix removes the kill; that this process was the killed one is
   the likely reading, not a measured one.
6. **Step 13, the E2E's scripted turns.** `path3.json`'s `PostToolUse.default` has no 2.1.287 fixture, so the
   Path 3 leg writes its own two-step script (`PostToolUse.parallel-1`, `Stop.default`, both recorded) beside its
   control file; Path 4 uses `path4.json` as committed.
7. **Step 13, the kill line.** `binary(mcp_tools)` joined the 20 s line (the plan's condition: it starts from
   stamped homes); its cases read 0.01–5.9 s.
8. **Step 15, the rig.** `live-drive.py` was copied but not run: it drives the release-check binary and its turn
   loop answers dialogs; this leg answers none, so the verbs were typed against the session's own build.
9. **CARRY 1's companion.** viola-channel's `fake-agent` feature now enables `tokio`, so a package-scoped mutation
   build (`--package viola-channel --features fake-agent`) compiles and tests the `tokio_client` it mutates; the
   release build never carries `fake-agent`, and `channel-tokio-free.sh` reads the default features.

## Surfaced

- **The live criterion (plan step 15 (d); `v1-38`).** Read: `viola mcp` started under the wrapped session
  (`process-start`, `instance` `driver`); the CLI's own MCP log refused `tools/list` (`ttlMs`, `cacheScope`) three
  times, then dropped the server; the turn ended without a tool call and without the fixed word; no permission
  dialog was raised. Not read: the `wait` frame, its 50–60 s duration, the fixed word. The cap was one start;
  not re-fired (`evidence/live-mcp.md`, `evidence/live-sessions.ndjson`).
- **A pre-existing product race (`viola_state::pin::pin_exe`, untouched here).** Two first `viola run`s on a fresh
  home pin the binary at once, and one agent's first hook spawn then fails: 6 of 24 concurrent unstamped
  two-instance boots, 0 of 24 single-instance ones (`evidence/unstamped-boot-race.md`). Path 7 now boots `builder`
  alone. The kernel error is not read (`ETXTBSY`, a hypothesis). Its owner is the wrap's to pin.
- **Expected amendments beyond the plan's list:** architecture [MCP] — the list result's `ttlMs` 0 and
  `cacheScope` `private` under protocol `2026-07-28`; test-plan §5 / §6 — the E2E client over an owned child (no
  `transport-child-process`) and `SessionOptions::unstamped`; test-plan's 20 s kill-line list gains `mcp_tools`;
  the Windows mutation scope gains `tokio_client.rs`, and viola-channel's `fake-agent` enables `tokio`.
- Recorded, not a red: `contract_fake_agent_drift fake_agent_dialog_replay_matches_every_recorded_dialog_set` read
  `SLOW > 10 s` (passed, 10.49 s) in the second full E2E firing under load.

## Process hygiene

Measured 2026-10-17T12:34:24Z against `/proc` (every process whose executable or working directory lies under the
repository, its `target/` or the tmpfs home base behind the `target/e2e-home` link):

| process | started by | final state |
|---|---|---|
| `builder` wrapper + fake agent (live leg) | this run (the rig) | terminated — stop file, 1 Ctrl-C, exit 0 |
| `driver` wrapper + `claude` 2.1.287 + its `viola mcp` (live leg) | this run (the rig) | terminated — stop file, 3 Ctrl-C, exit 0 |
| the 48 reproduction boots' supervisors, wrappers and agents | this run (`repro_unstamped.py`) | terminated — each cleaned through `viola-harness cleanup` |
| the harness sessions of the gate entries (`p-mcp-smoke`, every `harness_session`) | a gate entry | terminated — cleanup entries green; none at the census |
| the killed `mcp_tools_path7_unverified_cli_answer` session (second firing) | a gate entry | terminated — nextest's kill; none at the census |

None left running.

## Decisions & corrections

- The founder's cap: one live `claude` 2.1.287 start (his dialog 2026-10-17T11:18Z, in the operator's
  invocation): made once, ledgered first (`written_at` 12:20:50Z); a failed reading not re-fired.
- The operator's direction for this run: tools run bare; long runs journalled in the run dir before waiting
  (`journal.md`); no mutation run of any kind (none made); then the operator pass with the `ci.py` read.
- Guard refusals: a `cd` that moved the cwd (twice), a `cat` heredoc to a file (once); each re-issued in the
  guard's form (absolute paths, a subshell, the Write tool).
- A red was read to its cause before re-running (the standing rule): path7's timeout two-sided
  (`unstamped-boot-race.md`), the coverage merge by the profile sizes, the mcp_tools premises by their logs.
