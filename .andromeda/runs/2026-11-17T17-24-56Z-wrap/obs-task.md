@task_prose part=session
Test-only session, in the viola-e2e crate alone: no product crate, no product file, no dependency, IPC method, env var or log field was added or changed.
The standing live rig gained a hidden capture verb that writes the real CLI's UserPromptSubmit payload as one line holding only key names, a closed source word and a millisecond stamp, never a value.
The rig gained wake arms (an interval and a dynamic scheduled wake) and an imports arm stopped by a no-key group kill, plus a user-config mark witness that searches the user's config only for two rig-made needles and keeps one closed word (unchanged, absent, changed-by-others, changed-by-rig, unread); the old word changed is retired.
Measured live: a fired scheduled wake's UserPromptSubmit carries no source key on claude 2.1.325 and 2.1.326, disproving the bundle schema read; the wrapper logged its prompt as origin human; the hook-latency window read 1 to 3 ms.
The test harness boot gained a per-boot readiness deadline (20 s by default), so a never-ready boot ends in about 2.5 s in its test; the CI test profile's 20 s kill line now covers three viola-e2e binaries (override filter 25 to 28 binaries).
@task_file n=1 path=crates/viola-e2e/src/rig/capture.rs
what: new; the standing live rig's own UserPromptSubmit capture, a test-only hook line with key names, a closed source word and no value
@task_file n=2 path=crates/viola-e2e/src/rig/wake.rs
what: new; the standing live rig's scheduled-wake arms, reading the wrapper's event stream and send replies as closed words
@task_file n=3 path=crates/viola-e2e/src/rig/isolate.rs
what: edited; the standing live rig's isolated child scratch and its user-config witness, the mark witness with closed words
@task_file n=4 path=crates/viola-e2e/src/rig/mod.rs
what: edited; the standing live rig: its arms, labels, session, no-key group-kill stop and summary words
@task_file n=5 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: edited; the standing live rig binary, its hidden capture verb and its wake and stand options
@task_file n=6 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited; the live rig's rehearsals against the fake agent
@task_file n=7 path=crates/viola-e2e/src/harness/boot.rs
what: edited; the test harness boot of an instance and its readiness deadline
@task_file n=8 path=crates/viola-e2e/src/harness/run/perf.rs
what: edited; the harness perf run that boots the benchmark instance timed for the hook perf budgets
@task_file n=9 path=crates/viola-e2e/tests/cli.rs
what: edited; the CLI integration tests run against booted instances
@task_file n=10 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: edited; the harness lifecycle tests, the never-ready boot case among them
@task_file n=11 path=.config/nextest.toml
what: edited; the test runner profiles, the ci profile's 20 s kill line and its override filter of test binaries
@task_item n=1 part=touches
Rows that call recorded fixtures or wrapped claude internals no contract and no telemetry source: the session disproved a bundle schema read (a fired scheduled wake carries no source key on 2.1.325 and 2.1.326), a live reading beside those rows, not a change to them.
@task_item n=2 part=touches
Rows on hook stdin parsing and correlation, and parse-rejected for hook stdin: the real CLI's UserPromptSubmit on a fired wake carried exactly eight keys (cwd, hook_event_name, permission_mode, prompt, prompt_id, scratchpad_dir, session_id, transcript_path) and no source; the product hook was not changed.
@task_item n=3 part=touches
Rows on the hook latency budget and hook perf budgets: the rig read a live hook-latency window of 3 ms (R3, R3-326) and 1 ms (R3D) — a reading, not a budget; a row is touched only where its cell holds that window or budget.
@task_item n=4 part=touches
Rows on the wheel log line, human typing and the wheel event as audit record: every fired wake's prompt-submitted line read origin human; whether a scheduled wake keeps the wheel is a planned route entry (v2-36), not settled here.
@task_item n=5 part=touches
Rows that keep content and values out of every line (default-deny fields, NEVER-log floor, fixtures and screen content no telemetry): the rig's own capture line and user-config witness follow the same rule in a test-only tool — key names and closed words, no value or content kept.
@task_item n=6 part=touches
Rows on the hidden hook capture arms being uninstrumented and writing no lines: a picker should check that none reads as covering the new rig capture verb, which is a separate test-only verb of the live rig, not the product hook's arms.
@task_item n=7 part=touches
Rows on the hook perf job and its benchmark instance run against a passing instance: the perf harness's boot now carries the per-boot readiness deadline (20 s); no budget, sample count or gate moved.
@task_item n=8 part=touches
Rows on CI gates, test homes kept until gates run and gates running after failed tests: the ci profile's 20 s kill line now also covers three viola-e2e binaries (override filter 25 to 28); a row is touched where its cell holds that kill line or that count.
@task_item n=9 part=touches
Rows on no sleep-based log waits and on harness boot readiness: a boot's readiness wait is now bounded by its own deadline, and a never-ready boot ends at it (20.644 s to 2.487 s in its test) with its supervisor stopped.
@task_fact n=1 slug=obs-scope-summary
fact: The standing live rig's own UserPromptSubmit capture is a test-only witness, not telemetry: one line per payload with key names matching a closed pattern, a closed source word (schedule_wakeup, loop_wakeup, other, absent) and a millisecond stamp, never a value.
@task_item n=1 part=untouched
The product log line, diagnostics line and detail-file line schemas, their fields, corr and event enums: no product process emits anything new or different.
@task_item n=2 part=untouched
Exit codes, detail codes, refusal pairs and stderr lines of every product verb (run, send, wait, last, answer, verify, revive, link, mcp): no product code changed.
@task_item n=3 part=untouched
The events.ndjson event line and its own source field: the product's event stream is unchanged; only the CLI's hook payload was found to lack a source key.
@task_item n=4 part=untouched
Span and trace coverage, span names, span attributes and panic capture by role: nothing instrumented was added or removed.
@task_item n=5 part=untouched
Banned or admitted telemetry dependencies, OTel, error reporters and network egress: no dependency added or bumped and no exporter or endpoint.
@task_item n=6 part=untouched
CI secret scan, artifact upload and retention rules: no upload, scan scope or retention changed.
@task_item n=7 part=untouched
Heartbeat liveness, staleness thresholds and list liveness contracts: untouched by a test-harness readiness deadline.
