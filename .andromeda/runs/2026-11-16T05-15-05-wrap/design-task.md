@task_prose part=session
The session widened send confirmation: a driver's send is now confirmed as the driver's own when the submitted prompt equals the typed text, or equals it with a trailing run of ASCII space, TAB, CR or LF removed (the whitespace the CLI is assumed to trim at submit); the trailing-only scope stands PROVISIONAL, the founder's word owed, and a leading trim still claims nothing.
It closed a readiness-gate race: after the gate reads ready, the human-typing and turn-running reads, the link record, send-issued and the paste now run under one lock, so a turn the wrapper learns of before the paste refuses not-delivered turn-running with nothing typed, no link and no send-issued; the refusal order and words are unchanged.
The hook-event claim, the session-start claim, the turn start and end and the unsent-human wheel move are now decided under one lock; the relabel still writes origin driver alone and keeps the submitted text.
The test-only fake agent gained an argv flag --trim-prompt-end that trims the trailing whitespace of its submitted prompt.
No command, flag of the product, output line, refusal word, hint, exit code, event kind, colour, glyph or column was added, renamed or retired; a turn the CLI started whose hook has not yet reached the wrapper stays a named residual, unmeasured.
@task_file n=1 path=crates/viola-agent-claude/src/hook.rs
what: the Claude hook module: the hook payload read and the delivery-confirmation match of a submitted prompt against the typed text (now the trailing-whitespace tolerance)
@task_file n=2 path=src/run/send/slot.rs
what: the send slot: the in-flight send's claim of a submitted prompt, the session-start claim, turn start and end, and the wheel move for a human's unsent prompt
@task_file n=3 path=src/run/send.rs
what: the viola send path inside the wrapper: the readiness gate, the refusal reads (human-typing, turn-running), the link record, send-issued and the paste
@task_file n=4 path=src/run/send/relabel.rs
what: the driver relabel of a prompt-submitted event: writes origin driver when the submitted prompt is the driver's send, and its tables
@task_file n=5 path=src/run/send/test_support.rs
what: the send module's test helpers, including the post-gate-window race harness
@task_file n=6 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent standing in for the Claude CLI, and its argv-only options
@task_file n=7 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's argv option parse tests
@task_file n=8 path=tests/cli_send.rs
what: the booted integration tests of the viola send verb and its readback
@task_item n=1 part=touches
A send readback row for a prompt send (the read-back form): a send whose text ends in whitespace the CLI trims is now read back and confirmed, where before it could not be matched; the readback's what-was-sent wording is to be checked against the submitted text it keeps.
@task_item n=2 part=touches
The viola send readback mirror read-back and unconfirmable rows: what counts as confirmed widened to the trailing-whitespace tolerance, so a send that formerly fell to unconfirmable or unable now reads back.
@task_item n=3 part=touches
Nothing announces success before confirmation, and no optimistic send confirmation: confirmation now rests on a tolerant match (exact, or trailing whitespace trimmed), not on byte equality; the rule still holds but its basis widened, PROVISIONAL.
@task_item n=4 part=touches
The readback box as signature element and the readback box signature check: the box's contents come from a confirmation whose match is no longer exact.
@task_item n=5 part=touches
The refusal hint for not-delivered turn-running: the refusal now also fires for a turn the wrapper learned of between the gate reading ready and the paste; the word and hint are unchanged.
@task_item n=6 part=touches
The refusal hint for human-typing: the human-typing read is now made under the same lock as the paste, after the gate; the word and hint are unchanged.
@task_item n=7 part=touches
The flight-progress strip WHEEL field and the wheel shown as words: a driver's send ending in trimmed whitespace no longer moves the wheel to human; a human's own unsent prompt still takes it; the leading-trim case reads human.
@task_item n=8 part=touches
Pause and release as control transfer, and the handoff transfer marker: the wheel move for a human's unsent prompt is now decided under one lock with the claim; the transfer itself is unchanged.
@task_item n=9 part=touches
Any row that states the send confirmation or the driver-origin relabel as an exact equality of the submitted prompt to the typed text: now stale, the trailing-run tolerance applies.
@task_item n=10 part=touches
The send refusal order as rows state it (gate, then human-typing, then turn-running): unchanged in order, now made under the lock held across the link, send-issued and the paste.
@task_fact n=1 slug=surface-cli
fact: A viola send whose text ends in ASCII space, TAB, CR or LF is read back as the driver's own when the CLI submitted it with that trailing run trimmed; a prompt trimmed at the start is not confirmed and reads the wheel human; the trailing-only scope is PROVISIONAL.
@task_item n=1 part=untouched
Colour palette, colour tokens, colour decision order and colour depth: no colour, SGR span or token changed.
@task_item n=2 part=untouched
Typography, spacing, iconography, glyphs and separators: no glyph, column, width, separator or font rule changed.
@task_item n=3 part=untouched
viola list board and table rows, NAME truncation and skip counts: the board's shape and columns are unchanged.
@task_item n=4 part=untouched
CLI exit codes, exit 21 causes and their --json causes, wrapper fault details and peer reply screening: no code, cause or detail added or changed.
@task_item n=5 part=untouched
viola run, revive, verify, link, unlink, wait and last output and refusal rows: none of those verbs' output or refusals moved.
@task_item n=6 part=untouched
Names of commands: no verb taken, renamed or freed; the fake agent's new flag is test-only and no product name.
@task_item n=7 part=untouched
Brand identity, style direction, expression level, motion and the web or rendered-page absences: no surface or expression changed.
@task_item n=8 part=untouched
Strict-modes refusals, socket-dir refusals and the home-tree check: no start or frame-verb refusal changed.
