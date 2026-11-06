@task_prose part=session
The session made viola Linux-only by founder ruling: the OS target Windows first, macOS and Linux is retired for Linux only (core on any current distribution, CI on ubuntu-latest, the 0.2.0 window features on Hyprland), the Windows and macOS code recoverable at the pushed tag multi-os-final (ecbe7ab3).
It removed every Windows and macOS code arm, test, fixture and CI job: the ConPTY sideload (src/conpty.rs, viola-pty sideload, vendor/conpty, conpty_sideload test), the Windows named-pipe endpoint and SQOS/SDDL controls, the macOS $TMPDIR socket dir and macOS no-peer-pid carve-out, the .cmd/.bat refusal batch-script-child, the Windows registry env reader, the Windows VT enable (rung 7) and depth branch, the console-control stop arm, the CASE_INSENSITIVE fold, the Windows relied set of twenty-one, and the windows-sys dependency.
CI: the windows-2025 and macos-latest legs, windows-mutants.yml, scripts/junit-os-sets.py, the junit-os artifacts, the pwsh shim steps and scripts/agent-run.ps1 are gone; ci.yml jobs keep strategy.matrix.os with the single leg ubuntu-latest; workflows 3 to 2 (ci.yml, nightly.yml unchanged).
Counts moved: revive --json refusal codes 11 to 10, verify refusal codes 6 to 5, process-exit detail enum 20 to 19, diag-line os enum 3 words to linux alone, pty_backend always openpty, deny targets 3 triples to 1, nextest kill-line binaries 27 to 26.
Measured: coverage 3383 passed, 0 failed, 0 skipped; the a11y tui boundary and P6 cases run inside coverage on ubuntu-latest, none skipped; Linux human output byte-identical (rungs 1 to 6, stale-row witness, HELP, CAPTION).
A RUSTSEC-2026-0441 CARRY on Version done-check will measure moving portable-pty off =0.8.1; the pin itself stays =0.8.1 now.
@task_file n=1 path=.github/workflows/ci.yml
@task_file n=2 path=.github/workflows/windows-mutants.yml
@task_file n=3 path=scripts/agent-run.ps1
@task_file n=4 path=scripts/agent-run.sh
@task_file n=5 path=scripts/junit-os-sets.py
@task_file n=6 path=scripts/conpty-vendor.sh
@task_file n=7 path=src/conpty.rs
@task_file n=8 path=crates/viola-pty/src/sideload.rs
@task_file n=9 path=crates/viola-pty/src/lib.rs
@task_file n=10 path=vendor/conpty/1.24.260710001/x64/conpty.dll
@task_file n=11 path=vendor/conpty/1.24.260710001/x64/OpenConsole.exe
@task_file n=12 path=tests/conpty_sideload.rs
@task_file n=13 path=tests/tui_passthrough.rs
@task_file n=14 path=tests/tui_wheel.rs
@task_file n=15 path=tests/tui_pty_seam.rs
@task_file n=16 path=tests/cross_list_rows.rs
@task_file n=17 path=tests/cli_verify.rs
@task_file n=18 path=tests/cli_revive.rs
@task_file n=19 path=tests/cross_exit_causes.rs
@task_file n=20 path=tests/cli_program_resolution.rs
@task_file n=21 path=tests/security_negatives_server_identity.rs
@task_file n=22 path=tests/contract_os_gated_tests.rs
@task_file n=23 path=tests/hook_fail_open.rs
@task_file n=24 path=tests/contract_windows_mutation_scope.rs
@task_file n=25 path=src/human/style.rs
@task_file n=26 path=src/cmd/run.rs
@task_file n=27 path=src/cmd/verify.rs
@task_file n=28 path=src/cmd/verify/stop.rs
@task_file n=29 path=crates/viola-agent-claude/src/lib.rs
@task_file n=30 path=crates/viola-channel/src/endpoint.rs
@task_file n=31 path=crates/viola-channel/src/server/win.rs
@task_file n=32 path=crates/viola-state/src/liveness.rs
@task_file n=33 path=crates/viola-state/src/strict.rs
@task_file n=34 path=crates/viola-e2e/src/harness/run/mutants.rs
@task_file n=35 path=crates/viola-e2e/src/harness/run/mutants/scratch.rs
@task_file n=36 path=schemas/diag-line.v1.json
@task_file n=37 path=.config/nextest.toml
@task_file n=38 path=deny.toml
@task_item n=1 part=touches
The OS target: every claim naming three OSes, all three OS legs, Windows, macOS and Linux, or a per-OS leg is stale; viola is Linux only, CI on ubuntu-latest alone. This reaches the §1 CI integration line, the §4 cli tool-reach row, the P4 and P6 path scenarios, the §8 error-recovery verification, the §9 platform matrix and pipeline integration, and the §10 Standard+ invariant.
@task_item n=2 part=touches
The §9 CI matrix [windows-2025, macos-latest, ubuntu-latest] is now [ubuntu-latest]; the per-OS test job is one leg (jobs keep their matrix.os form), so each leg's own gate verdict is a single verdict; ci.yml and nightly.yml are the only workflows.
@task_item n=3 part=touches
The manual screen-reader pass: NVDA with Windows Terminal or conhost and VoiceOver with Terminal.app are retired with their OSes; Orca with GNOME Terminal is the remaining Linux target. This reaches the §1 screen-reader test pattern, the screen-reader-priority trigger assertion (NVDA / VoiceOver / Orca), the §4 supplemental-manual column (cli and tui rows) and the §7 per-surface SR table (cli Windows / macOS / Linux row, tui Windows NVDA row).
@task_item n=4 part=touches
The harness runner: scripts/agent-run.ps1 is deleted, scripts/agent-run.sh run is the harness alone; the §1 CI integration runner clause (or .ps1) and the §11 CI anti-pattern glob scripts/agent-run.* run name a removed shim pair.
@task_item n=5 part=touches
The §1 tui passthrough entity: the clause no viola literal on every OS, the child's own bytes on Unix, its Windows byte reading owed to Windows-only live measurements is stale; that route entry is retired and the Windows reading left with the ConPTY code, so the panic boundary is a Linux statement.
@task_item n=6 part=touches
P4 wheel: the Windows win32-input-mode fact (an injected mouse report arriving as typed keys takes the wheel, F-W3, on windows-2025) is retired with the ConPTY; this reaches the P4 required focus order, the §4 P4 scenario (mouse/resize not moving the wheel on Linux and macOS) and the §11 Keyboard anti-pattern's ConPTY clause; on Linux focus, mouse and resize sequences never move the wheel.
@task_item n=7 part=touches
P6 stale dim (path-p6-stale-dim): Unix legs only, ubuntu-latest and macos-latest, the outer-PTY witness not run on windows-2025 is stale; the witness cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal is now ungated and runs on ubuntu-latest.
@task_item n=8 part=touches
The §1 revive --json entity names batch-script-child among the start refusals; that refusal is retired (AgentError::BatchScriptChild gone, a given path as given, a bare name on PATH), revive's closed refusal codes 11 to 10; pinned-hash-mismatch, squatted-name and the program-not-found internal-error stand.
@task_item n=9 part=touches
§8 error-recovery verification (cognitive-errors): the server-verify peer pid witness on windows-2025 and ubuntu-latest, macOS reporting no peer pid with an euid mismatch alone, and the socket-directory refusal on ubuntu-latest and macos-latest are stale; SERVER_PID_REPORTED and PEER_EUID_REPORTED are folded, the macOS carve-out retired, every such witness runs on ubuntu-latest.
@task_item n=10 part=touches
Refusal counts: verify refusal codes 6 to 5 and human::refuse sites run 8 to 7, verify 6 to 5 (batch-script-child gone); any cognitive-errors entity counting refusals, refusal sites or naming the .cmd/.bat human pair (the claude CLI is a .cmd or .bat script / pass the real executable, not a .cmd or .bat shim) is stale.
@task_item n=11 part=touches
The socket directory: forms are now XDG_RUNTIME_DIR/viola/ and /tmp/viola-<uid>/; the macOS TMPDIR/viola/ form and the Windows named pipe are retired; any socket-directory refusal text or citation naming them is stale.
@task_item n=12 part=touches
The tui driver: the portable-pty outer PTY now always uses openpty (pty_backend words 3 to 1); ConPTY sideload, conpty_sideload test and run.conpty_sideload step are gone; any a11y citation of a ConPTY witness is stale; portable-pty stays =0.8.1 (a CARRY may move it at Version done-check).
@task_item n=13 part=touches
Visual (visual-sgr): the Windows VT enable (decision rung 7) and the Windows depth branch are retired; rungs 1 to 6 and the depth rule are unchanged, Linux output byte-identical; only an entity naming Windows VT, conhost or the Windows depth branch is touched.
@task_item n=14 part=touches
§10 invariant: the epoch-boundary mutation audit lost its Windows leg (windows-mutants.yml, viola-mutants-scratch, HOST_SCRATCH, scratch_bytes); run --mutants itself stays; any a11y wording naming the Windows mutation leg is stale.
@task_item n=15 part=touches
The §3 contract keys keyboard-test-harness, a11y-testing-tool-pick and ci-integration carry the OS-leg and .ps1 wording (sweep counts for a11y: os-target 19, wolm 3, bsc 3, sideload 8, os-enum 3, shim-pair 1, win-ci 16, macos-gap 3).
@task_item n=16 part=touches
Indirect, through the census tests/contract_os_gated_tests.rs: a test gate naming windows or target_os macos now reads as a fault; gates whose only sibling left were made unconditional (run_refuses_a_stale_name, the hook-fail-open and handlers impostor cases, security_negatives_server_identity.rs), so any a11y test inventory listing per-OS gating of these is stale.
@task_item n=17 part=touches
verify step lines: [NN/24] stands on Linux; LedgerRow::relied() is all twenty-four rows on every build (the Windows relied set of twenty-one retired); the shim-resolution row now reads claude --version answering; a P6 entity naming a Windows relied count is stale.
@task_item n=18 part=touches
Any a11y wording citing the diag-line os word as windows or macos, sideload_fallback, or the process-exit detail at 20 values is stale (os is linux alone, detail 19 members).
@task_fact n=1 slug=a11y-scope-summary
fact: The platform is Linux only: the core on any current distribution with CI on ubuntu-latest, the 0.2.0 window features on Hyprland, the Windows and macOS code removed and recoverable at the tag multi-os-final (ecbe7ab3).
@task_item n=1 part=untouched
The A11y tier Standard (1) and the rule that no WCAG conformance claim rests on assert_cmd / trycmd / portable-pty results: unchanged by an OS removal.
@task_item n=2 part=untouched
The not-triggered list (wcag-mapping, regulated-compliance, cognitive-accessibility, motion-sensitive, multi-language, target-size) and the compliance cross-check naming no regime: nothing in the session reaches them.
@task_item n=3 part=untouched
The CAPTION literal and caption order NAME · LIVE · STATUS · WHEEL · DIALOG · CLI for viola list, and the -- UNWRAPPED - READ-ONLY -- separator: byte-identical on Linux.
@task_item n=4 part=untouched
The not-assertable entities (plugin hooks, stdio MCP server, wrapper IPC, internal crates): the IPC transport lost its Windows arm but stays ipc-internal with no UI surface; no crate added or removed.
@task_item n=5 part=untouched
The gate --require coverage,doctest breach conditions, the ban on a new suite enum value such as a11y, the zero-flakiness rule and the harness run JSON suites[].failures[] landing: unchanged.
@task_item n=6 part=untouched
The §8 timeout-extensions list of time bounds (wait timeout, MCP wait cap, dialog hold, send readiness gate, confirmation windows, status-line deadline, agents read, budget pause): none moved.
@task_item n=7 part=untouched
The §8 plain-language fixed copy set and the rule that kebab-case codes (not-delivered, human-typing, unverified-cli) sit beside the plain word unable, and the unknown exit 14 hint-free exception: unchanged.
@task_item n=8 part=untouched
The §11 Strategy, Motion, SLO and Universal anti-patterns (no spinners, no retry-once, no soft SLO, no manual-only review, no parsing of the rendered claude TUI): unchanged.
@task_item n=9 part=untouched
The send Git Bash rewritten-path warning stays (it runs on Linux); the human-typing refusal of send and pause / release behaviour are unchanged.
