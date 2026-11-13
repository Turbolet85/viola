@task_prose part=chunk
Closes in code or tests the rule lines the rows landing owed the route: every refusal reason prints a human hint (unverified-cli and answer's not-delivered details included); the email check reaches every committed fixture kind, the hook, agents and statusline hygiene walks and the verify --record write; run parses its name through the one shared name parser.
Trusted-state strict-modes checks and the private diagnostics open refuse a symlink, and the diagnostics open narrows the opened file rather than a path; the hook and MCP payload types derive veil Redact.
Test-side: the PTY resize test sends no needless key, the fd test always reads the child's live /proc fd table with no self-dump, no OS split and no fd count bound; one shared digit_runs test helper in the test support module replaces three copies.
Re-reads the link snapshot-replay guard as the replay row's standing guard once the budget leg leaves, with its one-off remove-the-guard control run through the gate tool under a kill deadline; no mutation run and no live claude start.
@task_file n=1 path=src/cmd/send.rs mark=edits
what: the send CLI verb and the shared CLI name parser — edits: hint_cause (the refusal-reason to hint map), parse_name
@task_file n=2 path=src/cmd/run.rs mark=edits
what: the run CLI verb, the wrapper start path — edits: its own copy of parse_name, replaced by the shared one
@task_file n=3 path=src/cmd/answer.rs mark=edits
what: the answer CLI verb and its not-delivered refusal output
@task_file n=4 path=src/human/mirror.rs mark=edits
what: the human output writer for refusals and their hint lines — edits: write_unable
@task_file n=5 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the Claude agent crate's capability ledger, its fixture scrub and hook/MCP payload shapes — edits: text_fault, screen_text_fault, has_email, the hook and MCP payload types
@task_file n=6 path=crates/viola-state/src/strict.rs mark=edits
what: the strict-modes check over trusted state (instance, stamps, home paths) — edits: check_instance, check_stamps, check_path, check_read, unix::reading
@task_file n=7 path=crates/viola-state/src/fs.rs mark=edits
what: the state crate's private file helpers, the owner-only append open — edits: open_private_append, restrict
@task_file n=8 path=src/obs.rs mark=edits
what: the bin's diagnostics and process-log setup, the diagnostics files caller — edits: check_diagnostics
@task_file n=9 path=tests/contract_fixture_hygiene.rs mark=edits
what: the recorded-fixture hygiene walk over committed fixtures — edits: the walk filters, claude_fixtures, check_mcp
@task_file n=10 path=tests/support/hygiene.rs mark=edits
what: shared test support for the fixture hygiene walk — edits: the Violation classes (an email class)
@task_file n=11 path=tests/tui_pty_seam.rs mark=edits
what: the PTY seam tests on a real openpty — edits: the resize test's key write
@task_file n=12 path=tests/tui_channel_fds.rs mark=edits
what: the test of the fds a wrapped child inherits — edits: the self-dump fds receipt, the linux cfg split, the fd count assertion
@task_file n=13 path=tests/security_negatives_server_identity.rs mark=edits
what: server-identity security negative tests — edits: fn digit_runs, digit_runs_reads_whole_numbers_only
@task_file n=14 path=tests/security_negatives_socket_dir.rs mark=edits
what: socket-directory security negative tests — edits: fn digit_runs, digit_runs_reads_whole_numbers_only
@task_file n=15 path=tests/security_negatives_strict_modes.rs mark=edits
what: strict-modes security negative tests — edits: fn digit_runs
@task_file n=16 path=tests/cli_links.rs mark=re-reads
what: the snapshot-replay guard that the replay reads the link the wrapper wrote (e5_the_replay_reads_the_link_the_wrapper_wrote)
@task_item n=1 part=needs
The human refusal line and its hint: every refusal reason, unverified-cli and unknown included, ends with a hint line on human stderr; the hint is human output only, never a new refusal reason, detail, exit code or wire field.
@task_item n=2 part=needs
The not-delivered refusal details answer and send can print (empty-text, input-not-ready, no-prompt-submitted and the rest of the closed set) and the hint each one gets.
@task_item n=3 part=needs
The single human writer in the root bin: hint lines go through it, no print macros.
@task_item n=4 part=needs
The unclean-fixture refusal and its email class: the fixture scrub refuses an email in every committed fixture kind and in the verify --record write, next to absolute path, home path and username.
@task_item n=5 part=needs
Recorded fixtures scrubbed and hygiene-walked: the hook, agents, statusline, MCP and screen fixture walks all check emails; screen fixtures keep only signature rows.
@task_item n=6 part=needs
The Claude agent crate stays pure (no file I/O) and holds the Claude payload shapes: the email check and Redact derives land there without moving I/O into it.
@task_item n=7 part=needs
The CLI name check: run, like send, parses the instance name through the one shared parser into the ViolaName newtype; nutype validation errors stay out of crate enums.
@task_item n=8 part=needs
The strict-modes read rule over trusted state (instance folder, ledger stamps, home tree): a symlink is refused, not followed; a path under a regular file stays a strict-modes refusal; the verdicts and strict-modes-failed details keep their shape.
@task_item n=9 part=needs
The private diagnostics files: the open checks modes first, refuses a symlink, sets the private mode before any byte and narrows the opened handle, not the path; codes-only process logs and the detail file keep their lines.
@task_item n=10 part=needs
Indirect through libc as the one OS-binding crate: the no-follow open and handle-based mode change go through libc, no new OS-binding dependency.
@task_item n=11 part=needs
Upstream text and frame params never rendered in Debug: the hook and MCP payload types derive veil Redact as the channel Request does; sanitised error surfaces hold.
@task_item n=12 part=needs
The PTY seam on a real openpty: a resize alone reaches the child and its size receipt follows with no key typed.
@task_item n=13 part=needs
OS-gated tests and Linux only: the fd test drops its OS split and reads the live child fd table always; no test gate names another OS; no fd count assertion.
@task_item n=14 part=needs
Snapshot replay from the log: snapshots read through the replay, replay writes no file and recovers only log-derived fields, links recovered from link events; the link replay guard stands as the replay's guard independent of the budget gate.
@task_item n=15 part=needs
One shared test helper: digit_runs lives once in the test support module, its whole-numbers test beside it; the support module joins the module tree with no orphan.
@task_item n=16 part=needs
Indirect through the operator directive: a guard's remove-the-guard control runs through the gate tool under a kill deadline; no mutation run; no live claude start, real claude CLI only locally and never in CI.
@task_item n=17 part=needs
Tests run against the fake agent, each in its own isolated viola home with homes cleaned by owner.
@task_standing n=1
convention: each new or changed test runs against the fake agent in its own isolated viola home, never a live claude
home: established-decisions · ci-cd-approach
@task_standing n=2
convention: an OS-gated test names its reason; no test gate names another OS
home: cross-cutting-patterns
@task_standing n=3
convention: each new module or test support file joins the module tree (no orphan modules) and passes formatting and clippy with warnings denied and the print and dbg bans
home: build-system
@task_standing n=4
convention: end-to-end test runs leave zero panic lines; insta snapshots in check mode only
home: project-directory-structure
@task_standing n=5
convention: each test-side removal stays outside the working directory and test homes are removed by owner record only
home: occupied-resources
@task_item n=1 part=not_needed
Budget gate, budget thresholds, budget pause and release, budget.json and the statusline wrapper and its source: the chunk removes and changes none of them.
@task_item n=2 part=not_needed
MCP tool results, MCP list, MCP JSON-RPC errors and viola mcp exit codes: the MCP server's wire output is unchanged; only payload type derives move.
@task_item n=3 part=not_needed
Channel methods, channel frames, fault codes and protocol versioning: no new method, field, refusal reason or version bump.
@task_item n=4 part=not_needed
Normalised event kinds, event data per kind and wrapper event appends: no new event kind or field is added.
@task_item n=5 part=not_needed
CLI exit codes and the --json error and exit-21 documents: no exit code or JSON document changes.
@task_item n=6 part=not_needed
Wheel claim, release, take and dialog hold rows: the chunk changes no wheel or dialog behaviour.
@task_item n=7 part=not_needed
Revive, plugin install, pinned copy and child argv and environment rows: none is reached by this chunk's change.
@task_item n=8 part=not_needed
The board and list payload rows and agents listing: the list read is unchanged.
@task_item n=9 part=not_needed
Release, deploy, dist, crates.io publishing and container or hosting absences: nothing ships or deploys.
@task_item n=10 part=not_needed
Fuzz workspace, weekly fuzzing and cargo-deny licence and source rows: no fuzz target or dependency policy changes.
