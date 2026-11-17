@task_prose part=session
The session added a 23rd relied ledger row to viola verify, trust-record: verify writes a trust record for its own temp folder into the config file its child reads, starts a sixth interactive run (Run F, after Run E), checks the child opens with no trust modal, then removes only that record.
verify's step counter moved from [NN/22] to [NN/23] and its summary from 22 pass 0 fail to 23 pass 0 fail; trust-record is the last step line.
When no record could be written, the stamp's row value is the writer's closed code word (one of 14: written, removed, config-absent, config-unreadable, config-link, config-not-regular, config-foreign, config-oversize, config-malformed, config-dir-unusable, home-unknown, key-present, lock-held, lost); no line, stamp or summary carries a path, key or value of the config.
verify now reads the one value CLAUDE_CONFIG_DIR when the strip keeps that name, else the user home; a stop during Run F removes its record and exits 1.
One live reading on the real CLI 2.1.325 read trust-record fail at 7 of 23: it is recorded as read and does not verify the row; the row stands built and proven on the fake agent, unverified on the real CLI, with Session folder trusted by viola written only when it passes.
No colour, glyph, column, hint or exit code was added or changed.
@task_file n=1 path=crates/viola-agent-claude/src/trust.rs
what: new: the pure trust-record shapes of the Claude config file (its key rule, the add and remove of one record, the closed set of 14 trust code words)
@task_file n=2 path=src/trust_record.rs
what: new: the CLI's writer and remover of verify's own trust record in the config file, with its lock, three attempts and bounded reads
@task_file n=3 path=tests/contract_verify_home.rs
what: new: the census check that every test-started verify runs under a test-owned home
@task_file n=4 path=crates/viola-agent-claude/src/lib.rs
what: edited: the Claude agent crate's module list, now exposing the trust module
@task_file n=5 path=crates/viola-agent-claude/src/ledger.rs
what: edited: the relied ledger rows (now 23, trust-record last), each row's check and the stamp merge of rows and measured values
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: edited: unit tests of the ledger row list
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: edited: unit tests of each ledger row's pass check
@task_file n=8 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: edited: unit tests of the stamps.json merge, including the trust writer's word as a row value
@task_file n=9 path=src/main.rs
what: edited: the viola binary's module list
@task_file n=10 path=src/cmd/verify.rs
what: edited: the viola verify verb, its inputs and where it finds the config file the trust record goes to
@task_file n=11 path=src/cmd/verify/typed.rs
what: edited: verify's typed interactive runs and what each run reports to the ledger checks
@task_file n=12 path=src/cmd/verify/typed/runs.rs
what: edited: verify's interactive runs A to F, now with Run F, the trust-record run and its record guard
@task_file n=13 path=src/run/version_gate.rs
what: edited: the version gate that reads the stamps' rows for the running CLI version
@task_file n=14 path=src/bin/viola-fake-agent.rs
what: edited: the test-only fake agent, now reading a named trust config and a blank-start hold
@task_file n=15 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: edited: unit tests of the fake agent's argv options
@task_file n=16 path=tests/support/verify.rs
what: edited: test support that starts verify under a test-owned home and user config
@task_file n=17 path=tests/cli_verify.rs
what: edited: integration tests of viola verify's output, rows and the trust record
@task_file n=18 path=tests/cli_verify/runs.rs
what: edited: integration tests of verify's interactive runs and their step lines
@task_file n=19 path=tests/cli_verify/stamps.rs
what: edited: integration tests of verify's stamps.json writes and the summary line
@task_file n=20 path=tests/contract_ledger_probes.rs
what: edited: the contract test over the ledger rows and the stamped pass/fail summary
@task_file n=21 path=tests/contract_bounded_readers.rs
what: edited: the contract list of bounded file readers, with the trust config readers added
@task_file n=22 path=crates/viola-e2e/src/harness/boot.rs
what: edited: the end-to-end harness boot that stamps verify under the session's own home
@task_file n=23 path=crates/viola-e2e/src/harness/run.rs
what: edited: the end-to-end harness run steps and the expected live verify step lines
@task_file n=24 path=crates/viola-e2e/src/rig/mod.rs
what: edited: the live rig's arms and summary keys, now with the verify arm v
@task_file n=25 path=crates/viola-e2e/src/rig/isolate.rs
what: edited: the live rig's isolation of the private config and its check that no record is left
@task_file n=26 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: edited: the live rig's command entry
@task_file n=27 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited: the live rig's rehearsal tests against the fake agent
@task_item n=1 part=touches
contract viola-verify-output/step-line: the step counter now runs [NN/23] and trust-record is the 23rd and last step line.
@task_item n=2 part=touches
contract viola-verify-output/summary and viola-verify-output/summary-line: the stamped summary now reads 23 pass 0 fail (22 pass 1 fail when the trust record is not honoured); any row cell holding the 22 count is stale.
@task_item n=3 part=touches
decision step-counter-only-progress-pattern and one-progress-pattern: their counter's total moved from 22 to 23 steps; the pattern itself is unchanged.
@task_item n=4 part=touches
contract viola-verify-output/fail-word: the trust-record row prints pass or fail like every row, while its stamp may hold the writer's closed code word when nothing was written.
@task_item n=5 part=touches
contract viola-verify-output/stopped-or-refused and viola-verify-refusals/interrupted: a stop during the new Run F removes verify's own record and exits 1, as every stopped run does.
@task_item n=6 part=touches
contract cli-verification/verified and cli-verification/unverified-cli: a CLI version is now verified only when 23 rows pass, and trust-record stands unverified on the real CLI 2.1.325 (the live reading read fail at 7 of 23).
@task_item n=7 part=touches
contract viola-verify-output/help and name/command/verify: what verify checks now includes a written trust record honoured by a sixth interactive run.
@task_item n=8 part=touches
decision no-internals-in-errors and start-refusals-show-no-path-or-pid: verify's trust-record outcome names no config path, key or value in any line, stamp or summary.
@task_item n=9 part=touches
contract refusal-hints/unverified-cli: the hint for an unverified CLI now stands over a 23-row ledger whose last row is not yet verified live.
@task_item n=10 part=touches
contract viola-verify-refusals/home-tree-check: verify now reads its config location from the one kept CLAUDE_CONFIG_DIR value, else the user home; a config-absent or unusable config is a failed row word, not a refusal.
@task_fact n=1 slug=surface-cli
fact: viola verify's trust-record row writes its own temp folder's record into the config file its child reads and removes only that record after a pass, a fail or a stop; its stamp holds pass, fail, or one of 14 closed trust code words when nothing was written, and no output line names the file, key or value.
@task_fact n=2 slug=brand-identity
fact: the trust-record row stands built and proven on the fake agent and unverified on the real CLI 2.1.325; the display word Session folder trusted by viola is written only when that row passes.
@task_item n=1 part=untouched
Colour palette, colour tokens, colour depth and the colour decision order: no colour, SGR span or accent changed.
@task_item n=2 part=untouched
Typography, spacing, iconography and glyphs: no glyph, separator, column, width or bold rule changed.
@task_item n=3 part=untouched
viola list board and table, the strip and readback box displays, and the fleet console: no board, strip or readback text changed.
@task_item n=4 part=untouched
Send readback and send mirror, wait output, last output, link and unlink output: none of these verbs changed.
@task_item n=5 part=untouched
run and revive start refusals, revive --json codes, exit 21 causes, wrapper fault details and peer reply screening: no refusal, cause or code added.
@task_item n=6 part=untouched
CLI exit codes, stdout and stderr streams, catch sites and --json document rules: verify's exit values and streams are as before.
@task_item n=7 part=untouched
Anti-patterns, self-validation tests and the web surface and rendered page absences: no visual or page surface was added.
