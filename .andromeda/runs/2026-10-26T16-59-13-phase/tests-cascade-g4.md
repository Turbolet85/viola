# tests · cascade read · group 4 of 6

## Carried
- (.andromeda/test-plan-detail/e2e-test-strategy.md:11) The hook's added strict-modes check before its `VIOLA_DIR` snapshot read, and the fail-open matrix's new strict-modes failure of that home, must be driven by assert_cmd over recorded fixture stdin with the `Instant` deadline, the check staying inside that bound.
  > | cli (`viola hook` / `hook statusline`) | assert_cmd 2.2.2 with `Instant` deadline | direct | Fixture stdin from `fixtures/claude/<ver>/`.
  (.andromeda/test-plan.md:598)
  > the hook fail-open matrix — oversize stdin, malformed JSON, clap error, channel failure, a forced panic, a missing `VIOLA_NAME` and, on Linux and Windows, a server that is not the recorded wrapper, each exit 0 with no body — plus the `--home` strict-modes refusals
- (.andromeda/test-plan-detail/e2e-test-strategy.md:14) A case of `viola mcp`'s frame tools refused through `live_endpoint` on a failed strict-modes check must drive a `viola mcp` child the test spawns and owns, in `crates/viola-e2e`'s `tests/mcp_tools.rs`, never through `TokioChildProcess`.
  > rmcp 3.4.1 client over a `viola mcp` child the test spawns and owns, closing its stdin and awaiting its exit (never rmcp's `TokioChildProcess`, which kills the server 3 s after a close and left a truncated coverage profile, as measured at chunk 2026-10-17-mcp-server-for-drivers) + jsonschema | direct | Lives in `crates/viola-e2e` (`tests/mcp_tools.rs`), which takes rmcp feature `client` only, as a normal dependency (`viola-e2e` carries no `[dev-dependencies]`), no `transport-child-process`.
  (.andromeda/test-plan.md:470)
  > an rmcp client over a `viola mcp` child the test spawns and owns, closing its stdin and awaiting its exit, never rmcp's `TokioChildProcess`
- (.andromeda/test-plan-detail/e2e-test-strategy.md:28) Path 1's tamper and restore steps must stand as the end-to-end witness of the pinned-binary re-hash before reuse and the per-start `plugin/` and `settings.json` rewrite that the chunk re-reads.
  > 5. Tamper and rewrite: stop the first wrapper (Ctrl-C into the outer PTY, `child.wait()`). Overwrite one byte of the file at snapshot `pinned_bin`, and write a sentinel into the instance's `plugin/` hooks.json and `settings.json`. Then run `viola run builder --home H` again.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:29)
  > 6. Restore the pinned exe by deleting it. Run again, and read the plugin files.
  (.andromeda/test-plan.md:898)
  > NEVER use `assert_cmd` `.assert()` on the long-lived `viola run` wrapper. It blocks until exit; use std::process + a `Drop` guard.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:34) The tampered pinned exe must exit `run` 1 and a restart must rewrite `plugin/` with the absolute pinned path and, on Unix only, `settings.json` whole at 0600, the values the chunk re-asserts.
  > The duplicate `run` exits 1. The squatted endpoint exits 1. The tampered pinned exe exits 1.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:35)
  > After restart, `plugin/` hooks.json no longer contains the sentinel, and its hook commands are the absolute `pinned_bin` path, not a PATH lookup (M6).
  (.andromeda/test-plan-detail/e2e-test-strategy.md:36)
  > The `settings.json` half of step 5 is Unix's. On Unix the restart rewrites the instance's `settings.json` whole at 0600 as `{"statusLine":{"type":"command","command":"<pinned path> hook statusline"}}` with the absolute pinned path, and only when every character of that path is an ASCII letter, a digit or one of `_ - . / :`. On Windows no `settings.json` is written.
  (.andromeda/test-plan.md:488)
  > a duplicate run, a squatted endpoint and a tampered pinned exe each exit 1; a restart rewrites `plugin/` with absolute hook commands, never a PATH lookup, and on Unix the `settings.json` override.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:37) `run`'s added per-entry-point checks must keep the one-home ordered Path 1 flow green over `stamped_home`, every signal asserted, inside the `ci` profile's 20 s kill line.
  > steps 1–6 run as one ordered flow over one stamped home in the root path binary `tests/path1_run_start.rs`, `path1_run_start_one_home_in_order(stamped_home)` (`[[test]]` `required-features = ["fake-agent"]`; in the E2E layer, `nextest-e2e`; on the `ci` profile's verify-driven 20 s kill line), every signal above asserted, the `settings.json` half Unix's, its start-refusal helpers shared with `tests/cli_instance_state.rs` through `tests/support/start.rs`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:90) A strict-modes refusal at a CLI frame verb must exit 21 with exactly this `instance-unreachable` document, and at an MCP frame tool return `is_error:true` with the CLI document less `v` and text `refused: instance-unreachable`.
  > Step 7: CLI exit 21 with `{"error":"instance-unreachable","detail":null}`; MCP `is_error:true` with the same body, `{"error":"instance-unreachable","detail":null}` (the CLI document less `v`), text `refused: instance-unreachable`.
  (.andromeda/test-plan.md:606)
  > a Windows `--home` outside the profile given a protected DACL (its test home never under `%TEMP%`), umask-independent modes, single writers of `stamps.json` and `snapshot.json`, and, on Linux and Windows, a CLI frame verb writing no frame to a server that is not the recorded wrapper

## Bindings
- **hook driver ↔ perf gate and the security sweep**: the hook's added check must keep the spine hooks inside the fail-open matrix's `Instant` bound (.andromeda/test-plan-detail/e2e-test-strategy.md:11) while the matrix gains the `VIOLA_DIR` home's strict-modes failure (.andromeda/test-plan.md:598).
- **Path 1 tamper signal ↔ the contract suite**: the tampered pinned exe exiting 1 end to end (.andromeda/test-plan-detail/e2e-test-strategy.md:34) rests on the pinned-exe content hash pinned by literal vectors (.andromeda/test-plan.md:617).
- **Path 1 as landed ↔ test data (`stamped_home`)**: the ordered flow starts from a stamped home viola created (.andromeda/test-plan-detail/e2e-test-strategy.md:37), which must pass the new checks at every verb.
- **Path 3 step 7 refusal ↔ cross-surface parity and the MCP driver**: the MCP strict-modes refusal equals the CLI document less `v` (.andromeda/test-plan-detail/e2e-test-strategy.md:90, .andromeda/test-plan.md:611), driven over an owned `viola mcp` child (.andromeda/test-plan-detail/e2e-test-strategy.md:14).
- **E5 ↔ the founder's P4 fork**: the readers' move onto `read_snapshot_or_replay` stays a CARRY owed to this entry (.andromeda/test-plan.md:590), neither planned in nor out until his answer.
