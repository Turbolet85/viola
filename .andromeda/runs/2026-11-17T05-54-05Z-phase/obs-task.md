@task_prose part=chunk
Builds one standing, committed, test-only live rig that later live rounds reuse in place of per-round evidence scripts: it starts the real claude CLI, or viola wrapping it, in a pty it owns, keeps a terminal screen model and the screens it reads as evidence, and reads the CLI's trust dialog and modal dialogs by their drawn literals and selector rows.
The rig owns its isolation from the founder's real Claude config and login (scratch root outside every repository, private config dir, a credentials copy guarded by a token-window read and removed on every exit path) and its process hygiene (it ends what it starts on every exit path, proven by a process census under its scratch root).
It re-measures what each earlier round's evidence rig built, got wrong and must own, and is to take carried live readings: whitespace trimmed at submit, the hook-latency window before a send's paste, a fired scheduled wake's prompt-submit source field and a real scheduled-wake count of 1, and the record home of a live viola verify --record.
The fake agent gains DEL fidelity and modal fidelity (the trust dialog's literal and selector rows as the live CLI draws them); every isolation property is proven once by a named check on the fake agent with no live start.
The product binary gains no verb and no surface; no product behaviour changes beyond what a carried reading forces and the founder places.
@task_item n=1 part=needs
Real-CLI runs stay out of CI: the live rig starts the real claude CLI only by hand, outside CI, while CI verification and the rig's own checks run against the fake agent only.
@task_item n=2 part=needs
Recorded and kept evidence is no telemetry source: the rig's kept screens and captured payloads are test evidence, screen content never becomes product telemetry, and the wrapped claude CLI's internals, transcripts and drawn dialogs are read as observed, not as a contract.
@task_item n=3 part=needs
Nothing of the host reaches a record: no absolute host path, no status line, no config of the founder's, no credential and no token-window value appears in any record, log line or kept evidence the rig writes; the token-window value is read to refuse a start under 20 minutes and is never written.
@task_item n=4 part=needs
Secrets and CLAUDE environment values never logged: the rig's credentials copy, its private config dir and the CLAUDE values it sets or strips for the started CLI never appear in a diagnostics line, detail file or evidence artifact.
@task_item n=5 part=needs
Local run outputs and live evidence are never uploaded: a live round's scratch root, kept screens and readings stay local, and any artifact that does leave goes only after the secret scan.
@task_item n=6 part=needs
Process hygiene over the pty child: the rig and any booted test it stands for end every process they start on every exit path, including their own kill deadline (a kill with a fallback), proven by a census of processes under the rig's scratch root at exit; no booted wrapper outlives a killed test.
@task_item n=7 part=needs
Diagnostics home resolution for a live run: the home a live viola verify --record takes comes from a helper like the tests' record_home() and holds no path component named home; a stamped home is taken from the shared stamp's copy, and viola verify stays the only stamps writer.
@task_item n=8 part=needs
The whitespace-trim reading reads delivery on the interactive input box without logging what was sent: sent text and typed keys never appear in a diagnostics line or the rig's records beyond the closed words the reading needs; the delivery confirmation's tolerance stays trailing-only (ASCII space, TAB, CR, LF) until the reading lands.
@task_item n=9 part=needs
The hook-latency reading measures the window between a turn the CLI starts and its UserPromptSubmit hook reaching the wrapper before a send's paste, read from the wrapper's existing hook-invoked, send-issued and send-confirmed lines, their corr join and their duration and timestamp fields.
@task_item n=10 part=needs
The scheduled-wake reading captures a fired wake's prompt-submit payload shape as closed words only (whether the optional source field carries schedule_wakeup or loop_wakeup, on which CLI versions), with no payload body in any record; the hook's handling of the payload is unchanged, and the turn-ended scheduled_wakes count is read live as 1.
@task_item n=11 part=needs
The rig reads the trust dialog and modal dialogs by drawn literal and selector rows; dialog text it reads is evidence only and never reaches a diagnostics line, and the fake agent's trust dialog literal and selector rows mirror the live CLI's.
@task_item n=12 part=needs
The rig's terminal screen model (the vt100 crate the product pins for readiness, or another settled at P3) is fed from the rig's own pty; a panic in feeding it is contained in the rig and never ends a run silently.
@task_item n=13 part=needs
Test waits are bounded: a test waiting on a channel, thread or log line takes a bound and waits on the condition, never on a sleep; the rig's live waits carry their own deadline.
@task_item n=14 part=needs
Test homes and sockets: each rig or test home lives under the integration-test home root that the secret scan covers and is kept until the gates run; a test socket lives under the short /tmp root.
@task_item n=15 part=needs
No mutation run in this chunk; the rig is test-only code, so no obs mutation obligation over product code is added by it.
@task_item n=16 part=needs
Any log line the rig adds to the product, only if a carried reading forces it and the founder places it, joins the closed event enum and the additive-only diagnostics schema, and is schema-checked like every emitted line.
@task_standing n=1
convention: Integration and E2E test homes live under one root inside the secret-scan root and are kept until the panic, schema and scan gates run.
home: 9-ci-integration · ci-integration
@task_standing n=2
convention: Every line a test or rig process emits is schema-checked in integration and E2E tests against the diagnostics line schema.
home: ci-integration
@task_standing n=3
convention: No sleep-based log waits; a test waits on a condition with a bound.
home: 10-slo-invariants-telemetry-budgets
@task_standing n=4
convention: An artifact a test or CI job uploads comes only from synthetic inputs and only after a passing secret scan; local run outputs are never uploaded.
home: 8-pii-scrubbing-compliance · pii-scrubbing-compliance
@task_item n=1 part=not_needed
Metric derivation: counters, gauges, histograms and quantiles derived by jq over the ndjson are untouched; the rig derives no product metric.
@task_item n=2 part=not_needed
A future opt-in error reporter (its transport, its never-log floor, its egress entry, its being off by default) is not built or enabled here.
@task_item n=3 part=not_needed
OTel SDK, exporters, receivers and W3C traceparent rows: the rig exports nothing and adds no telemetry pipeline.
@task_item n=4 part=not_needed
The viola-mcp stdio server's tools, its frame refusals and its MCP stdin limits are not changed or exercised by the rig.
@task_item n=5 part=not_needed
Heartbeat liveness and staleness, list liveness and agent-run status states are not changed.
@task_item n=6 part=not_needed
viola revive preflight order and its detail codes, and link and unlink lines and methods, are outside this chunk.
@task_item n=7 part=not_needed
The span attributes and span chains of viola run start, wait, wheel and dialog paths are unchanged; the product gains no span.
@task_item n=8 part=not_needed
config.json keys (dialog_hold_s, diagnostics_level, its version and known keys) are not changed.
