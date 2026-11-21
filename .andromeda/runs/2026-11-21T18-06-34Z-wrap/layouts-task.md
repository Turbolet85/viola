@task_prose part=session
The session added a 29th relied capability-ledger row, cross-session-prefix, reading pass or fail only: viola verify now counts 29 relied rows, its step lines run [NN/29] and its clean summary reads 29 pass 0 fail.
viola verify's Run B gains a cross-session turn: a print-mode sender child, verify-sender, sends to the Run B instance by a uds socket address verify composes from the runtime dir and the child's pid, and the row reads whether the arrival carried the CLI's own cross-session prefix.
viola verify now reads XDG_RUNTIME_DIR once, held in memory; its value and the composed address enter no output line, log line, stamp, error or step line; without a runtime dir the turn reads not-run and the row fails.
The hidden viola hook verb gained a hidden --names-only flag, valid only with --capture, filing only closed tool words and no payload byte.
The recorded 2.1.328 fixture set was re-recorded whole by verify --record, 32 to 33 files, the new cross-session arrival variant among them; measured: its prompt holds the recording host's runtime-dir path and the sender's pid inside the CLI's own prefix, which the scrub and the hygiene walk do not normalise or refuse.
Retired: addressing the sender by a bare session id; nothing in the CLI's verbs, refusals or exit codes was removed.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger: the relied row list and its count, each row's reading, the stamp merge, the recorded-fixture scrub and the hygiene walk
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: the agent crate's compiled literals: CLI flags, the socket scheme, directory, suffix and path bound
@task_file n=3 path=crates/viola-agent-claude/src/hook.rs
what: hook payload reading: the harness prefix match that names a prompt's origin
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: unit tests of the capability ledger's words and capture documents
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit tests of each ledger row's pass and fail readings
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: unit tests of the ledger's framing turns
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit tests of the ledger stamp written by viola verify
@task_file n=8 path=src/cmd/hook.rs
what: the hidden viola hook verb and its capture arm, now with the hidden --names-only flag
@task_file n=9 path=src/cmd/verify.rs
what: the viola verify verb: its runs, its step lines, its summary and its stop
@task_file n=10 path=src/cmd/verify/typed.rs
what: viola verify's typed runs and their inputs, now the runtime dir read once
@task_file n=11 path=src/cmd/verify/typed/runs.rs
what: viola verify's typed run ends
@task_file n=12 path=src/cmd/verify/typed/turns.rs
what: viola verify's Run B turns, now the cross-session turn and the sender child
@task_file n=13 path=src/run/version_gate.rs
what: the start's version gate that runs the automatic verify on a new CLI version (tests only)
@task_file n=14 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent that stands in for the claude CLI, now a cross-session sender and receiver
@task_file n=15 path=src/bin/viola-fake-agent/agents.rs
what: the fake agent's sibling lookup, now by socket address
@task_file n=16 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: tests of the fake agent's modes, the sender and receiver among them
@task_file n=17 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: tests of the fake agent's options over the recorded set
@task_file n=18 path=schemas/diag-line.v1.json
what: the diagnostic log line schema: its subject list, verify-sender added
@task_file n=19 path=crates/viola-e2e/src/harness/boot.rs
what: the test harness boot stamp that runs viola verify
@task_file n=20 path=crates/viola-e2e/src/harness/run.rs
what: the test harness run checks, the expected verify step lines among them
@task_file n=21 path=crates/viola-e2e/src/rig/mod.rs
what: the standing live rig: its rounds, kept stamps and summary lines
@task_file n=22 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsal tests
@task_file n=23 path=tests/cli_verify.rs
what: integration tests of viola verify's output, rows and step lines
@task_file n=24 path=tests/cli_verify/runs.rs
what: integration tests of viola verify's runs
@task_file n=25 path=tests/cli_verify/stamps.rs
what: integration tests of viola verify's ledger stamp
@task_file n=26 path=tests/cli_fake_agent.rs
what: integration tests of the fake agent's receipt
@task_file n=27 path=tests/cli_send.rs
what: integration tests of viola send (a recorded session id literal moved)
@task_file n=28 path=tests/chaos_revive.rs
what: chaos tests of viola revive (a recorded session id literal moved)
@task_file n=29 path=tests/contract_diag_schema.rs
what: the contract test of the diagnostic line schema
@task_file n=30 path=tests/contract_fake_agent_drift.rs
what: the contract test that the fake agent matches the recorded dialog sets
@task_file n=31 path=tests/contract_fixture_hygiene.rs
what: the contract test of the recorded fixtures' hygiene
@task_file n=32 path=tests/contract_ledger_probes.rs
what: the contract test of the ledger probes over every stamped set
@task_file n=33 path=tests/security_negatives_env_capture.rs
what: the security negative tests of the hook capture arm and env capture
@task_file n=34 path=tests/support/home.rs
what: test support for a viola home and wrapper starts
@task_file n=35 path=tests/support/start.rs
what: test support for run and revive starts
@task_file n=36 path=tests/support/verify.rs
what: test support for viola verify runs, now with a test-owned runtime dir
@task_file n=37 path=fixtures/claude/2.1.328
what: the recorded claude 2.1.328 hook fixture set that verify --record writes and verify reads, replaced whole
@task_file n=38 path=fixtures/claude/2.1.328/UserPromptSubmit.cross-session-1.json
what: the new recorded cross-session arrival variant of the 2.1.328 set
@task_item n=1 part=touches
viola verify step line: the relied count it prints moved 28 to 29 ([NN/29]); a row holding the figure is stale, a row stating the count by rule is not.
@task_item n=2 part=touches
viola verify summary and summary line: the clean summary now reads 29 pass 0 fail; any row holding 28 is stale.
@task_item n=3 part=touches
viola verify output exit 0 and exit 1, failing row: a new row id cross-session-prefix, reading pass or fail only, can be the failing row; with no runtime dir it fails (not-run in the stamp).
@task_item n=4 part=touches
viola verify output plain ASCII and stdout-versus-stderr: the sender child adds no line; neither the runtime dir nor the composed socket address enters any verify line, step line, error or stop.
@task_item n=5 part=touches
viola verify output --record: the re-recorded set gained a cross-session arrival variant whose prompt keeps the recording host's runtime-dir path (with its uid) and the sender's pid inside the CLI's own prefix; the scrub rewrites only the home and the user word.
@task_item n=6 part=touches
viola verify refusals, recorded fixture not clean: the hygiene walk refuses only a drive-letter, home or Users path, so a runtime-dir path in a fixture passes it and raises no refusal.
@task_item n=7 part=touches
viola verify output --help paragraph and usage: any row listing what verify runs or the relied count now meets the sender child and the 29th row.
@task_item n=8 part=touches
viola run and viola revive passed start output: the automatic verify on a new CLI version runs the same measure, the sender child and the 29-row count included.
@task_item n=9 part=touches
hidden verbs: viola hook gained a hidden --names-only flag valid only with --capture; no help row, verb group or usage line shows it.
@task_item n=10 part=touches
hints never quote text or name paths: the composed socket address is a path verify holds and must name in no hint or refusal.
@task_item n=11 part=touches
names taken: the row id cross-session-prefix, the diag subject verify-sender, the flag --names-only and the capture file name Tool.k.json.
@task_fact n=1 slug=surface-cli
fact: viola verify prints no line holding the runtime dir's value or the socket address it composes for the cross-session turn; that address reaches only the sender child's prompt.
@task_fact n=2 slug=surface-cli
fact: viola hook takes a hidden --names-only flag, valid only with --capture, filing only closed tool words and never a payload byte; it appears in no help output.
@task_fact n=3 slug=surface-cli
fact: verify --record keeps a cross-session arrival's prompt as the CLI wrote it, so the recorded fixture holds the recording host's runtime-dir path and the sender's pid; the scrub and the recorded-fixture-not-clean refusal do not catch it.
@task_item n=1 part=untouched
The viola list board, its BAY line, columns, colour depth, truncation and JSON document: no board cell or state word moved.
@task_item n=2 part=untouched
viola send, wait, last, answer and their readback mirror and vocabulary: no outcome word, issue line or hint moved.
@task_item n=3 part=untouched
CLI exit codes, the JSON error form and the internal-error output: no code added or reassigned.
@task_item n=4 part=untouched
CLI token rendering, SGR styling, the no-spinner, no-banner and no-progress decisions: no styling or glyph changed.
@task_item n=5 part=untouched
The MCP server and its tools and results: no tool, argument or result shape changed.
@task_item n=6 part=untouched
The frame verbs' exit-21 hints and detail, the socket-dir-refused start refusals and viola link and unlink output: the channel endpoint's socket dir is unchanged.
@task_item n=7 part=untouched
viola revive refusals, its --list rows and preflight order, and viola plugin install output: unchanged.
@task_item n=8 part=untouched
The web surface and deep-link absences: still absent.
