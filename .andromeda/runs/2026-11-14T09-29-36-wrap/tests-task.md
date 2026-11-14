@task_prose part=session
Added the Hyprland seam: a new sync library crate viola-hyprland that reads relied-on hyprctl queries (clients, activewindow, workspaces, version) and issues two focus dispatches through a direct spawn with a closed argv, no shell, a MAX_FRAME+1 bounded stdout and a 5 s provisional deadline that kills and reaps the child; no product caller yet.
Measured live on the dev host (Hyprland 0.56.2, levels L1 and L2): hyprctl dispatch runs its arguments as Lua, so the planned focuswindow/workspace argv was refused (exit 7) and replaced by the measured hl.dsp.focus form.
Added a test-only recorder viola-hypr-tap in the E2E crate, a recorded fixture set with its own schema and a closed per-request string keep-list scrub, a hyprctl personality of the fake agent chosen by its executable file name, a contract binary contract_hyprland_seam, a Hyprland class in the fixture hygiene walk, five 512-case proptests with committed regressions and a hyprctl_json fuzz target.
Counts moved: sync crates 4 to 5, log targets 7 to 8, diag-line subject words 7 to 8 (hyprctl-probe), parse-rejected parser words 10 to 11 (hyprctl-json), fuzz targets 6 to 7, test-only recorders 1 to 2 beside viola verify, census PRODUCT_ROOTS 7 to 8; HyprlandError stands on the error-body census exempt list.
Nothing retired; the real hyprctl is never run in CI, the fake replays the recorded set there.
@task_file n=1 path=crates/viola-hyprland/Cargo.toml
what: the manifest of the new sync library crate viola-hyprland: a workspace member with a code-free fake-agent feature, no tokio in its normal graph
@task_file n=2 path=crates/viola-hyprland/src/lib.rs
what: the Hyprland seam's closed request set, its closed argv per request, the Address and WorkspaceId newtypes and the HyprlandError closed code words with fixed messages
@task_file n=3 path=crates/viola-hyprland/src/reply.rs
what: the hyprctl reply parsers (clients, active window, workspaces, version, dispatch), tolerant and closed, dropping every text field, refusing past the frame cap as oversize; their unit cases and proptests
@task_file n=4 path=crates/viola-hyprland/src/read.rs
what: the bounded hyprctl reader: direct spawn with no shell, stdin null, named env vars removed, stdout bounded to MAX_FRAME+1, a deadline that kills and reaps the child, the process-start/process-exit and parse-rejected lines
@task_file n=5 path=crates/viola-hyprland/proptest-regressions/reply.txt
what: committed proptest regression seeds of the Hyprland reply parsers
@task_file n=6 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs
what: the test-only recorder of Hyprland replies, a binary of the test-only E2E crate: records the live set, scrubs it to a closed per-request keep-list plus the home-path and username scrub, refuses a dirty set whole
@task_file n=7 path=crates/viola-e2e/Cargo.toml
what: the manifest of the test-only E2E crate, now declaring the viola-hypr-tap binary and depending on viola-hyprland
@task_file n=8 path=src/bin/viola-fake-agent/hyprctl.rs
what: the fake agent's hyprctl personality: answers as hyprctl when its executable file name is hyprctl, replaying recorded fixtures byte for byte, with recorded, oversize, malformed and absent modes from files beside its executable
@task_file n=9 path=src/bin/viola-fake-agent.rs
what: the fake agent's entry point, now dispatching to the hyprctl personality by its own file name
@task_file n=10 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the fake agent's unit cases of its modes, now with the hyprctl replay and mode bytes case
@task_file n=11 path=tests/support/fake.rs
what: the shared test helper that places the fake agent on a test path, now with hyprctl_bin placing it as hyprctl with a fixture set and mode
@task_file n=12 path=tests/contract_hyprland_seam.rs
what: the E2E-layer contract binary of the Hyprland seam: the fake replays every recorded reply byte for byte, the read parses each, the no-instance reply reads unreachable, oversize and malformed modes, the canary reaches no line and no error
@task_file n=13 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene run-time walk and its planted violations, now with a Hyprland fixture class (schema, keep-list, planted title and unlisted strings refused by class)
@task_file n=14 path=tests/contract_bounded_readers.rs
what: the bounded-readers census and its BOUNDARIES rows, now holding the hyprctl reader and the new crate among PRODUCT_ROOTS
@task_file n=15 path=tests/contract_error_bodies.rs
what: the error-body census of product error writers, now listing HyprlandError among NOT_ERROR_BODIES and the new crate among PRODUCT_ROOTS
@task_file n=16 path=schemas/hyprland-fixture.v1.json
what: the JSON schema of a recorded Hyprland fixture envelope (v, hyprland version, request word, args, exit, stdout)
@task_file n=17 path=schemas/diag-line.v1.json
what: the diagnostics line schema, its subject enum now with hyprctl-probe and the parse-rejected parser enum now with hyprctl-json
@task_file n=18 path=src/obs.rs
what: the process log setup and the list of viola log targets at info, now with viola_hyprland
@task_file n=19 path=scripts/sync-crates.txt
what: the list of sync crates each checked alone and held to the per-crate tokio ban, now five
@task_file n=20 path=Cargo.toml
what: the workspace manifest: members, workspace dependencies, the root package's dev-dependencies and test targets, now with viola-hyprland and the contract_hyprland_seam test requiring fake-agent
@task_file n=21 path=Cargo.lock
what: the workspace lockfile
@task_file n=22 path=fuzz/Cargo.toml
what: the separate fuzz workspace manifest and its target list, now with hyprctl_json
@task_file n=23 path=fuzz/Cargo.lock
what: the fuzz workspace lockfile, refreshed for a stale veil edge and the new crate
@task_file n=24 path=fuzz/fuzz_targets/hyprctl_json.rs
what: the fuzz target over the Hyprland reply parsers
@task_file n=25 path=fuzz/corpus/hyprctl_json/
what: the committed fuzz corpus of the hyprctl_json target, nine synthetic seeds, replayed by the fuzz corpus replay
@task_file n=26 path=fixtures/hyprland/0.56.2/ActiveWindow.recorded-1.json
what: a recorded Hyprland 0.56.2 fixture, the activewindow query reply, scrubbed
@task_file n=27 path=fixtures/hyprland/0.56.2/Clients.recorded-1.json
what: a recorded Hyprland 0.56.2 fixture, the clients query reply, scrubbed
@task_file n=28 path=fixtures/hyprland/0.56.2/FocusWindow.recorded-1.json
what: a recorded Hyprland 0.56.2 fixture, the focus-window dispatch reply
@task_file n=29 path=fixtures/hyprland/0.56.2/Version.recorded-1.json
what: a recorded Hyprland 0.56.2 fixture, the version query reply, scrubbed
@task_file n=30 path=fixtures/hyprland/0.56.2/Version.absent-1.json
what: a recorded Hyprland 0.56.2 fixture, the version query with no compositor instance reachable
@task_file n=31 path=fixtures/hyprland/0.56.2/Workspace.recorded-1.json
what: a recorded Hyprland 0.56.2 fixture, the workspace focus dispatch reply
@task_file n=32 path=fixtures/hyprland/0.56.2/Workspaces.recorded-1.json
what: a recorded Hyprland 0.56.2 fixture, the workspaces query reply, scrubbed
@task_file n=33 path=.gitattributes
what: the repository's byte-exactness attributes for committed fixtures and corpora
@task_item n=1 part=touches
decision/fixtures-recorded-by-live-verify names the test-only recorders; a second one, viola-hypr-tap, now records Hyprland replies live on the dev host beside viola verify and viola-mcp-tap.
@task_item n=2 part=touches
decision/mcp-exchange-recorded-by-tap, decision/mcp-tap-refuses-dirty-exchange, decision/recorder-scrub and decision/dirty-payload-refuses-recording rule over recorders and their scrub; the new recorder scrubs to a closed per-request keep-list, then the home-path and username scrub, and refuses a dirty set whole by closed code.
@task_item n=3 part=touches
decision/fixture-hygiene-walk, decision/fixtures-walked-for-hygiene, decision/planted-hygiene-violations and decision/hygiene-reports-class-only gain a Hyprland fixture class: schema, keep-list check, planted title, lastwindowtitle, unlisted-string and non-success text replies refused by class; an empty walk fails.
@task_item n=4 part=touches
decision/fake-agent-on-test-path and decision/external-processes-real-stubs: the fake agent is now also placed on a test path as hyprctl, a real stub executable for the compositor's client.
@task_item n=5 part=touches
decision/fake-agent-argv-only and contract/fake-agent-cli/argv-only-options: the hyprctl personality is chosen by the executable's file name and configured by files beside it (hyprctl-set, hyprctl-mode), no option and no env var; an exception to configuration by argv alone.
@task_item n=6 part=touches
decision/fake-agent-matches-recordings, decision/fake-agent-replays-recorded-bytes, decision/fake-agent-and-verify-fixtures-are-contract and decision/fixture-drift-byte-compare: the fake replays every recorded Hyprland reply byte for byte, held by the contract binary.
@task_item n=7 part=touches
decision/fixtures-schema-checked and decision/fixtures-match-tolerant-schema: a new fixture schema, hyprland-fixture v1, checks the recorded Hyprland set.
@task_item n=8 part=touches
decision/fixtures-per-cli-version and decision/replay-every-cli-version-fixture: Hyprland fixtures are kept per compositor version (0.56.2) and every recorded file is replayed.
@task_item n=9 part=touches
decision/property-tests-on-parsers, decision/parser-property-fuzz-coverage, decision/property-fuzz-for-parsers, decision/fuzz-the-parser-surfaces and decision/proptest-parser-invariants: a new parser surface, the hyprctl reply parsers, under five 512-case proptests and a fuzz target.
@task_item n=10 part=touches
decision/proptest-persisted-seeds, decision/proptest-seeds-committed and decision/proptest-committed-seeds: a committed regression file for the Hyprland reply proptests.
@task_item n=11 part=touches
decision/fuzz-corpus-replay-in-ci, decision/fuzz-corpus-replay-on-push, decision/fuzz-corpus-never-vacuous, decision/fuzz-reproducers-committed, decision/fuzz-corpora-byte-exact and decision/fuzz-separate-workspace: the fuzz workspace gains a seventh target, hyprctl_json, with a nine-seed synthetic corpus in the replay.
@task_item n=12 part=touches
decision/bounded-readers and decision/bounded-readers-keep-the-cap: a new external reader, the hyprctl stdout, bounded at MAX_FRAME+1, over-cap refused whole as oversize before the exit is read, with its BOUNDARIES row and witness.
@task_item n=13 part=touches
decision/error-bodies-held-by-surface-witnesses, decision/error-writers-each-named and decision/error-bodies-hold-no-path: HyprlandError joins the census as not an error body since no verb calls the seam yet; its messages interpolate nothing.
@task_item n=14 part=touches
decision/thiserror-messages-interpolate-nothing: a new thiserror enum, HyprlandError, five closed code words, each message fixed and pinned by literal.
@task_item n=15 part=touches
decision/sync-crates-without-tokio, decision/tokio-banned-per-sync-crate, decision/sync-crates-compile-without-features and decision/cargo-deny-tokio-and-c-crate-bans: the sync crate list grows from four to five with viola-hyprland, its tokio ban green.
@task_item n=16 part=touches
decision/release-build-viola-only and decision/no-test-binary-in-release: a new test-only binary, viola-hypr-tap, kept out of the release build.
@task_item n=17 part=touches
decision/single-test-only-crate and decision/test-only-e2e-crate: the test-only E2E crate gains the recorder binary and a dependency on viola-hyprland.
@task_item n=18 part=touches
decision/integration-binary-surface-prefix and decision/e2e-layer-by-binary-prefix: a new root test binary contract_hyprland_seam, E2E layer by its contract_ prefix, requiring the fake-agent feature.
@task_item n=19 part=touches
decision/diagnostics-schema-conformance, contract/log-event-corr/process-start, contract/log-event-corr/process-exit and contract/log-event-corr/parse-rejected: the diag-line schema gains subject hyprctl-probe and parser hyprctl-json, each with its emitter.
@task_item n=20 part=touches
decision/synthetic-inputs-with-canary and decision/synthetic-fixture-text: a canary planted as window title and class reaches no line and no error; fixture text off the keep-list is the placeholder viola fixture text.
@task_item n=21 part=touches
decision/live-readings-not-a-test-layer, decision/live-verify-never-in-ci and decision/no-human-in-loop-verification: the Hyprland set is recorded by live runs on the operator's word on the dev host, never in CI; CI replays it through the fake.
@task_item n=22 part=touches
decision/codes-only-role-logs and decision/logs-carry-no-secrets: the hyprctl read writes codes-only lines; no title, class or other reply text reaches a line.
@task_item n=23 part=touches
decision/child-env-strip and decision/no-process-env-mutation-in-tests: the read removes named env vars from its child per spawn; the recorder removes HYPRLAND_INSTANCE_SIGNATURE for the absent variant.
@task_item n=24 part=touches
decision/no-sleep-synchronization and decision/within-7s-wait-bound: the deadline witness plants a script that execs a 30 s sleep and is bounded below 7 s, reading the pid through /proc.
@task_fact n=1 slug=7-test-data-fixtures
fact: Hyprland replies are recorded live on the dev host by the test-only recorder viola-hypr-tap, one set per compositor version, each fixture keeping only a closed per-request string keep-list (window addresses, the digit-form version tag, the ok dispatch text) and every other string replaced by the placeholder.
@task_fact n=2 slug=test-data-fixtures
fact: The fake agent answers as hyprctl when its executable file name is hyprctl, replaying the recorded Hyprland set byte for byte, its mode (recorded, oversize, malformed, absent) and set read from files beside its executable, never from an option or env var.
@task_fact n=3 slug=7-test-data-fixtures
fact: The fixture hygiene walk has a Hyprland class: schema conformance and the keep-list check, planted title and unlisted strings refused by class, and an empty Hyprland set fails the walk.
@task_fact n=4 slug=2-test-strategy
fact: The real hyprctl is never run in CI; the Hyprland seam is held in CI by the recorded set replayed through the fake, and its live recording waits on the operator's own word per level.
@task_fact n=5 slug=e2e-test-strategy
fact: The hyprctl reader is bounded at MAX_FRAME+1 with an over-cap reply refused as oversize before the exit is read, and a child past its deadline killed and reaped, each held by a remove-the-guard witness.
@task_fact n=6 slug=e2e-test-strategy
fact: HyprlandError stands on the error-body census as not an error body until a verb calls the seam.
@task_item n=1 part=untouched
The MCP surface: tools/list, tool results, invalid-params fields, MCP stdin line cap; no MCP tool calls the seam.
@task_item n=2 part=untouched
The wrapper channel, its JSON-RPC errors, link and unlink methods and channel logs.
@task_item n=3 part=untouched
Hook verbs, dialog bodies, hook deadlines and the perf budgets of hooks.
@task_item n=4 part=untouched
CLI verbs and their exit codes, json documents and human lines (send, wait, last, list, release, revive, run, answer); no verb calls the seam.
@task_item n=5 part=untouched
The test harness commands and documents: boot, status, cleanup, session record, pre-push, gate breach codes, run reasons.
@task_item n=6 part=untouched
Coverage floors, mutation gate thresholds and kill lines, and CI artifacts and retention.
@task_item n=7 part=untouched
Budget governor, wheel, send confirmation, PTY passthrough and screen reading rows.
@task_item n=8 part=untouched
Socket directory, strict modes, peer admission and home read rules.
@task_item n=9 part=untouched
The claude agent's own recorded hook payloads and CLI-version fixtures, whose rows stay as they are.
