@task_prose part=session
The session added a Hyprland seam: a new sync library crate, viola-hyprland, that spawns the compositor client hyprctl directly (no shell, no PATH lookup, stdin null) for four JSON queries (clients, activewindow, workspaces, version) and two focus dispatches, then parses each reply through a closed, tolerant parser that drops every text field (titles, classes, names).
Each read is bounded at MAX_FRAME and refused whole as oversize. A child past a provisional 5 s deadline is killed, reaped and reads as unreachable. Every failure is one of five closed code words with a fixed message: unsupported, unreachable, oversize, malformed, refused.
Observability: the hyprctl spawn writes process-start and process-exit with the new subject hyprctl-probe, and a reply it cannot parse writes parse-rejected with the new parser hyprctl-json. The read runs in the span hyprland.read (skip_all), and viola_hyprland joins the viola log targets (7 to 8). No product verb calls the seam yet.
Measured live on Hyprland 0.56.2: dispatch runs as Lua, so the dispatch argv is the Lua focus form, and the only value inserted into it is a closed address or an i32 workspace id.
Test side: a test-only recorder binary, viola-hypr-tap, scrubs each recorded reply to a closed per-request keep-list. The fake agent answers as hyprctl when started under that file name, and a new E2E contract replays the recorded set. A new fuzz target with 9 synthetic seeds was added (fuzz targets 6 to 7), and the new crate is the fifth sync crate under the per-crate tokio ban. Nothing was retired.
@task_file n=1 path=crates/viola-hyprland/Cargo.toml
what: the manifest of the new Hyprland seam crate, a sync library crate with no tokio
@task_file n=2 path=crates/viola-hyprland/src/lib.rs
what: the Hyprland seam's closed request set and argv, the address and workspace id newtypes, and the closed error enum with fixed messages
@task_file n=3 path=crates/viola-hyprland/src/reply.rs
what: the hyprctl reply parsers, bounded at MAX_FRAME and tolerant, which drop every text field, with property tests
@task_file n=4 path=crates/viola-hyprland/src/read.rs
what: the hyprctl spawn and read: a direct spawn, the deadline kill and reap, the process-start and process-exit pair and the parse-rejected line, and the span hyprland.read
@task_file n=5 path=crates/viola-hyprland/proptest-regressions/reply.txt
what: the committed property-test regressions of the hyprctl reply parsers
@task_file n=6 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs
what: the test-only recorder of live hyprctl replies, which scrubs each reply to a closed keep-list
@task_file n=7 path=src/bin/viola-fake-agent/hyprctl.rs
what: the fake agent's hyprctl personality, which replays recorded compositor replies in CI
@task_file n=8 path=schemas/hyprland-fixture.v1.json
what: the schema of a recorded Hyprland reply fixture
@task_file n=9 path=fixtures/hyprland/0.56.2/ActiveWindow.recorded-1.json
what: a recorded and scrubbed hyprctl reply fixture for the active window
@task_file n=10 path=fixtures/hyprland/0.56.2/Clients.recorded-1.json
what: a recorded and scrubbed hyprctl reply fixture for clients
@task_file n=11 path=fixtures/hyprland/0.56.2/FocusWindow.recorded-1.json
what: a recorded hyprctl dispatch reply fixture for focusing a window
@task_file n=12 path=fixtures/hyprland/0.56.2/Version.recorded-1.json
what: a recorded and scrubbed hyprctl reply fixture for the version
@task_file n=13 path=fixtures/hyprland/0.56.2/Version.absent-1.json
what: a recorded hyprctl reply fixture for the no-instance case
@task_file n=14 path=fixtures/hyprland/0.56.2/Workspace.recorded-1.json
what: a recorded hyprctl dispatch reply fixture for focusing a workspace
@task_file n=15 path=fixtures/hyprland/0.56.2/Workspaces.recorded-1.json
what: a recorded and scrubbed hyprctl reply fixture for workspaces
@task_file n=16 path=tests/contract_hyprland_seam.rs
what: the E2E contract for the Hyprland seam: fake replay, parse, oversize and malformed lines that carry codes only, and a canary that must reach no line
@task_file n=17 path=fuzz/fuzz_targets/hyprctl_json.rs
what: the fuzz target over the hyprctl reply parsers
@task_file n=18 path=fuzz/corpus/hyprctl_json
what: the synthetic seed corpus of the hyprctl reply fuzz target
@task_file n=19 path=Cargo.toml
what: the workspace manifest: its members and the root package's dev-dependencies
@task_file n=20 path=Cargo.lock
what: the workspace lockfile
@task_file n=21 path=crates/viola-e2e/Cargo.toml
what: the manifest of the e2e test-binaries crate
@task_file n=22 path=scripts/sync-crates.txt
what: the list of sync crates that the per-crate tokio ban and cargo check cover
@task_file n=23 path=src/obs.rs
what: the obs module and its viola log target list
@task_file n=24 path=schemas/diag-line.v1.json
what: the diag-line v1 schema, with its subject enum and parse-rejected parser enum
@task_file n=25 path=src/bin/viola-fake-agent.rs
what: the fake agent test binary used on the test path
@task_file n=26 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the fake agent's unit tests for its modes
@task_file n=27 path=tests/support/fake.rs
what: the test support helper that places the fake agent on the test path
@task_file n=28 path=tests/contract_bounded_readers.rs
what: the bounded-readers census of product roots and their BOUNDARIES rows
@task_file n=29 path=tests/contract_error_bodies.rs
what: the error-bodies census of product roots and of the types that are not error bodies
@task_file n=30 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene walk over recorded fixtures, now with a Hyprland class check against the keep-list
@task_file n=31 path=fuzz/Cargo.toml
what: the fuzz crate's manifest and its target list
@task_file n=32 path=fuzz/Cargo.lock
what: the fuzz crate's lockfile
@task_file n=33 path=.gitattributes
what: the repository's git attributes for the binary fuzz corpus
@task_item n=1 part=touches
contract/diag-line-fields/process-start: its subject list grows from 7 to 8 words with hyprctl-probe.
@task_item n=2 part=touches
contract/diag-line-fields/parse-rejected: its parser list grows from 10 to 11 words with hyprctl-json.
@task_item n=3 part=touches
decision/viola-crates-log-at-info and decision/unlisted-targets-log-off: the viola log target set grows from 7 to 8 with viola_hyprland.
@task_item n=4 part=touches
contract/child-spawn-log/subject: the child spawn events gain the subject hyprctl-probe for the compositor client spawn.
@task_item n=5 part=touches
contract/diag-subject/agents-probe: the sibling that the new subject hyprctl-probe mirrors, which is another probe child spawned at its call site.
@task_item n=6 part=touches
contract/parse-rejected/claude-agents-json: the sibling that the new parser hyprctl-json mirrors, which also refuses untrusted upstream JSON.
@task_item n=7 part=touches
decision/os-apis-at-call-sites and decision/instrument-call-sites-not-os-internals: the hyprctl spawn is instrumented at viola's own call site in the new crate.
@task_item n=8 part=touches
decision/agents-cli-output-untrusted and decision/agents-probe-output-untrusted: hyprctl output is now a second untrusted upstream reply that viola parses.
@task_item n=9 part=touches
decision/per-parser-parse-failure-counts, decision/parse-failures-surface-as-unknown and decision/agents-cli-parse-failure-unknown: a new parser surfaces failures as the malformed or oversize code.
@task_item n=10 part=touches
contract/parse-rejected-detail/oversize and contract/parse-rejected-detail/malformed: both detail words are now also emitted by the hyprctl-json parser.
@task_item n=11 part=touches
decision/span-naming, decision/static-span-names and decision/instrument-requires-skip-all: the new span hyprland.read is declared skip_all.
@task_item n=12 part=touches
decision/sync-instrumentation-no-tokio and decision/sync-crates-without-tokio: a fifth sync crate is instrumented without tokio, and the per-crate tokio ban covers it.
@task_item n=13 part=touches
decision/no-telemetry-from-screen and decision/no-telemetry-from-screen-content: window titles, classes and names are dropped at the parse and reach no line or error.
@task_item n=14 part=touches
decision/default-deny-log-fields and decision/source-allow-list-logging: the compositor reply contributes no text field to any line.
@task_item n=15 part=touches
decision/typed-errors-fixed-display, decision/error-messages-interpolate-nothing and decision/error-bodies-clean: a new closed error enum has five fixed messages, and it is listed as not an error body until a verb calls the seam.
@task_item n=16 part=touches
decision/boundary-errors-end-in-logged-line: when no program is resolved, the seam's read returns unsupported without a spawn and writes no line.
@task_item n=17 part=touches
decision/fixtures-not-telemetry and decision/fixtures-not-a-telemetry-source: a new class of recorded fixtures, the Hyprland replies, is used for replay only.
@task_item n=18 part=touches
decision/claude-fixtures-scrub-required: the new Hyprland fixture class is scrub-required too, by a closed per-request keep-list.
@task_item n=19 part=touches
decision/fuzz-artifacts-synthetic-seeds-only and decision/fuzz-artifacts-need-synthetic-corpus: a new fuzz target has 9 synthetic seeds, and fuzz targets go from 6 to 7.
@task_item n=20 part=touches
decision/ci-verify-fake-agent-only and decision/real-cli-verify-never-in-ci: the real hyprctl was measured only on the dev host, and CI replays it through the fake agent's hyprctl personality.
@task_item n=21 part=touches
decision/diag-schema-additive-only, decision/log-schema-additive-only and decision/unemittable-schema-words-removed: two words were added to the schema's enums, each with its emitter.
@task_item n=22 part=touches
decision/statusline-deadline and decision/statusline-command-deadline: these sibling deadlines on a viola-spawned child now sit beside a provisional 5 s hyprctl deadline that kills and reaps.
@task_item n=23 part=touches
decision/child-exit-from-handle-wait and decision/child-exit-from-process-handle: the hyprctl child's exit is read from its process handle, polled until the deadline.
@task_item n=24 part=touches
contract/mcp-stdin/max-frame and contract/mcp-stdin/line-within-max-frame: the shared MAX_FRAME cap now also bounds the hyprctl reply, which is refused at one byte past the cap.
@task_item n=25 part=touches
decision/uninstrumented-components: the test-only recorder viola-hypr-tap and the fake hyprctl are components left uninstrumented.
@task_fact n=1 slug=log-coverage
fact: contract/diag-subject/hyprctl-probe: the subject of the process-start and process-exit pair that the Hyprland seam writes around each hyprctl spawn.
@task_fact n=2 slug=log-coverage
fact: The parse-rejected line with parser hyprctl-json is written by the Hyprland seam when a hyprctl reply is oversize or malformed. The line carries codes only.
@task_fact n=3 slug=span-trace-coverage
fact: The Hyprland seam's read is one manual span, hyprland.read, declared skip_all.
@task_fact n=4 slug=pii-scrubbing-compliance
fact: No text from a compositor reply is kept: window titles, classes, names and every other reply string are dropped at the parse, and a canary reaches no line and no error.
@task_fact n=5 slug=1-obs-scope-summary
fact: Hyprland reply fixtures are scrub-required by a closed per-request keep-list: only addresses and the version tag in digit form are kept, every other string becomes a placeholder, and the hygiene walk refuses any fixture that breaks the list.
@task_fact n=6 slug=7-error-capture-reporting
fact: The Hyprland seam's errors are five closed code words with fixed messages: unsupported, unreachable, oversize, malformed and refused. An unresolved hyprctl reads unsupported and writes no line.
@task_fact n=7 slug=log-coverage
fact: A hyprctl child past its deadline (5 s, PROVISIONAL until its first product caller) is killed and reaped and reads unreachable.
@task_item n=1 part=untouched
Channel, JSON-RPC and MCP rows, and the send, wait, last, dialog, hook, statusline and wheel paths: no verb calls the seam yet.
@task_item n=2 part=untouched
Panic capture, catch sites, panic lines and the zero-panics gate: no panic path changed.
@task_item n=3 part=untouched
The events.ndjson product stream, state recovery, the heartbeat and liveness: the seam writes no product event.
@task_item n=4 part=untouched
config.json keys, log levels and env-var rules: no config key or env var is read by viola.
@task_item n=5 part=untouched
Service identity and the process-start os, pid and version fields: unchanged.
@task_item n=6 part=untouched
OTel, exporter, reporter, metrics and network-egress absences and decisions: nothing was added there.
@task_item n=7 part=untouched
CI artifact uploads, the secret scan, artifact retention and the hyperfine performance budgets: unchanged.
@task_item n=8 part=untouched
The detail-file line and its fields, and the instance detail file: the seam writes nothing to a detail file.
