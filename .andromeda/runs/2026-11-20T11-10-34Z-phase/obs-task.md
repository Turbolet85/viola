@task_prose part=chunk
A question dialog whose first raise reaches viola through the PermissionRequest hook is answered, on the driver's answer carrying the answers, with an allow decision body whose updatedInput carries those answers, where today that question stays null to the human.
A new relied capability-ledger row in the claude agent crate for that body, with its viola verify probe; the dialog decision stays gated on a verify stamp on which every relied row counts.
The CLI versions are re-stamped against the grown relied set (which versions and which home is the founder's word at P3); no live start this phase.
Capability v2-20: the driver answers a permission-request question with a body carrying the answers, never leaves it to the human; the new row has its probe and the re-stamped versions pass it.
@task_file n=1 path=crates/viola-agent-claude/src/dialog.rs mark=edits
what: the claude dialog component, the mapping from the driver's answer to the dialog decision — edits: the dialog answer mapping
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the capability ledger, its relied rows and their count, read to decide a verified CLI — edits: the relied row list and its count
@task_file n=3 path=crates/viola-agent-claude/src/hook.rs mark=edits
what: the claude hook decision body the hook writes on stdout — edits: the PermissionRequest decision body (allow with updatedInput)
@task_file n=4 path=src/cmd/verify.rs mark=edits
what: viola verify, its probes and the stamp it writes over the relied row set — edits: the new probe, the verify stamp's row set
@task_file n=5 path=src/cmd/hook.rs mark=edits
what: viola hook, the dialog hook role answering PermissionRequest — edits: the PermissionRequest path for a question
@task_file n=6 path=src/run/dialog.rs mark=edits
what: the run wheel's dialog relay between the hook and the driver's answer — edits: the PermissionRequest question relay
@task_file n=7 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent CI verify runs against — edits: the PermissionRequest question script
@task_file n=8 path=fixtures/claude/2.1.328 mark=edits
what: the recorded claude fixtures for a CLI version, scrub-required
@task_item n=1 part=needs
The PermissionRequest hook decision for a question: an allow decision body whose updatedInput carries the driver's answers, written as the hook's stdout decision body and nothing else, the hook exiting 0.
@task_item n=2 part=needs
The hook's fail-open and silence rules still bind the new path: no stderr from the hook, a failed check or unreachable channel fails open with its hook-decision detail code, a hook without a trusted instance stays silent.
@task_item n=3 part=needs
Dialog content stays out of every log: the question, option and answer text the body carries is content, never logged on raise, answer or hook decision; only codes and typed fields reach the process log.
@task_item n=4 part=needs
The dialog path's log lines for the question case: dialog-raised, dialog-answered, hook-invoked and hook-decision with their required fields, corr by event and event order, deadline_hit on the hook-decision line.
@task_item n=5 part=needs
The dialog decision stays gated on a verify stamp on which every relied ledger row counts; an unreadable ledger or a stamp missing the new row means an unverified CLI, and the driver-side answer refusal unverified-cli stands.
@task_item n=6 part=needs
The capability ledger grows one relied row and its count; ledger path and stamps are never logged; the run log's cli_verified and cli_version and the ledger-stamps parse-rejected line keep their words.
@task_item n=7 part=needs
The new verify probe: its child spawn subject and process-exit line, its verify stdout step line and summary line, verify lines carrying no content, verify the only writer of stamps, verify exit codes unchanged.
@task_item n=8 part=needs
The human always wins: a refusal reaches automation only, the human at the keyboard can answer the dialog himself at any moment, a concurrent second dialog gets null, and an expired dialog hold (dialog_hold_s, the dialog-expired event) returns the dialog to the human with today's conservative fall-back.
@task_item n=9 part=needs
viola stays mechanism: it carries the driver's answers verbatim and never chooses them; option and answer text is content, never a command, and no telemetry is taken from screen content.
@task_item n=10 part=needs
The answer surface the driver uses, the viola-mcp answer tool and the CLI answer verb, with its refusals not-delivered and unverified-cli and its implicit link; its shape changes only if the answers' shape must.
@task_item n=11 part=needs
CLI versions re-stamped against the grown relied set; verify against the real CLI never runs in CI, CI verifies against the fake agent only, and no live start or read of the operator's real home happens in this phase.
@task_item n=12 part=needs
Wrapped claude internals are not a contract: the updatedInput body shape is read from recorded fixtures, which are scrub-required and never a telemetry source; a drift of that shape reports to the diagnostics detail file only.
@task_item n=13 part=needs
The PermissionRequest hook path stays inside the hook latency and observability overhead budgets and their perf gate, measured on the maximum sample.
@task_item n=14 part=needs
The open carry of the intermittent rig-rehearsal red is acted on only at its next red: read the printed rig summary and kept frame, name the cause, then mend the wait; no retry rule, no sleep-based log wait, no loosened entry.
@task_standing n=1
convention: each new log line goes through the obs_event macro with a closed event value, an allow-listed field set and no content, and is schema-checked by the conformance gate
home: 11-obs-anti-patterns-never-do-these
@task_standing n=2
convention: each new integration or E2E test keeps its home under the one scanned test-home root, kept until the panic and schema gates run, with its emitted lines schema-checked
home: 9-ci-integration · ci-integration
@task_standing n=3
convention: each new or recorded claude fixture is scrubbed before it is committed and holds synthetic content only
home: 1-obs-scope-summary · obs-scope-summary
@task_item n=1 part=not_needed
The send path: send-issued, send-confirmed and send-refused lines, the confirm window, the readiness gate and their spans; the chunk sends no text.
@task_item n=2 part=not_needed
The wait and last paths, their spans, log fields and connect refusals.
@task_item n=3 part=not_needed
Run and revive start preflight: exit-1 detail codes, collision check, pin copy, trust record, the start's span attributes and start events order.
@task_item n=4 part=not_needed
The explicit link and unlink methods, their invalid-params refusal and their link and unlink events.
@task_item n=5 part=not_needed
Liveness and heartbeat: the heartbeat file, list liveness and staleness thresholds.
@task_item n=6 part=not_needed
State recovery and snapshot replay: state-recovered lines, torn-line healing, replayed snapshot fields.
@task_item n=7 part=not_needed
The claude agents probe and the hyprctl probe with their parse-rejected parsers; the chunk adds no agents or hyprland read.
@task_item n=8 part=not_needed
Metric coverage: counters, gauges, histograms and quantiles derived from logs; the chunk adds no metric.
@task_item n=9 part=not_needed
Future error reporter rows: its transport, opt-in and egress entry; no reporter is added.
@task_item n=10 part=not_needed
PTY seam log events for spawn, paste and kill-fallback, and the wheel's typed-key and human-typing send refusal.
