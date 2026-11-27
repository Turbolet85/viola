@task_prose part=chunk
Three pure moves the Epoch 5 code audit ranked sixth, with no behaviour change.
First, the fleet re-open loop of up (upped) is split into a plan (refusal, order, skip) and an act (spawn, wait, line), so that no function is over the cognitive ceiling.
Second, the switch's respawn code moves out of the viola-pty pump into its own viola-pty module. The pump's inline test module also leaves the file.
Third, run's window step and trust step move out of the shared start of run, revive and adopt, to sit beside the window code and the trust record. That file's test module also leaves it.
The chunk re-measures every touched or created file against the audit's size and complexity ceilings, and re-runs the existing tests unchanged, with equal case counts.
@task_file n=1 path=src/cmd/up.rs mark=edits
what: the up command, which re-opens the fleet of instances gone before the boot — edits: the upped function (the fleet re-open loop), split into its plan and its act
@task_file n=2 path=crates/viola-pty/src/pump.rs mark=edits
what: the viola-pty PTY pump and its main loop (Pumping::run) — edits: the switch's respawn code, moved out to its own module of viola-pty; the inline test module, moved out into test files
@task_file n=3 path=src/cmd/run.rs mark=edits
what: the shared start of run, revive and adopt (and through them open and up), run::start — edits: the window step (open_window) and the trust step, moved out; the test module, moved out into test files
@task_file n=4 path=src/run/window.rs
what: the run window code (the window step's home, through the Hyprland seam), which the moved window step and trust step are placed beside
@task_item n=1 part=needs
The restructure's invariant covers three moves. Up's fleet re-open loop keeps its refusal, its re-open order and its skips, and keeps spawning, waiting and printing one line per instance. The pump's switch respawn keeps re-spawning the child in place on a model or effort switch. The start's window step and trust step keep opening the window and writing the trust record exactly as before. The existing tests prove this: every layer green with no test body edited.
@task_item n=2 part=needs
The audit's own measure decides each move. Size is tokei code lines per .rs file, with the file's inline cfg(test) module counted in it; a file over 800 is a sizes.over_800 row. Complexity is the rust-code-analysis cognitive sum per function; a function over 15 is a complexity.over_ceiling row. Every file the moves leave behind or create stands at 800 or under, and every function at 15 or under.
@task_item n=3 part=needs
The next audit's complexity.over_ceiling list names no function of the up command, and its sizes.over_800 list names neither the pump, nor the shared start of run, nor any file the moves create.
@task_item n=4 part=needs
The split of upped is the only place the chunk may add a function. Nowhere else does it add a function or make a refactor the audit did not name.
@task_item n=5 part=needs
The respawn leaves the pump's main loop, following the audit's proposal to keep the respawn out of that loop. How far that brings the main loop's cognitive figure under the ceiling is measured after the move, not engineered.
@task_item n=6 part=needs
The respawn's new module stays inside viola-pty. It depends on no viola crate, knows no agent, and keeps its claude flag literals out of the pty seam. It joins viola-pty's mod tree so it is no orphan, and it exposes only what the pump calls.
@task_item n=7 part=needs
The window step and trust step move inside the root bin, beside the run window code and the trust record. The root bin keeps consuming only normalised events, and talks to the compositor only through the Hyprland seam crate.
@task_item n=8 part=needs
The start order is unchanged across run, revive and adopt: the window step and the trust step keep their place, and the child is still spawned last. Revive still resumes in place through the start path, and up and open still reach the start through run. The boot path changes, so the plan lists the harness smoke trio as operator entries.
@task_item n=9 part=needs
The test modules that leave the pump and the shared start each land in at least two files of at most 800 code lines. The tests reach the moved units only through a re-pointed path, module or name, and the plan names each re-point. Each moved module's test cases are counted before and after the move, and the counts are equal.
@task_item n=10 part=needs
No test is deleted, merged or weakened to fit a size bound. Every moved function keeps its tests with it. The root crate's diff-mutants baseline (326 caught, 0 missed) is the yardstick for the sites the next audit's diff reads. No mutation run is planned or run in this chunk.
@task_item n=11 part=needs
Where a test, a contract census, a lint list or a mod tree reads a path, module name or text that a move changes, it is re-pointed. A master row or leaf that names a moved path is an expected wrap amendment.
@task_item n=12 part=needs
The proof stays offline. There is no live start, no window, no compositor dispatch and no real claude, only the fake agent. There is no live arm of any r*.sh script, and nothing is read from the operator's real ~/.claude* or ~/.viola.
@task_item n=13 part=needs
The moved and new code passes the build gates as it stands. That means fmt, and clippy with warnings denied together with its print and dbg bans, tracing level macro bans, and obs_event! as the only event emitter. Every moved line also stays inside the coverage floors the harness gate enforces.
@task_item n=14 part=needs
A plan step that asserts what existing code does cites the symbol and the line it read.
@task_standing n=1
convention: each new test file runs its cases in its own isolated viola home, and its homes are removed by owner record only
home: established-decisions · occupied-resources
@task_standing n=2
convention: each new module or test file joins its crate's mod tree, so no orphan module is left
home: build-system
@task_standing n=3
convention: each OS-gated test in a moved or new test file names its reason, and no test gate names another OS
home: cross-cutting-patterns
@task_standing n=4
convention: each moved test that uses insta snapshots runs them in check mode only
home: project-directory-structure
@task_standing n=5
convention: each root test in a new test file that waits on a wrapper shares the one root wait bound, and freezes a wrapper only through the one SIGSTOP guard
home: occupied-resources
@task_item n=1 part=not_needed
The MCP server, its tools, its results and its stdin rules are not reached: no move touches the mcp crate or what it serves.
@task_item n=2 part=not_needed
Hook tiers, hook output and the hook-to-kind map are not reached: no hook code moves.
@task_item n=3 part=not_needed
The wrapper channel's methods, frames, params and fault codes are not reached: no channel method or frame changes.
@task_item n=4 part=not_needed
Normalised event kinds, event data per kind and wrapper event appends are not reached: no event is added, removed or reshaped.
@task_item n=5 part=not_needed
The capability ledger, ledger stamps and verify's refusals, exit details and probes are not reached: verify is outside the chunk's boundary.
@task_item n=6 part=not_needed
The board and list payload contracts and the agents listing are not reached: list reads state without any code that moves.
@task_item n=7 part=not_needed
config.json keys and configuration precedence are not reached: no configuration value is added or read anew.
@task_item n=8 part=not_needed
The fuzz workspace, its toolchain and its lockfile are not reached: no fuzz target moves.
@task_item n=9 part=not_needed
The release workflow, dist, actions pinning, workflow permissions and the weekly advisory and fuzz schedules are not reached: no workflow or job is added or changed.
@task_item n=10 part=not_needed
Design-philosophy absences such as a daemon, a graphical frontend, a mobile view or a usage-budget gate are not reached: nothing in the chunk comes near them.
@task_item n=11 part=not_needed
The send path, paste bounds and the confirmation of local commands (/clear, /compact) are not reached: no send code moves.
