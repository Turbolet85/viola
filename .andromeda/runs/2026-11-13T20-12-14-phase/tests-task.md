@task_prose part=chunk
Closes in code or tests the rule lines the rows landing owed the route: every refusal reason prints a human hint (unverified-cli and answer's other not-delivered details included), and run parses its name through the one shared name check.
Brings the email check to every committed fixture kind and to the --record write, adds an email class to the hygiene checker, and has the hook, agents and statusline hygiene walks check emails.
Makes the trusted-state strict-modes checks and the private diagnostics open refuse a symlink, narrowing the opened file rather than the path; the hook and MCP payload types derive veil Redact.
Trims the PTY resize test of its needless key, makes the fd test always read the live child's /proc fd table with no fd-count assertion, and folds three copies of the digit_runs test helper into one shared copy.
Re-verifies the snapshot-replay guard of the link replay as the replay row's guard, with its one-off remove-the-guard control run through the gate tool; no mutation run and no live claude start.
@task_file n=1 path=src/cmd/send.rs mark=edits
what: the send verb and its human hint cause mapping, home of the shared CLI name parse — edits: hint_cause, parse_name
@task_file n=2 path=src/cmd/run.rs mark=edits
what: the run verb's argument handling, holding its own copy of the CLI name parse — edits: its parse_name copy
@task_file n=3 path=src/cmd/answer.rs mark=edits
what: the answer verb and its not-delivered refusal details, whose human lines gain a hint
@task_file n=4 path=src/human/mirror.rs mark=edits
what: the human-mode refusal (unable) line writer, the hint optional today — edits: write_unable
@task_file n=5 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the ledger's fixture scrub checks and the verify fixture recording write — edits: text_fault, has_email, screen_text_fault
@task_file n=6 path=tests/contract_fixture_hygiene.rs mark=edits
what: the fixture hygiene run-time walk over every committed fixture kind — edits: the walk filters, claude_fixtures, check_mcp
@task_file n=7 path=tests/support/hygiene.rs mark=edits
what: the hygiene checker and its violation classes — edits: Violation
@task_file n=8 path=crates/viola-state/src/strict.rs mark=edits
what: the strict-modes check of trusted state (instance, stamps, path and read rules) — edits: check_instance, check_stamps, check_path, check_read, unix::reading
@task_file n=9 path=crates/viola-state/src/fs.rs mark=edits
what: the private owner-only file open used for diagnostics files — edits: open_private_append
@task_file n=10 path=src/obs.rs mark=edits
what: the process diagnostics files setup and its diagnostics path check — edits: check_diagnostics
@task_file n=11 path=tests/tui_pty_seam.rs mark=edits
what: the PTY seam tests on a real openpty, the resize size receipt test
@task_file n=12 path=tests/tui_channel_fds.rs mark=edits
what: the fd test over the live child's fd table and the fake agent's fds receipt
@task_file n=13 path=tests/cli_links.rs mark=re-reads
what: the snapshot-replay guard e5_the_replay_reads_the_link_the_wrapper_wrote, re-run with its one-off remove-the-guard control
@task_file n=14 path=tests/security_negatives_server_identity.rs mark=edits
what: the server identity check negative tests, holding a digit_runs helper copy and its whole-numbers test — edits: digit_runs, digit_runs_reads_whole_numbers_only
@task_file n=15 path=tests/security_negatives_socket_dir.rs mark=edits
what: the socket directory check negative tests, holding a digit_runs helper copy and its whole-numbers test — edits: digit_runs, digit_runs_reads_whole_numbers_only
@task_file n=16 path=tests/security_negatives_strict_modes.rs mark=edits
what: the strict-modes negative tests, holding a digit_runs helper copy — edits: digit_runs
@task_item n=1 part=needs
The human hint cause: every refusal reason and exit cause, unverified-cli and each answer not-delivered detail included, maps to its own hint line in human output.
@task_item n=2 part=needs
Driver-facing hints the chunk adds must never name or suggest release.
@task_item n=3 part=needs
The human refusal (unable) line and its hint, pinned by literals in human-mode output and escaping control characters.
@task_item n=4 part=needs
No new refusal reason, exit code, event kind or wire field: answer and send refusals, exit 12 unverified-cli and the --json documents stay as they are; a hint is a human-output line only.
@task_item n=5 part=needs
CLI names (run, send, answer targets) go through one name check; an agent row or unwrapped name is never a target.
@task_item n=6 part=needs
The fixture scrub: absolute path, home path, username and email checked on every committed fixture kind (hook payloads, MCP exchange, agents, statusline, screen) and a dirty payload refuses the recording whole, with the email refusal code.
@task_item n=7 part=needs
The hygiene checker reports a violation by class only, never the matched bytes; a new email class is proven red by a planted input.
@task_item n=8 part=needs
The fixture hygiene walk reaches every fixture kind, hook, agents and statusline included, for every class.
@task_item n=9 part=needs
Strict-modes write and read rules over the instance dir, stamps and home tree: a symlink is refused (no metadata call that follows a link), and the check runs before any read or open.
@task_item n=10 part=needs
Strict-modes refusals keep their surfaces: exit 21 strict-modes-failed detail, the start exit 1 cause, the hook and MCP frame tool refusals.
@task_item n=11 part=needs
Private diagnostics files: owner-only modes, the path checked before narrowing, a symlink refused, and the mode narrowed on the opened file rather than by path; placement mode-first, umask-independent.
@task_item n=12 part=needs
Payload types derive Redact so secrets never reach logs or diagnostics; the channel request is the existing instance.
@task_item n=13 part=needs
PTY tests on a real openpty: a resize alone produces the size receipt, no key needed; the key-after-resize waits for the new size.
@task_item n=14 part=needs
The fd test reads the live child's /proc fd table always, with no self-dumped fds receipt and no linux cfg split, and makes no fd-count assertion.
@task_item n=15 part=needs
OS-gated test cases: dropping the linux cfg split changes what the OS-gated registry and split binary root must list.
@task_item n=16 part=needs
Unreadable snapshot replayed from the log: the replay row's guard is the link replay test, every snapshot reader going through read-or-replay, the replay writing no file.
@task_item n=17 part=needs
A guard test ships a bounded one-off remove-the-guard witness, run under a kill deadline through the gate tool on a green gate entry.
@task_item n=18 part=needs
One shared copy of each test helper: digit_runs lives once in the shared test support module, its whole-numbers test beside it once.
@task_item n=19 part=needs
No mutation run, local or dispatched, and no live claude start without a per-chunk cap; claude --version is no live start.
@task_standing n=1
convention: every new test follows the test function naming rule
home: 2-test-strategy · test-strategy
@task_standing n=2
convention: no test retries and no sleep synchronisation; waits key on logged events under bounded waits
home: 2-test-strategy
@task_standing n=3
convention: every test creates its own fresh private home and seeds its own data
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=4
convention: test inputs and planted fixture text are synthetic and carry the canary; no real data, emails or paths
home: 7-test-data-fixtures · e2e-test-strategy
@task_standing n=5
convention: test oracles are literals; no comparison by Debug string; no shared mutable state or process-wide env mutation across parallel tests
home: 11-test-anti-patterns-never-do-these
@task_standing n=6
convention: real filesystem and real processes in tests; mocks only at product seams
home: 8-mocking-stubbing-discipline · mocking-stubbing-discipline
@task_standing n=7
convention: one negative test per security control, the symlink refusals included
home: e2e-test-strategy
@task_item n=1 part=not_needed
Budget gate, pause thresholds, budget override and budget.json rows: this chunk removes no budget gate or status line and changes no budget behaviour.
@task_item n=2 part=not_needed
Harness boot, cleanup, status, supervisor and session record contracts: the chunk changes no harness command.
@task_item n=3 part=not_needed
Mutation gate thresholds, cadence, mutants verdict and mutants kill-line rows: no mutation run belongs to this chunk.
@task_item n=4 part=not_needed
Performance budget and hyperfine rows: no hook path timing changes.
@task_item n=5 part=not_needed
Wheel, send confirmation window, local command and dialog decision body rows: the chunk changes none of them.
@task_item n=6 part=not_needed
Revive refusal codes, revive preflight order and revive documents: revive is not touched.
@task_item n=7 part=not_needed
Wait, last and list board contracts: their outputs do not change.
@task_item n=8 part=not_needed
CI workflow rows (SHA-pinned actions, zizmor audit, workflow permissions, artifact retention, concurrency): no workflow file changes.
@task_item n=9 part=not_needed
Fuzz workspace and nightly fuzzing rows: no fuzz target is added or changed.
@task_item n=10 part=not_needed
Fake agent turn script, control-file gating and input framing contracts: the fake agent's input and script behaviour do not change.
