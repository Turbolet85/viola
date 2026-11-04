@carried n=1 label=test-plan/integration-test-strategy fact=fct-vmzc7an2
why: The Unix wrapper's openpty parity must be read at the Module to PTY boundary over real openpty spawns of the fake agent: exit on the handle, a resize propagated (the start-window one included) and a key written after a resize only once the child reports the new size.
quote: Real ConPTY / openpty spawn of `viola-fake-agent` through `viola-pty`. The no-EOF mode checks exit on handle, a resize is propagated — including one that lands between the spawn sizing and the pump's first look, because the pump's resize baseline is the size the PTY was spawned with, never a second host read (`pump_forwards_a_resize_that_lands_before_its_first_look`; the forced-window tui test `tui_host_resize_in_the_pump_start_window_reaches_the_child` via the `fake-agent`-only `FAKE_AGENT_PUMP_DELAY_MS` seam, as measured at chunk 2026-09-26-local-linux-pre-push-gate: 6/6 red before the fix, 6/6 green after) — and a `.cmd` child is refused (`#[cfg(windows)]`). A resize reaches a child that reads no key (`spawn_reports_a_resize_to_a_child_that_reads_no_key`, the child's key-free size watcher). A test that writes a key after a resize sends it only once the child reports the new size (the red test `spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code` waits for its `size 120x40` line: 0 of 200 lost on the windows-2025 runner, ci#36529038462).
@carried n=1 run=2 label=test-plan fact=fct-nvtf2boy
quote: **Module ↔ PTY:** real ConPTY / openpty spawns of `viola-fake-agent` through `viola-pty`: exit detected on the handle, a resize propagated, a key written after a resize only once the child reports the new size, a `.cmd` child refused, and on Windows x64 the root tests' children hosted on the sideloaded ConPTY.
@carried n=1 run=3 label=test-plan fact=fct-5vfqglut
quote: **PTY:** real portable-pty through `viola-pty`.
@carried n=2 label=test-plan/integration-test-strategy fact=fct-ouc4a2b7
why: The Unix-socket branch of the multi-os-compat trigger must hold the 0700 directory and 0600 socket read back and the socket directory refusals on Linux and macOS, as the task's OS-specific behaviours name them.
quote: Unix socket dir 0700 / file 0600 (set by a chmod after the bind) are read back with `std::fs::metadata`. On Unix the directory is also checked before use: `check_socket_dir`, through `check_endpoint_dir`, refuses a directory with a foreign bit or a foreign owner, a symlink or a non-directory, passes an absent path and never chmods, a refused directory staying as found; a directory at 0755 and a symlink to 0700 are refused in §6 Security control negatives (`tests/security_negatives_socket_dir.rs`), a foreign owner by injected uid at `socket_dir_verdict` (§4 viola-state).
@carried n=2 run=2 label=test-plan fact=fct-ctentyeb
quote: the Unix socket modes and the Windows pipe DACL read back — the DACL compared in its canonical form, never the literal it set.
@carried n=3 label=test-plan/integration-test-strategy fact=fct-ksdwqwkf
why: The macOS carve-out the scope keeps must stay as recorded: the server-identity negatives run on Linux and Windows only, compiled out on macOS, and no test reads the macOS check as a pid or start-time comparison.
quote: a server pid / start-time mismatch versus `snapshot.json` gives CLI exit 21 and the hook exits 0 with no body, on Linux and Windows: the CLI half in `tests/security_negatives_server_identity.rs`, the hook half the impostor rows of `tests/hook_fail_open.rs` (§6 Security sweep). macOS reports no peer pid, so there the pid and start-time comparison is absent, the named carve-out, owned by no route entry, and the server-identity negatives stay compiled out; its peer euid and socket directory checks are built, a refused euid reading `server-verify-failed` and a refused directory `socket-dir-refused`.
@carried n=3 run=2 label=test-plan/integration-test-strategy fact=fct-ksdwqwkf
quote: a peer euid `peer_admitted` refuses or a failed `server_verified` is `server-verify-failed` (Linux, Windows and macOS, where on macOS it means an euid mismatch alone)
@carried n=3 run=3 label=test-plan fact=fct-lktzdsoq
quote: NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
@carried n=4 label=test-plan/integration-test-strategy fact=fct-ksdwqwkf
why: A channel test run on both Unix runners must keep the per-OS close oracle that accepts Linux's reset and macOS's EPIPE or ENOTCONN while still requiring the -32600 reply and the close.
quote: a frame over `MAX_FRAME` gives -32600 and the socket closes. After the close, Windows reads 0 bytes, Linux may see ECONNRESET, and on macOS the sender's tail write may get EPIPE (as measured in CI run 36313377307) or ENOTCONN (os error 57, as measured in CI run 36482322449), whichever state the close reached first; the test accepts EPIPE, ENOTCONN or a reset on the write and still requires the -32600 reply and the close
@carried n=5 label=test-plan/integration-test-strategy fact=fct-aajqyzhy
why: Every Unix-run case of the hook fail-open matrix must assert the hook's closed contract on each OS, the event line, hook trace and budget.json read the same on Linux and macOS as on Windows.
quote: `viola hook <event>` fed fixture stdin with `VIOLA_NAME`/`VIOLA_DIR` against a live test wrapper. The event line, the `diagnostics/hook-<name>.ndjson` trace and `budget.json` are asserted.
@carried n=5 run=2 label=test-plan fact=fct-3aeso5ca
quote: **Hook exec ↔ channel ↔ disk:** `viola hook <event>` fed fixture stdin against a live test wrapper, its event line, hook trace and `budget.json` asserted.
@carried n=6 label=test-plan/integration-test-strategy fact=fct-hctsd5xl
why: One diagnostics schema on every OS requires every line of every shared diagnostics file to parse as one JSON object under concurrent appenders on ubuntu-latest and macos-latest as on windows-2025, the not-yet-built role files being rows the plan may add.
quote: **Concurrent-append check** (obs-plan multi-platform item, all 3 OSes): separate processes append at once to each shared diagnostics file (`hook-<name>.ndjson` from parallel hook processes, `mcp.ndjson` from two `viola mcp`, `cli-<name>.ndjson` from parallel verbs, a colliding second `run` on the live `run-<name>` file, and the shared `detail-hook` / `detail-cli` / `detail-mcp` files), including at least one detail line **larger than 4 KiB**. Every line of every file must parse as one JSON object; a torn or interleaved line fails the test.
@carried n=6 run=2 label=test-plan fact=fct-rhuz6eg7
quote: with a concurrent-append check that every line of every shared diagnostics file parses as one JSON object.
@carried n=7 label=test-plan/integration-test-strategy fact=fct-b4mgg72y
why: The v1-40 witness's `viola list` and every Unix reading must reach `claude` only as the fake agent on a test-scoped PATH, never the real CLI.
quote: The only external boundary is child processes. `claude agents --json` is served by `viola-fake-agent` copied to a test-scoped `PATH` as `claude`, in recorded, oversize and malformed modes.
@carried n=7 run=2 label=test-plan fact=fct-jnilykpz
quote: the only external boundary is child processes, served by `viola-fake-agent` as `claude` on a test-scoped `PATH` and by a fixture npm shim.
@carried n=8 label=test-plan/integration-test-strategy fact=fct-5qxwlgko
why: The Unix endpoint-gone reading after a force-killed wrapper is witnessed on ubuntu-latest alone, so a fake-agent test reading a behaviour both Unix OSes share must also run on macos-latest or carry its named one-OS reason.
quote: **Harness `cleanup` witnesses** (the `endpoint_gone` reading on Unix, §3 `cleanup` step 4): `cleanup_after_a_force_killed_wrapper_reads_the_endpoint_gone` (`crates/viola-e2e/tests/harness_lifecycle.rs`) on `ubuntu-latest`, and the unit `unconnectable_reads_a_leftover_socket_file_no_one_listens_on` (`crates/viola-e2e/src/harness/cleanup.rs`).
@carried n=9 label=test-plan/integration-test-strategy fact=fct-qmrx4mno
why: The G2 and G4 re-read on ubuntu-latest and macos-latest must keep `tests/chaos_feed_panic.rs` the only home outside the scans, its forced vt100 panic line asserted by the test itself.
quote: A second carve-out (chunk 2026-10-04-confirmed-send-with-cl-1-records, the founder's ruling, live, 2026-10-04, relayed by the overseer): `tests/chaos_feed_panic.rs` alone boots in `TestHome::outside_scan()`, a home under the system temp dir (`viola-chaos-*`), not under `target/e2e-home`, because its forced vt100 feed panic writes a G2-counted `event:"panic"` line; that home is therefore outside G2 (zero panics), G4 (schema conformance) and the secret scan, and the test asserts its panic line and its `parse-rejected{vt100-feed, panicked}` line present itself.
@carried n=10 label=test-plan/integration-test-strategy fact=fct-qmrx4mno
why: Reading the fake-agent suite green on macos-latest must keep the readiness stall report as the witness for the macOS readiness stall inside `run.start`, signalling nothing and adding nothing to a passing boot.
quote: **Readiness stall report:** when `tests/support/home.rs` `wait_ready` reaches its deadline, it appends the stalled wrapper's state to the watch report before the panic, so the report reaches the JUnit. That state is the wrapper's last `run-<name>.ndjson` event; on macOS `/usr/bin/sample <pid> 1`'s call-graph stack lines (bounded at 4 s, at most 120 lines); on Linux `/proc/<pid>/wchan` and the `stat` state field (`note_stall`; `wrapper_pid` by one sysinfo read; `process_state` per OS, a no-op elsewhere). It signals nothing and adds nothing to a passing boot. It is the witness for the macOS readiness stall inside `run.start`, whose phase is not yet named (route WATCH).
@carried n=10 run=2 label=test-plan fact=fct-xq6vpu6l
quote: **Readiness stall report:** at its deadline `wait_ready` appends the stalled wrapper's state to the watch report before the panic; it signals nothing and adds nothing to a passing boot.
@carried n=11 label=test-plan/integration-test-strategy fact=fct-bt3npe7x
why: `tests/run_cli.rs` is reached whole, so its home-tree refusal case must keep its oracle and its unseeded home while the chunk changes the file.
quote: the start and verify home-tree checks (their exits in the strict-modes item above): `tests/run_cli.rs` `run_refuses_a_widened_home_tree_before_its_log` widens the home, `instances/<name>`, `bin/` or the pinned copy after one clean start, and the next `viola run` is refused before its log opens with the home's or the instance's pair, no child and the widening as found; its home starts unseeded, because a seeded home's companions are hard links to one shared seed file that an inheritable grant on the home or `bin/` would reach.
@carried n=12 label=test-plan/integration-test-strategy fact=fct-bt3npe7x
why: One diagnostics schema across OSes requires the events.ndjson event line to round-trip the same keys and `ts` form on every OS.
quote: every event kind round-trips `{"v":1,"ts","instance","kind","source","data"}` (`crates/viola-state/tests/`).
@carried n=12 run=2 label=test-plan/integration-test-strategy fact=fct-bt3npe7x
quote: `ts` is RFC 3339 UTC with ms and `Z`
@carried n=12 run=3 label=test-plan fact=fct-zpi22xot
quote: **On-disk:** every event kind round-trips the event line, `ts` is RFC 3339 UTC with ms, tailing ignores symlinks and invalid names
@carried n=13 label=test-plan/integration-test-strategy fact=fct-bt3npe7x
why: A one-OS strict-modes case must carry its named reason where it stands, and a three-OS refusal must stay read on all three runners.
quote: `tests/cli_links.rs` `link_verbs_refuse_a_widened_instance_before_any_log` (Unix) reads the mode after the refusal, because opening a role file narrows an existing home to 0700 and a check placed after it would pass a 0770 home.
@carried n=13 run=2 label=test-plan/integration-test-strategy fact=fct-bt3npe7x
quote: a home made before viola that another user may read (Unix 0750, Windows a Users read grant) refuses `viola run` on all 3 OSes
@carried n=14 label=test-plan/integration-test-strategy fact=fct-lzdm6e7w
why: The per-OS gating must keep `viola verify`'s relied-row literals per OS under `cfg(unix)` / `cfg(windows)`, the Unix count 24 and the Windows count 21, never one OS's literal forced onto another.
quote: `MM` the relied count (`LedgerRow::relied()`: 24 on Unix, the last `[24/24] local-command-compact …`; 21 on Windows, the last `[21/21] local-command-compact …`; `STEPS_PASS` and `ROW_IDS` are `[&str; 24]` on Unix and `[&str; 21]` on Windows, `STATUSLINE_ROWS` the three statusline ids, the per-OS literals under `cfg(unix)` / `cfg(windows)`)
@carried n=15 label=test-plan/integration-test-strategy fact=fct-lzdm6e7w
why: A per-OS oracle and a `cfg(unix)` table are the form a one-OS test must take, its reason stated where it stands.
quote: `tests/cli_verify.rs` `verify_under_a_regular_file_refuses_the_home` has a per-OS oracle: on Unix a home under a regular file cannot be statted and is refused with exit 1 and the home pair; on Windows the same stat reads `NotFound`, an absent home the check skips, so the home's creation fails at the `cli` catch site's one line.
@carried n=15 run=2 label=test-plan/integration-test-strategy fact=fct-lzdm6e7w
quote: and the stop table `verify_stopped_by_a_signal_removes_only_its_own_dirs_and_exits_1` (chunk 2026-10-13-interrupted-verify-cleanup, `cfg(unix)`, one rstest table of two cases, `sigint` and `sigterm`)
@carried n=16 label=test-plan/integration-test-strategy fact=fct-lzdm6e7w
why: The v1-40 witness must read SGR dim under the outer PTY through the shared SGR parser in `tests/support/sgr.rs`, parsing `ESC [ params m` parameters and never the screen text.
quote: `viola --help` under the outer PTY carries no SGR whose parameters hold bold `1` or underline `4` (the `ESC [ params m` parameters are parsed, never the screen text — ConPTY adds its own `ESC[m`), and on Unix no CR that is not directly followed by LF; one case self-checks that parser against literals.
@carried n=16 run=2 label=test-plan/integration-test-strategy fact=fct-lzdm6e7w
quote: The SGR parser `sgr_attributes`, with `redraws` and `sets_a_foreground`, lives in `tests/support/sgr.rs`; its self-check case stays in `tests/cli_output_plain.rs`.
@carried n=17 label=test-plan/integration-test-strategy fact=fct-lzdm6e7w
why: The v1-40 integration-tier witness of a stale `viola list` row lands beside the board's literal pins and their `board_row_*` units, which it must not displace.
quote: The `viola list` board is pinned by literals, not by a trycmd case (chunk 2026-10-11-the-board-viola-list): `tests/cli_list.rs` `list_human_board_is_the_literal_rows` (the BAY line, the caption row NAME · LIVE · STATUS · WHEEL · DIALOG · CLI as the `CAPTION` literal, the rows) and `list_of_a_stopped_wrapper_is_an_empty_board`, backed by the `src/human/board/` units `bay_line_*` and `board_row_*`
@carried n=18 label=test-plan/integration-test-strategy fact=fct-lzdm6e7w
why: The stale wrapper the v1-40 witness needs on Unix is the `Frozen` SIGSTOP holder, the existing way a root test makes a heartbeat older than 5 s.
quote: `Frozen` (`tests/cli_instance_state.rs`) is a guard on Windows and on Unix, a `SIGSTOP` on Unix; `run_refuses_a_stale_name` and `revive_json_of_a_stale_holder_is_the_stale_heartbeat_document` stand on it.
@carried n=19 label=test-plan/integration-test-strategy fact=fct-lzdm6e7w
why: Every `--json` document must hold the same keys and closed words on every OS, and the CLI suites that pin a document per exit code are where that is read on the Unix runners.
quote: root `tests/cli_*.rs` covers the `--json` document per exit code
@carried n=19 run=2 label=test-plan/integration-test-strategy fact=fct-lzdm6e7w
quote: `tests/cli_machine_contract.rs` reads each driver-facing verb's `--json` document and literal exit over one live wrapper's life
@carried n=19 run=3 label=test-plan fact=fct-w5xcngvx
quote: **CLI:** root `tests/cli_*.rs` cover the `--json` document per exit code
@binding n=1
**integration-test-strategy (Module ↔ PTY) ↔ e2e-test-strategy (tui outer-PTY driver)**: the openpty spawn boundary (test-plan/integration-test-strategy fct-vmzc7an2) and the outer-PTY driver on Unix (test-plan fct-ktlsbc3h) bind together: exit only via `child.wait()`, the master reader drained on its own thread, and the forced vt100 panic case's byte search, which on Unix runs in the `outside_scan()` home outside G2 and G4 (test-plan/integration-test-strategy fct-qmrx4mno).
@binding n=2
**integration-test-strategy (stale `viola list` row) ↔ e2e-test-strategy, quality-gates-coverage-targets**: the v1-40 witness reads SGR dim through `tests/support/sgr.rs` (test-plan/integration-test-strategy fct-lzdm6e7w) beside a `Frozen` stale holder (test-plan/integration-test-strategy fct-lzdm6e7w), staleness resting on the 1 s heartbeat and 5 s threshold (test-plan fct-w7y4wg56), with TTY rendering asserted only on the SGR of verb output (test-plan fct-gnzxb6ex), and the wait past 5 s held inside the mutants profile's 10 s kill (task item 11).
@binding n=3
**integration-test-strategy (wrapper channel macOS carve-out) ↔ e2e-test-strategy (exit-cause matrix, security sweep)**: the server-identity negatives compiled out on macOS (test-plan/integration-test-strategy fct-ksdwqwkf) bind with the exit-cause matrix's same carve-out (test-plan fct-ndx7lm2p) and the hook rows run on Linux and Windows only (test-plan fct-4aawmv2h); the socket directory refusals run on Linux and macOS (test-plan/integration-test-strategy fct-ouc4a2b7).
@binding n=4
**integration-test-strategy (on-disk, concurrent append, `--json`) ↔ e2e-test-strategy (schema conformance) and the body's log format**: the event line round-trip (test-plan/integration-test-strategy fct-bt3npe7x), the whole-line concurrent appends (test-plan/integration-test-strategy fct-hctsd5xl) and the `--json` document per exit code (test-plan/integration-test-strategy fct-lzdm6e7w) bind with G4's validation of every home-level line against `schemas/diag-line.v1.json` (test-plan fct-nfrombzn) and the event line shape (test-plan fct-yp3slbux), the same keys and closed words on every OS.
@binding n=5
**integration-test-strategy (one-OS tests) ↔ quality-gates-coverage-targets (per-OS coverage gate)**: each test moved onto or kept off the Unix runners, with its named reason (test-plan/integration-test-strategy fct-lzdm6e7w), changes what that OS job counts against lines 85, functions 95, regions 80 (test-plan fct-ryyndnld), and a macOS reading comes only from macos-latest (test-plan fct-lktzdsoq).
@binding n=6
**integration-test-strategy (hook exec) ↔ the exit-code contract**: every Unix-run hook fail-open case (test-plan/integration-test-strategy fct-aajqyzhy) asserts exit 0 with no body and no stderr (test-plan fct-hyr7b53t), and the typed exit set gains no code.
