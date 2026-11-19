@task_prose part=session
viola verify's human stdout gains one fixed line when a run met the browser-tools prompt at start, written after the stamps are updated and before the summary, which stays the last line.
The line, in the founder's words: the browser-tools prompt stood at start; viola never answers it: answer it once in claude, then run viola verify again. It is plain ASCII, carries no SGR, goes to stdout, and adds no log line; the prompt-absent stdout is unchanged.
The agent ledger gains a public reader that says whether a run met the browser-tools prompt; verify reads it to decide the line.
The test-only live rig gains two arms (vh, the real-home verify; cx, a /context send) with labels L1, L1-2 and L2 and rehearse flags --target-home and --claude-dir; no product verb, flag, exit code, IPC method or env var is added.
Measured live on claude 2.1.328: verify read 24 of 24 relied rows passing under the founder's real home; the pinned product was rebuilt and placed beside the product binary, not switched.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs
what: the relied-row ledger of the claude agent: the check of each relied row of a verify run, and the reading of whether the browser-tools prompt stood
@task_file n=2 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger's unit cases for stamps and the browser-tools prompt reading
@task_file n=3 path=src/cmd/verify.rs
what: the viola verify verb: its steps, its human stdout lines and its summary, the stamp update
@task_file n=4 path=tests/cli_verify.rs
what: the integration checks of viola verify's output: the step lines, the summary, the exit codes, now the browser-tools line before the summary
@task_file n=5 path=tests/support/fake.rs
what: the shared test support of the fake agent: the screens it serves, now the browser-tools prompt screen
@task_file n=6 path=tests/cli_send.rs
what: the integration checks of viola send and viola run output
@task_file n=7 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only live rig: its arms, labels, plans and the summary words it prints
@task_file n=8 path=crates/viola-e2e/src/rig/isolate.rs
what: the test-only live rig's isolation boundary: the child env and the readers of a real home's config by key names only
@task_file n=9 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the test-only viola-live-rig binary: its live and rehearse commands and their flags
@task_file n=10 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the rehearsals of the test-only live rig, each arm against a fake home
@task_item n=1 part=touches
contract/viola-verify-output/exit-0: verify's stdout on a pass now holds one more line before the summary when the browser-tools prompt stood at start.
@task_item n=2 part=touches
contract/viola-verify-output/summary and contract/viola-verify/summary: the summary stays the last stdout line; a new line can now stand directly before it.
@task_item n=3 part=touches
contract/viola-verify-output/step-line: the browser-tools line is a new kind of stdout line of verify beside the step lines, not a step line.
@task_item n=4 part=touches
contract/viola-verify-output/plain-ascii: the new line is plain ASCII with no SGR, under that rule.
@task_item n=5 part=touches
contract/viola-verify-output/exit-1 and contract/viola-verify-output/failing-row: check whether the line's placement before the summary applies on a failing run too; the line is written after the stamps update returns, before the summary.
@task_item n=6 part=touches
decision/human-output-is-contract: verify's human stdout changed shape by one fixed line; the wireframe must show it.
@task_item n=7 part=touches
decision/stdout-results-stderr-rest: the browser-tools line goes to stdout, not stderr, though it reads as guidance.
@task_item n=8 part=touches
decision/hints-name-next-verb and decision/hint-last-on-stderr: the line names a next action (answer once in claude, then run viola verify again) yet is a stdout line, not a stderr hint.
@task_item n=9 part=touches
decision/hints-never-quote-or-locate: the line names no path and quotes no user text.
@task_item n=10 part=touches
decision/words-carry-every-state and decision/no-banner: the prompt state is carried by a fixed word line, not colour or a banner.
@task_item n=11 part=touches
contract/viola-verify-output/help-paragraph and contract/viola-verify-output/usage: no flag was added to verify; check they need no mention of the line.
@task_fact n=1 slug=surface-cli
fact: viola verify's stdout, when the browser-tools prompt stood at start, holds one fixed line before the summary — the browser-tools prompt stood at start; viola never answers it: answer it once in claude, then run viola verify again — plain ASCII, no SGR, the summary still last; the prompt-absent stdout is unchanged.
@task_item n=1 part=untouched
viola list board, BAY line and its colour, width and truncation rows: no board or list output moved.
@task_item n=2 part=untouched
viola send, wait, last, answer, readback mirror and send refusal output rows: no wording or column moved.
@task_item n=3 part=untouched
viola revive and viola run start output, refusals and --json error codes: unchanged.
@task_item n=4 part=untouched
CLI exit codes, global flags, --json mode and internal error rows: no code, flag or document added.
@task_item n=5 part=untouched
viola verify refusals (CLI not found, version unreadable, fixture not clean, home not private, stopped): none moved.
@task_item n=6 part=untouched
MCP server, MCP tools and tool results rows: no tool or result changed.
@task_item n=7 part=untouched
CLI token rendering, spacing, motion, radius and SGR styling rows: the new line uses none.
@task_item n=8 part=untouched
link, plugin install, wheel verb and frame verb hint rows; the web surface and deep links absences: untouched.
@task_item n=9 part=untouched
The test-only live rig's arms, labels and flags are no product CLI verb or flag; no CLI verbs row takes them.
