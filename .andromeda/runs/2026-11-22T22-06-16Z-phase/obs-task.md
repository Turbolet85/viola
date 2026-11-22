@task_prose part=chunk
Builds the lineage registry: a wrapped session's declared parent and role recorded once, at wrapper start, as a new `lineage` event in that instance's own events.ndjson, read back the same after a crash and a replay.
`viola list` shows the fleet as trees in the board's six fixed captions and slot order, plain when stdout is not a terminal; what `list --json` gains is an operator fork.
`viola revive` keeps the parent the lineage recorded; a reused name never inherits an old parent; a parent edge is a drawing label only.
Observed links (0.1.0's link and unlink) are drawn apart from parent edges, a link name shown only once it parses as a viola name.
P3 first measures, by test or no-start probe only, how a parent is declared at launch and what a starting wrapper can see of it, what each new record writes to disk, what outlives a SIGKILL, and what of a parent's identity lands in a file another session can read.
@task_file n=1 path=crates/viola-core/src/lib.rs mark=edits
what: the product event kinds and the event line every events.ndjson record follows — edits: the event kind list (the new lineage kind)
@task_file n=2 path=src/run/mod.rs mark=edits
what: the viola run start path that writes the start's events into the instance's events log — edits: the start path where the lineage line is written
@task_file n=3 path=src/cmd/revive.rs mark=edits
what: the viola revive verb, its preflight and start arm — edits: the revive start (the parent carried)
@task_file n=4 path=crates/viola-state/src/board.rs mark=edits
what: the board read model the list view reads, built from the replayed state — edits: the parent edge and the observed links read apart
@task_file n=5 path=src/human/board/rows.rs mark=edits
what: the human board rendering in its fixed captions — edits: the tree drawing and the piped plain form
@task_file n=6 path=src/cmd/list.rs mark=edits
what: the viola list verb, its terminal output and its list --json payload — edits: the fleet tree output and the list --json payload
@task_item n=1 part=needs
A new product event kind, lineage, added only in viola-core's event kinds and never Claude-named; its line follows the events.ndjson event line shape, and the event kind is a fork for the operator.
@task_item n=2 part=needs
Where the lineage line falls in the run start events written to events.ndjson, the wrapper its one writer, recorded once per launch, never rewritten.
@task_item n=3 part=needs
The lineage read back the same after a crash and a replay: product readers read state through replay, never the raw snapshot, and a snapshot rebuilt by replay carries the parent as recorded.
@task_item n=4 part=needs
What survives a SIGKILL of the wrapper: the event line is one append write, a torn last line is healed by the appender and never by the reader, and the events log is never rotated.
@task_item n=5 part=needs
How a parent is declared at launch through viola run (a flag or the launching session's identity), measured at P3; the declared parent is self-reported, a drawing label and no authenticated identity.
@task_item n=6 part=needs
The parent's name and role are event payload, kept out of the process logs: no event payload copied into a diagnostics line, codes-only process lines, names never written to home-level files.
@task_item n=7 part=needs
Any new process-log line, event word, detail code or field the chunk adds stays inside the closed event enum and the additive-only diagnostics schema, the schema kept matching the enum, and each new word is an operator fork.
@task_item n=8 part=needs
viola list draws the fleet as trees in the board's six fixed captions, plain when stdout is not a terminal, with control characters in names escaped in terminal rendering.
@task_item n=9 part=needs
list --json's payload contract: what it gains for parent and role is an operator fork, its existing fields (liveness among them) left as they are.
@task_item n=10 part=needs
A reused instance name never inherits an old parent: a parent belongs to one instance's launch, not to its name.
@task_item n=11 part=needs
viola revive keeps the parent the lineage recorded; revive start logs as run, so the revived start writes its lineage under the same start path rules.
@task_item n=12 part=needs
A parent edge never changes the wheel, the links or whether a send is accepted: send refusal and wheel transitions read no lineage.
@task_item n=13 part=needs
Observed links (the events log link and unlink records, and the implicit link recorded by send and answer) shown apart from parent edges on every view that shows both, a link name shown only after it parses as a viola name.
@task_item n=14 part=needs
Link pair names and parent names never reach a process log; the link verbs' own logging stays as it is.
@task_item n=15 part=needs
The list verb's process log is opened only under a parsed viola name, and its refusal stays the unable and hint pair on stderr.
@task_item n=16 part=needs
viola-core emits no telemetry: the new event kind adds no logging inside viola-core.
@task_item n=17 part=needs
The run under the overseer's directive: no live start, no rig start of a real claude, no mutation run, no witness run; every measurement from a test or a no-start probe; recorded fixtures are synthetic and no telemetry source.
@task_standing n=1
convention: each new integration or E2E test that runs viola has its emitted diagnostics lines schema-checked, its test home under the one scanned root and kept until the gates run
home: ci-integration · 9-ci-integration
@task_standing n=2
convention: each new log line goes through the obs_event macro with its closed event value and explicit corr and instance; no raw tracing or print macros, no bare instrument
home: 11-obs-anti-patterns-never-do-these
@task_standing n=3
convention: each new test waits on a line or a state, never by sleeping, and its verdict is machine-asserted by exit code
home: 10-slo-invariants-telemetry-budgets
@task_item n=1 part=not_needed
The hook path: hook stdout, stderr, fail-open, capture arms, hook decision details and hook perf budgets; the chunk adds no hook behaviour.
@task_item n=2 part=not_needed
The dialog path: dialog raise, answer, expiry and their log fields and spans; lineage touches no dialog.
@task_item n=3 part=not_needed
viola verify, plugin install and the capability ledger: their probes, stamps, spawn subjects and exit codes; the chunk adds no verify step.
@task_item n=4 part=not_needed
The MCP stdio server's framing: stdin size bounds, oversize lines, frame-tool refusals and JSON-RPC errors; the chunk changes no MCP frame.
@task_item n=5 part=not_needed
Panic capture and error reporting: the panic line, panic outcome by role, backtraces, and any future error reporter; the chunk adds no catch site.
@task_item n=6 part=not_needed
Telemetry export and absences: OTel SDK, exporters, traceparent, network egress, hosted deployment, on-call paging.
@task_item n=7 part=not_needed
CI artifact handling: the secret scan, its hit report, upload gating, artifact retention and SHA-pinned actions; the chunk adds no CI job or upload.
@task_item n=8 part=not_needed
The PTY seam and the claude child: paste, spawn, kill-fallback, environment strip and the version gate's probe lines; lineage is written beside them, not through them.
@task_item n=9 part=not_needed
Channel socket safety: socket-directory and strict-modes refusals, server verification by peer credentials and their exit-21 cause words; the chunk adds no connect path.
