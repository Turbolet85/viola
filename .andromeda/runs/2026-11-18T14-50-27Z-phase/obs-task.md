@task_prose part=chunk
Adds a compiled screen literal for the wrapped CLI's browser-tools offer (the 2.1.325 ChromeAutoEnableDialog) to the modal set the one settled-box predicate reads, so verify's pastes and run's pre-send gate read that screen as a standing modal, never as a settled input box.
Adds the capability ledger's 24th relied row and the viola verify probe that measures it on an installed CLI, a version-stamped dependence like every other screen signature; verify's row count goes from 23 to 24.
Makes run's pre-send gate refuse to type at a standing browser-tools prompt, as at every modal: no byte is typed into it, the stop with the modal standing included.
Teaches the fake agent the browser-tools screen so CI proves the literal, the row and the gate's refusal against recorded screens; nothing is re-pinned, and live starts are only planned (one founder-forks block at the end of P3).
@task_file n=1 path=viola-agent-claude mark=edits
what: the wrapped-claude adapter: the capability ledger of relied rows, the screen-signature literals and the modal set the settled-box predicate reads — edits: the capability ledger (a new relied row), the screen-signature literals, the modal set
@task_file n=2 path=src/run/ mark=edits
what: viola run, its pre-send readiness gate that refuses to type at a standing modal — edits: the pre-send gate's reading of a modal
@task_file n=3 path=src/cmd/verify mark=edits
what: viola verify and its probe runs, the only writer of the ledger stamps — edits: the new row's probe and its stamp, the row count 23 to 24
@task_file n=4 path=viola-fake-agent mark=edits
what: the fake agent CI verifies against, its set of recorded screens where CI's modal cases live — edits: a browser-tools screen
@task_file n=5 path=fixtures/ mark=edits
what: the recorded claude fixture set, scrub-required, touched only where the recorded set needs the browser-tools screen
@task_item n=1 part=needs
viola verify's stdout step and summary lines: one more step line for the new relied row, and a summary read over 24 rows instead of 23.
@task_item n=2 part=needs
The new probe's child spawn: it runs under an existing verify child spawn subject (the PTY probe or the print-mode probe), with its process-start and process-exit lines, child exit status and duration, in the existing subject enum; any new subject or detail word is an additive schema change.
@task_item n=3 part=needs
verify lines carry no content: the probe's outcome for the browser-tools row is logged as a code, never the screen text, the prompt's title or its options.
@task_item n=4 part=needs
No telemetry from screen content: matching the compiled literal against the screen emits no line carrying screen bytes; the PTY byte stream is never logged.
@task_item n=5 part=needs
The settled-box predicate change reaches the confirmed send path: a send met by the standing browser-tools modal ends in the existing wrapper send-refused path and its closed detail words (input-not-ready class), with the readiness gate span and send-refused line as they already stand.
@task_item n=6 part=needs
Typed keys and sent text never logged: the gate's refusal and the probe's no-type rule add no record of what would have been typed; the PTY seam's paste event is not written for the modal case because nothing is pasted.
@task_item n=7 part=needs
The capability ledger's path and stamps are never logged, and an unreadable ledger reads as an unverified CLI; a home stamped at 23 rows meets a binary that relies on 24, and what run reads there surfaces only through run's cli_verified and cli_version fields and the ledger-stamps parse-rejected line.
@task_item n=8 part=needs
verify is the only stamps writer: the new row's stamp is written by verify alone, under its existing strict-modes and stamp-write refusals and its exit codes 0 and 1.
@task_item n=9 part=needs
Wrapped claude internals are not a contract: the browser-tools literal is a measured, version-stamped screen signature, and the extension-detected cache and feature flag that arm the prompt are read, never written or logged.
@task_item n=10 part=needs
CI verifies against the fake agent only and a real-CLI verify never runs in CI: the new screen, the row and the gate's refusal are proved on recorded fake-agent screens, and the live 24-row verify stays a planned, carried step.
@task_item n=11 part=needs
Recorded fixtures are no telemetry source and claude fixtures are scrub-required: any browser-tools screen added to the recorded set is scrubbed before it lands.
@task_item n=12 part=needs
A verify record word, if the plan adds one for the new row, joins the closed detail words additively, and no schema word is left that no build can emit.
@task_item n=13 part=needs
viola run stays silent on the terminal while its gate waits out its bound at the standing modal; the wait is visible only in the run process log's existing lines.
@task_standing n=1
convention: every line a new integration or E2E test makes a process emit is schema-checked against the diagnostic line schema, and the panic and schema gates run even after failed tests
home: ci-integration
@task_standing n=2
convention: every new test owns its user home, under the one integration-test root the secret scan covers, kept until the gate steps run
home: 9-ci-integration
@task_standing n=3
convention: a new test waits on a log line by reading for it, never by sleeping, and stays under the 20 s wait bound
home: 10-slo-invariants-telemetry-budgets
@task_standing n=4
convention: any new log line goes through the obs_event macro with a closed event value, allow-listed fields and no raw tracing or print macros
home: 11-obs-anti-patterns-never-do-these
@task_standing n=5
convention: a new fake-agent screen or recorded fixture carries only synthetic or scrubbed content, never a real host's screen
home: pii-scrubbing-compliance
@task_item n=1 part=not_needed
The MCP stdio server and its tools, frames, stdin limits and isError results: the chunk adds no tool and changes no MCP reply.
@task_item n=2 part=not_needed
The hook process (its stdout decision body, fail-open, stderr ban, latency and perf budgets) and the hook-borne permission dialog lines: the browser-tools offer is a screen modal read by the predicate, not a hook dialog, and no hook changes.
@task_item n=3 part=not_needed
The viola-channel JSON-RPC lines, their params, error codes and the conn and corr join between processes: no channel method or reply changes.
@task_item n=4 part=not_needed
viola revive, its preflight and the claude agents probe and its parse: the chunk does not touch revive or the agents listing.
@task_item n=5 part=not_needed
viola link and unlink, the events log's link records and the wheel's pause and release: no link or wheel behaviour changes.
@task_item n=6 part=not_needed
Heartbeat, liveness, list --json and the stale threshold: the standing modal changes no liveness reading.
@task_item n=7 part=not_needed
Snapshot replay, state-recovered lines and the start's trust-record line: the chunk writes no state and no trust record.
@task_item n=8 part=not_needed
The future opt-in error reporter, OTel export, network egress and the metric coverage sections: the chunk adds no exporter, reporter or derived metric.
@task_item n=9 part=not_needed
The CI artifact upload rules, retention and the secret-scan hit report's shape: the chunk adds no upload and no CI job.
