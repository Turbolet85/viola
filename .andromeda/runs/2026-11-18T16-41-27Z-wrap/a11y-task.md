@task_prose part=session
The session made the CLI's browser-tools prompt (literal Yes, use my browser) a third compiled modal signature beside the trust and imports literals, and a sixth first-screen closed word browser-tools.
It added a 24th relied ledger row browser-tools-prompt with three words pass, absent, fail; absent counts toward verified on this row alone, so relied rows moved 23 to 24 and the verify step counter to [NN/24].
viola verify's step line now ends in pass, absent or fail, and its summary stamped <ver>  n pass  m fail counts an absent row in n; the all-pass summary reads 24 pass  0 fail.
Verify's settle and the run pre-send gate refuse at once on the prompt and type nothing into it; viola send on a verified CLI refuses with exit 13 while the prompt stands, no key sent.
Test-only: the fake agent gained the argv option --start-screen, the live rig gained the VP and VK arms with a no-key stop and the cache-a key-name read (the founder's own word); measured live on 2.1.325, BP showed the prompt and BK read 24/24.
No IPC method, env var, crate, dependency or config key was added; nothing retired.
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: the compiled screen signatures: the modal literals, the first-screen classification and the input-box readiness read from them
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger: its relied rows, each row's verdict word, and the verified rule over the stamp
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the ledger's crate tests of a row's readings
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the ledger's crate tests of the modal-signature and trust-record checks
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger's crate tests of the stamp and the verified rule
@task_file n=6 path=src/cmd/verify.rs
what: viola verify output: the step line and the summary line
@task_file n=7 path=src/cmd/verify/typed/pty.rs
what: viola verify's settle on a first screen over the outer PTY
@task_file n=8 path=src/cmd/verify/typed/turns.rs
what: viola verify's typed runs, which paste nothing into a modal
@task_file n=9 path=src/run/gate.rs
what: the run pre-send gate: input-box readiness and the refusal on a modal screen
@task_file n=10 path=src/run/version_gate.rs
what: the version gate: a version verified only when every relied row counts
@task_file n=11 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent and its argv-only options
@task_file n=12 path=tests/cli_send.rs
what: the viola send integration suite, its refusal cases
@task_file n=13 path=tests/cli_verify.rs
what: the viola verify integration suite, its step-line and summary literal pins
@task_file n=14 path=tests/cli_verify/runs.rs
what: the viola verify integration cases over the typed runs
@task_file n=15 path=tests/cli_verify/stamps.rs
what: the viola verify integration cases over the written stamp
@task_file n=16 path=tests/contract_ledger_probes.rs
what: the ledger probe contract suite, its relied-row id list
@task_file n=17 path=crates/viola-e2e/src/harness/run.rs
what: the test harness's local-live row list and its verify-output fixture
@task_file n=18 path=crates/viola-e2e/src/rig/mod.rs
what: the standing live rig: its arms, labels and summary lines
@task_file n=19 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's private scratch home: the seeded markers and the cache-a key-name read
@task_file n=20 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsals against the fake agent
@task_item n=1 part=touches
contract/viola-verify/step-line: the step line's verdict word may now be absent (on the browser-tools-prompt row alone) beside pass and fail, and its counter reads [NN/24].
@task_item n=2 part=touches
decision/critical-path-cli-board-output: verify output, a critical path, gained the absent verdict and a summary whose n counts the rows that count.
@task_item n=3 part=touches
decision/no-screen-content-parsing: the signatures it allows grew by a third modal literal, Yes, use my browser, and a first-screen word browser-tools; still no parsing beyond signatures.
@task_item n=4 part=touches
decision/fixed-cli-strings-pinned and decision/fixed-cli-strings-pinned-by-literal: new fixed literals pinned in tests, the 24th step line ending absent and the summary 24 pass  0 fail.
@task_item n=5 part=touches
contract/cli-exit-codes/13 and contract/send-refusal/input-not-ready: viola send on a verified CLI refuses at exit 13 while the browser-tools prompt stands, no key sent; check the refusal's wording still holds.
@task_item n=6 part=touches
decision/send-refused-while-human-typing and decision/human-keystrokes-never-blocked: the gate and verify type nothing into the prompt, which a person must answer once; human keys still reach the child.
@task_item n=7 part=touches
decision/boundary-check-never-inspects-claude-content and decision/no-screen-content-parsing: the test rig now reads whether a key NAME stands in its own private scratch config, never a value, the founder's widening.
@task_item n=8 part=touches
decision/live-readings-never-gate: live BP and BK readings on 2.1.325 proved the new row but gate no CI step.
@task_item n=9 part=touches
decision/time-bounds-never-on-human and decision/time-bounds-fall-on-the-driver: on a host where the prompt stands unanswered verify settles at once and the version stays unverified until a person answers it; no timer falls on the human.
@task_item n=10 part=touches
decision/a11y-inside-existing-suites: the new verify and send literal cases went into the existing integration suites.
@task_fact n=1 slug=1-a11y-scope-summary
fact: viola verify's summary line stamped <ver>  n pass  m fail counts in n every relied row that counts, an absent browser-tools-prompt row included, while its step line still prints absent.
@task_item n=1 part=untouched
viola list board, caption, colour, stale-row and NAME truncation rows: no board output moved.
@task_item n=2 part=untouched
viola revive, link, unlink, plugin install, wait and help contracts and their --json refusal documents: none changed.
@task_item n=3 part=untouched
TUI passthrough rows (entry, vendor content, terminal bytes, focus, mouse and resize sequences, vt100 feed panic): the wrapper's passthrough bytes did not change.
@task_item n=4 part=untouched
SGR and colour rules (word beside every SGR, no SGR off a terminal, amber and bold): verify output gained a word, no colour.
@task_item n=5 part=untouched
frame verb, hook call, MCP tool and instance-unreachable refusal rows (socket-dir, strict-modes, server-verify): no IPC method or refusal code added.
@task_item n=6 part=untouched
CI integration and SLO rows (fail-fast gate, zero flakiness, no own artifact, coverage suite): the CI workflow and its gates did not change.
@task_item n=7 part=untouched
screen-reader targets and manual Orca and keyboard passes, and the absences (web frontend, touch, i18n, rendered page): untouched.
