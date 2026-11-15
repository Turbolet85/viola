@task_prose part=chunk
Builds the first live round on the product with the budget gate and the status-line interception removed: test-only rigs that drive the real claude CLI headless on a plain pty the rig opens itself, each rehearsed first under the fake agent in CI, then named operator steps fired only on the operator's word, each with its live-start count.
Proves the wrapped start live (matrix v2-03): the unmodified claude wrapped with no byte of viola's own on the pty, keys passed unchanged, the pinned copy run through a freshly written plugin, the start order with no budget step, a live or stale name refused, the child's end detected on the process, the capability ledger green for the CLI run.
Reads a live dialog hold close to its 3600 s ceiling (matrix v2-02), so the dialog_hold_s ceiling stands or moves on the founder's word and its PROVISIONAL mark goes; product behaviour changes only where a live reading exposes a defect.
Re-records the MCP list tool fixture so its description drops the usage budget, records by viola verify --record a box-only screen fixture for the fake agent to replay in CI, and reads a wrapped session's status line beside an unwrapped one's, writing nothing to the user's settings.
@task_file n=1 path=fixtures/claude/2.1.302/Mcp.tools-list-1.json
what: a recorded claude fixture of the viola-mcp stdio server's tools list, scrub-required, re-recorded by a live start through the MCP tap so the list tool's description drops the usage budget
@task_file n=2 path=scripts/agent-run.sh mark=edits
what: the agent-run rig script that drives a live round and reads back its logs and status — edits: the live-round rig steps
@task_item n=1 part=needs
The live-round rigs in the test-only e2e crate (the harness and the MCP tap) and the agent-run script: they spawn real claude only as named operator steps, never in CI; CI verifies against the fake agent only, and each rig is rehearsed there first.
@task_item n=2 part=needs
viola verify --record and the recorded fixture class per CLI version: a new box-only screen fixture and the MCP tools-list re-record, each passed through the record scrub; recorded fixtures are scrub-required and are no telemetry source.
@task_item n=3 part=needs
The viola-mcp list tool's description changes (the usage budget words drop) in the re-recorded fixture: the list tool row of the viola-mcp stdio server tools boundary is what the fixture must match.
@task_item n=4 part=needs
The wrapped start read live: viola run silent on the terminal, the child's output passed unchanged on the pty, the run start events and start span chain in the documented order with no budget step, the process-start and process-exit lines of the claude child.
@task_item n=5 part=needs
A live or stale name refused at start: the run start exit-1 detail codes already-live and stale-heartbeat, read on the run process log's process-exit line.
@task_item n=6 part=needs
The child's end detected on the process handle, never from screen content; the run log's exit_code read as the evidence of it.
@task_item n=7 part=needs
The pinned copy run through a freshly written plugin by absolute path: the pinned copy is placed and never replaced, plugin install's exit codes and --json document read; the endpoint and pinned binary path never logged.
@task_item n=8 part=needs
The capability ledger's stamps for the CLI the round runs, written only by viola verify on a test home: the run log's cli_verified (true for the founder's account) and cli_version, an unreadable ledger reading as unverified; ledger path and stamps never logged.
@task_item n=9 part=needs
The dialog hold read near its ceiling: DialogHold / DIALOG_HOLD_MAX and config.json's dialog_hold_s (60 to 3600 s) read, not changed unless the founder's word moves it; the reading taken from the dialog log fields, the hook-decision deadline_hit field and the events log's dialog-expired.
@task_item n=10 part=needs
The hour-long hold keeps a window idle past the CLI's compact: the step journals its state in the work dir before the wait; dialog content is never logged in that journal or the process logs.
@task_item n=11 part=needs
The status-line reading beside an unwrapped session: it reads the user's own status-line setting and writes nothing to the user's claude settings; the child settings argument is never logged, and wrapped claude internals are no contract.
@task_item n=12 part=needs
Live homes are test homes on tmpfs under the integration-test home root (a viola-live prefix), kept until the gate steps run; nothing is written under the user's own viola home, and the pinned product binary is untouched.
@task_item n=13 part=needs
Every spawn the rigs make has a kill deadline; waits on log lines are never sleep-based.
@task_item n=14 part=needs
The CLAUDE environment strip on the claude child the rig spawns: stripped values are never logged; the process-start line carries env names and counts only.
@task_item n=15 part=needs
The readings are taken through the diagnostics files and the harness / agent-run logs command and status (merged sources, filters, torn lines, last_error), machine-read, never by a human reading a screen.
@task_item n=16 part=needs
Evidence: a live-round record per operator step in the chunk's work dir evidence folder; it carries no sent text, typed keys, dialog text, assistant message or secrets, and local run outputs are never uploaded.
@task_item n=17 part=needs
A guard test the rigs add carries its remove-the-guard control through the gate tool's control step, named by the test's name; no mutation run in this chunk.
@task_item n=18 part=needs
A defect a live reading exposes folds into this chunk as its own item with its own acceptance; any fix it brings stays inside the product's existing log line schema, detail codes and redaction floor.
@task_standing n=1
convention: every line a new integration or E2E test makes a process emit is schema-checked, and the panic and schema gates run over its test home
home: ci-integration · 9-ci-integration
@task_standing n=2
convention: every new test home lives under the one integration-test home root the secret scan covers, kept until the gate steps run
home: 9-ci-integration · ci-integration
@task_standing n=3
convention: every new recorded claude fixture is scrub-required and no telemetry source
home: 1-obs-scope-summary · obs-scope-summary
@task_standing n=4
convention: every new test waits on log lines without a sleep and asserts budgets by exit code
home: 10-slo-invariants-telemetry-budgets
@task_item n=1 part=not_needed
OTel, OTLP, exporter, trace-context propagation and network-egress rows: the round adds no telemetry pipeline and no endpoint.
@task_item n=2 part=not_needed
Metric rows (counters, gauges, histograms, quantiles, metric grouping): the hold and start readings are evidence records, not derived metrics.
@task_item n=3 part=not_needed
Hook performance budgets and the hyperfine perf job (spine and session-end hook gates): the round times no hook.
@task_item n=4 part=not_needed
The future error reporter rows (opt-in, ureq and rustls, egress entry) and Sentry: no reporter is added.
@task_item n=5 part=not_needed
Send, wait, last, wheel, pause and release paths, their refusal details, span chains and confirm-window rows: the round drives no send or wheel through viola.
@task_item n=6 part=not_needed
Revive, list, link and unlink rows, their preflight codes and output: the round runs none of those verbs.
@task_item n=7 part=not_needed
Socket-directory, server-verify and strict-modes refusal detail codes across the CLI, MCP and hook: no refusal path changes.
@task_item n=8 part=not_needed
The hyprctl probe and its parse rows: live sessions run headless with no compositor.
@task_item n=9 part=not_needed
Panic outcome by role and the panic hook's install order: no product role's panic handling changes.
@task_item n=10 part=not_needed
The detail-file line schema fields and the secret-scan hit report fields: neither shape changes.
@task_item n=11 part=not_needed
State recovery rows (snapshot replay, torn-line healing, state-recovered detail): the round reads no replayed snapshot.
