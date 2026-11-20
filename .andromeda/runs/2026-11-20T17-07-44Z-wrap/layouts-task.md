@task_prose part=session
A viola run or viola revive start that meets a claude version with no stamp now runs viola verify once by itself, before the child, when the new config.json key verify_new_cli is true; the key is off by default.
The start writes one of five fixed stderr lines about that verify (verifying, did not verify, earlier red, verifying elsewhere, no stamp with a closed word), plain ASCII with no colour, holding only the parsed version and a closed word, and none under revive --json.
The verify child's own step lines and summary pass through to the terminal during the start (they are discarded under revive --json); a SIGINT or SIGTERM during it stops the verify and ends the start by that signal, with no session.
The start order gained the verify step after the socket bind, and the trust record is now written after it, from the final verdict; the instance snapshot's cli_verified reports that final verdict.
A green automatic stamp lifts the unverified-CLI gate the same way a hand-run verify does; exit code 12 and its rule are unchanged. A red one is recorded once, and later starts print the earlier-red line and run no verify.
Counts moved: the known config keys 4 to 5, the diag-line spawn subjects 7 to 8 (auto-verify added), and the controls table 8 to 9 entries; no row is retired.
@task_file n=1 path=src/run/auto_verify.rs
what: the start's automatic verify step: reads the verify_new_cli key, decides whether to verify, runs viola verify as a child under a lock, handles the stop signal, and returns the final gate verdict
@task_file n=2 path=src/human/auto_verify.rs
what: the human output of the start's automatic verify: the five fixed stderr lines (verifying, did not verify, earlier red, elsewhere, no stamp with spawn-failed / unstamped / lock-refused), one write each
@task_file n=3 path=src/cmd/run.rs
what: the viola run and viola revive start sequence: preflight, socket bind, the automatic verify, the trust record, then the child
@task_file n=4 path=src/run/version_gate.rs
what: the unverified-CLI version gate a start reads: the claude version probe, the strict stamps read, and the verified verdict
@task_file n=5 path=src/run/mod.rs
what: the run module list that the start's components hang from
@task_file n=6 path=src/human.rs
what: the human-output module list for CLI verbs, now including the automatic verify lines
@task_file n=7 path=src/obs.rs
what: diagnostics and the config.json reader: the list of known config keys and the parse-rejected lines
@task_file n=8 path=schemas/config.v1.json
what: the config.json schema, listing its known keys
@task_file n=9 path=schemas/diag-line.v1.json
what: the diagnostic line schema, including the closed list of spawn subjects
@task_file n=10 path=crates/viola-agent-claude/src/ledger.rs
what: the relied-row ledger and its stamp verdict readers (verified, and now attempted)
@task_file n=11 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit tests of the ledger's stamp readers
@task_file n=12 path=crates/viola-state/src/stamps.rs
what: the stamps state file under the viola home's ledger directory, its single writer lock, and the new attempt lock ledger/verify.lock
@task_file n=13 path=src/bin/viola-fake-agent.rs
what: the test-only fake claude agent, now taking extra arguments from a fake-args file beside it
@task_file n=14 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: unit tests of the fake agent's option handling
@task_file n=15 path=tests/support/fake.rs
what: test support that sets up the fake agent
@task_file n=16 path=tests/support/home.rs
what: test support for a test viola home and a started wrapper, now with a start whose child can be watched before it is ready
@task_file n=17 path=tests/support/outer_pty.rs
what: test support for the outer terminal a test start runs in
@task_file n=18 path=tests/cli_version_gate.rs
what: integration tests of the unverified-CLI gate and the start's automatic verify, its lines and its stop
@task_file n=19 path=tests/cli_controls_not_disableable.rs
what: integration tests that no setting disables a control, now including the verify_new_cli key
@task_file n=20 path=tests/contract_bounded_readers.rs
what: the bounded-readers contract test, listing each bounded input reader
@task_file n=21 path=crates/viola-e2e/src/rig/mod.rs
what: the live end-to-end rig: its arms, sessions, stop modes and summary lines, now with the AV arm
@task_file n=22 path=crates/viola-e2e/src/rig/isolate.rs
what: the rig's isolated scratch viola home and its config seed
@task_file n=23 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig command line and its labels
@task_file n=24 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: rehearsals of the live rig's arms against the fake agent
@task_item n=1 part=touches
The passed start output of viola run: one automatic verify line and the verify child's own output may now come before the child takes the terminal (the start-output contract for run).
@task_item n=2 part=touches
The passed start output of viola revive: the same lines on a revive start, and silence under revive --json.
@task_item n=3 part=touches
The viola run and viola revive verb contracts: a start may now run viola verify itself when the key is on.
@task_item n=4 part=touches
Exit code 12 (the unverified-CLI refusal): an automatic green stamp now lifts it without a hand-run verify; the code and rule are unchanged.
@task_item n=5 part=touches
The viola verify output rows (step line, summary, exit 0, exit 1, failing row, plain ASCII): that output now also appears inside a start, inherited to the terminal, and discarded under revive --json.
@task_item n=6 part=touches
The viola verify stop output and the verify-stopped refusal: a SIGINT or SIGTERM during the start's verify now reaches it, and the start then ends by the signal with no session.
@task_item n=7 part=touches
Results on stdout, the rest on stderr: the five new automatic verify lines go to stderr.
@task_item n=8 part=touches
Plain output, the TTY expression level and no-colour rules: the new lines are ASCII, one write each, with no SGR in every mode.
@task_item n=9 part=touches
No spinner, no progress glyphs and the single progress pattern: the new verifying line is a fixed one-line notice, not progress.
@task_item n=10 part=touches
Hints never quote text or name paths: the new lines carry only the parsed version and a closed word.
@task_item n=11 part=touches
No interactive prompt: the automatic verify is turned on by a config key, never asked about at the start.
@task_item n=12 part=touches
The run and revive start preflight order and the home-tree refusal: the home-tree check still comes first; the verify step now comes after the socket bind, with the trust record after it.
@task_item n=13 part=touches
The shared run/revive --json start refusals: under revive --json the automatic verify writes nothing to the terminal.
@task_item n=14 part=touches
The viola list board's CLI column and its unknown state: the snapshot's cli_verified it reads is now the start's final verdict, after any automatic verify.
@task_fact n=1 slug=surface-cli
fact: With config.json verify_new_cli true (off by default), a viola run or viola revive start that meets an unstamped claude version runs viola verify once before the child; with the key off, no verify runs and nothing is printed.
@task_fact n=2 slug=surface-cli
fact: A start's automatic verify writes exactly one of five fixed stderr lines (verifying; did not verify; earlier red, no verify; verifying elsewhere, not waited for; no stamp with spawn-failed, unstamped or lock-refused), each ASCII, one write, no SGR, holding only the version and a closed word, and none under revive --json.
@task_fact n=3 slug=surface-cli
fact: The start's verify output goes straight to the terminal before the child takes it (discarded under revive --json), and a SIGINT or SIGTERM during it stops the verify and ends the start by that signal with no session.
@task_fact n=4 slug=surface-cli
fact: A red automatic verify is recorded once per version; later starts print the earlier-red line and run no verify until the stamp is cleared by a hand-run viola verify.
@task_item n=1 part=untouched
viola send output, its readback mirror, word column, refusals and --json document: the session does not change sending.
@task_item n=2 part=untouched
viola wait output, viola answer output and the dialog words: unchanged.
@task_item n=3 part=untouched
viola link / unlink output and the wheel verb output (pause, release): unchanged.
@task_item n=4 part=untouched
The viola mcp tools and their results, and the hidden mcp verb: unchanged.
@task_item n=5 part=untouched
CLI token rendering (colour, spacing, radius, motion, roles) and the BAY line: no token or colour added.
@task_item n=6 part=untouched
viola --help, per-verb usage and the verb groups: no verb or flag added; the switch is a config key.
@task_item n=7 part=untouched
viola revive --list, its rows and its revive-specific refusals (already live, no session, cwd missing, stale heartbeat): unchanged.
@task_item n=8 part=untouched
The exit-21 frame-verb hints and instance-unreachable details, and viola last output: unchanged.
@task_item n=9 part=untouched
The absences of a web surface and of deep links: still true.
