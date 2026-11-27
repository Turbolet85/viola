@task_prose part=chunk
A corrective chunk that mends the Epoch 5 audit's ranked defects. It adds no product behaviour, no flag, no event kind, no refusal code and no master rule.
It makes the root crate's mutation baseline green by moving the one whole-verify test by name into the verify class's longer kill, then runs the root crate's in-diff mutants ONCE, behind an announced stop, and lands the score as an addendum to the code audit's ledger record.
It adds killing tests, each with a remove-the-guard control, for viola-pty's nine restart-path survivors and viola-state's three survivors (the ancestor-walk bound first).
It adds a pre-start precondition to the r2 open rehearsal script: refuse on a locked desktop or a scratch dir that is not fresh.
It makes three pure structural moves: one shared start prologue for revive and adopt, one refusal helper for open and up, and three file splits (up's upped, the pty pump's restart code, run's window and trust steps). Each is re-measured against the audit's complexity and size ceilings, and the existing layers are re-run green.
@task_file n=1 path=.config/nextest.toml mark=edits
what: the nextest profiles, the mutants profile with its per-class kill overrides — edits: the verify_window_ override (the whole-verify test joins it by name; the general 15 s limit unchanged)
@task_file n=2 path=tests/cli_answer.rs mark=re-reads
what: the baseline check, the path-4 stamp-failing-dialog-rows test that runs its own viola verify, re-run under the mutants profile with no TIMEOUT
@task_file n=3 path=crates/viola-pty/src/pump.rs mark=edits
what: the PTY output pump, its restart path: the generation guard on which output copy ended and the time limits in reap_within and drain — edits: the restart code (moved out to its own file), new killing tests for the nine survivors
@task_file n=4 path=crates/viola-state/src/liveness.rs mark=edits
what: the state crate's liveness reading, the ancestor-pid walk ancestors(pid, max) bounded at 8 ancestor pids — edits: new killing tests for the three survivors, the chain.len() < max bound first
@task_file n=5 path=src/cmd/revive.rs mark=edits
what: the viola revive verb, its start prologue: the home strict-modes check that runs before the log opens, then the log open — edits: the start prologue (made the one shared function)
@task_file n=6 path=src/cmd/adopt.rs mark=edits
what: the viola adopt verb, its copy of revive's start prologue — edits: the prologue copy (replaced by a call to the shared prologue)
@task_file n=7 path=src/cmd/up.rs mark=edits
what: the viola up verb, its upped function and its refused() refusal helper — edits: upped (split out), refused() (made the one shared definition)
@task_file n=8 path=src/cmd/open.rs mark=edits
what: the viola open verb, its byte-identical copy of the refused() refusal helper — edits: refused() (replaced by a call to the shared one)
@task_file n=9 path=src/cmd/run.rs mark=edits
what: the viola run start, on the boot path, with its window step and trust-record step — edits: the window and trust steps (moved out to their own files)
@task_file n=10 path=scripts/r2-open.sh mark=edits
what: the r2 open live-rehearsal script, its pre-start preconditions — edits: the desktop-not-locked (hyprctl locked) and fresh-scratch-dir checks before any start
@task_item n=1 part=needs
The restructure's invariant. The units that move are: the revive start prologue, now shared with adopt (the home strict-modes check before any log opens, which writes no line on refusal, then the log open, in revive's preflight order); up's upped; the pump's restart code (the generation guard and the reap_within and drain time limits); run's window step and trust-record step; and the refused() helper of open and up (their --json refusal document and the stderr unable/hint pair). Each does what it did. This is held by the whole layers green with no test edited beyond a re-pointed path or name, and by the smoke trio for run's boot path.
@task_item n=2 part=needs
Moved code keeps how it is instrumented. Its lines go out only through the obs_event macro, with no raw tracing or print macros. Its spans keep static names, skip all arguments and use an explicit field allow-list. Steps that were uninstrumented (the window line, the window view restore, the trust-record write) stay uninstrumented, and the trust-record log line stays as it is.
@task_item n=3 part=needs
The chunk adds no event value, detail code, log field, refusal word or flag. The closed event enum, the detail codes and the diagnostics schema stay as they are, so the shared prologue and the shared refused() emit exactly the lines and codes their callers emitted, strict-modes-failed and the exit-1 detail of up and open included.
@task_item n=4 part=needs
A moved unit stays inside its own crate, in a module of the verb or pump it came from, and exposes no more than it did. The shared prologue and refused() are crate-internal functions both verbs call. A test reaches a moved unit only through its re-pointed path or name.
@task_item n=5 part=needs
The measure for the splits is the audit's own: complexity over its ceiling and file size over 800 lines. Every moved file and every remaining file must fall under both, and the next audit's M1, M2 and M3 rows must stop listing the refused() pair, upped, the pump, and run.
@task_item n=6 part=needs
The mutants nextest profile: the path-4 whole-verify test moves by name into the verify_window_ class's 20 s x 4 kill. Any other test P3's census finds that runs a whole verify moves with it. The general 15 s x 3 kill is not raised. The unmutated viola baseline under that profile passes with no TIMEOUT.
@task_item n=7 part=needs
The one mutation run covers the root crate's 372 in-diff mutants, in the recorded form (in-diff from 15a566e8, copy-target false, 5 jobs, a kill deadline, TMPDIR a short-named dir under the mutants scratch root). It runs once, after the overseer's go. The addendum records a mutation:viola score with caught, missed, timeout and unviable counts, held to the rule that obs code is mutation-tested like product code. Its outputs and diff stay local and are never uploaded.
@task_item n=8 part=needs
viola-pty's nine restart-path survivors get one killing test per survivor class: the generation guard on which output copy ended, the reap_within time limit and the drain time limit. Each test is shown red with the guard removed and green with it. No mutation run proves them.
@task_item n=9 part=needs
viola-state's three survivors get one killing test each with its remove-the-guard control. The ancestor-walk bound comes first: chain.len() < max, held to the security plan's limit of 8 ancestor pids.
@task_item n=10 part=needs
The r2 open script's new behaviour: before starting anything it refuses on a locked desktop (hyprctl locked) and on a scratch dir that is not fresh, the same preconditions its sibling scripts read. A check proves this through a rehearsal or a hyprctl stub, running no live arm.
@task_item n=11 part=needs
How the chunk is proved: no live start, no window opened, nothing dispatched to the compositor, no key sent, no real claude run. Tests run against the fake agent and synthetic test homes, never the operator's real ~/.claude* or ~/.viola. A real-CLI verify never runs.
@task_item n=12 part=needs
run's start is on the boot path, so after its window and trust steps move, the run start events and the start's process log lines keep their order: process-start, the trust-record line, the window line, process-exit.
@task_standing n=1
convention: each new test's home lives under the single integration-test home root that the secret scan covers, and stays until the gate steps have run
home: ci-integration · 9-ci-integration
@task_standing n=2
convention: each new test's emitted diagnostics lines are schema-checked, and no test run may leave a panic line, a nonzero count or a surviving obs mutant
home: ci-integration · 9-ci-integration
@task_standing n=3
convention: a new test that waits on a log line or a child never waits by sleeping; budgets and verdicts are asserted by exit code
home: 10-slo-invariants-telemetry-budgets · slo-invariants-telemetry-budgets
@task_standing n=4
convention: new fixtures and test inputs are synthetic; recorded claude fixtures are scrub-required and are never a telemetry source
home: 1-obs-scope-summary · obs-scope-summary
@task_item n=1 part=not_needed
The MCP stdio server's tools, frames, stdin MAX_FRAME and isError results: the chunk touches no MCP role.
@task_item n=2 part=not_needed
The hook role's stdout decision body, fail-open, instance check, latency budgets and hook perf gates: no hook code is moved or added.
@task_item n=3 part=not_needed
Send, wait, last, wheel and dialog log lines, their spans and their corr-by-event rules, and the channel's conn and corr join keys: no message or dialog path is touched.
@task_item n=4 part=not_needed
The future error reporter, OTel SDK, exporters, Sentry and network egress: the chunk adds no dependency, sink or exporter.
@task_item n=5 part=not_needed
Metric coverage (counters, gauges from harness snapshots, histograms, quantiles): the chunk derives no metric.
@task_item n=6 part=not_needed
Service identity (the version constant, service.name, the process-start os and pid fields) and the harness logs and agent-run logs commands' output: none of them changes.
@task_item n=7 part=not_needed
The detail-file line schema, panic backtrace capture, and the veil redaction of payload types: no payload type or detail record is added.
