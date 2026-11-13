@task_prose part=chunk
Changes viola verify's typed run (Run B: the probe paste, then the long, tag-like, newline, /compact and local-command pastes) so it decides the input box is up from the input box itself, not from a footer literal that a transient footer notice («Update available!», a usage warning) displaces after a turn.
Re-tests the diagnosis (a footer literal displaced by the notice) rather than assuming it; the readiness literal stays a capability-ledger row with a viola verify probe.
Adds fake-agent screens and recorded fixtures that replay a footer notice, replayed in CI.
Proof: one live viola verify on the newest installed claude CLI reading every ledger row pass but the account-dependent statusline-rate-limits, its stamps written by verify itself; a named operator step, local only.
Turns CI's supply-chain job green again: cargo deny advisories against rmcp 3.4.1 (three RustSec entries) and rustix 1.1.5 (one), by a bump or a reasoned per-advisory ignore, rustix by a lockfile bump.
@task_file n=1 path=src/cmd/verify/typed/turns.rs mark=edits
what: viola verify's typed run turns — the input-box readiness check before each paste and the turn's settle — edits: input_box_up, framing_turns, turn's settle
@task_file n=2 path=src/cmd/verify/typed/run.rs mark=edits
what: viola verify's typed run state and its waits on the agent screen — edits: Run's box_wait
@task_file n=3 path=crates/viola-agent-claude/src/screen.rs mark=edits
what: viola-agent-claude's compiled screen signatures (input-box and modal literals) and the readiness verdict read from them — edits: SIGNATURES.input_box
@task_file n=4 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: viola-agent-claude's capability ledger, its rows and their verify probes — edits: the input-box readiness ledger row(s)
@task_file n=5 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent (viola-fake-agent) that serves the claude boundary from recorded screens — edits: its screens, a footer notice replayed after a turn
@task_file n=6 path=Cargo.toml mark=edits
what: the workspace manifest and its exactly pinned third-party dependencies — edits: [workspace.dependencies] rmcp's ratified version range
@task_file n=7 path=Cargo.lock mark=edits
what: the workspace lockfile — edits: the rmcp and rustix entries
@task_file n=8 path=deny.toml mark=edits
what: the cargo deny supply-chain gate configuration — edits: the advisories ignore list
@task_item n=1 part=needs
viola verify's typed run (Run B) and its pastes: probe, long, tag-like, newline, /compact and local-command, each pasted only once the input box is up.
@task_item n=2 part=needs
The input-box readiness check: an input-box literal on some row and no modal literal on any; how it holds when a transient footer notice replaces the resting footer hint after a turn.
@task_item n=3 part=needs
viola-agent-claude's compiled screen signatures: the input-box literal set and the modal literals; the screen is read only for compiled signatures, never parsed beyond them.
@task_item n=4 part=needs
The capability ledger: a readiness literal is a ledger row with a viola verify probe; every relied row must pass for the version gate; a failing row is no refusal; stamps refuse a widened ledger.
@task_item n=5 part=needs
Stamps (ledger/stamps.json) written only by viola verify, single writer; tests get stamps only through verify against the fake agent.
@task_item n=6 part=needs
viola verify's outputs: step line, summary line, stamped line, last line, and its process-exit details (unclean-fixture, version-unreadable, interrupted).
@task_item n=7 part=needs
Readiness waits in the typed run: the box wait that keeps waiting past the gate maximum without a literal and ends on a quiet literal or a poisoned screen; the settle of a turn; the readiness stall report.
@task_item n=8 part=needs
The fake agent: its recorded screens per CLI version, replaying recorded bytes, its input handling (bracketed and framing paste, /compact, keystroke), and the scripted turn that shows a footer notice.
@task_item n=9 part=needs
Recorded fixtures under fixtures/claude/<cli-version>/: recorded by live verify only when no row fails, scrubbed, checked by the hygiene walk; every CLI-version fixture set replayed in CI and compared byte for byte to the fake agent.
@task_item n=10 part=needs
Live claude starts are local only and never in CI: the proof run and any probe of the footer notice are named operator steps waiting on the founder's per-chunk cap; 2.1.296 is never re-verified.
@task_item n=11 part=needs
Indirect: per-test kill bounds that verify-driven tests and the verify-window class run under; the new readiness tests must finish inside them without sleeps or retries.
@task_item n=12 part=needs
Indirect: test waits key on observed screen or logged events, never on sleeps; the fake-agent footer notice must be gated by its control file or script step.
@task_item n=13 part=needs
CI's supply-chain job: cargo deny advisories as a fail-closed gate, the weekly advisories run, and the decision that no advisory is ignored against a reasoned per-advisory ignore.
@task_item n=14 part=needs
rmcp's version and features: exact third-party pins, rmcp limited to its ratified features (server and transport-io in the product, client only in the test crate); a bump past the ratified range must keep those features.
@task_item n=15 part=needs
Indirect: cargo deny bans (tokio and C crates, serde_json unbounded depth, tokio-free sync crates and viola-channel's graph) that a dependency bump of rmcp or rustix must still pass; every deny ban has a negative probe.
@task_item n=16 part=needs
rustix bumped in the lockfile through tempfile and viola-state; the minimum supported Rust and the pinned toolchain the bumped crates must build under.
@task_standing n=1
convention: each new unit test inline in its module, table-driven in rstest cases, named by the test-function naming rule, using shared constructors as rstest fixtures
home: 2-test-strategy · test-strategy
@task_standing n=2
convention: each new or re-recorded fixture is synthetic or scrubbed, passes the fixture hygiene walk, is recorded per CLI version and never hand-written
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=3
convention: each new fake-agent screen or fixture is held to the recorded verify fixtures byte for byte and replayed in CI
home: 1-test-scope-summary · test-scope-summary
@task_standing n=4
convention: each new test avoids sleeps, retries, shared mutable state, process-wide env mutation and real network, and runs in parallel
home: 11-test-anti-patterns-never-do-these
@task_standing n=5
convention: each new test creates its own private home and gets stamps only through verify against the fake agent
home: test-data-bootstrap
@task_item n=1 part=not_needed
MCP tool surfaces: tools/list, tool results, invalid-params fields, MCP reply screens and stdin line caps; the chunk changes no MCP tool.
@task_item n=2 part=not_needed
Hook dialogs and decision bodies (permission, question, plan answers, concurrent dialog, impostor server) and hook performance budgets; the chunk changes no hook.
@task_item n=3 part=not_needed
The budget governor and statusline: pause thresholds, budget.json, budget-gate records, release --budget; untouched by this chunk.
@task_item n=4 part=not_needed
viola revive: its exit codes, refusal codes, preflight order and documents; the chunk does not touch revive.
@task_item n=5 part=not_needed
viola list board and list --json: columns, rows, liveness, skipped counts; untouched.
@task_item n=6 part=not_needed
The wrapper channel's JSON-RPC errors, link and unlink methods, link records and the channel endpoint; the chunk sends no channel frame.
@task_item n=7 part=not_needed
The test harness commands and documents: session record, status document and states, boot readiness codes, cleanup document, pre-push stages, gate breaches and harness exit codes; the chunk changes no harness command.
@task_item n=8 part=not_needed
Socket-directory, strict-modes and peer-credential refusals and their exit-21 details; the chunk changes no such check.
@task_item n=9 part=not_needed
CI artifacts, artifact retention and the secret scan's classes and outputs; the chunk adds no artifact.
