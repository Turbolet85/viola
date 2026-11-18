@task_prose part=session
viola run and the start of viola revive now write the claude CLI's trust record for an untrusted spawn folder, so no trust dialog stands at a wrapped start and no key is typed.
The write is flag only: it sets hasTrustDialogAccepted in the folder's entry and keeps every other key, or inserts the one-key entry where none stands; the record persists after the session.
It runs after the version gate on a verified CLI only, skips the user home folder and any folder already trusted by its own key or a parent up to the git root, goes through the one guarded trust record writer with three read-backs, and never refuses the start.
A new trust-record log event (one per start, its detail one of 16 closed start words, no path, key or value) grows the closed event set from 15 to 16; CLAUDE_CONFIG_DIR's value is now read at the start too when claude_env_keep keeps it.
Retired: the rule that run and revive write no trust record, and the fake agent's oracle that inside a repository only the root's record counts; every test-side run and revive and the harness's wrappers now get a test-owned user home.
Proven live on claude 2.1.325 (a wrapped start in a fresh untrusted folder shows the input box, a plain claude start shows the trust form); no dependency, CI step, IPC method, exit code or json code moved.
@task_file n=1 path=crates/viola-agent-claude/src/trust.rs
what: the Claude adapter's trust record shape logic, no file I/O: the CLI's trust walk over the exact key then each parent to the git root, the flag-only set, the exact read-back, and the closed trust and start word sets
@task_file n=2 path=src/trust_record.rs
what: the guarded trust record writer under the Claude config (link, regular-file, owner, MAX_FRAME and lock checks, read-back), the child's config file resolution, and now the start's trust step with the home-folder rule
@task_file n=3 path=src/cmd/run.rs
what: viola run and the start of viola revive: the start order (strict modes, version gate, socket-directory check, bind, spawn), now with the trust step after the version gate
@task_file n=4 path=src/cmd/verify.rs
what: viola verify, the real-CLI verify that writes its own trust record and the stamps; it now calls the shared config-file resolution
@task_file n=5 path=crates/viola-core/src/obs.rs
what: the closed observability event set and its log-format literals, now with the trust-record event
@task_file n=6 path=schemas/diag-line.v1.json
what: the diagnostics log line schema: the event enum, each event's message and its closed detail words
@task_file n=7 path=schemas/diag-detail.v1.json
what: the diagnostics detail schema's event enum
@task_file n=8 path=src/bin/viola-fake-agent.rs
what: the fake agent test binary under the fake-agent feature: its trust-config reading, now the CLI's walk from its cwd to the git root
@task_file n=9 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's test cases and options, its trust walk test
@task_file n=10 path=tests/support/home.rs
what: the test support for test homes and wrapper boots: the unmade user home, an untrusted boot as a user and a revive as a user
@task_file n=11 path=tests/support/cli.rs
what: the test CLI runner, which now gives every test run and revive a test-owned user home
@task_file n=12 path=tests/run_cli.rs
what: the integration tests of viola run, now with the start's trust cases (untrusted folder, flag false, already trusted, trusted parent, unverified home, user home, kept config dir, refusal forms)
@task_file n=13 path=tests/cli_revive.rs
what: the integration tests of viola revive, now with the revive writing the record for its recorded cwd
@task_file n=14 path=tests/tui_passthrough.rs
what: the terminal passthrough tests: a trust-writing start adds no viola byte to the terminal
@task_file n=15 path=tests/contract_diag_schema.rs
what: the contract test that diagnostics lines from real runs validate against the schema and the event enum equals the log-format literals
@task_file n=16 path=tests/contract_bounded_readers.rs
what: the bounded readers census contract test: every product reader bounded or named
@task_file n=17 path=tests/chaos_snapshot_replay.rs
what: the replayed snapshot chaos tests, now run under a test-owned user home
@task_file n=18 path=tests/cli_controls_not_disableable.rs
what: the tests that no config, env var or flag switches off a control, now run under a test-owned user home
@task_file n=19 path=tests/cli_fake_agent.rs
what: the fake agent CLI tests, now run under a test-owned user home
@task_file n=20 path=tests/cli_version_gate.rs
what: the tests of viola run's version gate, now run under a test-owned user home
@task_file n=21 path=tests/cross_exit_causes.rs
what: the cross-process exit cause tests, now run under a test-owned user home
@task_file n=22 path=tests/cross_killed_test.rs
what: the killed-test cross-process tests, now run under a test-owned user home
@task_file n=23 path=tests/channel_error_bodies.rs
what: the channel error body tests, now run under a test-owned user home
@task_file n=24 path=crates/viola-e2e/src/harness/boot.rs
what: the e2e harness boot of supervised wrappers, now with a session user home seeded for a stamped session
@task_file n=25 path=crates/viola-e2e/src/harness/supervise.rs
what: the e2e harness supervision of wrappers, now handing them the session user home
@task_file n=26 path=crates/viola-e2e/src/rig/mod.rs
what: the live round rig: its arms against the real claude CLI, the floor rows, the ledger cap, now the trust start and trust control arms
@task_file n=27 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's isolated scratch: the child's own config away from the user's, now a seeded sentinel entry, a fresh untrusted cwd and the entry read-back words
@task_file n=28 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig binary's command line, now with the live S1 and S2 arms and their rehearsals
@task_file n=29 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig rehearsal tests on the fake agent, now for the trust start and trust control arms
@task_item n=1 part=touches
The trust record writer guarded decision: the guarded writer now also serves the start of run and revive, flag only, beside verify's own trust record; it stays the one writer.
@task_item n=2 part=touches
The rows that viola writes nothing in the Claude config or under the Claude home, with verify's own trust record as the one write: run and revive's start now write a second kind of trust record, so those rows are stale.
@task_item n=3 part=touches
The trust record closed words decision: a new closed set of 16 start words sits beside the writer's words; key-present stays verify's only and removed and key-present map to lost at the start.
@task_item n=4 part=touches
The recorded cwd rows (recorded cwd kept to the snapshot, used only to spawn, spawn directory kept in the snapshot only, revive spawns in the recorded cwd): the recorded cwd now also keys the revive's trust record, a second use.
@task_item n=5 part=touches
The rows that say CLAUDE_CONFIG_DIR's value is read by verify only (R8 CLAUDE environment strip, credential strip by name only, CLAUDE values never logged and the like): run and revive's start now read that value too when claude_env_keep keeps the name, to find the child's config file.
@task_item n=6 part=touches
The new env read needs a log entry convention: the start's read of CLAUDE_CONFIG_DIR's value is a new environment read at run and revive.
@task_item n=7 part=touches
The verify needs a pre-trusted folder rows and verify trusted runs accepted risk: viola itself now makes an untrusted spawn folder trusted at a wrapped start, and the record persists after the session (the F-T1 price).
@task_item n=8 part=touches
The unverified CLI gate decision: the trust step runs only when the version gate reads the CLI verified; an unverified CLI reads no config file.
@task_item n=9 part=touches
The verify never answers dialogs and never types into a dialog rows, and the live round rig keys only into its own pty: the trust dialog is avoided by the record, no key is typed, and the live S1 and S2 starts type none.
@task_item n=10 part=touches
The bounded readers census, the one MAX_FRAME cap and the serde_json default depth rows: the start's config read is a new product reader, bounded at MAX_FRAME and parsed at the default depth.
@task_item n=11 part=touches
The security refusals never block the human rule: the start's trust word never refuses the start; no exit code, hint or json code moves.
@task_item n=12 part=touches
The rows on what the logs and diagnostics may hold (user content only to diagnostics, never log CLAUDE values, transcript path redacted): the new trust-record line carries a closed word only, no path, key or value.
@task_item n=13 part=touches
The tests use their own home decision: every test-side run and revive and the harness's supervised wrappers now also get a test-owned user home, so no test touches the runner's real user config.
@task_item n=14 part=touches
The live rig rows (live rig child isolated from the user's config, child config seeded from bundle logic only): the rig now seeds a sentinel entry into the child's config and reads it back unchanged.
@task_item n=15 part=touches
The same-user tampering outside the boundary decision: a same-user racer that adds a key during the read-back leaves the flag set and the next walk reads the folder trusted.
@task_item n=16 part=touches
The strict-modes and start-order rows over run and revive's start: the trust step sits after the version gate and before the endpoint bind.
@task_item n=17 part=touches
The test seams only under the fake-agent feature rows: the fake agent's trust reading changed to the CLI's walk and its root-only oracle is retired.
@task_fact n=1 slug=data-protection
fact: run and the start of revive write the claude CLI's trust record for an untrusted spawn folder, flag only (hasTrustDialogAccepted set, every other key kept, a one-key entry inserted where none stands), on a verified CLI only, and the record persists after the session.
@task_fact n=2 slug=data-protection
fact: the start writes no trust record for the user home folder or for a folder already trusted by its own key or by a parent up to the git root, by the CLI's own walk.
@task_fact n=3 slug=data-protection
fact: the start's trust write goes through the guarded trust record writer (no link, regular file, owner, MAX_FRAME size, lock, three read-backs) and each refusal form writes nothing.
@task_fact n=4 slug=input-validation
fact: the start's trust outcome is one of a closed set of 16 start words, logged as one trust-record line per start with no path, key or value, and it never refuses the start.
@task_fact n=5 slug=input-validation
fact: the recorded cwd keys the revive's trust record, a second use beside the spawn directory.
@task_fact n=6 slug=secret-management
fact: CLAUDE_CONFIG_DIR's value is read by run and revive's start too, when claude_env_keep keeps the name, to find the child's config file.
@task_fact n=7 slug=secret-management
fact: a test that starts the product never runs under the runner's real user home: every test-side run and revive and every supervised harness wrapper gets a test-owned user home.
@task_item n=1 part=untouched
Dependency security: no crate, feature, pin, advisory ignore or toolchain moved.
@task_item n=2 part=untouched
API security and the MCP tools: no rmcp feature, MCP tool or MCP input moved.
@task_item n=3 part=untouched
The channel's methods, frames, error codes and peer checks (socket directory, peer euid, server verification): no IPC method, endpoint, socket or port was added or changed.
@task_item n=4 part=untouched
CI workflows, the artifact canary scan and the release build rows: no CI step, workflow or release gate changed.
@task_item n=5 part=untouched
Paste, send and answer text validation and their refusal words: no typed-text rule moved.
@task_item n=6 part=untouched
The pinned binary and plugin file rows: no pinning, re-hash or plugin rewrite changed.
@task_item n=7 part=untouched
User accounts, secrets at rest and the network absences: no account, stored secret, network call or listener was added.
