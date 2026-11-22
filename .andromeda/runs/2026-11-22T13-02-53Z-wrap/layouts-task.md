@task_prose part=session
The wrapper's paste of a send's text into the claude child now ends within a compiled bound, PASTE_BOUND of 5 s, set by no config key, env var or flag.
A paste that cannot finish in time is cut: the bracket is closed with no Enter, and viola send is refused not-delivered with the new closed detail word paste-cut and exit 13, with the send's cursor. The wheel does not move.
After a cut, the wrapper holds a cut mark in memory. The next send is refused paste-cut with nothing issued until a human key, a prompt-submitted, a session-start or a session-end clears the mark.
A new provisional refusal hint for paste-cut says a human must look at the session before the next send. The readback mirror gains its struck line for the cut.
The paste now goes through one writer thread fed by a bounded queue that also carries human keys. Human keys are never refused; they wait behind at most one chunk. The shared input-writer lock is retired.
The longest delivered send total moved from 48.5 s to 53.5 s. The fake agent gains an argv-only --stop-reading option. No capability is claimed, and this chunk closes Epoch 4.
@task_file n=1 path=crates/viola-pty/src/pump.rs
what: the PTY pump that copies the child's output and writes human keys and pasted send text into the child; edited to add the bounded paste writer thread and its queue
@task_file n=2 path=crates/viola-pty/src/lib.rs
what: the PTY crate's paste handle and its error kinds; edited: paste takes a bound and gains a paste-cut error
@task_file n=3 path=crates/viola-agent-claude/src/screen.rs
what: the claude agent's compiled screen and timing constants; edited to add the compiled paste bound of 5 s
@task_file n=4 path=crates/viola-core/src/lib.rs
what: the core closed vocabularies, including the not-delivered refusal detail words; edited to add paste-cut
@task_file n=5 path=crates/viola-mcp/src/tools.rs
what: the viola mcp tools and their call deadline; edited: the test that holds the call deadline above the wrapper's send total now counts 53.5 s
@task_file n=6 path=src/human/hints.rs
what: the refusal hint for every cause, the last stderr line of a refusal; edited to add the paste-cut send hint
@task_file n=7 path=src/human/mirror.rs
what: the readback mirror line that echoes a send's issue and outcome; edited to pin the struck line for a paste-cut refusal
@task_file n=8 path=src/run/send.rs
what: the wrapper's send handling and its refusal order; edited to add the paste-cut refusal and the cut-mark rung
@task_file n=9 path=src/run/send/slot.rs
what: the send slot's in-flight record; edited to add the cut mark and the hook events that clear it
@task_file n=10 path=src/run/wheel.rs
what: the wheel slot that tracks human typing and hold; edited to add a count of human key reads
@task_file n=11 path=src/cmd/run.rs
what: the viola run verb that wraps the claude child and pumps its PTY; edited to pass the paste bound
@task_file n=12 path=src/cmd/verify/typed/run.rs
what: viola verify's typed run driver that pastes text into a real claude; edited to pass the paste bound
@task_file n=13 path=src/bin/viola-fake-agent.rs
what: the fake agent binary used by tests in place of claude; edited to add the --stop-reading option
@task_file n=14 path=src/bin/viola-fake-agent/input.rs
what: the fake agent's input reading; edited to add a mode that never reads
@task_file n=15 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's option parsing tests; edited to add the --stop-reading parse case
@task_file n=16 path=tests/cli_send.rs
what: the cross-process tests of viola send; edited to add the four paste-cut and blocked-paste cases
@task_item n=1 part=touches
contract/cli-exit-codes/13 names the not-delivered details under exit 13. paste-cut is a new detail under exit 13.
@task_item n=2 part=touches
A row that lists the closed not-delivered detail words, or send refusal words and their hints, now owes paste-cut. This covers send refusals, the shared refusal word column and the send output rows.
@task_item n=3 part=touches
decision/hint-for-every-cause: the new cause paste-cut has its own provisional send hint. Check also decision/hints-name-next-verb and decision/hints-never-quote-or-locate against the hint: it names the next send and quotes no text or path.
@task_item n=4 part=touches
contract/readback-mirror/issue-and-outcome and contract/readback-mirror/layout: a paste-cut send ends in a struck outcome line in the mirror.
@task_item n=5 part=touches
contract/viola-send-output/unable, contract/viola-send-output/json-mode and contract/send-json/output: a send can now end not-delivered paste-cut. It ends with its cursor when the paste itself was cut, and with no cursor when the cut mark refuses it.
@task_item n=6 part=touches
contract/viola-send-output/input-not-ready-hint: any other paste failure stays input-not-ready with its cursor. Only a cut became paste-cut.
@task_item n=7 part=touches
contract/mcp-tools/send and contract/mcp-tool-results/refusal: the MCP send tool can now return the paste-cut refusal. The MCP call deadline still sits above the wrapper's longest send total, now 53.5 s.
@task_item n=8 part=touches
decision/wrapper-repeats-send-refusals: the wrapper now has a refusal of its own after issue, paste-cut, and a rung that refuses before issue while the cut mark stands.
@task_item n=9 part=touches
decision/turn-ended-not-send-outcome: a turn-ended event does not clear the cut mark. Only a human key, prompt-submitted, session-start or session-end clears it.
@task_item n=10 part=touches
contract/cli-verbs/send: the send verb can now wait up to the paste bound before its outcome. A second send during a blocked paste reads turn-running after that wait.
@task_item n=11 part=touches
contract/cli-exit-codes/json-form: the --json refusal carries detail paste-cut. A client built before the word reads exit 13 with paste-cut shown as the detail and no hint line.
@task_fact n=1 slug=surface-cli
fact: viola send is refused not-delivered paste-cut at exit 13, with its cursor and the wheel unmoved, when the claude child does not take the whole text within the compiled paste bound of 5 s. The bracket is closed with no Enter.
@task_fact n=2 slug=surface-cli
fact: After a paste-cut, every following send is refused paste-cut with no cursor and nothing issued until a human key, a prompt-submitted, a session-start or a session-end. A release with no key and a turn-ended leave the refusal in place.
@task_fact n=3 slug=surface-cli
fact: The paste-cut refusal has its own provisional hint, which tells the human to look at the session before the next send, and its own struck mirror line. The answer hints are unchanged.
@task_item n=1 part=untouched
The web surface absence: viola still renders no page.
@task_item n=2 part=untouched
The viola list board and its BAY line, columns, colour depth, truncation and unwrapped rack rows: no list output changed.
@task_item n=3 part=untouched
The viola revive and the viola run start refusals, and their --json documents: no start path changed.
@task_item n=4 part=untouched
The CLI token rendering rows and the styling decisions (amber, SGR, no spinner, no banner, no progress glyphs): no colour or token was added.
@task_item n=5 part=untouched
viola link and unlink output, viola plugin install, viola last output and viola wait output: none of these verbs' outputs moved.
@task_item n=6 part=untouched
The frame verb exit-21 hints and the instance-unreachable details: no exit-21 cause was added.
@task_item n=7 part=untouched
viola answer output and its not-delivered hints: the answer hint is unchanged.
