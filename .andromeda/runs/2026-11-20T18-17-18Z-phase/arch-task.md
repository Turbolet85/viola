@task_prose part=chunk
Turns two claude CLI behaviours the product already relies on with no capability-ledger row into relied ledger rows in viola-agent-claude, each with its own viola verify probe.
Row one: the harness prompt prefixes, the compiled raw-prefix set that files a prompt as a harness turn that never moves the wheel; its probe is a second session sending into a verify child and reading the raw start of that prompt.
Row two: the R8 identity floor, the eleven measured CLAUDE parent-identity environment names the child is always stripped of; its probe captures env names, never values.
The new rows are counted in a viola verify stamp from a live run in the rig's scratch home only; the fake agent replays both probes in CI.
Re-reads what the tree can show of the standing facts: the relied-row count, the stamped fixture set(s), the default-version constants and the rig's kept stamp.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the per-behaviour CLI capability ledger, its relied rows and the relied-id set a stamp counts — edits: the relied-row list, the stamp's relied-id set
@task_file n=2 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the capability ledger's tests over its rows, runs, checks and stamps
@task_file n=3 path=crates/viola-agent-claude/src/hook.rs
what: hook payload shapes and the compiled harness-prompt raw prefixes by which a prompt is classed as a harness turn
@task_file n=4 path=crates/viola-agent-claude/src/lib.rs
what: the agent crate's root, holding the identity floor of eleven CLAUDE names stripped from the child's environment
@task_file n=5 path=fixtures/claude/2.1.328
what: the recorded, scrubbed fixture set of one claude CLI version written by viola verify --record and replayed by the fake agent
@task_item n=1 part=needs
The capability ledger gains two relied rows, the harness prompt prefixes and the R8 identity floor, each a dependence on undocumented claude behaviour carried as a ledger row with a viola verify probe; the ledger stays the single gate for CLI behaviour.
@task_item n=2 part=needs
The harness-prefix row rests on the rule that harness prompts are classified by raw prefix before any unwrapping or un-escaping, and that a harness prompt is no human take and never moves the wheel; the prefix set stays compiled literals.
@task_item n=3 part=needs
The identity-floor row rests on the R8 parent-identity floor: eleven named CLAUDE identity variables are always stripped from the child's environment whatever claude_env_keep keeps, and the strip reads names only.
@task_item n=4 part=needs
The harness-prefix probe adds a second session sending into a verify child; it must stay inside verify's bounded child set and runs, the Run order verify keeps, and verify's rule that it never types into a CLI dialog; how the second session reaches the child is open to P3.
@task_item n=5 part=needs
The identity-floor probe captures environment variable names, never values, through the hidden hook --capture arm or a probe of its kind; capture arms stay inert outside verify, their closed ids and payload-file shape bind any new capture.
@task_item n=6 part=needs
No environment value may reach any file, fixture, stamp, log line or screen read; the chunk owes a failing test proving a captured value is refused, and the process-start line of the claude child keeps reporting kept and stripped names only.
@task_item n=7 part=needs
viola verify --record and its scrub: fixtures are recorded only from a clean verify, scrubbed and hygiene-walked, and an unclean fixture is refused with its closed refusal words; a captured name that itself carries private text must be handled by the scrub.
@task_item n=8 part=needs
The ledger stamps envelope gains the new rows: verify is the sole stamps writer, a merge replaces only its own version, and a version is verified only when every relied row counts, so adding relied rows makes an older, shorter stamp read unverified.
@task_item n=9 part=needs
What a now-shorter stamp means for a start: a start on a version whose stamp lacks the new rows reads the CLI unverified, with the partial gate and transport-only degrade, and an automatic verify of a new CLI version when keyed.
@task_item n=10 part=needs
viola verify's stdout step and summary lines and its exit code 1 on a failed relied row now cover the two new rows.
@task_item n=11 part=needs
The fake agent replays the new probes from the recorded fixture set in CI; no live claude runs in CI and real-CLI probes run only locally.
@task_item n=12 part=needs
The live rig's verify arm runs the live stamp in the rig's scratch home only; which version or versions are stamped is the founder's choice at P3's end.
@task_item n=13 part=needs
No live start and no read of the operator's real claude config in this phase; the live home's stamps and config.json and the pinned product stay untouched, owned by the re-pin step.
@task_item n=14 part=needs
viola-agent-claude does no file I/O and holds all Claude-specific shapes and literals; the new probes' file writes, capture and scratch dirs live outside it.
@task_item n=15 part=needs
Verify's probe and scratch folders are created and removed by drop guard, verify removes only its own dirs, writes and removes its own trust record, and keeps plan files in its run dir.
@task_item n=16 part=needs
Captured hook payloads are external payloads read tolerantly, unparseable values becoming unknown, and upstream text is untrusted content.
@task_item n=17 part=needs
Screen signatures stay compiled literals, screen row text is read only by verify, and screen fixtures keep only signature rows; no compiled screen literal is printed in a chat message.
@task_item n=18 part=needs
Verify's CLI transcripts stay an accepted residual; the residual rows and what leaves the process are swept for what the new probes add.
@task_item n=19 part=needs
Home-level process logs and role logs hold codes only, and error context chains reach only the detail file; any log line the probes add obeys that.
@task_item n=20 part=needs
The master-row sweep across all seven masters: every row the change reaches, covering verify's child set and runs, the capture arm, env reads and the env keep-list, the identity floor, the harness prefixes, the ledger's rows and the residual rows.
@task_item n=21 part=needs
Environment variables are no configuration channel: VIOLA_ names stay product plumbing, test-harness knobs carry AGENT_RUN_ and are never read by viola, and test seams are not configuration.
@task_standing n=1
convention: each new test runs in its own isolated viola home, cleaned by its owner process
home: established-decisions · occupied-resources
@task_standing n=2
convention: a test gated on the OS names its reason
home: cross-cutting-patterns
@task_standing n=3
convention: each new log line is emitted through obs_event!, with no print or tracing level macros
home: build-system · project-directory-structure
@task_standing n=4
convention: each new crate test keeps coverage floors and snapshot tests run in check mode only
home: ci-cd-approach · project-directory-structure
@task_standing n=5
convention: end-to-end runs keep zero panic lines and test-support code stays out of product builds
home: project-directory-structure · occupied-resources
@task_item n=1 part=not_needed
The Hyprland compositor seam and window features: the probes open no window.
@task_item n=2 part=not_needed
The revive verb, its refusals and its --json error codes: no revive path changes.
@task_item n=3 part=not_needed
The plugin install verb, the marketplace tree and the plugin's MCP manifest: the plugin's contents do not change.
@task_item n=4 part=not_needed
The board and list payload, agents listing and session liveness rows: nothing listed or shown on the board changes.
@task_item n=5 part=not_needed
Link and unlink recording, link events and the link set: the chunk records no link.
@task_item n=6 part=not_needed
Dialog hold, answer responses and pending-dialog rows: the probes answer no dialog.
@task_item n=7 part=not_needed
The fuzz workspace, its nightly toolchain and weekly fuzzing: no fuzz target is added.
@task_item n=8 part=not_needed
Release tooling, dist, crates.io publishing and the licence choice: nothing is released.
@task_item n=9 part=not_needed
The daemon, database, broker, graphical, mobile and network-port absences: nothing of those kinds is built.
@task_item n=10 part=not_needed
Human output colour environment rows: no human-coloured output changes.
