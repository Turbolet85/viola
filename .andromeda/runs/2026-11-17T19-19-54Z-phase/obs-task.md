@task_prose part=chunk
Adds the 23rd relied capability-ledger row in viola-agent-claude: the claude CLI honours a per-project trust record (hasTrustDialogAccepted under the CLI's own key) that another process wrote into the config file its child reads.
Gives the row its viola verify probe: a SIXTH interactive verify child run that writes the trust record for its own OS-temp dir, expects the child's first settled frame to be no trust dialog, and removes only that one record of its own on every exit path.
Builds the record writer once (lock when free, atomic replace, read-back, at most 3 attempts), placed where the later product write can reach it, and makes the fake agent read the record from the config file rather than argv alone, so the probe and its withheld-record control can fail.
Re-runs the step-count sweep over every spelling of verify's step count, and stamps the row by one live viola verify (S0, plus one spare) on the 2.1.325 install under the rig's private config; once relied, every 22-row stamp reads unverified.
Owes a reading of the CLI's rootOnly branch trigger, read from the installed CLI's strings if reachable without a start, else named unmeasured.
@task_item n=1 part=needs
viola verify's child set grows from five interactive runs to six: the sixth interactive claude child is spawned, logged and ended like the other interactive verify children, under the existing verify child spawn subjects, with its process-start and process-exit lines carrying child_exit_status and duration_ms.
@task_item n=2 part=needs
verify stdout: the new probe adds a step line and moves verify's step counts; the summary line prints what proves verify's own trust record is gone after the run, without printing the config file's path, the key or any record content.
@task_item n=3 part=needs
The capability ledger gains a 23rd relied row: once relied, every 22-row stamp reads unverified (an unverified CLI), and a stamp is written only by verify as the sole stamps writer; ledger paths and stamps are never logged, and an unreadable ledger still means an unverified CLI.
@task_item n=4 part=needs
The probe's row outcome is a closed word: when the config file is absent, unreadable, a link, not the JSON expected or oversize, nothing is written and the row names what it could not do in a closed word, never pass; when the CLI does not honour the record the row reads fail. Detail words carry no path or pid.
@task_item n=5 part=needs
Reading the claude config file is parsing an external, untrusted Claude-owned file: parse failures (malformed, oversize, unreadable, link) surface as an unknown or refused outcome, not a crash; claude internals are not a contract, and serde error chains never reach stderr or a log line verbatim.
@task_item n=6 part=needs
verify log lines carry no content: no line or span names the config file path, the trust key (the cwd or git root path), the OS-temp dir, the JSON record, or the founder's config; paths stay out of home-level lines and spans skip all arguments.
@task_item n=7 part=needs
The record writer's race handling is a bounded lock-then-atomic-replace with read-back and at most 3 attempts; it must stay inside the rules on retry policies and bounded waits, and its failure ends in a fixed, pathless error message and a closed detail word.
@task_item n=8 part=needs
Record removal on every exit path verify reaches, including an interrupted verify (the interrupted process-exit detail and the stop's stderr pair) and a panic caught at verify's catch site; what a SIGKILLed verify leaves is named, and the stop removes it.
@task_item n=9 part=needs
The writer's placement: the record's JSON shape on the Claude side (viola-agent-claude does no file I/O), the lock, atomic replace and read-back on a Claude-free side that both verify and the later run reach; that side stays synchronous, Tokio-free and C-free, with its errors typed and fixed in Display.
@task_item n=10 part=needs
The sixth child's environment: CLAUDE_CONFIG_DIR, where the home keeps it for the child, decides which config file the record goes into; the claude child's env lines carry names only (env_kept, stripped counts), never values.
@task_item n=11 part=needs
The probe reads the child's first settled PTY frame for the presence of the trust dialog: no screen or dialog text, typed key or PTY byte enters telemetry; verify pastes only its own ratified literals.
@task_item n=12 part=needs
The fake agent decides trust by reading the record from exactly the config file and key the writer used (CLAUDE_CONFIG_DIR's .claude.json, else the user's), so the post-condition and the withheld-record control both prove on the fake; CI runs verify against the fake agent only, and a change to the fake agent's screens or trust reading puts cli_verify in the gate block.
@task_item n=13 part=needs
The live stamp: real-CLI verify never runs in CI; S0 (and the spare S0-2) runs under the rig's private config, ledgered with the clock's time beforehand, keys: none and its stop named; the founder's real config file is read only by the mark witness.
@task_item n=14 part=needs
Tests the chunk adds: each control fired with a named --timeout, control and sweep ids at most 16 characters, a waiting test bounded under the 20 s lint with no sleep-based log waits, and the step-count sweep over every spelling of the count; no mutation run is taken for this chunk.
@task_item n=15 part=needs
The tests' shared stamp and CI stay green once the 23rd row is relied; the product binary pinned for the overseer is not rebuilt and a live home is not re-stamped by this chunk, and the plan names what turns stale.
@task_standing n=1
convention: every log line a new test drives is schema-checked against the diagnostic line schema in integration and E2E tests
home: ci-integration
@task_standing n=2
convention: every new test's home lives under the one scanned integration-test root and is kept until the gates run
home: 9-ci-integration
@task_standing n=3
convention: a new test that waits for a log line or state polls with a bound, never a sleep
home: 10-slo-invariants-telemetry-budgets
@task_standing n=4
convention: every new log line goes through the single obs_event macro with a closed event value; no raw tracing or print macros
home: 11-obs-anti-patterns-never-do-these
@task_standing n=5
convention: every new span or instrumented function skips all arguments and uses a static name
home: bootstrap-phases-derive-for-route-setup-project
@task_item n=1 part=not_needed
The viola-channel JSON-RPC lines, channel params, conn and corr join keys and the peer-credential checks: the chunk opens no channel connection.
@task_item n=2 part=not_needed
The viola mcp server's tools, stdin frame limits and its isError results: the chunk does not touch the MCP role.
@task_item n=3 part=not_needed
The viola hook's lines, decisions, fail-open behaviour and latency budgets: the chunk writes no hook path.
@task_item n=4 part=not_needed
viola run and revive start spans, start failure detail codes, preflight order and run-start event order: nothing of run or revive changes here, and the product's trust write belongs to a later entry.
@task_item n=5 part=not_needed
Send, wait, last, wheel and dialog span chains and their log fields: the chunk drives no send, wait or dialog answer.
@task_item n=6 part=not_needed
The events.ndjson product stream, snapshot replay and state-recovered lines: the chunk writes no event and recovers no state.
@task_item n=7 part=not_needed
The harness logs command and agent-run logs and status output: the chunk adds no reader of logs.
@task_item n=8 part=not_needed
The future error reporter, OTel exporters and network egress: the chunk adds no reporter or exporter.
@task_item n=9 part=not_needed
Metric derivation, histograms, gauges and quantiles: the chunk adds no metric.
@task_item n=10 part=not_needed
Heartbeat file and liveness rows: verify has no heartbeat.
@task_item n=11 part=not_needed
Link and unlink, implicit link records and link-name rules: the chunk links nothing.
@task_item n=12 part=not_needed
CI artifact upload, retention and the detail-file upload rules: the chunk adds no upload.
