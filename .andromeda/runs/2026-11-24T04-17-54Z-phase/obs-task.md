@task_prose part=chunk
Adds a new human CLI verb `viola up` that re-opens the fleet after a host reboot: it reads the board's instances under the home, selects those that read as live before the host went down, and orders them parents before children by the recorded lineage.
It skips every instance whose recorded pid and start time verify live, and re-opens each remaining one resumed in place through the existing revive start path, in the terminal window viola open would give it, in its recorded workspace.
It prints one line per instance naming the instance and a closed outcome word, and under --json a document of codes and closed words only.
It adds tests proving the selection, the order, the skip, the lines and the argv handed to each start, over the fake agent and the Hyprland fakes under a test-owned home; every live reading of up stays the operator's, and a clause the fakes cannot carry is claimed GATED on it.
It re-measures at HEAD the lineage and launch lines at every start, viola open's window record under --origin, and revive resuming in place.
@task_file n=1 path=src/cmd/mod.rs mark=edits
what: the root bin's clap dispatch, the CLI verb table and help rows of the human cli role — edits: the verb table entry and dispatch arm for up
@task_file n=2 path=src/cmd/up.rs
what: the new cli verb up: board scan, was-live selection, parent-before-child order, live skip, and a revive start per remaining instance in a viola open terminal
@task_file n=3 path=src/human/ mark=edits
what: the human module rendering a cli verb's terminal lines — edits: the per-instance line of up with its closed outcome word
@task_file n=4 path=tests/cli_up.rs
what: the cli integration tests of up over the fake agent and the Hyprland fakes under a test-owned home
@task_item n=1 part=needs
The CLI verb table, help rows and exit-cause tables gain the verb up, each exit cause with its own distinct closed detail code, and the cli --json machine contract gains its document of codes and closed outcome words only, with no path, pid, address or workspace in it.
@task_item n=2 part=needs
The board read model the verb consumes: the board scan of the home's state, liveness read from the snapshot pid and start time and the heartbeat with its staleness threshold, and a snapshot rebuilt by replay after a reboot whose live fields (endpoint, pid, started_at) are no live reading and must not make an instance read live or reachable.
@task_item n=3 part=needs
Parent-before-child ordering read from the lineage record (parent and role) at every start; the parent and role names stay confined to product surfaces and never enter a process-log line.
@task_item n=4 part=needs
Each re-open goes through revive's start arm: its preflight order (claude agents --json probe, session-live check that fails open), its strict-modes checks and its closed refusal codes (already-live, session-live, no-session, cwd-missing, stale-heartbeat, squatted-name, pinned-hash-mismatch, socket-dir-refused, strict-modes-failed), and a revive start logs as run in the run process log; up maps each to its per-instance outcome word without inventing new causes for them.
@task_item n=5 part=needs
The terminal spawn viola open makes and the wrapper's window record (address, workspace, placement, origin) under --origin, through the Hyprland seam's closed request set; the terminal and hyprctl-probe spawn subjects on child spawn log lines, and a hyprctl JSON parse failure surfacing as unknown with a parse-rejected line.
@task_item n=6 part=needs
The launch pair (model, effort) recorded at every start and the window line, lineage line and launch line each having no trace of their own.
@task_item n=7 part=needs
The cli process-log file for the new verb, as viola open, viola focus and the revive start arm each have theirs: process-start and process-exit lines with a closed detail code per exit cause, written only under a parsed instance name; how a fleet-wide verb that names no single instance logs, given no process log without an instance and instance never from the working directory.
@task_item n=8 part=needs
The per-instance outcome as an entry of the obs event catalog: a closed event value and closed outcome words, corr set explicitly by the caller where correlated, emitted through the single obs event macro.
@task_item n=9 part=needs
The NEVER-log floor over the verb's lines: no window address, pid or workspace in a process-log line, home-level lines path-free, detail codes carrying no path or pid, endpoint and pinned binary never logged, CLAUDE environment values never logged.
@task_item n=10 part=needs
Strict-modes checks on the home tree before the cli log opens: a refusal before the log opens writes no line and exits with the strict-modes-failed code and the fixed unable and hint pair on stderr.
@task_item n=11 part=needs
The terminal surface of the per-instance line: fixed stderr refusal and fault messages that interpolate nothing, control characters escaped in terminal rendering, serde and error chains reduced to a fixed message with the full chain only in the cli detail log.
@task_item n=12 part=needs
The cli manual dispatch span per verb for up: a static, low-cardinality span name, skip-all arguments with an explicit field allow-list, no endpoint, path or env values on spans, no err or ret options.
@task_item n=13 part=needs
Panic capture for the cli role and cli --json: the panic outcome by role, the exit decided at the catch site, one single-line panic record, anyhow at the dispatch edge only.
@task_item n=14 part=needs
The fleet re-open stays CLI-only: no MCP tool, no viola-mcp stdio tool or frame, as the conservative arm of a boundary widening.
@task_item n=15 part=needs
Proof through the fake agent and Hyprland fakes only: CI verifies against the fake agent, the real claude CLI never runs in CI, no window is opened, moved, focused or closed and no Hyprland dispatch, key or real session revive happens in this phase; clauses needing a real host restart or a real resumed session are GATED on the operator's live reading.
@task_item n=16 part=needs
A gate red with no failed assertion is read against host load and free /tmp space first, never re-run for green; re-run is no substitute for diagnostics.
@task_item n=17 part=needs
The re-measure at HEAD of the lineage and launch lines at every start, viola open's foot window and the wrapper's window record under --origin, and revive resuming in place: read from the events log and run process log, not copied from earlier claims.
@task_standing n=1
convention: every new integration test's home lives under the one scanned e2e home root and is kept until the gate steps run
home: 9-ci-integration · ci-integration
@task_standing n=2
convention: every line a new test makes a process emit is schema-checked by the schema conformance gate on real process output
home: 9-ci-integration · ci-integration
@task_standing n=3
convention: every new log line goes through the single obs event macro with a closed event value; no raw tracing macros, no print macros, no bare instrument
home: 11-obs-anti-patterns-never-do-these
@task_standing n=4
convention: every new logged field passes the default-deny source allow-list and is added to the log schema additively, never renamed
home: 8-pii-scrubbing-compliance · pii-scrubbing-compliance
@task_standing n=5
convention: every new test file joins the OS-gated test census and its run must leave zero panic lines in home-level files
home: 9-ci-integration · ci-integration
@task_standing n=6
convention: every new fixture is synthetic and scrub-clean, never a recording of the operator's real claude home
home: 1-obs-scope-summary · obs-scope-summary
@task_item n=1 part=not_needed
The channel JSON-RPC request and response lines, their params, errors and corr: up sends no frame, text or key to any instance.
@task_item n=2 part=not_needed
The send, wait, last, dialog, answer, pause-release and wheel paths, their log fields, spans and refusal details: up drives none of them.
@task_item n=3 part=not_needed
The hook role: its stdout and stderr streams, fail-open, capture arms and the spine and session-end hook performance budgets.
@task_item n=4 part=not_needed
viola verify, its capability ledger and stamps, its spawn subjects and its stdout step and summary lines.
@task_item n=5 part=not_needed
The future opt-in error reporter rows: its transport, its egress entry, its default-off switch.
@task_item n=6 part=not_needed
The harness logs command and agent-run logs and status output, their filters, merge and torn-line handling.
@task_item n=7 part=not_needed
Explicit link and unlink methods and their events log lines; plugin install exit codes and documents.
@task_item n=8 part=not_needed
Mutation and fuzz artifact upload rules and the secret-scan hit report fields: the chunk adds no CI job and no upload.
