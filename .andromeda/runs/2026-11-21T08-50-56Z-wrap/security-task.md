@task_prose part=session
The session lands the R8 identity floor as the 28th and last relied ledger row, identity-floor, with a viola verify probe: every CLAUDE name the CLI adds to a hook must be a known name.
The identity floor grows from 11 to 13 names, adding CLAUDE_CODE_CHILD_NOTES_DIR and CLAUDE_CODE_LAUNCHED_BY_SESSION_IDS; it is still read by the strip of every viola child and now also by the env reading.
A new four-name list of non-identity hook names (CLAUDE_PLUGIN_ROOT, CLAUDE_PROJECT_DIR, CLAUDE_ENV_FILE, CLAUDE_PLUGIN_DATA) is read only by the env reading.
The hidden hook capture arm on SessionStart now also writes an owner-only (0600) env-names file: CLAUDE names kept, every other name counted, no value ever read; verify reads it bounded by MAX_FRAME, stamps an env_probe block of names and counts, and with record writes a names-only Env.recorded-1.json fixture that the scrub refuses as username when a name carries the user word.
The 2.1.328 fixture set was re-recorded whole; the live rig gained kept-stamp labels HF, HF-2 and HG; the CI canary scan's critical list grew 12 to 16. Verified live once (HG, 28/28) on 2.1.328 and CI green.
The cross-session prefix row, its sender and the harness-prefix id list were written by a stopped run and cut whole, never committed; nothing about them is claimed.
@task_file n=1 path=crates/viola-agent-claude/src/lib.rs
what: the Claude adapter's identity floor (the parent identity names stripped from every claude child) and the new non-identity hook-name list, with the child env strip plan
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the relied ledger rows, the verify stamp's merge, the env-capture document, the env reading and the recorder's env-name scrub fault
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: unit tests of the ledger's env-capture document and env file name
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit tests of the identity-floor row check: pass on floor and hook names, fail on an unknown name, kept names dropped first
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: unit tests of the ledger's verify runs, moved to the 28-row count
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit tests of the verify stamp's merge, the env_probe block and the verified/unverified row sets
@task_file n=7 path=src/cmd/hook.rs
what: the viola hook command and its hidden capture arm, which now also writes the SessionStart env-names file owner-only
@task_file n=8 path=src/cmd/verify.rs
what: viola verify: the bounded env-file read, the identity-floor step, the stamp's env_probe, the record of the env fixture and the fixture scrub refusing a user-word name
@task_file n=9 path=src/run/version_gate.rs
what: viola run's version gate, which reads the verify stamp; its tests moved to the new stamp merge
@task_file n=10 path=src/run/auto_verify.rs
what: the automatic verify of a new CLI version at start; its tests moved to the new stamp merge
@task_file n=11 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent, now handing each recorded env name to every SessionStart hook child with a fixed fake value
@task_file n=12 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: fake agent tests of the SessionStart env-name replay
@task_file n=13 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: fake agent option tests, moved to the re-recorded fixture set's session ids
@task_file n=14 path=crates/viola-e2e/src/rig/mod.rs
what: the live rig: its kept-stamp labels HF, HF-2, HG with owner-only kept stamps, its identity-floor and env-unknown summary lines
@task_file n=15 path=crates/viola-e2e/src/harness/run.rs
what: the test harness runner's expected live verify step lines, moved to 28 rows
@task_file n=16 path=crates/viola-e2e/src/harness/secret_scan.rs
what: the CI artifact canary scan's critical canary list, grown by the kept, other, notes and launched env canaries
@task_file n=17 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig rehearsal tests, pass-count moved to 28/28
@task_file n=18 path=fixtures/claude/2.1.328/Env.recorded-1.json
what: a new recorded fixture class: the CLAUDE env names a hook receives, names only, no value
@task_file n=19 path=fixtures/claude/2.1.328/SessionStart.default.json
what: a recorded hook payload fixture of the 2.1.328 set, re-recorded whole with the rest of the set under the fixture scrub
@task_file n=20 path=tests/security_negatives_env_capture.rs
what: security negative tests: the capture arm writes env names and no value, no env file for other events, a kept name reaches no stamp or record, a user-word name is refused and nothing recorded
@task_file n=21 path=tests/tui_env_strip.rs
what: the child env strip oracle: the thirteen floor names and an unknown name stripped, a kept floor name stripped all the same
@task_file n=22 path=tests/contract_fixture_hygiene.rs
what: the recorded-fixture hygiene walk, with a new env fixture class (shape, username, absolute path, email checks)
@task_file n=23 path=tests/cli_verify.rs
what: viola verify integration tests: identity-floor pass over recorded env names and fail on an unknown name
@task_file n=24 path=tests/cli_verify/runs.rs
what: viola verify run tests, moved to the env-names file beside the capture files
@task_file n=25 path=tests/cli_verify/stamps.rs
what: viola verify stamp tests, the env_probe block and the 28-row count
@task_file n=26 path=tests/contract_ledger_probes.rs
what: the ledger probe contract over every stamped fixture set, at 28 rows
@task_file n=27 path=tests/contract_fake_agent_drift.rs
what: the fake agent drift contract against the recorded fixtures, at 28-row literals
@task_file n=28 path=tests/support/verify.rs
what: test support for verify: the four non-identity names and a writer of an env fixture set
@task_file n=29 path=tests/support/start.rs
what: test support for start, its pinned recorded session id moved
@task_file n=30 path=tests/chaos_revive.rs
what: revive chaos tests, their pinned recorded session id moved
@task_file n=31 path=tests/cli_send.rs
what: viola send integration tests, their pinned clear session id moved
@task_file n=32 path=Cargo.toml
what: the workspace manifest's test target list, registering the env-capture security negative tests
@task_file n=33 path=.config/nextest.toml
what: the test runner's configuration, one line for the new test target
@task_item n=1 part=touches
decision/identity-floor-eleven-names is stale: the floor is now thirteen names, the eleven plus CLAUDE_CODE_CHILD_NOTES_DIR and CLAUDE_CODE_LAUNCHED_BY_SESSION_IDS; the row must name the thirteen by name and its title no longer holds.
@task_item n=2 part=touches
decision/r8-claude-env-strip: the floor by name grows to thirteen, and a four-name non-identity hook list (CLAUDE_PLUGIN_ROOT, CLAUDE_PROJECT_DIR, CLAUDE_ENV_FILE, CLAUDE_PLUGIN_DATA) now exists, read only by the env reading; names only.
@task_item n=3 part=touches
decision/parent-identity-env-stripped and decision/claude-credentials-stripped-from-child: the strip of every viola child (run, verify, the agents read) now removes the two added floor names.
@task_item n=4 part=touches
contract/config-json/claude-env-keep and contract/claude-child-process-start/env-stripped-known: a floor name listed in the keep list is stripped all the same, and the known stripped set is now the thirteen floor names; where a cell holds the floor count it moved 11 to 13.
@task_item n=5 part=touches
decision/hook-capture-arm and decision/capture-arms-isolated: the hidden capture arm now also writes, on SessionStart alone, an owner-only env-names file of its own process: CLAUDE names kept, other names counted, no value read.
@task_item n=6 part=touches
decision/new-env-read-needs-log-entry: the capture arm's read of its process's env names and verify's env reading are a new environment read and owe the Decisions Log entry.
@task_item n=7 part=touches
decision/never-log-claude-values, decision/credential-strip-by-name-only and decision/inherited-credentials-never-serialised: CLAUDE env names now reach a capture file, the verify stamp and a recorded fixture, names only, values never read, kept names removed before stamp and record.
@task_item n=8 part=touches
decision/fixture-scrub-and-refusal, decision/fixture-recording-scrubbed, decision/fixture-hygiene and decision/viola-format-fixtures-hygiene: a new recorded fixture class, the env-names record, passes the record scrub and an env-name fault and has its own class in the hygiene walk.
@task_item n=9 part=touches
contract/fixture-refusal-codes/username and contract/fixture-refusal-line/form: verify's record refuses an env name carrying the user word as the file plus username, recording nothing.
@task_item n=10 part=touches
decision/verify-probe-captures-transient and decision/probe-artifacts-cleanup: the SessionStart env-names file is a new verify probe capture beside the payload captures.
@task_item n=11 part=touches
decision/max-frame-cap, decision/reads-capped-at-max-frame and decision/bounded-readers-census: verify's read of the env-names file is a new product reader, bounded by MAX_FRAME and shape-checked, an off-shape document read as unread.
@task_item n=12 part=touches
decision/stamps-written-only-by-verify: the verify stamp gains a measured env_probe block (read, added, unknown, other count, unshaped) of names and counts only.
@task_item n=13 part=touches
decision/unverified-cli-gate and decision/version-gate-strict-stamps-read: the relied rows went 27 to 28, so a 27-row stamp now reads unverified and only a 28-row stamp verified.
@task_item n=14 part=touches
decision/live-rig-kept-stamp-owner-only is stale: its labels lack HF, HF-2 and HG with their kept stamps floor-stamp and floor-stamp-2.
@task_item n=15 part=touches
decision/artifact-canary-scan and contract/artifact-canary-scan-classes/claude-stripped: the critical canary list grew 12 to 16 with the kept, other, notes and launched env canaries.
@task_item n=16 part=touches
decision/test-seams-fake-agent-only and decision/test-seams-compiled-out: the test-only fake agent now replays recorded env names to SessionStart hook children with a compiled fake value.
@task_item n=17 part=touches
decision/live-rig-isolated-child: the HG live start ran claude 2.1.328 under the rig's private config and scratch home, the live home's stamp read unmoved.
@task_item n=18 part=touches
decision/env-not-config-channel and decision/controls-not-switchable: no config key or env var can keep a floor name; the keep list cannot keep the floor.
@task_fact n=1 slug=secret-management
fact: The identity floor is thirteen names by name and is checked by the 28th relied ledger row, identity-floor: every CLAUDE name the CLI adds to a hook must be a floor name or one of four non-identity hook names, an unknown name fails the row, and names kept by claude_env_keep are dropped before the check, the stamp and the record.
@task_fact n=2 slug=secret-management
fact: Four CLAUDE names the CLI adds to a hook are known and not identity, CLAUDE_PLUGIN_ROOT, CLAUDE_PROJECT_DIR, CLAUDE_ENV_FILE and CLAUDE_PLUGIN_DATA; they are read only by verify's env reading and are not on the strip floor.
@task_fact n=3 slug=input-validation
fact: The hidden capture arm on SessionStart alone writes an owner-only env-names file of its own process: CLAUDE names kept, every other name only counted, no value ever read; it reads no VIOLA name and adds no env var.
@task_fact n=4 slug=data-protection
fact: The recorded env fixture holds only a version and the added CLAUDE names, never a value; it passes the record scrub and the env-name fault, an env name carrying the user word is refused as username and nothing is recorded, and the hygiene walk checks it as its own class.
@task_fact n=5 slug=secret-management
fact: The verify stamp's env_probe holds names and counts only (read, added, unknown, other count, unshaped), kept names removed before it is written.
@task_fact n=6 slug=secret-management
fact: CLAUDE_CODE_LAUNCHED_BY_SESSION_IDS stands on the identity floor on a reading of the CLI bundle's strings alone, never met live; the identity row is verified by one recording on one version (2.1.328) under a scratch home, a provisional floor accepted as such.
@task_item n=1 part=untouched
Authentication and authorization: peer euid checks, socket directory and socket mode rules, server verification and strict-modes refusals are not reached; no listener, endpoint, socket or channel method was added.
@task_item n=2 part=untouched
The wrapper channel methods, MCP tools and their contracts, and the IPC refusal and error contracts are unchanged; no event kind or status shape moved.
@task_item n=3 part=untouched
Typed text validation (paste and dialog free-text character sets, control-character refusal, empty-text refusal) and the send refusal contracts are not reached.
@task_item n=4 part=untouched
Dependency security: no crate, feature, toolchain, action pin or workflow permission was added or changed; licence and advisory lists are unchanged.
@task_item n=5 part=untouched
Harness-prefix literals and anything only for a cross-session sender, its inbound accept setting or a print-mode send tool: the cross-session row was cut whole and never committed, so rows ratified only for those are not amended.
@task_item n=6 part=untouched
Absences stand: no outbound network call, public listener, stored secret, model API call or release stage was added.
@task_item n=7 part=untouched
Error-handling rows for channel faults, exit codes, instance-unreachable details and hook-decision details are unchanged; only verify's existing username fixture refusal gained a new subject.
