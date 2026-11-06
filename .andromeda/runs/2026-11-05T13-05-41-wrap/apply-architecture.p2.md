@apply_row n=1 ref_file=fanout-architecture--cascade-g1.md ref_n=2 v=applied edit=E1
@apply_row n=2 ref_file=fanout-architecture--cascade-g1.md ref_n=3 v=applied edit=E2
@apply_row n=3 ref_file=fanout-architecture--cascade-g1.md ref_n=4 v=applied edit=E3
@apply_row n=4 ref_file=fanout-architecture--cascade-g1.md ref_n=5 v=refused
why: a hand removal · fct-q5vugilx — the Windows pipe open its Technology cell retires holds the escaped pipes of `CreateFileW(SECURITY_SQOS_PRESENT … FILE_FLAG_OVERLAPPED)`, so an in-line change is refused (a table cell boundary removed); the whole row goes and its Linux row replaces it: the Technology cell without «, which opens a Windows pipe through the same SQOS + overlapped `client::open_handle`», the `CreateFileW` sentence, the SDDL pipe-DACL sentence and «on Unix,» (the Tokio flavour clause, the uid, chmod 0600 and `check_endpoint_dir` clauses kept), the Role cell «One endpoint per `viola run`: a Unix domain socket»
@apply_row n=5 ref_file=fanout-architecture--cascade-g1.md ref_n=6 v=applied edit=E4
@apply_row n=6 ref_file=fanout-architecture--cascade-g1.md ref_n=7 v=applied edit=E5
@apply_row n=7 ref_file=fanout-architecture--cascade-g1.md ref_n=8 v=applied edit=E6
@apply_row n=8 ref_file=fanout-architecture--cascade-g1.md ref_n=9 v=applied edit=E7
@fact id=fct-nuruygpl expect_rev=1
old: One native binary `viola` / `viola.exe`.
new: One native binary `viola`.

@fact id=fct-kxiazeyf expect_rev=1
old: windows-sys 0.61.2 for the `TerminateProcess` kill fallback, the host console's raw/VT modes (`Win32_System_Console`) and its input read (`ReadConsoleW`, behind `viola_pty::host_stdin()`); libc `=0.2.189` (Unix only) for the host tty's raw mode — both behind `HostTerminal`; windows-sys also restricts every `viola` process's DLL search to System32 (`SetDefaultDllDirectories`) and pre-loads the sideloaded `conpty.dll` by absolute path (`LoadLibraryExW`, `Win32_System_LibraryLoader`). On Windows x64 the ConPTY host is Microsoft's `Microsoft.Windows.Console.ConPTY` 1.24.260710001 (`conpty.dll` + `OpenConsole.exe`, MIT), a vendored non-crate binary pair under `vendor/conpty/`, embedded in `viola`
new: libc `=0.2.189` for the host tty's raw mode, behind `HostTerminal`; the host's input is `std::io::stdin()` (`viola_pty::host_stdin()`) and its size the `TIOCGWINSZ` read (`viola_pty::host_size()`)
old: Hosts the unmodified `claude` in the sideloaded ConPTY (Windows x64; the inbox ConPTY is the fail-open fallback and the host on other Windows targets) or openpty (Unix)
new: Hosts the unmodified `claude` in openpty (`pty_backend()` always `openpty`)

@fact after=fct-kxiazeyf as=n1
| OS bindings | libc, a plain (not target-gated) `[dependencies]` entry of the root, `viola-channel`, `viola-pty`, `viola-state` and `viola-e2e`; no crate depends on windows-sys | The one OS-binding crate of the Linux-only build |

@fact id=fct-s7lo3z2q expect_rev=1
old: (`viola_state::fs::replace_private`: the mode
new: (`viola_state::fs::replace_private`, one attempt: the mode
old: opened for write (Windows refuses an exclusive lock on an append-only handle);
new: opened for write;
old: through libc (Unix modes and owner) and windows-sys (owner + DACL, and the protected user + SYSTEM DACL on a home created outside the user's profile, `Win32_UI_Shell` `GetUserProfileDirectoryW`; a DACL that cannot be set fails the creation)
new: through libc (modes and owner)

@fact id=fct-itjxddea expect_rev=1
old: before every reuse; on Windows x64 also the full SHA-256 of each held ConPTY companion against its compiled pin (`viola_state::pin::pin_companions`). A product
new: before every reuse. A product

@fact id=fct-sb4r6c4a expect_rev=1
old: GitHub Actions matrix `windows-2025`, `macos-latest` (macOS 26), `ubuntu-latest`;
new: GitHub Actions matrix `[ubuntu-latest]` (`test`, `perf`, `lint`, `release`; every job and artifact name keeps its `${{ matrix.os }}` form);
old: Build, lint and test on all three OSes against the fake agent
new: Build, lint and test on `ubuntu-latest` against the fake agent

@fact id=fct-vp2lxvsd expect_rev=1
old: (Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the SQOS-adopted client end, none on macOS)
new: (`SO_PEERCRED`)
old: On Unix both clients also read
new: Both clients also read
old: `TokioClient::server_euid`), none on Windows, where the DACL and the pid comparison stay the controls.
new: `TokioClient::server_euid`).
@pattern id=arch2-01
fixed: native binary `viola` / `viola.exe`
@pattern id=arch2-02 ignore_case=true
fixed: in the sideloaded ConPTY
@pattern id=arch2-03 ignore_case=true
fixed: Windows refuses an exclusive lock on an append-only handle
@pattern id=arch2-04 ignore_case=true
fixed: SHA-256 of each held ConPTY companion
@pattern id=arch2-05 ignore_case=true
fixed: test on all three OSes
@pattern id=arch2-06
fixed: none on Windows, where the DACL and the pid comparison stay the controls
