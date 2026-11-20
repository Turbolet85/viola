@task_prose part=session
A viola run start (and revive's start, through the same start path) can now stamp an unstamped claude version itself: with the new config.json key verify_new_cli on, it runs viola verify once as a child before the claude child, and the gate takes the final verdict.
The key is off by default (absent or false) and only a JSON boolean turns it on; any other value reads false with one parse-rejected config-json malformed line.
The version gate now reads the stamps entry as unusable, unattempted or attempted; an attempted entry that did not verify (an earlier red) is not verified again, and an entry short of a relied row is verified again once.
A new attempt lock, ledger/verify.lock (0600), separate from the stamps writer's own lock, lets one start verify while a second concurrent start skips; verify stays the one stamps writer.
The start order gains the verify step after the endpoint bind, and the trust step moves after it, written on the final verdict; a SIGINT or SIGTERM during the wait TERMs the verify child, waits 30 s, then KILLs it and re-raises the signal.
Counts moved: config keys 4 to 5, the diag-line subject enum 7 to 8 (auto-verify added), the controls settings table 8 to 9; no capability claimed, nothing retired, no dependency added.
@task_file n=1 path=src/run/auto_verify.rs
what: new module, the start's automatic verify: reads the verify_new_cli key, decides nothing, earlier-red or verify, takes the attempt lock, spawns and waits on the viola verify child under a stop handler, and rereads the stamps
@task_file n=2 path=src/human/auto_verify.rs
what: new module, the start's five human stderr lines about the automatic verify (verifying, did not verify, earlier red, elsewhere, no stamp with a closed word), one ASCII write each, no colour
@task_file n=3 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger in the pure agent crate; gains attempted, which reads whether a version's stamps entry holds every relied row, beside verified
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger's stamps unit cases; gains the attempted cases
@task_file n=5 path=crates/viola-state/src/stamps.rs
what: the ledger stamps store; gains the attempt lock (try_verify_lock, VerifyLock) on ledger/verify.lock, strict-modes checked first, never the stamps writer's own lock
@task_file n=6 path=src/obs.rs
what: the observability layer holding the known config.json keys list, now five, and the config-json parse-rejected reading
@task_file n=7 path=schemas/config.v1.json
what: the config.json schema; gains the verify_new_cli boolean property
@task_file n=8 path=schemas/diag-line.v1.json
what: the diagnostics line schema; its closed spawn subject enum gains auto-verify
@task_file n=9 path=src/run/version_gate.rs
what: the start's CLI version gate and its strict stamps read; the gate gains the stamps entry (unusable, unattempted, attempted) and a stamps reading used again after the verify
@task_file n=10 path=src/run/mod.rs
what: the run module tree; declares the auto verify module
@task_file n=11 path=src/cmd/run.rs
what: the run and revive start order; runs the automatic verify after the endpoint bind, then the trust record on the final verdict, and drops the server and raises the signal on a stop
@task_file n=12 path=src/human.rs
what: the root bin's human output module; declares the auto verify lines
@task_file n=13 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent; gains a fake-args file beside its executable, its arguments put ahead of argv
@task_file n=14 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's option unit cases; gains the fake-args case
@task_file n=15 path=tests/support/fake.rs
what: test support for the fake agent; writes the fake-args file
@task_file n=16 path=tests/support/home.rs
what: test support for isolated viola homes and booted wrappers; gains a start observed before ready (Starting)
@task_file n=17 path=tests/support/outer_pty.rs
what: test support for the outer PTY a wrapper runs in; reads its child pid
@task_file n=18 path=tests/cli_version_gate.rs
what: the integration cases of the CLI version gate; gains the key-on, key-off, red, two-starts, stop and unstamped cases
@task_file n=19 path=tests/cli_controls_not_disableable.rs
what: the integration check that no setting lifts a safety control; gains the verify_new_cli key cases, the gate still holding
@task_file n=20 path=tests/contract_bounded_readers.rs
what: the bounded-readers boundary list check; gains the config.json verify_new_cli boundary
@task_file n=21 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only live rig in the e2e crate; gains the AV arm, its summary lines and a TERM stop
@task_file n=22 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's isolated scratch home; seeds the verify_new_cli key for the AV arm
@task_file n=23 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the viola-live-rig test binary; takes the AV labels
@task_file n=24 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the rig's rehearsal tests against the fake agent; gains the AV rehearsals
@task_item n=1 part=touches
decision/run-start-order: the start gains the automatic verify step after the endpoint bind, and the trust step moves from before the bind to after the verify, on the final verdict; the claude child is still spawned last.
@task_item n=2 part=touches
decision/verify-child-set and decision/verify-sole-stamps-writer, decision/stamps-single-writer, contract/ledger-stamps/writer: verify stays the one stamps writer, but a start may now start it as a child; the start never writes stamps itself.
@task_item n=3 part=touches
decision/unverified-cli-transport-only and decision/partial-gate-on-unverified-cli: with the key on, a start meeting an unattempted version verifies first, and an automatic green stamp lifts the gate for that start; with the key off, nothing changes.
@task_item n=4 part=touches
decision/version-gate-strict-stamps-read and decision/malformed-stamps-unverified: the gate's strict read now also yields the entry (unusable, unattempted, attempted), and the strict read runs again under the attempt lock and after the verify.
@task_item n=5 part=touches
decision/capability-ledger-gate, decision/version-verified-all-relied-rows, contract/refusal-reason/unverified-cli, contract/cli-exit-codes/12: the rule and the code stay as they are; an automatic stamp is verify's own and counts only when every relied row counts.
@task_item n=6 part=touches
contract/instance-snapshot/cli-verified: the snapshot records the final verdict, taken after the automatic verify.
@task_item n=7 part=touches
decision/config-keys-match-schema and contract/config-json/* (v, unknown-keys, the per-key items): config.json gains a fifth known key, verify_new_cli; where a cell holds the count of known keys (four) it is now five.
@task_item n=8 part=touches
decision/config-bad-value-falls-back and decision/config-falls-back-to-defaults: a non-boolean verify_new_cli falls back to false with one parse-rejected config-json malformed line; a v other than 1 reads false too.
@task_item n=9 part=touches
decision/test-seams-not-configuration, decision/no-environment-configuration, decision/config-precedence: the new key turns a behaviour on only from config.json; no environment variable or flag turns it on.
@task_item n=10 part=touches
decision/run-terminal-child-only and decision/cli-verbs-human-stderr: before the child, a start may now write one of five fixed ASCII lines to stderr, and verify's own output is inherited; nothing under --json.
@task_item n=11 part=touches
contract/diag-line/format, contract/diagnostics-lines/process-log-line and the verify child spawn subject rows: the closed spawn subject set gains auto-verify (seven to eight values), a process-start and process-exit pair with child_exit_status and duration_ms.
@task_item n=12 part=touches
decision/on-disk-file-naming, decision/separate-lock-files, decision/stamps-writer-checks-before-and-under-lock, decision/strict-read-rule: ledger/verify.lock is a new on-disk name and the attempt lock, checked by strict modes first, never the stamps writer's own lock.
@task_item n=13 part=touches
decision/verify-probe-dirs-drop-guarded and decision/verify-removes-only-own-dirs: a stop of the start now reaches its verify child (TERM, 30 s grace, KILL), and the child is tied to the start by a parent-death TERM, so its probe folders still go.
@task_item n=14 part=touches
decision/child-env-claude-strip, decision/claude-env-stripped-from-child, contract/child-environment/viola-name, viola-dir and viola-bin: the start's verify child gets the same strip plan and none of VIOLA_NAME, VIOLA_DIR, VIOLA_BIN.
@task_item n=15 part=touches
decision/no-shell-out, decision/run-pins-checked-bytes, decision/start-pins-the-bytes-it-checked: the verify child is a direct spawn of the start's pinned copy with a closed argv (--home, verify, --, the resolved program).
@task_item n=16 part=touches
decision/real-claude-local-only, decision/no-live-claude-in-ci, decision/real-cli-probe-local-only, decision/ci-against-fake-agent: the key is off in every CI home; the live AV arm runs the real claude only locally.
@task_item n=17 part=touches
decision/verify-transcripts-accepted-residual and decision/verify-never-types-into-dialog: verify's residuals now arise with no human present, once per new version or relied set, key on.
@task_item n=18 part=touches
decision/bounded-readers: a new reader boundary, config.json verify_new_cli, read under the frame cap with default depth.
@task_item n=19 part=touches
decision/fake-agent-test-seams and name/binary/viola-fake-agent: the fake agent gains a fake-args file beside its executable as a test seam; name/binary/viola-live-rig gains an AV arm.
@task_item n=20 part=touches
decision/socket-dir-strict-check and decision/strict-check-before-log-open: the start's home-tree strict check precedes verify's own check.
@task_fact n=1 slug=established-decisions
fact: With config.json verify_new_cli on, a viola run or revive start that meets a claude version with no attempted stamps entry runs viola verify once as a child after the endpoint bind and before the trust record and the claude child, and gates on the final verdict; the key is off by default.
@task_fact n=2 slug=established-decisions
fact: An automatic verify runs only for a version whose stamps entry is short of a relied row; an entry holding every relied row that did not verify (an earlier red) is never verified again automatically, so a red version does not loop.
@task_fact n=3 slug=established-decisions
fact: A stop (SIGINT or SIGTERM) during the start's verify TERMs the verify child, waits 30 s, then KILLs it, writes the exit line and re-raises the signal; the child is tied to the start by a parent-death SIGTERM, and the start makes no snapshot or session.
@task_fact n=4 slug=occupied-resources
fact: ledger/verify.lock (0600) is the automatic verify's attempt lock, a named exception in on-disk file naming, separate from stamps.json.lock; a start that finds it held runs no verify and starts unverified.
@task_fact n=5 slug=occupied-resources
fact: config.json key verify_new_cli: a JSON boolean, default false; any other value reads false with one parse-rejected config-json malformed line; v must be 1.
@task_fact n=6 slug=occupied-resources
fact: The diagnostics spawn subject auto-verify: one process-start and process-exit pair per automatic verify, with child_exit_status and duration_ms, on the run start's span.
@task_fact n=7 slug=occupied-resources
fact: The start's verify child runs the pinned copy with --home, verify, -- and the resolved program, in the start's cwd, stdin null, stdout and stderr inherited (null under revive --json), with the start's strip plan applied and VIOLA_NAME, VIOLA_DIR and VIOLA_BIN removed.
@task_fact n=8 slug=cross-cutting-patterns
fact: Run's terminal-child-only rule has one exception before the child: the start's five automatic-verify lines (verifying, did not verify, earlier red, elsewhere, no stamp with spawn-failed, unstamped or lock-refused) on stderr, one ASCII write each, no SGR, only the version and a closed word, none under --json, plus verify's own inherited output.
@task_fact n=9 slug=conventions
fact: The fake agent reads a fake-args file beside its executable, one argument per line under the frame cap, put ahead of argv, argv's own value winning; a test-only seam.
@task_item n=1 part=untouched
The MCP server and its tools, results and stdin limits: no MCP surface changed.
@task_item n=2 part=untouched
The wrapper channel: its methods, frames, fault codes, refusal reasons other than unverified-cli, and the send, wait, last and answer verbs.
@task_item n=3 part=untouched
Hooks: the hook tiers, the hook-to-kind map, hook exit and body, dialog hold and dialog answering.
@task_item n=4 part=untouched
The wheel, pause, release and link rules and the human-typing take: no wheel or link behaviour changed.
@task_item n=5 part=untouched
The normalised event kinds, event data and the events log: no event kind added; auto-verify is a diagnostics subject, not an event.
@task_item n=6 part=untouched
The board and list, session liveness and the agents listing.
@task_item n=7 part=untouched
The Hyprland seam, the PTY seam and the vt100 readiness gate.
@task_item n=8 part=untouched
CI workflows, release, fuzz, supply-chain and lint policy, MSRV and the toolchain pin: no dependency, workflow or gate threshold changed.
@task_item n=9 part=untouched
The plugin, its install and its embedded MCP-only content.
