@task_prose part=chunk
Builds the lineage registry: a new agent-neutral `lineage` event recorded once when the wrapper starts, holding a wrapped session's declared parent and role, in that instance's own events log with the wrapper its one writer, read back the same after a crash and a snapshot replay.
Adds how a parent is declared at launch through `viola run` (a flag or the launching session's own identity); a parent belongs to one instance's launch, never to its name, and `revive` keeps the parent the lineage recorded.
Changes `viola list` to show the fleet as trees in the board's six fixed captions and slot order, plain when stdout is not a terminal; what `list --json` gains is left as an operator fork.
Shows observed links (0.1.0 session links) apart from parent edges on every view that shows both, a link name shown only after it parses as a viola name; a parent edge is a drawing label only and never moves the wheel, the links or send acceptance.
Measures first at P3, from tests or no-start probes only: what a starting wrapper can see of its parent, what each new record writes to disk, what outlives a SIGKILL, and what of a parent's identity lands in a file another session can read.
@task_file n=1 path=crates/viola-core/src/lib.rs mark=edits
what: the normalised event kinds and the event line shape, the agent-neutral core every other crate depends on — edits: the event kind set (a new `lineage` kind) and its event data
@task_file n=2 path=crates/viola-core/src/obs.rs mark=edits
what: the obs event catalog behind the one event emitter, the codes-only process-log lines
@task_file n=3 path=src/run/mod.rs mark=edits
what: the wrapper's start path and start order, where a wrapped instance's own events log is first appended — edits: the start path (where the lineage line is written)
@task_file n=4 path=src/cmd/run.rs mark=edits
what: the `viola run` verb and its CLI arguments, the launch of a wrapped session — edits: the launch-time parent declaration
@task_file n=5 path=src/cmd/revive.rs mark=edits
what: the `viola revive` verb, reviving a gone instance in place through the start path from its snapshot — edits: the parent carried
@task_file n=6 path=crates/viola-state/src/replay.rs mark=edits
what: the snapshot replay that recovers the log-derived fields of an instance from its events log
@task_file n=7 path=crates/viola-state/src/board.rs mark=edits
what: the board read model, sync and Claude-free, shared by CLI list and MCP list — edits: the fleet tree (parent edges, links apart)
@task_file n=8 path=src/cmd/list.rs mark=edits
what: the `viola list` verb, a caller-side read of state with no wrapper, its human text and its `--json` list payload — edits: the piped plain form, the list payload
@task_file n=9 path=src/human/board.rs mark=edits
what: the human board drawing of `viola list` in the board's fixed captions and slot order — edits: the tree drawing, the plain form when stdout is not a terminal
@task_file n=10 path=src/human/board/rows.rs mark=edits
what: the board's row and cell rendering under the fixed captions — edits: tree rows, parent edges and links drawn apart
@task_file n=11 path=src/run/links.rs mark=edits
what: the wrapper's link set of observed driver-to-driven links, recorded as log events and shown on the board — edits: links shown apart from parent edges, name-checked
@task_file n=12 path=src/cmd/link.rs mark=re-reads
what: the 0.1.0 `link` / `unlink` verbs and their observed links, re-read for where links are stored and shown
@task_item n=1 part=needs
A new `lineage` event kind: added only in the core crate, agent-neutral and never Claude-named, its wire value kebab-case, its line held to the event line shape (v, timestamp, source, data) and given its own event data per kind.
@task_item n=2 part=needs
The wrapper is the one writer of the lineage line, appended once at start into that instance's own events log, one write per line, at a place in the start order where a failed start leaves no half-registered instance.
@task_item n=3 part=needs
The lineage reads back the same after a crash and a replay: the snapshot replay recovers it as a log-derived field, a torn last line is healed by the next append, and an older reader meeting the new kind or field tolerates it (versioned tolerant records, missing equals null, skipped lines counted).
@task_item n=4 part=needs
A parent declared at launch through `viola run`: its CLI argument style, the declared name parsed as a viola name and refused, never stripped, on control characters; which names, sockets or records a starting wrapper can see of its parent (its environment, the child environment names) is P3's first measurement.
@task_item n=5 part=needs
A reused name never inherits an old parent: identity is by instance, never by name or location, and the existing refusals of a start over a held, live or stale name stay as they are.
@task_item n=6 part=needs
`revive` keeps the parent its lineage recorded: revive goes in place through the start path, reads the snapshot once, and must carry the recorded parent rather than re-declare one.
@task_item n=7 part=needs
A parent edge is a drawing label only: it never changes who holds the wheel, never adds or removes a link (the link set changes only on implicit links and link calls and follows the log), and never decides whether a send is accepted or refused.
@task_item n=8 part=needs
`viola list` as fleet trees: the board's six fixed captions, their words and slot order unchanged; plain when stdout is not a terminal, with colour variables deciding only SGR output; list stays a caller-side read of state with no wrapper, without pagination, and the whole board is still refused on one failed check.
@task_item n=9 part=needs
The board contract shared by CLI list and MCP list and the list payload items: what `list --json` gains is an operator fork, and any new field keeps snake_case names, declared key order and absent optional fields omitted.
@task_item n=10 part=needs
Observed links drawn apart from parent edges on every view that shows both, and a link name shown only after it parses as a viola name; a name read from disk is untrusted upstream text, sanitised before any human line.
@task_item n=11 part=needs
Every new closed word, event kind, field, column, refusal, time bound or split is a fork for the operator, written priced into one block of the run's founder-forks file at P3's end; every master row the change reaches is named after a search of all seven masters.
@task_item n=12 part=needs
Disk and privacy of the new record: what the lineage line writes to disk, what of it outlives a SIGKILL (crash-safe writes, never-rotated events log), and what of a parent's identity lands in a file another session can read (content-bearing detail only in the instance, file mode set before any byte, codes-only process logs) are stated at P3.
@task_item n=13 part=needs
Any new log line goes through the one event emitter into the obs event catalog and carries codes only, never a parent's name or role text.
@task_item n=14 part=needs
Whether the new kind or payload field bumps the protocol version follows the protocol-version bump rule and mixed-version tolerance between an older and a newer binary on one home.
@task_item n=15 part=needs
Proof under the directive: measurements from tests or no-start probes only — no live start, no rig start of a real claude, no witness run, no mutation run in this phase; the tests run against the fake agent in isolated viola homes.
@task_item n=16 part=needs
The live-host leg of the lineage registry capability (the overseers run wrapped with their parents declared) cannot be proved here; whether it is claimable whole is read at P4/P5.
@task_standing n=1
convention: each new test runs in its own viola home against the fake agent, never a live claude, its home removed by its owner record
home: established-decisions · occupied-resources
@task_standing n=2
convention: each new JSON field is snake_case, each wire enum value kebab-case, absent optional fields omitted on write, missing read as null
home: conventions
@task_standing n=3
convention: each new log line is emitted through the one event emitter, codes only, at the role's diagnostics level
home: occupied-resources · build-system
@task_standing n=4
convention: each new module is reached from the crate tree (no orphan modules), uses no print or dbg macro, and passes fmt and clippy with warnings denied
home: build-system
@task_standing n=5
convention: each new test seam or test-support code stays out of product builds and is no configuration channel
home: occupied-resources
@task_standing n=6
convention: each new OS-gated test names its reason and no test gate names another OS
home: cross-cutting-patterns
@task_standing n=7
convention: new code holds the per-crate coverage floors enforced by the harness gate
home: ci-cd-approach
@task_item n=1 part=not_needed
Dialog handling: the dialog hold, pending dialogs, answer contracts and the permission, plan and question hook contracts; the lineage is not a dialog.
@task_item n=2 part=not_needed
The hook-to-kind map and hook tiers: the lineage line is written by the wrapper at start, never mapped from a Claude Code hook.
@task_item n=3 part=not_needed
Send delivery mechanics: bracketed paste, the paste bound and cut mark, confirmation windows, the readiness gate, screen signatures and local command classification.
@task_item n=4 part=not_needed
The capability ledger, `viola verify`, the ledger stamps envelope, the trust writer code words and real CLI probes.
@task_item n=5 part=not_needed
MCP server internals: the stdin line cap and oversize handling, rmcp features, the MCP call deadline and wait cap.
@task_item n=6 part=not_needed
Plugin install, the marketplace tree and the plugin embedded in the binary.
@task_item n=7 part=not_needed
The Hyprland compositor seam and window features: `viola open`, `focus`, `up` and `adopt` are their own Epoch 5 entries.
@task_item n=8 part=not_needed
Release tooling, dist, crates.io publishing, licence and deployment model.
@task_item n=9 part=not_needed
The fuzz workspace, its nightly toolchain and weekly fuzzing.
