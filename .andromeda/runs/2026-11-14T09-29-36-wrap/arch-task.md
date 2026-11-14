@task_prose part=session
The session added the Hyprland seam: a new sync library crate viola-hyprland that runs the relied-on compositor queries (clients, active window, workspaces, version) and dispatches (focus a window, focus a workspace) through hyprctl, with no product caller yet.
hyprctl is spawned directly from a caller-resolved path, no shell and no PATH lookup, stdin null, a closed argv; its stdout is read under MAX_FRAME + 1 and refused whole as oversize, and a child past a provisional 5 s deadline is killed and reaped.
Measured live on Hyprland 0.56.2: dispatch runs as Lua, so the dispatch argv is the Lua form hl.dsp.focus with only a closed Address or an i32 workspace id inside it; the plan's old dispatch form was disproved and replaced.
Every reply string (titles, classes, names) is dropped at the parse; failures are one closed error enum with five code words and fixed messages.
A test-only recorder binary viola-hypr-tap records scrubbed fixtures under a closed per-request keep-list; the fake agent answers as hyprctl when its file is named hyprctl; CI replays the recorded set, and a new fuzz target hyprctl_json joins the fuzz replay.
Counts moved: sync crates 4 to 5, viola log targets 7 to 8, diag-line subject words 7 to 8, parse-rejected parser words 10 to 11, fuzz targets 6 to 7, test-only recorders beside verify --record 1 to 2.
@task_file n=1 path=crates/viola-hyprland/Cargo.toml
what: the manifest of the new viola-hyprland crate, a sync workspace member depending on viola-core only among viola crates, with a code-free fake-agent feature
@task_file n=2 path=crates/viola-hyprland/src/lib.rs
what: the Hyprland seam's closed request set, the Address and WorkspaceId newtypes, the hyprctl program name and the crate's one error enum with its code words
@task_file n=3 path=crates/viola-hyprland/src/reply.rs
what: the tolerant closed parsers of hyprctl replies, dropping every text field, refusing past the frame cap
@task_file n=4 path=crates/viola-hyprland/src/read.rs
what: the hyprctl spawn and bounded read: direct spawn, no shell, no PATH lookup, stdin null, named env vars removed, frame-capped stdout, deadline kill and reap, its spawn pair and parse-rejected lines
@task_file n=5 path=crates/viola-hyprland/proptest-regressions/reply.txt
what: the committed property-test regressions of the hyprctl reply parsers
@task_file n=6 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs
what: the test-only recorder of Hyprland fixtures, scrubbing each reply to a closed keep-list and refusing an unclean fixture
@task_file n=7 path=src/bin/viola-fake-agent/hyprctl.rs
what: the fake agent's hyprctl personality, replaying recorded Hyprland fixtures by argv and its failure modes
@task_file n=8 path=schemas/hyprland-fixture.v1.json
what: the schema of a recorded Hyprland fixture envelope
@task_file n=9 path=fixtures/hyprland/0.56.2/ActiveWindow.recorded-1.json
what: a recorded scrubbed hyprctl fixture (active window) for Hyprland 0.56.2
@task_file n=10 path=fixtures/hyprland/0.56.2/Clients.recorded-1.json
what: a recorded scrubbed hyprctl fixture (clients) for Hyprland 0.56.2
@task_file n=11 path=fixtures/hyprland/0.56.2/FocusWindow.recorded-1.json
what: a recorded hyprctl dispatch fixture (focus window) for Hyprland 0.56.2
@task_file n=12 path=fixtures/hyprland/0.56.2/Version.recorded-1.json
what: a recorded scrubbed hyprctl fixture (version) for Hyprland 0.56.2
@task_file n=13 path=fixtures/hyprland/0.56.2/Workspace.recorded-1.json
what: a recorded hyprctl dispatch fixture (focus workspace) for Hyprland 0.56.2
@task_file n=14 path=fixtures/hyprland/0.56.2/Workspaces.recorded-1.json
what: a recorded scrubbed hyprctl fixture (workspaces) for Hyprland 0.56.2
@task_file n=15 path=fixtures/hyprland/0.56.2/Version.absent-1.json
what: the recorded hyprctl reply with no compositor instance, for Hyprland 0.56.2
@task_file n=16 path=tests/contract_hyprland_seam.rs
what: the end-to-end contract test of the Hyprland seam against the fake hyprctl: byte-for-byte replay, parse of each recorded reply, failure modes, the canary that no reply text reaches a line or error
@task_file n=17 path=fuzz/fuzz_targets/hyprctl_json.rs
what: the fuzz target of the hyprctl reply parsers
@task_file n=18 path=fuzz/corpus/hyprctl_json
what: the synthetic seed corpus of the hyprctl_json fuzz target, replayed in CI
@task_file n=19 path=Cargo.toml
what: the workspace manifest: its member crates, the shared dependency pins, the root package's dev-dependencies and test targets
@task_file n=20 path=Cargo.lock
what: the workspace lockfile
@task_file n=21 path=crates/viola-e2e/Cargo.toml
what: the manifest of the test-only viola-e2e crate and its test-only binaries
@task_file n=22 path=scripts/sync-crates.txt
what: the list of sync crates each checked tokio-free under its own deny-sync policy
@task_file n=23 path=src/obs.rs
what: the root bin's observability setup and its list of viola log targets
@task_file n=24 path=schemas/diag-line.v1.json
what: the diagnostics line schema, its subject words and parse-rejected parser words
@task_file n=25 path=src/bin/viola-fake-agent.rs
what: the fake agent test binary that stands in for claude in tests
@task_file n=26 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the fake agent's unit tests of its modes
@task_file n=27 path=tests/support/fake.rs
what: the root tests' support for placing the fake agent, now also as a fake hyprctl
@task_file n=28 path=tests/contract_bounded_readers.rs
what: the census of bounded readers at every boundary, its BOUNDARIES rows and product roots
@task_file n=29 path=tests/contract_error_bodies.rs
what: the census of error types against the error bodies surfaces write, its product roots and its not-error-bodies list
@task_file n=30 path=tests/contract_fixture_hygiene.rs
what: the recorded-fixture hygiene walk, now with the Hyprland fixture class and its keep-list check
@task_file n=31 path=fuzz/Cargo.toml
what: the separate fuzz workspace's manifest and its fuzz targets
@task_file n=32 path=fuzz/Cargo.lock
what: the fuzz workspace's lockfile, audited separately
@task_file n=33 path=.gitattributes
what: the repository's git attributes for committed files
@task_item n=1 part=touches
decision/window-features-target-hyprland: the session builds the first code under it, a Hyprland seam crate measured on 0.56.2; the row may want the seam beside it.
@task_item n=2 part=touches
decision/no-shell-out and decision/statusline-only-shell-out: hyprctl is a new external child, spawned directly with no shell; the dispatch argv now carries a Lua expression whose only inserted value is a closed Address or an i32, no reply text.
@task_item n=3 part=touches
decision/child-program-resolution, decision/no-path-lookup, decision/hooks-and-mcp-never-search-path: the hyprctl read does no PATH lookup; the caller resolves the program, and with none it reads unsupported and spawns nothing.
@task_item n=4 part=touches
decision/bounded-readers and contract/wrapper-channel-frames/max-frame: a new boundary, hyprctl stdout read under MAX_FRAME + 1 on its own thread, over-cap refused whole as oversize before the exit is read, stderr drained to the cap and dropped; the bounded-readers census gains it.
@task_item n=5 part=touches
decision/tolerant-external-payloads, decision/tolerant-serde-external, decision/unparseable-external-values-unknown, decision/serde-json-recursion-limit: hyprctl replies are parsed tolerantly and closed, a key of another type read as absent, a too-deep array malformed.
@task_item n=6 part=touches
decision/untrusted-upstream-text, decision/upstream-text-is-content, decision/codes-only-process-logs, decision/process-logs-hold-codes-only: every Hyprland reply string is dropped at the parse; no title or class reaches a line or an error.
@task_item n=7 part=touches
decision/thiserror-one-enum-per-crate, decision/one-error-enum-per-crate, decision/thiserror-messages-interpolate-nothing, contract/code-word/format: the new crate has one error enum HyprlandError with five fixed messages and closed code words.
@task_item n=8 part=touches
decision/nutype-newtypes, decision/nutype-domain-newtypes, decision/control-characters-refused: the new Address newtype refuses any other byte, a control character included, never strips.
@task_item n=9 part=touches
Sync crates and tokio containment (decision/sync-crates-tokio-free, decision/no-tokio-in-sync-crates, decision/sync-crates-compile-without-tokio, decision/tokio-containment, the deny-sync per crate row): viola-hyprland is the fifth sync crate, its graph free of tokio; where a row holds the count four it is stale.
@task_item n=10 part=touches
decision/cargo-workspace-flat-crates, decision/flat-cargo-workspace, decision/crate-naming, decision/viola-name-prefix: a new flat workspace member crate named viola-hyprland.
@task_item n=11 part=touches
Crate dependency direction (decision/core-depends-on-no-viola-crate, decision/pty-knows-no-agent, decision/claude-shapes-in-one-crate): the new crate depends on viola-core only, and on no viola-agent-claude, viola-state or viola-channel.
@task_item n=12 part=touches
Test-only binaries (decision/test-binaries-never-released, decision/e2e-test-only, decision/release-build-viola-only, decision/release-carries-viola-only, name/binary/viola-mcp-tap): viola-e2e gains a second test-only recorder viola-hypr-tap; a row naming the test-only binaries as viola-harness and viola-mcp-tap alone is stale.
@task_item n=13 part=touches
Recorded fixtures (decision/recorded-fixtures-scrubbed, decision/screen-fixtures-signature-rows-only, contract/unclean-fixture-refusal rows): a new Hyprland fixture class scrubbed to a closed per-request keep-list, every other string replaced by a placeholder, refused whole by the same unclean-fixture codes, and checked by the hygiene walk.
@task_item n=14 part=touches
Fake-agent seams (decision/fake-agent-test-seams, decision/tests-against-fake-agent, decision/ci-against-fake-agent, decision/test-seams-not-configuration): the fake agent gains a hyprctl personality chosen by its own file name, no option and no env var; CI replays the recorded Hyprland set.
@task_item n=15 part=touches
decision/fake-agent-feature-on-named-members and decision/e2e-noop-fake-agent-feature: viola-hyprland carries a code-free fake-agent feature, and the new contract test target requires it.
@task_item n=16 part=touches
Real CLIs only locally (decision/real-claude-local-only, decision/no-live-claude-in-ci, decision/real-cli-probe-local-only): the live hyprctl is measured and recorded only on the dev host; no CI image carries it.
@task_item n=17 part=touches
decision/no-probe-for-unrelied-shapes: only the relied set is queried and recorded; the exec placement dispatch and the monitors query are left out.
@task_item n=18 part=touches
Fuzzing (decision/fuzz-replay-ci, decision/fuzz-own-workspace, decision/fuzz-separate-workspace, decision/fuzz-lockfile-audited): a seventh fuzz target hyprctl_json joins the replay; the fuzz lockfile was found stale and refreshed.
@task_item n=19 part=touches
Observability (decision/obs-event-only-emitter, decision/diagnostics-level-sole-log-level, contract/diag-line/format, contract/parse-rejected-event/vt100-feed): a new subject hyprctl-probe on the spawn pair and a new parse-rejected parser hyprctl-json, a span hyprland.read; viola log targets 7 to 8; a row listing the targets, subjects or parsers or holding those counts is stale.
@task_item n=20 part=touches
Environment rows (decision/env-strip-names-only, decision/env-strip-reads-names-only, decision/harness-env-unread-by-product, decision/no-environment-configuration): the read removes named variables from hyprctl's environment by name; viola reads no new env var and adds no config key.
@task_item n=21 part=touches
Time bounds (decision/hook-connect-deadline, decision/statusline-deadline, decision/send-time-bound): a new provisional HYPRCTL_DEADLINE of 5 s falls on viola's own child, killed and reaped past it.
@task_item n=22 part=touches
decision/new-capabilities-as-verbs and decision/linux-only: the seam is a library with no verb and no product caller yet; it is Linux and Hyprland only.
@task_item n=23 part=touches
decision/sanitised-error-surfaces: HyprlandError stands in the error-bodies census as an error no surface writes yet.
@task_fact n=1 slug=occupied-resources
fact: The crate name viola-hyprland is occupied: the sync library crate holding viola's Hyprland seam.
@task_fact n=2 slug=occupied-resources
fact: The binary name viola-hypr-tap is occupied: a test-only recorder of Hyprland fixtures in viola-e2e, never in a release build.
@task_fact n=3 slug=crate-dependency-direction
fact: Compositor queries and dispatches go only through viola-hyprland's closed request set; the crate is sync and tokio-free and depends on viola-core alone among viola crates, never on viola-agent-claude, viola-state or viola-channel.
@task_fact n=4 slug=standard-contracts
fact: hyprctl is spawned directly from a caller-resolved path, no shell and no PATH lookup, stdin null, with a closed argv; a dispatch is the Lua form hl.dsp.focus whose only inserted value is a closed Address or an i32 workspace id, never reply text.
@task_fact n=5 slug=cross-cutting-patterns
fact: hyprctl stdout is read under MAX_FRAME + 1 and an over-cap reply is refused whole as oversize before the exit is read; a child past the provisional 5 s deadline is killed and reaped.
@task_fact n=6 slug=cross-cutting-patterns
fact: No Hyprland reply text crosses the seam: titles, classes, names and every other reply string are dropped at the parse, only addresses, ids, numbers, flags and the version digits kept.
@task_fact n=7 slug=standard-contracts
fact: The Hyprland seam fails with one closed set of code words, unsupported, unreachable, oversize, malformed and refused, each with a fixed message that interpolates nothing.
@task_fact n=8 slug=occupied-resources
fact: Recorded Hyprland fixtures keep only a closed per-request list of strings (window addresses and the version tag in digit form); every other string is replaced by a placeholder, a non-JSON reply is kept only when it is the success text, and the hygiene walk checks the same list.
@task_fact n=9 slug=occupied-resources
fact: The fake agent answers as hyprctl when its own executable file is named hyprctl, replaying recorded fixtures; no option and no env var selects it.
@task_item n=1 part=untouched
The wheel, the human take and release rules: no wheel rule changed.
@task_item n=2 part=untouched
Send, typing, paste and send confirmation, and the screen model readiness gate: nothing in them moved.
@task_item n=3 part=untouched
The budget governor, its gate, readings and thresholds.
@task_item n=4 part=untouched
Claude Code hooks, dialogs, the hook-to-kind map and the dialog hold.
@task_item n=5 part=untouched
The MCP server, its tools and results, and its stdin bounds.
@task_item n=6 part=untouched
The wrapper channel's methods, fault codes and JSON-RPC framing beyond the shared frame cap value.
@task_item n=7 part=untouched
CLI verbs, their arguments and exit codes: no verb was added or changed.
@task_item n=8 part=untouched
run, revive and the start order, the pinned copy and the claude child's argv and environment.
@task_item n=9 part=untouched
The events log, instance snapshots, links and the board read by list.
@task_item n=10 part=untouched
The statusline wrapper and its settings override, the plugin and its install.
@task_item n=11 part=untouched
viola verify, the capability ledger and its stamps: the claude CLI ledger is not touched by the Hyprland recordings.
@task_item n=12 part=untouched
config.json keys and the VIOLA_ environment namespace: no key and no variable was added.
