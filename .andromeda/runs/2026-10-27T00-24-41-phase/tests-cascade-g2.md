# tests · cascade read · group 2 of 3

## Carried
- (.andromeda/test-plan-detail/integration-test-strategy.md:12) A concurrent-pin control must race separate processes over the real pinned copy in a real tempfile home, and where the fix serialises the absent-copy write through a lock, that contention is tested on disk, never mocked.
  > Replaced by module ↔ on-disk state: real ndjson, snapshots and `.lock` siblings in a tempfile home, with concurrent appenders checking one line per `write`, plus `File::lock` contention.
  (.andromeda/test-plan.md:421)
  > there is no database; module ↔ on-disk state is tested instead over real ndjson, snapshots and `.lock` siblings in a tempfile home
- (.andromeda/test-plan-detail/integration-test-strategy.md:22) A Windows x64 concurrent first start writes the companions into the same `bin/<key>/` as the exe, so the sideload boundary must keep holding: tampered pinned copies fall back to the inbox ConPTY and are never overwritten by the fix.
  > On Windows x64 every root test that starts `viola run` hosts its child on the sideloaded `bin/<key>/conpty/OpenConsole.exe` (`pty_backend` `conpty-sideload`); viola-pty's own tests, `tests/support/outer_pty.rs`'s outer PTY and the harness `supervise::spawn_in_pty` stay on the inbox ConPTY. The sideload boundary: the root integration binary `conpty_sideload` (the default sideload, planted copies in the CWD and on `PATH`, tampered pinned copies falling back to `System32\conhost.exe`, and `run_on_a_companion_another_user_may_write_falls_back_to_conhost`: a companion its owner/DACL check refuses leaves the child on the inbox ConPTY with `sideload_fallback` `strict-modes-failed` and the exit unchanged)
- (.andromeda/test-plan-detail/integration-test-strategy.md:33) The race witness's rig differs by host: dev-host homes sit on tmpfs through the link while CI homes end the pinned copy in a real `sync_all()`, so the witness must show the race window reachable on the rig it reads, and the product's first-start write must stay covered by unseeded starts.
  > on the Linux dev host `target/e2e-home` is an operator-set symbolic link to the owner-only tmpfs directory `/tmp/viola-e2e-home-<uid>`, so the homes' bytes sit outside the working directory while every home class keeps its path, viola still creates the home itself, and G2, G4 and the secret scan reach it through the link. No CI runner has a link, so CI's homes stay on the runner's disk and the pinned copy still ends in the real `sync_all()`.
  (.andromeda/test-plan-detail/integration-test-strategy.md:34)
  > The reason for the seed is the first start's companion write, as measured at that chunk on the dev host under a parallel suite: a median 1 193 ms, which pushed first-start tests past their 7 s waits; no bound was raised. The product's first-start write stays covered by the unseeded starts: `tests/conpty_sideload.rs` and `run_viola_unseeded` never seed.
  (.andromeda/test-plan.md:431)
  > Two carve-outs: Windows x64 starts seed the ConPTY companions before viola runs, into a home `seed_conpty` makes private
- (.andromeda/test-plan-detail/integration-test-strategy.md:62) The home-tree strict-modes check over `bin/` and the pinned copy must keep refusing before the log for `run` and `verify`, whose pin calls the chunk edits, so the fix runs after that check and changes none of its refusals.
  > `tests/run_cli.rs` `run_refuses_a_widened_home_tree_before_its_log` widens the home, `instances/<name>`, `bin/` or the pinned copy after one clean start, and the next `viola run` is refused before its log opens with the home's or the instance's pair, no child and the widening as found; its home starts unseeded, because a seeded home's companions are hard links to one shared seed file that an inheritable grant on the home or `bin/` would reach. `tests/cli_verify.rs` `verify_refuses_a_widened_ledger_before_any_step` widens `ledger/` on a stamped home, and the next `viola verify` is refused with the home pair, no step line or summary and `stamps.json` byte for byte.
  (.andromeda/test-plan.md:441)
  > strict-modes refusals give exit 1 for `run` / `mcp` / `viola revive` / `viola verify`
