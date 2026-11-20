@task_prose part=session
A viola run or revive start that meets a claude version with no stamp now runs viola verify once by itself, before the child starts, when the new config.json key verify_new_cli is true; the key is off by default, and any value other than a JSON boolean reads as off.
The start prints one of five fixed stderr lines about that verify (verifying, did not verify, an earlier red, verifying elsewhere, no stamp with a closed word spawn-failed, unstamped or lock-refused): one write each, ASCII, no SGR, holding only the parsed version and a closed word, and none under --json.
The verify child gets the start's terminal for its own output (nothing under revive --json). A green stamp lifts the unverified-cli gate for that start. A red attempt is recorded, and the next start runs no verify. Two starts at once run one verify between them, through a new attempt lock in the ledger directory.
The start order gains one step after the socket bind, and the trust-record step moves after it so it is written on the final verdict. A SIGINT or SIGTERM during that verify stops the verify child (30 s grace, then kill) and ends the start by the signal, with no stamp and no session.
Counts moved: known config keys 4 to 5, closed spawn subjects 7 to 8, the controls table 8 to 9 rows. Nothing was retired. The unit, integration and e2e layers, a live round, pre-push and CI all ran green.
@task_file n=1 path=src/run/auto_verify.rs
what: the start's automatic verify step: reads the verify_new_cli key, decides whether to verify, spawns viola verify under the attempt lock, and handles the stop signal during the wait
@task_file n=2 path=src/human/auto_verify.rs
what: the five fixed stderr lines a viola run start prints about its automatic verify, and the closed no-stamp words
@task_file n=3 path=crates/viola-agent-claude/src/ledger.rs
what: the CLI verification ledger: which relied rows a version's stamp must hold; gains the attempted reading beside verified
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit cases for the ledger's stamp readings
@task_file n=5 path=crates/viola-state/src/stamps.rs
what: the stamps.json state file and its locks; gains the verify attempt lock (ledger/verify.lock, 0600)
@task_file n=6 path=src/obs.rs
what: the observability layer: known config.json keys list and config parse-rejected diagnostics
@task_file n=7 path=schemas/config.v1.json
what: the config.json schema; gains the verify_new_cli boolean property
@task_file n=8 path=schemas/diag-line.v1.json
what: the diagnostic log line schema; its spawn subject enum gains auto-verify
@task_file n=9 path=src/run/version_gate.rs
what: the unverified-cli version gate of viola run: the strict stamps read and verified verdict, now also the version's entry state
@task_file n=10 path=src/run/mod.rs
what: the viola run module list
@task_file n=11 path=src/cmd/run.rs
what: the viola run and revive start sequence: start order, gate, socket bind, trust record, child start
@task_file n=12 path=src/human.rs
what: the human-mode output module list
@task_file n=13 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent standing in for claude; gains extra arguments read from a file beside it
@task_file n=14 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: unit cases for the fake agent's options
@task_file n=15 path=tests/support/fake.rs
what: integration test support for configuring the fake agent
@task_file n=16 path=tests/support/home.rs
what: integration test support for a test home and a wrapped start
@task_file n=17 path=tests/support/outer_pty.rs
what: integration test support for the outer terminal a start runs in
@task_file n=18 path=tests/cli_version_gate.rs
what: integration cases for the unverified-cli gate and the start's automatic verify
@task_file n=19 path=tests/cli_controls_not_disableable.rs
what: integration cases proving no setting turns a safety control off
@task_file n=20 path=tests/contract_bounded_readers.rs
what: the contract test listing every bounded reader of outside input
@task_file n=21 path=crates/viola-e2e/src/rig/mod.rs
what: the live e2e rig: arms, sessions, stops and summary lines; gains the AV arm
@task_file n=22 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's isolated scratch home and its seeded config
@task_file n=23 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig's command-line entry
@task_file n=24 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: rehearsals of the live rig's arms against the fake agent
@task_item n=1 part=touches
contract/viola-run-output/child-started and contract/viola-revive-output/after-child-start: a start with the key on can now print an automatic-verify line, plus verify's own output, before the child starts.
@task_item n=2 part=touches
decision/run-prints-nothing-while-wrapped: the new lines and verify's output come before the child holds the terminal. Check that the row still holds and that it names this window before the child.
@task_item n=3 part=touches
contract/cli-streams/stderr, contract/cli-surface/output-streams and decision/stdout-data-stderr-messages: the automatic-verify lines are messages on stderr, one write each.
@task_item n=4 part=touches
contract/viola-verify-output/step-line, summary, summary-line, fail-word, stopped-or-refused and stamps-refused: verify's output now also appears inside a viola run or revive start. A start now starts verify, so verify is no longer only run by hand. The words are unchanged, and the output is nulled under revive --json.
@task_item n=5 part=touches
contract/viola-verify-refusals/home-tree-check, interrupted and stamps-update: the start's own home-tree check runs before verify's. A stop during the start's verify ends verify through its stop path, and verify is still the one writer of stamps.json.
@task_item n=6 part=touches
contract/cli-verification/unverified-cli and contract/cli-verification/verified: a version can now become verified with no hand-run verify, through the start's automatic green stamp. A red attempt leaves it unverified.
@task_item n=7 part=touches
contract/cli-exit-codes/12 and contract/refusal-hints/unverified-cli: an automatic green stamp lifts the gate. The rule, the code and the hint words are unchanged, but the hint's route to verified gains the config key.
@task_item n=8 part=touches
contract/viola-run-start-refusals/home-tree-check and contract/viola-revive-refusals/order: the start order gains the automatic-verify step after the socket bind, and the trust-record step moves after it.
@task_item n=9 part=touches
decision/no-hint-under-json, decision/json-one-document-on-stdout and decision/no-json-after-child-held-terminal: under --json no automatic-verify line prints and the verify child's output is nulled.
@task_item n=10 part=touches
decision/cli-ascii-only, decision/ascii-only-human-output and contract/cli-colour-decision-order/viola-run: the new lines are ASCII with no SGR, whatever the colour decision.
@task_item n=11 part=touches
decision/fixed-phraseology-vocabulary: new closed words spawn-failed, unstamped and lock-refused join the start's vocabulary.
@task_item n=12 part=touches
decision/start-refusals-show-no-path-or-pid, decision/no-internals-in-errors and decision/hint-never-quotes-text: the new lines hold only the parsed version and a closed word. They show no path, pid or row text.
@task_item n=13 part=touches
decision/no-success-before-confirmation and decision/confirmations-carry-context: L1 says verifying before any verdict. The verified verdict comes only from the stamp written after verify.
@task_item n=14 part=touches
decision/step-counter-only-progress-pattern, decision/one-progress-pattern, decision/no-spinner and decision/no-spinners-in-verb-output: the start's verify window shows verify's own step lines with no spinner. Check that no second progress pattern was added.
@task_item n=15 part=touches
decision/no-interactive-prompt-or-pager: the key turns the automatic verify on through config, never by a prompt. Check that the row holds.
@task_item n=16 part=touches
name/command/verify, name/command/run and name/command/revive: verify is now also started by run and revive. No verb was added or renamed.
@task_item n=17 part=touches
contract/cli-exit-codes/1 and contract/cli-exit-codes/0: a start stopped by a signal during its verify ends by that signal, and a green automatic verify lets the start run on.
@task_fact n=1 slug=surface-cli
fact: The config.json key verify_new_cli (a JSON boolean, default false; any other value reads false) makes a viola run or revive start that meets a claude version with no complete stamp run viola verify once before the child starts. A green stamp lifts the unverified-cli gate for that start. The key never lifts the gate by itself.
@task_fact n=2 slug=surface-cli
fact: A start's automatic verify prints exactly one of five fixed stderr lines: verifying, did not verify, an earlier red, verifying elsewhere, or no stamp with spawn-failed, unstamped or lock-refused. Each is one write, ASCII, no SGR, holding only the parsed version and a closed word. None prints under --json.
@task_fact n=3 slug=surface-cli
fact: The automatic verify never loops. A version whose entry already holds every relied row and is not green reads earlier red and runs no verify. A second start while one verify holds the attempt lock reads verifying elsewhere, runs none and starts unverified.
@task_fact n=4 slug=surface-cli
fact: The start's verify child uses the start's terminal for its own output (nulled under revive --json). SIGINT or SIGTERM during it terminates the verify (30 s grace, then kill) and ends the start by that signal, with no stamp, no snapshot and no session.
@task_fact n=5 slug=surface-cli
fact: The start's trust record is written on the final verdict after the automatic verify, which runs after the socket bind.
@task_item n=1 part=untouched
brand-identity's strip, readback box, handoff, send readback and pause or release rows: the session changes no strip field, readback or transfer display.
@task_item n=2 part=untouched
colour-palette, typography, spacing, depth, border-radius and iconography rows, including the colour tokens, glyphs, separators and list columns: no colour, token, glyph or column was added or changed.
@task_item n=3 part=untouched
The viola list board and table rows, the viola wait output rows and the viola last output rows: none of those verbs changed.
@task_item n=4 part=untouched
viola send mirror, peer-reply screening, refusal hints of not-delivered and the wrapper fault --json detail rows: send and the peer protocol are not reached.
@task_item n=5 part=untouched
The exit 21 causes, the revive --json refusal codes and the viola link or unlink output rows: no cause, code or link wording moved.
@task_item n=6 part=untouched
The viola --help verb groups and the hook and mcp verb rows: no verb was added, hidden or regrouped.
@task_item n=7 part=untouched
The web surface absence, the rendered page absence, the self-validation protocol and the anti-pattern rows on decoration, shimmer, status dots and sorting: nothing visual or web-facing changed.
