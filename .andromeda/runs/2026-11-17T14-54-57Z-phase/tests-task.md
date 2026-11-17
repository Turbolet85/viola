@task_prose part=chunk
Takes the first live readings on the committed standing live rig, each written down as closed words in the chunk's evidence: R3a whether a fired scheduled wake's prompt-submit payload carries the optional source field, R3b turn-ended scheduled_wakes read live as 1, R3c the hook-latency window before a send's paste, R4 the live viola verify --record and its record home.
Also reads live the external-imports modal's shape and R2's unexplained box-text holds-text on an empty input box, naming each as read; the rig is extended only as far as a reading needs.
Fixes the CI kill line: the ci profile's 20 s per-test kill override gains viola-e2e's cli and harness_tied binaries as they are, and harness_lifecycle's slow boot-readiness case is brought under a bound by shortening its own readiness deadline or by a bound of its own, with the measured wall after the change.
@task_file n=1 path=.config/nextest.toml mark=edits
what: the nextest runner profiles and the ci profile's per-test kill bound for verify-driven, stamped-home tests — edits: the second ci override's binary list (the 20 s kill line, period 10s, terminate-after 2) and, if harness_lifecycle takes a bound of its own, an override for that binary
@task_file n=2 path=crates/viola-e2e/src/bin/viola-live-rig.rs mark=edits
what: the one standing committed live rig's entry, starting a real claude session under isolation for local live readings — edits: its arms, which the rig gains only as far as a reading needs (a wake arm, a latency arm, an imports-modal arm, a verify-record arm)
@task_file n=3 path=crates/viola-e2e/src/rig/ mark=edits
what: the standing live rig's modules in the test-only E2E crate: scratch root, private config dir, guarded credentials copy, the locally kept screen and the closed-word record — edits: only as far as a reading needs
@task_item n=1 part=needs
The ci profile's per-test kill line for tests that drive a stamping verify through boot: cli and harness_tied join the 20 s override as they are, so every verify-driven binary sits under the per-test kill bound.
@task_item n=2 part=needs
harness_lifecycle's case whose boot never gets ready waits out boot's 20 s readiness deadline; it is brought under a bound either by shortening that case's own readiness deadline as a parameter, without changing the product, or by a bound of the binary's own, and never by letting the kill line end the case; the measured wall is taken after the change.
@task_item n=3 part=needs
Harness boot's readiness deadline and its readiness-timeout refusal: the boot command's timeout, its readiness failure and the boot deadline not being a performance budget, as the harness_lifecycle case exercises them.
@task_item n=4 part=needs
Root waits stay bounded below the kill lines, and a test that boots a harness session stays tied so it ends with its test when killed.
@task_item n=5 part=needs
R4: a live viola verify --record takes its record home from a helper like the tests' record_home(), one shared copy, never a copied helper; the home holds no path component named home.
@task_item n=6 part=needs
Live verify and fixture recording: fixtures refreshed by live verify, recorded only when no ledger row fails, scrubbed of home paths and usernames, stamps written only by verify, and a dirty payload refusing the recording; no fixture set is re-recorded unless R4's own record home is the reading.
@task_item n=7 part=needs
Which claude a start runs: each reading names its binary; PATH claude 2.1.326 has no stamp while 2.1.325 does, so a reading on 2.1.326 and R4's stamping of it are counted apart, under the unverified CLI version gate and per-CLI-version fixtures.
@task_item n=8 part=needs
R3b: turn-ended event data's scheduled_wakes read live as 1 for one start, two sends and the wrapper stopped before the wake is due, beyond the measured 0 and the synthetic counts.
@task_item n=9 part=needs
R3a: a fired scheduled wake's UserPromptSubmit payload and its optional source field (schedule_wakeup, loop_wakeup), captured as closed words on named CLI versions; the hook tolerates and today drops unknown payload fields, and no classification of a wake prompt is built.
@task_item n=10 part=needs
R3c: the window in which a CLI-started turn's UserPromptSubmit has not yet reached the wrapper before a send's paste; the flight lock closes the race only for a turn the wrapper has learned of; read on the send confirmation and prompt-submitted records.
@task_item n=11 part=needs
The external-imports modal and R2's holds-text input box are read as screen signatures only, the screen kept locally; no key is typed into a CLI-native modal and no blanket dialog approval is given; the start cap is the founder's.
@task_item n=12 part=needs
The standing live rig's isolation as built: a scratch root outside every repository, a private config dir, the credentials copy guarded by the token-window read and removed on every exit path, and the rig never reading the host's claude sessions.
@task_item n=13 part=needs
Real claude is driven locally only: live verify and the rig never run in CI, and a fake-agent run is no proof of real-CLI behaviour, so each live reading is its own evidence.
@task_item n=14 part=needs
Closed words only in a record: no secrets, tokens or prompt text in the evidence, logs or diagnostics the readings produce.
@task_item n=15 part=needs
Rig code stays in the test-only E2E crate, where tokio test code is confined, and no release build carries it.
@task_item n=16 part=needs
R3's scheduled wake is set inside the rig's own started session; this session sets no scheduled wake or timer of its own.
@task_standing n=1
convention: each new test function is named by the test function naming rule and synchronises on logged events, never on sleeps or retries
home: 2-test-strategy
@task_standing n=2
convention: each new test creates its own fresh, private home and seeds its own data
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=3
convention: each new test spawning a long-lived wrapper or harness session holds it in a drop guard so it ends with its test
home: 11-test-anti-patterns-never-do-these
@task_standing n=4
convention: each kill bound or nextest override is set below cargo-mutants' timeout and with no retries in the CI profile
home: bootstrap-phases-derive-for-route-setup-project
@task_item n=1 part=not_needed
MCP server, MCP tool results, MCP invalid-params fields and the MCP fixture tap: no MCP surface is read or changed.
@task_item n=2 part=not_needed
viola revive verbs, their refusal codes, exit codes and documents: no revive path is touched.
@task_item n=3 part=not_needed
viola list board, list --json rows and their control escaping: the readings do not use the board.
@task_item n=4 part=not_needed
Channel link and unlink methods, link records and link verb lines: no linking is read or changed.
@task_item n=5 part=not_needed
Dialog decision bodies, viola answer and its insta snapshots: no dialog is answered and no body changes.
@task_item n=6 part=not_needed
Fuzz targets, corpus replay and property-test generators: no parser surface is changed.
@task_item n=7 part=not_needed
Mutation testing runs, mutation verdict contracts and the epoch-boundary mutation gate: this chunk runs no mutation.
@task_item n=8 part=not_needed
Ledger probe contract suite and a verify probe for a fired wake: that belongs to the entry for a fired scheduled wake keeping the wheel, not here.
@task_item n=9 part=not_needed
Pre-push local gate stages, details and reasons: the pre-push gate is not changed.
@task_item n=10 part=not_needed
Cross-user peer checks, peer euid admission and impostor server rows: no IPC security control is changed.
