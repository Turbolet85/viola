@task_prose part=session
The Hyprland compositor seam (viola-hyprland) widened its closed request set from 6 to 8: a monitors query and one move-to-workspace dispatch.
The move is ONE argv element, the Lua dispatcher hl.dsp.window.move, with only a checked window address and an i32 workspace entering it; it was measured live once on Hyprland 0.56.2 (exit 0, stdout ok).
The monitors reply is read into a closed Monitor record of numbers and booleans only (id, size, position, focused, active workspace); every reply text is dropped at the parse.
The compositor read deadline HYPRCTL_DEADLINE was settled at its unchanged 5 s (no longer provisional), and the bounded read's drain past MAX_FRAME got a witness test.
The test-only recorder viola-hypr-tap gained two verbs (record-monitors, record-move), and two recorded fixtures joined the Hyprland 0.56.2 set; the fixture schema's request enum went from 6 to 8 words.
The seam still has no product caller: no viola verb places a window. No CLI verb, channel method, event, socket, env var, crate or dependency was added; no capability was claimed.
@task_file n=1 path=crates/viola-hyprland/src/lib.rs
what: edited — the viola-hyprland compositor seam: its closed request set (Request, WORDS) and the closed argv each request hands to hyprctl spawned directly
@task_file n=2 path=crates/viola-hyprland/src/reply.rs
what: edited — the seam's tolerant parse of hyprctl JSON replies into closed types (now Monitor, parse_monitors, the move's success text)
@task_file n=3 path=crates/viola-hyprland/src/read.rs
what: edited — the seam's bounded, time-bound hyprctl read: the HYPRCTL_DEADLINE read deadline and the MAX_FRAME drain cap
@task_file n=4 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs
what: edited — the test-only viola-hypr-tap fixture recorder: its verbs and its string keep-list for recorded compositor replies
@task_file n=5 path=crates/viola-e2e/tests/hypr_tap.rs
what: edited — the rehearsal of the test-only recorder over planted stub hyprctl and terminal programs
@task_file n=6 path=tests/contract_bounded_readers.rs
what: edited — the bounded-readers contract: the Boundary rows naming each boundary's cap and its witness test (the hyprctl reply row)
@task_file n=7 path=tests/contract_hyprland_seam.rs
what: edited — the Hyprland seam contract: recorded replies replayed through the fake and parsed into their types
@task_file n=8 path=tests/contract_fixture_hygiene.rs
what: edited — the recorded-fixture hygiene walk: planted unclean or unknown-request fixtures refused
@task_file n=9 path=schemas/hyprland-fixture.v1.json
what: edited — the Hyprland fixture schema: its closed request enum
@task_file n=10 path=fuzz/fuzz_targets/hyprctl_json.rs
what: edited — the hyprctl-json fuzz target in the separate fuzz workspace, replayed in CI
@task_file n=11 path=fuzz/corpus/hyprctl_json/monitors-row
what: added — a seed in the hyprctl-json fuzz corpus
@task_file n=12 path=fixtures/hyprland/0.56.2/Monitors.recorded-1.json
what: added — a scrubbed recorded fixture of the compositor's monitors reply
@task_file n=13 path=fixtures/hyprland/0.56.2/MoveToWorkspace.recorded-1.json
what: added — a scrubbed recorded fixture of the move-to-workspace dispatch and its success reply
@task_item n=1 part=touches
decision/hyprctl-direct-spawn names only the hl.dsp.focus dispatch: it is stale; the closed argv now also holds the move's ONE element hl.dsp.window.move with only the checked address and an i32 entering it (rests on the overseer's delegate word F3: PROVISIONAL).
@task_item n=2 part=touches
decision/hyprland-seam-crate (the one compositor seam, a closed request set): the set it rules over grew from 6 to 8 requests (5 queries, 3 dispatches); true as stated unless a cell counts it; the seam still has no product caller.
@task_item n=3 part=touches
The Hyprland seam errors contracts (malformed, oversize, refused, unreachable, unsupported): the two new requests report through the same five words, no word added; check their wording or any listed request set.
@task_item n=4 part=touches
decision/bounded-readers: the hyprctl reply boundary's witness moved to the drain test (a reply twice the cap keeps one byte past MAX_FRAME and reads oversize); a row naming the old witness or guard is stale.
@task_item n=5 part=touches
decision/tolerant-external-payloads, decision/untrusted-upstream-text and decision/upstream-text-is-content: the monitors reply is parsed tolerantly into a closed record with no text field, reply text dropped at the parse.
@task_item n=6 part=touches
decision/recorded-fixtures-scrubbed: two recorded Hyprland fixtures added; the monitors fixture keeps numbers and booleans only, the move only its success text.
@task_item n=7 part=touches
decision/e2e-test-only, decision/test-binaries-never-released and name/binary/viola-hypr-tap: the test-only recorder gained the verbs record-monitors and record-move (2 to 4 verbs), still test-only.
@task_item n=8 part=touches
decision/fuzz-replay-ci, decision/fuzz-own-workspace and decision/fuzz-separate-workspace: the hyprctl-json fuzz target now also drives parse_monitors, with no dependency added to the fuzz lockfile.
@task_item n=9 part=touches
decision/window-features-target-hyprland: the seam can now issue a window move to a workspace and read the monitors, though no viola verb places a window yet.
@task_item n=10 part=touches
decision/no-shell-out: the move dispatch is spawned directly with a closed argv, the Lua element holding no free text.
@task_fact n=1 slug=standard-contracts
fact: The compositor's monitors reply is read into a closed Monitor record of numbers and booleans only (a required id; optional width, height, x, y, focused and active workspace); every text of the reply is dropped at the parse.
@task_item n=1 part=untouched
The wrapper channel, its JSON-RPC methods, fault codes and frames: no method, frame or error code moved.
@task_item n=2 part=untouched
The CLI verbs, their arguments, exit codes and --json documents, and the MCP tools: no verb or tool added, none changed.
@task_item n=3 part=untouched
Hooks, dialogs, the wheel, sends, links, lineage, revive and the launch pair: no part of them was reached.
@task_item n=4 part=untouched
State and disk: event kinds, the events log, snapshots, ledger stamps, config.json keys and diagnostics lines are unchanged.
@task_item n=5 part=untouched
The build system and CI: no dependency added or bumped, no toolchain, lint, cargo-deny, workflow or gate step changed.
@task_item n=6 part=untouched
Occupied names: no crate, binary, command, env var, socket or port was taken or freed.
@task_item n=7 part=untouched
The capability ledger and verify: no capability claimed or flipped, no ledger row moved.
