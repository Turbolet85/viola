@task_prose part=chunk
Closes in code or tests the rows landing's route-owed rule lines: a hint on every refusal reason, the email check on every fixture kind and at the fixture record write, run's name through the shared name check, a symlink refused by the trusted-state strict-modes checks and by the diagnostics open, and the hook and MCP payload types deriving veil Redact.
Test-side it walks the hook, agents and statusline fixtures for emails, gives the hygiene violation an email class, drops a needless key from the PTY resize test, makes the fd test read the live child fd table with no count assertion, and makes the three digit_runs copies one shared test helper.
It re-runs the snapshot-replay guard that reads the link the wrapper wrote, with a one-off remove-the-guard control through the gate tool, as the replay row's guard once the budget leg leaves; it removes no budget gate.
Every coordinate is re-tested at HEAD; no mutation run and no live claude start; adds no refusal reason, exit code, event kind or wire field.
@task_file n=1 path=src/cmd/send.rs mark=edits
what: viola send, a CLI frame verb, its refusal output and the shared CLI name parse through the viola name check — edits: hint_cause (the unverified-cli and unknown causes print no hint), parse_name (made the one shared copy)
@task_file n=2 path=src/cmd/run.rs mark=edits
what: viola run, the start of a wrapped instance, which parses the instance name — edits: its own copy of parse_name, replaced by the shared one
@task_file n=3 path=src/cmd/answer.rs mark=edits
what: viola answer, a CLI frame verb whose not-delivered refusal details are printed to the human — edits: the not-delivered details that print no hint
@task_file n=4 path=src/human/mirror.rs mark=edits
what: human-mode output, the refusal line and its hint on stderr — edits: write_unable (the hint made always present)
@task_file n=5 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the recorded fixture ledger and its scrub check, the fixture record write and the hook and MCP payload types — edits: text_fault (gains the email check), has_email, screen_text_fault, the --record write, the hook and MCP payload types (derive Redact)
@task_file n=6 path=crates/viola-state/src/strict.rs mark=edits
what: the strict-modes check on trusted state — edits: check_instance, check_stamps, check_path, check_read, unix::reading (stops following a symlink)
@task_file n=7 path=crates/viola-state/src/fs.rs mark=edits
what: owner-only file helpers under the viola home, the private append open — edits: open_private_append (refuses a symlink, narrows the open file rather than the path)
@task_file n=8 path=src/obs.rs mark=edits
what: the diagnostics files and their owner-only, fail-open open — edits: check_diagnostics
@task_file n=9 path=tests/contract_fixture_hygiene.rs mark=edits
what: the recorded fixture hygiene walk over every committed fixture kind — edits: the walk filters, claude_fixtures, check_mcp (email check extended to the hook, agents and statusline walks)
@task_file n=10 path=tests/support/hygiene.rs mark=edits
what: the shared fixture hygiene checker for tests — edits: Violation (gains an email class)
@task_file n=11 path=tests/tui_pty_seam.rs mark=edits
what: the PTY seam test on a real openpty — edits: the resize test (the pty.write key goes)
@task_file n=12 path=tests/tui_channel_fds.rs mark=edits
what: the channel handle inheritance test on the child's fd table — edits: the agent's fds self-dump receipt, the linux-only split, the fd count assertion (go); the /proc fd table assertions stay
@task_file n=13 path=tests/support/mod.rs mark=edits
what: the shared test support module — edits: the one shared digit_runs helper and its digit_runs_reads_whole_numbers_only test
@task_file n=14 path=tests/security_negatives_server_identity.rs mark=edits
what: the server identity security negative tests — edits: its digit_runs copy and its digit_runs_reads_whole_numbers_only test (go)
@task_file n=15 path=tests/security_negatives_socket_dir.rs mark=edits
what: the socket-directory security negative tests — edits: its digit_runs copy and its digit_runs_reads_whole_numbers_only test (go)
@task_file n=16 path=tests/security_negatives_strict_modes.rs mark=edits
what: the strict-modes security negative tests — edits: its digit_runs copy (goes)
@task_file n=17 path=tests/cli_links.rs mark=re-reads
what: the snapshot-replay guard e5_the_replay_reads_the_link_the_wrapper_wrote, the replayed snapshot read of the link the wrapper wrote, with its one-off remove-the-guard control
@task_item n=1 part=needs
The recorded fixture scrub: a recorded fixture is scrubbed or refused whole, the email class included, on every committed fixture kind and at the --record write.
@task_item n=2 part=needs
The unclean-fixture refusal codes and line form, the email code among them, as the fixture scrub check now raises them on every fixture kind.
@task_item n=3 part=needs
Recorded fixture hygiene and fixtures reviewed before commit: the hook, agents, statusline, MCP and screen fixtures all walked for absolute paths, user words and emails.
@task_item n=4 part=needs
CLI names through the viola name check and through the validated name type: run's instance name parsed by the one shared parse.
@task_item n=5 part=needs
The strict-modes check on trusted state and its read rule: the instance, stamps, path and read checks refuse a symlink instead of following it, before any trusted read or narrowing.
@task_item n=6 part=needs
The strict-modes refusal message for a symlink and its refusal surfaces (CLI frame verbs, run and revive, verify, hook, mcp), reached when the checks now refuse a link.
@task_item n=7 part=needs
Private diagnostics files: the diagnostics open refuses a symlink and narrows the open file to owner-only, closing the check-then-open-then-chmod race.
@task_item n=8 part=needs
A refused diagnostics open fails open and writes nowhere; diagnostics files failing strict modes fail open, which the hardened open must keep.
@task_item n=9 part=needs
Owner-only file modes under the viola home and explicit mode before any byte, which the private append open still sets.
@task_item n=10 part=needs
Refusal output to the human: every refusal reason, unverified-cli and answer's not-delivered details included, prints a hint, a human-output line only; the hint carries no path, pid, id or upstream text and escapes control characters.
@task_item n=11 part=needs
No new refusal reason, exit code, event kind or wire field: the CLI exit codes, channel refusals, send and answer refusal words and MCP structured content stay as they are.
@task_item n=12 part=needs
Hook payload is content and user content only to diagnostics: the hook and MCP payload types derive Redact so their content never reaches a log in clear.
@task_item n=13 part=needs
No channel handle inheritance: the fd test reads the child's live fd table under /proc on every run, with no count bound.
@task_item n=14 part=needs
Replayed snapshot is inert and yields nothing, its replay read guarded by the link test once the budget leg's replay guard leaves; only the instance's wrapper writes its snapshot.
@task_item n=15 part=needs
Test seams only under the fake-agent feature and compiled out of release: the fd test's dropped self-dump receipt and any seam the tests lean on stay test-only.
@task_item n=16 part=needs
Mutation testing at the epoch audit, not here: the guard's remove-the-guard control runs once through the gate tool, no mutation run local or dispatched.
@task_item n=17 part=needs
Real claude CLI only locally and CI holds no secrets: no live claude start in this chunk, the tests run on the fake agent.
@task_item n=18 part=needs
Tests use their own home and test-side removal only of their own dirs, for every changed or new test.
@task_standing n=1
convention: each new or changed test uses its own home under a test-created directory, never the user's viola or Claude home, and removes only what it created
home: secret-management · security-anti-patterns
@task_standing n=2
convention: each committed or recorded fixture is scrubbed of absolute paths, user words and emails, or refused whole, and walked by the hygiene test
home: data-protection · security-anti-patterns-never-do-these
@task_standing n=3
convention: each test seam stays under the fake-agent feature and out of the release build
home: security-anti-patterns · input-validation
@task_item n=1 part=not_needed
Dependency security, CI workflow pins, cargo-deny, zizmor, fuzzing and release rows: the chunk adds no crate, workflow, action or release step.
@task_item n=2 part=not_needed
The budget gate, budget.json writers and the statusline shell-out: their removal belongs to the Epoch 2 entries and this chunk leaves them as they stand.
@task_item n=3 part=not_needed
Socket directory, socket bind, chmod-after-bind and peer euid checks: the chunk changes no listener, endpoint or peer check.
@task_item n=4 part=not_needed
Paste text and dialog free-text character sets, bracketed paste and delivery confirmation: no typed-text rule changes.
@task_item n=5 part=not_needed
Pinned binary placement, re-hash and plugin file rewriting: the chunk touches no pinned copy or plugin install.
@task_item n=6 part=not_needed
Child environment stripping, credential strip and hermetic pre-push environment: no child spawn or environment change.
@task_item n=7 part=not_needed
Channel frame caps, JSON depth limits, MCP stdin line caps and bounded readers: no reader bound changes.
@task_item n=8 part=not_needed
Revive arguments, revive --json codes and the collision check: revive's behaviour is not changed.
@task_item n=9 part=not_needed
Threat model absences (hosting, network listener, accounts, secrets, database, uploads, webhooks): none is touched.
