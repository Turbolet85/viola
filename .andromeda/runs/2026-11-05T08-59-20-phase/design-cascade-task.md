@task_prose part=chunk
Sets and pushes the tag multi-os-final on the last three-OS commit before any removal, then removes every cfg(windows) and target_os = "macos" arm with its tests and fixtures, the four Windows-only files, vendor/conpty, and the cfg(windows) dependencies with their deny.toml entries.
Removes the Windows and macOS jobs from ci.yml and nightly.yml, deletes windows-mutants.yml, and drops any harness or pre-push step that runs only for those OSes; a cfg(unix) arm whose only sibling left loses its gate.
Re-points or retires the census of OS-gated tests, and makes the two e2e-home free-space harness cases decide on a figure the test controls, so msrv is green.
Re-reads at take-up that no multi-os tag stands and that the four files hold 1 146 lines; P3 measures the cfg pattern count. No Linux behaviour, exit code, closed word or wire contract moves.
The masters' OS target becomes Linux only; the plan names the amendments (design-system where it names a Windows terminal behaviour among them) as expected, the wrap writes them.
@task_file n=1 path=src/conpty.rs
@task_file n=2 path=crates/viola-pty/src/sideload.rs
@task_file n=3 path=tests/conpty_sideload.rs
@task_file n=4 path=tests/contract_windows_mutation_scope.rs
@task_file n=5 path=vendor/conpty
@task_file n=6 path=Cargo.toml mark=edits
what: the [target.'cfg(windows)'] dependencies
@task_file n=7 path=deny.toml mark=edits
what: the entries of the removed cfg(windows) dependencies
@task_file n=8 path=.github/workflows/ci.yml mark=edits
what: the Windows and macOS jobs (test, lint, perf, release), the two pwsh shim steps
@task_file n=9 path=.github/workflows/nightly.yml mark=edits
what: the Windows and macOS jobs
@task_file n=10 path=.github/workflows/windows-mutants.yml
@task_file n=11 path=tests/contract_os_gated_tests.rs mark=edits
what: the census of OS-gated tests, re-pointed or retired with the gates it counted
@task_file n=12 path=crates/viola-e2e/src/harness/mod.rs mark=edits
what: the cases e2e_home_backing_gone_target_is_made_again_owner_only and e2e_home_backing_owner_only_target_is_kept_untouched, the free-space floor panic message
@task_file n=13 path=scripts/agent-run.ps1
@task_item n=1 part=needs
Surface: cli, its Platform list (Windows Terminal, conhost, macOS and Linux terminals): the masters' OS target change lands here as an expected amendment, leaving Linux terminals as the one platform.
@task_item n=2 part=needs
Surface: cli, Toolkit: the Windows VT enabling through windows-sys SetConsoleMode is a cfg(windows) arm of the SGR module that leaves, and windows-sys is a cfg(windows) dependency that leaves with its deny.toml entry; the hand-written SGR module and std IsTerminal TTY detection stay.
@task_item n=3 part=needs
The colour decision order (Tokens, platform-specific): its step for a failed Windows VT enable and the 256-colour depth rule's Windows clause go with the Windows arm, while the remaining steps (--json, run, hook and mcp, NO_COLOR, TERM=dumb, non-terminal stdout) and the truecolor, 256color and 16-colour depth choice stay exactly as they behave on Linux.
@task_item n=4 part=needs
Platform-Specific Notes: the Git Bash on Windows rewritten-path warning line and the conhost-without-VT fallback name Windows terminal behaviour; each is in scope only where the removal leaves it naming code that no longer exists, listed by P3 and decided by P4 against the masters' change.
@task_item n=5 part=needs
The closed start-refusal codes of run and revive (the list under viola verify / viola run, batch-script-child among them, the .cmd / .bat refusal): no closed word changes on Linux, so a code whose producing arm is Windows-only is weighed by P3 and P4, never re-worded on Linux.
@task_item n=6 part=needs
Exit-code phraseology: the typed exit set stays 0, 1, 2, 10 to 14, 20, 21 with its human and --json forms; the removal adds no exit code and changes none of the forms.
@task_item n=7 part=needs
Exit 21 strict-modes refusals and their closed causes (strict-modes-failed, socket-dir-refused and the rest): where a cfg(unix) strict-modes or socket arm loses its gate because its Windows sibling left, its home-state and instance-state unable: / hint: pairs and --json documents stay as they are.
@task_item n=8 part=needs
hook and mcp under Platform-Specific Notes: hook keeps its contract (exit 0, no body, no stderr) and mcp writes only MCP frames on stdout, through any arm removal in their paths.
@task_item n=9 part=needs
The NEVER-log floor in the cli Per-Surface Bans: no error, hint or refusal the chunk touches prints upstream text, paths, pids or anyhow chains; the floor stands as it is.
@task_item n=10 part=needs
Doc comments, help strings and refusal text naming Windows or macOS without a cfg gate, as far as they reach CLI human output: in scope only where the removal leaves them naming removed code, decided per item by P4.
@task_item n=11 part=needs
The Design Decisions Log and its live amendments file: the design-system amendment for a Windows terminal behaviour is expected, its sidecar entry carrying the founder's ruling, written by the wrap and named by this chunk's plan.
@task_standing n=1
convention: any human output line, help string or refusal the chunk re-words stays standard phraseology, ASCII only, uncoloured except DIALOG and stale rows, results on stdout and refusals with their hint: lines on stderr
home: ## Anti-Patterns (NEVER do these) › ### Per-Surface Bans
@task_standing n=2
convention: any error or refusal the chunk re-words is a fixed message that names no path, pid or upstream text, and no stack trace prints
home: ## Surface: cli › ### Platform-Specific Notes
@task_item n=1 part=not_needed
The Color Palette and its Core Colors, Surface Scale, Text Hierarchy, Semantic Colors and Border Progression: 0.2.0 frontend inputs with no OS arm; the chunk renders no page.
@task_item n=2 part=not_needed
Typography's face choice and per-OS fallback stacks with their Linux-fallback render assertions: 0.2.0 frontend inputs, no CI step renders them, and the scope's design-system amendment reaches only Windows terminal behaviour.
@task_item n=3 part=not_needed
Spacing, Depth Strategy, Border Radius, Motion and Iconography: frontend tokens and glyph rules the removal of OS arms does not reach.
@task_item n=4 part=not_needed
Surface: web-spa: the frontend stays deferred to 0.2.0.
@task_item n=5 part=not_needed
Brand Identity: domain anchors, the readback box signature, the cocked strip and the expression level; the chunk changes no output that draws them.
@task_item n=6 part=not_needed
Component Patterns' layouts of viola list, send, wait / last, the wheel, link and gate verbs, plugin install and verify: no Linux output moves, so no column, sample line or step counter changes.
@task_item n=7 part=not_needed
The Width rules and the message-mode C0/C1 escaper: no OS arm, unchanged on Linux.
@task_item n=8 part=not_needed
Navigation Pattern: the flat verb set and the viola --help grouping stay as they are.
@task_item n=9 part=not_needed
Universal Bans and Rejected Defaults: design-intent rules about colour, dots, spinners and toasts that an OS removal does not touch.
@task_item n=10 part=not_needed
The Self-Validation Protocol (swap, squint, signature, token and sameness tests): run before presenting new UI output, which this chunk does not add.
