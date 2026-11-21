@task_prose part=session
The session landed the R8 identity floor as the 28th and last relied ledger row, id identity-floor, words: every CLAUDE name the CLI adds to a hook is a known name; relied rows moved 27 to 28 and a 27-row stamp now reads unverified.
The identity floor grew from 11 to 13 names (adds CLAUDE_CODE_CHILD_NOTES_DIR and CLAUDE_CODE_LAUNCHED_BY_SESSION_IDS), read by the strip of every viola child and by the new check; a new list of 4 non-identity hook names (CLAUDE_PLUGIN_ROOT, CLAUDE_PROJECT_DIR, CLAUDE_ENV_FILE, CLAUDE_PLUGIN_DATA) is read only by the check.
The hidden capture arm on SessionStart now also writes Env.<k>.json, owner-only, holding the process's CLAUDE env names only, every other name counted, no value; viola verify reads Run B's first SessionStart env file under MAX_FRAME, prints an identity-floor step line, stamps measured.env_probe and with --record writes a new names-only fixture class Env.recorded-1.json.
The fake agent replays the recorded env names to each SessionStart hook child; the claude 2.1.328 fixture set was re-recorded whole; live verified once on 2.1.328 (identity-floor pass, 28/28) and CI green.
The planned cross-session prefix row, its sender, receiver and harness-prefix list were cut whole before commit and never landed; nothing was retired from the committed tree.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger: the relied row list, the probes, the stamp merge and envelope, the identity-floor check, the env capture document and the fixture scrub fault for env names
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: the agent crate's identity floor list and the new non-identity hook name list, and the child environment strip plan
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the ledger's unit tests for the env capture document and the env file name
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the ledger's unit tests of the relied row checks, now the identity-floor pass and fail table
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: the ledger's unit tests of verify's runs and their row count
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger's unit tests of the stamp merge, the verified-version rule and the env_probe block
@task_file n=7 path=src/cmd/hook.rs
what: the viola hook command and its hidden --capture probe arm, which now writes the env-names file beside a SessionStart payload
@task_file n=8 path=src/cmd/verify.rs
what: viola verify: its probe runs, step and summary lines, stamp measure, fixture record and record scrub
@task_file n=9 path=src/run/version_gate.rs
what: the start's version gate over the ledger stamps (tests moved to the new stamp merge input)
@task_file n=10 path=src/run/auto_verify.rs
what: the start's automatic verify of a new CLI version (tests moved to the new stamp merge input)
@task_file n=11 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent, now handing the recorded env names to every SessionStart hook child
@task_file n=12 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the fake agent's mode tests, the SessionStart env replay case
@task_file n=13 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's option tests over the recorded fixture set, its pinned session id
@task_file n=14 path=fixtures/claude/2.1.328
what: the recorded hook payload fixture set for claude 2.1.328, re-recorded whole, 32 files
@task_file n=15 path=fixtures/claude/2.1.328/Env.recorded-1.json
what: the new recorded env-names fixture, names only, the fake agent's SessionStart env source
@task_file n=16 path=crates/viola-e2e/src/harness/run.rs
what: the test harness runner and its live verify pass literal of 28 step lines
@task_file n=17 path=crates/viola-e2e/src/harness/secret_scan.rs
what: the harness secret scan and its critical canary list
@task_file n=18 path=crates/viola-e2e/src/rig/mod.rs
what: the standing live rig: its start labels, kept stamps, summary lines and floor row count
@task_file n=19 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsal tests and their pass-count literals
@task_file n=20 path=tests/security_negatives_env_capture.rs
what: the security negative tests of the capture arm's env-names file, the canary values and the kept-name and user-word cases
@task_file n=21 path=tests/tui_env_strip.rs
what: the child environment strip oracle, the list of stripped identity and unknown CLAUDE names
@task_file n=22 path=tests/contract_fixture_hygiene.rs
what: the recorded fixture hygiene walk, now with an env fixture class
@task_file n=23 path=tests/contract_ledger_probes.rs
what: the ledger probe contract test over every stamped fixture set
@task_file n=24 path=tests/contract_fake_agent_drift.rs
what: the fake agent drift contract against the recorded fixtures, its relied row literals
@task_file n=25 path=tests/cli_verify.rs
what: viola verify's integration tests, now the identity-floor pass and unknown-name fail cases
@task_file n=26 path=tests/cli_verify/runs.rs
what: viola verify's run tests and the probe files each run leaves
@task_file n=27 path=tests/cli_verify/stamps.rs
what: viola verify's stamp tests, the 28-row stamp and its env block
@task_file n=28 path=tests/support/verify.rs
what: the test support for verify: the env-names set writer and the known hook names
@task_file n=29 path=tests/support/start.rs
what: the test support for starts, its pinned recorded session id
@task_file n=30 path=tests/chaos_revive.rs
what: the revive chaos tests, their pinned recorded session id
@task_file n=31 path=tests/cli_send.rs
what: the send integration tests, the pinned clear session id
@task_file n=32 path=Cargo.toml
what: the root package manifest, its test target entries
@task_file n=33 path=.config/nextest.toml
what: the nextest test runner configuration and its test groups
@task_item n=1 part=touches
decision/identity-floor-eleven-names is stale: the floor is now thirteen names, adding CLAUDE_CODE_CHILD_NOTES_DIR and CLAUDE_CODE_LAUNCHED_BY_SESSION_IDS; its name and any count of eleven in its cells moved.
@task_item n=2 part=touches
decision/r8-identity-floor: the R8 parent-identity floor is now thirteen names and is itself checked by a relied ledger row with a viola verify probe; any eleven-name figure in it is stale.
@task_item n=3 part=touches
decision/probe-capture-arms-inert: the capture arm on SessionStart now also writes an env-names file; still inert, no line, values never read.
@task_item n=4 part=touches
contract/hook-capture-arm/payload-file and contract/probe-capture/payload-file: a new file Env.<k>.json, 0600, on SessionStart alone, beside the <PascalEvent>.<k>.json payload.
@task_item n=5 part=touches
contract/ledger-stamps/content and contract/ledger-stamps/measured: the stamp gains measured.env_probe with read, added, unknown, other_count and unshaped; kept names never reach it.
@task_item n=6 part=touches
contract/ledger-stamps/rows, contract/ledger-stamps/verified-version and decision/version-verified-all-relied-rows: relied rows 27 to 28; a 27-row stamp reads unverified, a 28-row one verified.
@task_item n=7 part=touches
decision/cli-capability-ledger, decision/capability-ledger and decision/capability-ledger-gate: a new relied row identity-floor, the 28th and last.
@task_item n=8 part=touches
contract/viola-verify-output/step-line and contract/viola-verify-output/summary-line: step lines read [NN/28], a new identity-floor step line, summary 28 pass 0 fail.
@task_item n=9 part=touches
contract/viola-verify-exit-codes/1-relied-row-failed and contract/viola-verify-exit-codes/1: an unknown CLAUDE name or an unread env file now fails the identity-floor row and exits 1.
@task_item n=10 part=touches
decision/verify-record-only-clean, decision/recorded-fixtures-scrubbed and contract/unclean-fixture-refusal/username: a new names-only fixture class Env.recorded-1.json goes through the record scrub; an env name carrying the user word is refused as <file> username.
@task_item n=11 part=touches
decision/child-env-claude-strip, decision/claude-env-stripped-from-child, decision/env-strip-names-only and decision/env-strip-reads-names-only: the strip of every viola child reads the thirteen-name floor; a floor name is stripped even when listed to keep.
@task_item n=12 part=touches
contract/config-json/claude-env-keep and contract/process-start-claude-child/env-kept and env-stripped-known: floor names stay stripped whatever claude_env_keep lists; kept names are removed from the env reading before the check, stamp and record.
@task_item n=13 part=touches
decision/plugin-env-vars-unused: CLAUDE_PLUGIN_ROOT, CLAUDE_PLUGIN_DATA, CLAUDE_PROJECT_DIR and CLAUDE_ENV_FILE are now listed as known non-identity hook names, read only by the identity-floor check; viola still uses none.
@task_item n=14 part=touches
decision/verify-run-e-resumes-run-b-first-session and decision/verify-child-set: verify now reads Run B's first SessionStart env file for the new row; no child added.
@task_item n=15 part=touches
decision/bounded-readers and contract/wrapper-channel-frames/max-frame: verify's env file read is bounded by MAX_FRAME; an off-shape document reads unread.
@task_item n=16 part=touches
decision/stamps-single-writer, decision/verify-sole-stamps-writer and decision/stamps-merge-own-version-only: the stamp merge now takes all probe readings including env; verify stays the one writer.
@task_item n=17 part=touches
decision/start-verifies-new-cli-version and decision/automatic-verify-attempt-lock: the automatic verify of a new CLI version now must pass 28 rows, the identity floor included.
@task_item n=18 part=touches
decision/fake-agent-test-seams and decision/tests-against-fake-agent: the fake agent now hands each recorded env name to every SessionStart hook child, value the compiled fake-agent word; no seam variable added.
@task_item n=19 part=touches
decision/no-probe-for-unrelied-shapes: the env names gain a probe because the identity floor is now a relied row.
@task_item n=20 part=touches
name/binary/viola-live-rig: the live rig gains start labels HF, HF-2, HG with kept stamps and summary lines identity-floor and env-unknown.
@task_fact n=1 slug=occupied-resources
fact: The identity floor is thirteen CLAUDE names, among them CLAUDE_CODE_CHILD_NOTES_DIR and CLAUDE_CODE_LAUNCHED_BY_SESSION_IDS; the latter stands on a reading of the CLI bundle's strings alone, never met live.
@task_fact n=2 slug=occupied-resources
fact: Four CLAUDE names the CLI adds to a hook are known non-identity names, CLAUDE_PLUGIN_ROOT, CLAUDE_PROJECT_DIR, CLAUDE_ENV_FILE and CLAUDE_PLUGIN_DATA, read only by the identity-floor check.
@task_fact n=3 slug=established-decisions
fact: The identity-floor relied ledger row passes when every CLAUDE name the CLI adds to a SessionStart hook is a floor name or a known non-identity name, and fails on an unknown name or an unread or off-shape env file; kept names are dropped before the check.
@task_fact n=4 slug=occupied-resources
fact: The capture arm writes Env.<k>.json owner-only on SessionStart alone, holding the CLAUDE names of its own process, every other name only counted, no value ever read or written.
@task_fact n=5 slug=standard-contracts
fact: The stamp's measured.env_probe holds read, added, unknown, other_count and unshaped, names only, kept names removed.
@task_fact n=6 slug=occupied-resources
fact: verify --record writes Env.recorded-1.json as {v:1, added:[names]}, names only, through the record scrub; the hygiene walk holds an env fixture class, and an env name carrying the user word is refused.
@task_fact n=7 slug=conventions
fact: The fake agent hands each name of a fixture set's recorded env-names file to every SessionStart hook child, its value a compiled fake-agent word, never a recorded value.
@task_item n=1 part=untouched
Harness prompt prefix classification and the harness-prompts-not-human rule: the cross-session prefix row was cut before commit and no prefix list or accept setting landed.
@task_item n=2 part=untouched
Channel methods, frames, fault codes, event kinds and event data: no IPC method, event kind, endpoint or socket added.
@task_item n=3 part=untouched
The VIOLA_ environment namespace and the wrapped child's VIOLA_BIN, VIOLA_DIR and VIOLA_NAME: no VIOLA_ name added or read.
@task_item n=4 part=untouched
The diagnostics line schema, process logs and role log files: unchanged; the capture arm writes no line.
@task_item n=5 part=untouched
MCP tools and results, the plugin and its install, CLI verbs other than verify and their exit codes.
@task_item n=6 part=untouched
Wheel, send, dialog hold, revive, list and board rules: no behaviour of the wrapper's sending or reading moved.
@task_item n=7 part=untouched
PTY seam, screen model, readiness gate and Hyprland seam: untouched.
@task_item n=8 part=untouched
CI workflows, release, toolchain pins, crate set and crate dependency direction: no crate added or removed, no workflow changed.
