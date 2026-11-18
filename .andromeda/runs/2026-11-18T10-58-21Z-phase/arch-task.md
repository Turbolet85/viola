@task_prose part=chunk
Builds the product write of the CLI's per-project trust record at a wrapped start: at viola run and viola revive (their one shared start), after the version gate and before the spawn, on a verified CLI, when no record already trusts the child's folder read the CLI's way.
viola writes only projects[<key>].hasTrustDialogAccepted, the key the CLI's own, into the one Claude config file the started child reads, under the CLI's lock when free, by atomic replace, read back, at most 3 attempts.
A refused file form or a lost record writes nothing more and leaves the trust dialog to the human, viola typing nothing; one closed word per start reaches the run log, nothing of the file anywhere.
The existing trust writer built for verify's trust-record probe is reused, not duplicated; every refusal and skip gets a case proving nothing was written, an e2e case on the fake agent can fail when the write is gone.
The live proof is S1 (viola run in a fresh untrusted folder under the rig's private config) with control S2 (plain claude in the same folder shape), on the relied trust-record ledger row read pass on 2.1.325; claims matrix v2-33.
@task_file n=1 path=crates/viola-agent-claude/src/trust.rs mark=edits
what: the Claude trust record's shape and the CLI's key rule (git root inside a repository, else the cwd), Claude-specific shapes in the pure agent crate — edits: the key rule and the CLI's trust reading (exact key, then the parent walk)
@task_file n=2 path=src/trust_record.rs mark=edits
what: the trust writer — the file write of the Claude config's trust record under the CLI's lock, atomic replace keeping the file's mode, its refusals and its closed trust writer code words — edits: the refusals, the read-back and the retry of at most 3 attempts, the write for an absent entry
@task_item n=1 part=needs
The shared start path of run and revive: the trust write sits after the version gate and before the spawn, the child still spawned last in the start order; revive reaches it through the same start path.
@task_item n=2 part=needs
The version gate's strict stamps read decides "verified": a CLI version is verified only when every relied row passes (now 23 rows, trust-record among them), malformed or untrusted stamps read unverified, and an unverified CLI degrades to transport-only and writes no trust record.
@task_item n=3 part=needs
The key and the trust reading follow the CLI: the key the CLI itself writes on an accepted dialog (git root inside a repository, else the cwd), trust read by the exact key then the parent walk (to the git root inside a repository, to / outside one); no write for a folder already trusted, and none for the home folder.
@task_item n=4 part=needs
Which file is written: <CLAUDE_CONFIG_DIR>/.claude.json when viola's home keeps that variable for the child through config.json claude_env_keep, else the user's ~/.claude.json; viola reads that one environment value for that one purpose, a carve-out from the rules that the environment is no configuration channel and that inherited CLAUDE names are stripped from the child by name only.
@task_item n=5 part=needs
The bound of the write: only projects[<key>].hasTrustDialogAccepted for one folder; no other key changed, no entry removed, the file never created; the file replaced atomically with its mode kept, crash-safe, the CLI's own lock taken only when free.
@task_item n=6 part=needs
The refusals: the file absent, unreadable, a link, not a regular file, not the JSON expected (not an object, projects not an object, the entry not an object) or oversize past MAX_FRAME — each writes nothing; the external config is read bounded and parsed tolerantly.
@task_item n=7 part=needs
The race: the record read back before the spawn, at most 3 attempts; a record still lost writes nothing more and the trust dialog stays the human's; viola types into no trust dialog; the residual of a concurrent write by another session lost in the window is accepted and named.
@task_item n=8 part=needs
The trust writer code words are the closed set the run log carries: one closed word per start in the run role's process log, codes only, nothing of the file's content or path in any log, error or output; error messages interpolate nothing.
@task_item n=9 part=needs
One writer: the product write reuses the existing trust writer rather than adding a second; the agent crate stays free of file I/O, holding only the shape, the key rule and the trust reading, the file work in the root crate.
@task_item n=10 part=needs
Each refusal and each skip (unverified CLI, a folder the exact key or a parent already trusts, the home folder, a lost race) gets a case proving nothing was written: the file's bytes unchanged, no temp sibling left, no lock left.
@task_item n=11 part=needs
The fake agent's trust decision is read from the Claude config file, so an e2e case of the write can fail when viola's write is gone; a change to its screens or its trust reading puts cli_verify in the gate block.
@task_item n=12 part=needs
Controls of the guards: each control's --timeout named in the plan and fired with that flag, on an entry whose build reaches every binary its case needs; ids within 16 characters; holds means those cases ran and failed with no TIMEOUT line; each control says what the subject does with the guard gone, measured or marked unmeasured.
@task_item n=13 part=needs
A test that waits takes a bound under the 20 s lint and the shared root test wait bound; a test-support helper that spawns an instrumented binary under a cleared environment runs it in a scratch directory; a gate entry naming an installed tool's path is checked against the host.
@task_item n=14 part=needs
The live proof runs the real claude CLI locally only, never in CI, through the live rig under the rig's private config with one other project's entry as sentinel: S1 red when the first screen reads trust, the sentinel differs by a byte or the written record is not exactly the one key; S2 red when the first screen does not read trust; both red on a missing teardown word.
@task_item n=15 part=needs
The trust-record ledger row is the relied row this write lands on, read live on 2.1.325; the write happens only where that row is in the verified set of the home's stamps.
@task_item n=16 part=needs
Writing the CLI's own state file is a boundary widening: rows stating viola writes nothing into ~/.claude.json are amended by this chunk's wrap, listed as expected amendments.
@task_standing n=1
convention: each new test runs in its own isolated viola home, its homes removed by owner record and its evidence surviving a killed test
home: established-decisions · occupied-resources
@task_standing n=2
convention: each new test runs against the fake agent on ubuntu CI, never the real claude CLI
home: ci-cd-approach
@task_standing n=3
convention: an OS-gated test names its reason, and no test gate names another OS
home: cross-cutting-patterns
@task_standing n=4
convention: each new log line goes through obs_event! as the only emitter, with no print macros or tracing level macros
home: build-system
@task_standing n=5
convention: new code passes formatting and clippy with warnings denied, and each new module is reachable, no orphan module
home: build-system
@task_standing n=6
convention: end-to-end runs keep zero panic lines and snapshots stay in check mode
home: project-directory-structure
@task_standing n=7
convention: new code is held to the coverage floors of the harness gate
home: ci-cd-approach
@task_item n=1 part=not_needed
MCP server, MCP tool and MCP result rows: the chunk adds no MCP tool and changes no MCP output.
@task_item n=2 part=not_needed
Send, paste, confirmation, wheel and wait rows: the chunk changes nothing of how viola types or confirms a send or holds the wheel.
@task_item n=3 part=not_needed
Hook tier, hook contract and dialog-hold rows: the trust dialog is not answered through a hook, and no hook changes.
@task_item n=4 part=not_needed
Board, list payload and agents-listing rows: the chunk adds nothing to what list shows.
@task_item n=5 part=not_needed
Link, unlink, pause and release verb rows and their exit codes: no such verb changes.
@task_item n=6 part=not_needed
Hyprland seam and window-feature rows: no compositor work.
@task_item n=7 part=not_needed
Plugin install, marketplace and plugin precedence rows: the plugin is untouched.
@task_item n=8 part=not_needed
Release tooling, dist, publishing and licence rows: no release work.
@task_item n=9 part=not_needed
Fuzz workspace rows and mutation testing at the epoch boundary: the scope runs no mutation run and adds no fuzz target.
@task_item n=10 part=not_needed
The browser-tools prompt read as a modal, the live home's re-stamp, the recorded 2.1.325 fixture set and scrubbing of recorded fixtures, any reading on 2.1.326 or 2.1.327, and a re-pin of the pinned product: all named out of this chunk.
@task_item n=11 part=not_needed
Snapshot, event-line and normalised event kind rows: the start writes no new event kind and no snapshot field.
