@task_prose part=session
The session closed the route-owed rule lines of the rows landing in code and tests; no verb, flag, exit code or wire body was added or retired.
A human-mode send refused unverified-cli (exit 12) now ends with the hint run viola verify for this CLI version, where it printed no hint before; answer's hint table now names the same string for unverified-cli.
A human-mode answer refused not-delivered now ends with a hint for six details (unknown-dialog its own; control-character, empty-text, input-not-ready, no-prompt-submitted, turn-running the send hint for that detail byte for byte); unknown, a fault, an opaque detail or none ends with no hint.
verify --record's scrub now refuses an email-shaped token in every payload kind through the existing fixture-not-clean refusal and the existing email check code; run's name argument goes through send's one name check, its refusal unchanged (usage error, exit 2, invalid instance name).
The strict-modes check now refuses a symbolic link at a trusted state path before the mode verdict, each verb printing the strict-modes refusal it already prints; the private append open refuses a link; hook and MCP payload types that carry content derive veil Redact so Debug never shows content.
Tests: the fd test reads the live child table on every Unix with no fd count; the resize test sends no key after the resize; one shared digit_runs helper; the snapshot-replay link guard proven by a control.
@task_file n=1 path=src/human/hints.rs
what: the refusal hint tables for send and answer — the hint line that ends a human-mode refusal on stderr, keyed by refusal reason and detail
@task_file n=2 path=src/cmd/send.rs
what: the viola send verb — its refusal-to-hint mapping and the one instance-name check other verbs share
@task_file n=3 path=src/cmd/answer.rs
what: the viola answer verb — its unable refusal output and the hint it ends with
@task_file n=4 path=src/cmd/run.rs
what: the viola run verb — its name argument parsing
@task_file n=5 path=crates/viola-agent-claude/Cargo.toml
what: the Claude agent crate's dependency list
@task_file n=6 path=crates/viola-agent-claude/src/ledger.rs
what: the recorded-fixture scrub verify --record runs over every payload before it writes — the fixture-not-clean refusal's check
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit cases for the recorded-fixture scrub
@task_file n=8 path=crates/viola-agent-claude/src/hook.rs
what: the hook payload types the wrapper reads from Claude's hooks
@task_file n=9 path=crates/viola-agent-claude/src/dialog.rs
what: the dialog payload types — a pending dialog and the answer response
@task_file n=10 path=crates/viola-mcp/Cargo.toml
what: the viola mcp server crate's dependency list
@task_file n=11 path=crates/viola-mcp/src/tools.rs
what: the viola mcp tools' input types — send, answer, wait, last, list
@task_file n=12 path=crates/viola-state/src/strict.rs
what: the strict-modes check over the viola home, instances, instance dirs, snapshot and stamps — the check behind every strict-modes refusal
@task_file n=13 path=crates/viola-state/src/fs.rs
what: the private-file opens for the wrapper's state and diagnostics files
@task_file n=14 path=Cargo.lock
what: the workspace lockfile
@task_file n=15 path=tests/cross_exit_causes.rs
what: the cross-cause check of refusal codes and their hints — no two codes share a hint, no hint names release, every code a code word
@task_file n=16 path=tests/cli_answer.rs
what: the integration tests of viola answer output
@task_file n=17 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene walk over every committed fixture kind
@task_file n=18 path=tests/contract_os_gated_tests.rs
what: the list of OS-gated tests with each one's gate and reason
@task_file n=19 path=tests/tui_pty_seam.rs
what: the real-pty seam tests, the resize case among them
@task_file n=20 path=tests/tui_channel_fds.rs
what: the test of the file descriptors the wrapped child inherits
@task_file n=21 path=tests/security_negatives_server_identity.rs
what: the security negative tests of the server identity check
@task_file n=22 path=tests/security_negatives_socket_dir.rs
what: the security negative tests of the socket directory check
@task_file n=23 path=tests/security_negatives_strict_modes.rs
what: the security negative tests of the strict-modes check
@task_file n=24 path=tests/support/hygiene.rs
what: the shared fixture hygiene check and its violation classes
@task_file n=25 path=tests/support/mod.rs
what: the shared test-support module list
@task_file n=26 path=tests/support/digits.rs
what: the shared test helper that reads whole digit runs from text
@task_item n=1 part=touches
decision/hint-for-every-cause: send's unverified-cli refusal now has its hint, and answer's not-delivered details now have theirs; any owed or excepted cause the row names for these closes.
@task_item n=2 part=touches
contract/cli-exit-codes/12: a human-mode send refused unverified-cli at exit 12 now ends with hint run viola verify for this CLI version.
@task_item n=3 part=touches
contract/viola-send-output/unable: the unable refusal for unverified-cli now carries a hint line last on stderr.
@task_item n=4 part=touches
contract/viola-answer-output/unknown-dialog: the human-mode hint is pinned as that dialog is not pending; viola list shows the current DIALOG.
@task_item n=5 part=touches
decision/hint-last-on-stderr and decision/hints-name-next-verb: the new send and answer hints end the stderr output and name viola verify or viola list.
@task_item n=6 part=touches
decision/no-hint-names-release: the new unverified-cli hint is now checked against it in the refusal-hint cross check.
@task_item n=7 part=touches
contract/viola-send-output/input-not-ready-hint, contract/send-refusals/control-character, contract/send-refusals/empty-text: the send hint strings for these details are now also the answer hints for the same not-delivered details, byte for byte.
@task_item n=8 part=touches
decision/wrapper-repeats-send-refusals: answer's not-delivered refusals now reuse send's hint per detail.
@task_item n=9 part=touches
contract/viola-verify-refusals/fixture-not-clean and contract/viola-verify-output/record: verify --record now refuses an email-shaped token in every payload kind, not only screen text, through the same refusal and check code.
@task_item n=10 part=touches
contract/cli-verbs/run and decision/targets-are-wrapped-names: run's name goes through the same name check as send; the refusal stays a usage error, exit 2, invalid instance name.
@task_item n=11 part=touches
the strict-modes refusal rows (contract/frame-verbs/strict-modes-refused, contract/viola-list/strict-modes-refused, contract/viola-list-json/strict-modes-failed, contract/viola-plugin-install/strict-modes-refused, contract/viola-revive-json/strict-modes-failed, contract/instance-unreachable-detail/strict-modes-failed, contract/run-revive-start/home-tree-refused): a symbolic link at a checked path now reaches these refusals; their text and codes are unchanged.
@task_item n=12 part=touches
contract/mcp-tools/send and contract/mcp-tools/answer: their input types now derive Redact so Debug shows no text or response; the tool schema and wire bodies are unchanged; wait, last and list inputs derive none.
@task_fact n=1 slug=surface-cli
fact: A human-mode answer refused not-delivered ends with a hint for unknown-dialog and for control-character, empty-text, input-not-ready, no-prompt-submitted and turn-running, each the hint send holds for that detail; unknown, a fault, an opaque detail or no detail end with no hint.
@task_fact n=2 slug=surface-cli
fact: send and answer refused unverified-cli both end with the one hint run viola verify for this CLI version.
@task_item n=1 part=untouched
The web surface absence: no page or web output is involved.
@task_item n=2 part=untouched
Board and list layout rows — columns, widths, truncation, colour depth, stale rows, the BAY line, the rack separator — no board output changed.
@task_item n=3 part=untouched
CLI token rendering rows, styling, SGR, glyph, spinner, banner and footer decisions — no styling changed.
@task_item n=4 part=untouched
viola wait, viola last, link and unlink, revive list and start output, plugin install output and wheel verb output rows — none of their lines or refusals changed.
@task_item n=5 part=untouched
The exit-21 frame verb hints and the --json documents and detail codes — no code, field or document shape changed; the --json forms carry no hint line.
@task_item n=6 part=untouched
viola verify step, summary, stop and exit rows other than the record scrub — the verify run itself is unchanged.
@task_item n=7 part=untouched
clap, TTY detection, help and usage rows — no verb, flag or help text was added or retired.
