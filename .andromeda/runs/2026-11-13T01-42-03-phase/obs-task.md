@task_prose part=chunk
Changes how the typed run of `viola verify` (its probe paste, then the long, tag-like, newline, `/compact` and local-command pastes) decides the claude input box is up. It reads that from the input box itself, not from a footer-hint literal that a passing footer notice («Update available!», a usage warning) can push aside after a turn.
Moves the input-box screen signature in the claude agent crate and its capability-ledger row(s) to match, and adds fake-agent screens and recorded fixtures that replay a footer notice so CI covers the case.
Re-tests the diagnosis, which is not assumed. The proof is one `viola verify` on the newest installed claude CLI with every ledger row passing except the account-dependent `statusline-rate-limits` and the pastes landing while a footer notice stands. That run writes the stamps for that version, and it is a named operator step that waits on the founder's live-start cap.
Turns CI's `supply-chain` job (cargo deny advisories) green again. Three RustSec advisories hit rmcp 3.4.1 and one hits rustix 1.1.5 through tempfile. The fix is either an rmcp bump past the ratified range or a reasoned per-advisory ignore, plus a rustix lockfile bump.
@task_file n=1 path=src/cmd/verify/typed/turns.rs mark=edits
what: viola verify typed run, the turns it pastes and the readiness check before each paste — edits: function `input_box_up`, its callers `framing_turns` and `turn`'s settle
@task_file n=2 path=src/cmd/verify/typed/run.rs mark=edits
what: viola verify typed run, the run that waits for the input box before its pastes — edits: `Run`'s `box_wait`
@task_file n=3 path=crates/viola-agent-claude/src/screen.rs mark=edits
what: viola-agent-claude screen signatures, the readiness literals read off the claude screen, each one a capability-ledger row with a viola verify probe — edits: `SIGNATURES.input_box`
@task_file n=4 path=Cargo.toml mark=edits
what: the workspace dependency list, the third-party pins, among them rmcp's ratified version range — edits: `[workspace.dependencies]` rmcp entry
@task_file n=5 path=Cargo.lock mark=edits
what: the dependency lockfile, the resolved versions of rmcp and rustix (through tempfile) that cargo deny checks
@task_file n=6 path=deny.toml mark=edits
what: the cargo deny configuration, the advisory check the supply-chain CI job runs — edits: the advisories `ignore` list
@task_item n=1 part=needs
The viola verify typed run: its readiness wait before each paste, its PTY pastes into the claude child, and its captures. The readiness read stays a read of the screen only, and nothing it reads off the screen becomes telemetry.
@task_item n=2 part=needs
viola verify's own log lines (process-start, process-exit, child spawn lines for the verify, verify-pty and version probes). They carry no content, they are written only inside an instance, and pasted text, typed keys and the PTY byte stream are never logged.
@task_item n=3 part=needs
The capability ledger and its stamps. verify is the only writer of the stamps, the ledger path and stamps are never logged, and an unreadable ledger counts as an unverified CLI. The new readiness literal is a ledger row with a verify probe.
@task_item n=4 part=needs
viola verify's terminal and exit surface: the stdout step line and summary line for each ledger row, the stderr pair on a stop, exit codes 0 and 1, and the process-exit detail words (interrupted, strict-modes-failed and the verify refusals).
@task_item n=5 part=needs
The fake agent and its recorded claude fixtures under the per-CLI-version fixture tree. CI's verify runs against the fake agent only and the real CLI never runs in CI. Claude fixtures are scrub-required, and fixtures are no telemetry source.
@task_item n=6 part=needs
Wrapped claude internals (footer hints, footer notices, screen layout) are not a contract. A readiness literal drifting under a new CLI version is caught by verify and reported only to diagnostics, never assumed.
@task_item n=7 part=needs
The PTY seam's paste and spawn log events, which the typed run's pastes and the claude child's start go through. The vt100 screen feed's panics stay contained when the readiness read parses a screen with a footer notice.
@task_item n=8 part=needs
The supply chain under cargo deny. Observability dependencies must pass cargo deny, and a cargo deny failure counts as a failure. The banned telemetry and logging dependencies must not come in through an rmcp or rustix bump.
@task_item n=9 part=needs
rmcp as the stdio MCP server library, used with features client, schemars, server, transport-async-rw, transport-io and uuid (no auth, no streamable HTTP). An rmcp version change keeps rmcp's tracing target off in the diagnostics sink and keeps the MCP server on stdio.
@task_item n=10 part=needs
rustix reached through tempfile and the state crate. A lockfile bump to 1.1.6 must keep the sync crates building without Tokio and the obs dependencies building no C.
@task_item n=11 part=needs
Readiness waits in the new fake-agent tests and the footer-notice replays are not sleep-based waits on log lines, and no retry policy covers a failed readiness check.
@task_item n=12 part=needs
Live claude starts (the proof run, any probe of the footer notice) are named operator steps outside CI. Their diagnostics and captures are local run outputs that are never uploaded.
@task_standing n=1
convention: every new integration or E2E test checks the lines it makes against the diagnostic line schema and keeps its test home under the scanned integration-test root until the gates have run
home: ci-integration · 9-ci-integration
@task_standing n=2
convention: every new recorded claude fixture or fake-agent screen is scrubbed before commit and is never a telemetry source
home: 1-obs-scope-summary · obs-scope-summary
@task_standing n=3
convention: every new test waits on a condition, never on a sleep, and adds no retry; the CI gates still fail closed on its output
home: slo-invariants-telemetry-budgets · 10-slo-invariants-telemetry-budgets
@task_item n=1 part=not_needed
viola run's readiness gate for driver sends, its manual readiness-gate span and its send-refused input-not-ready detail. The run pump is out of scope unless research shows the same literal fails there, and then it is named rather than quietly widened.
@task_item n=2 part=not_needed
Hook output streams, the hook log file, hook latency budgets and the statusline shell-out with its deadline and detail codes. The chunk changes no hook or statusline arm.
@task_item n=3 part=not_needed
The budget governor (budget-gate events, budget spans, config.json five_hour and seven_day). Nothing here touches budgets.
@task_item n=4 part=not_needed
Dialog raise and answer lines, the wheel and pause/release lines, and link/unlink and their events-log records. The chunk changes no messaging path.
@task_item n=5 part=not_needed
Heartbeat liveness, list liveness and staleness thresholds, and agent-run status. Nothing here touches instance liveness.
@task_item n=6 part=not_needed
Snapshot replay, state-recovered lines and torn-line healing of the event log. The state store's behaviour is unchanged.
@task_item n=7 part=not_needed
viola revive preflight and session-live checks, and the claude agents probe parse. The chunk does not touch revive.
@task_item n=8 part=not_needed
Future error reporter rows and the absences of Sentry, OTel export, OTLP receivers, network egress and hosted deployment. The chunk adds no reporter or exporter.
