@task_prose part=session
The session added a 28th and last relied ledger row, identity-floor, with its viola verify probe: every CLAUDE name the CLI adds to a hook must be a known name. Relied rows moved 27 to 28 in every step line, summary and count literal.
The identity floor grew from 11 to 13 names, now named in full (adding CLAUDE_CODE_CHILD_NOTES_DIR and CLAUDE_CODE_LAUNCHED_BY_SESSION_IDS). A new list of 4 non-identity hook names (CLAUDE_PLUGIN_ROOT, CLAUDE_PROJECT_DIR, CLAUDE_ENV_FILE, CLAUDE_PLUGIN_DATA) is read only by the probe.
The hidden hook capture arm now also writes an owner-only env-names file on SessionStart: CLAUDE names kept, every other name counted, no value. verify reads it into the stamp's env_probe block, and verify --record writes a new names-only fixture class, Env.recorded-1.json, through the record scrub and a username refusal.
The fake agent passes each recorded env name to every SessionStart hook child. The live rig gained the HF, HF-2 and HG labels, identity-floor and env-unknown summary lines, and FLOOR_ROWS 24. The claude 2.1.328 fixture set was re-recorded whole (31 to 32 files), and the strip oracle, the secret-scan canary list and the pinned recorded session ids were moved to match.
Measured: live HG on claude 2.1.328 read identity-floor pass, env-unknown 0 and 28/28, and CI was green 7/7. Retired: the cross-session-prefix row, its sender and the harness-prefix names, which were cut whole and never committed.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs
what: the relied ledger: the row list, each row's verify probe check, the stamp merge (now with the env_probe block), the env capture document and the env-names fixture fault check
@task_file n=2 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the ledger's inline unit tests: the env capture document and the env file name
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the ledger's inline unit tests of each row's probe check, including the identity-floor pass and fail table
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: the ledger's inline unit tests of the verify runs, holding the relied-row count literal
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger's inline unit tests of stamp writing and reading: the 27-row stamp read unverified, the 28-row one verified, and the env block written
@task_file n=6 path=crates/viola-agent-claude/src/lib.rs
what: the claude adapter's identity floor (the 13-name list behind the R8 CLAUDE env strip of every viola child) and the new 4-name non-identity hook list
@task_file n=7 path=src/cmd/hook.rs
what: viola hook, including the hidden capture arm, which now also files the owner-only env-names file beside a SessionStart payload
@task_file n=8 path=src/cmd/verify.rs
what: viola verify: the probe runs, the step and summary lines, the stamp, the fixture recording and its record scrub
@task_file n=9 path=src/run/auto_verify.rs
what: the automatic verify a viola run start does for a new CLI version, and its tests of the stamp merge
@task_file n=10 path=src/run/version_gate.rs
what: the unverified-CLI version gate, which needs every relied row, and its tests
@task_file n=11 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent that serves the claude boundary and replays recorded fixtures; it now hands recorded env names to SessionStart hook children
@task_file n=12 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the fake agent's unit tests of its modes, including the SessionStart hook carrying the recorded env names
@task_file n=13 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's unit tests of its command-line options and the recorded set it serves, including the resume case's pinned session id
@task_file n=14 path=crates/viola-e2e/src/rig/mod.rs
what: the standing committed live rig: its start labels, kept stamps, summary lines and the floor-row count
@task_file n=15 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the rig rehearsal tests over the fake agent, holding the pass-count literals
@task_file n=16 path=crates/viola-e2e/src/harness/run.rs
what: the harness run command, including the local-live suite and its expected live pass literal of 28 step lines
@task_file n=17 path=crates/viola-e2e/src/harness/secret_scan.rs
what: the harness secret scan and its single-sourced canary list (CRITICAL)
@task_file n=18 path=tests/cli_verify.rs
what: the viola verify integration suite run as a subprocess, including the identity-floor pass and fail cases over the recorded env names
@task_file n=19 path=tests/cli_verify/runs.rs
what: the viola verify integration cases of the verify runs and the files they write
@task_file n=20 path=tests/cli_verify/stamps.rs
what: the viola verify integration cases of the stamp content
@task_file n=21 path=tests/contract_fake_agent_drift.rs
what: the fake-agent drift contract test comparing it to the recorded fixtures, holding the relied-row literals
@task_file n=22 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene walk and its planted violations, now with an env-names class (shape, username, absolute path, email)
@task_file n=23 path=tests/contract_ledger_probes.rs
what: the ledger probe contract suite over every stamped fixture set
@task_file n=24 path=tests/security_negatives_env_capture.rs
what: a new security negative suite: the capture arm writes env names and no value, writes no env file for other events, keeps kept names out of stamp and record, and refuses a name carrying the user word
@task_file n=25 path=tests/tui_env_strip.rs
what: the child env strip test and its literal strip oracle STRIPPED (14 entries: the 13 floor names and one unknown), plus the floor-grows guard
@task_file n=26 path=tests/support/verify.rs
what: the shared test helper for verify runs, now writing a recorded env set
@task_file n=27 path=tests/support/start.rs
what: the shared test helper for starts, holding the pinned recorded fixture session id
@task_file n=28 path=tests/chaos_revive.rs
what: the chaos revive suite, holding the pinned recorded fixture session id
@task_file n=29 path=tests/cli_send.rs
what: the viola send integration suite, holding the pinned recorded clear session id
@task_file n=30 path=fixtures/claude/2.1.328/Env.recorded-1.json
what: a new recorded fixture class: the env names the CLI added to a SessionStart hook, names only
@task_file n=31 path=fixtures/claude/2.1.328
what: the recorded claude 2.1.328 fixture set (hook payloads, agents list, screens), re-recorded whole by verify
@task_file n=32 path=Cargo.toml
what: the workspace manifest, which registers the root integration test binaries
@task_file n=33 path=.config/nextest.toml
what: the nextest configuration: test groups and per-test kill bounds
@task_item n=1 part=touches
decision/identity-floor-tested-against-literal-copy: the floor went from eleven to thirteen names and is now pinned by name. A literal copy of the new four-name non-identity list is pinned beside it. The row's eleven-name wording is stale.
@task_item n=2 part=touches
decision/r8-claude-env-strip, decision/claude-env-strip, decision/claude-child-env-strip and decision/child-env-strip: the floor the strip removes now has thirteen names, including CLAUDE_CODE_CHILD_NOTES_DIR and CLAUDE_CODE_LAUNCHED_BY_SESSION_IDS. The new non-identity names are not stripped by the floor rule; only the probe reads them.
@task_item n=3 part=touches
decision/literal-test-oracles and decision/bounded-remove-the-guard-witness: the strip oracle STRIPPED now holds 14 entries. The floor-grows remove-the-guard control first failed to show the floor at all. It held only once the oracle kept the floor names and then asserted that none of them was kept.
@task_item n=4 part=touches
decision/ledger-row-verify-probe: a new relied row, identity-floor, has a verify probe that reads the env-names file of Run B's first SessionStart.
@task_item n=5 part=touches
decision/version-gate-needs-all-relied-rows and decision/stamps-refuse-widened-ledger: there are now 28 relied rows. A 27-row stamp reads unverified and a 28-row stamp verified.
@task_item n=6 part=touches
contract/verify-output/step-line, contract/verify-output/summary-line and contract/verify-output/last-line: step lines now read [NN/28] and the summary reads 28 pass 0 fail. A new identity-floor step line was added.
@task_item n=7 part=touches
decision/hook-payloads-recorded-by-verify, decision/hook-payloads-recorded-locally and decision/fixtures-per-cli-version: verify --record now writes a new names-only fixture class, Env.recorded-1.json. The 2.1.328 set was re-recorded whole and grew from 31 to 32 files.
@task_item n=8 part=touches
decision/fixture-named-by-hook-event and contract/claude-fixtures/payload-path: Env.recorded-1.json is a fixture that is not named by a hook event. The capture file Env.<k>.json sits beside the captured payloads.
@task_item n=9 part=touches
decision/fake-agent-matches-verify-fixtures, decision/fake-agent-replays-recorded-bytes, decision/fake-agent-matches-recordings and decision/fake-agent-replay-for-stamps: the fake agent now replays the recorded env names to every SessionStart hook child, with the compiled value fake-agent.
@task_item n=10 part=touches
contract/fake-agent-hooks/command-spawn and contract/fake-agent-receipt/env: the environment of a SessionStart hook child spawned by the fake agent now carries the recorded env names.
@task_item n=11 part=touches
decision/one-verify-stamp-per-build and decision/stamps-from-fake-agent-verify: the stamp gained a measured.env_probe block (read, added, unknown, other_count, unshaped), filled by a verify against the fake agent.
@task_item n=12 part=touches
decision/standing-live-rig, decision/live-failing-reading-not-verification and decision/rig-no-key-read-from-keys-word: the rig gained the start labels HF, HF-2 and HG with kept floor stamps, and the summary lines identity-floor and env-unknown. FLOOR_ROWS is now 24 and pass-count 28/28. The red HF start is recorded as a reading.
@task_item n=13 part=touches
decision/fixture-hygiene-walk, decision/fixtures-walked-for-hygiene, decision/planted-hygiene-violations and decision/hygiene-reports-class-only: the hygiene walk has a new env-names class. It checks the shape and refuses a username, an absolute path or an email, each proved by a planted case.
@task_item n=14 part=touches
decision/recorder-scrub, decision/dirty-payload-refuses-recording, decision/scrubbed-fixtures and contract/verify-exit-detail/unclean-fixture: the record scrub refuses an env name that carries the user word, as file and username, and records nothing.
@task_item n=15 part=touches
decision/canary-list-single-sourced, decision/synthetic-inputs-with-canary and decision/secret-scan-scope: the secret-scan CRITICAL canary list grew from 12 to 16, adding the kept, other, notes and launched canaries.
@task_item n=16 part=touches
decision/one-negative-per-security-control: the new security negative suite covers the capture arm. It writes names and never values, writes no env file for any event but SessionStart, and keeps kept names out of the stamp and the record.
@task_item n=17 part=touches
decision/ledger-probe-contract-suite: the probe suite now reads 2.1.328 at 28 pass 0 fail, with the env replay.
@task_item n=18 part=touches
contract/local-live-suite/pass: the local-live suite's expected pass now has 28 step lines.
@task_item n=19 part=touches
decision/fixture-drift-byte-compare, decision/fake-transcripts-compared-to-recordings and decision/send-cases-copy-recorded-fixtures: the whole re-record moved the recorded session ids. Four test sources that pin them as literals were moved with it.
@task_item n=20 part=touches
decision/real-claude-local-only and decision/live-verify-never-in-ci: the identity row is verified live by one local start, HG, on one version under a scratch home. CI replays only the recorded env names.
@task_item n=21 part=touches
contract/process-start-claude-child/env-stripped-known and contract/process-start-claude-child/env-stripped-count: the set of known floor names stripped from the claude child grew to thirteen.
@task_fact n=1 slug=4-unit-test-strategy
fact: Four non-identity hook names (CLAUDE_PLUGIN_ROOT, CLAUDE_PROJECT_DIR, CLAUDE_ENV_FILE, CLAUDE_PLUGIN_DATA) are pinned against a literal copy. Only the identity-floor probe reads them, and they are never on the floor.
@task_fact n=2 slug=7-test-data-fixtures
fact: Recorded env-names fixtures (Env.recorded-1.json) hold names only, in the shape v 1 plus added. They are recorded by verify through the record scrub and checked by their own hygiene class.
@task_fact n=3 slug=test-data-fixtures
fact: The fake agent gives every SessionStart hook child each name of its set's recorded env names, always with the fixed value fake-agent.
@task_fact n=4 slug=test-strategy
fact: A remove-the-guard control over the CLAUDE env strip needs an oracle that keeps the guarded floor names, because the strip removes any unkept CLAUDE name anyway.
@task_fact n=5 slug=test-data-fixtures
fact: A whole re-record of a fixture set moves its recorded session ids. Every test that pins those ids as literals moves with the re-record.
@task_fact n=6 slug=e2e-test-strategy
fact: The capture arm writes an owner-only env-names file on SessionStart alone: names only, no value, and every non-CLAUDE name only counted. Kept names never reach the stamp or the record, and a security negative suite with canaries holds this.
@task_fact n=7 slug=6-e2e-test-strategy
fact: The identity-floor row is verified by one live recording on one CLI version (2.1.328) under a scratch home. CLAUDE_CODE_LAUNCHED_BY_SESSION_IDS stands on the floor from a reading of the bundle's strings alone and has never been met live.
@task_item n=1 part=untouched
MCP server, MCP tools and their invalid-params, refusal and parity contracts: no tool, schema or reply changed.
@task_item n=2 part=untouched
The wrapper channel and its JSON-RPC errors, link and unlink methods, MAX_FRAME and peer checks: no IPC method, endpoint or event kind was added.
@task_item n=3 part=untouched
Send, answer, release, wait and last verbs, send confirmation, dialogs and hook decision bodies: their behaviour and documents are unchanged.
@task_item n=4 part=untouched
revive and run start refusals, exit causes, heartbeat staleness, snapshot replay and the event log format: no start or revive path changed.
@task_item n=5 part=untouched
CI workflows, supply chain, cargo-deny, zizmor, MSRV, fuzzing, mutation testing, performance budgets and coverage floors: no workflow, gate threshold or tool version changed.
@task_item n=6 part=untouched
Process log format, log event corr, the diagnostics schema, harness status, boot, cleanup, pre-push and gate contracts: no log format, status shape or harness command shape changed.
@task_item n=7 part=untouched
Human-mode output, list board, help, SGR and control-character escaping: no CLI human output changed.
