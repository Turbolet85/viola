@fact id=fct-56elcbkj expect_rev=1
old: `scripts/agent-run.sh run` (POSIX) or `scripts/agent-run.ps1 run`, followed by `gate --require coverage,doctest` (skips never allowed), on every leg of `ci.yml`'s `test` job.
new: `scripts/agent-run.sh run`, followed by `gate --require coverage,doctest` (skips never allowed), on `ci.yml`'s `test` job (matrix `[ubuntu-latest]`).

@fact id=fct-gcqo3m4v expect_rev=1
old: (CI: the per-OS `test` job's `coverage` suite).
new: (CI: the `test` job's `coverage` suite, matrix `[ubuntu-latest]`).

@fact id=fct-dlpkde76 expect_rev=1
old: (one instrumented nextest run), on all three OS legs.
new: (one instrumented nextest run), on the `ubuntu-latest` leg.

@fact id=fct-yftct3hq expect_rev=1
old: run inside the per-OS `test` job's `coverage` suite
new: run inside the `test` job's `coverage` suite (matrix `[ubuntu-latest]`)

@fact id=fct-tocuc2rh expect_rev=1
old: cli: NVDA + Windows Terminal/conhost; VoiceOver + Terminal.app; Orca + GNOME Terminal.
new: cli: Orca + GNOME Terminal.
old: tui: NVDA + Windows Terminal. The wrapped
new: tui: Orca + foot on Hyprland. The wrapped

@fact id=fct-t3p7hz2f expect_rev=1
old: the manual NVDA / VoiceOver / Orca pass stays founder-owned
new: the manual Orca pass stays founder-owned

@fact id=fct-avr5o2ub expect_rev=1
old: On Linux and macOS it asserts no `wheel` record and the bytes reaching the child unchanged. On `windows-2025` it asserts the platform fact (the founder's live ruling F-W3, 2026-10-04, relayed by the overseer, "the error only ever favours the human"): the inbox ConPTY outer terminal swallows the focus reports, and under the sideloaded ConPTY's win32-input-mode (`ESC[?9001h`) the injected mouse report arrives as win32 key-down records and takes the wheel (as measured at ci#37227518624); a mouse report from a real Windows terminal is measured live at the working-route entry "Windows-only live measurements". A third
new: It asserts no `wheel` record and the bytes reaching the child unchanged. A third
old: contains no viola-originated bytes. On every leg that means none of viola's own message literals (diagnostics, refusal text, `hint:`). On the Linux and macOS legs it also means no SGR or cursor control the child did not emit:
new: contains no viola-originated bytes: none of viola's own message literals (diagnostics, refusal text, `hint:`) and no SGR or cursor control the child did not emit:
old: while the child holds the terminal, green on the three CI runners (chunk 2026-10-10-viola-revive).
new: while the child holds the terminal, green in CI (chunk 2026-10-10-viola-revive).
old: to the outer-PTY stream, green on the three CI runners (chunk 2026-10-10-statusline-pass-through, run 38080855246). On Unix that start writes the settings override and passes `--settings`; on Windows it writes none.
new: to the outer-PTY stream, green in CI (chunk 2026-10-10-statusline-pass-through, run 38080855246). That start writes the settings override and passes `--settings`.
old: `budget-paused`, green on the three CI runners (chunk 2026-10-10-budget-governor, run 38091081413).
new: `budget-paused`, green in CI (chunk 2026-10-10-budget-governor, run 38091081413).
old: The focus / mouse / resize clause has no budget-specific case. On Windows the check is that viola's own literals are absent, because the ConPTY host itself emits bytes on every spawn and re-renders nested output — neither viola's bytes nor the child's. The inbox host (kernel32's ConPTY, conhost) emits `ESC[?9001h ESC[?1004h ESC[?25l ESC[2J ESC[m ESC[H`, an OSC 0 title holding the exe path and `ESC[?25h` (as measured at chunk 2026-09-25-pty-wrapper-on-windows research fact 4). The sideloaded host, `bin/<key>/conpty/OpenConsole.exe` (the Windows x64 default, `pty_backend` `conpty-sideload`), emits a different preamble, `ESC[1t ESC[c ESC[?1004h ESC[?9001h`, and `ESC[?1004l ESC[?9001l` at exit (as measured at chunk 2026-09-29-sideloaded-conpty, `evidence/da1-stall.md`); its DA1 query `ESC[c` is answered by the terminal, or in the tests by the piped driver, never by viola, which stays silent. The Windows check runs on both backends: the default case on the sideload, and `conpty_sideload`'s tampered-companion case on the inbox fallback. The check never
new: The focus / mouse / resize clause has no budget-specific case. The check never
old: Each of the three boundary clauses is its own nextest case on all three OS legs.
new: Each of the three boundary clauses is its own nextest case on the `ubuntu-latest` leg.

@fact id=fct-v725cwjk expect_rev=1
old: `scripts/agent-run.sh logs` / `scripts/agent-run.ps1 logs` merges
new: `scripts/agent-run.sh logs` merges

@fact id=fct-i4z6jsd7 expect_rev=1
old: 8 in `detail-hook.ndjson`, all 3 CI OSes)
new: 8 in `detail-hook.ndjson`, on CI: then all 3 OS legs, now `ubuntu-latest` alone)
old: each over 4 096 B, all 3 CI OSes (chunk
new: each over 4 096 B, on CI: then all 3 OS legs, now `ubuntu-latest` alone (chunk
old: POSIX `O_APPEND` fixes the offset per write; on Windows, `append(true)` opens with `FILE_APPEND_DATA` only. Non-interleaving
new: POSIX `O_APPEND` fixes the offset per write. Non-interleaving
old: a tests-owned concurrent-append check on windows-2025, macos-latest and ubuntu-latest asserts
new: a tests-owned concurrent-append check on `ubuntu-latest` asserts

@fact id=fct-brueynpb expect_rev=1
old: a `pty.spawn` / ConPTY spawn failure is an `Err`
new: a `pty.spawn` (openpty) spawn failure is an `Err`

@fact id=fct-r43ltybn expect_rev=1
old: Permissions: files 0600 in 0700 directories on Unix; the home DACL (user, SYSTEM, Administrators) on Windows. The tests secret-scan test enforces both.
new: Permissions: files 0600 in 0700 directories; the tests secret-scan test enforces it.

@fact id=fct-6bbewo2y expect_rev=1
old: it creates it first: 0700 on Unix, and on Windows the explicit protected user + SYSTEM DACL that security requires for a `--home` outside `%USERPROFILE%`.
new: it creates it first at 0700.
old: (owner, mode / DACL, not a symlink;
new: (owner, mode, not a symlink;
old: Only then create `<home>/diagnostics/` with `DirBuilderExt::mode(0o700)` on Unix; on Windows it inherits the already-verified home DACL.
new: Only then create `<home>/diagnostics/` with `DirBuilderExt::mode(0o700)`.
old: get the same checks as the home: on Unix, `lstat` owner == `geteuid()`, `mode & 0o077 == 0`, not a symlink (files opened with `O_NOFOLLOW`); on Windows, owner and DACL.
new: get the same checks as the home: `lstat` owner == `geteuid()`, `mode & 0o077 == 0`, not a symlink (files opened with `O_NOFOLLOW`).
old: and `OpenOptionsExt::mode(0o600)` on Unix.
new: and `OpenOptionsExt::mode(0o600)`.

@fact id=fct-rrjwwqcp expect_rev=1
old: `os` (`windows|macos|linux` from `std::env::consts::OS`)
new: `os` (`linux`, from `std::env::consts::OS`; the schema's one word)
