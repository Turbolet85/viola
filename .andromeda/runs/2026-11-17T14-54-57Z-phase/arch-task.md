@task_prose part=chunk
Takes the first live readings on the committed standing rig viola-live-rig and writes each down as closed words in the chunk's evidence: a fired scheduled wake's prompt-submit payload and whether it carries the optional source field, turn-ended scheduled_wakes read live as 1, the hook-latency window before a send's paste, the external-imports modal's shape, and R2's unexplained holds-text input box.
Runs a live viola verify --record whose record home comes from a helper like the tests' record_home and holds no path component named home.
Extends the rig only as far as a reading needs (a wake, latency, imports-modal or verify-record arm), its isolation kept as built.
Fixes the CI nextest kill line: viola-e2e's cli and harness_tied binaries join the ci 20 s override, and harness_lifecycle either has its case's boot readiness deadline shortened or takes a bound of its own, with the measured wall after the change.
Builds no ledger row, verify probe or wake classification, re-records no fixture set beyond R4's own record home, and runs no mutation.
@task_file n=1 path=.config/nextest.toml mark=edits
what: the nextest CI profile and its per-binary slow-timeout kill overrides for verify-driven test binaries — edits: the ci profile's second override list (the 20 s kill line), and possibly a bound of its own for the harness_lifecycle binary
@task_file n=2 path=crates/viola-e2e/src/bin/viola-live-rig.rs mark=edits
what: the standing live rig binary viola-live-rig that starts a real claude in an isolated scratch home and reads it — edits: its arms, extended only as far as a reading needs
@task_file n=3 path=crates/viola-e2e/src/rig/mod.rs mark=edits
what: the live rig's library (isolation, keys, ledger, processes, screen reading) behind viola-live-rig — edits: only what a new reading arm needs
@task_file n=4 path=crates/viola-e2e/tests/harness_lifecycle.rs mark=edits
what: the test harness's own lifecycle tests, driving a stamping verify through boot — edits: the case boot_that_never_gets_ready_times_out_and_stops_its_supervisor, its readiness deadline, if that fork is taken
@task_file n=5 path=crates/viola-e2e/src/harness/boot.rs
what: the test harness's boot, with its 20 s readiness deadline, a parameter a test case may shorten without changing the product
@task_item n=1 part=needs
The UserPromptSubmit hook payload of a fired scheduled wake and its optional source field (schedule_wakeup, loop_wakeup), which viola's hook drops today; read live and named per CLI version, raw shape as closed words.
@task_item n=2 part=needs
turn-ended event data scheduled_wakes, read live as 1 (one start, two sends, the wrapper stopped before the wake is due).
@task_item n=3 part=needs
The send flight lock and the window in which a CLI-started turn's UserPromptSubmit has not yet reached the wrapper before a send's paste; measured, not changed.
@task_item n=4 part=needs
viola verify --record live: fixtures recorded only from a clean verify, the stamps it writes as the one stamps writer, and the record home naming no path component called home; a stamping of 2.1.326 counted apart from the stamped versions.
@task_item n=5 part=needs
Recorded fixtures scrubbed and hygiene-walked, and the unclean-fixture refusal (home-path, username, absolute-path, email), as they bear on R4's live record.
@task_item n=6 part=needs
Which claude a start runs: the child program resolution, the CLI --version first line, and the capability ledger stamps per version; each reading names its binary.
@task_item n=7 part=needs
The external-imports modal of the claude CLI and the rule that viola and verify type into no CLI dialog or trust dialog; any key into it is the founder's, none granted.
@task_item n=8 part=needs
The rig's isolation as built: scratch root outside every repository, private config dir, the credentials copy guarded by the token-window read and removed on every exit path, the screen kept locally, closed words only in a record.
@task_item n=9 part=needs
Real claude CLI probes run only locally, never in CI; the live rig and its readings stay off CI.
@task_item n=10 part=needs
The vt100 screen model's input-box reading (box-text empty versus holds-text) as R2 read it, named as read.
@task_item n=11 part=needs
Verify-driven test binaries held to a 20 s kill line in CI, never by letting the kill line end a case that waits out a deadline by design.
@task_item n=12 part=needs
The test harness boot's 20 s readiness deadline, shortened only for the harness_lifecycle case if that fork is taken, with the product unchanged.
@task_item n=13 part=needs
Test-only binaries kept out of release builds and viola-e2e kept test-only, for any rig arm added.
@task_item n=14 part=needs
Test homes isolated per test, kept or removed by owner record only, and test evidence surviving a killed test, for the bounded binaries.
@task_item n=15 part=needs
No live claude in CI and tests run against the fake agent: every step but the live start, a modal key and a split's placement is planned on the fake agent.
@task_standing n=1
convention: each new or changed test passes under its CI kill-line bound and leaves no process behind its test
home: occupied-resources
@task_standing n=2
convention: each rig arm or reading writes closed words only in a record, never raw upstream text or secrets
home: cross-cutting-patterns
@task_standing n=3
convention: each new code line passes formatting and clippy with warnings denied, no print macros, no orphan modules
home: build-system
@task_item n=1 part=not_needed
The MCP server and its tools, results and stdin caps: no reading or fix reaches the MCP surface.
@task_item n=2 part=not_needed
The wheel, dialog hold, link and unlink recording, and pause or release verbs: nothing here changes how the wheel or links are decided.
@task_item n=3 part=not_needed
The board, list payload and agents listing contracts: no list surface is read or changed.
@task_item n=4 part=not_needed
Revive's refusals, JSON error codes and resume-in-place: no revive path is taken.
@task_item n=5 part=not_needed
The Hyprland seam and window features: nothing touches the compositor.
@task_item n=6 part=not_needed
Release tooling, dist, deployment model and crates.io publishing: no release is built.
@task_item n=7 part=not_needed
Fuzz workspace, nightly fuzzing and fuzz corpus replay: no fuzz target changes.
@task_item n=8 part=not_needed
Channel fault codes, frame envelope and protocol version bump rules: no frame or channel method changes.
@task_item n=9 part=not_needed
Mutation testing at the epoch boundary and the mutants profile: no mutation run here.
@task_item n=10 part=not_needed
Config.json keys, configuration precedence and colour environment: no configuration is added or read differently.
