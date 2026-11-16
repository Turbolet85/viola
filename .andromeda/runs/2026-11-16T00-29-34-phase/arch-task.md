@task_prose part=chunk
Documents and pins by tests the wake condition of `wait` (CLI `viola wait`, `wait --json`, MCP `wait`): which logged event kinds return a parked wait and which are log-only and park it again, stated in the wait contract and proved over the fake agent.
A turn end while a background task still holds the session wakes `wait` (the `turn-ended` line carrying `background_tasks`, or the `question` it raises); a notification-class line after a task-free turn end gives no false wake to a wait armed after that end.
Re-measures at P3, from the code and the fake agent, the driver-side instance where a turn end reading `background_tasks` 0 while the session still waits on a notification returns the wait, and re-reads today's wake kinds versus log-only kinds before the plan states either.
Takes the carried CI red as its own item: the stamped-home test fixture's unbounded `viola verify` pushes a start-order test past the nextest `ci` profile's 20 s kill under load; P3 chooses by measurement between stamping a home once and sharing or copying it, or sizing the class's `ci` kill to the loaded time.
@task_file n=1 path=src/run/wait.rs mark=edits
what: the wrapper's wait feed — the channel wait method that parks until a wake-kind event lands after its start offset — edits: its wake-kind set and its unit tests, the wake-kind and log-only-kind cases
@task_file n=2 path=src/cmd/wait.rs mark=edits
what: the `wait` CLI verb and its `--json` output, a frame-writing client of the channel wait method
@task_file n=3 path=src/human/wait.rs mark=edits
what: the human text rendering of the `wait` CLI verb's result
@task_file n=4 path=crates/viola-mcp/src/tools.rs mark=edits
what: the MCP tools of the stdio MCP server — edits: the MCP `wait` tool
@task_file n=5 path=crates/viola-core/src/lib.rs mark=edits
what: the normalised event kinds and their event data, the agent-neutral event catalog in viola-core
@task_file n=6 path=crates/viola-agent-claude/src/hook.rs
what: the hook to kind map — Claude hook payloads (Stop, StopFailure, Notification) normalised into event kinds, with `turn-ended` `background_tasks`
@task_file n=7 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent binary and its turn scripts, the CI stand-in for claude — edits: turn scripts ending a turn with and without a background task and emitting a later notification
@task_file n=8 path=fixtures/fake-scripts/ mark=edits
what: the fake agent's scripted turn fixtures
@task_file n=9 path=tests/support/verify.rs mark=edits
what: the test support that stamps a test home by running `viola verify` (the `stamped_home` fixture, `within: None`) — edits: the `stamped_home` fixture
@task_file n=10 path=.config/nextest.toml mark=edits
what: the nextest profiles, the `ci` profile's per-class slow-timeout kill — edits: the `ci` profile's kill for the stamped-home test class
@task_item n=1 part=needs
The channel wait method's wake kinds: the closed set of event kinds that return a parked wait versus the log-only kinds that park it again, documented in the wait contract and pinned by a test per kind.
@task_item n=2 part=needs
A `turn-ended` event whose `background_tasks` is above 0, or a `question` raised at that turn end, wakes a parked wait; the wait never parks past it.
@task_item n=3 part=needs
No false wake: after a turn ended with no background task, a later notification-class line (the Notification hook mapped through `hook.event` to a log-only line) does not return a fresh wait armed after that end.
@task_item n=4 part=needs
The wait's start offset and `after` cursor: a wait armed after a turn end reads only events past its offset, by events.ndjson byte offsets, so an earlier turn end cannot wake it.
@task_item n=5 part=needs
The driver-side instance (a turn end reading `background_tasks` 0 while the session still waits on a notification returns the wait and the driver must re-arm) is stated by the plan as documented behaviour, a defect fixed, or the driver's to handle, only after P3 re-measures it.
@task_item n=6 part=needs
The same wake condition holds on all three surfaces: CLI `viola wait`, `wait --json`'s result document, and the MCP `wait` tool under its 50 s cap; the wait result shape and timeout_ms bound stay as the contract states.
@task_item n=7 part=needs
`turn-ended` event data — `background_tasks`, `cause`, `last_assistant_message` — and the `question` event data, as the wait wake reads them.
@task_item n=8 part=needs
Any new or changed event kind is added only in viola-core, agent-neutral, with kebab-case wire values and versioned-tolerant reading; wrapper-appended events stay log-only.
@task_item n=9 part=needs
wait blocks on the channel and nothing polls: a wait with nothing to wake it parks until its timeout, never a poll loop.
@task_item n=10 part=needs
The wake scenarios are proved over the fake agent through its turn scripts and its test seam variables; no live claude in CI, and any live claude start is asked first with its exact count and runs under a private CLAUDE_CONFIG_DIR.
@task_item n=11 part=needs
The stamped-home fixture's `viola verify` runs unbounded (`within: None`) and its time sits inside the measured test; the fix either takes stamping out of the measured time (a home stamped once and shared or copied, naming the isolation it costs) or sizes the `ci` profile kill to the loaded time, with a bound shown holding at load.
@task_item n=12 part=needs
A shared or copied stamped home must keep the test-home rules: each test in its own viola home, test homes cleaned by their owner, verify as the one stamps writer, and the strict-modes read rule over a copied tree.
@task_item n=13 part=needs
Root test waits share one bound, and the nextest `ci` profile's kill is the CI timeout the stamped-home test class runs under.
@task_item n=14 part=needs
Each new guard test carries its one-off remove-the-guard control, its case named by the test's name alone; no mutation run in this chunk; coverage floors still gate the change.
@task_standing n=1
convention: each new test runs in its own isolated viola home, cleaned by its owner, with evidence surviving a killed test
home: occupied-resources
@task_standing n=2
convention: each new test runs against the fake agent, never a live claude in CI
home: ci-cd-approach
@task_standing n=3
convention: each new fake-agent test seam variable carries the fake-agent prefix and is no configuration channel
home: conventions
@task_standing n=4
convention: each new OS-gated test names its reason and no test gate names another OS
home: cross-cutting-patterns
@task_standing n=5
convention: each new end-to-end run holds zero panic lines and insta snapshots stay in check mode
home: project-directory-structure
@task_item n=1 part=not_needed
Hyprland window features and the compositor seam: the chunk opens no window.
@task_item n=2 part=not_needed
Send confirmation, the readiness gate's turn race and bracketed-paste send rules: the scope names them not this chunk's.
@task_item n=3 part=not_needed
Local-command delivery and its post-conditions, and the StopFailure live reading with its capability-ledger exception: not this chunk's.
@task_item n=4 part=not_needed
Dialog answering — permission, plan and question answer responses, dialog hold and its bounds: the chunk reads a `question` only as a wake, never answers it.
@task_item n=5 part=not_needed
The wheel, links, pause and release: the wait wake changes no wheel or link state.
@task_item n=6 part=not_needed
The board and list: the agents listing, list payload items and liveness rows.
@task_item n=7 part=not_needed
Revive: its program resolution, cwd and refusal codes.
@task_item n=8 part=not_needed
Plugin install, the marketplace tree and the plugin's .mcp.json.
@task_item n=9 part=not_needed
Release tooling, dist, deployment and publishing rows.
@task_item n=10 part=not_needed
The fuzz workspace, its nightly toolchain and weekly fuzzing.
@task_item n=11 part=not_needed
config.json keys and their fallbacks: the chunk adds no configuration.
@task_item n=12 part=not_needed
Status-line interception and the usage-budget gate absences.
