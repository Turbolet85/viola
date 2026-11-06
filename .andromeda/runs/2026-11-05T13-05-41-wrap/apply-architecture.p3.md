@apply_row n=1 ref_file=fanout-architecture--cascade-g2.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-architecture--cascade-g2.md ref_n=2 v=applied edit=E2
@apply_row n=3 ref_file=fanout-architecture--cascade-g2.md ref_n=3 v=applied edit=E3
@apply_row n=4 ref_file=fanout-architecture--cascade-g2.md ref_n=4 v=refused
why: a hand removal · fct-7xqqphkl
@apply_row n=5 ref_file=fanout-architecture--cascade-g2.md ref_n=5 v=applied edit=E4
@apply_row n=6 ref_file=fanout-architecture--cascade-g2.md ref_n=6 v=applied edit=E5
@apply_row n=7 ref_file=fanout-architecture--cascade-g2.md ref_n=7 v=applied edit=E6
@apply_row n=8 ref_file=fanout-architecture--cascade-g2.md ref_n=8 v=applied edit=E7
@apply_row n=9 ref_file=fanout-architecture--cascade-g2.md ref_n=9 v=applied edit=E8
@apply_row n=10 ref_file=fanout-architecture--cascade-g2.md ref_n=10 v=applied edit=E9
@apply_row n=11 ref_file=fanout-architecture--cascade-g2.md ref_n=11 v=applied edit=E10
@apply_row n=12 ref_file=fanout-architecture--cascade-g2.md ref_n=12 v=applied edit=E11
@apply_row n=13 ref_file=fanout-architecture--cascade-g2.md ref_n=13 v=applied edit=E12
@apply_row n=14 ref_file=fanout-architecture--cascade-g2.md ref_n=14 v=applied edit=E13
@apply_row n=15 ref_file=fanout-architecture--cascade-g2.md ref_n=15 v=applied edit=E14
@apply_row n=16 ref_file=fanout-architecture--cascade-g3.md ref_n=1 v=applied edit=E15
@apply_row n=17 ref_file=fanout-architecture--cascade-g3.md ref_n=2 v=applied edit=E16
@apply_row n=18 ref_file=fanout-architecture--cascade-g3.md ref_n=3 v=folded_into to_file=fanout-architecture--cascade-g3.md to_n=2
@apply_row n=19 ref_file=fanout-architecture--cascade-g3.md ref_n=4 v=folded_into to_file=fanout-architecture--cascade-g3.md to_n=2
@apply_row n=20 ref_file=fanout-architecture--cascade-g3.md ref_n=5 v=folded_into to_file=fanout-architecture--cascade-g3.md to_n=2
@apply_row n=21 ref_file=fanout-architecture--cascade-g3.md ref_n=6 v=folded_into to_file=fanout-architecture--cascade-g3.md to_n=2
@apply_row n=22 ref_file=fanout-architecture--cascade-g3.md ref_n=7 v=folded_into to_file=fanout-architecture--cascade-g3.md to_n=2
@apply_row n=23 ref_file=fanout-architecture--cascade-g3.md ref_n=8 v=folded_into to_file=fanout-architecture--cascade-g3.md to_n=2
@apply_row n=24 ref_file=fanout-architecture--cascade-g3.md ref_n=9 v=folded_into to_file=fanout-architecture--cascade-g3.md to_n=2
@apply_row n=25 ref_file=fanout-architecture--cascade-g3.md ref_n=10 v=folded_into to_file=fanout-architecture--cascade-g3.md to_n=2
@apply_row n=26 ref_file=fanout-architecture--cascade-g3.md ref_n=11 v=applied edit=E17
@apply_row n=27 ref_file=fanout-architecture--cascade-g3.md ref_n=12 v=applied edit=E18
@apply_row n=28 ref_file=fanout-architecture--cascade-g3.md ref_n=13 v=folded_into to_file=fanout-architecture--cascade-g3.md to_n=12
@apply_row n=29 ref_file=fanout-architecture--cascade-g3.md ref_n=14 v=folded_into to_file=fanout-architecture--cascade-g3.md to_n=12
@apply_row n=30 ref_file=fanout-architecture--cascade-g3.md ref_n=15 v=applied edit=E19
@apply_row n=31 ref_file=fanout-architecture--cascade-g3.md ref_n=16 v=applied edit=E20
@apply_row n=32 ref_file=fanout-architecture--cascade-g3.md ref_n=17 v=applied edit=E21
@apply_row n=33 ref_file=fanout-architecture--cascade-g3.md ref_n=19 v=applied edit=E22
@apply_row n=34 ref_file=fanout-architecture--cascade-g3.md ref_n=20 v=folded_into to_file=fanout-architecture--cascade-g3.md to_n=19
@apply_row n=35 ref_file=fanout-architecture--cascade-g3.md ref_n=21 v=folded_into to_file=fanout-architecture--cascade-g3.md to_n=19
@apply_row n=36 ref_file=fanout-architecture--cascade-g3.md ref_n=22 v=refused
why: a hand removal · fct-5neshki5
@apply_row n=37 ref_file=fanout-architecture--cascade-g3.md ref_n=23 v=applied edit=E23
@apply_row n=38 ref_file=fanout-architecture--cascade-g3.md ref_n=24 v=folded_into to_file=fanout-architecture--cascade-g3.md to_n=23
@apply_row n=39 ref_file=fanout-architecture--cascade-g3.md ref_n=25 v=applied edit=E24
@apply_row n=40 ref_file=fanout-architecture--cascade-new-established-decisions.md ref_n=1 v=applied edit=E25,E26
@fact id=fct-gmzv3mtg expect_rev=1
old: evaluated for the Windows, macOS and Linux target triples, plus a `cargo check` of the listed sync crates on all three OSes (CI job 3)
new: evaluated for the one target triple `x86_64-unknown-linux-gnu`, plus a `cargo check` of the listed sync crates on `ubuntu-latest` (CI job 3)

