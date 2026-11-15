@task_prose part=session
The session retired the status-line interception whole: viola no longer hands the agent child a settings override or a --settings flag, no longer reads a status-line source, and no longer runs any shell.
The viola hook statusline arm and its hidden capture probe arm are gone; the word statusline now maps to no hook event and takes the unknown-event exit 0, reading and writing nothing; the hook keeps one capture arm.
viola verify loses its two status-line ledger rows (statusline-override, statusline-shell): the relied set goes 23 to 21, so verify prints 21 step lines and stamps 21 pass; the step counter and summary stay symbolic.
The instance snapshot drops statusline_command (old snapshots holding it still read, the key skipped); every own-format v stays 1, under the widened protocol-version-bump except.
The log schema loses the subject statusline-shell, the hook_event word statusline and the field shell_exit_status; the fake agent loses its statusline-echo mode and statusline receipt, the harness boot its --statusline-echo.
The StatusLine fixtures, their schema and two hook stdin fuzz seeds are deleted; the workspace.repo exceptions of fixture hygiene lose their subject.
@task_file n=1 path=crates/viola-agent-claude/src/statusline.rs
what: the status-line shell-out and per-session settings override builder (deleted): /bin/sh -c of the user's status-line command, the override document
@task_file n=2 path=src/cmd/hook/statusline.rs
what: the viola hook statusline arm and its --capture --shell probe arm with its 5 s deadline (deleted)
@task_file n=3 path=src/cmd/hook.rs
what: the hook verb: maps hook event words to arms, its exit code and silent stderr, the unknown-event exit, the capture arm
@task_file n=4 path=crates/viola-agent-claude/src/hook.rs
what: the agent crate's hook stdin reading for the hook verb
@task_file n=5 path=src/cmd/run.rs
what: viola run: the start path, the instance snapshot it writes, formerly the status-line source read and settings override write
@task_file n=6 path=src/run/mod.rs
what: the child launch of run and revive: the agent child's argv, now --plugin-dir then the caller's args, no --settings
@task_file n=7 path=crates/viola-agent-claude/src/lib.rs
what: the agent crate root holding the claude flag literals (--plugin-dir, --settings)
@task_file n=8 path=crates/viola-agent-claude/src/ledger.rs
what: viola verify's ledger: the relied rows verify steps through and stamps, now 21 rows
@task_file n=9 path=src/cmd/verify.rs
what: viola verify: its step lines, summary line and stamps.json update
@task_file n=10 path=src/cmd/verify/typed.rs
what: viola verify's typed runs and their inputs
@task_file n=11 path=src/cmd/verify/typed/runs.rs
what: viola verify's Runs B, C and D; C and D still pass --settings, Run B's probe settings gone
@task_file n=12 path=src/run/version_gate.rs
what: the CLI version gate reading stamps for the verified or unverified-cli state
@task_file n=13 path=crates/viola-state/src/snapshot.rs
what: the instance snapshot document written and read per instance
@task_file n=14 path=schemas/diag-line.v1.json
what: the log line schema: subjects, hook_event words, process-exit fields
@task_file n=15 path=schemas/claude-statusline-fixture.v1.json
what: the StatusLine fixture schema (deleted)
@task_file n=16 path=fixtures/claude/2.1.287/StatusLine.default-1.json
what: a recorded StatusLine fixture holding workspace.repo (deleted)
@task_file n=17 path=fixtures/claude/2.1.302/StatusLine.default-1.json
what: a recorded StatusLine fixture holding workspace.repo (deleted)
@task_file n=18 path=fuzz/corpus/hook_stdin/statusline-absent
what: a hook stdin fuzz seed for the status-line arm (deleted)
@task_file n=19 path=fuzz/corpus/hook_stdin/statusline-epoch
what: a hook stdin fuzz seed for the status-line arm (deleted)
@task_file n=20 path=fixtures/viola-0.1.0/retired-words.json
what: the list of retired words the compat tests read: snapshot key, ledger rows, hook word, source file, log subject
@task_file n=21 path=fixtures/viola-0.1.0/snapshot.json
what: the 0.1.0-shaped instance snapshot the compat read is tested on
@task_file n=22 path=src/bin/viola-fake-agent.rs
what: the fake agent: its mode words and options, --settings accepted and ignored
@task_file n=23 path=src/bin/viola-fake-agent/statusline.rs
what: the fake agent's statusline-echo mode (deleted)
@task_file n=24 path=crates/viola-e2e/src/bin/viola-harness.rs
what: the e2e harness CLI: boot and its options
@task_file n=25 path=crates/viola-e2e/src/harness/run.rs
what: the harness's expected verify output: the relied rows and the live pass lines [NN/21]
@task_file n=26 path=tests/hook_statusline.rs
what: the status-line hook tests (deleted)
@task_file n=27 path=tests/support/statusline.rs
what: the status-line test support (deleted)
@task_file n=28 path=tests/hook_fail_open.rs
what: the hook verb's fail-open tests, now with the retired word as an unknown event writing nothing
@task_file n=29 path=tests/cli_verify.rs
what: viola verify's output tests: step lines and summary on 21 rows
@task_file n=30 path=tests/cli_instance_state.rs
what: the instance state tests: no settings file in the instance dir, user settings left as found
@task_file n=31 path=tests/tui_passthrough.rs
what: the TUI passthrough check: zero viola bytes in the outer stream
@task_file n=32 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene walk over recorded fixtures
@task_file n=33 path=.config/nextest.toml
what: the test runner config with the ci kill-line filter of test binaries
@task_item n=1 part=touches
The hook verb's event words: statusline is no longer taken; viola hook takes nine events and any other word, statusline included, is an unknown event exiting 0 with nothing read or written; rows naming the hook verb, its arms or a statusline exception (name/command/hook, contract/hook-verb/exit-code, contract/hook-verb/stderr) are touched.
@task_item n=2 part=touches
Any row that lets viola hook write the agent's status line to stdout, or names hook output passed byte-for-byte, loses its subject: the hook writes no status line any more (decision/stdout-data-stderr-messages, contract/cli-streams/stdout, contract/mcp-verb/stdout where it lists the hook).
@task_item n=3 part=touches
contract/cli-colour-decision-order/viola-hook: if its cell names the statusline arm or its passthrough output, that arm is gone.
@task_item n=4 part=touches
Silent catch sites and silent verbs (decision/silent-catch-sites, contract/cli-catch-site/silent-verbs): the statusline hook arm and its probe arm are no longer among the silent paths.
@task_item n=5 part=touches
viola verify output: the relied set is 21 rows, the two status-line rows gone, Run B's probe settings gone; rows on the step line, summary, summary-line, fail word or a stopped run (contract/viola-verify-output/*) are touched where they name a count, a status-line row or a unix-only status-line failure.
@task_item n=6 part=touches
The verified / unverified-cli state (contract/cli-verification/*, contract/refusal-hints/unverified-cli): stamps now hold 21 relied rows; a stamp still holding the two retired row ids reads on the relied rows alone.
@task_item n=7 part=touches
viola run and revive start (name/command/run, contract/viola-run-output/child-started, contract/viola-revive-output/after-child-start, decision/run-prints-nothing-while-wrapped): the child is launched with no settings override and no --settings flag, nothing written to a settings file in the instance dir, the user settings left untouched.
@task_item n=8 part=touches
viola verify refusals over fixtures (contract/viola-verify-refusals/unclean-fixture): the StatusLine fixtures and their workspace.repo exception are gone.
@task_item n=9 part=touches
Any row saying viola shells out, runs a user command through /bin/sh or bounds a status-line deadline: viola now runs no shell at all.
@task_item n=10 part=touches
Any row naming the agent's status line as passed through, wrapped or left exactly as unwrapped: the wrapped session's status line is now the agent's own, by construction.
@task_fact n=1 slug=surface-cli
fact: viola intercepts no status line: no settings override, no --settings on the child, no status-line source read, no shell-out; the agent's status line is its own, exactly as unwrapped.
@task_fact n=2 slug=surface-cli
fact: viola hook takes nine hook event words; the retired word statusline is an unknown event: exit 0, nothing read, nothing written.
@task_item n=1 part=untouched
Brand identity rows (strips, readback box, slots, expression level, phraseology) and the flight-progress strip contracts: no strip, field or wording moved.
@task_item n=2 part=untouched
Colour palette, colour tokens, colour depth and the amber, dim and handoff-blue rules: no colour use changed.
@task_item n=3 part=untouched
Typography, spacing, depth, border radius, rendered-page and web-surface absences: no glyph, column or type rule changed.
@task_item n=4 part=untouched
viola list and its board, columns, NAME truncation, skip counts and row escaping: the board is not reached.
@task_item n=5 part=untouched
Send readback, the send mirror, wait, last, link and unlink output, pause and release: those verbs are not reached.
@task_item n=6 part=untouched
Exit 21 causes, revive --json refusal codes, peer reply screening, wrapper fault detail and the strict-modes refusals: no code or cause added or retired.
@task_item n=7 part=untouched
The Self-Validation Protocol checks and the anti-pattern rows on spinners, emoji, traffic lights and optimistic confirmation: none ruled on a status line.
