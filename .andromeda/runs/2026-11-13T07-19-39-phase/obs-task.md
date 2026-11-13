@task_prose part=chunk
Builds one shared, separately tested source-masking scanner in test support (masks comments, and by a flag string literals, out of Rust source) and swaps it in for the three hand scanners of the error-body, bounded-reader and OS-gated-census contract tests, their verdicts unchanged.
Merges two duplicated product-function pairs (in the hook command, and between run links and the run snapshot) into one function each, behaviour unchanged; deletes or wires the dead strict-modes presence check; deletes the dead non-unix constants in the e2e run harness.
Splits three over-800-line files (the ledger test cases, the fake agent binary, the verify CLI tests) to at most 800 lines each, moved bodies byte-identical, every test name kept.
Closes named mutant survivors by new tests (the MCP tap, the e2e free-bytes probe, the state fs/pin error-kind guards, the MCP WARN parse-rejected line and stdin buffer), never by a mutation run; adds a viola-made (0700) test-home helper, a remove-the-guard control helper and one named temp root.
Removes the cookie class from the e2e secret scan while it still catches every other class; re-measures every triage coordinate at HEAD; no mutation run and no live claude start.
@task_file n=1 path=tests/contract_error_bodies.rs mark=edits
what: the clean error-bodies contract test — edits: its blanked comment scanner, replaced by the shared scanner
@task_file n=2 path=tests/contract_bounded_readers.rs mark=edits
what: the bounded-readers contract test — edits: its code_only scanner, replaced by the shared scanner
@task_file n=3 path=tests/contract_os_gated_tests.rs mark=edits
what: the OS-gated test census check — edits: its census source scanner, replaced by the shared scanner
@task_file n=4 path=tests/support/mod.rs mark=edits
what: the shared integration-test support module — edits: its module list, gaining the shared source-masking scanner and the test helpers
@task_file n=5 path=tests/support/home.rs mark=edits
what: the integration-test home helpers under the one test-home root — edits: one helper making a test home the way viola makes one (0700)
@task_file n=6 path=tests/cli_verify.rs mark=edits
what: the viola verify CLI integration tests — split to at most 800 lines a file, every test name kept
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases.rs mark=edits
what: the capability-ledger parser test cases — split to at most 800 lines a file, bodies byte-identical
@task_file n=8 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent binary CI verify runs against — split to at most 800 lines a file, behaviour unchanged
@task_file n=9 path=src/cmd/hook.rs mark=edits
what: the viola hook command (hook events, statusline arm, fail-open) — edits: the duplicated function pair, merged into one
@task_file n=10 path=src/run/links.rs mark=edits
what: the run's link and unlink handling — edits: the function duplicated with the run snapshot, merged into one
@task_file n=11 path=src/run/snapshot.rs mark=edits
what: the run's snapshot writing and replayed-snapshot reading — edits: the function duplicated with run links, merged into one
@task_file n=12 path=crates/viola-state/src/strict.rs mark=edits
what: the home strict-modes check of the state crate — edits: check_present, the dead presence check, deleted or wired and tested
@task_file n=13 path=.claude/docs/services/viola-state.md mark=edits
what: the viola-state service doc — edits: its one mention of check_present
@task_file n=14 path=crates/viola-state/src/fs.rs mark=edits
what: the state crate filesystem refusals and error-kind guards — edits: tests for the error-kind guard survivor
@task_file n=15 path=crates/viola-state/src/pin.rs mark=edits
what: the pinned-copy placement and its error-kind guard — edits: tests for the error-kind guard survivor
@task_file n=16 path=crates/viola-mcp/src/lib.rs mark=edits
what: the viola mcp stdio server and its WARN parse-rejected line — edits: a test asserting that line
@task_file n=17 path=crates/viola-mcp/src/stdin.rs mark=edits
what: the viola mcp stdin line reader bounded by MAX_FRAME — edits: a test with a buffer smaller than the pending line; the CHUNK constant marked equivalent
@task_file n=18 path=crates/viola-e2e/src/bin/viola-mcp-tap.rs mark=edits
what: the test-only MCP tap binary of the e2e harness — edits: its tests, or its recorded exclusion from the mutation scope
@task_file n=19 path=crates/viola-e2e/src/harness/mod.rs mark=edits
what: the e2e harness core and its temp root — edits: fn free_bytes tests; the host's quota'd temp dir named in one place
@task_file n=20 path=crates/viola-e2e/src/harness/run.rs mark=edits
what: the e2e harness run driver — edits: the three dead #[cfg(not(unix))] constants, removed
@task_file n=21 path=crates/viola-e2e/src/harness/secret_scan.rs mark=edits
what: the end-to-end secret scan of uploads and its pattern classes — edits: the cookie class (generic cookie header and the GUI viola_<port>= pair) removed, with its tests
@task_item n=1 part=needs
The secret scan's pattern classes and its hit report's pattern-class field: the cookie class leaves, every other class still hits, the hit report still carries no matched bytes.
@task_item n=2 part=needs
The secret scan's scope and its gating of CI uploads: the scan must still always run and still block upload on a hit after the cookie class goes.
@task_item n=3 part=needs
The rule that the Cookie header is never logged: with the scan's cookie class gone, the no-Cookie rules leave with their tests; the plan's own row change is the wrap's amendment.
@task_item n=4 part=needs
The OS-gated test census: its verdict over cfg-gated tests must stay unchanged when its scanner is swapped, and the removed non-unix constants on a Linux-only target must not leave it miscounting.
@task_item n=5 part=needs
The clean error-bodies contract and the fixed, interpolate-nothing error messages it checks: the scanner swap must keep its verdict.
@task_item n=6 part=needs
The bounded readers, including viola mcp stdin bounded by MAX_FRAME and the oversize-line parse-rejected path: the scanner swap keeps the bounded-readers verdict and the new stdin test exercises a buffer smaller than the pending line.
@task_item n=7 part=needs
The viola mcp parse-rejected WARN line (parser mcp-stdin, detail oversize) and its fields: the new test asserts it as emitted.
@task_item n=8 part=needs
Home strict-modes checks and strict-modes-failed refusals: deleting or wiring the dead presence check must not change any strict-modes refusal, its exit codes, its detail word or its no-line-before-log-open rule.
@task_item n=9 part=needs
Diagnostics files and directories made owner-only: the test-home helper must make homes the way viola does (0700) so tests reddened on 0755 homes read the real mode rules.
@task_item n=10 part=needs
Integration-test homes under one root, kept until the gates run and under the scanned root: the shared home helper and the one named temp root must stay inside that root so the panic, schema and secret-scan gates still read them.
@task_item n=11 part=needs
The hook command's fail-open, silent-without-instance, exit-0 and stdout-decision-body-only behaviour, and its hook-invoked / hook-decision lines: the merge in the hook command keeps every one unchanged.
@task_item n=12 part=needs
Link and unlink handling (events-log link records, link names never in a process log, link methods untraced) and the snapshot writing and replayed-snapshot reading: the links/snapshot merge keeps them unchanged.
@task_item n=13 part=needs
The state crate's filesystem refusals and pinned copy placed never replaced: the new tests on the fs and pin error-kind guards act through those refusals without changing them.
@task_item n=14 part=needs
The fake agent binary that CI verify runs against, and the viola verify stdout, stderr and exit-code contracts its tests assert: the splits keep every verdict and test name.
@task_item n=15 part=needs
Obs code mutation-tested like product code and a surviving obs mutant counting as a failure: the named survivors are closed by tests or a recorded mutants-scope exclusion, with no mutation run in this chunk and mutation outputs never uploaded.
@task_item n=16 part=needs
The test-only MCP tap of the e2e harness: either tests that exercise it or its recorded exclusion from the mutation scope.
@task_item n=17 part=needs
The e2e harness free-bytes probe, its quota'd temp root and a refused recursive rm on the host: the temp root named in one place and the free-bytes survivors closed by tests.
@task_item n=18 part=needs
The remove-the-guard control: one project script or test helper that removes a guard and shows the guarding test go red, replacing the per-chunk hand rebuild.
@task_item n=19 part=needs
The capability-ledger parser and its parse-rejected ledger-stamps line, through the ledger test cases being split: the cases' count unchanged.
@task_item n=20 part=needs
Linux-only OS target: the dead non-unix constants go because the tree builds for Linux only.
@task_standing n=1
convention: each new or moved integration test file takes its home from the one shared test-home root and keeps that home until the panic, schema and secret-scan gates have run
home: ci-integration · 9-ci-integration
@task_standing n=2
convention: each new test that drives a viola process schema-checks the diagnostic lines it emits against the line schema
home: ci-integration · 9-ci-integration
@task_standing n=3
convention: each new test, fixture or harness output that may be uploaded uses synthetic inputs only and passes the secret scan before upload
home: pii-scrubbing-compliance · 8-pii-scrubbing-compliance
@task_standing n=4
convention: each new test waits on a log line by reading for it, never by sleeping, and asserts budgets by exit code
home: slo-invariants-telemetry-budgets · 10-slo-invariants-telemetry-budgets
@task_item n=1 part=not_needed
The dialog-hold ceiling and the config dialog_hold_s value: carried by 0.2.0 elsewhere, not taken up by this chunk.
@task_item n=2 part=not_needed
Span declarations, span chains and span attributes of start, send, wait, dialog, wheel and budget paths: the chunk declares no span and changes no traced path.
@task_item n=3 part=not_needed
Metric coverage: counters, gauges, histograms and quantiles derived from logs; the chunk adds no signal to derive.
@task_item n=4 part=not_needed
Exporters, error reporters and their absences (OTel SDK, OTLP receivers, Sentry, future reporter transport and opt-in): nothing here exports or reports.
@task_item n=5 part=not_needed
The budget governor, confirmed send, wait and last, and the wheel's pause and release paths with their log fields and refusal words: no item touches them.
@task_item n=6 part=not_needed
Panic capture and the panic outcome by role, the panic hook and the catch sites: no item changes panic handling.
@task_item n=7 part=not_needed
Service identity, process-start fields, config.json keys and home resolution order: the chunk adds no key and changes no identity.
@task_item n=8 part=not_needed
Heartbeat file, liveness and staleness thresholds and the list liveness output: untouched by this chunk.
@task_item n=9 part=not_needed
The agent-run logs and status outputs and the harness logs command's merge, filters and torn-line handling: the chunk changes no reader of logs.
@task_item n=10 part=not_needed
Correlation and trace-context propagation (corr, conn, srv_conn, rpc_id) on channel, dialog and send lines: no item alters correlation.
@task_item n=11 part=not_needed
The rows landing's route-owed rule lines: they ride the next route entry, not this one.
