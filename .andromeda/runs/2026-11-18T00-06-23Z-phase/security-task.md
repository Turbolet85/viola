@task_prose part=chunk
Re-reads live the relied ledger row trust-record (built and proven on the fake agent, unverified on the real 2.1.325 CLI) before any product write relies on it, claiming no capability.
Builds the instruments that let one live start tell apart why the last start stopped short: verify keeps one closed first-screen word per interactive run in the stamp's measured block; the live rig's V arm prints the six words before teardown and runs on a git-initialised scratch root seeded with the trust record at the root's key.
Bounds the verify_window_ test class's kill and the 52 s verify-window case with the measured walls, and gives the --record verify a witness of where its record went.
Then spends ONE base live verify start (plus two spares only on a named cause) under the founder's cap, read against the floor predicate of the 21 rows the live home's 2.1.325 stamp passes; viola run and revive do not change.
@task_file n=1 path=src/cmd/verify/typed/runs.rs mark=edits
what: verify's interactive runs on the agent CLI and what each run reads from its screen through compiled signatures — edits: the closed first-screen word kept per interactive run
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the Claude adapter's ledger of relied rows and the verify stamp it writes, with the stamp's measured block — edits: the stamp's measured block, gaining the per-run first-screen words
@task_file n=3 path=crates/viola-e2e/src/rig/mod.rs mark=edits
what: the standing live-round rig that starts the real claude CLI under an isolated private config, its arms and its teardown — edits: the V arm (git-initialised scratch root, trust record seeded at the root's key, six words printed before teardown)
@task_file n=4 path=tests/cli_verify/runs.rs mark=edits
what: the cli_verify tests of verify's interactive runs on the fake agent, including the verify-window case with no screens and the --record verify cases — edits: the 52 s verify-window case, the --record witness, the first-screen word assertions
@task_file n=5 path=tests/support/verify.rs mark=edits
what: the shared test support that drives verify on the fake agent, its test home and its --record environment — edits: the --record witness of where the record went
@task_file n=6 path=.config/nextest.toml mark=edits
what: the test runner's per-class overrides, holding the verify_window_ class and its kill bound — edits: the verify_window_ kill
@task_item n=1 part=needs
Verify's stamp gains one first-screen word per interactive run, from a closed set (trust, imports, box, none; null for a run that did not start), read only against compiled signature literals; no screen text is kept in the stamp or any artefact.
@task_item n=2 part=needs
The closed facts that separate the candidate causes (whether the child exited before the read, the count of settled frames, the run's wall) are recorded as closed words and numbers only, with nothing of the founder's config.
@task_item n=3 part=needs
Verify stays a reader of signatures: it answers no trust or imports dialog, types no byte into a dialog, and pastes only its own ratified literals into the input box; its interactive children stay directly spawned.
@task_item n=4 part=needs
The trust-record row's outcome stays a closed code word (pass or fail) and the row is read, not taken as floor; local-command-echo likewise is read and not floor.
@task_item n=5 part=needs
The live rig's V arm git-initialises its own scratch root and seeds the trust record at that root's key in the rig's private config only, Runs B to E in that trusted repository and Run F in a fresh directory outside any repository; viola's own trust-record writer and the rule that viola writes nothing under the Claude home are untouched by the seed.
@task_item n=6 part=needs
The rig keys nothing into any dialog or box; its stop is its standing one, Ctrl-C into its own pty or a no-key group kill, named beside keys for each start.
@task_item n=7 part=needs
The rig's child stays isolated from the user's config: verify's interactive children receive the kept private CLAUDE_CONFIG_DIR, inherited credentials stay stripped and never serialised, and the rig's credentials copy stays transient and unrecorded.
@task_item n=8 part=needs
The --record witness shows where verify's record went without asserting on a shared real file the host's own sessions rewrite; the witness reads names and places only, no record content.
@task_item n=9 part=needs
The verify_window_ class's kill and the 52 s verify-window case are bounded by the measured walls (52.2 to 52.3 s against a 60 s kill), either by bounding the case or moving the kill; any test that waits takes a bound under the 20 s lint.
@task_item n=10 part=needs
The live start S0 prime is one viola verify on the 2.1.325 install by path under the rig's private config (seven claude starts: one print probe, six interactive), ledgered with date -u before it is made; a spare only after a named cause, a token-window refusal uncounted with no timer, a fourth start the founder's.
@task_item n=11 part=needs
The floor predicate: the chunk reads red when any of the 21 rows the live home's 2.1.325 stamp passes reads below pass in S0 prime's stamp, and red stops implement before the operator pass with the six words and closed facts written.
@task_item n=12 part=needs
The real claude CLI runs only locally: the live start is never a CI step, CI holds no secrets and no real claude, and the fake-agent work is green and landed before the first live start.
@task_item n=13 part=needs
Live session screens and verify's real-CLI transcripts stay local and transient; verify's probe directories and the rig's scratch root are removed by their own guards, and the session ends every process it starts.
@task_item n=14 part=needs
Controls for the new guards: no mutation run; each control fired with a named --timeout on an entry whose build reaches every binary its case needs, ids within 16 characters, holds meaning those cases ran and failed with no TIMEOUT line, each saying what the subject does when the guard is gone.
@task_item n=15 part=needs
A change to verify's runs puts cli_verify in the gate block, and a gate entry naming an installed tool's path is checked against the host before the card.
@task_item n=16 part=needs
Verify's strict-modes refusal and home-tree check before anything still precede the new stamp field: the stamps file is written only by verify, after the strict-modes check.
@task_standing n=1
convention: each new test uses its own test home, never the user's home or Claude config, and test homes stay out of git
home: secret-management
@task_standing n=2
convention: any new test-only seam lives only under the fake-agent feature and is compiled out of the release build
home: security-anti-patterns
@task_standing n=3
convention: test-side removal touches only directories the test created
home: security-anti-patterns-never-do-these
@task_standing n=4
convention: every new stamp field or log line holds closed values only, with no paths, screen text, identity values or upstream text
home: data-protection
@task_item n=1 part=not_needed
MCP server tools, MCP stdin line caps and the rmcp feature set: the chunk adds or changes nothing of the MCP surface.
@task_item n=2 part=not_needed
Wrapper channel methods, channel frames, channel JSON-RPC errors and faults, and link or unlink: nothing of viola run's wrapper or its channel changes.
@task_item n=3 part=not_needed
Socket directory checks, peer euid checks and server-verification refusals at the client seams: the chunk opens no channel and binds no socket.
@task_item n=4 part=not_needed
revive arguments, revive --json codes, the snapshot and its replay: revive is untouched.
@task_item n=5 part=not_needed
Send and answer refusal contracts, human-typing wheel events and the stdin classifier's non-editing list: no product send path changes.
@task_item n=6 part=not_needed
Release, signing, deploy, attestations and self-update: the chunk ships no release artefact.
@task_item n=7 part=not_needed
viola list's board scan, terminal colour facts and the Hyprland recorder: none is reached by verify's runs or the rig's V arm.
@task_item n=8 part=not_needed
Event log rotation, growth and events.ndjson link lines: the chunk writes no event log line.
@task_item n=9 part=not_needed
The absences of hosting, a public network listener, a database, webhooks, file upload and user accounts: none is touched.
@task_item n=10 part=not_needed
The product write that trusts a session folder, its e2e tests and re-stamping the live home: those belong to later entries, not this one.
