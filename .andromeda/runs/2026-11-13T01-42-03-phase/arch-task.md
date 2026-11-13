@task_prose part=chunk
Changes how `viola verify`'s typed run (Run B: the probe paste, then the long, tag-like, newline, /compact and local-command pastes) decides that the input box is up. It reads the input box itself, not a footer hint literal that a passing footer notice (an update notice, a usage warning) pushes off the screen after a turn.
Changes the input-box screen signature and its capability-ledger row(s), and the fake agent's screens and fixtures so that they replay a footer notice.
Re-measures with one live `viola verify` on the newest installed claude CLI: every capability-ledger row passes except the account-dependent statusline-rate-limits row, the pastes land while a footer notice is on screen, and verify writes the stamps for that version itself. The older CLI version is not verified again.
Turns CI's supply-chain job (cargo deny advisories) green again: three RustSec advisories against rmcp 3.4.1 and one against rustix 1.1.5, fixed by a version bump or a reasoned per-advisory ignore.
@task_file n=1 path=src/cmd/verify/typed/turns.rs mark=edits
what: viola verify's typed run (Run B) and its readiness check before each paste — edits: input_box_up, framing_turns, turn's settle, Run's box_wait
@task_file n=2 path=crates/viola-agent-claude/src/screen.rs mark=edits
what: the screen signatures as compiled literals in the Claude agent crate, each readiness literal a capability-ledger row — edits: SIGNATURES.input_box and its ledger row(s)
@task_file n=3 path=fixtures/claude/ mark=edits
what: the recorded screen fixtures, one folder per CLI version, that the fake agent replays in CI — edits: the fixtures that replay a footer notice
@task_file n=4 path=Cargo.toml mark=edits
what: the workspace manifest, with third-party versions pinned in one place under workspace dependencies — edits: the rmcp entry's version range
@task_file n=5 path=Cargo.lock mark=edits
what: the workspace lockfile — edits: the rmcp and rustix versions
@task_file n=6 path=deny.toml mark=edits
what: the cargo-deny dependency policy (advisories, bans, licences, sources) — edits: the advisories ignore list
@task_file n=7 path=ledger/stamps.json
what: the ledger stamps envelope, written only by viola verify, recording the verified CLI version and its row results
@task_item n=1 part=needs
The capability ledger: a readiness literal is a ledger row that a viola verify probe proves. Any change to the input-box signature changes or adds a ledger row, and the row must still pass on the newest CLI.
@task_item n=2 part=needs
Screen signatures stay compiled literals read through the vt100 screen model. Only verify reads screen row text, and the new input-box test has to be a screen read that fits this rule.
@task_item n=3 part=needs
The Claude agent crate stays pure (no file I/O), and Claude-specific screen shapes stay in that one crate. The new signature lives there.
@task_item n=4 part=needs
Verify waits for an input-box literal after added turns (the box wait). This chunk changes what counts as the box being up for that wait and for the check before each paste.
@task_item n=5 part=needs
Verify never types into, and never answers, a CLI dialog. The new readiness test must still see a modal as not ready.
@task_item n=6 part=needs
A CLI version counts as verified only when every relied row passes. The proof run reads all rows except the account-dependent statusline-rate-limits row, and that exception has to be squared with this rule.
@task_item n=7 part=needs
Verify is the only stamps writer. The writer checks strict modes before and under its lock, merges only its own version's entry, and malformed or untrusted stamps read as unverified.
@task_item n=8 part=needs
The ledger stamps envelope: verified version, rows, measured and content, as the proof run writes them for the newest CLI.
@task_item n=9 part=needs
Verify's run structure: its bounded set of child processes, Run E resuming Run B's first session, probe folders removed by a drop guard and only its own, and plan files kept in its own run folder. The changed waits must not break these.
@task_item n=10 part=needs
viola verify output and exit codes: the step line, the summary line, exit 1 when a relied row fails, and its process-exit refusal codes (including unclean-fixture).
@task_item n=11 part=needs
Fixtures are recorded only from a clean verify, scrubbed and hygiene-walked, and keep only signature rows. Each new footer-notice fixture follows this.
@task_item n=12 part=needs
The fake agent (viola-fake-agent) and its test-seam variables: its recorded screens must replay a footer notice standing over a turn, and CI replays them.
@task_item n=13 part=needs
CI runs on ubuntu against the fake agent, with no live claude in CI. Real-CLI probes run only locally, and here every live start is a named operator step.
@task_item n=14 part=needs
cargo-deny dependency policy: the advisories check behind CI's supply-chain job and the weekly advisory audit. Any ignore is per advisory and gives its reason.
@task_item n=15 part=needs
Third-party versions are pinned in one place, in workspace dependencies. A bump of rmcp past its ratified range (>=3.4.1, <3.5) is weighed against the pinning decision.
@task_item n=16 part=needs
rmcp's features stay exactly server and transport-io for release (no auth, no streamable-HTTP feature). A bump or an ignore relies on this feature set.
@task_item n=17 part=needs
rmcp is the stdio MCP server, and the MCP crate takes the channel with tokio. An rmcp bump must keep the MCP server building and serving.
@task_item n=18 part=needs
Supply-chain bans that a version bump could trip: crates.io sources only, no C-building crates, banned dependency features, no telemetry crates, licence allowlist, every ban proven live, policy for one target triple.
@task_item n=19 part=needs
tempfile as the snapshot writer, pinned exactly, and its rustix dependency. The rustix lockfile bump reaches viola-state through it.
@task_item n=20 part=needs
The MSRV floor and the pinned Rust toolchain: a newer rmcp or rustix must still build on them.
@task_standing n=1
convention: each new test runs against the fake agent, never a live claude
home: ci-cd-approach
@task_standing n=2
convention: each new test runs in its own isolated viola home
home: established-decisions
@task_standing n=3
convention: each new or re-recorded screen fixture is scrubbed, hygiene-walked and keeps only signature rows
home: occupied-resources
@task_standing n=4
convention: each new test respects the coverage floors and the gate steps that fail closed
home: ci-cd-approach
@task_standing n=5
convention: each new end-to-end test leaves zero panic lines, and insta snapshots run in check mode only
home: project-directory-structure
@task_standing n=6
convention: each new log line goes through the one event emitter, with no print macros and no tracing level macros
home: build-system
@task_item n=1 part=not_needed
MCP tools, MCP tool results and MCP list results: the chunk changes no MCP tool's behaviour, only the rmcp version or advisory policy.
@task_item n=2 part=not_needed
Wrapper channel methods, frames, fault codes and the channel response contracts: the chunk sends no new frame and changes no channel method.
@task_item n=3 part=not_needed
Hook contracts, hook tiers, the hook-to-kind map and dialog-hold rules: no hook's behaviour changes.
@task_item n=4 part=not_needed
Wheel, human-take, link and unlink rules: verify's typed run holds no wheel and records no link.
@task_item n=5 part=not_needed
Budget governor, budget.json and the budget gate: no budget reading or gate changes.
@task_item n=6 part=not_needed
Revive, its refusals and its --json codes: revive is untouched.
@task_item n=7 part=not_needed
Board and list payloads, agents listing and session liveness: no list or liveness reading changes.
@task_item n=8 part=not_needed
Normalised event kinds, event data per kind and wrapper event appends: verify appends no new event kind.
@task_item n=9 part=not_needed
Plugin install, the plugin's marketplace tree and the statusline wrapper and its source: none is touched.
@task_item n=10 part=not_needed
Instance snapshot fields, the snapshot envelope and snapshot replay: verify's change writes no instance snapshot.
@task_item n=11 part=not_needed
Release tooling, dist, the release build and deployment: nothing is released or deployed.
@task_item n=12 part=not_needed
Product absences (graphical frontend, mobile view, daemon, database, broker, Windows or macOS, containers and hosting): nothing here adds such a thing.
