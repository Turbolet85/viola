@task_prose part=chunk
Three pure moves that change no behaviour, ranked sixth by the Epoch 5 code audit.
First, viola up's fleet re-open loop (`upped`) is split into a plan (refusal, order, skip) and an act (spawn, wait, line). This is the only place the chunk may add a function.
Second, the switch's respawn (restart code) moves out of the PTY pump into its own module of the viola-pty crate, and the pump's inline test module moves out with it.
Third, run::start's window step (`open_window`) and its trust step move out of the shared start of run, revive and adopt, to sit beside the window code and the trust record, and that file's test module moves out too.
Each moved file and each file left behind is measured again: at most 800 tokei code lines per file, and no function over cognitive 15 in up and its parts.
@task_file n=1 path=src/cmd/up.rs mark=edits
what: viola up, the fleet re-open command — edits: the function `upped` (the re-open loop), split into its plan and its act
@task_file n=2 path=crates/viola-pty/src/pump.rs mark=edits
what: the wrapper's PTY pump and its main loop, including the switch's respawn of the claude child — edits: the respawn arm of `Pumping::run`, moved out; the inline test module, moved out
@task_file n=3 path=crates/viola-pty/src/lib.rs mark=edits
what: the viola-pty crate's module tree — edits: the mod list, which gains the restart module
@task_file n=4 path=src/cmd/run.rs mark=edits
what: the shared start of run, revive and adopt (and through them open and up) — edits: `run::start`'s window step call (`open_window`) and its trust step, moved out; the test module, moved out
@task_file n=5 path=src/run/window.rs mark=edits
what: the wrapper's window step, which moves its own terminal's window; the window and trust steps moved out of the start land beside it
@task_item n=1 part=needs
Restructure invariant. These units move and keep what they do. viola up's re-open loop keeps its refusal, order and skip, and its spawn, wait and result line. The switch's respawn still ends and restarts only the wrapper's own child, with the same child environment (parent identity and CLAUDE credential variables stripped, the pinned folder first on PATH), and the screen feed stays inside its panic catch. The start's window step still moves only its own terminal's window, and its trust step still writes only the trust record flag and skips the home folder and trusted folders. The start's check order stays: socket directory, strict modes and home tree all come before any bind, snapshot read, log open or child. The existing whole-layer tests hold all of this with no test body edited, and each moved test module has the same case count before and after the move.
@task_item n=2 part=needs
The measure the moves are read by. A file's size is its tokei code lines, its inline test module included, and must be at most 800 for each moved file, each new file and each file left behind. A function's complexity is its rust-code-analysis cognitive sum, and must be 15 or under for every function of viola up and of any file its parts land in. Pump::run's figure is measured after the respawn leaves it.
@task_item n=3 part=needs
Splitting `upped` into a plan and an act must keep viola up's refusals ahead of its log and of any spawn. The plan decides refusal, order and skip before the act spawns, waits or writes a line.
@task_item n=4 part=needs
The restart module is new code in a sync crate. It must use no tokio and add no dependency or crate feature, so the root graph's deny gate and the release build's test-feature refusal read the same as before.
@task_item n=5 part=needs
A moved unit stays in its own crate. The restart code stays in viola-pty, and the window and trust steps stay in the root crate beside the window code and the trust record. A moved unit exposes no more than its callers need, and the move adds no new public surface, seam or environment read.
@task_item n=6 part=needs
A test reaches a moved unit only through a changed path, module or name, and the plan names each such change. No test is deleted, merged or weakened to fit a size bound. Every moved function keeps its tests with it, so the root crate's diff-mutant coverage survives the moved sites. Each moved test module is split into at least two files of at most 800 code lines.
@task_item n=7 part=needs
Any path, module name or text a move changes is updated wherever a census, lint list or mod tree reads it. This covers the bounded-readers census (every product reader bounded or named), the census of named product error writers, the zero-panics gate and its single exemption, and any contract census that names a moved file. A master row naming a moved path is an expected wrap amendment.
@task_item n=8 part=needs
The chunk's proof runs without anything live: no live start, window, compositor dispatch or real claude CLI, and no live arm of a harness script. Tests use their own home and read nothing of the operator's real Claude home or viola home. Because the boot path changes, the harness smoke trio is held as operator entries.
@task_item n=9 part=needs
A plan step that asserts what existing code does cites the symbol and the line it read.
@task_standing n=1
convention: Every new or moved test file uses its own temporary home and never the real viola home or Claude home.
home: secret-management
@task_standing n=2
convention: Every test seam a moved test file uses stays compiled only under the fake-agent test feature and out of the release build.
home: security-anti-patterns · input-validation
@task_standing n=3
convention: Test-side cleanup in a moved test file removes only the directories that test created.
home: security-anti-patterns-never-do-these
@task_standing n=4
convention: No test home or local state from a new test file is committed to git.
home: bootstrap-phases
@task_item n=1 part=not_needed
The MCP tool contracts, the MCP stdin cap and the MCP error shapes. The chunk changes no MCP server code or tool.
@task_item n=2 part=not_needed
The channel frame, params and error-code contracts, and the wrapper channel methods. The chunk adds and changes no method, frame or reply.
@task_item n=3 part=not_needed
The hook decisions and contracts (hook stdin, hook-decision detail, failing open, exit zero). The hook command is not touched.
@task_item n=4 part=not_needed
viola verify's decisions and contracts: probes, stamps, fixture refusal codes and verify refusal codes. The scope keeps verify out of the chunk.
@task_item n=5 part=not_needed
Decisions on the harness live-round scripts (the R5 adopt script, the R7 placing script) and the live rig arms. The chunk runs or edits none of the scripts.
@task_item n=6 part=not_needed
The board and viola list decisions: home read rule, re-parsed launch pair, lineage, window origin and terminal colour facts. viola list is not touched.
@task_item n=7 part=not_needed
Paste text and send-text validation: character sets, control-character refusal, empty text, bracketed paste, and the send refusal contracts. The chunk moves no input check.
@task_item n=8 part=not_needed
The revive --json code contracts, the revive argument contracts and the revive list document. The chunk changes no revive output.
@task_item n=9 part=not_needed
The config.json key contracts and the config parsing decisions. The chunk reads and adds no configuration.
@task_item n=10 part=not_needed
Workflow security: actions pinned by SHA, least permissions, zizmor, checkout credentials and CI upload scanning. The chunk edits no workflow or CI job.
@task_item n=11 part=not_needed
The absences: hosting, network listeners, databases, accounts, secrets stores and the release stage. A pure move adds none of these.
@task_item n=12 part=not_needed
The pinned binary and plugin placement decisions: hashing, no-clobber placement, plugin files rewritten each start. The chunk moves none of that code.
