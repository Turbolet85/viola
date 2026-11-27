@task_prose part=chunk
Three pure moves, ranked sixth by the Epoch 5 code audit, with no behaviour change.
One: the fleet re-open loop `upped` of viola up is split into its plan (refusal, order, skip) and its act (spawn, wait, line).
Two: the switch's respawn (restart) code and the pump's inline test module leave the pty pump file for a module of their own in the viola-pty crate.
Three: the window step and the trust step of the shared start of run, revive and adopt move beside the window code and the trust record, and that start's test module leaves the file too.
It re-measures every moved and left file against the audit's ceilings (800 code lines per file, cognitive 15 per function), and the existing test layers re-assert that behaviour is kept.
@task_file n=1 path=src/cmd/up.rs mark=edits
what: the viola up command, the fleet re-open loop that writes the up instance line — edits: the function `upped`, split into its plan and its act
@task_file n=2 path=crates/viola-pty/src/pump.rs mark=edits
what: the pty pump of the TUI passthrough, its main loop and the switch's respawn — edits: the respawn code, the respawn arm of `Pumping::run`, the inline test module
@task_file n=3 path=src/cmd/run.rs mark=edits
what: the shared start of run, revive and adopt (and through them open and up) — edits: `run::start`'s window step call `open_window`, its trust step, the test module
@task_file n=4 path=crates/viola-pty/src/lib.rs mark=edits
what: the viola-pty crate's module tree — edits: the new respawn module's declaration
@task_item n=1 part=needs
The restructure's invariant: the units moved are the fleet re-open loop of viola up (refusal, order, skip, then spawn, wait and the instance line), the switch's respawn in the pty pump, and the start's window step and trust step; each does after the move exactly what it did before, held by the existing test layers passing whole with no test body edited.
@task_item n=2 part=needs
The measure the next audit reads: tokei code lines per file with its inline test module counted in it, no file left or created over 800; rust-code-analysis cognitive sum per function, no function of the viola up command over 15.
@task_item n=3 part=needs
The one new function the chunk may add is the split of the fleet re-open loop into its plan and its act; no other new function, no refactor the audit did not name.
@task_item n=4 part=needs
How a test reaches a moved unit: only a re-pointed path, module or name, each re-point named in the plan; a moved test module's case count is equal before and after.
@task_item n=5 part=needs
No test is deleted, merged or weakened to fit a size bound; every moved function keeps its tests with it, since a move changes the sites the next diff mutation audit reads.
@task_item n=6 part=needs
Where a moved part may live: the respawn in its own module of the viola-pty crate, its exposure no wider than the pump needs; the window and trust steps beside the window code and the trust record.
@task_item n=7 part=needs
Every census, lint list or module tree that reads a moved path, module or text is re-pointed with the move, including any contract census naming the pump, the start or the up command.
@task_item n=8 part=needs
The boot path of run, revive and adopt changes shape, so the harness smoke trio is held as operator entries, never run by the chunk.
@task_item n=9 part=needs
No live start, no window, no compositor dispatch, no real claude, no live arm of a harness script, no mutation run, and nothing read of the operator's real home config, in any step or test of the chunk.
@task_item n=10 part=needs
A step that asserts what existing code does cites the symbol and the line it read.
@task_standing n=1
convention: every moved or new test file keeps its a11y assertions inside the existing suites, with no separate a11y driver, suite or artifact
home: 9-ci-integration
@task_standing n=2
convention: the CI test gate allows no skips and no retries; every moved test runs and passes without flakiness
home: 10-slo-invariants-a11y-budgets
@task_standing n=3
convention: fixed CLI strings stay pinned by their literal in the tests that move
home: 8-cognitive-accessibility · cognitive-accessibility
@task_item n=1 part=not_needed
Keyboard navigation, visual design, contrast, focus and screen-reader sections for a web or graphical frontend: viola has none, and the chunk adds no surface.
@task_item n=2 part=not_needed
The viola list board rows (colour, captions, order, stale-row escapes, unwrapped separator, NAME truncation): the chunk moves no list code.
@task_item n=3 part=not_needed
viola link and unlink, plugin install, and revive --list output contracts: none of these commands is moved or changed.
@task_item n=4 part=not_needed
MCP frame tool, frame verb and hook-call refusal contracts: the chunk moves no MCP or hook code.
@task_item n=5 part=not_needed
The manual keyboard and Orca screen-reader passes and their test setups: supplemental, gating nothing, and the chunk runs no live terminal.
@task_item n=6 part=not_needed
The structured violation JSON rows and WCAG criteria mapping: the chunk emits no new report or a11y row.
@task_item n=7 part=not_needed
The internationalization, touch-surface and user-time-limit absences, and reading-level or copy-from-design rules: the chunk writes no new user-facing text.
@task_item n=8 part=not_needed
viola verify's step line and interrupted output: verify is out of the chunk's bounds and untouched.
