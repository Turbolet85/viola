@task_prose part=chunk
Adds a CLI verb `viola adopt <session-id> --name <n> [--parent <p>]`: a plain claude session named by its session-shaped id resumes under a viola wrapper with the given name and parent, then stands on the board and is driven like any wrapped session.
It is built on revive's start path: the resume argv, the `claude agents --json` preflight with its closed session-live refusal, and the start shared with run.
A malformed id is refused by the closed session-id check with a closed word; a session live in another claude is refused before any child spawns, and no running process's terminal is ever taken over.
Folded CI item: it re-measures the two env-capture security-negative cases (one killed at the 20 s line on the runner) against their kill line, locally and under coverage, and gives them a margin by their class or reduced work, never by raising the kill line for every test.
Nothing live in this phase: every live reading of adopt is gated on the operator.
@task_file n=1 path=src/cmd/revive.rs
what: the revive verb's start path — the resume argv, the claude agents preflight and its session-live refusal, the start shared with run — that adopt is built on
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: the claude agent crate's resume argv builder (resume_args) for `--resume <id>`
@task_file n=3 path=.config/nextest.toml mark=edits
what: the nextest test configuration — the per-test kill line and test classes — edits: the class or override for the two env-capture security-negative cases
@task_file n=4 path=tests/security_negatives_env_capture.rs mark=edits
what: the env-capture security-negative suite — edits: the cases env_name_with_the_user_word_reads_fail_and_records_nothing and kept_name_reaches_no_stamp_or_record
@task_item n=1 part=needs
A new served verb `adopt` in `viola --help`: its usage line, its row under the right group label, served verbs only.
@task_item n=2 part=needs
Adopt's refusals in the human form: the unable line with its kebab-case code word beside a plain word, then exactly one hint line; refusal and failure words never coloured; stderr layout pinned by literal.
@task_item n=3 part=needs
Adopt's refusals under `--json`: one clean document with no hint, the exit-1 refusal document shape that revive's start refusals use (session-live, already-live, squatted-name, strict-modes-failed, socket-dir-refused, internal-error and the like), and a new closed code word for a malformed session id.
@task_item n=4 part=needs
The session-live refusal reached through revive's preflight: a session live in another claude is refused before any child spawns, with a hint to end it by hand; the claude agents read and how it behaves when that read fails.
@task_item n=5 part=needs
The closed session-id check: a malformed id is refused with a closed word before any home read beyond the strict ones; strict-modes and socket-directory refusals come before any write or frame.
@task_item n=6 part=needs
Exit codes adopt returns: usage errors (exit 2) for missing or bad arguments, exit 1 for start refusals, the fault exit; each refusal's code word fixed by literal.
@task_item n=7 part=needs
Printed names are the parsed arguments: the name, parent and session id adopt prints are the operator's own arguments, with C0/C1 control characters escaped in every output line; no paths carried in its output.
@task_item n=8 part=needs
No SGR off a colour-capable terminal and none in plain or `--json` modes; any SGR span only beside its word.
@task_item n=9 part=needs
Linear static CLI output with no spinner and no interactive input: adopt asks the operator nothing on the terminal.
@task_item n=10 part=needs
The wrapped session's TUI passthrough once adopt's child is running: the terminal belongs to the child, zero viola bytes on the terminal, human keystrokes never blocked, run diagnostics never on the terminal — the same entry as revive's.
@task_item n=11 part=needs
Time bounds fall on the driver only, never on the human: any wait adopt's start makes (preflight read, start verify stop grace) is bounded off the human.
@task_item n=12 part=needs
Errors as word, detail and hint: no stack trace or bare status as error text, thiserror messages interpolate nothing, faults carry no hint, the unknown refusal carries no hint, and no hint names release.
@task_item n=13 part=needs
The two env-capture cases' wait: say from the code and the nextest config what each waits on and its time locally and under coverage, then hold a measured margin against the kill line by class or reduced work — zero flakiness, no retries, the kill line not raised for every test.
@task_item n=14 part=needs
Live readings never gate: every clause of adopt the fakes cannot carry (ending a real session, real claude starts) is planned gated on the operator and stays out of the CI gate.
@task_standing n=1
convention: each new a11y assertion lives inside an existing test suite, not a separate a11y driver or suite
home: 1-a11y-scope-summary · bootstrap-phases-derive-for-route-setup-project
@task_standing n=2
convention: each new fixed CLI string (help row, unable line, hint, code word) is pinned by its literal in a test and comes from design copy
home: 8-cognitive-accessibility · cognitive-accessibility
@task_standing n=3
convention: each new a11y check is a nextest failure inside the coverage suite in the GitHub Actions push/PR workflow, fail-fast, with no own artifact
home: 9-ci-integration · bootstrap-phases-derive-for-route-setup-project
@task_standing n=4
convention: each new test is deterministic — zero flakiness, no retries, a gate breach fails the build
home: 10-slo-invariants-a11y-budgets
@task_item n=1 part=not_needed
The board's visual rules — colour, dim on stale rows, bold names, NAME truncation, caption order, the unwrapped separator: adopt adds an instance to the board but does not change how the board renders.
@task_item n=2 part=not_needed
Link and unlink, plugin install, verify, wait, send and answer surfaces and their exit codes 10–21: adopt neither uses nor changes them.
@task_item n=3 part=not_needed
Dialog pending and answer, send readback and the send-refused-while-typing rule: adopt sends nothing to the session and answers no dialog.
@task_item n=4 part=not_needed
Hook event and dialog calls, and the instance-unreachable details of frame verbs: adopt makes no hook call and drives no frame verb of its own.
@task_item n=5 part=not_needed
Manual Orca and keyboard passes and their screen-reader setups: supplemental, they gate nothing in this chunk.
@task_item n=6 part=not_needed
Absences — web or graphical frontend, touch surface, rendered page, internationalization, user time limits: nothing in adopt adds any of them.
@task_item n=7 part=not_needed
Structured violation JSON for a11y rows and WCAG mapping: no new violation report shape is added.
