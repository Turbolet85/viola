@task_prose part=session
The session added a 28th and last relied ledger row, identity-floor, so viola verify now prints step lines [01/28] through [28/28] and summaries read 28 pass 0 fail; every relied-count literal moved from 27 to 28.
The identity floor, the CLAUDE names stripped from every viola child (viola run, viola verify and the agents read), grew from 11 to 13 names, and a new list of 4 non-identity hook names was added that only the verify env read uses.
The hidden hook capture arm now also writes, on SessionStart only, an owner-only env-names file holding only the process's CLAUDE env names and a count of the other names, never a value; viola verify reads it, adds an env_probe block to the stamp, and record writes a new names-only fixture class.
The fake agent replays the recorded env names to its SessionStart hook children; the live rig gained the HF, HF-2 and HG labels and identity-floor and env-unknown summary lines; the recorded claude 2.1.328 fixture set was re-recorded whole.
A cross-session prefix row, its sender and receiver, were written by a stopped run and cut whole before commit, so none of them was ever committed; nothing was retired from the committed tree.
CI ran green 7 of 7 with 3891 tests passed and none skipped; the operator-run pre-push, hygiene and smoke checks were green as well.
@task_file n=1 path=.config/nextest.toml
what: the test runner's configuration, the nextest profiles the existing suites run under (edited)
@task_file n=2 path=Cargo.toml
what: the workspace manifest, its test targets (edited, a new integration test target registered)
@task_file n=3 path=crates/viola-agent-claude/src/ledger.rs
what: the claude relied-row ledger behind viola verify, its rows, step lines, stamp and the record scrub (edited, the identity-floor row and env read added)
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the ledger's unit tests (edited)
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the ledger's per-row check unit tests (edited, identity-floor pass and fail tables added)
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: the ledger's run-reading unit tests (edited, row count literals)
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger's verify-stamp unit tests (edited, the env block in the stamp)
@task_file n=8 path=crates/viola-agent-claude/src/lib.rs
what: the claude agent crate's root, holding the identity-floor name list and the env strip applied to every viola child (edited, two floor names and the non-identity hook name list added)
@task_file n=9 path=crates/viola-e2e/src/harness/run.rs
what: the end-to-end harness's run driver, holding the pinned literal of viola verify's passing step-line output (edited, the 28-line literal)
@task_file n=10 path=crates/viola-e2e/src/harness/secret_scan.rs
what: the harness secret scan, its critical canary list that must never reach output (edited, four canaries added)
@task_file n=11 path=crates/viola-e2e/src/rig/mod.rs
what: the standing live rig that drives real claude starts and reads verify stamps (edited, new labels and summary lines)
@task_file n=12 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsal tests (edited, pass-count literals)
@task_file n=13 path=fixtures/claude/2.1.328/Env.recorded-1.json
what: a recorded claude fixture, the env names the CLI adds to a hook, names only (new file)
@task_file n=14 path=fixtures/claude/2.1.328
what: the recorded claude 2.1.328 hook payload fixture set verify records and the fake agent replays (replaced, re-recorded whole)
@task_file n=15 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent that replays recorded claude behaviour (edited, SessionStart env replay)
@task_file n=16 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the fake agent's mode tests (edited)
@task_file n=17 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's option tests (edited, a pinned recorded session id)
@task_file n=18 path=src/cmd/hook.rs
what: the viola hook command, the hook event and dialog calls and the hidden capture arm (edited, env-names file on SessionStart)
@task_file n=19 path=src/cmd/verify.rs
what: the viola verify command, its probe runs, step-line output, stamp and record (edited, the identity-floor step and env read)
@task_file n=20 path=src/run/auto_verify.rs
what: the automatic verify a start fires for a new CLI version (edited, tests only)
@task_file n=21 path=src/run/version_gate.rs
what: the start's version gate over the verify stamp (edited, tests only)
@task_file n=22 path=tests/chaos_revive.rs
what: the revive chaos integration tests (edited, a pinned recorded session id)
@task_file n=23 path=tests/cli_send.rs
what: the viola send integration tests (edited, a pinned recorded session id)
@task_file n=24 path=tests/cli_verify.rs
what: the viola verify integration tests, pinning its step-line output by literal (edited, identity-floor cases)
@task_file n=25 path=tests/cli_verify/runs.rs
what: the viola verify run integration tests (edited)
@task_file n=26 path=tests/cli_verify/stamps.rs
what: the viola verify stamp integration tests (edited, 28-row stamp verified, 27-row unverified)
@task_file n=27 path=tests/contract_fake_agent_drift.rs
what: the contract test holding the fake agent to the recorded fixtures (edited, 28-row literals)
@task_file n=28 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene contract test, rejecting user names, absolute paths and emails in recorded fixtures (edited, the env fixture class)
@task_file n=29 path=tests/contract_ledger_probes.rs
what: the contract test reading every stamped fixture set through the ledger probes (edited)
@task_file n=30 path=tests/security_negatives_env_capture.rs
what: a new security negative test suite, that the capture arm writes env names and no value and a kept name reaches no stamp or record (new file)
@task_file n=31 path=tests/support/start.rs
what: the integration tests' shared start support (edited, a pinned recorded session id)
@task_file n=32 path=tests/support/verify.rs
what: the integration tests' shared verify support (edited, an env fixture writer)
@task_file n=33 path=tests/tui_env_strip.rs
what: the TUI passthrough env strip test, the strip oracle of names removed from the child (edited, 12 to 14 entries)
@task_item n=1 part=touches
The viola verify step line contract: the step lines now number [NN/28] and a new identity-floor step line exists; a row whose cell holds 27 or the step list is stale.
@task_item n=2 part=touches
Fixed CLI strings pinned by literal: viola verify's passing output literal and the summary 28 pass 0 fail moved, each pinned by literal tests.
@task_item n=3 part=touches
The viola run TUI passthrough entry: the env strip applied to the wrapped child now removes 13 identity-floor names; a row stating the child's environment or the floor count is touched.
@task_item n=4 part=touches
Run diagnostics never reach the terminal and zero viola bytes on the terminal: the capture arm writes its env-names file off the terminal and prints nothing; the rule now covers the new file write.
@task_item n=5 part=touches
Critical path CLI board, wait/last and verify output: verify output gained one step line.
@task_item n=6 part=touches
viola verify interrupted pair and exit: the verify run now reads one more file before its last step; a row on interrupted verify output is touched only through the larger step count.
@task_item n=7 part=touches
Paths not carried: the new env-names fixture and capture file hold names only, and fixture hygiene rejects absolute paths, emails and the user word in them.
@task_item n=8 part=touches
Live readings never gate: the live rig gained the HF, HF-2 and HG starts and identity-floor and env-unknown summary lines; live readings still gate nothing.
@task_item n=9 part=touches
CI test gate allows no skips, zero flakiness and a11y inside existing suites: CI ran 3891 passed with none skipped; the watched human-takes-the-wheel case passed in four CI runs.
@task_item n=10 part=touches
Critical path human takes the wheel: its watched pause-then-release test passed in CI and locally; no behaviour changed.
@task_item n=11 part=touches
Single vendor-content boundary and boundary check never inspects claude content: the capture arm reads only env names of its own process, never vendor content or values.
@task_item n=12 part=touches
Output-discipline assertions stand in for a11y tools: new literal tests over the verify output and the strip oracle extend that set.
@task_item n=1 part=untouched
viola list board, its caption, colour, SGR, stale-row dim and NAME truncation: no board output changed.
@task_item n=2 part=untouched
CLI refusal format, unable and hint lines, reason codes and exit codes 2, 10, 11, 13, 14, 20 and 21: no refusal or exit code changed.
@task_item n=3 part=untouched
viola revive and its --json refusals, viola link and unlink, viola plugin install and viola --help output: untouched.
@task_item n=4 part=untouched
Dialog hold, time bounds on the driver, wait timeouts and send refusals: no time bound or dialog path changed.
@task_item n=5 part=untouched
Manual Orca and keyboard passes and their screen-reader setups: no manual pass ran or changed.
@task_item n=6 part=untouched
Absences of a web or graphical frontend, a touch surface, internationalization, a rendered page and a user time limit: still absent.
@task_item n=7 part=untouched
Human keystrokes, focus, mouse and resize sequences and the closed non-editing stdin list in the TUI passthrough: unchanged.
@task_item n=8 part=untouched
Structured violation JSON and the log identity tags: no format changed.
