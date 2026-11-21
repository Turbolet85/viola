@task_prose part=session
The session added a 29th relied capability-ledger row, cross-session-prefix, which viola verify stamps live by having a print-mode sender session send a message to Run B's interactive session and reading the harness prefix on the arrival. The row reads pass or fail only.
The relied ledger rows went from 28 to 29. Verify's step lines now count [NN/29], the summary reads 29 pass 0 fail, and a 28-row stamp now reads unverified.
viola verify now reads XDG_RUNTIME_DIR once, held in memory, to compose the receiving session's uds socket address. The address goes only into the sender's prompt and reaches no log line, step line, stamp, error or fixture that verify writes itself.
The hook verb gained a hidden --names-only option, valid only with --capture, that files closed tool words and no payload byte. The diag-line subject enum gained verify-sender.
The recorded claude 2.1.328 fixture set was re-recorded whole and went from 32 to 33 files, with a new cross-session arrival file. Measured by the wrap: that arrival's prompt holds the recording host's runtime-dir socket path and the sender's pid, inside the CLI's own prefix, and the scrub and hygiene walk do not normalise it.
Every test and harness verify now runs with a test-owned runtime dir. Bare session-id addressing was measured red and replaced by the address form. Nothing was retired.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger: the relied rows, their readings, the probe plugins, the stamp merge and the fixture scrub; edited (the 29th row, the uds address composer, the sender prompt, the closed tool and result words)
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: the agent crate's compiled literals (socket scheme, socket dir, socket suffix, socket path bound, allowed-tools flag); edited
@task_file n=3 path=crates/viola-agent-claude/src/hook.rs
what: the hook payload reading and prompt-origin classification, now over named harness prefixes; edited
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: unit tests of the ledger's tool words, capture documents and sender plugin; edited
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit tests of the ledger row checks, the cross-session row and the address composer; edited
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: unit tests of the ledger's framing turns; edited
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit tests of the stamp merge and its measured blocks; edited
@task_file n=8 path=src/cmd/hook.rs
what: the hidden hook verb and its capture arm, gaining the names-only capture; edited
@task_file n=9 path=src/cmd/verify.rs
what: the viola verify verb: its runs, its single runtime-dir read and its sender plugin; edited
@task_file n=10 path=src/cmd/verify/typed.rs
what: verify's typed measurement inputs, now holding the runtime dir; edited
@task_file n=11 path=src/cmd/verify/typed/runs.rs
what: verify's typed runs and how Run B ends; edited
@task_file n=12 path=src/cmd/verify/typed/turns.rs
what: verify's typed turns, gaining Run B's cross-session turn and the print-mode sender; edited
@task_file n=13 path=src/run/version_gate.rs
what: tests of the start's CLI version gate over the relied row count; edited
@task_file n=14 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent, gaining the fake sender and receiver of a cross-session message; edited
@task_file n=15 path=src/bin/viola-fake-agent/agents.rs
what: the fake agent's sibling lookup, now by uds address; edited
@task_file n=16 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: tests of the fake agent's sender and receiver modes; edited
@task_file n=17 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: tests of the fake agent's options; edited
@task_file n=18 path=schemas/diag-line.v1.json
what: the diag line schema, its subject enum gaining verify-sender; edited
@task_file n=19 path=crates/viola-e2e/src/harness/boot.rs
what: the e2e harness boot stamp, now giving verify an owner-only runtime dir; edited
@task_file n=20 path=crates/viola-e2e/src/harness/run.rs
what: the e2e harness run and its expected live verify step lines at 29; edited
@task_file n=21 path=crates/viola-e2e/src/rig/mod.rs
what: the live rig, its kept stamps and summary lines for the cross-session row and the sender's words; edited
@task_file n=22 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig rehearsals, now carrying a command's env over and setting a runtime dir; edited
@task_file n=23 path=tests/cli_verify.rs
what: integration tests of viola verify's step lines and the cross-session row's pass, not-run, red and kill cases; edited
@task_file n=24 path=tests/cli_verify/runs.rs
what: integration tests of verify's runs; edited
@task_file n=25 path=tests/cli_verify/stamps.rs
what: integration tests of verify's stamps at the relied count; edited
@task_file n=26 path=tests/cli_fake_agent.rs
what: tests of the fake agent's receipt, its start receipt key pin now nine keys; edited
@task_file n=27 path=tests/cli_send.rs
what: an integration test whose recorded session id literal moved with the re-recorded fixture set; edited
@task_file n=28 path=tests/chaos_revive.rs
what: a chaos test whose recorded session id literal moved with the re-recorded fixture set; edited
@task_file n=29 path=tests/contract_diag_schema.rs
what: the contract test of the diag line schema's subjects; edited
@task_file n=30 path=tests/contract_fake_agent_drift.rs
what: the contract test that the fake agent's replay matches the recorded dialog sets; edited
@task_file n=31 path=tests/contract_fixture_hygiene.rs
what: the contract test walking the recorded fixture sets for hygiene, now walking the cross-session variant; edited
@task_file n=32 path=tests/contract_ledger_probes.rs
what: the contract test that the ledger probes pass over every stamped set; edited
@task_file n=33 path=tests/security_negatives_env_capture.rs
what: the security negative tests of the hook capture arms, gaining the names-only canary case; edited
@task_file n=34 path=tests/support/home.rs
what: test support for test homes and starting a wrapper as a user with a test-owned socket root; edited
@task_file n=35 path=tests/support/start.rs
what: test support for starts, a recorded session id literal moved; edited
@task_file n=36 path=tests/support/verify.rs
what: test support for running verify with and without a test-owned runtime dir; edited
@task_file n=37 path=fixtures/claude/2.1.328
what: the recorded claude 2.1.328 hook fixture set, re-recorded whole, 32 to 33 files; replaced
@task_file n=38 path=fixtures/claude/2.1.328/UserPromptSubmit.cross-session-1.json
what: the recorded cross-session arrival fixture, a new file holding the CLI's cross-session prefix with the sender's socket address
@task_item n=1 part=touches
contract/viola-verify-output/step-line: the step counter's total moved from 28 to 29 relied rows ([NN/29]), and a new step line exists for cross-session-prefix. Check whether the row holds the figure or states the count by rule.
@task_item n=2 part=touches
contract/viola-verify-output/summary and contract/viola-verify-output/summary-line: the summary now reads 29 pass 0 fail. Check whether a cell holds the relied count as a figure.
@task_item n=3 part=touches
contract/cli-verification/verified and contract/cli-verification/unverified-cli: verified now needs all 29 relied rows, so a stamp that holds only the old 28 reads unverified. A live home stamped before this session is now unverified.
@task_item n=4 part=touches
contract/viola-verify-output/fail-word: the new cross-session-prefix row reads pass or fail only. With no runtime dir its turn does not run and the row fails.
@task_item n=5 part=touches
contract/viola-verify-output/trust-record-names-nothing: the new row's step line also names nothing. The composed socket address, the runtime-dir path and the pid reach no step line, summary or refusal.
@task_item n=6 part=touches
decision/no-internals-in-errors and decision/start-refusals-show-no-path-or-pid, indirectly through verify: the new runtime-dir read and the composed address (a path holding the uid, plus a pid) are kept out of every human and --json output line.
@task_item n=7 part=touches
decision/step-counter-only-progress-pattern and decision/one-progress-pattern, through verify's new Run B sender turn: it adds no progress line of its own beyond the counted step line.
@task_item n=8 part=touches
name/command/hook, contract/hook-verb/exit-code and contract/hook-verb/stderr: the hidden hook verb gained a hidden --names-only option, valid only with --capture, that files closed-word tool files and no payload byte.
@task_item n=9 part=touches
contract/viola-verify-refusals/unclean-fixture: the hygiene walk behind this refusal reads only home and user paths. The recorded cross-session arrival holds a runtime-dir socket path and a pid that pass that walk unflagged.
@task_item n=10 part=touches
contract/auto-verify-line rows, indirectly: the start's automatic verify runs the same measure, so it now spawns the sender too and inherits the start's runtime dir. The line forms themselves did not move.
@task_fact n=1 slug=surface-cli
fact: viola hook takes a hidden --names-only option, valid only with --capture, that files closed tool words and never a payload byte; it appears in no help output.
@task_fact n=2 slug=surface-cli
fact: viola verify reads XDG_RUNTIME_DIR once to compose a session's socket address, and that address, path or pid never appears in any verify step line, summary, refusal or --json document; without a runtime dir the cross-session-prefix row fails rather than refusing the run.
@task_item n=1 part=untouched
brand-identity strips, the readback box, the send readback contracts and handoff display: the session changed no board, strip, readback or send output.
@task_item n=2 part=untouched
color-palette, typography, spacing, depth-strategy, border-radius and iconography: no colour, glyph, column, separator or font rule moved.
@task_item n=3 part=untouched
viola list board and table contracts, skip counts and NAME truncation: list output did not change.
@task_item n=4 part=untouched
run and revive start refusals, revive --json codes, exit-21 causes and the strict-modes refusal pairs: no refusal form, cause or exit code was added or changed.
@task_item n=5 part=untouched
wrapper fault --json details, peer reply screening, the wrapper socket refused peer, and the link, unlink, wait and last outputs: nothing in the frame verbs moved.
@task_item n=6 part=untouched
anti-patterns and self-validation protocol rows: no output pattern was added that they govern beyond the verify lines named in the touches.
@task_item n=7 part=untouched
the web surface absence and the rendered-page absence: still true, nothing rendered.
