# Remove-the-guard readings — 2026-09-27-wrapper-channel

testing.md 2026-09-25: every new guard test carries its run with the guard neutralised (red), then
restored (green). The torn receipt read's pair is in `red-torn-receipt-read.md`.

## `conn` never reaches the dispatch (`crates/viola-channel/src/server.rs` `answer`)

Guard: `request.params` loses `conn` before `dispatched` runs, so no method can use it as identity
(verification-matrix v1-20: "the server's dispatch takes no `conn` input").
Test: `server::tests::dispatch_never_receives_the_peer_conn` (`cargo nextest run -p viola-channel`).

- Neutralised (`fields.remove("conn")` replaced by `let _ = fields;`, the edit confirmed by grep before
  the run): **red** —
  `left: [Object {"v": Number(1), "conn": String("cli-1-2-3"), "text": String("t")}]`,
  `right: [Object {"v": Number(1), "text": String("t")}]`.
- Restored (grep: the marker gone, `fields.remove("conn");` at line 199): **green**, the crate
  102/102.

## A peer's `sender` is logged only as `MAJOR.MINOR.PATCH` (`server.rs` `version_label`)

Guard: `sender` reaches `channel-request` only as three dot-separated digit runs.
Test: `tests/channel_endpoint.rs::channel_debug_level_keeps_request_content_out_of_the_role_files`
(v1-20's redaction half), which sends the tests-owned canary in `params`, `conn`, `sender`, `from`
and `method` at `diagnostics_level: "debug"`.

- With the first, looser label (1–32 of `[0-9A-Za-z.+-]`): **red** —
  `…/home/diagnostics/run-builder.ndjson carries request content` (the canary passed as a version).
- With the strict label: **green**. Unit cases `version_label_takes_version_strings_only` pin it
  (`canary-chain-value-5c1e`, `1.2.3-rc.1`, `1234567890.1.1` refused; `123456789.20.300` taken).

## `pre-push` stops the VM before the host stages (`crates/viola-e2e/src/harness/pre_push.rs`)

Guard: `release_vm` runs `wsl.exe --terminate Ubuntu` after the ubuntu verdict's copy-back and
before any host stage (overseer direction after two host memory stops).
Test: `pre_push::tests::pre_push_stops_the_vm_after_the_copy_back_and_before_the_host_stages`.

- Neutralised (`out(runner, terminate)` replaced by `false`, confirmed by grep): **red** — `wsl.exe
  --terminate Ubuntu in [...]`, the call list running from the copy-back `cat` and the `du` probes
  straight into `cargo llvm-cov nextest` with no terminate.
- Restored (grep: marker gone): **green**, the `pre_push` tests 29/29.

## E2: the wrapped child holds nothing of viola's (`tests/tui_channel_fds.rs`)

All on Linux, in the pre-push WSL clone under `env -i`, on the working tree synced by a temp-index
binary patch over HEAD `17c99c9`; every experiment edit was reverted and the clone compared
byte-exact with the host after.

1. Tree as written: **green**.
2. Plant (clone only): before `viola_pty::spawn`, `viola run` opens `instances/builder/events.ndjson`,
   clears `FD_CLOEXEC` with `fcntl` (asserted `!= -1`) and leaks the fd: **green**. The rebuild of
   `viola` is in the log (`Compiling viola v0.1.0`).
3. Plant + a probe asserting the WRAPPER's own `/proc/<pid>/fd` holds the planted file: **green** —
   the leak was live in the wrapper and absent in the child.
4. Probe without the plant (control): **red** — `PROBE: the wrapper does not hold the plant`, the
   wrapper's table listed as `/dev/pts/2` ×3, `…/home/diagnostics/run-builder.ndjson`,
   `/tmp/viola-1000/viola-8176799ebc82.lock`, `socket:[34498]`, `/dev/ptmx` ×3: the role file, the
   endpoint lock and the listener, none of which reached the child in any reading.
5. The fixture exemption dropped (the agent's receipt no longer excused): **red** —
   `fd 3 holds a viola home file: [(0, "/dev/pts/3"), (1, "/dev/pts/3"), (2, "/dev/pts/3"),
   (3, ".../home/fake/builder.receipt.ndjson")]`.

Reading 2 cannot go red: portable-pty 0.8.1 runs `close_random_fds()` in the child's `pre_exec`
(`src/unix.rs:239`), closing every fd above 2 whatever its CLOEXEC flag. That dependency guard is
what holds the property; readings 2–4 measure it holding, reading 5 is the red that proves the
predicate fires on a home file the child really holds.
