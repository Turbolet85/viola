@task_prose part=chunk
Three pure moves the Epoch 5 code audit ranked sixth, with no behaviour changed by any of them.
Move one splits the fleet re-open loop of viola up into its plan (refusal, order, skip) and its act (spawn, wait, line).
Move two takes the switch's respawn out of the pty pump into a module of its own in viola-pty, and the pump's inline test module out with it.
Move three takes the window step call and the trust step out of the shared start of run, revive and adopt, and puts them beside the window step and the trust record. The start's test module leaves the file too.
Each move re-measures its files by the audit's own size and complexity measure, so that the next audit's over-ceiling and over-800 rows no longer list them.
@task_file n=1 path=src/cmd/up.rs mark=edits
what: the viola up verb, the fleet re-open loop that re-opens logged sessions in order with their outcome lines — edits: the function upped, split into its plan (refusal, order, skip) and its act (spawn, wait, line)
@task_file n=2 path=crates/viola-pty/src/pump.rs mark=edits
what: the pty pump of viola-pty, the main loop that carries the child's terminal and the switch's respawn — edits: the switch's respawn (the restart code), Pumping::run's respawn arm, the inline test module
@task_file n=3 path=src/cmd/run.rs mark=edits
what: the shared start of run, revive and adopt (and through them open and up), the boot path with its window step and trust step — edits: run::start's window step call open_window, its trust step, the file's test module
@task_file n=4 path=src/run/window.rs
what: the window step, the placement of a started session's window, which the moved window and trust steps land beside
@task_item n=1 part=needs
The restructure's invariant. Three sets of units move, and each keeps doing what it did. First, upped's refusal, order and skip go to its plan, and its spawn, wait and line go to its act. Second, the switch's respawn goes into its own viola-pty module, and the pump's inline tests go with it. Third, run::start's window step call and its trust step go beside the window step and the trust record, and the start's tests go with them. What holds the invariant: every existing layer stays green with no test body edited, and each moved test module has the same number of cases before and after.
@task_item n=2 part=needs
The size measure: the tokei code lines of each .rs file, with the file's inline cfg(test) module counted in it. After the moves, the up verb's file, the pump's file, the start's file and every file the moves create each read 800 code lines or under.
@task_item n=3 part=needs
The complexity measure: the rust-code-analysis cognitive sum of each function. Every function of the up verb's file, and of any file its parts land in, reads 15 or under, so the plan and the act of the split re-open loop each stay inside that bound.
@task_item n=4 part=needs
The respawn moves out of the pump's main loop, as the audit's M2 proposal words it: keep the respawn out of the pump's main loop. Pumping::run reads 17 today, and its figure after the move is measured.
@task_item n=5 part=needs
A moved test module is split, where its size needs it, into at least two files of at most 800 code lines each. Every moved function keeps its tests with it. No test is deleted, merged or weakened to fit a size bound.
@task_item n=6 part=needs
Tests reach a moved unit only through a re-pointed path, module or name, and the plan names each re-point. No test body changes. Where a test, a contract census, a lint list or the mod tree reads a path, module name or text that a move changes, that reader is re-pointed.
@task_item n=7 part=needs
The only function the chunk may add is one that the split of upped needs. It does no refactor the audit did not name: not the start prologue, not the refusal helper, not verify, not the window code's logic, and not the harness r-scripts.
@task_item n=8 part=needs
The start is the shared boot path of run, revive and adopt, and through them of open and up. Because that path changes, the plan holds the harness smoke trio as operator entries rather than running them.
@task_item n=9 part=needs
The chunk's checks run nothing live: no live start, no window, no compositor dispatch, no real claude, no live arm of any harness r-script and no mutation run. Nothing is read of the operator's real claude or viola home directories.
@task_item n=10 part=needs
The mutation baseline of the root crate's diff (326 caught, 0 missed at the operator's pre-CI commit) stays reachable. Tests move with the functions whose sites the next audit's diff reads.
@task_item n=11 part=needs
A plan step that asserts what existing code does cites the symbol and the line it read.
@task_item n=12 part=needs
A master row or leaf that names a moved path or module is an expected wrap amendment, not a change this chunk makes to the plan.
@task_item n=1 part=not_needed
brand-identity: the strip-bay direction, the readback box and the strip fields. A pure move adds or changes no visible element.
@task_item n=2 part=not_needed
color-palette, typography and spacing: the amber accent, bold, dim and column spacing. The moves print no new line and restyle none.
@task_item n=3 part=not_needed
iconography and the viola list board and table contracts: list columns, row order, NAME cut, skip counts and colour spans. The list verb is not reached.
@task_item n=4 part=not_needed
surface-web-spa, depth-strategy and border-radius: viola renders no page.
@task_item n=5 part=not_needed
The send readback, the send mirror, and the wait verb's output contracts. Neither the send verb nor the wait verb is reached.
@task_item n=6 part=not_needed
The output contracts of the link and unlink verbs and their --json ok documents. Those verbs are not reached.
@task_item n=7 part=not_needed
The verify verb's output and refusal contracts. The scope leaves verify out of the moves.
@task_item n=8 part=not_needed
The wrapper fault --json detail, wrapper socket and peer reply screening contracts. They govern the MCP and socket replies, which the moves do not reach.
@task_item n=9 part=not_needed
The command names register. No verb is added, renamed or hidden.
@task_item n=10 part=not_needed
self-validation-protocol: the squint, swap and readback-box signature checks are design reviews of new visible output, and the chunk adds none.
