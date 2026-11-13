@task_prose part=session
The session closed the route-owed rule lines of the rows landing in code and tests, and proved the snapshot-replay link guard by a control; no spec text changed and nothing was retired.
Refusal hints: a human-mode send refused unverified-cli (exit 12) now ends with the hint run viola verify for this CLI version; answer's human refusals for not-delivered now carry the same hint send holds for that detail (2 to 6 details hinted); unknown, a fault or an opaque detail stays unhinted.
Trusted state: the strict-modes check reads the path itself and refuses a symbolic link (Refused::Symlink) before the mode verdict at the home, instances, an instance dir, the snapshot and the stamps; the private diagnostics append open refuses a link (no-follow) and sets mode 0600 through the open handle.
Fixtures: verify --record's scrub now refuses an email-shaped token in every payload kind (existing unclean-fixture code, email check code), and the fixture hygiene walk refuses a planted email as class email on every kind through one shared check.
Debug redaction: the hook and dialog payload types and the MCP send and answer inputs derive veil Redact instead of Debug (redacted payload types 1 to 7); three content-free MCP inputs derive none because veil 0.3.0 refuses a derive with no redacted field; veil (workspace pin =0.3.0, no toggle) added to viola-agent-claude and viola-mcp.
Also: run's name parses through send's one name check (2 to 1 copies, refusal unchanged, exit 2); the resize test sends no key after the resize; the fd test reads the child's live fd table always with no count and no Linux split; one shared digit_runs test helper (3 to 1).
@task_file n=1 path=src/human/hints.rs
what: the human hint lines table — the one line of help that ends a human refusal on stderr, for send and answer
@task_file n=2 path=src/cmd/send.rs
what: the send CLI verb — its refusal-to-hint cause mapping and the one instance-name check of the CLI verbs
@task_file n=3 path=src/cmd/answer.rs
what: the answer CLI verb — its human refusal output
@task_file n=4 path=src/cmd/run.rs
what: the run CLI verb — its arguments, the instance name taken through the shared name check
@task_file n=5 path=crates/viola-agent-claude/Cargo.toml
what: the viola-agent-claude crate manifest — its dependencies (now veil)
@task_file n=6 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger and the verify --record fixture scrub (unclean / is_clean) refusing unclean payload text
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the ledger's unit cases for the fixture scrub checks
@task_file n=8 path=crates/viola-agent-claude/src/hook.rs
what: the Claude hook payload shapes normalised into events (Normalised, Known), their Debug redaction
@task_file n=9 path=crates/viola-agent-claude/src/dialog.rs
what: the Claude dialog payload and answer response shapes (Dialog, Response), their Debug redaction
@task_file n=10 path=crates/viola-mcp/Cargo.toml
what: the viola-mcp crate manifest — its dependencies (now veil)
@task_file n=11 path=crates/viola-mcp/src/tools.rs
what: the MCP server's tool input types (send, answer, wait, last, list), their schema and Debug redaction
@task_file n=12 path=crates/viola-state/src/strict.rs
what: the strict-modes check over the trusted home tree (home, instances, instance dir, snapshot, stamps, socket dir) — the read rule and its refusals
@task_file n=13 path=crates/viola-state/src/fs.rs
what: the private file helpers — the owner-only append open of the diagnostics files and logs, and the lock open
@task_file n=14 path=Cargo.lock
what: the workspace lockfile — the resolved dependency graph (veil edges)
@task_file n=15 path=tests/cross_exit_causes.rs
what: the cross-exit refusal and hint table check — no two codes share a hint, no hint names release, every code a code word
@task_file n=16 path=tests/cli_answer.rs
what: the answer CLI integration tests — human refusal output for unknown-dialog
@task_file n=17 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene walk over committed recorded fixtures (agents, statusline, claude, script, MCP exchange, screen)
@task_file n=18 path=tests/contract_os_gated_tests.rs
what: the OS-gated tests list — each gated test with its gate and its reason
@task_file n=19 path=tests/tui_pty_seam.rs
what: the PTY seam test on a real openpty — resize reaching the child
@task_file n=20 path=tests/tui_channel_fds.rs
what: the inherited file descriptor test — the wrapped child's live fd table
@task_file n=21 path=tests/security_negatives_server_identity.rs
what: the security negatives for server identity verification before the first frame
@task_file n=22 path=tests/security_negatives_socket_dir.rs
what: the security negatives for the socket directory check
@task_file n=23 path=tests/security_negatives_strict_modes.rs
what: the security negatives for the strict-modes check
@task_file n=24 path=tests/support/hygiene.rs
what: the shared fixture hygiene check — violation classes (now email) refused before the schema
@task_file n=25 path=tests/support/mod.rs
what: the shared test support module list
@task_file n=26 path=tests/support/digits.rs
what: the shared test helper digit_runs — whole digit runs read from text
@task_item n=1 part=touches
decision/frame-params-redacted and decision/sanitised-error-surfaces: Debug redaction now reaches beyond the channel Request — the hook and dialog payload types and the MCP send and answer inputs derive veil Redact (redacted payload types 1 to 7); content-free MCP inputs derive none.
@task_item n=2 part=touches
decision/workspace-dependency-pins and the crate-dependency-direction area: veil (workspace pin =0.3.0, no toggle) is now a dependency of viola-agent-claude and viola-mcp; decision/mcp-dependencies and decision/agent-claude-pure read their crate's dependency set.
@task_item n=3 part=touches
decision/banned-features, decision/cargo-deny-bans and decision/sync-crates-tokio-free: veil enters a sync crate (viola-agent-claude) without toggle; cargo deny and the per-sync-crate tokio bans read green.
@task_item n=4 part=touches
decision/strict-read-rule, decision/strict-modes-check-before-log, decision/strict-check-before-log-open, decision/instance-check-before-any-read: the strict check now reads the path itself and refuses a symbolic link before the mode verdict, leaving link and target as found.
@task_item n=5 part=touches
decision/path-under-a-file-is-a-strict-modes-refusal, decision/version-gate-strict-stamps-read, decision/stamps-writer-checks-before-and-under-lock, decision/verify-refuses-untrusted-home-or-stamps: a symlinked home, instance dir, snapshot or stamps file is now a strict-modes refusal; no new variant or message.
@task_item n=6 part=touches
decision/socket-dir-strict-check and decision/socket-dir-check-verdicts: check_read and check_path on a link now refuse (Symlink) where they followed the link before; the socket-dir check still reads the path itself.
@task_item n=7 part=touches
contract/cli-exit-21/strict-modes-failed, contract/run-start-refusals/home-tree, contract/verify-refusals/home-tree, contract/mcp-frame-tools/strict-modes-failed: the same refusal now also fires for a link in the trusted tree; callers print the refusal they already print.
@task_item n=8 part=touches
decision/private-mode-before-write and decision/diagnostics-open-checks-modes: the private append open refuses a link at the path (no-follow, a failed open) and narrows mode 0600 through the open handle, not by path; the lock open is unchanged.
@task_item n=9 part=touches
decision/append-one-write-per-line, decision/events-log-never-truncated, decision/codes-only-role-logs: the events log, heartbeat and role and detail diagnostics files open through the changed append helper; a failed open is the existing dropped line.
@task_item n=10 part=touches
decision/verify-record-only-clean and contract/unclean-fixture-refusal/email: verify --record's scrub now refuses an email-shaped token in any string or key of every non-screen payload, not only the screen; contract/verify-process-exit/unclean-fixture and contract/viola-verify-refusal-codes/unclean-fixture carry it, no new code.
@task_item n=11 part=touches
decision/recorded-fixtures-scrubbed and decision/mcp-tools-list-held-to-recorded-exchange: the fixture hygiene walk gains class email on every fixture kind, the MCP exchange walked through the one shared check with its schema.
@task_item n=12 part=touches
contract/refusal-reason/unverified-cli and contract/cli-exit-codes/12: a human-mode send refused unverified-cli now ends with the hint run viola verify for this CLI version (no hint before); answer carries the same.
@task_item n=13 part=touches
contract/human-stderr/refusal, contract/human-hint-lines/input-not-ready, contract/answer-refusals/unknown-dialog: answer's human not-delivered refusals now end with the hint send holds for control-character, empty-text, input-not-ready, no-prompt-submitted and turn-running; unknown-dialog its own hint; hinted not-delivered details 2 to 6.
@task_item n=14 part=touches
contract/refusal-reason/unknown and contract/refusal-reason/detail: unknown, a fault, an opaque detail or no detail still end with no hint.
@task_item n=15 part=touches
decision/viola-name-instance-key and contract/cli-verbs/send: run takes its instance name through send's one name check (fn parse_name copies 2 to 1); the refusal stays clap's usage error, exit 2, invalid instance name.
@task_item n=16 part=touches
decision/os-gated-tests-name-reason, decision/os-gated-tests-named and decision/no-other-os-test-gates: the fd test's gate row reads unix with reason inherited file descriptors are Unix's; its Linux cfg split is gone.
@task_item n=17 part=touches
decision/human-key-never-held-behind-resize and decision/no-key-held-behind-resize: the resize test now writes no key after the resize and waits on the size receipt alone.
@task_item n=18 part=touches
decision/snapshots-read-through-replay, decision/snapshot-replay-log-fields-only, decision/link-set-follows-log: no code change; the replay's link pass is proven guarded by the test e5_the_replay_reads_the_link_the_wrapper_wrote (control link-replay holds) — a guard cell naming the replay's guard reads it.
@task_item n=19 part=touches
decision/claude-shapes-in-one-crate, decision/hook-output-one-crate, decision/untrusted-upstream-text, decision/content-detail-per-instance: the Claude hook and dialog shapes keep content out of Debug; wire bodies untouched.
@task_item n=20 part=touches
contract/mcp-tools/send, contract/mcp-tools/answer, contract/mcp-tools/wait, contract/mcp-tools/last, contract/mcp-tools/list: input types change derives only (send, answer Redact; wait, last, list JsonSchema alone); tool schemas and results unchanged.
@task_item n=21 part=touches
decision/no-orphan-modules: a new test support module (digits) declared in the support module list; one shared digit_runs body.
@task_fact n=1 slug=cross-cutting-patterns
fact: Every hook, dialog and MCP payload type that carries content derives veil Redact so Debug never reveals content; a payload type with no content field derives none, since veil 0.3.0 refuses a Redact derive with no redacted field.
@task_fact n=2 slug=crate-dependency-direction
fact: viola-agent-claude and viola-mcp depend on veil through the workspace pin (=0.3.0, without its toggle feature).
@task_fact n=3 slug=occupied-resources
fact: The strict-modes check reads each trusted path itself and refuses a symbolic link (home, instances, an instance dir, the snapshot, the stamps) before its mode verdict, leaving link and target as found.
@task_fact n=4 slug=occupied-resources
fact: The private append open of logs and diagnostics files refuses a symbolic link at the path (no-follow) and sets mode 0600 through the open handle.
@task_fact n=5 slug=occupied-resources
fact: verify --record refuses an email-shaped token in any string or key of every recorded payload kind, and the fixture hygiene walk refuses a planted email as class email on every fixture kind.
@task_fact n=6 slug=standard-contracts
fact: A human-mode refusal of send or answer for unverified-cli ends with the hint run viola verify for this CLI version; answer's not-delivered details carry the same hint send holds for that detail.
@task_item n=1 part=untouched
The budget governor rows (thresholds, gate, pause and release, budget.json): no budget behaviour moved.
@task_item n=2 part=untouched
The wheel rows (claim, release, human take, wheel events): untouched.
@task_item n=3 part=untouched
Revive and pinned-copy rows (revive refusals and its --json codes, pinned copy placement and rehash): untouched beyond the shared strict check named in touches.
@task_item n=4 part=untouched
The CI/CD rows (workflows, actions pinning, coverage floors, mutation, fuzz, release via dist): no script, CI step or gate surface changed.
@task_item n=5 part=untouched
The occupied names (binaries, commands, crates, env vars, endpoints): no name taken or freed; the new test helper is no product name.
@task_item n=6 part=untouched
The channel protocol rows (frames, methods, fault codes, protocol version) and the event kind and event data contracts: no wire or event shape changed.
@task_item n=7 part=untouched
The board and list rows and config.json keys: untouched.
