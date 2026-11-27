@task_prose part=chunk
Three pure moves the Epoch 5 code audit ranked sixth, with no behaviour change.
First, the fleet re-open loop `upped` of `viola up` is split into its plan (refusal, order, skip) and its act (spawn, wait, line), every function at cognitive 15 or under.
Second, the switch's respawn (restart code) leaves the pty pump into its own module of the viola-pty crate, the pump's inline test module moving out too, every file at 800 code lines or under.
Third, the window step (open_window) and the trust step of the run start move beside the window code and the trust record, the start's test module moving out too, every file at 800 code lines or under.
It re-measures each moved and each remaining file by tokei code lines and rust-code-analysis cognitive sums, and it re-runs the whole test layers green with equal case counts.
@task_file n=1 path=src/cmd/up.rs mark=edits
what: the up verb, the fleet re-open of the saved instances — edits: the function `upped` (the re-open loop), split into its plan and its act
@task_file n=2 path=crates/viola-pty/src/pump.rs mark=edits
what: the pty pump of viola-pty, its main loop and the switch's respawn — edits: the restart code (the respawn), moved out to its own module; `Pumping::run`; its inline test module, moved out into test files
@task_file n=3 path=src/cmd/run.rs mark=edits
what: the shared start of run, revive and adopt (and through them open and up), `run::start` — edits: the window step call `open_window` and the trust step, moved out; its test module, moved out into test files
@task_file n=4 path=src/run/window.rs
what: the window code of the run start, the home beside which the window and trust steps land; its logic is not changed
@task_item n=1 part=needs
The restructure's invariant: the units moved are the re-open loop of up (refusal, order, skip, then spawn, wait, line), the pump's respawn of a switch restart, and the run start's window step and trust step (after the bind, in their place in the start order). Each does what it did before. The whole test layers stay green with no test body edited, and each moved test module has the same number of cases before and after the move.
@task_item n=2 part=needs
The size measure the next audit reads: tokei code lines per .rs file, a file's inline cfg(test) module counted in it. A file over 800 is a sizes.over_800 row. The pump, the run start and every file the moves create must be at 800 or under.
@task_item n=3 part=needs
The complexity measure the next audit reads: the rust-code-analysis cognitive sum per function. A function over 15 is a complexity.over_ceiling row. No function of the up verb, or of any file its parts land in, may be over 15. The pump main loop's figure (17 at the audit) is measured after the respawn leaves it.
@task_item n=4 part=needs
No new function beyond what the split of upped needs, and no refactor the audit did not name. The other two moves only relocate code. A moved unit is exposed no wider than its callers across the new module boundary need.
@task_item n=5 part=needs
Where a moved unit may live: the respawn goes in its own module inside the viola-pty sync crate, which stays tokio-free and builds alone without features. The window and trust steps go in the run start's window and trust area of the root crate.
@task_item n=6 part=needs
How a test reaches a moved unit: only by a re-pointed path, module or name, each re-point named in the plan. A moved test keeps its function name and its body. Every moved function keeps its tests with it, in the same package.
@task_item n=7 part=needs
A test module leaving its file, and the inline unit-test rule: the cases of the pump's and the run start's test modules move into at least two test files each, every one at 800 code lines or under. They stay unit tests of their own crate and reach private units as before.
@task_item n=8 part=needs
Every new module file joins the crate's mod tree, so the orphan-module check finds no orphan. Paths and module names a move changes are re-pointed wherever a contract census, a lint list or a mod tree reads them.
@task_item n=9 part=needs
No test deleted, merged or weakened to fit a size bound. The root crate's diff mutants read 326 caught and 0 missed at the prior commit, and the moves change the sites the next audit's diff mutants read, so every moved function's catching tests must still reach it.
@task_item n=10 part=needs
The boot path changes because the run start is shared by run, revive, adopt, open and up. The harness smoke trio (boot, run, cleanup of the test harness commands) is held as operator entries.
@task_item n=11 part=needs
A plan step that asserts what existing code does cites the symbol and the line it read.
@task_item n=12 part=needs
No live proof: no live start, no window, no compositor dispatch, no real claude, no live arm of a live-rig script, and no mutation run planned or run. Every test the moves touch keeps running under a test-owned home, reading nothing of the operator's real claude or viola homes.
@task_item n=13 part=needs
The moved code must pass the lint gate (formatting drift and clippy warnings fail the build) and the whole CI run, which was green 7/7 at the base commit.
@task_item n=14 part=needs
A master row or leaf that names a moved path or module is an expected wrap amendment. It is no reason to keep a unit where it was.
@task_standing n=1
convention: every test file a moved test module lands in holds unit tests of its own crate, under the crate's unit-test layout
home: 2-test-strategy
@task_standing n=2
convention: every module or test file the chunk creates is reached from the crate's mod tree, with no orphan module
home: 1-test-scope-summary · 9-ci-integration
@task_standing n=3
convention: a test helper shared by cases split across two files exists once and is never copied
home: 2-test-strategy · test-strategy
@task_standing n=4
convention: every new file passes formatting and clippy with warnings denied
home: 9-ci-integration · ci-integration
@task_item n=1 part=not_needed
Wrapper channel, JSON-RPC error and MCP tool contracts: no channel method, frame or tool result is touched.
@task_item n=2 part=not_needed
CLI exit codes, the CLI --json documents and the human output lines of send, answer, wait, last, list, release and revive: no verb's output or exit code changes.
@task_item n=3 part=not_needed
The fake agent's command line, receipt, hook execution and screen contracts, and the recorded fixtures: the chunk adds no fixture and changes no fake-agent behaviour.
@task_item n=4 part=not_needed
Hook performance budgets, hyperfine invocation and the hook deadlines: no hook path is touched.
@task_item n=5 part=not_needed
Property tests, fuzz targets and their corpora: no parser surface is touched.
@task_item n=6 part=not_needed
Gated live readings of window raise, placement, fleet re-open and adopt (R2 to R5): the window code's logic is unchanged, and the chunk plans nothing live.
@task_item n=7 part=not_needed
The test harness's own boundaries (session record, status document, pre-push stages and reasons, gate breaches, secret-scan classes, run documents): the harness is not changed, and is only run as the operator's smoke trio.
@task_item n=8 part=not_needed
Process log line fields, log-event corr and the event-line keys: no log line or event record is added or changed.
@task_item n=9 part=not_needed
CI artifacts, workflow permissions, SHA pinning and the supply-chain gates: no workflow or dependency changes.
@task_item n=10 part=not_needed
IPC security vectors (peer euid, socket-directory refusal, server identity, strict modes): none of those checks is moved or changed.
