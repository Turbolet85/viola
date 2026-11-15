@task_prose part=chunk
A test-only corrective chunk that makes a green coverage suite and a green mutation baseline under the mutation scratch reachable; it runs neither a mutation run nor a coverage run as its acceptance, and starts no live claude.
It gives the hook fail-open test's channel endpoint a socket name that does not grow with the pid or the event label, and proves the path stays inside the unix socket path bound with a 7-digit pid under the mutation scratch.
It resizes the mutants test profile's slow-case kill and the receipt wait together (the wait stays strictly below the kill, the kill below cargo-mutants' 20 s auto-timeout floor), re-measuring the slow cases' wall time with the host load beside it.
It kills the ten surviving mutants in the test-only window-recorder binary by tests that fail on each mutated site, each proved by a remove-the-guard control, or states in the test-plan tier text why the recorder's main is outside the tier.
It makes the test-home helper refuse a home given to verify --record that holds a path component named home, so no test can name it wrong.
@task_file n=1 path=tests/hook_fail_open.rs mark=edits
what: the hook fail-open integration test, whose fake channel endpoint is a unix socket under a temp dir — edits: the test channel's socket name and its join and bind
@task_file n=2 path=tests/support/watch.rs mark=edits
what: the test rig's receipt wait, the bounded wait for a log or event receipt shared by every wait_for caller — edits: the WITHIN bound and its doc comment
@task_file n=3 path=.config/nextest.toml mark=edits
what: the nextest test-runner configuration — edits: the mutants profile's slow-timeout kill, and any per-test override for the slow case class
@task_file n=4 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs mark=edits
what: the test-only window-compositor recorder binary of the e2e crate — edits: its tests (main, version_of and texts arms covered), its main only if a test proves a recorder line wrong
@task_file n=5 path=tests/support/home.rs mark=edits
what: the test-home helper every integration test takes its home from (TestHome, the home fixture, scratch) — edits: the home-naming rule for homes given to verify --record
@task_item n=1 part=needs
The hook fail-open test's channel endpoint: a test-only unix socket whose full path, under the mutation scratch dir with a 7-digit pid and the longest event label the cases carry, must stay inside the 107-byte sun_path bound, proved by a computed or asserted bound in the test.
@task_item n=2 part=needs
Restructure invariant for the hook fail-open test: its cases keep proving the hook's fail-open behaviour by their existing assertions; only the test channel's socket name changes.
@task_item n=3 part=needs
The mutants test profile's slow-timeout kill: a margin sized by this chunk from the measured wall time of the ~5 s class (tampered pinned copy, plugin folder rewrite, revive after a killed wrapper, start over an unreadable snapshot replaying the log), below the 20 s cargo-mutants auto-timeout floor.
@task_item n=4 part=needs
The receipt wait bound shared by every wait_for caller: it stays strictly below the mutants profile's kill and moves with it, as a per-call or scaled bound, so the verify-stopped-during-resume case stops timing out in the coverage suite.
@task_item n=5 part=needs
Waits in tests are bounded polls for a receipt in a log or events file, never a fixed sleep; a raised deadline changes the bound, not the wait's kind.
@task_item n=6 part=needs
The window-recorder binary's ten surviving mutants: main's argument guard (--program and --out), its record and record-dispatch arms, version_of's version-reply arm and texts' array arm, each killed by a named test that fails on the mutated site, proved by a one-off remove-the-guard control under a kill deadline.
@task_item n=7 part=needs
Mutation testing scope: obs and test-support code is mutation-tested like product code, and a surviving mutant is a failure unless the test plan's tier text excludes the unit with its reason.
@task_item n=8 part=needs
The record-home naming rule: a test home given to viola verify --record never holds a path component named home, enforced in the shared test-home helper, so the verify home-tree check and its strict-modes refusal are never tripped by a test's own home name.
@task_item n=9 part=needs
Test homes made by the helper stay under the integration-test home root that CI scans and keeps until the gate steps run; the naming rule must not move them out of it.
@task_item n=10 part=needs
verify --record in tests runs against the fake agent only; no real claude CLI is spawned, in CI or locally, by any test this chunk adds or changes.
@task_item n=11 part=needs
No product crate line changes: product logging, events, exit codes and diagnostics stay as they are unless a test proves a line wrong, which then folds in as an inferred item with its own acceptance.
@task_item n=12 part=needs
Behaviours of tools the chunk relies on (nextest override inheritance and the slow-timeout form, tempfile placing its dir under TMPDIR) are measured at P3 and cited, not assumed.
@task_standing n=1
convention: each new or changed test waits for a log or event receipt by a bounded poll, never a sleep-based log wait
home: slo-invariants-telemetry-budgets · 10-slo-invariants-telemetry-budgets
@task_standing n=2
convention: each new test home lives under the scanned integration-test home root and is kept until the gates run
home: ci-integration · 9-ci-integration
@task_standing n=3
convention: each new test that exercises verify uses the fake agent; real-CLI verify never runs in CI
home: span-trace-coverage · 4-span-trace-coverage
@task_standing n=4
convention: each new test's verdict is machine-asserted by its exit, not read by a human
home: 11-obs-anti-patterns-never-do-these
@task_item n=1 part=not_needed
Error capture and panic reporting (the panic line, panic outcome by role, the future error reporter): the chunk adds no panic path and no reporter.
@task_item n=2 part=not_needed
PII scrubbing and redaction (veil, the detail-file line fields, the secret-scan hit report): no logged field or detail record changes.
@task_item n=3 part=not_needed
Metric coverage (derived counters, gauges, histograms, metric grouping): the chunk derives or groups no metric.
@task_item n=4 part=not_needed
The MCP stdio server's tools, frames and refusals: no MCP surface is touched.
@task_item n=5 part=not_needed
Send, wait, wheel and dialog span chains and their log fields: the chunk changes no product send, wait or dialog path.
@task_item n=6 part=not_needed
Trace-context propagation (conn and corr in channel params, connection ids): no correlation value is produced or changed.
@task_item n=7 part=not_needed
Service identity and the process-start line's fields (service name, version, os, pid): unchanged by a test-only chunk.
@task_item n=8 part=not_needed
Heartbeat ticks and liveness (staleness threshold, list liveness): not reached.
@task_item n=9 part=not_needed
OpenTelemetry, exporters, network egress and hosted deployment absences: the chunk adds no dependency and no egress.
@task_item n=10 part=not_needed
Log rotation and log file location rules for product role files: the chunk moves no product log file.
