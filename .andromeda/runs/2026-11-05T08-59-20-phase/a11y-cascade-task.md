@task_prose part=chunk
Makes viola Linux-only. It first sets and pushes the tag multi-os-final on the last three-OS commit, then removes every cfg(windows) and target_os = "macos" arm with its tests and fixtures, the four Windows-only files, vendor/conpty and the Windows target dependencies with their deny entries.
It drops the Windows and macOS CI jobs (test, lint, perf, release, nightly) and the whole windows-mutants workflow. A cfg(unix) arm whose only sibling is gone becomes unconditional, and the census of OS-gated tests is re-pointed or retired.
It makes the two e2e-home free-space floor cases decide on a figure the test controls, not the host's free space, so msrv goes green. No Linux behaviour, exit code, closed word or wire contract moves.
Re-reads at take-up: no multi-os-* tag stands, the four files total 1 146 lines, and every named path stands at HEAD. P3 re-measures the count of OS gates. The masters' OS-target change, a11y-plan included where it names a Windows terminal behaviour, is listed as expected amendments that the wrap writes.
@task_file n=1 path=src/conpty.rs
@task_file n=2 path=crates/viola-pty/src/sideload.rs
@task_file n=3 path=tests/conpty_sideload.rs
@task_file n=4 path=tests/contract_windows_mutation_scope.rs
@task_file n=5 path=vendor/conpty/
@task_file n=6 path=Cargo.toml mark=edits
what: the [target.'cfg(windows)'] dependencies
@task_file n=7 path=deny.toml mark=edits
what: the entries of the removed Windows target dependencies
@task_file n=8 path=.github/workflows/ci.yml mark=edits
what: the Windows and macOS jobs (test, lint, perf, release), the two pwsh shim steps
@task_file n=9 path=.github/workflows/nightly.yml mark=edits
what: the Windows and macOS jobs
@task_file n=10 path=.github/workflows/windows-mutants.yml
@task_file n=11 path=tests/contract_os_gated_tests.rs mark=edits
what: the census of OS-gated tests, re-pointed or retired with the gates it counted
@task_file n=12 path=crates/viola-e2e/src/harness/mod.rs mark=edits
what: the cases e2e_home_backing_gone_target_is_made_again_owner_only and e2e_home_backing_owner_only_target_is_kept_untouched, and the free-space floor panic message they hit
@task_file n=13 path=scripts/agent-run.ps1
@task_item n=1 part=needs
The CI integration for CLI output discipline and the TUI boundary: these cases run inside the test job's single instrumented coverage suite, and its gate --require coverage,doctest never allows skips. Once the Windows and macOS test jobs are gone, the ubuntu-latest leg is the only verdict, and the cases must still run there in full.
@task_item n=2 part=needs
The tui boundary on the portable-pty outer-PTY driver (path P4, the wheel): viola emits zero bytes of its own, human keystrokes are never blocked past the current atomic paste, and focus, mouse and resize sequences and terminal replies do not take the wheel. Its Linux cases stay as they are. The Windows arms leave: the win32-input-mode mouse report that takes the wheel (F-W3) and the windows-2025 platform fact.
@task_item n=3 part=needs
The boundary after a contained vt100 feed panic: no viola literal reaches the outer terminal, and the child's own bytes pass on Unix. The Windows byte reading owed to the retired entry Windows-only live measurements loses its code.
@task_item n=4 part=needs
Path P6, the CLI board: the TTY-only behaviours are asserted under the outer PTY on all three OS legs, and the dim-only-on-stale-rows witness runs on ubuntu-latest and macos-latest. After the macOS arms leave, both rest on the ubuntu leg alone.
@task_item n=5 part=needs
The error-recovery verification: trycmd pins the unable reason detail plus a final hint: layout for each refusal, and assert_cmd checks the typed exit codes and a hint-free --json. The per-OS carve-outs move: the server-verify peer pid witness (windows-2025 and ubuntu-latest), macOS's euid-mismatch-only word with no peer pid (SERVER_PID_REPORTED), and the socket-directory refusal on ubuntu-latest and macos-latest.
@task_item n=6 part=needs
The viola revive --json refusal set shared with run, batch-script-child among them (the .cmd / .bat refusal). If the removal leaves its text naming Windows code that no longer exists, P4 decides whether it changes. Its Linux exit-1 document stays as it is.
@task_item n=7 part=needs
The pinned fixed CLI strings (trycmd or literal): a copy change fails CI until the pin is updated on purpose. Any help string, doc comment or refusal text that drops a Windows or macOS word must update its pin knowingly. No Linux closed word changes.
@task_item n=8 part=needs
The visual SGR rules (visual-sgr, visual-sgr-stale-row): no SGR of viola's own under non-TTY, NO_COLOR, TERM=dumb, --json, viola run or a passed viola revive, and every span sits beside its word. A cfg(unix) arm over TTY or terminal detection that becomes unconditional must keep this behaviour on Linux unchanged.
@task_item n=9 part=needs
The CI platform matrix of section 9 ([windows-2025, macos-latest, ubuntu-latest]) and the per-pipeline-stage row run on all three OS legs. After this chunk ci.yml carries ubuntu-latest alone, and this is one of the expected a11y-plan amendments.
@task_item n=10 part=needs
The SLO invariant that the CLI output-discipline invariants (P4, P6) pass on all three OS legs, with the epoch-boundary mutation audit (run --mutants) covering the ones in crate tests. Removing windows-mutants.yml must not drop that audit's coverage on Linux. The three-leg wording is an expected amendment.
@task_item n=11 part=needs
The runner named for a11y CI: scripts/agent-run.sh run (or .ps1), and the CI anti-pattern that allows no driver outside scripts/agent-run.* run. If the PowerShell twin leaves, the sh harness stays the only runner.
@task_item n=12 part=needs
The a11y-plan text that names a Windows or macOS terminal behaviour, an expected amendment under the masters' OS-target change: the Windows byte reading and F-W3 in the scope summary, the keyboard anti-pattern and the focus order; the testing tool reach rows (NVDA with Windows Terminal or conhost, VoiceOver with Terminal.app); and the per-surface screen-reader table (cli on Windows, macOS and Linux; tui on Windows). Orca with GNOME Terminal stays the Linux target. Each amendment carries the ruling in its sidecar entry.
@task_item n=13 part=needs
The tests' zero-flakiness rule (retries 0, #[ignore] banned, no retry-once), through the e2e-home free-space floor cases: they pass or fail by the host's disk, so the fix must make them deterministic on a figure the test controls, never ignore or retry them.
@task_standing n=1
convention: every new or changed test runs inside the existing nextest suites through scripts/agent-run.sh run (in CI, the test job's coverage suite); no new suite enum value such as a11y and no separate a11y driver without a Decisions Log entry
home: ## 9. CI Integration; ## 11. A11y Anti-Patterns § CI
@task_standing n=2
convention: every CI job keeps the gate --require coverage,doctest with no skips allowed; a suite-failed, suite-skipped, suite-missing or artifact-missing breach fails the build
home: ## 10. SLO Invariants & A11y Budgets
@task_standing n=3
convention: every test is zero-flake: retries 0, no #[ignore], no retry-once policy; a flake keeps the chunk red until the root cause is fixed
home: ## 10. SLO Invariants & A11y Budgets; ## 11. A11y Anti-Patterns § SLO
@task_standing n=4
convention: a cli or tui failure lands as a nextest failure in the harness run JSON (suites[].failures[]), and no a11y artifact is uploaded of its own
home: ## 9. CI Integration
@task_standing n=5
convention: no test parses the rendered claude TUI screen or takes screenshots as a11y evidence; assertions stay on text, streams and the outer-PTY boundary
home: ## 11. A11y Anti-Patterns § Universal
@task_item n=1 part=not_needed
The frontend deferred to 0.2.0 and its a11y tooling: the chunk touches no page, and its technology is not chosen.
@task_item n=2 part=not_needed
The timeout-extension section (cognitive-timeouts) and the list of time bounds viola holds: no wait, hold, readiness or budget bound changes.
@task_item n=3 part=not_needed
Motion: the spinner ban and the motion-sensitive trigger. The chunk adds no output of its own.
@task_item n=4 part=not_needed
The triggers that do not fire (wcag-mapping, regulated-compliance, cognitive-accessibility, multi-language, target-size) and the compliance cross-check: no regime applies, and the chunk changes none of them.
@task_item n=5 part=not_needed
The consistency rule on viola list's caption order and its CAPTION literal (cognitive-consistency): no column or caption moves.
@task_item n=6 part=not_needed
Path P2's send readback mirror [RB] and path P3's answer unverified-cli refusal: their Linux text and behaviour stay as they are.
@task_item n=7 part=not_needed
The not-assertable entities (the plugin hooks, the stdio MCP server, the wrapper IPC channel, the internal crates): they have no UI surface for this chunk to move. hook keeps its contract.
@task_item n=8 part=not_needed
The ARIA landmark inventory (aria-landmarks) and the POUR signal pyramid of the strategy: they are N/A for the terminal, and no structural equivalent changes.
@task_item n=9 part=not_needed
The ban on claiming WCAG conformance for the cli or the TUI: the chunk makes no conformance claim.
