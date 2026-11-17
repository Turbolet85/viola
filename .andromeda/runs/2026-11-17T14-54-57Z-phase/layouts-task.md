@task_prose part=chunk
Takes the first live readings on the committed standing rig viola-live-rig, each written down as closed words in the chunk's evidence.
R3 reads live a fired scheduled wake's prompt-submit `source` field, turn-ended's scheduled_wakes reading 1, and the hook-latency window before a send's paste.
R4 runs a live viola verify --record whose record home names no path component called home; it also reads the external-imports modal live and looks into R2's unexplained holds-text input-box reading.
One CI-configuration fix: the nextest ci 20 s kill override gains the viola-e2e verify-driven binaries, with the harness_lifecycle case's readiness wait shortened or bounded on its own, and its wall re-measured.
The rig is extended only as far as a reading needs. No ledger row, verify probe or wake classification is built.
@task_file n=1 path=crates/viola-e2e/src/bin/viola-live-rig.rs mark=edits
what: the standing live rig's entry point, the binary that starts a real claude in an isolated scratch root and reads it — edits: its arms (r1/r2 as built; a wake, latency, imports-modal or verify-record arm only as a reading needs)
@task_file n=2 path=crates/viola-e2e/src/rig/ mark=edits
what: the standing live rig's modules: scratch root, private config dir, credentials copy and token-window guard, screen reading, closed-word records
@task_file n=3 path=.config/nextest.toml mark=edits
what: the nextest test-runner configuration, its ci profile's per-binary kill overrides — edits: the second ci override's binary list (the 20 s kill line, period 10s, terminate-after 2), and a bound of its own for harness_lifecycle if that branch is taken
@task_file n=4 path=crates/viola-e2e/src/harness/ mark=edits
what: the e2e test harness that boots a wrapper and fake agent for a test, with boot's 20 s readiness deadline — edits: the readiness deadline the harness_lifecycle never-ready case waits out, if that branch is taken
@task_item n=1 part=needs
viola verify --record run live: its record output and the record home it writes, taken from a helper like the tests' record_home() and holding no path component named home.
@task_item n=2 part=needs
viola verify's refusals a live run can meet before it records: the viola home not private, the claude CLI not found, the CLI version unreadable, the recorded fixture not clean.
@task_item n=3 part=needs
Which claude binary a live start and a live verify run: PATH claude is 2.1.326, which no stamp covers, while 2.1.325 is installed beside it; each reading names its binary, and a 2.1.326 reading and its stamping are counted apart.
@task_item n=4 part=needs
The turn-ended outcome's scheduled_wakes count read live as 1, through viola wait's turn-ended output: one viola run start, two viola send calls, the wrapper stopped before the wake is due.
@task_item n=5 part=needs
A fired scheduled wake's UserPromptSubmit payload, read raw: whether it carries the optional source field (schedule_wakeup, loop_wakeup), on which CLI versions, written down as closed words; the hook drops that field today and no classification of it is built.
@task_item n=6 part=needs
The hook-latency window measured live: the time in which a turn the CLI starts has not yet had its UserPromptSubmit reach the wrapper before a send's paste, the race the flight lock closes only for turns the wrapper already knows of.
@task_item n=7 part=needs
The external-imports modal read live on a viola run start: raised by a cwd whose CLAUDE.md imports a file outside the project with no approval in the rig's private config; its shape, what closes it, what the start reads if it does not show; no key typed into it.
@task_item n=8 part=needs
The input-box state word read on a start's first screen: R2's holds-text with nothing typed, against R1's empty after the trust pass, now investigated.
@task_item n=9 part=needs
viola run's passed start output and its start refusals as the rig meets them on a live start (home tree refused, socket directory refused, already live).
@task_item n=10 part=needs
The 20 s kill line for verify-driven tests: every test binary driving a stamping verify through boot sits under it; cli and harness_tied join as they are, measured slowest PASS 5.991 s and 1.014 s.
@task_item n=11 part=needs
The harness_lifecycle binary under a kill bound: its never-ready boot case (20.644 s, waiting out boot's 20 s readiness deadline) either gets a shorter readiness deadline of its own or the binary gets its own bound, with the wall measured after the change; never added so that the kill line ends that case.
@task_item n=12 part=needs
Boot's readiness deadline as a value a harness case can shorten without changing the product's own deadline.
@task_item n=13 part=needs
The rig's isolation kept as built for every new arm: scratch root outside every repository, private config dir, the credentials copy guarded by the token-window read and removed on every exit path, the screen kept locally, closed words only in a record.
@task_item n=14 part=needs
The viola home a live run is pointed at through the --home global flag, the private home the rig gives a live start and a live verify.
@task_item n=15 part=needs
The founder's calls left to the overseer: the live start cap, any key typed into a CLI-native modal, and a split's placement; every other step is planned on the fake agent. This session sets no scheduled wake or timer of its own.
@task_item n=1 part=not_needed
The web surface: viola renders no page, and nothing here reaches one.
@task_item n=2 part=not_needed
The viola list board and its BAY line, columns, colour depth, truncation, stale rows and --json document: no reading here reads or changes the board.
@task_item n=3 part=not_needed
The CLI token rendering and styling decisions (SGR module, colour roles, spacing, radius, motion, banner, spinner, footer): the chunk adds no human output styling.
@task_item n=4 part=not_needed
viola mcp, its hidden verb and its tools and tool results: no reading goes through the MCP server.
@task_item n=5 part=not_needed
viola revive, its --list, preflight order and refusals: no reading revives a session.
@task_item n=6 part=not_needed
viola link and unlink output: no reading links or unlinks names.
@task_item n=7 part=not_needed
viola plugin install output and refusals: the rig's private config is set up as built, not through plugin install.
@task_item n=8 part=not_needed
viola answer output and the DIALOG word: the external-imports modal is a CLI-native modal no key is typed into, not a viola dialog answered.
@task_item n=9 part=not_needed
The wheel verbs pause and release and their output: the wake that keeps the wheel is a later entry, not built here.
@task_item n=10 part=not_needed
viola --help and per-verb usage output: no verb or flag is added.
