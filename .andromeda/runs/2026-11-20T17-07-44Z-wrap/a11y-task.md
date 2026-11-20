@task_prose part=session
Added an opt-in automatic verify: with the new config.json key verify_new_cli on, a viola run (and revive's start) meeting a claude version with no stamp entry runs viola verify itself, once, before the wrapped child starts; the key is off by default.
The start order changed: the start's verify runs after the endpoint bind, and the trust step now follows it on the final verdict; the unverified-CLI gate, its rule and exit code are unchanged, a green automatic stamp lifts it.
New human output: five fixed stderr lines L1 to L5 (verifying, did not verify, earlier red, verified elsewhere, no stamp with a closed word spawn-failed, unstamped or lock-refused), each one write, ASCII, no SGR, the parsed version and a closed word only; silent under revive --json, where verify's own output is null too.
A SIGINT or SIGTERM during the start's verify TERMs the verify child, waits a 30 s grace, then KILLs, writes the exit line and re-raises the signal: the start ends with no stamp and no session; a red automatic verify is recorded once and no later start retries it for that version.
New diagnostics subject auto-verify (diag-line subject enum 7 to 8), new on-disk attempt lock ledger/verify.lock, CONFIG_KEYS 4 to 5, the controls table 8 to 9 entries; the test rig gained a live AV leg and rehearsal; nothing was retired.
Measured: the integration cases 6.7 to 6.8 s; the live AV leg 43 s from ledger line to summary, green.
@task_file n=1 path=src/run/auto_verify.rs
what: the start's automatic verify step of viola run: reads the verify_new_cli key, decides, takes the attempt lock, spawns viola verify with null stdin, the stop handler and its grace
@task_file n=2 path=src/human/auto_verify.rs
what: the fixed human stderr lines L1 to L5 of the start's automatic verify, one write each, ASCII, no SGR
@task_file n=3 path=crates/viola-agent-claude/src/ledger.rs
what: the claude verify ledger and its stamps reading, now also whether a version's entry was attempted
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger's stamps reading tests
@task_file n=5 path=crates/viola-state/src/stamps.rs
what: the stamps state file and its locks, now the verify attempt lock after the strict-modes check
@task_file n=6 path=src/obs.rs
what: the diagnostics-level config.json read and its list of known config keys
@task_file n=7 path=schemas/config.v1.json
what: the config.json schema, its known keys
@task_file n=8 path=schemas/diag-line.v1.json
what: the diagnostic line schema, its closed spawn subject list
@task_file n=9 path=src/run/version_gate.rs
what: the unverified-CLI version gate of viola run and its strict stamps read
@task_file n=10 path=src/run/mod.rs
what: the viola run start modules list
@task_file n=11 path=src/cmd/run.rs
what: the viola run and revive start order: bind, the automatic verify, trust record, then the wrapped child
@task_file n=12 path=src/human.rs
what: the human output modules of the CLI
@task_file n=13 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent standing in for claude
@task_file n=14 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's option tests
@task_file n=15 path=tests/support/fake.rs
what: the test support that seeds the fake agent
@task_file n=16 path=tests/support/home.rs
what: the test support that boots a wrapper in a test home and watches a start
@task_file n=17 path=tests/support/outer_pty.rs
what: the test outer PTY boundary driver
@task_file n=18 path=tests/cli_version_gate.rs
what: the integration suite for the unverified-CLI gate, now the start's automatic verify cases pinning L1 to L5 by literal
@task_file n=19 path=tests/cli_controls_not_disableable.rs
what: the integration suite holding that no setting switches a control off
@task_file n=20 path=tests/contract_bounded_readers.rs
what: the bounded-readers contract list of every input read under a frame cap
@task_file n=21 path=crates/viola-e2e/src/rig/mod.rs
what: the live test rig, its arms, session stop and summary lines
@task_file n=22 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's scratch home isolation and its seeded config
@task_file n=23 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig's command entry, its labels and rehearse words
@task_file n=24 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsal tests with the fake agent
@task_item n=1 part=touches
decision/human-keystrokes-never-blocked and contract/tui-passthrough/human-keystrokes: the start's automatic verify window comes before the wrapped child exists and runs verify with null stdin, so the human's keys reach no child until it ends; the plan names this window as the rule's added exception.
@task_item n=2 part=touches
contract/tui-passthrough/viola-terminal-bytes, contract/tui-passthrough/own-bytes and decision/zero-viola-bytes-on-the-terminal: L1 to L5 and verify's own inherited output now reach the terminal before the child's bytes, when the key is on.
@task_item n=3 part=touches
decision/tui-terminal-belongs-to-child and contract/tui-passthrough/entry: the terminal is not yet the child's during the start's verify; the passthrough entry now follows that step.
@task_item n=4 part=touches
decision/run-diagnostics-off-terminal and contract/tui-passthrough/run-diagnostics: the new process-start and process-exit pair with subject auto-verify goes to the log, never the terminal.
@task_item n=5 part=touches
contract/viola-verify/step-line, decision/critical-path-cli-board-output: verify's step lines can now appear at a viola run start, unchanged in form.
@task_item n=6 part=touches
contract/interrupted-verify/exit, contract/interrupted-verify/stderr and contract/viola-verify/interrupted-pair: a stop of the start now TERMs the start's verify, which writes its own interrupted pair; the start then re-raises the signal.
@task_item n=7 part=touches
decision/time-bounds-never-on-human, decision/time-bounds-fall-on-the-driver, absence/user-time-limit and decision/no-unwarned-timeouts: the start's verify holds the human about 40 s behind L1, and its stop carries a 30 s grace on the verify child before KILL; no bound falls on the human.
@task_item n=8 part=touches
decision/no-spinners, decision/static-linear-output and decision/linear-static-cli-output: the verify wait shows one static L1 line, no spinner or progress redraw.
@task_item n=9 part=touches
decision/no-sgr-in-plain-modes, decision/word-beside-every-sgr, decision/sgr-beside-its-word and decision/unable-fail-never-coloured: L1 to L5 carry no SGR, their did-not-verify and no-stamp words uncoloured.
@task_item n=10 part=touches
decision/fixed-cli-strings-pinned, decision/fixed-cli-strings-pinned-by-literal, decision/cli-a11y-by-literal-tests and decision/fixed-copy-from-design: L1 to L5 are new fixed strings, pinned by literal in unit and integration tests; their copy is the design-system's L1 to L5 rows.
@task_item n=11 part=touches
decision/paths-not-carried, decision/control-character-escaping and decision/printed-names-are-parsed-arguments: L1 to L5 carry only the parsed claude version and a closed word, no path.
@task_item n=12 part=touches
contract/viola-revive-json/start-refusal, contract/viola-revive-json/pump-child-error and contract/tui-passthrough/pump-child-error: under revive --json the start's verify prints no L line and verify's output is null, keeping the one document clean.
@task_item n=13 part=touches
decision/strict-modes-refusal-before-any-write, contract/start-refusal/home-strict-modes and decision/unstattable-home-is-strict-modes-refusal: the attempt lock ledger/verify.lock is opened only after the strict-modes check; the start's home check precedes verify's own.
@task_item n=14 part=touches
contract/config-json/dialog-hold-s: a sibling config key verify_new_cli now stands beside it with the same tolerant parse, one parse-rejected config-json malformed line for any non-boolean value, v 2 included.
@task_item n=15 part=touches
decision/cli-not-keyboard-interactive and decision/cli-no-interactive-input: the start's verify child takes null stdin and asks nothing.
@task_item n=16 part=touches
decision/live-readings-never-gate and decision/manual-a11y-pass-gates-nothing: the live rig gained an AV leg whose readings are recorded, not a CI gate.
@task_item n=17 part=touches
decision/tui-boundary-driver-pin and decision/portable-pty-outer-pty-driver: the outer PTY driver now exposes the start's own pid to the stop cases.
@task_item n=18 part=touches
decision/time-bounds-never-on-human sits beside a dialog answer rule: decision/dialog-hold-bounds-the-driver and the unverified-CLI gate it hangs on are unchanged; a green automatic stamp lifts the gate as a hand-run verify does.
@task_fact n=1 slug=2-a11y-strategy
fact: Human keystrokes never blocked has one named exception: the window of a viola run or revive start's automatic verify (key verify_new_cli on, a claude version with no stamp entry), when no wrapped child exists yet and verify runs with null stdin; it ends with verify, before the child starts.
@task_fact n=2 slug=1-a11y-scope-summary
fact: viola run start's automatic verify lines L1 to L5 on stderr: verifying, did not verify, earlier red, verified elsewhere, and no stamp with a closed word spawn-failed, unstamped or lock-refused; each one write, ASCII, no SGR, the parsed version and the closed word only; they and verify's own output precede the child's bytes; none under revive --json.
@task_fact n=3 slug=cognitive-accessibility
fact: config.json verify_new_cli: a JSON boolean, default false; any other value reads false with one parse-rejected config-json malformed line; v must be 1; it turns the start's automatic verify on and lifts no gate by itself.
@task_fact n=4 slug=cognitive-accessibility
fact: A SIGINT or SIGTERM during a start's automatic verify TERMs the verify child, waits a 30 s grace on it then KILLs, writes the exit line and re-raises the signal: the start ends with no stamp and no session; the bound falls on the child, never the human.
@task_item n=1 part=untouched
The viola list board, its captions, colour, stale-row dim, NAME truncation and unwrapped separator: no board output changed.
@task_item n=2 part=untouched
viola link and unlink, plugin install, revive --list and viola wait output: none of these verbs changed.
@task_item n=3 part=untouched
The send, answer and dialog paths: send refusals, the human taking the wheel, the wheel's handling of focus, mouse and resize sequences, the non-editing stdin list.
@task_item n=4 part=untouched
Frame verb, MCP frame tool and hook call refusals, instance-unreachable details and the --json fault bodies: no refusal word, hint or exit code was added or changed.
@task_item n=5 part=untouched
The CLI exit codes table rows and the refusal layout (unable, hint, reason code): the start's automatic verify adds no refusal and no exit code.
@task_item n=6 part=untouched
The CI a11y gate, its suites, artifacts, flakiness and mutation rows, and the manual Orca and keyboard passes: no suite, gate or manual step changed.
@task_item n=7 part=untouched
The absences of a web, graphical or touch frontend, internationalization and a rendered page; the vt100 feed panic and refused-peer passthrough rows.
@task_item n=8 part=untouched
viola --help, its usage line, group labels and served verbs: no verb or flag was added.
