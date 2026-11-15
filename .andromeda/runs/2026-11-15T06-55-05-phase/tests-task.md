@task_prose part=chunk
Builds the first live round on the product with the budget gate and the status-line interception removed: rigs in the test-only E2E crate that drive the real claude CLI headless on a plain pty the rig opens itself, each rehearsed under the fake agent first, then named operator steps fired only on the operator's word.
Reads live the wrapped start of the unmodified CLI on Linux (matrix v2-03, heir of v1-27) on a ledger that reads cli_verified true.
Reads live a dialog hold near its 3600 s ceiling (matrix v2-02) so the PROVISIONAL mark on DIALOG_HOLD_MAX can be settled by the founder's word.
Re-records the MCP tools/list fixture through the MCP tap so the list tool's description drops the usage budget, and records a box-only screen fixture by a clean viola verify --record for the fake agent to replay in CI.
Reads a wrapped session's status line beside an unwrapped one's as live evidence for the already-verified v2-05, writing nothing to the user's settings.
@task_file n=1 path=fixtures/claude/2.1.302/Mcp.tools-list-1.json mark=edits
what: the recorded MCP tools/list exchange of the claude 2.1.302 fixture set, recorded by the test-only MCP tap and replayed by the fake agent in CI — edits: the list tool's description
@task_file n=2 path=scripts/agent-run.sh mark=edits
what: the agent-run shim over viola-harness, the rig entry point that boots, polls status, streams logs and cleans up a harness session
@task_item n=1 part=needs
Live-round rigs in the one test-only E2E crate (viola-harness, viola-mcp-tap) that start real claude on a pty they open themselves, run only locally through the local-live runner seam, never in CI; each rig is first rehearsed against the fake agent.
@task_item n=2 part=needs
The wrapped start proof (v2-03): viola run spawns the unmodified CLI with the child's screen carrying no byte of viola's own, human keys passed unchanged, the pinned copy run through a plugin rewritten on start by absolute path with its pinned exe hash checked, the run start events in their documented order with no budget step.
@task_item n=3 part=needs
Start refusals read live: a live name refused as already-live and a stale name refused as stale-heartbeat, both run start exit 1.
@task_item n=4 part=needs
The claude child's end detected on the process handle, never on pty EOF, during the live wrapped start.
@task_item n=5 part=needs
The capability ledger stamped by viola verify on a test home for the CLI the round runs, reading cli_verified true; stamps written only by verify; the unverified-CLI gate and the version gate needing every relied row decide what a different CLI version would degrade to.
@task_item n=6 part=needs
The dialog hold: DialogHold and DIALOG_HOLD_MAX (60 to 3600 s, config dialog_hold_s, bad value falls back to default, read once at start), its expiry and the rule that dialog timeouts outlast the longest hold; read live near 3600 s and changed only if the founder's word moves it.
@task_item n=7 part=needs
The hour-long hold step journals its state in the work dir before the wait and names what a resumed window re-reads, since the wait outlasts the CLI's compact.
@task_item n=8 part=needs
The MCP tools/list tool set and its list tool description as recorded through the MCP tap; the tap refuses a dirty exchange whole by its scrub refusal codes (home path, username, email, absolute path); the fake agent must still match the re-recorded fixture byte for byte.
@task_item n=9 part=needs
The box-only screen fixture: recorded by viola verify --record only when no ledger row fails, scrubbed by the recorder and checked scrub-as-detector, box rows kept as their literals alone, never hand-written, filed in the per-CLI-version fixture class and replayed by the fake agent in CI.
@task_item n=10 part=needs
The status-line reading of a wrapped session beside an unwrapped one: viola run start passes no settings flag and writes no settings file; the reading only reads the user's own status-line setting.
@task_item n=11 part=needs
Live homes are test homes on tmpfs under target/e2e-home (viola-live-*), the base's headroom checked first; nothing written under the user's viola home or claude settings; the pinned product binary untouched.
@task_item n=12 part=needs
Every spawn in a rig has a kill deadline; waits are bounded below the kills and keyed on logged events, never on sleeps; processes killed only after pid and start time are verified.
@task_item n=13 part=needs
The four founder forks the plan leaves fillable by one line: the live-start count per item and total, the per-chunk word that lets a rig type arrow and Enter into its own pty on a trust dialog, which CLI version the round runs among the 2.1.296 shim, the newer PATH claude and the 2.1.302 fixture set, and the hold's PROVISIONAL mark.
@task_item n=14 part=needs
Order: rigs and their fake rehearsals and every gate that can go green without a live start come before the first live step, where implement stops.
@task_item n=15 part=needs
Live readings are evidence, not a test layer: one live-round record per operator step in the chunk's evidence dir; a fake-agent run is not proof of real-CLI behaviour; v2-02 and v2-03 rest on a gated claim on the operator's word.
@task_item n=16 part=needs
No mutation run; a guard test carries its remove-the-guard control through the gate's control command, its case the test's name alone.
@task_item n=17 part=needs
A red or defect a live reading exposes folds into this chunk as an inferred item with its own acceptance; no other product behaviour changes.
@task_item n=18 part=needs
The live claude child's environment: CLAUDE* variables stripped and the child given its viola name, instance dir and binary, as the process-start claude-child record shows.
@task_standing n=1
convention: each new test is named by the test function naming rule and lives in the single test-only E2E crate, tokio confined to it
home: 2-test-strategy · test-strategy
@task_standing n=2
convention: each new test or rig creates its own private home under target/e2e-home, cleaned up idempotently unless kept on request
home: 5-command-implementation · test-data-bootstrap
@task_standing n=3
convention: each new fixture is recorded per CLI version from a real run, synthetic and scrubbed, never hand-written
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=4
convention: each new wait is keyed on a logged event and bounded, with no sleep and no retry
home: 11-test-anti-patterns-never-do-these
@task_standing n=5
convention: each new harness log line is a JSON tracing line with codes only, detail apart, carrying the owned fields
home: log-format
@task_item n=1 part=not_needed
Driver verbs send, answer, wait, last and release, their confirmation windows, refusals and JSON documents: the round starts and reads sessions but drives no turn through a verb.
@task_item n=2 part=not_needed
Revive: its preflight order, refusal codes, exit codes and documents; no session is revived.
@task_item n=3 part=not_needed
Wrapper channel link and unlink methods, JSON-RPC error codes and channel response log details: no channel method is added or changed.
@task_item n=4 part=not_needed
viola list human board and list JSON layout, row escaping and stale-row dim: no board output is read or changed.
@task_item n=5 part=not_needed
Performance budgets, hyperfine invocation and the perf job: no hook is timed.
@task_item n=6 part=not_needed
Fuzz targets, corpora and nightly fuzzing, and parser property tests: no parser changes.
@task_item n=7 part=not_needed
Mutation verdict contracts, thresholds, kill lines and cadence: no mutation run in this chunk.
@task_item n=8 part=not_needed
CI artifact names, retention and the secret-scan upload contracts: the chunk adds no CI job or artifact.
@task_item n=9 part=not_needed
Hook decision body snapshots per dialog kind and the dialog answer mapping: the hold is read, no dialog is answered through a decision body.
@task_item n=10 part=not_needed
The operator's desk clean-up of the live home's stale budget and status-line files, and the unpin of the 2.1.296 shim: out of scope, the operator's own.
