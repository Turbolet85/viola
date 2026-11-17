@task_prose part=chunk
Takes the first live readings on the committed standing rig viola-live-rig, each written down as closed words in the chunk's evidence: R3a a fired scheduled wake's prompt-submit source field, R3b turn-ended's scheduled_wakes read live as 1, R3c the hook-latency window before a send's paste.
Takes R4, a live viola verify --record, whose record home comes from a helper like the tests' record_home() and holds no path component named home.
Reads live the external-imports modal's shape and R2's unexplained box-text holds-text on an empty input box, both provisional.
Fixes the CI configuration: the nextest ci 20 s kill line gains viola-e2e's cli and harness_tied binaries, and harness_lifecycle either has its case's readiness deadline shortened or takes a bound of its own, with the measured wall after the change.
Extends the rig only as far as a reading needs; builds no ledger row, verify probe or wake classification, and re-records no fixture set unless R4's own record home is the reading.
@task_file n=1 path=.config/nextest.toml mark=edits
what: the nextest test-runner configuration, the CI kill line for verify-driven test binaries — edits: the ci profile's second override list, the 20 s kill override (period 10s, terminate-after 2)
@task_file n=2 path=crates/viola-e2e/src/bin/viola-live-rig.rs mark=edits
what: the standing live rig's entry point, the binary that starts a real claude session in an isolated scratch root and records closed words of what it reads
@task_file n=3 path=crates/viola-e2e/src/rig/ mark=edits
what: the standing live rig's arms and isolation (scratch root outside every repository, private config dir, guarded credentials copy, screen kept locally) — edits: the rig's arms, extended only as far as a reading needs (a wake, latency, imports-modal or verify-record arm)
@task_file n=4 path=crates/viola-e2e/src/harness/ mark=edits
what: the e2e test harness's boot of a wrapper and its fake agent, with boot's 20 s readiness deadline — edits: the readiness deadline as a parameter the harness_lifecycle case may shorten without changing the product
@task_item n=1 part=needs
The live viola verify --record run (R4): what the verify verb prints for each step, its summary and fail word, read live and written down as closed words.
@task_item n=2 part=needs
viola verify's stamps.json update and its refusals (stamps update refused, CLI not found, version unreadable, unclean fixture, interrupted run, home-tree check) as a live R4 run may meet them; a stamping on claude 2.1.326 is counted apart from one on 2.1.325.
@task_item n=3 part=needs
The CLI verification state, verified or unverified-cli: PATH claude 2.1.326 has no stamp, so a live start on it runs an unverified CLI; each reading names its binary.
@task_item n=4 part=needs
R4's record home: the home a live verify --record takes holds no path component named home, so the home-tree checks and their refusals stay as the record-home rule leaves them.
@task_item n=5 part=needs
The turn-ended event's scheduled_wakes count read live as 1 (one start, two sends, the wrapper stopped before the wake is due), as viola wait or last would show a turn end.
@task_item n=6 part=needs
The prompt-submit hook payload of a fired scheduled wake and its optional source field (schedule_wakeup, loop_wakeup), captured as closed words; the hook verb's exit code and silent stderr stay as they are while the rig reads it.
@task_item n=7 part=needs
The hook-latency window: a turn the CLI starts whose UserPromptSubmit has not yet reached the wrapper before a send's paste, measured live; the send's not-delivered turn-running refusal is the state the flight lock gives only for a turn the wrapper has learned of.
@task_item n=8 part=needs
The external-imports modal, a CLI-native dialog raised by a cwd whose CLAUDE.md imports a file outside the project: its shape, what closes it and what the start reads if it does not show; no key is typed into it, any key being the founder's.
@task_item n=9 part=needs
The input box's text state on a fresh start (holds-text versus empty), read on R2's first screen, named as read and not yet investigated.
@task_item n=10 part=needs
Every reading is written down only as closed words in a record; no raw screen, path, pid or prompt text leaves the rig's local screen.
@task_item n=11 part=needs
The rig's isolation stands as built: scratch root outside every repository, private config dir, the credentials copy guarded by the token-window read and removed on every exit path.
@task_item n=12 part=needs
This session sets no scheduled wake or timer of its own; R3's wake is set inside the rig's own started session, and the live start cap is the founder's.
@task_item n=13 part=needs
The CI kill line: verify-driven test binaries driving a stamping verify through boot sit under the 20 s kill; harness_lifecycle's slow case is fixed by its own shorter deadline or its own bound, never by letting the kill line end that case.
@task_item n=14 part=needs
Fake-agent tests the chunk adds or changes for the rig or harness end with their test, and report a measured wall after the change.
@task_item n=15 part=needs
No mutation run is taken in this chunk.
@task_item n=1 part=not_needed
The strip-board identity rows (flight-progress strip fields, cocked or dimmed strips, stable session slots, expression level, style direction): no board or strip is drawn or changed.
@task_item n=2 part=not_needed
Colour palette, typography, spacing and iconography rows (amber, dim, bold NAME column, glyphs, separators, column widths): the chunk adds no human output styling.
@task_item n=3 part=not_needed
viola list board and table rows (columns, row order, NAME cut, skip counts, BAY line): no list output is touched.
@task_item n=4 part=not_needed
viola revive and viola run start-refusal rows and the revive --json refusal codes: no start or revive refusal is added or changed.
@task_item n=5 part=not_needed
viola link and unlink output, pause, release and answer verb output: those verbs are not driven or changed.
@task_item n=6 part=not_needed
Peer reply screening and wrapper fault --json detail rows: no wrapper socket reply or fault mapping is changed.
@task_item n=7 part=not_needed
The rendered-page and web-surface absences and the token, squint and swap tests of the self-validation protocol: no page or visual design is made.
@task_item n=8 part=not_needed
A ledger row, verify probe or classification of a wake prompt for a fired scheduled wake keeping the wheel: that is a later entry's build, not this chunk's.
