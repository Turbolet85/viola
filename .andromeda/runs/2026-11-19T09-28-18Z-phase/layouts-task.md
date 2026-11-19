@task_prose part=chunk
Lands a new recorded fixture set for claude 2.1.328, recorded by a clean live viola verify --record under the rig's private config plus the viola-mcp-tap re-record, landed whole as a plain copy of the recorders' output.
Points the ledger probes' STAMPED at the new set, so the drift suites and the recorded-exchange suites read over it.
Re-reads the 2.1.325 set's plan and report as ground, and settles whether the rig's R4 / R4M labels or the fake agent need any change for 2.1.328.
Disposes each of the entry's three carries by name, including where the half-unread /context post-condition on 2.1.328 is read from a readable transcript or where it goes.
@task_file n=1 path=fixtures/claude/2.1.328/
what: the recorded fixture set for claude 2.1.328 — the recorded exchanges and screens viola verify --record writes, through the recorder's scrub, replayed by the drift and recorded-exchange suites
@task_item n=1 part=needs
viola verify --record on claude 2.1.328: the recording run that writes the fixture set, its step lines, summary and --record output as viola verify prints them.
@task_item n=2 part=needs
The recorded fixture must be clean: a payload the recorder's hygiene refuses is a red (the recorded-fixture-not-clean refusal), never scrubbed by hand.
@task_item n=3 part=needs
viola verify's refusals the recording run can meet: claude CLI not found, CLI version unreadable, viola home not private, verify stopped — the run must finish with none of them.
@task_item n=4 part=needs
viola verify's exit 0 and summary line on the recording run: a clean record is what lands; an exit 1 or a failing row lands nothing.
@task_item n=5 part=needs
The viola home the recording uses is the rig's private config, given by path (the --home global flag), never the operator's real home.
@task_item n=6 part=needs
The ledger probes' STAMPED value names the new 2.1.328 set; the drift suites and recorded-exchange suites replay that set against the fake agent and must stay green.
@task_item n=7 part=needs
The /context send's read back on 2.1.328: the stdout-tag half of its post-condition, read from a readable transcript beside the recording, or named as leaving with its next reader.
@task_item n=8 part=needs
The send outcome not-delivered (no prompt submitted) as the rig's L2 send read it on 2.1.328, which left the /context read back unverified.
@task_item n=9 part=needs
Any 2.1.328 difference the recorded screens carry — the trust dialog's new decline text, a trust prompt only with --add-dir — and whether the fake agent or the recorded-screen replay must account for it.
@task_item n=10 part=needs
The viola-mcp-tap re-record (R4M): the recorded MCP exchange for 2.1.328, landed under the same hygiene as the verify recording.
@task_item n=11 part=needs
Every test start runs under a test-owned user home; a test that waits takes a bound under the 20 s lint; every process started is ended; no scheduled wake or timer.
@task_item n=12 part=needs
Every live start of the chunk waits on the founder's cap word; this phase makes none and reads nothing of the operator's real config.
@task_standing n=1
convention: each recorded fixture file comes from the recorder unedited, passes viola verify's clean-fixture check, and is replayed by the existing suites rather than asserted by hand
home: surface-cli
@task_item n=1 part=not_needed
The web surface: viola renders no page, and a fixture set adds none.
@task_item n=2 part=not_needed
CLI token rendering and styling (colour roles, SGR module, spacing, motion, radius): the chunk records and replays, it changes no styling.
@task_item n=3 part=not_needed
The viola list board and its BAY line, columns, truncation, escaping and --json document: no board code or its output changes.
@task_item n=4 part=not_needed
viola revive, its --list rows, refusals, preflight order and --json error codes: the chunk makes no revive start.
@task_item n=5 part=not_needed
viola link / unlink output and viola plugin install output: neither verb is touched by a recording or a stamp.
@task_item n=6 part=not_needed
The --help and per-verb usage output, the verb grouping and argument parsing choices (flat verbs, clap derive, argument order): no verb is added or renamed.
@task_item n=7 part=not_needed
The wheel verbs (pause, release) and their spoken output: no wheel or handoff behaviour changes.
@task_item n=8 part=not_needed
Hint wording rules (a hint for every cause, hints name the next verb, no hint names release): no new refusal or hint is added.
