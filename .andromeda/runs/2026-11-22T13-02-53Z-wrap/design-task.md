@task_prose part=session
The wrapper's bracketed paste of a send's text into the child's terminal now ends within a compiled bound of 5 s, PASTE_BOUND, set by no config key, env var or flag.
A paste the child does not take in time is cut: the bracket is closed, no Enter is typed, the wheel is unmoved, and the send is refused not-delivered with the new closed detail word paste-cut, exit 13, with the send's cursor.
After a cut, every next send is refused paste-cut with nothing typed until a human key is read in the session or a prompt-submitted, session-start or session-end event lands; a release or a turn end alone leaves it refused.
The human's keys are never refused or dropped: they wait in order behind at most one in-flight chunk of the paste and its closing marker.
A new refusal hint for paste-cut (PROVISIONAL wording) and a struck mirror line for it were added; the fake agent gained an argv-only stop-reading option; send's longest delivered total moved from 48.5 s to 53.5 s.
No capability is claimed and no verb, flag, env var, colour or column was added.
@task_file n=1 path=crates/viola-pty/src/pump.rs
what: the PTY pump of viola run: the copy threads and the new single writer thread that types the human's keys and the bounded bracketed paste into the child, edited
@task_file n=2 path=crates/viola-pty/src/lib.rs
what: the PTY crate's paste handle and its error set, gaining the paste-cut error, edited
@task_file n=3 path=crates/viola-agent-claude/src/screen.rs
what: the Claude agent adapter's compiled screen constants, gaining the paste bound of 5 s, edited
@task_file n=4 path=crates/viola-core/src/lib.rs
what: the shared core types holding the closed not-delivered detail words, gaining paste-cut, edited
@task_file n=5 path=crates/viola-mcp/src/tools.rs
what: the MCP frame tools and their call-deadline arithmetic test, the send total now 53.5 s, edited
@task_file n=6 path=src/human/hints.rs
what: the refusal hints of human mode, one hint per refusal word, gaining the not-delivered paste-cut hint, edited
@task_file n=7 path=src/human/mirror.rs
what: the viola send readback mirror and its unable struck line, gaining the paste-cut line pin, edited
@task_file n=8 path=src/run/send.rs
what: the send path of a wrapped run: the send refusal order, the bracketed paste and its not-delivered refusals, edited
@task_file n=9 path=src/run/send/slot.rs
what: the send slot's in-flight state, gaining the in-memory cut mark and its clears, edited
@task_file n=10 path=src/run/wheel.rs
what: the wheel of a wrapped run, who holds the session, gaining a count of human key reads, edited
@task_file n=11 path=src/cmd/run.rs
what: the viola run verb, its attached paste caller now passing the paste bound, edited
@task_file n=12 path=src/cmd/verify/typed/run.rs
what: the viola verify typed run driver, its paste caller now passing the paste bound, edited
@task_file n=13 path=src/bin/viola-fake-agent.rs
what: the fake agent test binary standing in for the CLI, gaining the stop-reading option, edited
@task_file n=14 path=src/bin/viola-fake-agent/input.rs
what: the fake agent's stdin reader, gaining a read-nothing mode, edited
@task_file n=15 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's option parsing tests, edited
@task_file n=16 path=tests/cli_send.rs
what: the cross-process tests of the viola send verb, gaining the four paste-cut and blocked-paste cases, edited
@task_item n=1 part=touches
contract/cli-exit-codes/13: the not-delivered details exit 13 lists now include paste-cut.
@task_item n=2 part=touches
contract/cli-exit-codes/10-14: the exit 13 range's not-delivered detail words gain paste-cut where the cell lists them.
@task_item n=3 part=touches
The refusal-hints rows (refusal-hints/not-delivered-*): a new not-delivered paste-cut hint stands beside them, PROVISIONAL wording, telling the human to look at the session before the next send.
@task_item n=4 part=touches
contract/refusal-hints/refusal-line and contract/cli-stderr/hint-line and contract/cli-refusal-line/hint: one more refusal word carries a hint line in human mode, none under --json.
@task_item n=5 part=touches
contract/viola-send-mirror/unable and contract/send-readback/unable: a paste-cut send ends in the mirror's struck unable line, nothing confirmed, the wheel unmoved.
@task_item n=6 part=touches
decision/no-optimistic-send-confirmation and decision/no-success-before-confirmation: a cut paste is never shown as sent; no Enter is typed and the send is refused.
@task_item n=7 part=touches
decision/hint-never-quotes-text and decision/no-internals-in-errors: the new paste-cut hint names the session and quotes no text and no internal.
@task_item n=8 part=touches
decision/refused-send-folds-into-send-line: the paste-cut refusal is one more refused send that folds into its send line.
@task_item n=9 part=touches
The peer-reply-screening rows (code-word, unlisted-code-word): a client built before the word reads exit 13 with paste-cut shown as the detail and no hint line, not unknown.
@task_item n=10 part=touches
decision/no-hint-for-unknown and decision/no-hint-for-faults: paste-cut is a closed refusal word, not a fault or unknown, so it has a hint.
@task_item n=11 part=touches
contract/flight-progress-strip/wheel and decision/wheel-as-words: a cut leaves the wheel unmoved and writes no wheel record; the wheel only gains an internal count of human key reads.
@task_item n=12 part=touches
decision/pause-release-transfer-control: after a cut a release alone does not clear the refusal; a human key read in the session or a prompt-submitted, session-start or session-end event does.
@task_item n=13 part=touches
contract/cli-callers/llm-driver: the driver's send may now take up to the 5 s paste bound longer, its longest delivered total 53.5 s, and a second send during a blocked paste waits up to that bound then reads turn-running.
@task_item n=14 part=touches
contract/cli-callers/human: the human's keys are never refused during a paste; they wait behind at most one chunk and the closing marker.
@task_item n=15 part=touches
name/command/send: the send verb gains a refusal word and a hint but no flag or output form.
@task_fact n=1 slug=surface
fact: refusal hints · not-delivered · paste-cut: the hint says the session did not take the whole text in time and a human must look at the session before the next send; PROVISIONAL wording; human mode only.
@task_fact n=2 slug=surface-cli
fact: After a paste-cut every next send to that session is refused not-delivered paste-cut with no cursor and nothing typed, until a human key is read in the session or a prompt-submitted, session-start or session-end event lands; a release or a turn end leaves the refusal standing; the mark lives in memory only.
@task_fact n=3 slug=surface-cli
fact: A cut paste closes its bracketed-paste markers and types no Enter; the human's keys are never refused or dropped during a paste, only queued behind at most one in-flight chunk and the closing marker.
@task_item n=1 part=untouched
Brand identity, colour palette, typography, spacing, depth, border radius and iconography: no colour, glyph, font, column or motion changed.
@task_item n=2 part=untouched
viola list board and table rows (columns, NAME width, BAY line, skip counts, row escaping): the board is not touched.
@task_item n=3 part=untouched
exit 21 causes and their --json causes, strict-modes refusals, socket-dir refusals and fixed refusal pairs: no instance-reach refusal changed.
@task_item n=4 part=untouched
viola revive and viola run start refusals and their --json codes: no start refusal changed.
@task_item n=5 part=untouched
viola verify output and refusals: no verify line, summary or refusal changed.
@task_item n=6 part=untouched
viola link and unlink output, viola wait output, viola last output and wrapper fault --json detail rows: untouched.
@task_item n=7 part=untouched
CLI colour decision order, colour depth and colour tokens, and the --help verb groups: no verb, colour token or env var added.
@task_item n=8 part=untouched
The web-surface and rendered-page absences and the self-validation protocol: viola still renders no page.
