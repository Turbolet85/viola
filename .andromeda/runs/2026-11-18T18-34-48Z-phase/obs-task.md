@task_prose part=chunk
Lands a new recorded fixture set for claude 2.1.325, written whole by a clean live `viola verify --record` under the rig's private config, every file through the recorder's scrub and none hand-edited.
Re-records the viola-mcp `list` tool listing through `viola-mcp-tap` on 2.1.325 and places it in the same set.
Re-reads the ledger probes' STAMPED reading (the 24 relied rows) and the drift suites against the new set, the fake agent replaying it in CI.
Measures live beside the recording: a StopFailure payload's error word against the cause map, the /context post-condition and whether /mcp disable writes the real config, the box placeholder hint, whether Run E starts, and the bt-absent control on a filtered run.
Disposes each of the entry's eleven carries by name (built, measured, or leaving with reason and next reader); live starts wait on the founder's cap word.
@task_file n=1 path=fixtures/claude/2.1.325/ mark=edits
what: the recorded fixture set of the wrapped claude CLI at 2.1.325 — the scrub-required recorded payloads and screens the fake agent replays in CI and the drift suites read
@task_file n=2 path=fixtures/claude/2.1.325/Mcp.tools-list-1.json mark=edits
what: the recorded tools listing of the viola-mcp stdio server tools (answer, last, list, send, wait) as the CLI reads them, re-recorded through the MCP tap — edits: the list tool's description
@task_item n=1 part=needs
The recorded fixtures of the wrapped claude CLI are scrub-required: every recorded file passes the recorder's scrub, holds no host path, user name, email, token or status-line text, and a payload holding a home path is refused, never scrubbed by hand.
@task_item n=2 part=needs
Recorded fixtures are no telemetry source: nothing in the new set is read as or copied into diagnostics lines, and kept live screens stay local, never uploaded.
@task_item n=3 part=needs
The home a live `viola verify --record` takes holds no path component named home, produced by a record-home helper like the tests' one (the live rig's half of the record-home rule).
@task_item n=4 part=needs
viola verify as the only writer of capability-ledger stamps: the STAMPED reading over the 24 relied rows, the version unverified while any relied row is unread, and an unreadable ledger read as an unverified CLI.
@task_item n=5 part=needs
viola verify's own output and lines: the stdout step and summary lines that carry the relied-row count, verify exit codes 0 and 1, verify process-exit details, and verify log lines carrying no content, ledger path or stamps.
@task_item n=6 part=needs
The verify child spawn subjects a recording run drives (version-probe, agents-probe, verify-probe, verify-pty-probe) and their process-exit lines with child exit status and duration.
@task_item n=7 part=needs
Drift suites read against the new set: drift reports go to the detail file only, never to stderr or home-level lines.
@task_item n=8 part=needs
CI runs verify against the fake agent replaying the recorded set only; a real-CLI verify never runs in CI — every live start is a local, founder-capped run.
@task_item n=9 part=needs
The viola-mcp `list` tool's description as the CLI reads it, re-recorded live (one live `claude -p` start through the MCP tap), its description read without the usage-budget clause.
@task_item n=10 part=needs
The StopFailure hook payload recorded live inside a started session's turn: its error word mapped to a cause of the closed cause map, and for a word of the CLI's 13 no parse-rejected unknown-value line from the hook-stdin parser; settling the PROVISIONAL exception (a ledger row with its probe, or kept) is the founder's.
@task_item n=11 part=needs
Hook stdin parse failures surface as unknown plus a parse-rejected count; hook stdin values never logged verbatim.
@task_item n=12 part=needs
The local-command echo relied row re-stamped live; the /context local command's post-condition measured live, and whether `/mcp disable` of an absent probe server writes to the user's real Claude config; the transcript echo read stays uninstrumented.
@task_item n=13 part=needs
The box-text placeholder hint (holds-text) read from the screen model: whether a cell attribute tells the hint from typed text; no telemetry is drawn from screen content.
@task_item n=14 part=needs
The browser-tools prompt and trust-record relied rows read live on 2.1.325 under the private config only; viola never answers or seeds the browser-tools prompt, and its reading stays uninstrumented.
@task_item n=15 part=needs
Run E's resume start read from the clean recording: whether a SessionStart capture exists that the resume target accepts; hidden hook capture arms stay uninstrumented.
@task_item n=16 part=needs
The bt-absent remove-the-guard control fired on a filtered entry (a case filter or no-fail-fast) so the named browser-tools-row case reads red without the guard.
@task_item n=17 part=needs
Every live start names the 2.1.325 install by its path, not the bare claude command (which resolves to 2.1.327); nothing is claimed for 2.1.326 or 2.1.327.
@task_item n=18 part=needs
The pinned product binary that drives the session is not rebuilt, re-pinned or replaced before or by the verify, unless the founder says so.
@task_item n=19 part=needs
Wrapped claude internals are not a contract and their child env values never logged: stripped CLAUDE values and the child settings argument stay out of every recorded and logged line.
@task_item n=20 part=needs
No mutation run and no scheduled wake or timer; mutation outputs and local run outputs are never uploaded.
@task_standing n=1
convention: each new recorded fixture file is scrub-required and admissible by its content alone — no host path, name, email, token or status-line text, never hand-edited
home: pii-scrubbing-compliance · 8-pii-scrubbing-compliance
@task_standing n=2
convention: each test home or record home a new test or recording run takes lives under the one scanned root and names no path component home
home: ci-integration · 9-ci-integration
@task_item n=1 part=not_needed
Future error-reporter rows (opt-in, ureq and rustls, egress entry): the chunk adds no reporter.
@task_item n=2 part=not_needed
Panic capture and panic-outcome-by-role rows: the chunk changes no catch site or panic hook.
@task_item n=3 part=not_needed
Span declaration, span naming and span attribute rows of run start, confirmed send, wait, last and wheel: no span is added or changed.
@task_item n=4 part=not_needed
Link, unlink, pause and release-from-driver rows and their events-log entries: none of those verbs is touched.
@task_item n=5 part=not_needed
Heartbeat file, liveness staleness and list --json liveness rows: liveness is not read or changed.
@task_item n=6 part=not_needed
Metric derivation rows (counters, gauges, histograms, quantiles, closed-enum grouping): no metric is derived or reported.
@task_item n=7 part=not_needed
OpenTelemetry SDK, exporter, OTLP receiver, traceparent and network-egress decisions and absences: nothing is exported.
@task_item n=8 part=not_needed
config.json keys, diagnostics level and log-level-from-environment rows: no configuration key is added or read differently.
@task_item n=9 part=not_needed
Revive preflight and revive list rows, and the exit-1 and exit-21 detail-code tables of start, send, wait and frame verbs: no exit cause is added.
@task_item n=10 part=not_needed
Channel JSON-RPC params, conn and corr join rows and channel response codes: the channel's wire is not touched.
