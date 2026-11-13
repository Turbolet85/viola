@task_prose part=session
The session closed the rows landing's route-owed rule lines in code and tests, and proved the snapshot-replay guard with a control.
Refusal hints: a human-mode send refused unverified-cli (exit 12) now ends with the hint run viola verify for this CLI version; answer gives that same hint, and its not-delivered refusals for control-character, empty-text, input-not-ready, no-prompt-submitted and turn-running now end with the hint send gives for that detail, word for word; unknown, a fault, an opaque detail or no detail still carry no hint.
Every-refusal-ends-with-a-hint loses its owed divergence: no two codes share a hint and no hint names release, now checked over unverified-cli too.
run's instance name goes through the same name check as send; the refusal stays clap's usage error, exit 2, invalid instance name.
Trusted-state checks now refuse a symbolic link at the home, instances, an instance dir, the snapshot or the stamps file as the existing strict-modes refusal; the private append open for run diagnostics refuses a link and sets 0600 through the open handle.
TUI boundary tests: the resize test sends no key after the resize and waits on the size receipt alone; the fd test waits on the start receipt, reads the live child's fd table on every Unix, and drops its fd count and its Linux-only split; hook, dialog and MCP payload types that carry content derive veil Redact, wire bodies untouched.
@task_file n=1 path=src/human/hints.rs
what: the refusal hint tables for send and answer, one hint per code, the fixed hint strings beside unable
@task_file n=2 path=src/cmd/send.rs
what: viola send, its refusal output and the instance-name check the CLI verbs share
@task_file n=3 path=src/cmd/answer.rs
what: viola answer, its human-mode refusal output (unable line and hint line)
@task_file n=4 path=src/cmd/run.rs
what: viola run, the TUI passthrough's entry and its instance-name argument
@task_file n=5 path=crates/viola-agent-claude/Cargo.toml
what: the claude agent crate's dependencies, now veil
@task_file n=6 path=crates/viola-agent-claude/src/ledger.rs
what: the verify --record fixture scrub that refuses an unclean payload
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the unit cases of the verify --record scrub
@task_file n=8 path=crates/viola-agent-claude/src/hook.rs
what: viola hook's event payload types, their Debug held redacted
@task_file n=9 path=crates/viola-agent-claude/src/dialog.rs
what: the dialog and its answer payload types, their Debug held redacted
@task_file n=10 path=crates/viola-mcp/Cargo.toml
what: the MCP server crate's dependencies, now veil
@task_file n=11 path=crates/viola-mcp/src/tools.rs
what: the MCP server frame tools' input types (send text, answer response) and their strict-modes reads
@task_file n=12 path=crates/viola-state/src/strict.rs
what: the strict-modes check over the home, instances, instance dir, snapshot and stamps, before any write or frame
@task_file n=13 path=crates/viola-state/src/fs.rs
what: the private append open for run diagnostics, heartbeat and event files, mode 0600
@task_file n=14 path=Cargo.lock
what: the workspace lockfile, veil's two new edges
@task_file n=15 path=tests/cross_exit_causes.rs
what: the cross-exit check that no two codes share a hint and no hint names release, now with an exit-12 refusals table
@task_file n=16 path=tests/cli_answer.rs
what: viola answer's integration cases, pinning the unknown-dialog hint by literal
@task_file n=17 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene walk over recorded fixtures, now refusing a planted email
@task_file n=18 path=tests/contract_os_gated_tests.rs
what: the list of OS-gated tests and their gate reasons
@task_file n=19 path=tests/tui_pty_seam.rs
what: the TUI boundary test on a real openpty, the resize reaching the child
@task_file n=20 path=tests/tui_channel_fds.rs
what: the TUI boundary test of the child's inherited file descriptors, 0-2 on the terminal
@task_file n=21 path=tests/security_negatives_server_identity.rs
what: the server-identity negative cases, server-verify-failed refusals
@task_file n=22 path=tests/security_negatives_socket_dir.rs
what: the socket-directory negative cases, socket-dir-refused refusals
@task_file n=23 path=tests/security_negatives_strict_modes.rs
what: the strict-modes negative cases, strict-modes-failed refusals
@task_file n=24 path=tests/support/digits.rs
what: the shared test helper that reads whole numbers out of output text
@task_file n=25 path=tests/support/hygiene.rs
what: the shared fixture hygiene check and its violation classes
@task_file n=26 path=tests/support/mod.rs
what: the shared test helper set
@task_item n=1 part=touches
decision/refusal-ends-with-hint changed: its owed divergence closes; send's unverified-cli and answer's six not-delivered details now end with a hint.
@task_item n=2 part=touches
contract/cli-refusal/hint-line: the hint line now follows send's unverified-cli refusal and answer's not-delivered refusals for the newly hinted details.
@task_item n=3 part=touches
decision/one-hint-per-code and decision/no-hint-names-release: the check now covers unverified-cli at exit 12; answer reuses send's hint for the same detail, byte for byte.
@task_item n=4 part=touches
decision/unknown-refusal-has-no-hint, contract/cli-refusal/unknown-refusal and decision/faults-carry-no-hint: unknown, a fault, an opaque detail or no detail still carry no hint (held, now pinned in the hint tables).
@task_item n=5 part=touches
decision/error-word-detail-hint and contract/cli-refusal/stderr-layout: answer's hint is now chosen by reason and detail, and the send unverified-cli layout gains its hint line.
@task_item n=6 part=touches
contract/send-refusal/input-not-ready and contract/send-refusal/no-prompt-submitted: their hint strings are now shared with answer's not-delivered refusals of the same detail.
@task_item n=7 part=touches
decision/fixed-cli-strings-pinned-by-literal, decision/fixed-cli-strings-pinned and decision/cli-a11y-by-literal-tests: a new fixed hint string and a literal pin of answer's unknown-dialog hint.
@task_item n=8 part=touches
decision/printed-names-are-parsed-arguments and contract/cli-exit-codes/2 (also contract/cli-usage-error/exit-2): run's name now goes through send's name check; refusal stays exit 2, invalid instance name.
@task_item n=9 part=touches
decision/strict-modes-refusal-before-any-write and decision/unstattable-home-is-strict-modes-refusal: a symbolic link at a trusted-state path, or a failed stat, is now refused as strict-modes before the mode verdict.
@task_item n=10 part=touches
contract/start-refusal/home-strict-modes, contract/frame-verb-refusal/strict-modes-failed, contract/hook-call/strict-modes-failed, contract/mcp-frame-tool/strict-modes-failed, contract/revive-json-refusal/strict-modes-failed and contract/instance-unreachable-detail/strict-modes-failed: same refusal text, now also reached by a link; no new variant or message.
@task_item n=11 part=touches
decision/run-diagnostics-off-terminal and contract/tui-passthrough/run-diagnostics: the private append open now refuses a link at the path (a failed open, already a dropped line) and narrows to 0600 through the handle.
@task_item n=12 part=touches
decision/boundary-tests-sequenced-on-records: the resize test now waits on the size receipt alone with no key after it; the fd test waits on the start receipt.
@task_item n=13 part=touches
contract/tui-passthrough/focus-mouse-resize-sequences and decision/wheel-ignores-terminal-events: their resize guard test changed shape (no key follows the resize).
@task_item n=14 part=touches
decision/zero-viola-bytes-on-the-terminal, decision/tui-boundary-assertions-only and decision/tui-boundary-only: the fd test now reads the live child fd table always (0-2 on the pty, no socket, no endpoint or lock), dropping its fd count.
@task_item n=15 part=touches
decision/linux-only: the fd test's OS gate is now unix with the reason inherited file descriptors are Unix's; its Linux-only split is gone.
@task_item n=16 part=touches
decision/single-vendor-content-boundary and contract/tui-passthrough/vendor-content (indirect, through the hook and dialog payload types): content-bearing payload types now derive veil Redact so Debug shows no prompt or answer; wire bodies unchanged.
@task_item n=17 part=touches
decision/json-failure-documents-are-clean (indirect, through the verify --record scrub): an email-shaped token is now refused on every recorded kind, under the existing unclean-fixture code.
@task_fact n=1 slug=cognitive-accessibility
fact: answer's not-delivered refusals with detail control-character, empty-text, input-not-ready, no-prompt-submitted or turn-running end with the hint send gives for that detail, byte for byte; unknown-dialog has its own hint; an opaque or missing detail has none.
@task_fact n=2 slug=cognitive-accessibility
fact: unverified-cli (exit 12) ends with the hint run viola verify for this CLI version on both send and answer.
@task_item n=1 part=untouched
Colour and SGR rows (amber DIALOG, bold NAME, dim stale rows, no SGR off a terminal, word beside every SGR): no colour or escape-sequence output changed.
@task_item n=2 part=untouched
viola list board and caption rows, NAME truncation and the unwrapped separator: the board was not touched.
@task_item n=3 part=untouched
viola --help rows (usage line, group labels, handoff row, served verbs): help output did not change.
@task_item n=4 part=untouched
link and unlink, plugin install and revive --list contracts: none of their output or exit codes moved.
@task_item n=5 part=untouched
Budget gate, config.json keys, dialog hold and time-bound rows: no budget, timeout or config value moved.
@task_item n=6 part=untouched
CI integration, SLO and gate rows (fail-fast gate, zero flakiness, no retries, coverage and doctest): no CI step, gate or threshold changed.
@task_item n=7 part=untouched
Screen-reader targets and manual keyboard or Orca passes, and the scope-summary tier and compliance rows: no manual pass, target or tier changed.
@task_item n=8 part=untouched
Absences (web or graphical frontend, touch surface, rendered page, internationalization, user time limit) and the N/A harness sections: no new surface.
