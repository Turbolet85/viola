@task_prose part=chunk
Adds two relied capability-ledger rows in viola-agent-claude for claude behaviours relied on today with no row: the harness prompt prefixes and the R8 identity floor of eleven CLAUDE parent-identity names.
Each new row gets a viola verify probe: a second session sending into a verify child whose prompt's raw start is read for the harness-origin prefix, and a capture of environment NAMES, never values.
The new probes are replayed by the fake agent in CI and recorded through verify --record and its scrub into a recorded fixture set.
The new rows are stamped live, counted in a viola verify stamp from a live run in the rig's scratch home only; the live home's stamps stay untouched.
Re-reads what the tree can show of the 27 relied rows, the stamped fixture sets and the default-version constants.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the capability ledger of the claude agent: its relied rows, their verify probes and the stamp's relied-id set — edits: the relied-row list, the ledger's rows, the stamp's relied-id set
@task_file n=2 path=crates/viola-agent-claude/src/ledger/test_cases.rs mark=edits
what: the capability ledger's tests: the relied rows, their runs, checks and stamps
@task_file n=3 path=crates/viola-agent-claude/src/hook.rs
what: the hook seam of the claude agent: the harness prompt prefixes, the compiled raw-prefix set that files a prompt as a harness turn, and the hidden hook --capture arm
@task_file n=4 path=crates/viola-agent-claude/src/lib.rs
what: the claude agent's child environment rule: IDENTITY_FLOOR, the eleven CLAUDE parent-identity names always stripped from the child's environment
@task_file n=5 path=fixtures/claude/2.1.328
what: the recorded claude fixture set for one CLI version, written by viola verify --record and its scrub, replayed by the fake agent in CI
@task_item n=1 part=needs
Two new relied ledger rows, harness prompt prefixes and the R8 identity floor, each with a viola verify probe; the relied-row count and the stamp's relied-id set grow past 27.
@task_item n=2 part=needs
viola verify's child set and runs gain the harness-prefix probe: a second session sends into a verify child and the prompt's raw start is read to tell the harness-origin prefix.
@task_item n=3 part=needs
The identity-floor probe captures environment variable names only, never values: no value may reach any file, fixture, stamp, log line or screen read, and a failing test proves a captured value is refused.
@task_item n=4 part=needs
Stripped CLAUDE values are never logged; the R8 strip is logged as names only, and the claude child's process-start line carries env_kept, env_stripped_count and env_stripped_known as they stand.
@task_item n=5 part=needs
The identity floor is always stripped from the claude child's environment; the eleven measured names stay the floor the new row relies on.
@task_item n=6 part=needs
The harness prompt prefixes file a prompt as a harness turn that is not human and never moves the wheel; the new row relies on that set as it stands.
@task_item n=7 part=needs
The hidden hook --capture arm and its closed ids: any new capture id stays closed, and capture arms stay uninstrumented and write no lines.
@task_item n=8 part=needs
verify --record and its scrub write the new probes into a recorded fixture set; claude fixtures are scrub-required, a name that itself carries private text is scrubbed, and recorded fixtures are no telemetry source.
@task_item n=9 part=needs
verify log lines carry no content: no endpoint, binary path, env value, prompt text or screen bytes; verify logs only inside an instance.
@task_item n=10 part=needs
verify's spawn subjects (version-probe, verify-probe, verify-pty-probe, agents-probe) and its process-exit lines with child_exit_status and duration_ms cover any child the new probes spawn; a new subject joins the closed subject enum.
@task_item n=11 part=needs
verify is the only stamps writer; capability ledger path and stamps are never logged; an unreadable ledger means an unverified CLI; a ledger-stamps parse rejection is a parse-rejected line.
@task_item n=12 part=needs
verify stdout step and summary lines count the new rows; verify exit codes 0 and 1 and its strict-modes refusals (home tree, stamp write) stay as they are.
@task_item n=13 part=needs
CI runs verify against the fake agent only, replaying the new probes; real-CLI verify never runs in CI.
@task_item n=14 part=needs
Stamped live in the rig's scratch home only: no live start and no read of the operator's real claude config in this phase; the live home's stamps, config.json and the pinned product stay untouched.
@task_item n=15 part=needs
No telemetry from screen content: what the verify child shows may be read for the probe's verdict but never logged; no compiled screen literal is printed.
@task_item n=16 part=needs
Wrapped claude internals and transcripts are not a contract: every dependence on an undocumented claude behaviour is a ledger row with a verify probe.
@task_item n=17 part=needs
No retry policies and no sleep-based log waits in the new probes or their tests; a red is read, not re-run.
@task_item n=18 part=needs
Environment variables are not a configuration channel: the captured names configure nothing.
@task_standing n=1
convention: every new test's emitted diagnostics lines are schema-checked, and the schema stays additive only
home: ci-integration · 9-ci-integration
@task_standing n=2
convention: new integration-test homes live under the one scanned root and are kept until the gates run
home: 9-ci-integration · ci-integration
@task_standing n=3
convention: new fixtures carry synthetic input only and pass the secret scan before any upload
home: 8-pii-scrubbing-compliance · pii-scrubbing-compliance
@task_standing n=4
convention: every new log line goes through the one obs_event macro with a closed event value and default-deny fields
home: 11-obs-anti-patterns-never-do-these
@task_standing n=5
convention: new obs-touching code is mutation-tested like product code and fails closed on a surviving mutant
home: ci-integration · 10-slo-invariants-telemetry-budgets
@task_item n=1 part=not_needed
The viola mcp stdio server tools, frames and their refusals: the chunk adds no tool and changes no MCP output.
@task_item n=2 part=not_needed
The confirmed send path, the confirm window and its span chain: the chunk changes no send.
@task_item n=3 part=not_needed
Dialog raise, answer and hold: no dialog behaviour changes.
@task_item n=4 part=not_needed
viola revive, link and unlink output and process logs: untouched.
@task_item n=5 part=not_needed
Heartbeat liveness, staleness and the list and agent-run status outputs: untouched.
@task_item n=6 part=not_needed
A future error reporter, OTel export, Sentry and network egress: nothing the chunk adds exports anything.
@task_item n=7 part=not_needed
Wait and last spans, their log fields and connect refusals: untouched.
