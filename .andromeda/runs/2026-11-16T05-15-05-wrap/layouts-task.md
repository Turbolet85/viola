@task_prose part=session
The session widened the send's delivery confirmation: a driver's send is now confirmed as its own when the submitted prompt equals the typed text, or equals it with a trailing run of ASCII space, TAB, CR and LF removed (PROVISIONAL, the founder's word owed; leading or inner differences still claim nothing).
It closed the readiness-gate turn race: after the gate returns ready, the human-typing and turn-running reads, the link record, send-issued and the paste run under one flight lock, so a turn the wrapper learns of before the paste refuses not-delivered with turn-running, nothing typed.
A hook event's claim, session-start claim, turn start and end and unsent-human wheel move are now decided under one flight guard; the relabel still writes origin driver alone and keeps the submitted text.
The test-only fake agent gained the argv flag --trim-prompt-end; no verb, flag of viola, exit code, refusal word, event kind, json field or output line was added, removed or reworded.
CI on the pushed head is green 7/7; nothing retired, no count moved.
@task_file n=1 path=crates/viola-agent-claude/src/hook.rs
what: the claude hook payload module - where a submitted prompt is judged the driver's own send (the delivery-confirmation match behind the read back)
@task_file n=2 path=src/run/send/slot.rs
what: the wrapper's send slot - the in-flight send, its claim of a submitted prompt, the turn and wheel state a send reads
@task_file n=3 path=src/run/send.rs
what: the wrapper side of viola send - the readiness gate, the refusal order, the issue and the paste, the confirmation wait
@task_file n=4 path=src/run/send/relabel.rs
what: the driver relabel of a submitted prompt and its tests
@task_file n=5 path=src/run/send/test_support.rs
what: test helpers for the wrapper's send path
@task_file n=6 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent and its argv options
@task_file n=7 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's argv option parse tests
@task_file n=8 path=tests/cli_send.rs
what: the booted integration tests of viola send - its read back, refusals and wheel
@task_item n=1 part=touches
The read back of viola send (contract/viola-send-output/read-back and its /clear and /compact forms): a send whose text ends in whitespace the CLI trims now reads back as confirmed where before it went unconfirmed; the line's shape is unchanged.
@task_item n=2 part=touches
The unconfirmable outcome (contract/viola-send-output/unconfirmable): fewer sends reach it; a row stating that only an exactly equal submitted prompt confirms is now stale.
@task_item n=3 part=touches
The readback mirror rows (decision/readback-mirror-signature, contract/readback-mirror/issue-and-outcome): the outcome line's confirmation now rests on the trailing-run tolerance, not exact equality.
@task_item n=4 part=touches
The readback vocabulary state words (contract/readback-vocabulary/words): any word's meaning that says the submitted prompt equals the sent text exactly.
@task_item n=5 part=touches
The send refusals repeated by the wrapper (decision/wrapper-repeats-send-refusals) and the not-delivered turn-running refusal: the post-gate reads now run under the flight lock held across the link, send-issued and the paste; the refusal order and words are unchanged.
@task_item n=6 part=touches
viola send --json (contract/send-json/output, contract/viola-send-output/json-mode, json-ok): the outcome a trailing-whitespace send reports moves to confirmed; no field changed.
@task_item n=7 part=touches
The send verb and its MCP tool (contract/cli-verbs/send, contract/mcp-tools/send): their stated delivery confirmation, if a cell says an exact match.
@task_item n=8 part=touches
The issue line (contract/viola-send-output/issue-line): it is printed only once the paste ran under the lock; a turn reached before the paste now refuses with nothing issued.
@task_item n=9 part=touches
The send text refusals before any request (contract/send-refusals/control-character, empty-text): a refused character is still refused; nothing is stripped from the typed text - a row tying refusal to the confirmation match is read again.
@task_fact n=1 slug=surface-cli
fact: viola send reads a send back as delivered when the prompt the CLI submitted equals the typed text or equals it with a trailing run of space, TAB, CR and LF removed (PROVISIONAL); any leading or inner difference still leaves it unconfirmed.
@task_item n=1 part=untouched
The web surface absence and the CLI deep-links absence: no page and no URL coupling were added.
@task_item n=2 part=untouched
Styling and token rendering - SGR styling, colour roles, amber, spacing, motion tokens, no spinner or progress glyphs: no output colour or glyph changed.
@task_item n=3 part=untouched
The viola list board, BAY line, columns and the --json list document: the board was not reached.
@task_item n=4 part=untouched
viola revive, viola run starts, plugin install, verify, link and unlink outputs and refusals: none of those verbs changed.
@task_item n=5 part=untouched
CLI exit codes, the internal-error forms, the frame-verb exit-21 hints and detail codes: no exit code, hint or detail word was added or reworded.
@task_item n=6 part=untouched
viola wait, viola last, viola answer and the wheel verbs pause and release: their outputs are unchanged; a human key still takes the wheel and never waits on the lock.
@task_item n=7 part=untouched
The help output, verb groups, flat verbs, clap parsing and the hidden mcp verb: no verb or flag of viola was added.
