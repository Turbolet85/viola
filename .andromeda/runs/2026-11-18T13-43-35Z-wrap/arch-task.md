@task_prose part=session
viola run and viola revive now write the CLI's trust record for an untrusted spawn folder at a wrapped start, so no trust dialog stands and no key is typed (matrix v2-33).
The start sets only the hasTrustDialogAccepted flag in the folder's entry of the child's Claude config (inserting a one-key entry where none stands), through the guarded trust writer, after the version gate and before the endpoint bind, on a verified CLI only, with three read-backs; the resulting word never refuses the start.
The trust walk follows the CLI's: the exact key, then each parent folder up to the git root; the user's home folder is never trusted; revive keys the record by the snapshot's recorded cwd.
A new obs event trust-record (one INFO line per start, detail one of a new closed set of 16 start words, no path, key or value) moves the closed event set from 15 to 16; CLAUDE_CONFIG_DIR's value is now read by the start too.
Retired: the rule that run and revive write no trust record (verify was the only writer under the Claude config); the fake agent's oracle that inside a repository only the root's record counts.
Measured live on claude 2.1.325 only: a wrapped start in a fresh untrusted folder opens on the input box with the flag written; a plain claude in the same folder shape opens on the trust form.
@task_file n=1 path=crates/viola-agent-claude/src/trust.rs
what: the Claude-shape trust-record logic in the agent crate, no file I/O: the trust walk, the flag-only set, the exact read-back, and the closed start-word set
@task_file n=2 path=src/trust_record.rs
what: the guarded trust-config writer, the one writer under the Claude config, and the trust config file resolution now shared by verify and the run/revive start
@task_file n=3 path=src/cmd/run.rs
what: the viola run and revive start path and its start order: version gate, the new trust step, endpoint bind, child spawn
@task_file n=4 path=src/cmd/verify.rs
what: viola verify, the CLI capability probe and its own trust record writer
@task_file n=5 path=crates/viola-core/src/obs.rs
what: the closed obs event set and the obs event emitter of viola-core
@task_file n=6 path=schemas/diag-line.v1.json
what: the diagnostics line schema: its event enum, messages and per-event detail enums
@task_file n=7 path=schemas/diag-detail.v1.json
what: the diagnostics detail file schema and its event enum
@task_file n=8 path=src/bin/viola-fake-agent.rs
what: the fake agent test binary that stands in for the claude CLI, including its trust-config reading
@task_file n=9 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's own test cases for its options and trust reading
@task_file n=10 path=tests/support/home.rs
what: the test-support home helpers: isolated viola homes, stamped homes, wrapper boot and revive helpers, now a test-owned user home
@task_file n=11 path=tests/support/cli.rs
what: the test-support CLI runner that every test run and revive goes through, now given an unmade user home
@task_file n=12 path=tests/run_cli.rs
what: the integration tests of viola run, its role log lines and the new trust cases
@task_file n=13 path=tests/cli_revive.rs
what: the integration tests of viola revive
@task_file n=14 path=tests/tui_passthrough.rs
what: the terminal passthrough tests: run writes only the child's bytes to the terminal
@task_file n=15 path=tests/contract_diag_schema.rs
what: the contract test holding diagnostics lines from real runs to the diag-line schema and the event enum to the log-format literals
@task_file n=16 path=tests/contract_bounded_readers.rs
what: the contract test that every boundary names a bounded read that stands
@task_file n=17 path=tests/chaos_snapshot_replay.rs
what: the chaos tests of snapshot replay
@task_file n=18 path=tests/cli_controls_not_disableable.rs
what: the tests that the human controls cannot be disabled
@task_file n=19 path=tests/cli_fake_agent.rs
what: the integration tests running viola against the fake agent
@task_file n=20 path=tests/cli_version_gate.rs
what: the tests of the start's CLI version gate
@task_file n=21 path=tests/cross_exit_causes.rs
what: the cross-component tests of child exit causes
@task_file n=22 path=tests/cross_killed_test.rs
what: the cross-component test of a killed test's evidence
@task_file n=23 path=tests/channel_error_bodies.rs
what: the tests of wrapper channel error bodies
@task_file n=24 path=crates/viola-e2e/src/harness/boot.rs
what: the end-to-end harness's wrapper boot, now giving a session user home
@task_file n=25 path=crates/viola-e2e/src/harness/supervise.rs
what: the end-to-end harness's supervised wrappers
@task_file n=26 path=crates/viola-e2e/src/rig/mod.rs
what: the live real-CLI rig: its arms, ledger of base starts, cap, floor rows and summaries
@task_file n=27 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's scratch isolation: scratch roots, sentinel entry in the user config, fresh folders
@task_file n=28 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the viola-live-rig binary's command line, now with the S1 and S2 arms
@task_file n=29 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsal tests against the fake agent
@task_item n=1 part=touches
decision verify-own-trust-record is now stale: it holds that run and revive write no trust record and that verify is the only writer under the Claude config; the run/revive start now writes the flag too, through the same guarded writer.
@task_item n=2 part=touches
decision run-start-order: a trust step now stands after the version gate and before the endpoint bind, the child still spawned last.
@task_item n=3 part=touches
the trust-writer-words contract rows: key-present and removed are now verify's words only (the start maps them to lost); written, lost, lock-held and the config-* words are reused by the start; a new start-word set sits beside them.
@task_item n=4 part=touches
contract instance-snapshot/cwd gains a second use: revive keys its trust record by the recorded cwd, beside its use as the spawn dir.
@task_item n=5 part=touches
decision trust-record-ledger-row and version-verified-all-relied-rows: the start's write now relies on the trust-record row and runs only on a verified CLI (cli_verified).
@task_item n=6 part=touches
decisions unverified-cli-transport-only and partial-gate-on-unverified-cli: an unverified CLI now also means no trust record read or written at the start.
@task_item n=7 part=touches
decisions no-typing-into-trust-dialogs, verify-never-types-into-dialog and verify-types-nothing-into-dialogs: still held; the start now avoids the trust dialog by writing the record, not by typing.
@task_item n=8 part=touches
rows that say CLAUDE_CONFIG_DIR's value is read by verify only, or that the env strip reads names only (env-strip-names-only, env-strip-reads-names-only, child-env-claude-strip, claude-env-stripped-from-child, config-json/claude-env-keep, process-start-claude-child/env-kept): the run/revive start now reads that value when claude_env_keep keeps the name, to find the child's config file.
@task_item n=9 part=touches
decision no-environment-configuration and harness-env-unread-by-product: CLAUDE_CONFIG_DIR's value read at the start is a location of the child's file, not a viola setting.
@task_item n=10 part=touches
the closed obs event set moved 15 to 16 with trust-record: rows over the event set and the emitter (obs-event-only-emitter, contract diag-line/format, diagnostics-role-files/run-name, codes-only-role-logs, process-logs-hold-codes-only, codes-only-process-logs) where a cell holds the set or the line's content.
@task_item n=11 part=touches
the start writes one more role line per viola run (the trust-record line); a row counting a start's role lines or naming its lines moves.
@task_item n=12 part=touches
decisions agent-claude-pure, claude-shapes-in-one-crate and claude-shapes-isolated: the trust walk, flag-only set and read-back are Claude-shape and live in the agent crate with no file I/O; the file I/O stays in the guarded writer.
@task_item n=13 part=touches
decisions bounded-readers, serde-json-unbounded-depth-banned and serde-json-recursion-limit: the start reads the Claude config through the guarded writer's bounds (no link, regular file, owner, MAX_FRAME size, default depth); contract wrapper-channel-frames/max-frame now also bounds that read.
@task_item n=14 part=touches
decision separate-lock-files and crash-safe-disk-writes: the start now takes the Claude config's lock and replaces the file under it, pausing at most 2 x 100 ms.
@task_item n=15 part=touches
decision run-terminal-child-only and passthrough-never-waits-on-model: still held; a trust-writing start adds no viola byte to the terminal.
@task_item n=16 part=touches
decision test-isolated-viola-home: every test run and revive and every supervised harness wrapper now also gets a test-owned user home, so no test start touches the runner's real Claude config.
@task_item n=17 part=touches
decisions fake-agent-test-seams and tests-against-fake-agent: the fake agent's trust reading now walks from its cwd to the git root like the CLI; its old root-only oracle is retired.
@task_item n=18 part=touches
name viola-live-rig and decisions real-cli-probe-local-only, real-claude-local-only, no-live-claude-in-ci: the live rig gains the S1 and S2 arms (a verified wrapped start, a plain claude start) and the ts and tc rehearsals, still local only.
@task_item n=19 part=touches
the run and revive start refusals and exit codes (viola-run-start-exit-codes, run-start-refusals, viola-revive-json-error-codes): the trust step adds no refusal, exit code, hint or --json code; a row claiming the start's full step list moves.
@task_fact n=1 slug=established-decisions
fact: A wrapped start (run and revive) on a verified CLI writes the CLI's trust record for an untrusted spawn folder: only hasTrustDialogAccepted is set in that folder's entry of the child's Claude config, every other key kept, a one-key entry inserted where none stands; nothing is written for the user's home folder, a trusted key or a trusted parent; its word never refuses the start.
@task_fact n=2 slug=established-decisions
fact: The start's trust walk follows the CLI's: the exact folder key first, then each parent folder up to the git root; the walk, flag set and exact read-back are Claude-shape and pure, the write goes through the guarded writer with three read-back attempts.
@task_fact n=3 slug=standard-contracts
fact: Start trust words are a closed set of 16 code words, the detail of the trust-record line; verify's key-present and removed map to lost at the start.
@task_fact n=4 slug=occupied-resources
fact: The obs event trust-record: INFO, one line per run or revive start, detail one of the 16 start words, no path, key or value, no span.
@task_fact n=5 slug=occupied-resources
fact: CLAUDE_CONFIG_DIR's value is read by the run and revive start, when claude_env_keep keeps the name, to locate the child's Claude config; the same resolution verify uses.
@task_fact n=6 slug=occupied-resources
fact: A test that starts the product never runs under the runner's real HOME: every test run and revive and every supervised harness wrapper gets a test-owned user home, since the start writes into the user's Claude config.
@task_item n=1 part=untouched
MCP tools, their results and the MCP server rules: no MCP surface changed.
@task_item n=2 part=untouched
the wrapper channel methods, frames, fault codes and protocol version: no IPC method, endpoint, socket or port added.
@task_item n=3 part=untouched
the wheel, send, confirmation and dialog-hold rules: the trust step runs before the child exists and sends nothing.
@task_item n=4 part=untouched
the hook rules, hook tiers and hook-to-kind map: no hook changed.
@task_item n=5 part=untouched
the board and list payload, liveness and snapshot replay rules beyond the snapshot's cwd: unchanged.
@task_item n=6 part=untouched
dependency policy, licences, toolchain and MSRV pins, banned features: no dependency added or bumped.
@task_item n=7 part=untouched
CI workflows, pre-push gate, coverage floors and release build: no CI step changed.
@task_item n=8 part=untouched
crate names and the crate set: no crate added, removed or renamed; config.json gained no key.
@task_item n=9 part=untouched
pinned copies, plugin install and plugin contents, Hyprland seam and window features: not reached.
