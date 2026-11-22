@task_prose part=chunk
Builds the lineage registry: a wrapped session's declared parent and role recorded ONCE at wrapper start as a new `lineage` event in that instance's own event log, the wrapper its one writer, read back the same after a crash and a replay.
Adds a launch-time parent declaration through `viola run` (a flag or the launching session's identity); what a starting wrapper can see of its parent is P3's first measurement, from a test or a no-start probe.
Draws the fleet as trees on `viola list` in the board's fixed columns, plain when stdout is not a terminal; what `list --json` gains is an operator fork.
A reused name never inherits an old parent; `revive` keeps the recorded parent; a parent edge is a drawing label only and never changes the wheel, the links or whether a send is accepted.
Shows observed links (0.1.0's link/unlink) apart from parent edges on every view that shows both, a link name shown only once it parses as a viola name.
@task_file n=1 path=crates/viola-core/src/lib.rs mark=edits
what: the event kinds and the event line contract of the core crate — edits: the event kind set (a new `lineage` kind)
@task_file n=2 path=crates/viola-core/src/obs.rs mark=edits
what: the obs event catalog of process log events — edits: the catalog entry for the lineage record
@task_file n=3 path=src/run/mod.rs mark=edits
what: the wrapper's `viola run` start path, the start events written in order to the instance event log — edits: the start path where the lineage line is written
@task_file n=4 path=src/cmd/run.rs mark=edits
what: the `viola run` verb and its CLI arguments — edits: the launch-time parent declaration
@task_file n=5 path=src/cmd/revive.rs mark=edits
what: the `viola revive` verb, its preflight and its restart of an instance — edits: the parent carried into the revived start
@task_file n=6 path=crates/viola-state/src/board.rs mark=edits
what: the board read model of the state crate, the rows `viola list` reads through read-or-replay — edits: the parent and lineage read
@task_file n=7 path=src/human/board.rs mark=edits
what: the human board of `viola list`, its fixed columns and captions, the unwrapped separator and the piped plain form — edits: the tree drawing and the piped form
@task_file n=8 path=src/cmd/list.rs mark=edits
what: the `viola list` verb and the `list --json` payload contract
@task_file n=9 path=src/run/links.rs mark=edits
what: the wrapper's observed session links from the link and unlink methods, where they are stored and shown — edits: links drawn apart from parent edges, link names name-checked before shown
@task_item n=1 part=needs
A new `lineage` event kind in the core event kinds, never Claude-named, carried on the standard event line: its format, keys, event kind round-trip and ts, and its data keys.
@task_item n=2 part=needs
The lineage line written once at wrapper start by the wrapper as the single writer of its own event log, and its place in the run start events order and the first records of a start.
@task_item n=3 part=needs
Lineage read back the same after a crash and a replay: the snapshot read-or-replay path, the replayed field set, replay writing no file, a torn tail healed by the next append.
@task_item n=4 part=needs
The parent declaration at launch through `viola run`: the CLI argument and its refusals, the declared parent name parsed as a viola name, the leading-slash and control-character rules on argv.
@task_item n=5 part=needs
A reused name never inherits an old parent: a start's lines are read from its own start offset, so a parent belongs to one instance's launch, not to its name.
@task_item n=6 part=needs
`viola revive` keeps the recorded parent: the revive start path, its preflight order, its exit codes and refusal codes held as they are.
@task_item n=7 part=needs
A parent edge is a drawing label only: the wheel at start, the link and unlink methods, and the send and answer refusal order stay unchanged by any parent.
@task_item n=8 part=needs
`viola list` human board drawn as trees in the fixed columns: the board columns and captions, the unwrapped separator, stale rows dim, list names and rows escaping control characters.
@task_item n=9 part=needs
Plain when piped: no SGR and no escape bytes in `viola list` output when stdout is not a terminal; SGR asserted only through an outer PTY.
@task_item n=10 part=needs
`list --json`: one JSON document per run, its items, skipped counts and name fields, its mirror of the channel; any new field a fork for the operator.
@task_item n=11 part=needs
Observed links shown apart from parent edges on every view showing both, a link name shown only after it parses as a viola name (raw-channel link name validation, link since kept, links surviving a wrapper restart).
@task_item n=12 part=needs
The board held to the read rule: list reads a live wrapper through the replay, a board refusal opens no log, the strict-modes check before any read.
@task_item n=13 part=needs
Unknown events counted and skipped: readers built before the new kind tolerate the `lineage` line, forward-compatible lines skipped.
@task_item n=14 part=needs
What the lineage record writes to disk, what outlives a SIGKILL of the wrapper, and what of a parent's identity lands in a file another session can read, stated at P3: owner-only file modes, no session names in the wrapper log, logs carry no secrets.
@task_item n=15 part=needs
Every new closed word, event kind, field, column, refusal or time bound is a closed-enum change that needs a decision: written as an operator fork at P3, never closed in code alone.
@task_item n=16 part=needs
Measurements at P3 come from a test or a no-start probe over the fake agent: no live start, no real claude CLI, no rig start, no mutation or witness run in this phase.
@task_standing n=1
convention: every new CLI test runs viola as a subprocess in its own fresh per-test home, waits keyed on logged events, no sleeps or retries
home: 5-integration-test-strategy · integration-test-strategy
@task_standing n=2
convention: every new human-mode output assertion is pinned by literals written in the test
home: 5-integration-test-strategy · integration-test-strategy
@task_standing n=3
convention: every new unit test is inline, table cases in rstest, named by the test function naming rule
home: 2-test-strategy · test-strategy
@task_standing n=4
convention: every new end-to-end test runs over the fake agent and asserts on events and records, never on the child's screen
home: 6-e2e-test-strategy · e2e-test-strategy
@task_standing n=5
convention: every new test's on-disk product state comes from real viola runs, never hand-written, with synthetic fixture text only
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=6
convention: every new test avoids the listed anti-patterns: no shared mutable state, no process env mutation, no Debug-string comparison, long-lived wrappers in a drop guard
home: 11-test-anti-patterns-never-do-these
@task_item n=1 part=not_needed
Hook dialog decision bodies, dialog answer mapping and dialog hold rules: the chunk adds no dialog and changes no hook decision.
@task_item n=2 part=not_needed
Send confirmation windows, paste framing, local-command post-conditions and the typed send forms: a parent edge changes no send, and no send path is built here.
@task_item n=3 part=not_needed
Viola verify stamping, ledger verify probes, recorded hook payload fixtures and the CLI version gate: no verify or stamp behaviour changes.
@task_item n=4 part=not_needed
Hook performance budgets, hyperfine invocation and the spine deadline: no hook path is touched.
@task_item n=5 part=not_needed
The fake agent's command-line options, receipts and screens: the chunk adds no fake-agent mode.
@task_item n=6 part=not_needed
Heartbeat staleness classification and liveness thresholds: the chunk draws liveness as it is and changes no classifier.
@task_item n=7 part=not_needed
Pause, release and the wheel's return through `viola release`: no wheel verb changes.
@task_item n=8 part=not_needed
Harness pre-push, supervisor, cleanup and secret-scan contracts: no harness command is added or changed.
@task_item n=9 part=not_needed
Fuzz workspace and nightly fuzz targets, and the MCP fixture scrub refusal codes: no fuzz target or MCP fixture is added.
@task_item n=10 part=not_needed
The absences of Windows and macOS support, a browser suite, a database and outbound HTTP: nothing here reaches them.
