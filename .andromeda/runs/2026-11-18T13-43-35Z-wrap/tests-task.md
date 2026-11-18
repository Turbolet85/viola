@task_prose part=session
viola run and viola revive now write the CLI's trust record for an untrusted session folder at the start, after the version gate and only on a verified CLI, so no trust dialog stands and no key is typed (matrix v2-33 implemented).
The write is flag only: it sets hasTrustDialogAccepted in a present entry and keeps every other key, or inserts a one-key entry; nothing is written for a trusted key or parent, the user home folder, an unverified CLI, or any refusal form, and the word never refuses the start.
A new obs event trust-record (INFO, one per start, detail one of a new closed set of 16 start words, no path, key or value) moves the closed event set from 15 to 16 and adds one role line to every viola run start.
Retired: the rule that run and revive write no trust record, and the fake agent's oracle that inside a repository only the root's record counts; the fake now walks from its cwd to the git root as the CLI does.
Every test-started viola run and revive, and the harness's supervised wrappers, now run under a test-owned user home so no test touches the runner's real Claude config; the live rig gained the S1 and S2 trust-start arms, proven on claude 2.1.325 only.
No CI step, dependency, IPC method, endpoint, env var name, exit code, hint or --json code changed.
@task_file n=1 path=crates/viola-agent-claude/src/trust.rs
what: edited - the Claude-shape trust module, no file I/O: the CLI's trust walk (exact key then each parent up to the git root), the flag-only setter, the exact read-back and the closed set of 16 start words
@task_file n=2 path=src/trust_record.rs
what: edited - the guarded trust-record writer, the one writer under the Claude config: now also the start's trust step (home-folder rule, walk, flag-only write, three read-backs) and the shared lookup of the Claude config file
@task_file n=3 path=src/cmd/run.rs
what: edited - the viola run and viola revive start sequence: a trust step after the version gate and before the endpoint bind, on a verified CLI only, logging one trust-record line
@task_file n=4 path=src/cmd/verify.rs
what: edited - viola verify: its Claude config file lookup moved out to the shared trust-record module
@task_file n=5 path=crates/viola-core/src/obs.rs
what: edited - the closed obs event set and the log-format event literals: trust-record added, 15 to 16
@task_file n=6 path=schemas/diag-line.v1.json
what: edited - the diagnostics line schema: trust-record in the event enum, its message const and its required detail enum of the 16 start words
@task_file n=7 path=schemas/diag-detail.v1.json
what: edited - the diagnostics detail schema: trust-record in the event enum
@task_file n=8 path=src/bin/viola-fake-agent.rs
what: edited - the fake agent: its trust reading now walks from its cwd to the git root, its record-key rule removed
@task_file n=9 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: edited - the fake agent's option and trust-config unit cases: the walk test renamed to record_trusts_walks_from_the_cwd_to_the_git_root
@task_file n=10 path=tests/support/home.rs
what: edited - the shared test home helpers: an unmade user home, a wrapper boot of an untrusted folder as that user, and a revive as that user
@task_file n=11 path=tests/support/cli.rs
what: edited - the shared CLI test runner: every test run and revive given an unmade test-owned user home
@task_file n=12 path=tests/run_cli.rs
what: edited - the viola run integration suite: eight trust-start cases and seven existing log-counting oracles moved by the new start line
@task_file n=13 path=tests/cli_revive.rs
what: edited - the viola revive integration suite: the revive writes the record for its recorded cwd
@task_file n=14 path=tests/tui_passthrough.rs
what: edited - the passthrough suite through an outer PTY: a trust-writing start adds no viola byte to the terminal
@task_file n=15 path=tests/contract_diag_schema.rs
what: edited - the diagnostics schema conformance check: event enum equals the log-format literals, lines from real runs validate with the new start line
@task_file n=16 path=tests/contract_bounded_readers.rs
what: edited - the bounded readers check: each named bounded read still stands after the fake agent's trust reader moved
@task_file n=17 path=tests/chaos_snapshot_replay.rs
what: edited - the chaos snapshot replay suite: its starts given a test-owned user home
@task_file n=18 path=tests/cli_controls_not_disableable.rs
what: edited - the controls-not-disableable suite: its starts given a test-owned user home
@task_file n=19 path=tests/cli_fake_agent.rs
what: edited - the fake agent CLI suite: its starts given a test-owned user home
@task_file n=20 path=tests/cli_version_gate.rs
what: edited - the unverified CLI version gate suite: its starts given a test-owned user home
@task_file n=21 path=tests/cross_exit_causes.rs
what: edited - the cross-surface exit causes suite: its starts given a test-owned user home
@task_file n=22 path=tests/cross_killed_test.rs
what: edited - the killed-test cross suite: its starts given a test-owned user home
@task_file n=23 path=tests/channel_error_bodies.rs
what: edited - the channel error bodies suite: its starts given a test-owned user home
@task_file n=24 path=crates/viola-e2e/src/harness/boot.rs
what: edited - the harness boot command: a session user home for a stamped session, seeded with an empty config
@task_file n=25 path=crates/viola-e2e/src/harness/supervise.rs
what: edited - the harness supervisor: supervised wrappers started under the session user home
@task_file n=26 path=crates/viola-e2e/src/rig/mod.rs
what: edited - the standing live rig: the trust-start arms S1 and S2, the 21 floor rows, the first-screen and floor words
@task_file n=27 path=crates/viola-e2e/src/rig/isolate.rs
what: edited - the live rig's scratch isolation: a sentinel entry seeded in the private user config, a fresh untrusted cwd, the entry and run-trust words
@task_file n=28 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: edited - the live rig command line: live S1 and S2 and the rehearse ts and tc arms
@task_file n=29 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited - the live rig rehearsal suite over the fake agent: the ts and tc rehearsals
@task_item n=1 part=touches
The fake agent's trust-config key rule (the fake agent command line --trust-config) is stale: the fake no longer reads only the root's record inside a repository; it walks the exact key of its cwd, then each parent up to the git root, as the CLI does.
@task_item n=2 part=touches
The fake agent's trust record and trust dialog screen rows (--trust-record-blank, the trust dialog screen) and the rule that test wrapper boots run trusted with screens: a viola run start on a verified CLI now writes the record itself, so an untrusted folder reaches the fake without the trust screen.
@task_item n=3 part=touches
The rule that every test verify runs under a test-owned user home and that no verify start runs outside a test home now has a sibling: every test-started viola run and revive also runs under a test-owned user home.
@task_item n=4 part=touches
Tests never read the host's claude sessions, per-test home and every test creates its own home: the start now locks, reads and writes a user config, so a test start under the runner's real HOME would touch the operator's real config; the test homes were extended to cover it.
@task_item n=5 part=touches
The run start events order and viola run checks before the child: a trust step now sits after the version gate and before the endpoint bind, and logs one trust-record line per start.
@task_item n=6 part=touches
The closed set of log events (process log line event, log-format line format, closed enums change only by decision): trust-record added, the set 15 to 16, with its detail one of 16 start words and required by the diagnostics line schema.
@task_item n=7 part=touches
Log event corr rows list one corr rule per event; trust-record is a new event with no corr row yet.
@task_item n=8 part=touches
Diagnostics schema conformance and the codes-only role logs rule: the trust-record line carries a code word detail only, no path, key or value, and must validate against both schemas.
@task_item n=9 part=touches
A start's lines read from its own offset and every oracle that counts start log lines: one more role line per viola run start moved seven run cases and the real-runs schema case.
@task_item n=10 part=touches
The unverified CLI version gate: on an unverified CLI the start reads no Claude config file and writes no record.
@task_item n=11 part=touches
The standing live rig and the 21 floor rows: new live S1 and S2 arms and ts and tc rehearsals; the live claim is for 2.1.325 only.
@task_item n=12 part=touches
Real claude CLI local-only and fake-agent run is not proof of real-CLI behaviour: the trust write was proven on the fake and separately live on 2.1.325, nothing claimed for later versions.
@task_item n=13 part=touches
All test homes under target/e2e-home: the trust cases' folders sit under /tmp outside any repository, an exception, since under the workspace they would key at its git root.
@task_item n=14 part=touches
Harness boot makes the home, stamps it and starts wrappers, and the supervisor is an ordinary child: the supervised wrappers now get a session user home seeded with an empty config.
@task_item n=15 part=touches
Test-started viola drops inherited env and the R8 CLAUDE environment strip: CLAUDE_CONFIG_DIR's value is now read by the run and revive start too when it is kept, through the same config lookup as verify.
@task_item n=16 part=touches
No unbounded JSON depth and bounded readers: the start's config read goes through the guarded writer with the MAX_FRAME size bound and serde default depth.
@task_item n=17 part=touches
The viola run start refusal rows and run and revive exit 1 causes: the start's trust word never refuses the start, no exit code, hint or --json code moves.
@task_fact n=1 slug=test-data-bootstrap
fact: Every test-started viola run and viola revive runs under a test-owned user home, never the runner's real HOME, because the start writes into the user's Claude config.
@task_fact n=2 slug=test-data-bootstrap
fact: The harness's supervised wrappers of a stamped session run under a session user home seeded with an empty config.
@task_fact n=3 slug=7-test-data-fixtures
fact: Trust-start test folders sit outside any repository (under /tmp), an exception to test homes under target/e2e-home, because a folder inside the workspace keys at its git root.
@task_fact n=4 slug=7-test-data-fixtures
fact: The fake agent reads trust by the CLI's walk: the cwd's exact key, then each parent up to the git root; inside a repository the cwd's own record counts.
@task_fact n=5 slug=log-format
fact: trust-record is an INFO event logged once per viola run or revive start on a verified CLI, its detail one of 16 start words and required, carrying no path, key or value.
@task_fact n=6 slug=6-e2e-test-strategy
fact: The live rig proves the trust start by two arms on one ledger: S1 verifies then runs viola in a fresh untrusted folder and reads the written record and the first screen; S2 starts a plain claude in the same folder shape, reads the trust form and is killed with no key.
@task_item n=1 part=untouched
MCP server, tools and replies: no MCP tool, schema or reply moved.
@task_item n=2 part=untouched
send, answer, wait, release, last and link verbs and their confirmation, wheel and refusal rules.
@task_item n=3 part=untouched
Dialog hooks, hook exec entry points, hook deadlines and decision bodies.
@task_item n=4 part=untouched
CI workflows, artifacts, supply-chain gates, pinned toolchain and Actions: no CI step and no dependency changed.
@task_item n=5 part=untouched
Coverage floors and thresholds, mutation testing, fuzzing and performance budgets: no threshold or budget moved.
@task_item n=6 part=untouched
list board, list --json, heartbeat staleness and instance liveness.
@task_item n=7 part=untouched
pre-push, gate, secret-scan and schema-check command contracts and their exit codes.
@task_item n=8 part=untouched
Absences (browser suite, database, outbound HTTP, Windows and macOS, deploy stage): none changes.
