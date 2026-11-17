@task_prose part=session
The session added one standing, committed, test-only live rig, the binary viola-live-rig in the viola-e2e test crate. It drives a wrapped claude child on its own PTY, reads the screen through a vt100 model into closed words, passes the trust dialog with a read-first key grant, and keeps its own isolation, credentials handling and process census.
Its rehearsals run in CI against the fake agent as a new integration test binary, rig_rehearsal. Its live starts against the real claude 2.1.325 run only by hand under a start cap of 2 plus 1 spare, and the live verb refuses to start under CI.
The fake agent under --screens with an untrusted cwd now draws claude 2.1.325's unnumbered trust form over the recorded modal, with the selector first on No, exit. It also draws a box rule and a prompt row on each typed key, and DEL erases the prompt's last character, receipted as a key.
Live it measured that claude 2.1.325 keeps a leading run of spaces at submit, removes a trailing run of spaces and TAB, and keeps inner runs and an inner LF. The user's claude.json read unchanged under a private CLAUDE_CONFIG_DIR.
The product binary is unchanged: no product diagnostics line, event, span, exit code or schema moved, and nothing was retired.
@task_file n=1 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig's binary, the test-only harness tool with the rehearse, live and census verbs that starts a wrapped claude child for a live reading
@task_file n=2 path=crates/viola-e2e/src/rig/mod.rs
what: the live rig's start sequence: the CI refusal, the start cap, the CLI version check, the token-window check, the child spawn and the closed-word summary of a start
@task_file n=3 path=crates/viola-e2e/src/rig/screen.rs
what: the live rig's screen model, a vt100 feed contained against panics that reads the trust dialog, the input box and the imports modal into closed words
@task_file n=4 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's isolation: the private scratch config, the credentials copy and its removal, the kept child environment names and the user config before and after check
@task_file n=5 path=crates/viola-e2e/src/rig/procs.rs
what: the live rig's process hygiene: the /proc census and sweep by pid and start time, the run marker, the watchdog and the stop handlers
@task_file n=6 path=crates/viola-e2e/src/rig/keys.rs
what: the live rig's key grant, the one typing site on the child's PTY, which refuses any key outside its grant
@task_file n=7 path=crates/viola-e2e/src/rig/ledger.rs
what: the live rig's start ledger and kept screen frames, its ndjson start and summary lines, test-only
@task_file n=8 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the rig rehearsal integration tests run in CI against the fake agent: the trust pass, the key grant, no credentials copy and no process left on any exit path, no credential or host path in any record, the CI refusal of a live start
@task_file n=9 path=crates/viola-e2e/Cargo.toml
what: the viola-e2e test crate's manifest, which declares the new rig binary and adds the vt100 dependency
@task_file n=10 path=crates/viola-e2e/src/lib.rs
what: the viola-e2e test crate's root, which declares the rig module
@task_file n=11 path=src/bin/viola-fake-agent.rs
what: the fake agent, the test-only stand-in for claude that CI verify and the integration tests boot; it draws recorded screens and fires hooks
@task_file n=12 path=src/bin/viola-fake-agent/input.rs
what: the fake agent's typed-input reader, which now holds the prompt text and erases it on DEL
@task_file n=13 path=Cargo.lock
what: the dependency lockfile, with one line for vt100 in the viola-e2e test crate
@task_item n=1 part=touches
Rows saying CI verify runs against the fake agent only and real-CLI verify never runs in CI. The new rig runs a real claude by hand only, its live verb refuses under CI, and its CI rehearsals boot only the fake agent. The rows stay true, but a test-only real-CLI harness now sits beside them.
@task_item n=2 part=touches
Rows holding that the fake agent and recorded fixtures are not a telemetry source. Under --screens with an untrusted cwd the fake agent now draws 2.1.325's trust form over the recorded modal and receipts DEL as a key, and viola verify --record over the fake still keeps the recorded trust row. No fixture, option or env seam was added.
@task_item n=3 part=touches
Rows that ban telemetry from screen content and never log the PTY byte stream. The rig reads the child's screen into closed words and keeps screen frames as files, test-only, under a gitignored target directory. That is a test-side screen reading beside rows that rule only over the product.
@task_item n=4 part=touches
Rows that never log typed keys or sent text. The rig's summary names keys by count only (down 1, enter 1), and its R2 texts compare to closed words (equal, trailing-trimmed). Any row whose rule reaches the harness or test records is touched.
@task_item n=5 part=touches
Rows that contain vt100 panics. The rig's screen model is a new vt100 feed, contained by catch_unwind, that reads as poisoned on a panic. The containment rule now has a second, test-only site.
@task_item n=6 part=touches
Rows on obs dependencies: they pass cargo deny, build no C, and leave crate log targets off. vt100 was added to the viola-e2e test crate at the existing workspace pin =0.16.2, already in the graph through the claude agent crate, and cargo deny reads green.
@task_item n=7 part=touches
Rows that never log the CLAUDE environment and its strip, and the claude child's env_kept field. The rig sets CLAUDE_CONFIG_DIR on its child to a private scratch config and sets a new run-marker name, AGENT_RUN_RIG, on the child only. R2's instance config keeps CLAUDE_CONFIG_DIR through claude_env_keep, so env_kept lists it in that run.
@task_item n=8 part=touches
Rows that never log secrets or signing credentials and keep home-level lines path-free. The rig reads a credentials file, parses only its expiry, places the bytes 0600 in scratch and removes them on every exit path. A rehearsal asserts that no record holds a credential or host path.
@task_item n=9 part=touches
Rows that never upload local run outputs and gate CI artifacts on the secret scan. The rig's kept frames and ledger stay under the gitignored target tree, and a kept live trust frame holds the scratch cwd path, which the CLI draws.
@task_item n=10 part=touches
Rows that put integration-test homes under one scanned root. The rig's scratch base is a private directory under /tmp per user, outside the e2e home, and the rehearsals place their scratch per case.
@task_item n=11 part=touches
Rows saying wrapped claude internals and transcripts are no contract. The rig reads claude 2.1.325's trust dialog form by screen literals: unnumbered, with the selector first on No, exit and the trust row below it. These are live readings of CLI internals, held only by the test crate.
@task_item n=12 part=touches
Rows on no print macros and no telemetry on stdout. The rig prints its closed-word summary as key: word lines on stdout. It sits in a test-only crate outside the product code paths, an exception any row whose scope reaches harness binaries would need.
@task_item n=13 part=touches
Rows on no sleep-based log waits. The rig and its rehearsal wait with bounded settle and deadline windows: settle 6 s, deadline 15 s, and a rehearsal WITHIN of 18 s, set under the contract lint's 20 s floor and the 30 s viola-e2e kill line.
@task_item n=14 part=touches
Any obs-plan row that words the send trim. R2 measured claude 2.1.325 keeping a leading run of spaces, removing trailing spaces and TAB, and keeping inner runs and LF. A rule that reads trailing-only stays true, and a cell that calls the trim set unmeasured or a leading trim open is now stale.
@task_item n=15 part=touches
The Linux-only OS target and the OS-gated test census. The rig's process census reads /proc by pid and start time, and a new integration test binary, rig_rehearsal, joins the test census.
@task_fact n=1 slug=8-pii-scrubbing-compliance
fact: The live rig's records (its stdout summary, its start ledger lines and its frame index) carry closed words only, never a path, credential or config value. The user's claude.json is compared by a hash prefix and mtime alone, never kept.
@task_fact n=2 slug=8-pii-scrubbing-compliance
fact: The live rig's kept screen frames stay local under the gitignored target tree and are never uploaded and never a telemetry source. A kept live frame may hold the scratch cwd path as the CLI draws it.
@task_fact n=3 slug=9-ci-integration
fact: The live rig's rehearsals run in CI against the fake agent only. Its live arm against the real CLI refuses to start under CI, so no real-CLI start ever runs in CI.
@task_item n=1 part=untouched
The product diagnostics line schema, its fields, event enum, detail codes and corr rules: the product binary is unchanged and no product line was added or moved.
@task_item n=2 part=untouched
Exit-code and exit-cause contracts of viola run, send, wait, last, revive, link, verify and mcp: no product exit or refusal word moved.
@task_item n=3 part=untouched
Span and trace coverage, span names, span attributes and span kinds: no product span was declared or changed.
@task_item n=4 part=untouched
Metric coverage, the derived counters, gauges and histograms, and the SLO and perf budgets and their thresholds: no budget, floor or gate threshold of obs-plan moved.
@task_item n=5 part=untouched
Hook, channel and MCP stdout, stderr and log-line contracts, and the heartbeat and liveness rows: none of those processes changed.
@task_item n=6 part=untouched
Error capture and the panic outcome by role, the panic hook and the detail-file line: no product role or catch site changed.
@task_item n=7 part=untouched
Logging stack, log levels, config.json keys, service identity and home resolution: no logger, sink or config key was touched.
@task_item n=8 part=untouched
Absences of a telemetry exporter, network egress, an error reporter, a database, gRPC and a hosted deployment: none was added.