- (.andromeda/test-plan-detail/e2e-test-strategy.md:12) The race witness's verdict must come from logged events, the fake-agent receipt and exit codes, never from timing.
  > Exit only via `child.wait()`, and the master reader is drained on its own thread. Verdicts come from `events.ndjson`, the fake-agent receipt and the exit status.
  (.andromeda/test-plan.md:471)
  > exit only via `child.wait()`, the master reader drained on its own thread, verdicts from `events.ndjson`, the receipt and the exit status.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:28) Path 1's restore-by-deletion drives the absent-copy arm the chunk edits, and its tamper step drives the present-copy re-hash that must never become an overwrite.
  > 5. Tamper and rewrite: stop the first wrapper (Ctrl-C into the outer PTY, `child.wait()`). Overwrite one byte of the file at snapshot `pinned_bin`, and write a sentinel into the instance's `plugin/` hooks.json and `settings.json`. Then run `viola run builder --home H` again.
  >   6. Restore the pinned exe by deleting it. Run again, and read the plugin files.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:31) The hook receipt `ran:true` on record three is the inverse of the race's `ran:false` signal, so it must hold after the fix, as must the tampered pinned exe's exit 1 and the per-start plugin rewrite with absolute commands.
  > The third record and the M6 check below are asserted by `path1_session_start_is_record_three_through_the_absolute_hook` (the fake agent's `hook` receipt reads `command_absolute:true, ran:true, exit_code:0`)
  (.andromeda/test-plan-detail/e2e-test-strategy.md:34)
  > The duplicate `run` exits 1. The squatted endpoint exits 1. The tampered pinned exe exits 1.
  >   - After restart, `plugin/` hooks.json no longer contains the sentinel, and its hook commands are the absolute `pinned_bin` path, not a PATH lookup (M6).
  (.andromeda/test-plan-detail/e2e-test-strategy.md:37)
  > The start runs the home-tree strict-modes check (`check_home_tree`: the home, `check_instance`'s set, `ledger/`, `bin/` and `plugin/`, then `bin/<key>/`, the pinned copy, `plugin/<key>/`, its `hooks/` and `.claude-plugin/`, each that exists) before the wrapper log opens.
  (.andromeda/test-plan.md:493)
  > a duplicate run, a squatted endpoint and a tampered pinned exe each exit 1; a restart rewrites `plugin/` with absolute hook commands, never a PATH lookup
- (.andromeda/test-plan-detail/e2e-test-strategy.md:38) The landed Path 1 flow reaches the absent-copy arm at its step 6 and must stay green inside its kill line once the fix changes that write.
  > steps 1–6 run as one ordered flow over one stamped home in the root path binary `tests/path1_run_start.rs`, `path1_run_start_one_home_in_order(stamped_home)` (`[[test]]` `required-features = ["fake-agent"]`; in the E2E layer, `nextest-e2e`; on the `ci` profile's verify-driven 20 s kill line)
- (.andromeda/test-plan-detail/e2e-test-strategy.md:194) The unstamped harness boot is the rig the CARRY measured the race on, and Path 7's one-instance boot is the standing workaround; whichever instance count P4 settles, Path 7's verdicts must hold.
  > `agent-run boot --session p7 --unstamped --instance builder` (no stamps at all). Separately, `agent-run boot --session p7-stamped --instance builder:--report-version 0.0.0-unlisted`.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:199)
  > `send` exits 0 (confirmed).
  >   - The `question` event is logged once. Hook stdout is empty, with exit 0 and returning within the spine deadline.
  (.andromeda/test-plan.md:541)
  > on an unverified CLI `send` still confirms, the `question` event is logged once with an empty hook stdout, and `answer` exits 12 `unverified-cli` (MCP alike)
- (.andromeda/test-plan-detail/e2e-test-strategy.md:283) A pinned exe failing its re-hash must stay a start refusal with exit 1, through `run` and `revive`'s start alike, and the fix must never turn it into a rewrite.
  > the pinned exe fails its SHA-256 re-hash (Path 1 step 5)
  (.andromeda/test-plan-detail/e2e-test-strategy.md:287)
  > A refusal `run`'s start makes after a passed preflight (`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, a `claude` not on `PATH` as `internal-error`) prints no document under `revive --json` and no case asserts one.
  (.andromeda/test-plan.md:607)
  > the exit-1 home-tree refusals of `run` and `verify` are cases of `tests/run_cli.rs` and `tests/cli_verify.rs`
- (.andromeda/test-plan-detail/e2e-test-strategy.md:306) The fix's absent-copy write must keep the pinned exe 0700 under any umask, and a concurrent Windows first start must add no `strict-modes-failed` fallback to the kept homes beyond that test's own.
  > Start `viola run` under `umask 000` (via `sh -c 'umask 000; exec viola run …'`). `events.ndjson`, `snapshot.json` and the other home files are still 0600, directories 0700, and the pinned `bin/<version>-<hash>/viola` is 0700 and never group- or world-writable.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:314)
  > `tests/conpty_sideload.rs` `run_on_a_companion_another_user_may_write_falls_back_to_conhost`: a companion another user may write leaves the child on the inbox ConPTY, its `process-start.sideload_fallback` `strict-modes-failed`, the exit unchanged; the kept homes of the Windows leg hold exactly one `strict-modes-failed` fallback, that test's (as measured at ci#40403389629).
  (.andromeda/test-plan.md:609)
  > umask-independent modes
- (.andromeda/test-plan-detail/e2e-test-strategy.md:367) The pinned-exe content-hash vectors stay unchanged, and that truncated hash is the `bin/<key>/` key Measurement B reads a rebuild of one tree against.
  > `tests/contract_content_hash.rs` pins the pinned-exe content hash (§1 "SHA-256 16-hex re-hash") to its library, `sha2` (a root dev-dependency): `sha256_published_vector_matches_digest` over the three FIPS 180-2 vectors and `sha256_truncated_to_eight_bytes_yields_sixteen_hex` over the 8-byte / 16-hex truncation, literal oracles, on all three OSes.
  (.andromeda/test-plan.md:620)
  > and the pinned-exe content hash is pinned to its library by literal vectors.

## Bindings
- **integration on-disk concurrency ↔ e2e race witness**: the concurrent-pin control runs as separate processes over the real pinned copy (.andromeda/test-plan-detail/integration-test-strategy.md:12), its verdict read from events, the receipt and exits (.andromeda/test-plan-detail/e2e-test-strategy.md:12), its signal the record-three hook receipt `ran:true` against the race's `ran:false` (.andromeda/test-plan-detail/e2e-test-strategy.md:31), on the unstamped harness boot rig (.andromeda/test-plan-detail/e2e-test-strategy.md:194).
- **integration lifecycle ↔ e2e race witness**: the race window the witness must show reachable depends on the home's filesystem, tmpfs on the dev host and the real `sync_all()` on CI (.andromeda/test-plan-detail/integration-test-strategy.md:33), and the first-start write stays covered by unseeded starts (.andromeda/test-plan-detail/integration-test-strategy.md:34).
- **integration strict-modes ↔ e2e Path 1 and exit-cause matrix**: the home-tree check over `bin/` and the pinned copy (.andromeda/test-plan-detail/integration-test-strategy.md:62, .andromeda/test-plan-detail/e2e-test-strategy.md:37) precedes the pin write the chunk edits, and the re-hash refusal stays exit 1 (.andromeda/test-plan-detail/e2e-test-strategy.md:283, .andromeda/test-plan-detail/e2e-test-strategy.md:34), with Path 1 step 6 the absent-copy arm's existing driver (.andromeda/test-plan-detail/e2e-test-strategy.md:28).
- **integration PTY sideload ↔ e2e security negatives**: the Windows companions in the same `bin/<key>/` keep falling back when tampered or refused (.andromeda/test-plan-detail/integration-test-strategy.md:22), and the Windows leg keeps exactly one `strict-modes-failed` fallback (.andromeda/test-plan-detail/e2e-test-strategy.md:314).
- **e2e contract suite ↔ Measurement B**: the truncated content hash pinned by literal vectors (.andromeda/test-plan-detail/e2e-test-strategy.md:367) is the key a rebuild that reads as another binary would move.
