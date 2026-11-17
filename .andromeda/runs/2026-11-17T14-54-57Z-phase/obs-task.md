@task_prose part=chunk
Takes the first live readings on the committed standing live rig and writes each down as closed words in the chunk's evidence: whether a fired scheduled wake's prompt-submit hook payload carries the optional source field and on which CLI versions (R3a); turn-ended's scheduled_wakes read live as 1 (R3b); the hook-latency window before a send's paste (R3c); a live viola verify --record and the home it records into (R4).
Also reads live the external-imports modal's shape and re-reads R2's unexplained holds-text input-box reading, both provisional.
Extends the rig only as far as a reading needs (a wake, latency, imports-modal or verify-record arm), its isolation kept as built.
Fixes one CI configuration: the nextest ci profile's 20 s kill line gains viola-e2e's cli and harness_tied binaries, and harness_lifecycle either has its slow case's boot readiness deadline shortened or takes a bound of its own, with the measured wall after the change.
@task_file n=1 path=.config/nextest.toml mark=edits
what: the test runner's CI profile, holding the kill line that bounds verify-driven integration-test binaries at 20 s — edits: the ci profile's second override entry (its binary set), and possibly a bound of its own for one binary
@task_file n=2 path=crates/viola-e2e/src/bin/viola-live-rig.rs mark=edits
what: the standing live rig, the local-only driver that starts a real claude session in an isolated scratch home and keeps the screen local — edits: its arms, only as far as a reading needs
@task_file n=3 path=crates/viola-e2e/src/rig/ mark=edits
what: the live rig's modules: the scratch root, the private config dir, the guarded credentials copy and the closed-word record of a start — edits: an arm a reading needs, where the rig lacks it
@task_file n=4 path=crates/viola-e2e/src/harness/
what: the integration-test harness's boot of a wrapper under test and its readiness deadline, which a test case may shorten without changing the product
@task_item n=1 part=needs
The prompt-submit hook's stdin payload as the wrapper reads it: a fired scheduled wake's payload is captured raw and named in closed words, while the hook today drops the source field; the chunk adds no classification of wake prompts, no new hook stdin field and no ledger row.
@task_item n=2 part=needs
The events stream's turn-ended event and its scheduled_wakes count, read live as 1 after one start and two sends with the wrapper stopped before the wake is due.
@task_item n=3 part=needs
The hook-latency window: the interval between the CLI starting a turn and its UserPromptSubmit hook reaching the wrapper, measured live relative to a send's paste, with the hook's own latency budget and the rule that latency is aggregated outside the process and reported with the gate statistic.
@task_item n=4 part=needs
A live viola verify --record against the real claude: verify is the only writer of capability stamps, its stdout step and summary lines, its exit codes and its process-exit detail are what the reading names; each reading names the claude binary and version it ran, and a 2.1.326 reading is counted apart from the stamped versions.
@task_item n=5 part=needs
The record home a live verify takes comes from a helper like the tests' record_home and holds no path component named home; the home resolution order and the home-tree strict-modes check before log init apply to it.
@task_item n=6 part=needs
Real-CLI verify never runs in CI and CI verify runs against the fake agent only: every live reading here stays local, and any rig arm or test that runs in CI is planned on the fake agent.
@task_item n=7 part=needs
Live readings are recorded as closed words only: no screen content, typed keys, sent text, assistant output, dialog text or CLAUDE environment values reach a record or a log line, and nothing from a local live run is uploaded.
@task_item n=8 part=needs
The rig's credentials copy and the private config dir are secrets territory: never logged, the copy removed on every exit path, and no secret in any artifact.
@task_item n=9 part=needs
The external-imports modal is a CLI-native dialog read live: its shape, what closes it and what the start reads when it does not show; no key is typed into it, and dialog content is never logged.
@task_item n=10 part=needs
R2's holds-text input box reading re-read on the rig and named as read; the wrapper's input-not-ready send refusal and the readiness gate are the surfaces that reading touches.
@task_item n=11 part=needs
The CI kill line binds verify-driven test binaries at 20 s: cli and harness_tied join it as they are, and harness_lifecycle's boot-never-ready case either waits a shortened readiness deadline or its binary takes its own bound, never ended by the kill line; the measured wall after the change is reported.
@task_item n=12 part=needs
Wherever the chunk waits on a live event (a wake firing, a hook line landing, a verify step), it waits on the line or event itself, never on a fixed sleep, and no retry is added to make a reading pass.
@task_standing n=1
convention: each new or changed integration test keeps its home under the single e2e test-home root that the secret scan covers, kept until the gate steps run
home: ci-integration · 9-ci-integration
@task_standing n=2
convention: each diagnostics line a new or changed test makes a wrapper emit is schema-checked against the diagnostic line schema in the test, and gate output never prints a line's content
home: ci-integration · 9-ci-integration
@task_standing n=3
convention: a new test waits on log lines by polling for the line, never by sleeping, and asserts budgets on the maximum sample by exit code
home: slo-invariants-telemetry-budgets · 10-slo-invariants-telemetry-budgets
@task_standing n=4
convention: a new test's log file is named after the test that writes it
home: obs-scope-summary · 1-obs-scope-summary
@task_item n=1 part=not_needed
The MCP stdio server's tools, frames, stdin limits and isError results: no reading or fix here reaches the MCP role.
@task_item n=2 part=not_needed
The channel JSON-RPC methods, params, error codes, link and unlink records and their corr or conn joins: nothing here changes the channel or its lines.
@task_item n=3 part=not_needed
Panic capture, panic outcome by role, the panic line and the detail-file error line schema: no panic path or error-capture code is touched.
@task_item n=4 part=not_needed
Span names, span attributes and span chains of run start, send, wait, wheel and dialog paths: no instrumented path gains or loses a span.
@task_item n=5 part=not_needed
Revive preflight, revive --list and its detail codes: no reading starts or lists a revive.
@task_item n=6 part=not_needed
Heartbeat file, liveness transitions and list liveness thresholds: not read or changed by any reading.
@task_item n=7 part=not_needed
A future error reporter, OTel SDK or exporter, OTLP receiver and in-process metrics pipeline: absences and future-only rules nothing here adopts.
@task_item n=8 part=not_needed
The exit-1 and exit-21 detail-code tables of start and frame verbs and their cause words: no exit cause is added or changed.
