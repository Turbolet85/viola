@task_prose part=session
The session made viola Linux only: the founder ruled the OS target Windows first, macOS and Linux retired, replaced by Linux only (the core on any current distribution, CI on ubuntu-latest, the window features of 0.2.0 on Hyprland), the Windows and macOS code recoverable at the tag multi-os-final (ecbe7ab3).
It removed every Windows and macOS code arm, test, fixture, CI job and the windows-sys dependency; no crate, surface or dependency was added, and Linux human output stays byte-identical (rungs 1 to 6, the stale-row witness, HELP, CAPTION).
Retired with it: the Windows VT enable (colour decision rung 7) and the Windows depth branch, the .cmd/.bat start refusal (batch-script-child and its human pair), the Windows relied set of twenty-one, the ConPTY sideload, the console-control stop arm, the Windows registry environment reader, the Windows named pipe and the macOS socket dir.
Counts moved: revive's --json refusal codes 11 to 10, verify's refusal codes 6 to 5, human::refuse sites run 8 to 7 and verify 6 to 5, the process-exit detail enum 20 to 19, the os enum to linux alone, pty_backend to openpty alone, workflows 3 to 2, the CI matrix to ubuntu-latest alone, deny targets 3 triples to 1, licence exceptions 2 to 1.
The verify shim-resolution row now checks claude --version answering alone, and LedgerRow::relied() is all twenty-four rows on every build.
@task_file n=1 path=src/human/style.rs
@task_file n=2 path=src/cmd/run.rs
@task_file n=3 path=src/cmd/verify.rs
@task_file n=4 path=src/cmd/verify/stop.rs
@task_file n=5 path=src/cmd/send.rs
@task_file n=6 path=src/main.rs
@task_file n=7 path=src/conpty.rs
@task_file n=8 path=src/run/env.rs
@task_file n=9 path=src/run/mod.rs
@task_file n=10 path=src/panic_frames.rs
@task_file n=11 path=src/human/verbs.rs
@task_file n=12 path=src/cmd/hook/statusline.rs
@task_file n=13 path=src/cmd/client.rs
@task_file n=14 path=src/cmd/hook.rs
@task_file n=15 path=crates/viola-agent-claude/src/lib.rs
@task_file n=16 path=crates/viola-agent-claude/src/ledger.rs
@task_file n=17 path=crates/viola-agent-claude/src/statusline.rs
@task_file n=18 path=crates/viola-pty/src/lib.rs
@task_file n=19 path=crates/viola-pty/src/sideload.rs
@task_file n=20 path=crates/viola-channel/src/endpoint.rs
@task_file n=21 path=crates/viola-channel/src/server/win.rs
@task_file n=22 path=crates/viola-state/src/strict.rs
@task_file n=23 path=crates/viola-state/src/liveness.rs
@task_file n=24 path=schemas/diag-line.v1.json
@task_file n=25 path=Cargo.toml
@task_file n=26 path=deny.toml
@task_file n=27 path=deny-sync.toml
@task_file n=28 path=.github/workflows/ci.yml
@task_file n=29 path=.github/workflows/windows-mutants.yml
@task_file n=30 path=.config/nextest.toml
@task_file n=31 path=scripts/agent-run.ps1
@task_file n=32 path=scripts/agent-run.sh
@task_file n=33 path=tests/cli_verify.rs
@task_file n=34 path=tests/cross_exit_causes.rs
@task_file n=35 path=tests/cross_list_rows.rs
@task_file n=36 path=tests/cli_program_resolution.rs
@task_file n=37 path=tests/contract_os_gated_tests.rs
@task_file n=38 path=tests/conpty_sideload.rs
@task_file n=39 path=vendor/conpty/1.24.260710001/x64/conpty.dll
@task_file n=40 path=vendor/conpty/1.24.260710001/x64/OpenConsole.exe
@task_item n=1 part=touches
The cli surface Platform line naming Windows 10/11 (Windows Terminal, conhost), macOS and Linux terminals is stale: the platform is Linux terminals only.
@task_item n=2 part=touches
The cli Toolkit line on Windows VT enabling through windows-sys 0.61.2 SetConsoleMode is retired: the VT enable left src/human/style.rs and windows-sys left every manifest; the clap, IsTerminal and hand-written SGR module lines stand.
@task_item n=3 part=touches
The colour decision order's rung 7, Windows VT enable failed, is retired; rungs 1 to 6 (--json, viola run, hook and mcp, NO_COLOR, TERM=dumb, stdout not a terminal) are unchanged.
@task_item n=4 part=touches
The depth rule's Windows branch (256-colour once VT enabling has succeeded on Windows) is retired; truecolor on COLORTERM, 256-colour on a TERM containing 256color and the 16-colour fallback stand; the style test colorterm_on_windows is renamed colorterm_without_term.
@task_item n=5 part=touches
The CLI colour tokens stated at truecolor, 256-colour and 16-colour depth: any Windows depth row or vt_enable_failed row in the surface detail is stale; the token values themselves are unchanged.
@task_item n=6 part=touches
Platform-Specific Notes: the conhost without VT note is retired with the Windows VT enable; the Git Bash rewritten-path warning in src/cmd/send.rs stays (it runs on Linux), so its line stands but a framing of it as a Windows-only platform is stale wording.
@task_item n=7 part=touches
revive's closed --json refusal code list loses batch-script-child: ten codes (strict-modes-failed, already-live, no-session, session-live, cwd-missing, stale-heartbeat, pinned-hash-mismatch, squatted-name, socket-dir-refused, internal-error), was eleven.
@task_item n=8 part=touches
The run and revive start refusal causes lose the .cmd/.bat refusal and its human pair, the claude CLI is a .cmd or .bat script / pass the real executable, not a .cmd or .bat shim; any per-cause line list, hint list or count of start causes in the surface detail moves with it.
@task_item n=9 part=touches
verify's refusal codes move 6 to 5 (the BatchScriptChild arm left src/cmd/verify.rs), and human::refuse sites read run 7 and verify 5; any count of verify refusals or refusal sites in the surface detail is stale.
@task_item n=10 part=touches
viola verify's relied count: the Windows relied set of twenty-one is retired; LedgerRow::relied() is all twenty-four rows on every build, so the summary's n + m and the [NN/24] step lines are twenty-four everywhere; a Windows relied count in the surface detail is stale.
@task_item n=11 part=touches
The verify shim-resolution row ([01/24]) stays, but its check now reads claude --version answering alone (ProbeRun.program_is_script removed); a description of it as a script or shim check is stale.
@task_item n=12 part=touches
verify's stop on Ctrl-C or TERM: the Windows console-control arm (CTRL_C_EVENT / CTRL_BREAK_EVENT) left src/cmd/verify/stop.rs; SIGINT and SIGTERM are unconditional; the stopped-run output (no step line, no summary, the fixed unable: / hint: pair, exit 1) is unchanged.
@task_item n=13 part=touches
Exit-code phraseology: exit 21's six closed causes stand unchanged (pinned in tests/cross_exit_causes.rs); socket-dir-refused now covers only the Linux socket directory forms $XDG_RUNTIME_DIR/viola/ and /tmp/viola-<uid>/, the macOS $TMPDIR/viola/ form and the Windows named pipe gone.
@task_item n=14 part=touches
The diag-line schema words design-system may cite: process-start os reads linux alone (was three words), pty_backend reads openpty alone, sideload_fallback is gone, and the process-exit detail enum is 19 members (batch-script-child removed), was 20.
@task_item n=15 part=touches
Typography rationale: Bahnschrift shipping with Windows because Windows is the first target, both faces being Windows-only, and D3 making macOS and Linux supported all state the retired OS target; the product is Linux only, so per-OS stack wording and the Windows-first justification are stale, the face choice itself a 0.2.0 frontend input.
@task_item n=16 part=touches
Typography per-OS fallback stacks naming a face per OS (Windows, macOS, Linux) and the Linux fallback described as the widest case: indirectly stale through the Linux-only target, where the Linux render is the one render; the assertions that hold on DejaVu Sans Condensed and DejaVu Sans Mono stand.
@task_item n=17 part=touches
Any design-system wording of cross-platform, Windows first, three OSes, all three OS legs, Windows, macOS and Linux, or on every OS is stale on a Linux-only product: re-worded to the Linux statement or dropped, the claim it carries kept.
@task_item n=18 part=touches
Any CI reference in design-system to the windows-2025 or macos-latest legs, windows-mutants.yml, the pwsh shim steps, scripts/agent-run.ps1 or a three-OS matrix is stale: CI is ci.yml and nightly.yml on ubuntu-latest, and the shim is scripts/agent-run.sh alone.
@task_item n=19 part=touches
Any mention of ConPTY, the ConPTY sideload, conpty-sideload or the vendored conpty tree as the run passthrough backend is stale: the pty backend is openpty on every build.
@task_item n=20 part=touches
Test inventory cited by the design acceptance: the stale-row witness cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal is now ungated, tests/cli_verify.rs pins the 24-row literals ungated, and tests/cli_program_resolution.rs (the .cmd/.bat resolution cases) is deleted.
@task_item n=21 part=touches
The run environment strip: the Windows registry reader (HKCU\Environment) and the case-insensitive match are gone; the exact strip, claude_env_keep and the eleven-name identity floor stand; a case-fold or registry statement in the surface detail is stale.
@task_item n=22 part=touches
Stack traces and diagnostics: src/panic_frames.rs keeps the Unix capture as its one path with the same line shape, and the panic hook stays main's first statement; the never-print-stack-traces rule stands unchanged.
@task_fact n=1 slug=surface
fact: viola's OS target is Linux only (the core on any current distribution, CI on ubuntu-latest, the window features of 0.2.0 on Hyprland); the Windows and macOS code is removed and recoverable at the tag multi-os-final (ecbe7ab3).
@task_fact n=2 slug=typography
fact: A 0.2.0 frontend inherits a Linux-only target, its window features on Hyprland; the Linux font resolution is the render its typography must hold.
@task_item n=1 part=untouched
The Color Palette: the eight hex values, the contrast table, core colours, surface scale, text hierarchy, semantic and domain status colours and border progression; no colour value or role moved.
@task_item n=2 part=untouched
Spacing, Border Radius, Depth Strategy and Motion tokens and rationale: 0.2.0 frontend inputs no code of this session reaches.
@task_item n=3 part=untouched
Iconography: the text glyphs, the ASCII-only CLI rule and the no-icon-library rule are unchanged.
@task_item n=4 part=untouched
Brand Identity: the personality, domain anchors, readback box states, cocked strip, expression level 0.2 and design direction are unchanged.
@task_item n=5 part=untouched
The viola list strip table layout, its columns, BAY line, escaping and empty line: the list command was not edited and Linux human output is byte-identical.
@task_item n=6 part=untouched
viola wait, last, link, unlink, pause, release, answer and plugin install output lines: the lines removed from those commands were OS arms, and their Linux output is byte-identical.
@task_item n=7 part=untouched
The Anti-Patterns universal bans, rejected defaults and per-surface cli bans: none is relaxed or added; the cli colour bans still hold over rungs 1 to 6.
@task_item n=8 part=untouched
The Self-Validation Protocol (swap, squint, signature, token and sameness tests): unchanged.
@task_item n=9 part=untouched
Navigation Pattern: the flat verb list, the viola --help groups and the hidden mcp verb are unchanged.
@task_item n=10 part=untouched
The Design Decisions Log pointer to design-system-amendments.md and its archive: no log entry moved.
