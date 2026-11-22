@task_prose part=chunk
Builds the lineage registry: a wrapped session's declared parent and role are recorded once, at wrapper start, as a new `lineage` event in that instance's own events.ndjson, the wrapper its one writer, and read back the same after a crash and a replay.
A parent is declared at launch through `viola run` (a flag or the launching session's own identity); what a starting wrapper can see of its parent is P3's first measurement, taken from a test or a no-start probe.
`viola list` shows the fleet as trees in the board's six fixed captions and slot order, plain when stdout is not a terminal; `revive` keeps the recorded parent; a reused name never inherits an old parent.
A parent edge is a drawing label only: it never changes the wheel, the links or whether a `send` is accepted; observed `link`s are drawn apart from parent edges on every view that shows both, a link name shown only after it parses as a viola name.
No live start, no real `claude`, no mutation run, no witness run; every new event kind, field, column, refusal or time bound is an operator fork written at P3's end.
@task_file n=1 path=crates/viola-core/src/lib.rs mark=edits
what: the core crate's event kinds and shared validators, behind the event line contract — edits: the event kinds (a new `lineage` kind, never Claude-named)
@task_file n=2 path=crates/viola-core/src/obs.rs mark=edits
what: the obs event catalog of process log lines — edits: the catalog entry for the lineage record
@task_file n=3 path=src/run/mod.rs mark=edits
what: the wrapper's start path, where the instance's event log opens and its first lines are written — edits: the start path (where the lineage line is written once)
@task_file n=4 path=src/cmd/run.rs mark=edits
what: the `viola run` verb, the CLI input trust boundary at launch — edits: the launch-time parent declaration
@task_file n=5 path=src/cmd/revive.rs mark=edits
what: the `viola revive` verb that restarts a recorded session from its snapshot — edits: the parent carried into the revived instance
@task_file n=6 path=crates/viola-state/src/board.rs mark=edits
what: the board read model that the board scan of `viola list` builds from each instance's state — edits: the parent edges and observed links read into the board
@task_file n=7 path=crates/viola-state/src/replay.rs mark=edits
what: the event log replay that rebuilds an instance's state after a crash — edits: the lineage read back on replay
@task_file n=8 path=crates/viola-state/src/events.rs
what: the events.ndjson reader and appender with bounded, torn-line-tolerant lines
@task_file n=9 path=src/human/board/rows.rs mark=edits
what: the human board's rows under the six fixed captions — edits: the tree drawing, the piped plain form, links drawn apart from parent edges
@task_file n=10 path=src/cmd/list.rs mark=edits
what: the `viola list` verb, its human form and its `--json` payload contract
@task_file n=11 path=src/run/links.rs
what: the wrapper's observed session links (the 0.1.0 `link` / `unlink` verbs), as stored in events.ndjson link lines and restored
@task_item n=1 part=needs
A new event kind in the core crate's closed event kinds, carried on the event line contract: closed types for its protocol fields, versioned-tolerant parsing by its readers, never Claude-named.
@task_item n=2 part=needs
The lineage line is written by the instance's own wrapper only, once at start, after the home tree's strict-modes check and before nothing else may write it; its file keeps the owner-only modes of the viola home and the appender heals a torn last line.
@task_item n=3 part=needs
Reading the lineage back on replay and on `list`: through the bounded event log reader, tolerant of torn lines, after the strict-modes check; a replayed snapshot stays inert and runs nothing.
@task_item n=4 part=needs
The declared parent and role cross the CLI input trust boundary at `viola run`: a parent name passes the viola name check through the validated name type before any record is written; an unwrapped agent or session name is never a target.
@task_item n=5 part=needs
Where the parent is taken from the launching session's own identity: the parent identity variables stay stripped from the child's environment, the environment is no configuration channel, and a new environment read needs its log entry.
@task_item n=6 part=needs
What of a parent's identity lands on disk, outlives a SIGKILL, and is readable by another session: same-OS-user trust boundary, plaintext state under OS permissions, no paths, pids or ids in refusals or external errors; stated at P3.
@task_item n=7 part=needs
A parent belongs to one instance's launch, not to its name: a reused name never inherits an old parent; instance paths only from validated names; the snapshot written only by its wrapper.
@task_item n=8 part=needs
`revive` keeps the recorded parent: the recorded values are read only after revive's strict-modes check, used as recorded and never trusted as a command; any field `revive --json` gains is a contract change and an operator fork.
@task_item n=9 part=needs
A parent edge is a drawing label only and authenticates nothing: like a self-reported from, it grants no access, never changes the wheel, the links, nor whether a `send` is accepted or refused; channel access stays all-or-nothing.
@task_item n=10 part=needs
`viola list` trees: the board scan keeps its home read rule and whole-board refusal, the strict-modes refusal of list in human form and under `--json` unchanged; human output escapes C0/C1 controls in every drawn name; colour and width chosen only from terminal facts, plain when stdout is not a terminal.
@task_item n=11 part=needs
`list --json` payload: whatever it gains is a fork for the operator; any new value is closed and read by its clients as closed values only.
@task_item n=12 part=needs
Observed links drawn apart from parent edges, and a link name shown only after it parses as a viola name (the trust link restore gives to names it wrote does not extend to display); events.ndjson link lines and their no-op pairs keep their data contract.
@task_item n=13 part=needs
Logging of the lineage: a new process log line enters the obs catalog; parent and link names kept out of the process log, user content only in diagnostics, CLAUDE values never logged.
@task_item n=14 part=needs
Any new refusal (a bad parent name, an unknown parent) is a fixed message interpolating nothing, a closed code word, an operator fork, and never blocks the human.
@task_item n=15 part=needs
Proof without a live start: measurements from tests or no-start probes with the fake agent, no real claude CLI in CI, no mutation run.
@task_standing n=1
convention: each new test runs in its own home and removes only the directories it created
home: secret-management · security-anti-patterns
@task_standing n=2
convention: each new test seam lives only under the fake-agent feature and is compiled out of release
home: input-validation · security-anti-patterns
@task_standing n=3
convention: each new recorded fixture is scrubbed or refused whole and reviewed before commit
home: data-protection
@task_standing n=4
convention: each new parser surface over viola's own formats joins the fuzz scope
home: input-validation
@task_standing n=5
convention: each new log or error line honours the never-log floor, full error detail only in diagnostics
home: error-handling
@task_item n=1 part=not_needed
The hook contract: hook stdin, hook-decision details and hook capture; the chunk adds nothing to what a hook reads or answers.
@task_item n=2 part=not_needed
Paste and dialog text: the paste and dialog free-text character sets, bracketed paste, delivery confirmation and typed-text line-ending normalisation; no typed text changes.
@task_item n=3 part=not_needed
Dialog answering and the verify command: verify stamps, verify probes, screen signatures and their residuals; no verify run is in reach.
@task_item n=4 part=not_needed
Live rigs and live rounds (HB, KR, MB, MCP, SF arms, live plugin-install rounds); the directive forbids any live start.
@task_item n=5 part=not_needed
Hyprland window recording and hyprctl reads; `open`, `focus` and windows are their own entries.
@task_item n=6 part=not_needed
MCP stdin framing and MCP tool input schemas; the chunk adds no MCP tool or input.
@task_item n=7 part=not_needed
Release, signing and deploy rows: release attestations, signed self-update, no-publish; no release stage is touched.
@task_item n=8 part=not_needed
API-error classes and transcript reads of the Claude adapter; lineage reads no transcript and no upstream error.
@task_item n=9 part=not_needed
Secret and key management absences and stored-secret rows; a parent declaration is no secret and no key.
