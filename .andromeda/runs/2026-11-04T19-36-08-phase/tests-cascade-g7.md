@carried n=1 label=test-plan/test-data-fixtures fact=fct-bxlupfu3
why: every test the chunk moves onto ubuntu-latest and macos-latest that needs a viola-made home must start from stamped_home, because a home made with create_dir_all is refused on Unix by the read rule.
quote: A home made with `create_dir_all` before viola is refused: on Unix (0755) by the read rule; on windows-2025 `Writable`, the refused entry an inherited write grant to a SID strict-modes does not trust, read by its right whichever SID holds it (`strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry`, as measured at ci#40439188053); that home's canonical SDDL is not recorded. A case that needs a home viola created on every OS starts from `stamped_home`, as the every-OS `hook_statusline` cases that run the arm's whole course do.
@carried n=1 run=2 label=test-plan fact=fct-54gkcbby
quote: a case that needs a home viola created starts from `stamped_home`, and a test that makes a home itself makes it through `viola_state::fs::create_private_dir`, never `create_dir_all`: a plain home is one the product refuses (on Unix by the read rule
@carried n=2 label=test-plan/test-data-fixtures fact=fct-ubb4ig4c
why: a case the parity suite runs on all three OSes whose behaviour diverges per OS must hold a per-OS oracle rather than stay compiled to one OS, as this home-under-a-regular-file case does.
quote: A home planted under a regular file takes a per-OS oracle: on Unix it is a strict-modes refusal, while on Windows such a path stats `NotFound`, the check skips it as absent and the catch-site line stands (`verify_under_a_regular_file_refuses_the_home`).
@carried n=3 label=test-plan/test-data-fixtures fact=fct-652w7iir
why: the statusline pass-through runs on Unix only, and its cases must plant the source by test and set the default home per OS on that one child.
quote: Written by the test, never by viola, which only reads it: `tests/support/home.rs` `plant_statusline_source(home, command)`, with the command from `statusline_echo_command(marker, extra)` and the marker at `statusline_marker(home)`. A default-home case takes `TestHome::default_of_user` and `Wrapper::boot_as_user`, which sets `HOME` (on Windows `USERPROFILE`) on that one child.
@carried n=3 run=2 label=test-plan fact=fct-iwzuwruj
quote: written by the test, never by viola, which only reads it.
@carried n=4 label=test-plan/test-data-fixtures fact=fct-jfjmhkwu
why: every Unix test that boots on a test-owned socket root must read the per-OS root (TMPDIR on macOS, XDG_RUNTIME_DIR on Linux) through tests/support/home.rs, and its macOS reading counts only from the macos-latest runner.
quote: `tests/support/home.rs`, under `/tmp`: `SOCKET_ROOT_VAR` (`TMPDIR` on macOS, else `XDG_RUNTIME_DIR`), `SocketRoot`, `Wrapper::boot_on_socket_root` and `first_life_on`, the last there and not in `tests/support/start.rs`
@carried n=4 run=2 label=test-plan fact=fct-lktzdsoq
quote: NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
@carried n=5 label=test-plan/test-data-fixtures fact=fct-t5v77dkt
why: the Windows DACL vs Unix modes OS-branch tests the chunk owes must widen the real home per OS through these helpers, never through viola or a stub.
quote: Set by the test on the real file, never by viola: `tests/support/home.rs` `widen` · `grant_read` · `unwiden` · `still_widened` (Unix modes; on Windows `icacls`, its exit asserted).
@carried n=5 run=2 label=test-plan/mocking-stubbing-discipline fact=fct-zsbxg2jw
quote: A widened or read-granted home tree is made on the real file by the §7 widened-test-home helpers and refused by the product's own check, never a stubbed one
@carried n=6 label=test-plan/test-data-fixtures fact=fct-2mxqmdgt
why: the Unix socket directory refusals the chunk names on Linux and macOS must stay set on the real directory and read back as found, a foreign owner reached only by an injected uid.
quote: Set by the test on the real directory, never by viola, whose check never chmods it: `tests/security_negatives_socket_dir.rs` `Form` (`apply` · `assert_as_found` · `restore`), a directory at 0755 or a symlink to 0700, the refused directory asserted as found. A foreign owner is not driven: `socket_dir_verdict` takes it by an injected uid.
@carried n=6 run=2 label=test-plan/mocking-stubbing-discipline fact=fct-zsbxg2jw
quote: a loosened socket directory is made on the real file too, at 0755 or as a symlink to 0700 (`Form` in `tests/security_negatives_socket_dir.rs`), refused by the product's own check and read back as found, and the pure `socket_dir_verdict` takes a foreign owner's uid by injection at the unit tier
@carried n=6 run=3 label=test-plan fact=fct-hkmaq6ye
quote: Filesystem: not mocked. A real tempfile home is used everywhere, because modes, `File::lock`, torn lines and symlink handling are the behaviour under test.
@carried n=7 label=test-plan/test-data-fixtures fact=fct-g6w3aucc
why: the v1-40 witness runs a viola list beside a stale wrapper on Unix, and that list must run under a test-scoped PATH holding the fake agent or no claude, never the real CLI.
quote: every test-started `list` / `revive` runs under a test-scoped `PATH` (`support::cli::spawn_on` / `viola_on`, or a plain `spawn` of `list` under an empty bin dir)
@carried n=7 run=2 label=test-plan fact=fct-ew5v6fx5
quote: `viola-fake-agent` placed as `claude` in a test-scoped bin dir, answering from files beside its exe with no env var; every test-started `list` / `revive` runs under a test-scoped `PATH`.
@carried n=7 run=3 label=test-plan fact=fct-egysilpo
quote: NEVER treat a green fake-agent run as proof of real-CLI behaviour.
@carried n=8 label=test-plan/test-data-fixtures fact=fct-6wjlyq3w
why: the exit-code contract the chunk re-asserts in every Unix-run test of the exit-cause matrix, and the socket-directory refusal on Linux and macOS, must read their codes and hint lines as literals in their test files.
quote: and the exit-cause table's codes, exits and last `hint:` lines (`tests/cross_exit_causes.rs` `CAUSES`), and the socket-directory refusal's last `hint:` line (`tests/cross_exit_causes.rs` `SOCKET_DIR`, `tests/security_negatives_socket_dir.rs` `HINT`), are literals in their test files
@carried n=9 label=test-plan/test-data-fixtures fact=fct-rgx56qfp
why: the macOS carve-out the chunk keeps requires the impostor cases to run on Linux and Windows only, since macOS reports no peer pid to compare.
quote: The server-identity impostor cases (Linux and Windows: `served` in `tests/security_negatives_server_identity.rs`, `Impostor::serve` in `tests/hook_fail_open.rs`) write an `InstanceSnapshot` themselves, recording a held live process's pid (the fake agent's holder, below) beside an endpoint the test binds in a private directory
@carried n=9 run=2 label=test-plan/mocking-stubbing-discipline fact=fct-pwscsozw
quote: in the server-verification negatives (Linux and Windows) an impostor, a server on the recorded endpoint while the snapshot records another held live process (Process mocking), stands in for a server that is not the recorded wrapper
@carried n=9 run=3 label=test-plan/test-data-fixtures fact=fct-vrevms4b
quote: The `--exit-no-eof` holder's re-exec mode `--hold-stdout-internal` (it sleeps 10 s and writes nothing) is also spawned directly by the server-identity impostor cases (`tests/security_negatives_server_identity.rs`, `tests/hook_fail_open.rs`) as the live process whose pid the impostor's snapshot records, killed and reaped by each file's `Held` `Drop` guard.
@carried n=10 label=test-plan/test-data-fixtures fact=fct-fvd26vjh
why: a test the chunk moves onto Unix runners that binds its own endpoint meets the socket-directory check there, so its endpoint directory must be made private or the case is refused ahead of what it reads.
quote: A directory holding a test's endpoint, where the case needs it to pass the socket-directory check, is made through `viola_state::fs::create_private_dir`, never left a bare `tempfile::tempdir()`: `tempfile` 3.27 leaves the mode to the umask (0755 as measured at chunk 2026-11-01-unix-endpoint-and-home-hardening), and such a directory meets `check_endpoint_dir` as a home meets the read rule.
@carried n=10 run=2 label=test-plan/mocking-stubbing-discipline fact=fct-pwscsozw
quote: a test server's endpoint directory is made through `viola_state::fs::create_private_dir`, never a bare `tempfile::tempdir()`, whose mode `tempfile` 3.27 leaves to the umask (0755 on a default one) and which the client's socket-directory check refuses ahead of the server check
@carried n=11 label=test-plan/test-data-fixtures fact=fct-q4qwgsv5
why: a test the chunk moves onto Linux runners that plants several fake-agent copies must plant them all before any child starts, or it meets ETXTBSY on the tmpfs home.
quote: A test that plants more than one fake-agent copy plants every copy (each recorded set's bin) one after another before any thread starts a child: on the tmpfs `target/e2e-home` the hard link fails, so each copy is written, and a child one thread forks inherits another thread's open write handle (ETXTBSY).
@carried n=12 label=test-plan/test-data-fixtures fact=fct-mw3qahin
why: the openpty resize reading the chunk owes (a key written after a resize only once the child reports the new size) must read the fake agent's size receipt as its oracle.
quote: `size` {`cols`, `rows`}: at start and again whenever the terminal size changed (the resize oracle) — in interactive mode a watcher thread writes the size at once, then polls it every 10 ms and writes on every change, so a resize reaches the receipt with no key after it; nothing while stdin/stdout is not a terminal, and print mode writes none
@carried n=13 label=test-plan/test-data-fixtures fact=fct-vrevms4b
why: the Unix no-EOF child exit and the child-ends-with-its-killed-wrapper reading run the holder on openpty, where it and every command the agent spawns sit in a process group of their own, and exit is read only on the handle.
quote: `--exit-no-eof` (a bounded re-exec holder keeps the child's console / PTY slave open for 10 s — not viola's stdout — writes a `hold` receipt, and on Unix spawns in its own process group)
@carried n=13 run=2 label=test-plan fact=fct-7o4nbra2
quote: on Unix every command it spawns (each hook, and the status line command) runs in a process group of its own (`process_group(0)`, as the `--exit-no-eof` holder does): its PTY's foreground group is what a hangup of that terminal reaches, at the agent's exit as its session leader or when a killed wrapper closes the PTY
@carried n=13 run=3 label=test-plan fact=fct-ukay3vyw
quote: NEVER detect child exit by reading master EOF; use `child.wait()` on the handle (ConPTY).
@carried n=14 label=test-plan/test-data-fixtures fact=fct-vrevms4b
why: the forced vt100 panic chaos case the chunk reads on Unix by byte search drives these measured bytes, needing no new env seam.
quote: `--vt100-panic-bytes` is built (with confirmed `send`'s chaos case): after its `start` receipt it writes `e4 b8 ad` once to its terminal and receipts nothing new. Its bytes are measured: under a 24×1 PTY a wide character (`e4 b8 ad`) panics vt100 0.16.2, so no new env seam is needed (as measured at chunk 2026-10-04-readiness-gate-and-timing-constants).
@carried n=15 label=test-plan/test-data-fixtures fact=fct-hc6txkos
why: G4 re-reads diag-line.v1.json as one process-log schema on every OS, so this closed detail word must stand in the same enum on every OS.
quote: its `process-exit` reads `detail` `unclean-fixture`, a member of the `process-exit` `detail` enum in `schemas/diag-line.v1.json`
@carried n=16 label=test-plan/mocking-stubbing-discipline fact=fct-je5xeiyl
why: the v1-40 witness's staleness rests on the 1 s heartbeat and 5 s threshold, which a unit case must drive through the injected clock while the integration witness waits it out for real in a spawned wrapper, its verdict the product's own.
quote: Time: sync crates use mock_instant 0.6.1 (global mode) or the injected `viola_core::Clock` for heartbeat 1 s / 5 s and the send confirmation window.
@carried n=16 run=2 label=test-plan/mocking-stubbing-discipline fact=fct-pskdyofh
quote: Inject the `Clock` dependency through constructors and call `MockClock::advance(Duration::from_secs(6))`
@carried n=16 run=3 label=test-plan fact=fct-v6in7u4x
quote: NEVER use mock_instant `MockClock` (or any in-process clock) to control another process's time. Global mode reaches only threads of the test process, never a spawned `viola run`.
@carried n=16 run=4 label=test-plan fact=fct-45jodiii
quote: E2E may block on a deadline inside the product that it cannot inject: the send confirmation window behind Path 2's `no-prompt-submitted`, `wait --timeout-ms` in Path 3, and heartbeat staleness in the chaos SIGSTOP case. Two conditions apply: the verdict is the product's own exit code or status field, and the same deadline is clock-driven in a §4 unit case.
@carried n=17 label=test-plan/mocking-stubbing-discipline fact=fct-sik7tvk4
why: the openpty backend sits behind the viola-pty seam, so its consumers keep a mockall double of the Pty trait while the Module to PTY boundary runs over real fake-agent spawns.
quote: mockall 0.15.0 on the `viola-pty` `Pty` trait and the liveness / `Clock` traits; native stub binaries (`viola-fake-agent`)
@carried n=17 run=2 label=test-plan fact=fct-76bthv7a
quote: mockall `automock` on seam traits only; external processes are real stub executables (`viola-fake-agent`).
@carried n=17 run=3 label=test-plan fact=fct-dlsufsua
quote: The wrapper channel, PTY, vt100 or serde in E2E. Code paths the agent must verify end-to-end go in E2E (§6).
@carried n=18 label=test-plan/mocking-stubbing-discipline fact=fct-63urz52n
why: the v1-40 witness's stale wrapper and the Unix killed-wrapper reading must be real process faults (a frozen real wrapper, a real kill), never a double.
quote: A stale holder is a real wrapper under the `Frozen` guard (`tests/cli_instance_state.rs`; on Unix a `SIGSTOP`), which `run_refuses_a_stale_name` stands on; a wrapper lost under an open call is a real kill, `support::start::kill_wrapper` (`tests/support/start.rs`) for the lost row of `tests/cross_exit_causes.rs` and for `tests/chaos_wait_vanish.rs`
@carried n=19 label=test-plan/mocking-stubbing-discipline fact=fct-6xaoh2kl
why: the scope forbids any env, flag or feature seam forcing a refusing peer-euid decision, so any Unix test the chunk adds must hold that decision only through these injected arguments.
quote: `peer_euid_admitted(observed, own, reported)` takes its euids as arguments and the private `serve_admitting` takes its decision as an `Admission` argument (`crates/viola-channel/src/server.rs`): dependency injection, with no env, flag or feature seam
@carried n=19 run=2 label=test-plan fact=fct-3a3m4jll
quote: prefer dependency injection over runtime patching of production code.
@binding n=1
**test-data-fixtures (loosened socket directory) ↔ mocking-stubbing-discipline (filesystem faults)**: the Linux and macOS socket-directory negatives must make the fault on the real directory through `Form` and read it back as found, refused by the product's own check, a foreign owner reached only by an injected uid at the unit tier (test-plan/test-data-fixtures fct-2mxqmdgt) (test-plan/mocking-stubbing-discipline fct-zsbxg2jw).
@binding n=2
**test-data-fixtures (impostor cases, held process) ↔ mocking-stubbing-discipline (fake servers, process mocking)**: the server-verification negatives must stay Linux and Windows only under the macOS carve-out, the impostor's recorded pid a real held process (`--hold-stdout-internal`), never a double (test-plan/test-data-fixtures fct-rgx56qfp) (test-plan/mocking-stubbing-discipline fct-pwscsozw) (test-plan/mocking-stubbing-discipline fct-sik7tvk4).
@binding n=3
**mocking-stubbing-discipline (time) ↔ mocking-stubbing-discipline (process faults)**: the v1-40 integration witness must make its stale wrapper as a real process fault and wait out the real 5 s threshold in a spawned wrapper, its verdict the product's own output, while the same threshold stays clock-driven in a unit case; no in-process clock reaches the wrapper (test-plan/mocking-stubbing-discipline fct-je5xeiyl) (test-plan/mocking-stubbing-discipline fct-63urz52n) (test-plan fct-45jodiii).
@binding n=4
**test-data-fixtures (endpoint directory) ↔ mocking-stubbing-discipline (fake servers)**: any test moved onto Unix runners that binds a test server makes its endpoint directory through `create_private_dir`, so the client's socket-directory check does not refuse it ahead of the server check (test-plan/test-data-fixtures fct-fvd26vjh) (test-plan/mocking-stubbing-discipline fct-pwscsozw).
@binding n=5
**data-fake-agent (size receipt, no-EOF holder) ↔ mocking-stubbing-discipline (process mocking)**: the openpty seam's consumers keep the mockall Pty double, while the Module to PTY boundary reads real openpty spawns of the fake agent: a resize through its size receipt, a no-EOF exit on the handle with the holder in its own process group (test-plan/test-data-fixtures fct-mw3qahin) (test-plan/test-data-fixtures fct-vrevms4b) (test-plan/mocking-stubbing-discipline fct-sik7tvk4).
@binding n=6
**test-data-fixtures (on-disk product state) ↔ test-data-fixtures (per-OS oracle)**: a test moved to every OS starts from `stamped_home`, and where the OSes diverge it holds a per-OS oracle instead of staying compiled to one OS (test-plan/test-data-fixtures fct-bxlupfu3) (test-plan/test-data-fixtures fct-ubb4ig4c).
