@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `.andromeda/architecture.md` §Standard Contracts (Wrapper 
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · 
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=39
last: - `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=47
last: - `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas
@leaf_read leaf=.claude/docs/services/viola-state.md lines=49
last: - `.andromeda/architecture.md` §Standard Contracts (snapshot
@leaf_edit n=1 leaf=.claude/docs/services/viola-channel.md line=6 class=retired
leaf-text: (named pipe `\\.\pipe\viola-<h12>` on Windows; `<per-user 0700 dir>/viola-<h12>.sock` on Unix)
source: fct-wular7qm «Unix domain socket file `<socket dir>/viola-<h12>.sock` in a per-user 0700 directory (`$XDG_RUNTIME_DIR/viola/` or `/tmp/viola-<uid>/`), never the `/tmp` root»
replace-with: (the Unix socket `<socket dir>/viola-<h12>.sock` in a per-user 0700 directory, `$XDG_RUNTIME_DIR/viola/` or `/tmp/viola-<uid>/`, never the `/tmp` root)
@leaf_edit n=2 leaf=.claude/docs/services/viola-channel.md line=6 class=retired
leaf-text: the per-OS listener hardening
source: fct-igbvtfv6 «the Windows and macOS code is out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`)»
replace-with: the Unix listener hardening
@leaf_edit n=3 leaf=.claude/docs/services/viola-channel.md line=6 class=retired
leaf-text: (`peer_euid_admitted`, `peer_admitted`, `PEER_EUID_REPORTED`, the last `cfg!(unix)`; a peer whose euid cannot be read is refused where it is reported)
source: fct-wular7qm «both sides admit only a peer of the owning user's euid, the wrapper dropping another before any frame»
replace-with: (`peer_euid_admitted`, `peer_admitted`; a peer whose euid cannot be read is refused)
@leaf_edit n=4 leaf=.claude/docs/services/viola-channel.md line=6 class=retired
leaf-text: (`Client::server_pid` / `TokioClient::server_pid`; `SERVER_PID_REPORTED` false on macOS)
source: fct-igbvtfv6 «the Windows and macOS code is out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`)»
replace-with: (`Client::server_pid` / `TokioClient::server_pid`)
@leaf_edit n=5 leaf=.claude/docs/services/viola-channel.md line=11 class=retired
leaf-text: windows-sys 0.61.2 (SID lookup, SDDL conversion via `Win32_Security_Authorization`, SQOS `CreateFileW`); libc (Unix: the uid for the socket dir)
source: fct-igbvtfv6 «the Windows and macOS code is out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`)»
replace-with: libc (the uid for the socket dir)
@leaf_edit n=6 leaf=.claude/docs/services/viola-channel.md line=14 class=retired
leaf-text: (`pub mod test_support`: `JsonFields`; on Windows `canonical_sddl`, `dacl_of`, an independent `user_sid`)
source: fct-4whhx2hw «`viola-channel` `test-support = []` gates `pub mod test_support` (the `JsonFields` tracing field visitor).»
replace-with: (`pub mod test_support`: `JsonFields`, the tracing field visitor)
@leaf_edit n=7 leaf=.claude/docs/services/viola-channel.md line=22 class=retired
leaf-text: Windows listener: protected SDDL `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)` converted by windows-sys (read back canonical as `(A;;FA;;;…)`, the SID possibly an alias), `accept_remote(false)`, first-instance, `inheritable(false)`. Unix: 0700 per-user dir
source: fct-wular7qm «Unix domain socket file `<socket dir>/viola-<h12>.sock` in a per-user 0700 directory (`$XDG_RUNTIME_DIR/viola/` or `/tmp/viola-<uid>/`), never the `/tmp` root»
replace-with: Listener: 0700 per-user dir (`$XDG_RUNTIME_DIR/viola/` or `/tmp/viola-<uid>/`, never the `/tmp` root)
@leaf_edit n=8 leaf=.claude/docs/services/viola-channel.md line=22 class=retired
leaf-text: socket chmod 0600 after the bind (never `ListenerOptionsExt::mode`: `Unsupported` on macOS fails the bind)
source: fct-wular7qm «the socket is chmod 0600 after the bind»
replace-with: socket chmod 0600 after the bind (never `ListenerOptionsExt::mode`)
@leaf_edit n=9 leaf=.claude/docs/services/viola-channel.md line=25 class=retired
leaf-text: Client: open (Windows SQOS Identification); `Client::server_pid`
source: fct-igbvtfv6 «the Windows and macOS code is out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`)»
replace-with: Client: `Client::server_pid`
@leaf_edit n=10 leaf=.claude/docs/services/viola-channel.md line=25 class=retired
leaf-text: (Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the SQOS-adopted client end, none on macOS, where `SERVER_PID_REPORTED` is false)
source: fct-igbvtfv6 «the Windows and macOS code is out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`)»
replace-with: (Linux `SO_PEERCRED`)
@leaf_edit n=11 leaf=.claude/docs/services/viola-channel.md line=25 class=retired
leaf-text: macOS reports no peer pid and stays the recorded gap, owned by no route entry (`server_verdict` returns `Matches` with no comparison); its client euid and socket-directory checks stand built, so on macOS `server-verify-failed` means an euid mismatch alone.
source: fct-ld365mzq «the connected peer's pid and its OS start time against these two fields, a mismatch writing no frame.»
replace-with: A peer pid not reported is a check that cannot be made (`server_verdict` → `Unverifiable`), so no frame is written.
@leaf_edit n=12 leaf=.claude/docs/services/viola-channel.md line=26 class=retired
leaf-text: takes the pipe-busy deadline
source: fct-igbvtfv6 «the Windows and macOS code is out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`)»
replace-with: takes a busy deadline, unused (a Unix socket has no busy state)
@leaf_edit n=13 leaf=.claude/docs/services/viola-channel.md line=31 class=retired
leaf-text: macOS socket paths cap at ~104 bytes; the recorded snapshot
source: fct-wular7qm «every other process connects to the `endpoint` recorded in the instance snapshot»
replace-with: The recorded snapshot
@leaf_edit n=14 leaf=.claude/docs/services/viola-channel.md line=32 class=tree
leaf-text: **SQOS spike passed** (security-plan Decisions Log `2026-09-25`): the client opens with windows-sys `CreateFileW(SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION | FILE_FLAG_OVERLAPPED)` and adopts via `Stream::try_from` — reuse the recipe `tests/channel_sqos_open.rs` pins. Keep `FILE_FLAG_OVERLAPPED` (non-overlapped adoption hangs) and never fall back to the default connect (the server then reads `SecurityImpersonation`). A safe-Rust equivalent was measured: std `OpenOptions::security_qos_flags` + `custom_flags(FILE_FLAG_OVERLAPPED)`.
source: fct-wular7qm «Linux abstract-namespace names are never used, because they carry no filesystem permissions and any local user could connect.»
replace-with: Linux abstract-namespace names are never used: they carry no filesystem permissions and any local user could connect.
@leaf_edit n=15 leaf=.claude/docs/services/viola-channel.md line=35 class=retired
leaf-text: `frame::framed`, and on Windows `client::open_handle`)
source: fct-igbvtfv6 «the Windows and macOS code is out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`)»
replace-with: `frame::framed`)
@leaf_edit n=16 leaf=.claude/docs/services/viola-channel.md line=36 class=retired
leaf-text: **Per-OS listener + client verification:** `#[cfg(windows)]` / `#[cfg(unix)]` modules
source: fct-igbvtfv6 «the Windows and macOS code is out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`)»
replace-with: **Listener + client verification:** `crates/viola-channel/src/server.rs` / `client.rs`
@leaf_edit n=17 leaf=.claude/docs/services/viola-channel.md line=41 class=retired
leaf-text: SDDL / 0700 / 0600 read-back
source: fct-wular7qm «Unix domain socket file `<socket dir>/viola-<h12>.sock` in a per-user 0700 directory (`$XDG_RUNTIME_DIR/viola/` or `/tmp/viola-<uid>/`), never the `/tmp` root»
replace-with: 0700 / 0600 read-back
@leaf_edit n=18 leaf=.claude/docs/services/viola-mcp.md line=14 class=retired
leaf-text: (as measured on claude 2.1.287 on the Linux dev host; no capability-ledger row; its Windows leg owed to "Windows-only live measurements")
source: fct-igbvtfv6 «the Windows and macOS code is out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`)»
replace-with: (as measured on claude 2.1.287 on the Linux dev host; no capability-ledger row)
@leaf_edit n=19 leaf=.claude/docs/services/viola-mcp.md line=22 class=retired
leaf-text: , no frame written and no log line (macOS reports no peer pid and holds the euid check alone, the named carve-out; Windows reads no euid, its DACL and the pid comparison the controls)
source: fct-ld365mzq «the connected peer's pid and its OS start time against these two fields, a mismatch writing no frame.»
replace-with: , no frame written and no log line
@leaf_edit n=20 leaf=.claude/docs/services/viola-mcp.md line=23 class=retired
leaf-text: (as measured on claude 2.1.287 on the Linux dev host; its Windows leg owed to "Windows-only live measurements")
source: fct-igbvtfv6 «the Windows and macOS code is out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`)»
replace-with: (as measured on claude 2.1.287 on the Linux dev host)
@leaf_edit n=21 leaf=.claude/docs/services/viola-mcp.md line=27 class=tree
leaf-text: (as measured live, `viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup-ii/evidence/live-tap.md`)
source: fct-kyb4p5xt «and the `Mcp.` exchanges the test-only `viola-mcp-tap` records, replayed against `viola mcp`»
replace-with: (as measured live; the exchange recorded as `fixtures/claude/2.1.302/Mcp.tools-list-1.json`)
@leaf_edit n=22 leaf=.claude/docs/services/viola-pty.md line=6 class=retired
leaf-text: in ConPTY on Windows and openpty on Unix. On Windows its `sideload` module (`restrict_dll_search`, `search_restricted`, `preload`) lets the root bin host `viola run`'s child in Microsoft's sideloaded ConPTY: `restrict_dll_search()` (`SetDefaultDllDirectories(LOAD_LIBRARY_SEARCH_SYSTEM32)`, process-wide, one-way) and `preload(&Path)` (`LoadLibraryExW` by absolute path; a relative path is refused; the module is never freed), after which portable-pty's own bare-name `LoadLibraryW("conpty.dll")` returns that module.
source: fct-omwrdnlc «Child exit is detected on the process handle, never on EOF; `pty_backend()` is `openpty`; viola never holds, queues or re-orders a human key behind a resize and never answers DA1 itself.»
replace-with: in an openpty (`pty_backend()` is `openpty`).
@leaf_edit n=23 leaf=.claude/docs/services/viola-pty.md line=6 class=retired
leaf-text: If 0.8.1's Windows `kill()` misbehaves, the fallback is `TerminateProcess` on `child.process_id()` via windows-sys.
source: fct-omwrdnlc «Child exit is detected on the process handle, never on EOF; `pty_backend()` is `openpty`; viola never holds, queues or re-orders a human key behind a resize and never answers DA1 itself.»
replace-with: viola never holds, queues or re-orders a human key behind a resize and never answers DA1 itself.
@leaf_edit n=24 leaf=.claude/docs/services/viola-pty.md line=11 class=retired
leaf-text: windows-sys 0.61.2 (Windows only: kill fallback, console raw/VT modes); libc `=0.2.189` (Unix only: termios raw mode, terminal size)
source: fct-igbvtfv6 «the Windows and macOS code is out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`)»
replace-with: libc `=0.2.189` (termios raw mode, terminal size)
@leaf_edit n=25 leaf=.claude/docs/services/viola-pty.md line=21 class=retired
leaf-text: `CreateProcessW` is called with `bInheritHandles = 0`: channel handles never leak into the child. Any replacement (the named candidate is portable-pty-psmux 0.9.7) must keep this.
source: fct-igbvtfv6 «the Windows and macOS code is out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`)»
replace-with: The Windows and macOS PTY code (ConPTY, the sideload, the `TerminateProcess` kill fallback) is out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`).
@leaf_edit n=26 leaf=.claude/docs/services/viola-pty.md line=22 class=retired
leaf-text: on Windows with a console stdin, viola's own `ReadConsoleW` reader (UTF-16 → UTF-8, a split surrogate carried, every `0x1A` kept, a 0-unit read read again), because std's console stdin drops a trailing `0x1A` and ends input on a lone `^Z` (measured on windows-2025); elsewhere `std::io::stdin()`
source: fct-igbvtfv6 «the Windows and macOS code is out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`)»
replace-with: `std::io::stdin()`
@leaf_edit n=27 leaf=.claude/docs/services/viola-pty.md line=23 class=stale
leaf-text: (`viola_pty::pty_backend()`: `conpty-sideload` after a successful pre-load, else `conpty` on Windows, `openpty` elsewhere)
source: fct-omwrdnlc «Child exit is detected on the process handle, never on EOF; `pty_backend()` is `openpty`; viola never holds, queues or re-orders a human key behind a resize and never answers DA1 itself.»
replace-with: (`viola_pty::pty_backend()`: `openpty`)
@leaf_edit n=28 leaf=.claude/docs/services/viola-pty.md line=26 class=retired
leaf-text: ConPTY does not close the output stream on exit — detect exit only via `wait()`.
source: fct-omwrdnlc «Child exit is detected on the process handle, never on EOF; `pty_backend()` is `openpty`; viola never holds, queues or re-orders a human key behind a resize and never answers DA1 itself.»
replace-with: Detect child exit only via `wait()` on the process handle, never on reader EOF.
@leaf_edit n=29 leaf=.claude/docs/services/viola-pty.md line=27 class=stale
leaf-text: 0.9.0 has the Windows garbage-read bug (wezterm#6783); it is excluded, and the swap trigger is portable-pty-psmux 0.9.7 if the O1 stray-spaces artifact reproduces only under `viola run`.
source: fct-omwrdnlc «moving `portable-pty` off 0.8.1 to a release whose graph drops `pin-utils` (RUSTSEC-2026-0441) is owed, owner: "Version done-check"»
replace-with: Moving `portable-pty` off 0.8.1 to a release whose graph drops `pin-utils` (RUSTSEC-2026-0441) is owed, owner: "Version done-check".
@leaf_edit n=30 leaf=.claude/docs/services/viola-pty.md line=28 class=retired
leaf-text: `.cmd` / `.bat` children are refused before spawn (portable-pty bypasses std's BatBadBut escaping); the caller passes an absolute program path and an explicit cwd (0.8.1 defaults to `%USERPROFILE%`).
source: fct-bvzxxcy7 «viola resolves the child program to an absolute path itself (a given path as given, a bare name searched on `PATH`) and spawns that.»
replace-with: The caller passes an absolute program path (viola resolves it itself: a given path as given, a bare name searched on `PATH`) and an explicit cwd.
@leaf_edit n=31 leaf=.claude/docs/services/viola-pty.md line=29 class=retired
leaf-text: switches the host terminal to raw input + VT output for the run and restores it on Drop — without it ConPTY line-buffers keys until Enter and swallows Ctrl-C.
source: fct-igbvtfv6 «the Windows and macOS code is out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`)»
replace-with: switches the host terminal to termios raw mode for the run and restores it on Drop.
@leaf_edit n=32 leaf=.claude/docs/services/viola-pty.md line=30 class=retired
leaf-text: Keep the ConPTY input writer until the child exits: dropping it makes conhost send `CTRL_CLOSE` (child exit `0xC000013A`).
source: fct-fa2klbw5 «Child processes are spawned directly.»
replace-with: Child processes are spawned directly: no design choice may depend on tmux or a shell.
@leaf_edit n=33 leaf=.claude/docs/services/viola-pty.md line=31 class=retired
leaf-text: H2, measured (chunk `2026-09-29-h2-conpty-resize-probe`): a key written right after `resize` returned was lost on the windows-2025 runner in 13 of 200 isolated iterations, every loss class K — the resize reached the child (its key-free watcher saw `size 120x40`), the key was written and flushed, the Rust test child never read it, and ConPTY wrote no `ESC[6n` (`dsr-cpr 0`). Builds: runner `windows-2025-vs2026` 20260922.246.2 (ci#36527891850); host conhost 10.0.26100.8875 read 20/20 with `dsr-cpr 0`. The loss lies between the ConPTY input-pipe write and that child's console read; its cause is not established, and whether the real `claude` (Node/libuv, `ReadConsoleInputW`) loses such a key is unmeasured — owned by the working-route entry "Windows-only live measurements", blocked on an interactive Windows host (`.claude/docs/gotchas.md`, H2); the harness's `run --local-live` does not claim it. The red test now writes its key after the child's `size 120x40` line: 200/200 on the runner (ci#36529038462). With and without the sideloaded ConPTY in one windows-2025 run (image `windows-2025-vs2026` 20260828.587, ci#36563868040, the measurement-only race): sideloaded 0 of 200 lost, inbox 14 of 200 — no rate claimed.
source: fct-bq65okf4 «OPEN — whether the real `claude` loses a key typed right after a resize · owner: the open residuals of origin `2026-11-04T23-11-43-wrap` (target `another-os`)»
replace-with: OPEN — whether the real `claude` loses a key typed right after a resize · owner: the open residuals of origin `2026-11-04T23-11-43-wrap` (target `another-os`).
@leaf_edit n=34 leaf=.claude/docs/services/viola-pty.md line=32 class=retired
leaf-text: `PtyError::Load` (fixed message `"pty load failed"`) is the pre-load's error. The sideloaded `OpenConsole.exe` writes its own spawn preamble, `ESC[1t ESC[c ESC[?1004h ESC[?9001h`, and holds the child's start until its DA1 query `ESC[c` is answered (a terminal answers; ~3 s when nothing does, measured on pipes) — viola never answers it itself. The crate's own tests, the root `OuterPty` and the harness `supervise::spawn_in_pty` neither restrict nor pre-load, so they stay on the inbox ConPTY.
source: fct-sha7o35z «One ratified exception, `viola-pty`'s hand-written `PtyError`»
replace-with: `PtyError` is hand-written with fixed messages, the one ratified exception to the thiserror rule ([Error Handling]).
@leaf_edit n=35 leaf=.claude/docs/services/viola-pty.md line=37 class=retired
leaf-text: **Kill fallback:** `#[cfg(windows)]` module
source: fct-igbvtfv6 «the Windows and macOS code is out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`)»
replace-with: **Pump (`pump_with_paste`, `PasteHandle`):** `crates/viola-pty/src/pump.rs`
@leaf_edit n=36 leaf=.claude/docs/services/viola-pty.md line=38 class=tree
leaf-text: **Sideload (DLL search restriction + absolute-path pre-load):** `crates/viola-pty/src/sideload.rs` (`cfg(windows)`); unit test `restrict_dll_search_keeps_planted_conpty_out_of_a_bare_name_load` (two-sided)
source: fct-igbvtfv6 «the Windows and macOS code is out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`)»
replace-with: **Host terminal (`HostTerminal`, `host_stdin`, `host_size`, `pty_backend`):** `crates/viola-pty/src/lib.rs`
@leaf_edit n=37 leaf=.claude/docs/services/viola-pty.md line=39 class=tree
leaf-text: through real ConPTY/openpty (no-EOF mode, resize); the `.cmd` refusal in the root `tests/cli_program_resolution.rs`
source: fct-igbvtfv6 «the Windows and macOS code is out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`)»
replace-with: through a real openpty (no-EOF mode, resize)
@leaf_edit n=38 leaf=.claude/docs/services/viola-pty.md line=43 class=stale
leaf-text: real ConPTY / openpty spawn of the fake agent; run on all three CI OSes
source: fct-bvzxxcy7 «GitHub Actions on `ubuntu-latest` (`strategy.matrix.os: [ubuntu-latest]`), tests against the fake agent»
replace-with: real openpty spawn of the fake agent; run in CI on `ubuntu-latest`
@leaf_edit n=39 leaf=.claude/docs/services/viola-pty.md line=44 class=retired
leaf-text: exit on the handle everywhere; the held output outlives the session leader natively on ConPTY and on Linux only for a holder in its own process group; on macOS it ends at the leader's exit (CI run 36166907442), so that half is asserted on Windows + Linux
source: fct-igbvtfv6 «the Windows and macOS code is out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`)»
replace-with: exit on the handle; the held output outlives the session leader on Linux only for a holder in its own process group, and that half is asserted on Linux
@leaf_edit n=40 leaf=.claude/docs/services/viola-state.md line=6 class=retired
leaf-text: with no peer pid reported, as on macOS, `Matches` with no comparison, the recorded gap
source: fct-ld365mzq «the connected peer's pid and its OS start time against these two fields, a mismatch writing no frame.»
replace-with: with no peer pid reported, `Unverifiable`, no frame written
@leaf_edit n=41 leaf=.claude/docs/services/viola-state.md line=6 class=stale
leaf-text: its `events.ndjson` (the write rule on Unix, the read rule on Windows)
source: fct-cicfj3bq «the home, the instance directory and its `diagnostics/` to the read rule (no group or other bit), `instances/` and `snapshot.json` to the write rule, and `events.ndjson` to the write rule»
replace-with: its `events.ndjson` (the write rule)
@leaf_edit n=42 leaf=.claude/docs/services/viola-state.md line=6 class=retired
leaf-text: the home strict-modes check (Unix modes; Windows owner + DACL) with 0700/0600 creation
source: fct-igbvtfv6 «the Windows and macOS code is out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`)»
replace-with: the home strict-modes check (Unix owner + modes) with 0700/0600 creation
@leaf_edit n=43 leaf=.claude/docs/services/viola-state.md line=6 class=retired
leaf-text: (`unix_read_verdict`, `windows_read_verdict`: on Unix no group or other bit, on Windows no allow ACE granting a right in `READ_RIGHTS` to a SID `trusted_sid` rejects)
source: fct-cicfj3bq «the home, the instance directory and its `diagnostics/` to the read rule (no group or other bit), `instances/` and `snapshot.json` to the write rule, and `events.ndjson` to the write rule»
replace-with: (`unix_read_verdict`: no group or other bit)
@leaf_edit n=44 leaf=.claude/docs/services/viola-state.md line=6 class=retired
leaf-text: a refused directory stays as found, and off Unix it returns `Ok(())`
source: fct-igbvtfv6 «the Windows and macOS code is out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`)»
replace-with: a refused directory stays as found
@leaf_edit n=45 leaf=.claude/docs/services/viola-state.md line=11 class=retired
leaf-text: windows-sys (Windows only: the `strict` module's owner + DACL check, and the protected user + SYSTEM DACL `fs::create_private_dir` gives the topmost folder it creates outside the user's profile, `Win32_UI_Shell`); libc (Unix only: the `strict` module's owner + mode check)
source: fct-igbvtfv6 «the Windows and macOS code is out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`)»
replace-with: libc (the `strict` module's owner + mode check)
@leaf_edit n=46 leaf=.claude/docs/services/viola-state.md line=22 class=retired
leaf-text: through `check_instance`, and on Windows x64 the ConPTY companions through `check_present` before the held-handle re-hash); Windows checks cover inherit-only and generic bits, NULL DACL and non-persistent-ACL volumes, and each existing `events.ndjson` before it is opened.
source: fct-cicfj3bq «the home, the instance directory and its `diagnostics/` to the read rule (no group or other bit), `instances/` and `snapshot.json` to the write rule, and `events.ndjson` to the write rule»
replace-with: through `check_instance`); each existing `events.ndjson` is checked before it is opened.
@leaf_edit n=47 leaf=.claude/docs/services/viola-state.md line=23 class=retired
leaf-text: On Windows, `replace_private` retries a `persist` refused with raw OS error 5 (a reader holding the target) every 10 ms, re-persisting the same temp file, at most `fs::REPLACE_ATTEMPTS` = 100 attempts; any other error returns at once. The loop sits in the private `replace_private_retrying`, which takes the retry decision (from the raw OS error) from its caller; `replace_private_with` passes `retry_replace(_, HOST_IS_WINDOWS)`, so the product behaves as above and a unit case exercises the attempt bound on every host. `write_snapshot` and `replace_private_shared` inherit it.
source: fct-2sdvn2pr «`viola_state::fs::replace_private` sets the temp file's mode before any byte is written, then writes, `sync_all`s and `persist`s, a failure leaving the target as it was»
replace-with: `replace_private` sets the temp file's mode before any byte is written, then writes, `sync_all`s and `persist`s, a failure leaving the target as it was; no call site carries its own temp + rename code.
@leaf_edit n=48 leaf=.claude/docs/services/viola-state.md line=24 class=retired
leaf-text: `pin` (Windows, as landed by 2026-09-29-sideloaded-conpty): `Companion { name, bytes, sha256_hex }`, `HeldCompanions { dir, _handles }`, `pin_companions(&Pinned, subdir, &[Companion])` — first the owner/DACL check (`strict::check_present`) on `bin/<version>-<hash>/conpty/` and on each companion that exists (a refusal is `PinError::StrictModes`, writes nothing, and the child runs on the inbox ConPTY with `sideload_fallback` `strict-modes-failed`), then each file written if absent through `replace_private_shared`, then opened with Win32 `FILE_SHARE_READ` only and fully re-hashed; a mismatch is `PinError::HashMismatch` with the file left as found. A held open refused with Win32 error 32 (sharing violation — two concurrent first starts, 5-11 ms measured) is retried under `REPLACE_ATTEMPTS` (`retry_open`, `open_held_waiting`). `pin_key(bytes)`
source: fct-hnee7xdr «which `viola run` places once when it is absent, with no clobber, and never replaces (0700)»
replace-with: `pin`: `pin_key(bytes)`
@leaf_edit n=49 leaf=.claude/docs/services/viola-state.md line=33 class=stale
leaf-text: Append + exclusive lock on the same file fails on Windows — lock the sibling.
source: fct-73drmylz «lock files are separate, a layout chosen while Windows (where append + exclusive lock fails) was a target, and kept.»
replace-with: Lock the `.lock` sibling, never the guarded file: a layout chosen while Windows (where append + exclusive lock fails) was a target, and kept.
@leaf_edit n=50 leaf=.claude/docs/services/viola-state.md line=40 class=retired
leaf-text: crate `tests/` against a real tempfile home; synthetic SDDL descriptors for the Windows cases runners cannot provision
source: fct-igbvtfv6 «the Windows and macOS code is out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`)»
replace-with: crate `tests/` against a real tempfile home
