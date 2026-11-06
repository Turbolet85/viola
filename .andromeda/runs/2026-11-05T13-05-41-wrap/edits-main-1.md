@fact id=fct-jbq7vmcz expect_rev=1
old: the `viola run` start refusals (`.cmd`/`.bat` child, live or stale name,
new: the `viola run` start refusals (live or stale name,

@fact id=fct-cotc6dxt expect_rev=1
old: per-crate `[[licenses.exceptions]]` for `doctest-file` and `recvmsg`, the two 0BSD crates interprocess 2.4.4 pulls in, never through `allow`
new: the per-crate `[[licenses.exceptions]]` for `doctest-file`, the one 0BSD crate interprocess 2.4.4 pulls into the Linux graph, never through `allow`
old: evaluated for the Windows, macOS and Linux target triples so `cfg`-gated dependencies are covered.
new: evaluated for the one target triple `x86_64-unknown-linux-gnu`.
old: (the same three triples, `exclude-dev = true`)
new: (the same one triple, `exclude-dev = true`)

@fact id=fct-4jixev2q expect_rev=1
old: `~/.viola/bin/<version>-<hash>/` (on Windows x64 also the embedded ConPTY companions in its `conpty/` subdirectory, with `conpty.dll` pre-loaded from there before the spawn), writes
new: `~/.viola/bin/<version>-<hash>/`, writes

@fact id=fct-ggzishfk expect_rev=1
old: tracing (the `pty.spawn` span), windows-sys (Windows only), libc (Unix only; termios raw mode and the terminal size)
new: tracing (the `pty.spawn` span), libc (termios raw mode and the terminal size)

@fact id=fct-vavh762f expect_rev=1
old: veil, windows-sys (Windows only: the SQOS client open adopted by `Stream::try_from`, and the listener DACL through `Win32_Security_Authorization`), libc (Unix only: the uid for the socket directory)
new: veil, libc (the uid for the socket directory)
old: `frame::framed`, and on Windows `client::open_handle`, the SQOS + overlapped open)
new: `frame::framed`)

@fact id=fct-a4ctqcf6 expect_rev=1
old: libc (Unix only: the `strict` module's owner + mode check) and windows-sys (Windows only: the `strict` module's owner + DACL check, and the protected `D:P(A;OICI;FA;;;<user>)(A;OICI;FA;;;SY)` DACL `fs::create_private_dir` gives the topmost folder it creates outside the user's profile, the profile read with `GetUserProfileDirectoryW` — the workspace pin gains `Win32_UI_Shell`), both already workspace-pinned.
new: libc (the `strict` module's owner + mode check), workspace-pinned.

@fact id=fct-sfkaxxdn expect_rev=1
old: `SETTINGS_FLAG`, `USER_SETTINGS`, `WRAPPED_HERE`, `BIN_SH`
new: `SETTINGS_FLAG`, `USER_SETTINGS`, `BIN_SH`
old: and the per-OS relied set (`relied()`, filtering on `relied_on(WRAPPED_HERE)`)
new: and the relied set (`relied()`: all twenty-four rows on every build)
old: `unclean` (the closed `Unclean` codes); `CASE_INSENSITIVE` and `is_script` are `pub` for `verify`; and
new: `unclean` (the closed `Unclean` codes); and

@fact id=fct-p5s5gpab expect_rev=1
old: `viola verify`'s Run A temp dir only), windows-sys (Windows only: the registry reader for the R8 persistent-environment names in `src/run/env.rs`, plus `Win32_System_Diagnostics_Debug` and `Win32_System_LibraryLoader` for `src/panic_frames.rs`'s `RtlCaptureStackBackTrace` / `GetModuleHandleExW` / `GetModuleFileNameW`, plus `Win32_System_Console` `SetConsoleCtrlHandler` for `src/cmd/verify/stop.rs`'s console routine, where Ctrl-C and Ctrl-Break set verify's stop and every other event keeps the OS default, and `GetConsoleMode` / `SetConsoleMode` for `src/human/style.rs`'s VT enable on stdout: `ENABLE_VIRTUAL_TERMINAL_PROCESSING` (`0x0004`) alone, never viola-pty's `0x000c`, made only once every no-colour rung has passed in `viola list`'s human mode, the saved mode restored on drop, a failed get or set writing no colour and changing no mode — no dependency or feature added, chunk 2026-10-17-cli-output-discipline) and libc `=0.2.189` (Unix only: `backtrace`
new: `viola verify`'s Run A temp dir only) and libc `=0.2.189` (`backtrace`