@fact id=fct-ad44nn2e expect_rev=1
old: If 0.8.1's Windows `kill()` misbehaves, the fallback is `TerminateProcess` on `child.process_id()` via windows-sys. The host stdin handed to the pump is `viola_pty::host_stdin()`: on Windows with a console stdin it is viola's own reader calling `ReadConsoleW` (no Ctrl-Z wakeup control; UTF-16 → UTF-8, a split surrogate carried, every `0x1A` kept, a 0-unit read read again), because std's console stdin drops a read's trailing `0x1A` and ends input on a lone `^Z` — source-read at the pinned toolchain and as measured on `windows-2025` (ci#37226294797, `tui_ctrl_z…` green); elsewhere, and for a redirected Windows stdin, it is `std::io::stdin()`. Its callers
new: The host stdin handed to the pump is `viola_pty::host_stdin()`, which is `std::io::stdin()`. Its callers
old: 0.9.0 is excluded because of the Windows garbage-read bug wezterm#6783. Swap triggers
new: 0.9.0's Windows garbage-read bug (wezterm#6783) does not bear on a Linux-only build; moving `portable-pty` off `=0.8.1` is owed to the working-route entry "Version done-check" (RUSTSEC-2026-0441, below). Swap triggers
old: (then test `PASSTHROUGH_MODE`). An own ConPTY on windows-sys 0.61.2 + nix 0.31.3 comes only after a measured defect the seam cannot absorb. As built (chunk 2026-09-25-pty-wrapper-on-windows, measured on the Windows host in its research facts 1-5): `viola run` puts its own host terminal into raw input + VT output for the run through `HostTerminal` (Windows console modes; Unix termios) and restores the saved modes on Drop — without it ConPTY line-buffers keys until Enter and swallows Ctrl-C; off a terminal it is a no-op.
new: (then test `PASSTHROUGH_MODE`). `viola run` puts its own host terminal into raw mode for the run through `HostTerminal` (Unix termios) and restores the saved modes on Drop; off a terminal it is a no-op.
old: current directory (portable-pty 0.8.1 defaults to `%USERPROFILE%`). A revived
new: current directory. A revived
old: an absolute path itself (portable-pty's Windows search tries the exact name first and hits the extensionless npm sh shim). The pump keeps the ConPTY input writer until the child exits, because dropping it makes conhost send `CTRL_CLOSE` to the child (exit `0xC000013A`). The exit
new: an absolute path itself. The pump keeps the input writer until the child exits. The exit
old: or `kill-fallback`. As measured (chunk 2026-09-29-h2-conpty-resize-probe, H2): a key written through the seam right after `resize` returned can fail to reach the child — on the windows-2025 runner (image `windows-2025-vs2026` 20260922.246.2), in an isolated loop of the viola-pty red test under `cargo llvm-cov nextest`, 13 of 200 iterations lost it (ci#36527891850), every loss the same class: the child's key-free size watcher saw the new size, viola's writer wrote and flushed the key, and the Rust test child blocked in its console read never received it; ConPTY wrote no cursor-position request (`dsr-cpr 0` on every loss, and on the dev host, Windows 10.0.26200.9457 / conhost 10.0.26100.8875, which read 20/20).
new: or `kill-fallback`.
old: The measured span runs from the ConPTY input-pipe write to that test child's console read; where inside it the key is lost is not established (an overseer read of microsoft/terminal's current source, relayed at that chunk's wrap, found no input-buffer flush on the resize path — a relay, not verified here). Whether the real `claude` (Node/libuv, `ReadConsoleInputW`) loses a key typed right after a resize is unmeasured; the working-route entry "Windows-only live measurements" owns that measurement, blocked on an interactive Windows host (the harness's local `run --local-live`, a real-`claude` `viola verify`, does not claim it), and the founder's product question on H2 stays open. viola never holds
new: viola never holds
old: observe the new size first (0 of 200 lost, ci#36529038462). As built (chunk 2026-09-29-sideloaded-conpty, founder ruling 2026-09-29): on Windows x64 the child's ConPTY is Microsoft's current one, sideloaded. Every `viola` process, in every role, restricts its DLL search to System32 as the second statement of `main` (`viola_pty::sideload::restrict_dll_search`, process-wide and one-way), so a bare-name load never picks a `conpty.dll` planted in the CWD or on `PATH`. `run` pins the embedded `conpty.dll` + `OpenConsole.exe` into `bin/<version>-<hash>/conpty/` ([Deployment / Distribution]) and pre-loads `conpty.dll` by absolute path (`viola_pty::sideload::preload`: a relative path is refused, the module is never freed), so portable-pty `=0.8.1`'s own bare-name `LoadLibraryW("conpty.dll")` returns that module and the child is hosted by the pinned `OpenConsole.exe`; the seam and portable-pty are unchanged, and viola-pty knows no pinned path.
new: observe the new size first.
old: `viola_pty::pty_backend()` reports `conpty-sideload` after a successful pre-load, else `conpty` (Windows) or `openpty`. An absent, tampered, unreadable or unloadable sideload, or one whose `conpty/` directory or a companion fails its owner/DACL check ([Deployment / Distribution]), fails open to kernel32's inbox ConPTY with no byte, refusal or exit change (`sideload_fallback` on `process-start`). Spawners outside `viola run` stay on the inbox ConPTY: the harness `supervise::spawn_in_pty`, root `tests/support/outer_pty.rs` and viola-pty's own tests. H2 with and without, measured in one run on the windows-2025 runner (image `windows-2025-vs2026` 20260828.587, ci#36563868040, the measurement-only race of the pre-`dce98ad` shape, 200 iterations each): sideloaded 0 of 200 lost, inbox 14 of 200 lost — beside the inbox 13 of 200 above; no rate is claimed. The sideloaded host writes a different spawn preamble, `ESC[1t ESC[c ESC[?1004h ESC[?9001h` (the inbox one: `ESC[?9001h ESC[?1004h`), and its DA1 query `ESC[c` holds the child's start until something answers it — as measured at the chunk's `evidence/da1-stall.md` on the dev host, 3.54 s unanswered on pipes vs 0.54 s answered (0.48 s on the inbox host); a terminal answers it, and the answer travels through viola's pump, so the child's start now waits for the pump. viola stays silent (it never answers DA1 itself); the stall of a `viola run` with no terminal on its stdio is an open product finding, owned by the working-route entry "Windows-only live measurements", blocked on an interactive Windows host.
new: `viola_pty::pty_backend()` is always `openpty`, and `viola_pty::host_size()` reads `TIOCGWINSZ`.

@fact after=fct-ad44nn2e as=n1
- RUSTSEC-2026-0441 (`pin-utils` unmaintained, via `portable-pty =0.8.1` → `nix 0.25.1`) reads red in supply-chain's advisories family, as measured at ci#42030025050; its owner is a CARRY on the working-route entry "Version done-check": moving `portable-pty` off `=0.8.1` to a release whose graph drops `pin-utils`, with no ignore-list widening.

@fact id=fct-va43dnf2 expect_rev=1
old: at the unit and cross-process layers on the three CI OSes.
new: at the unit and cross-process layers on the Linux CI runner (`ubuntu-latest`).
old: included), an LF typed inside a paste on Windows (owed to "Windows-only live measurements"), and what
new: included), and what

@fact id=fct-ak4xjyyx expect_rev=1
old: Lock files are separate from logs because append + exclusive lock fails on Windows (rust-lang/rust#54118).
new: Lock files are separate from logs, so no file is both appended to and exclusively locked (that pair fails on Windows, rust-lang/rust#54118).

@fact id=fct-kfz3cseb expect_rev=1
old: On Windows a `persist` refused with raw OS error 5 (a reader holding the target open, as measured at chunk 2026-09-27-hooks-to-normalised-events research.md M1 under every share mode) is retried every 10 ms, re-persisting the same temp file, at most `viola_state::fs::REPLACE_ATTEMPTS` = 100 attempts (an attempt count, never a time bound); any other error returns at once. The loop sits in the private `replace_private_retrying`, which takes the decision to retry a failed rename (from its raw OS error) from its caller; `replace_private_with` passes `retry_replace(_, HOST_IS_WINDOWS)`, so the product's behaviour is the one above and the loop's attempt bound is exercised by a unit case on every host (chunk 2026-10-13-epoch-4-survivors).
new: `replace_private` is one attempt: a failed `persist` returns at once, the target as it was; `write_snapshot` calls it.
old: (the plugin files, the Windows x64 ConPTY companions under `bin/<version>-<hash>/conpty/`):
new: (the plugin files):
old: The companions' held open (`FILE_SHARE_READ` only) retries a Win32 error 32 (sharing violation) under the same `REPLACE_ATTEMPTS` bound (`retry_open`, `open_held_waiting`), as measured at chunk 2026-09-29-sideloaded-conpty: two concurrent first starts, the refusal lasting 5-11 ms after the other start's write, 0 of 6 pairs over companions already written. No call site
new: No call site

@fact id=fct-k7elqwqc expect_rev=1
old: execs the placed copy from it on Unix, and `place_private_new_placed_copy_runs_at_once` runs it on Windows.
new: execs the placed copy from it on Unix.

@fact id=fct-wfmyj5kz expect_rev=1
old: never the raw `VIOLA_NAME` + path. On Windows the client connects only through the SQOS Identification open (windows-sys `CreateFileW` with `SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION | FILE_FLAG_OVERLAPPED`, adopted by `Stream::try_from`), never interprocess's default connect, so a squatted or rogue server cannot impersonate the client; a non-overlapped handle adopted this way hangs (as measured at chunk 2026-09-25-security-prerequisites). Rationale: one API over named pipes and Unix sockets, sync
new: never the raw `VIOLA_NAME` + path. Rationale: one API over Unix sockets, sync
old: and no daemon. macOS caps socket paths at about 104 bytes. Accepted risk
new: and no daemon. Accepted risk

@fact id=fct-rrvv5rrr expect_rev=1
old: `peer_creds().pid()`: Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the client end the SQOS Identification open adopts, none on macOS, where `SERVER_PID_REPORTED` is false; they write
new: `peer_creds().pid()` (`SO_PEERCRED`); they write
old: so on Linux and Windows a squatted or rogue server is never sent a frame either.
new: so a squatted or rogue server is never sent a frame.

@fact id=fct-wqjpkbtb expect_rev=1
old: exports `peer_euid_admitted`, `peer_admitted` and `PEER_EUID_REPORTED`, the last `cfg!(unix)`: `peer_euid_admitted(observed, own, reported)` is `!reported || observed == Some(own)`, so a peer whose euid cannot be read is refused where it is reported, and on Windows the DACL and the pid comparison stay the controls.
new: exports `peer_euid_admitted` and `peer_admitted`: `peer_euid_admitted(observed, own)` is `observed == Some(own)`, so a peer whose euid cannot be read is refused.

@fact id=fct-y7p4rnnj expect_rev=1
old: On macOS no peer pid is reported, so `viola_state::liveness::server_verdict` returns `Matches` with no comparison: the recorded gap, named and owned by no route entry. The macOS client euid and socket-directory checks, and the 0755-directory exit-cause case, stand built, so on macOS `server-verify-failed` means an euid mismatch alone.
new: `viola_state::liveness::server_verdict` and `verify_server` always compare the connected peer's pid and OS start time with the snapshot's: no verdict reads `Matches` without that comparison.

@fact id=fct-kbitf2mh expect_rev=1
old: a refused directory stays as found, and off Unix it returns `Ok(())`.
new: a refused directory stays as found.

@fact id=fct-vxml3b56 expect_rev=1
old: at "Version done-check", and the long hold's Windows leg at "Windows-only live measurements"; the ceiling
new: at "Version done-check"; the ceiling

@fact id=fct-4coxmu76 expect_rev=1
old: On Windows the platform decides what arrives (the founder's live ruling F-W3, 2026-10-04, relayed by the overseer): the inbox ConPTY outer terminal swallows focus reports, and under the sideloaded ConPTY's win32-input-mode request (`ESC[?9001h`) a mouse report written as text into the console's input arrives as one win32 key-down record per character (`ESC[0;0;<Uc>;1;0;1_`) and takes the wheel — as measured on `windows-2025` (ci#37227518624) for injected bytes; a mouse report from a real Windows terminal is not yet measured. The error only ever favours the human. A bridge send
new: A bridge send

@fact id=fct-ikuc6e4y expect_rev=1
old: As landed (chunk 2026-10-10-statusline-pass-through) the wrap is Unix only: on Windows no override is written and no command is run, and that half is owed to the working-route entry "Windows-only live measurements" (the founder's word on the cut, live, 2026-10-10, relayed verbatim by the operator). A reading
new: A reading

@fact id=fct-qwpklckh expect_rev=1
old: The relied set is `LedgerRow::relied()`, `ALL` filtered in its order by `relied_on(row, statusline::WRAPPED_HERE)`: the three statusline rows are relied only where the status line is wrapped (Unix), so a Unix host relies on all twenty-four rows and a Windows host on twenty-one, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`, the last three relied on every OS.
new: The relied set is `LedgerRow::relied()`: every row of `ALL`, in its order, all twenty-four.
old: runs a statusline command through on each OS. Rows
new: runs a statusline command through. Rows
old: relied only where the status line is wrapped (`statusline::WRAPPED_HERE`, Unix): on Unix the statusline pass-through
new: relied on every build: the statusline pass-through
old: refusal rest on stamped rows; the Windows legs are owed to "Windows-only live measurements" (§Cross-cutting
new: refusal rest on stamped rows (§Cross-cutting
old: `MM` being `LedgerRow::relied().len()`: 24 on Unix, the last `[24/24] local-command-compact …`; 21 on Windows, the last `[21/21] local-command-compact …`)
new: `MM` being `LedgerRow::relied().len()`, 24, the last `[24/24] local-command-compact …`)
old: On a wrapped OS (Unix), and only for shell-plain paths, it also gets
new: Only for shell-plain paths, it also gets
old: written 0600; Windows passes none)
new: written 0600)
old: all `pass`, reads unverified on every OS, and so do one holding the eighteen and `resume-session` and one holding the eighteen, `resume-session` and `agents-join` on Windows; one holding the eighteen, `resume-session`, `agents-join` and `local-command-compact` reads verified on Windows only; so every home
new: all `pass`, reads unverified; so every home
old: npm-shim → `claude.exe` resolution.
new: `shim-resolution`: `claude --version` answering (the row reads `run.version_answered` alone).
old: on the Linux dev host; its Windows reading owed on "Windows-only live measurements"). The row is relied on every OS and probed
new: on the Linux dev host). The row is relied on every build and probed
old: five starts), not measured on Windows (owed to "Windows-only live measurements"); the `resume-session` row
new: five starts); the `resume-session` row
old: six starts each), relied on every OS and not measured on Windows (owed to "Windows-only live measurements"); the `local-command-compact` row
new: six starts each); the `local-command-compact` row
old: one record round, six starts), relied on every OS and not measured on Windows (owed to "Windows-only live measurements"); `send`'s use
new: one record round, six starts); `send`'s use
old: five starts each), not measured on Windows, where they are not relied (owed to "Windows-only live measurements"). The real
new: five starts each). The real

@fact id=fct-jzpbbnum expect_rev=1
old: `batch-script-child`, `program-not-found`
new: `program-not-found`

@fact id=fct-44n3w6o5 expect_rev=1
old: program resolution (the `.cmd`/`.bat` refusal, which writes nothing), collision check, pinned copy and plugin folder, the ConPTY sideload (Windows x64, fail-open), version gate
new: program resolution (a given path as given, a bare name on `PATH`; it writes nothing), collision check, pinned copy and plugin folder, version gate
old: every step has its place (the sideload step added by 2026-09-29-sideloaded-conpty): program resolution → collision → pinned copy and plugin → ConPTY sideload (`run.conpty_sideload`, Windows: `pin_companions` + the absolute-path pre-load; `outcome` `loaded` · `hash-mismatch` · `strict-modes-failed` · `unreadable` · `load-failed` · `not-built`, never a refusal; its held handles live until `spawn_child` returns) → strip plan
new: every step has its place: program resolution → collision → pinned copy and plugin → strip plan
old: settings override (Unix only; §Occupied Resources
new: settings override (§Occupied Resources

@fact id=fct-uzcv7hop expect_rev=1
old: then, on Linux and Windows, compares
new: then compares

@fact id=fct-xqkfyxga expect_rev=1
old: portable-pty, tracing, windows-sys and libc
new: portable-pty, tracing and libc

@fact id=fct-5gkabtaw expect_rev=1
old: holds `resolve_program`'s two refusals, `BatchScriptChild` and `ProgramNotFound`, each with a fixed message: `resolve_program` returns
new: holds `resolve_program`'s one refusal, `ProgramNotFound`, with a fixed message: `resolve_program` (a given path as given, a bare name on `PATH`) returns

@fact id=fct-23yj6ah3 expect_rev=1
old: -<short content hash>/viola(.exe)` if
new: -<short content hash>/viola` if
old: and is left as found. On Windows x64 the binary also embeds Microsoft's ConPTY companions (`src/conpty.rs`: `include_bytes!` of `vendor/conpty/<version>/x64/{OpenConsole.exe,conpty.dll}`; the package version, the nupkg SHA-256 and the two file SHA-256s have that file's text as their one home) and writes them write-if-absent to `bin/<version>-<hash>/conpty/` (`viola_state::pin::pin_companions`; never overwritten). At every start each companion is opened with `FILE_SHARE_READ` only and re-hashed in full against its pin, and the handles are held until the spawn. Unlike the exe, a mismatched companion never refuses the start: it is left as found, never loaded, and the child runs on the inbox ConPTY ([PTY]). The subdirectory, not the pinned dir itself, because the pinned dir is first on the child's PATH. The first start of a version writes the two files (1 176 216 B); as measured at chunk 2026-09-29-sideloaded-conpty on the dev host under a parallel test suite, `pin_companions` took a median 1 193 ms there (each write ~0.55 s), while CI's perf and test jobs stayed green (ci#36566391084) — the root tests seed their homes with the companions instead (test-plan §5), and no bound was raised.
new: and is left as found.
old: live-round.md`; its Windows leg owed to "Windows-only live measurements"). Hooks
new: live-round.md`). Hooks

@fact id=fct-4jk25cex expect_rev=1
old: the ruled limit); its Windows leg is owed to "Windows-only live measurements". Rationale
new: the ruled limit). Rationale
old: and Runs C and D, and Run B on Windows, the user's own status line (on Unix Run B's status line is
new: and Runs C and D the user's own status line (Run B's status line is

@fact id=fct-onw4pywk expect_rev=1
old: `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch`
new: `stale-heartbeat` · `pinned-hash-mismatch`

@fact after=fct-onw4pywk file=established-decisions gid=new:dec-scale-product kind=values subject=dec-scale-product
- **[Scale / Product] Linux only.** The core runs on any current Linux distribution, CI on `ubuntu-latest`; the window features of 0.2.0 target Hyprland. No Windows or macOS code, test, fixture or CI job stands in the tree; that code is recoverable at the tag `multi-os-final` (`ecbe7ab3`).

@fact id=fct-h3mcbocf expect_rev=2
old: dec-cli-conventions · conv-refusal-reason
new: dec-cli-conventions · dec-scale-product · conv-refusal-reason

@pattern id=arch3-01
fixed: evaluated for the Windows, macOS and Linux target triples
@pattern id=arch3-02
fixed: the fallback is `TerminateProcess`
@pattern id=arch3-03
fixed: reports `conpty-sideload` after a successful pre-load
@pattern id=arch3-04
fixed: restricts its DLL search to System32
@pattern id=arch3-05
fixed: 0.9.0 is excluded because of the Windows garbage-read bug
@pattern id=arch3-06
fixed: on the three CI OSes
@pattern id=arch3-07
fixed: separate from logs because append + exclusive lock fails on Windows
@pattern id=arch3-08
fixed: is retried every 10 ms, re-persisting the same temp file
@pattern id=arch3-09
fixed: `place_private_new_placed_copy_runs_at_once` runs it on Windows
@pattern id=arch3-10
fixed: the client connects only through the SQOS Identification open
@pattern id=arch3-11
fixed: macOS caps socket paths at about 104 bytes
@pattern id=arch3-12
fixed: one API over named pipes and Unix sockets
@pattern id=arch3-13
fixed: none on macOS, where `SERVER_PID_REPORTED` is false
@pattern id=arch3-14
fixed: is `!reported || observed == Some(own)`
@pattern id=arch3-15
fixed: On macOS no peer pid is reported
@pattern id=arch3-16
fixed: off Unix it returns `Ok(())`
@pattern id=arch3-17
fixed: the long hold's Windows leg
@pattern id=arch3-18
fixed: On Windows the platform decides what arrives
@pattern id=arch3-19
fixed: the wrap is Unix only
@pattern id=arch3-20
fixed: a Windows host on twenty-one
@pattern id=arch3-21
fixed: relied only where the status line is wrapped
@pattern id=arch3-22
fixed: 21 on Windows, the last `[21/21]
@pattern id=arch3-23
fixed: On a wrapped OS (Unix)
@pattern id=arch3-24
fixed: reads verified on Windows only
@pattern id=arch3-25
fixed: npm-shim → `claude.exe` resolution
@pattern id=arch3-26
fixed: not measured on Windows (owed to
@pattern id=arch3-27
fixed: the `.cmd`/`.bat` refusal, which writes nothing
@pattern id=arch3-28
fixed: the ConPTY sideload (Windows x64, fail-open)
@pattern id=arch3-29
fixed: (Unix only; §Occupied Resources
@pattern id=arch3-30
fixed: on Linux and Windows, compares the connected peer's pid
@pattern id=arch3-31
fixed: portable-pty, tracing, windows-sys and libc
@pattern id=arch3-32
fixed: two refusals, `BatchScriptChild` and `ProgramNotFound`
@pattern id=arch3-33
fixed: /viola(.exe)
@pattern id=arch3-34
fixed: the binary also embeds Microsoft's ConPTY companions
@pattern id=arch3-35
regex: Windows leg (is )?owed to "Windows-only live measurements"
@pattern id=arch3-36
fixed: and Run B on Windows, the user's own status line
@pattern id=arch3-37
fixed: `stale-heartbeat` · `batch-script-child`
@pattern id=arch3-38
fixed: its Windows reading owed on
