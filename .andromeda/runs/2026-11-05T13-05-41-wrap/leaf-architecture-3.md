@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/services/viola.md lines=58
last: - `.andromeda/architecture.md` · `.claude/rules/api.md` · `.
@leaf_read leaf=.claude/docs/stack.md lines=66
last: 5. Commit as `chore: bump {tool} to {version}`
@leaf_read leaf=.claude/docs/workflow.md lines=67
last: This project uses a **reference-based** CLAUDE.md architectu
@leaf_edit n=1 leaf=.claude/docs/services/viola.md line=6 class=retired
leaf-text: a file to the read rule on Unix and the write rule on Windows
source: fct-5jecrew3 «a symbolic link refused, the folder to the read rule, the file to the read rule, an absent path passing»
replace-with: a file to the read rule
@leaf_edit n=2 leaf=.claude/docs/services/viola.md line=6 class=retired
leaf-text: `decide`, `read`, the Windows VT enable and its restoring guard)
source: fct-irhgihut «style.rs: the colour layer (`Depth`, `Tokens`, `Facts`, `decide`, `read`);»
replace-with: `decide`, `read`)
@leaf_edit n=3 leaf=.claude/docs/services/viola.md line=6 class=retired
leaf-text: (on Unix, for shell-plain paths only, also one
source: fct-qwpklckh «Only for shell-plain paths, it also gets one `--settings <probe dir>/typed/statusline.json` after its `--plugin-dir` pair»
replace-with: (for shell-plain paths only, also one
@leaf_edit n=4 leaf=.claude/docs/services/viola.md line=6 class=retired
leaf-text: files each status line payload with its shell verdict; none on Windows)
source: fct-hss5qy6y «No Windows or macOS code, test, fixture or CI job stands in the tree; that code is recoverable at the tag `multi-os-final` (`ecbe7ab3`).»
replace-with: files each status line payload with its shell verdict)
@leaf_edit n=5 leaf=.claude/docs/services/viola.md line=6 class=stale
leaf-text: `check_rows` walks `LedgerRow::relied()`: on Unix all twenty-four of `LedgerRow::ALL`, the last `local-command-compact`; on Windows twenty-one, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`, the last `local-command-compact`)
source: fct-sfkaxxdn «the twenty-four rows (`LedgerRow::ALL`) and the relied set (`relied()`: all twenty-four rows on every build)»
replace-with: `check_rows` walks `LedgerRow::relied()`: all twenty-four of `LedgerRow::ALL` on every build, the last `local-command-compact`)
@leaf_edit n=6 leaf=.claude/docs/services/viola.md line=6 class=retired
leaf-text: (`cmd::verify::stop`: Unix `SIGINT` / `SIGTERM`, Windows `CTRL_C_EVENT` / `CTRL_BREAK_EVENT`, installed by
source: fct-c3nhxk2n «stopped by Ctrl-C or TERM: `SIGINT` / `SIGTERM` (`cmd::verify::stop`, installed by `viola verify` alone»
replace-with: (`cmd::verify::stop`: `SIGINT` / `SIGTERM`, installed by
@leaf_edit n=7 leaf=.claude/docs/services/viola.md line=6 class=retired
leaf-text: SIGKILL, a crash, a failed handler install and a Windows close, logoff or shutdown keep the OS default
source: fct-c3nhxk2n «SIGKILL, a crash and a failed handler install (ignored, fail-open) keep the OS default and leave the dirs to be removed by hand»
replace-with: SIGKILL, a crash and a failed handler install keep the OS default
@leaf_edit n=8 leaf=.claude/docs/services/viola.md line=11 class=retired
leaf-text: windows-sys (Windows only: `GetConsoleMode` / `SetConsoleMode` for `src/human/style.rs`'s VT enable on stdout — `ENABLE_VIRTUAL_TERMINAL_PROCESSING` (`0x0004`) alone, never viola-pty's `0x000c`, made only once every no-colour rung has passed in `viola list`'s human mode, the saved mode restored on drop, a failed get or set writing no colour and changing no mode, no dependency or feature added —, the registry value-name reader for the R8 persistent set in `src/run/env.rs`, and `RtlCaptureStackBackTrace` / `GetModuleHandleExW` / `GetModuleFileNameW` for `src/panic_frames.rs`, and `SetConsoleCtrlHandler` (`Win32_System_Console`) for `src/cmd/verify/stop.rs`'s console routine, where Ctrl-C and Ctrl-Break set verify's stop and every other event keeps the OS default); libc (Unix only: `backtrace` + `dladdr` for the raw panic frames, and `sigaction` for `src/cmd/verify/stop.rs`'s `SIGINT` / `SIGTERM` handler, `SA_RESTART`; neither stop role added a dependency or a feature)
source: fct-p5s5gpab «libc `=0.2.189` (`backtrace` + `dladdr` for the raw, never-symbolised panic frames, and `sigaction` for the same module's `SIGINT` / `SIGTERM` handler, `SA_RESTART`)»
replace-with: libc `=0.2.189` (`backtrace` + `dladdr` for the raw, never-symbolised panic frames of `src/panic_frames.rs`, and `sigaction` for `src/cmd/verify/stop.rs`'s `SIGINT` / `SIGTERM` handler, `SA_RESTART`; neither role added a dependency or a feature; no crate depends on windows-sys)
@leaf_edit n=9 leaf=.claude/docs/services/viola.md line=20 class=stale
leaf-text: program resolution (`.cmd`/`.bat` refusal) → collision check
source: fct-44n3w6o5 «program resolution (a given path as given, a bare name on `PATH`; it writes nothing), collision check, pinned copy and plugin folder, version gate»
replace-with: program resolution (a given path as given, a bare name on `PATH`; it writes nothing) → collision check
@leaf_edit n=10 leaf=.claude/docs/services/viola.md line=20 class=retired
leaf-text: → ConPTY sideload (Windows, span `run.conpty_sideload` {`outcome` `loaded|hash-mismatch|strict-modes-failed|unreadable|load-failed|not-built`, `search_restricted`}: `pin_companions` of `src/conpty.rs`'s embedded `OpenConsole.exe` + `conpty.dll` into `bin/<key>/conpty/`, then the absolute-path pre-load; never a refusal — any other outcome leaves the child on the inbox ConPTY and adds `sideload_fallback` to the `claude-child` `process-start`; the held handles live until `spawn_child` returns) → strip plan
source: fct-44n3w6o5 «every step has its place: program resolution → collision → pinned copy and plugin → strip plan → version gate»
replace-with: → strip plan
@leaf_edit n=11 leaf=.claude/docs/services/viola.md line=20 class=retired
leaf-text: (the per-session settings override, Unix only, is rewritten after the first snapshot and before the spawn)
source: fct-44n3w6o5 «Between the first snapshot and the spawn `run` also rewrites the per-session settings override»
replace-with: (the per-session settings override is rewritten after the first snapshot and before the spawn)
@leaf_edit n=12 leaf=.claude/docs/services/viola.md line=21 class=stale
leaf-text: `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch`
source: fct-54jxzo44 «`<code>` one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name`»
replace-with: `stale-heartbeat` · `pinned-hash-mismatch`
@leaf_edit n=13 leaf=.claude/docs/services/viola.md line=22 class=retired
leaf-text: On Windows `viola_pty::sideload::restrict_dll_search()` is the second statement of `main`, in every role (the panic hook is the first): no DLL load of any `viola` process resolves from the CWD or `PATH`.
source: fct-irhgihut «(`sideload`, the Windows DLL-search module: removed; recoverable at the tag multi-os-final))»
replace-with: The panic hook is the first statement of `main`, in every role.
@leaf_edit n=14 leaf=.claude/docs/services/viola.md line=23 class=retired
leaf-text: and on Unix rewrites `instances/<name>/settings.json`
source: fct-klpb7auf «`instances/<ViolaName>/settings.json` holds a `statusLine` command running the pinned path's `hook statusline`, rewritten whole at every start before the spawn»
replace-with: and rewrites `instances/<name>/settings.json`
@leaf_edit n=15 leaf=.claude/docs/services/viola.md line=23 class=retired
leaf-text: always exit 0, empty stderr. On Windows no override is written and no command runs (owed to the route entry "Windows-only live measurements").
source: fct-hss5qy6y «No Windows or macOS code, test, fixture or CI job stands in the tree; that code is recoverable at the tag `multi-os-final` (`ecbe7ab3`).»
replace-with: always exit 0, empty stderr.
@leaf_edit n=16 leaf=.claude/docs/services/viola.md line=26 class=retired
leaf-text: harness-injected prompts never do. On Windows an injected mouse report under the ConPTY's win32-input-mode arrives as typed keys and takes the wheel, and focus reports are swallowed (founder ruling F-W3).
source: fct-kxiazeyf «Hosts the unmodified `claude` in openpty (`pty_backend()` always `openpty`)»
replace-with: harness-injected prompts never do.
@leaf_edit n=17 leaf=.claude/docs/services/viola.md line=33 class=retired
leaf-text: On Windows a hook fired within milliseconds of the wrapper's exit can still reach the exiting wrapper's pipe (measured 25 ms after the exit line, ci#36532038635; why the pipe outlives the exit is not established).
source: fct-hss5qy6y «No Windows or macOS code, test, fixture or CI job stands in the tree; that code is recoverable at the tag `multi-os-final` (`ecbe7ab3`).»
replace-with: A hook whose endpoint is unreachable writes no frame.
@leaf_edit n=18 leaf=.claude/docs/services/viola.md line=33 class=retired
leaf-text: The hook still fails open (exit 0, silent), but its `hook.event` reads as delivered, so `SessionEnd`'s direct-append fallback is skipped in that window.
source: fct-nm6raycf «on a replay or an absence hook `connect` is unreachable (fail open; SessionEnd appends directly)»
replace-with: The hook fails open (exit 0, silent), and `SessionEnd` appends directly.
@leaf_edit n=19 leaf=.claude/docs/services/viola.md line=35 class=stale
leaf-text: `Ended::Refused` code in `detail` (`batch-script-child` · `program-not-found`
source: fct-jzpbbnum «`program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`, beside `interrupted`»
replace-with: `Ended::Refused` code in `detail` (`program-not-found`
@leaf_edit n=20 leaf=.claude/docs/services/viola.md line=54 class=stale
leaf-text: the per-OS CI `perf` job
source: fct-3nc6guvb «`perf` (on `ubuntu-latest`, `fail-fast: false`, `contents: read`, `AGENT_RUN_KEEP_HOMES=1`; test-plan §9 Perf row)»
replace-with: the CI `perf` job on `ubuntu-latest`
@leaf_edit n=21 leaf=.claude/docs/stack.md line=9 class=retired
leaf-text: One native binary `viola` / `viola.exe`.
source: fct-nuruygpl «One native binary `viola`. 1.96 is the declared floor»
replace-with: One native binary `viola`.
@leaf_edit n=22 leaf=.claude/docs/stack.md line=12 class=retired
leaf-text: ; windows-sys 0.61.2 for the `TerminateProcess` kill fallback, the host console's raw/VT modes (`Win32_System_Console`) and its input read (`ReadConsoleW`, behind `viola_pty::host_stdin()`); libc `=0.2.189` (Unix only) for the host tty's raw mode — both behind `HostTerminal`; windows-sys also restricts every process's DLL search to System32 and pre-loads the sideloaded `conpty.dll` by absolute path (`Win32_System_LibraryLoader`). On Windows x64 the ConPTY host is Microsoft's `Microsoft.Windows.Console.ConPTY` 1.24.260710001 (`conpty.dll` + `OpenConsole.exe`, MIT), vendored under `vendor/conpty/` and embedded — a non-crate binary pair | Hosts the unmodified `claude` in the sideloaded ConPTY (Windows x64; the inbox ConPTY is the fail-open fallback and the host on other Windows targets) or openpty (Unix) |
source: fct-kxiazeyf «libc `=0.2.189` for the host tty's raw mode, behind `HostTerminal`; the host's input is `std::io::stdin()` (`viola_pty::host_stdin()`) and its size the `TIOCGWINSZ` read (`viola_pty::host_size()`)»
replace-with: ; libc `=0.2.189` for the host tty's raw mode, behind `HostTerminal`; the host's input is `std::io::stdin()` (`viola_pty::host_stdin()`) and its size the `TIOCGWINSZ` read (`viola_pty::host_size()`) | Hosts the unmodified `claude` in openpty (`pty_backend()` always `openpty`) |
@leaf_edit n=23 leaf=.claude/docs/stack.md line=14 class=retired
leaf-text: used by `viola-mcp` alone, which opens a Windows pipe through the same SQOS + overlapped `client::open_handle`); on Windows the client opens with windows-sys 0.61.2 `CreateFileW(SECURITY_SQOS_PRESENT \| SECURITY_IDENTIFICATION \| FILE_FLAG_OVERLAPPED)` + `Stream::try_from`, never the default connect; the server DACL is converted from SDDL by windows-sys (`Win32_Security_Authorization`); on Unix, libc gives the uid
source: fct-jsy3xdhu «used by `viola-mcp` alone); libc gives the uid for the per-user socket directory and the socket is chmod 0600 after the bind»
replace-with: used by `viola-mcp` alone); libc gives the uid
@leaf_edit n=24 leaf=.claude/docs/stack.md line=14 class=retired
leaf-text: | One endpoint per `viola run`: a named pipe on Windows, a Unix domain socket elsewhere; the Windows open leaves the server able to identify, never impersonate, the client (measured) |
source: fct-jsy3xdhu «| One endpoint per `viola run`: a Unix domain socket |»
replace-with: | One endpoint per `viola run`: a Unix domain socket |
@leaf_edit n=25 leaf=.claude/docs/stack.md line=17 class=retired
leaf-text: std `File::lock` on separate `.lock` files opened for write (Windows refuses the lock on an append-only handle)
source: fct-s7lo3z2q «std `File::lock` on separate `.lock` files, opened for write; std `OpenOptions::append` (one `write` per line)»
replace-with: std `File::lock` on separate `.lock` files, opened for write
@leaf_edit n=26 leaf=.claude/docs/stack.md line=17 class=retired
leaf-text: through libc (Unix modes and owner) and windows-sys (owner + DACL, and the protected user + SYSTEM DACL on a home created outside the user's profile, `Win32_UI_Shell` `GetUserProfileDirectoryW`; a DACL that cannot be set fails the creation)
source: fct-s7lo3z2q «the strict-modes check and private-dir creation through libc (modes and owner)»
replace-with: through libc (modes and owner)
@leaf_edit n=27 leaf=.claude/docs/stack.md line=18 class=retired
leaf-text: the re-hash of an existing pinned copy; on Windows x64 also the full SHA-256 of each held ConPTY companion against its compiled pin;
source: fct-itjxddea «and to re-hash an existing pinned copy (read in 64 KiB chunks) before every reuse»
replace-with: the re-hash of an existing pinned copy (read in 64 KiB chunks) before every reuse;
@leaf_edit n=28 leaf=.claude/docs/stack.md line=23 class=retired
leaf-text: (root bin `src/panic_frames.rs`: windows-sys `Win32_System_Diagnostics_Debug` + `Win32_System_LibraryLoader` on Windows, libc `backtrace` + `dladdr` on Unix)
source: fct-p5s5gpab «libc `=0.2.189` (`backtrace` + `dladdr` for the raw, never-symbolised panic frames, and `sigaction` for the same module's `SIGINT` / `SIGTERM` handler, `SA_RESTART`)»
replace-with: (root bin `src/panic_frames.rs`: libc `backtrace` + `dladdr`)
@leaf_edit n=29 leaf=.claude/docs/stack.md line=30 class=stale
leaf-text: GitHub Actions matrix `windows-2025`, `macos-latest` (macOS 26), `ubuntu-latest`;
source: fct-sb4r6c4a «GitHub Actions matrix `[ubuntu-latest]` (`test`, `perf`, `lint`, `release`; every job and artifact name keeps its `${{ matrix.os }}` form)»
replace-with: GitHub Actions matrix `[ubuntu-latest]` (`test`, `perf`, `lint`, `release`; every job and artifact name keeps its `${{ matrix.os }}` form);
@leaf_edit n=30 leaf=.claude/docs/stack.md line=30 class=stale
leaf-text: Build, lint and test on all three OSes against the fake agent
source: fct-sb4r6c4a «Build, lint and test on `ubuntu-latest` against the fake agent, on native runners.»
replace-with: Build, lint and test on `ubuntu-latest` against the fake agent
@leaf_edit n=31 leaf=.claude/docs/stack.md line=37 class=omission
leaf-text: windows-sys 0.61.2 SID / SQOS / DACL APIs · sha2
source: fct-iw73rjlh «| OS bindings | libc, a plain (not target-gated) `[dependencies]` entry of the root, `viola-channel`, `viola-pty`, `viola-state` and `viola-e2e`; no crate depends on windows-sys |»
replace-with: libc, a plain (not target-gated) `[dependencies]` entry of the root, `viola-channel`, `viola-pty`, `viola-state` and `viola-e2e`, the one OS-binding crate (no crate depends on windows-sys) · sha2
@leaf_edit n=32 leaf=.claude/docs/stack.md line=39 class=tree
leaf-text: ; `scripts/conpty-vendor.sh --verify` / `--probe` for the vendored ConPTY binaries (nupkg + per-file SHA-256, Authenticode signer; CI `windows-2025` step `ConPTY vendor verification`).
source: fct-irhgihut «(conpty-vendor.sh)      # removed with the Windows code; recoverable at the tag multi-os-final»
replace-with: .
@leaf_edit n=33 leaf=.claude/docs/workflow.md line=10 class=stale
leaf-text: Each OS's `test` job gates `coverage,doctest`.
source: fct-3nc6guvb «`test` (on `ubuntu-latest`; one harness `run --coverage` through the `sh` shim (`scripts/agent-run.sh`)»
replace-with: The `test` job (on `ubuntu-latest`) gates `coverage,doctest`.
@leaf_edit n=34 leaf=.claude/docs/workflow.md line=10 class=retired
leaf-text: the audit's Windows leg is the dispatch-only `windows-mutants.yml` (`gh workflow run windows-mutants.yml --ref build/viola-0.1.0`), never a gate.
source: fct-vf6cfvel «The boundary audit has no Windows leg: `windows-mutants.yml` left with the Windows code, recoverable at the tag `multi-os-final`.»
replace-with: the audit has no Windows leg: `windows-mutants.yml` left with the Windows code, recoverable at the tag `multi-os-final`.
@leaf_edit n=35 leaf=.claude/docs/workflow.md line=11 class=retired
leaf-text: ; no mutation stage; the other OSes are CI's alone)
source: fct-ni3gbuvj «`pre-push` runs on a Linux host only (elsewhere it refuses `pre-push-linux-only`, exit 2)»
replace-with: ; no mutation stage; off Linux it refuses `pre-push-linux-only`)
@leaf_edit n=36 leaf=.claude/docs/workflow.md line=47 class=retired
leaf-text: (from `run --mutants`; in the host mutation scratch on Windows, each run's copy archived in `target/run-archive/`)
source: fct-7wuxnmfc «on the Linux dev host the operator's NOCOW `TMPDIR` for mutation runs, an environment fact, not harness code; `run --mutants` has no scratch arm.»
replace-with: (from `run --mutants`, each run's copy archived in `target/run-archive/`)
@leaf_edit n=37 leaf=.claude/docs/workflow.md line=48 class=stale
leaf-text: CI (three OSes) is the authority for OS-specific code: a Linux-only or Windows-only local run is not proof for the other OSes.
source: fct-fa2klbw5 «The product builds and is tested on Linux (CI on `ubuntu-latest`); every OS-gated test names its reason in the census binary `tests/contract_os_gated_tests.rs`»
replace-with: The product builds and is tested on Linux only (CI on `ubuntu-latest`); CI's run on the pushed sha stays the verdict of record.
@leaf_edit n=38 leaf=.claude/docs/workflow.md line=48 class=omission
leaf-text: or its census goes red.
source: fct-fa2klbw5 «a gate whose predicate names `windows` or `target_os = "macos"` reads as a fault (`another OS: …`) even when a row names it.»
replace-with: or its census goes red; a gate whose predicate names `windows` or `target_os = "macos"` reads as a fault (`another OS: …`) even when a row names it.
@leaf_edit n=39 leaf=.claude/docs/workflow.md line=49 class=stale
leaf-text: (`LedgerRow::relied()`: all twenty-four on the Unix runners, twenty-one on `windows-2025`, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`)
source: fct-oqzfmod4 «(`LedgerRow::relied()`: all twenty-four on every build; `tests/cli_verify.rs`, harness `boot` step 4)»
replace-with: (`LedgerRow::relied()`: all twenty-four on every build)
