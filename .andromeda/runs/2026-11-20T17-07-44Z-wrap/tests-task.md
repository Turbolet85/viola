@task_prose part=session
A viola run (and the start of viola revive) meeting a claude version with no stamp now runs viola verify itself, once, when the new config.json key verify_new_cli is true; the key is off by default and in every CI and test home unless a test sets it.
The start's verify runs after the endpoint bind and before the child; the trust record is now written on the final verdict after it, and the start reads the stamps again for that final cli_verified.
One attempt lock (ledger/verify.lock, 0600, not the stamps writer's lock) lets only one of two concurrent starts verify; the other starts unverified with its own line; an entry already attempted and unverified runs no verify again (no loop).
A SIGINT or SIGTERM during the start's verify TERMs the verify child, waits a 30 s grace, then KILLs it, writes its exit line and ends the start by the signal with no stamp, no snapshot and no session; the verify child is tied to the start by parent-death TERM.
Five fixed ASCII stderr lines (L1 verifying, L2 did not verify, L3 earlier red, L4 verifying elsewhere, L5 no stamp with a closed word) are written one write each, none under revive --json; a new diag subject auto-verify carries one process-start/process-exit pair.
Counts moved: the known config keys 4 to 5, the diag-line subject enum 7 to 8, the controls table of flags, variables and keys 8 to 9; the fake agent gains a fake-args file beside its executable; the live rig gains the AV arm (labels AV, AV-2, rehearse word av); CI green 7/7 with 3853 tests.
@task_file n=1 path=src/run/auto_verify.rs
what: new module — the start's automatic verify: reads the verify_new_cli key, decides verify / earlier-red / nothing, takes the attempt lock, spawns the tied verify child, the stop handler and its grace
@task_file n=2 path=src/human/auto_verify.rs
what: new — the start's five fixed human stderr lines L1 to L5 and the closed no-stamp words spawn-failed, unstamped, lock-refused
@task_file n=3 path=crates/viola-agent-claude/src/ledger.rs
what: the ledger of relied rows and the stamps verdict; gains attempted, an entry holding every relied row id whatever its word
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit cases of the stamps reading of the ledger; edited with the attempted cases
@task_file n=5 path=crates/viola-state/src/stamps.rs
what: the on-disk stamps file and its lock; gains the attempt lock ledger/verify.lock taken after the strict-modes check
@task_file n=6 path=src/obs.rs
what: the diagnostics-level config.json read and its list of known config keys (now five)
@task_file n=7 path=schemas/config.v1.json
what: the config.json schema; gains the boolean property verify_new_cli
@task_file n=8 path=schemas/diag-line.v1.json
what: the diagnostics line schema; its closed spawn subject enum gains auto-verify
@task_file n=9 path=src/run/version_gate.rs
what: the start's unverified CLI version gate and its strict stamps read; gains the entry reading (unusable, unattempted, attempted)
@task_file n=10 path=src/run/mod.rs
what: the run module list; edited to add the automatic verify module
@task_file n=11 path=src/cmd/run.rs
what: the viola run start order; the automatic verify step after the bind, the trust step moved after it onto the final verdict
@task_file n=12 path=src/human.rs
what: the human-mode output module list; edited to add the automatic verify lines
@task_file n=13 path=src/bin/viola-fake-agent.rs
what: the fake agent standing in for claude; now reads a fake-args file beside its executable ahead of argv, argv winning
@task_file n=14 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: unit cases of the fake agent's options; edited with the fake-args case
@task_file n=15 path=tests/support/fake.rs
what: shared test helper planting fake agent copies; gains writing the fake-args file
@task_file n=16 path=tests/support/home.rs
what: shared test home and wrapper boot helpers; gains a start as a test-owned user that is observed before it is ready
@task_file n=17 path=tests/support/outer_pty.rs
what: shared outer PTY test driver; gains reading its own child's pid
@task_file n=18 path=tests/cli_version_gate.rs
what: integration suite of the unverified CLI version gate; edited with the key-on start cases (verify once, key off, red then none, two starts, stop during verify, verify's own stop)
@task_file n=19 path=tests/cli_controls_not_disableable.rs
what: integration suite that no setting disables a security control; edited with the verify_new_cli key not lifting the gate
@task_file n=20 path=tests/contract_bounded_readers.rs
what: the bounded readers contract list; gains the config.json verify_new_cli reader
@task_file n=21 path=crates/viola-e2e/src/rig/mod.rs
what: the standing live rig; gains the AV arm, its summary lines and a TERM stop of the session
@task_file n=22 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's isolated scratch home; gains keying verify_new_cli in the scratch home config
@task_file n=23 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig binary entry; edited for the AV labels
@task_file n=24 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the rig rehearsal suite over the fake agent; edited with the av rehearsals of a green and a red stamp
@task_item n=1 part=touches
The stamps-writer rows (stamps written only by verify, verify the only stamp writer, single writers of snapshot and stamps, stamps only from verify against the fake agent): verify stays the one writer, but a viola run start may now START it with the key on.
@task_item n=2 part=touches
The unverified CLI gate rows (unverified CLI version gate, degrades to send only, gates answer only, exit 12 unverified-cli, version gate needs every relied row): rule and code unchanged, but an automatic green stamp at start now lifts the gate for that start.
@task_item n=3 part=touches
The real-claude-local-only rows (real claude only locally, CI never touches the real CLI, no real claude in CI, live verify never in CI): the start's verify reaches the real CLI only with the key on, off by default in every CI home.
@task_item n=4 part=touches
The controls rows (no switch disables a security control, controls not disableable): a config key now turns a behaviour ON; the controls table of every flag, VIOLA_ variable and config key moves 8 to 9.
@task_item n=5 part=touches
The bad config value falls back row and config tolerant parsing: verify_new_cli takes a JSON boolean only, any other value reads false with one parse-rejected config-json malformed line; a v 2 document gets the diagnostics-level read's one line.
@task_item n=6 part=touches
The bounded readers rows (every external reader bounded, bounded readers keep the frame cap): a new bounded reader, the verify_new_cli read through the frame cap.
@task_item n=7 part=touches
The start order rows (run start events order, run checks before the child, child spawn after events, home-tree strict-modes check at start): one step added after the bind, the trust step moved after it, the start's home-tree check still before verify's own.
@task_item n=8 part=touches
The strict stamps read at start and the strict-modes refusal of viola run: the strict read now also runs under the attempt lock and after the verify, for the final verdict.
@task_item n=9 part=touches
The cli_verified status field and snapshot value: they now carry the final verdict after the start's verify.
@task_item n=10 part=touches
The verify stop row (viola verify interrupted by SIGINT or SIGTERM) and verify probe-dir cleanup: the stop now reaches the start's verify; no verify probe dir is left.
@task_item n=11 part=touches
The tied verify kill witness and the no untied viola spawn / every test verify start tied to a test-owned home rows: a start now spawns verify tied by parent-death TERM; its verify runs in the start's cwd and home.
@task_item n=12 part=touches
The verify_window_ 80 s kill class and test naming: the new start cases and rig rehearsals are named verify_window_ and fall in that class.
@task_item n=13 part=touches
The fake agent configured by argv alone rows (fake agent argv only, fake-agent command line modes and options): an exception, the fake-args file beside its executable is read ahead of argv, argv winning.
@task_item n=14 part=touches
The stamps refuse a widened ledger row: the attempt lock also refuses a widened ledger and leaves it as found.
@task_item n=15 part=touches
The diagnostics schema conformance row and the process-start / process-exit corr rows: the closed spawn subject enum moves 7 to 8 with auto-verify.
@task_item n=16 part=touches
The human output pinned by literals and no SGR without a terminal rows: five new fixed ASCII stderr lines, one write each, no SGR, pinned by literal.
@task_item n=17 part=touches
The standing live rig and live reading rows (one standing committed live rig, live reading recorded as read, never as verification): the rig gains the AV arm whose stamp verdict is a reading.
@task_item n=18 part=touches
The state files never hand-written row: the start writes ledger/verify.lock, a new on-disk file, and a start's verify now writes stamps.
@task_item n=19 part=touches
The human is never blocked row: the start's verify window precedes the child, so the human's keystrokes reach no child during it.
@task_item n=20 part=touches
The decision bodies only for a driver on a stamped CLI row: a stamp written by the start's verify now enables decision bodies in that same start.
@task_item n=21 part=touches
The harness boot makes the home, stamps it and starts wrappers / one verify stamp per build rows: the harness's homes leave the key off, so their stamp still comes from the boot's own verify.
@task_fact n=1 slug=integration-test-strategy
fact: config.json key verify_new_cli, JSON boolean default false, makes a viola run start meeting an unstamped claude version run viola verify once before the child; any other value reads false with one parse-rejected config-json malformed line.
@task_fact n=2 slug=integration-test-strategy
fact: Two concurrent starts run one verify: the attempt lock ledger/verify.lock (0600, taken non-blocking after the strict-modes check, never the stamps lock) makes the other start read verifying-elsewhere and start unverified.
@task_fact n=3 slug=integration-test-strategy
fact: A version whose stamp entry holds every relied row id yet is unverified runs no verify again at start (the earlier-red line); an entry short of one relied id verifies again once.
@task_fact n=4 slug=integration-test-strategy
fact: A SIGINT or SIGTERM during the start's verify TERMs the verify child, waits 30 s, then KILLs, writes its process-exit line and ends the start by that signal: no stamp, no snapshot, no session receipt, no verify probe dir.
@task_fact n=5 slug=6-e2e-test-strategy
fact: The start's verify child runs with stdin null, the start's environment strip and VIOLA_NAME, VIOLA_DIR, VIOLA_BIN removed, tied by parent-death TERM, its output silenced under revive --json.
@task_fact n=6 slug=integration-test-strategy
fact: The trust record of a start is written on the final verdict after the start's verify, which runs after the endpoint bind.
@task_fact n=7 slug=log-format
fact: The diag spawn subject auto-verify carries one process-start/process-exit pair with child_exit_status and duration_ms per start's verify.
@task_fact n=8 slug=test-data-fixtures
fact: The fake agent reads a fake-args file beside its executable, one argument per line within MAX_FRAME, ahead of argv, argv's own value winning: the named exception to argv-only configuration.
@task_fact n=9 slug=e2e-test-strategy
fact: The live rig's AV arm (labels AV and AV-2, rehearse word av) keys verify_new_cli in its scratch home alone and reads the start's verify pair, the stamp ids and verdict as a reading.
@task_fact n=10 slug=unit-test-strategy
fact: The start's five human lines L1 to L5 are fixed ASCII, one stderr write each, no SGR, carrying the parsed version and a closed word only, and none is printed under revive --json.
@task_item n=1 part=untouched
MCP tools, their schemas, replies and invalid-params fields: the session adds no MCP surface.
@task_item n=2 part=untouched
viola send, wait, last, release and the wheel rules: unchanged.
@task_item n=3 part=untouched
Hook exec-form entry points, hook deadlines and dialog decision-body shapes: unchanged.
@task_item n=4 part=untouched
Wrapper channel JSON-RPC errors, link and unlink methods and instance-unreachable causes: unchanged.
@task_item n=5 part=untouched
Revive refusal codes, run and revive exit-1 causes and viola list board and JSON: no code or word added.
@task_item n=6 part=untouched
CI workflows, supply-chain, coverage, mutation, fuzz and perf gates and their thresholds: unchanged.
@task_item n=7 part=untouched
The agent-run harness commands boot, status, cleanup, run, pre-push, gate, secret-scan and their documents and exit codes: unchanged.
@task_item n=8 part=untouched
Recorded claude fixtures, the fixture scrub and hygiene walk and the verify recording refusal: unchanged.
