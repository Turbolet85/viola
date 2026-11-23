@task_prose part=session
The Hyprland seam's closed request set grew from 6 to 8: a monitors query and one move-to-workspace dispatch, the move crossing to hyprctl as ONE argv element holding only a checked window address and an i32 workspace.
A monitors reply is read into a closed Monitor type with no text field (id, size, position, focused, active workspace), under the MAX_FRAME cap and serde's default depth; the reply's text is dropped at the parse.
The hyprctl read deadline was settled at its unchanged 5 s (no longer provisional), timed by a test, and the drain past the cap was witnessed by a test that keeps one byte past MAX_FRAME and reads oversize.
The test-only Hyprland recorder tap gained two verbs, record-monitors and record-move; its string keep-list keeps no monitors string and a move reply only as its success text; two scrubbed fixtures were recorded on the host (Hyprland 0.56.2), the move once on the operator's own word.
The seam still has no product caller: no viola verb places a window; no IPC method, socket, env var, dependency or crate was added and nothing was retired.
@task_file n=1 path=crates/viola-hyprland/src/lib.rs
what: the Hyprland seam crate - its closed request set and the argv each request hands the directly spawned hyprctl child, the move dispatch's one-element form
@task_file n=2 path=crates/viola-hyprland/src/reply.rs
what: the hyprctl reply parsers - tolerant JSON reads into closed types that drop the compositor's reply text, capped at MAX_FRAME
@task_file n=3 path=crates/viola-hyprland/src/read.rs
what: the bounded hyprctl child read - its deadline (HYPRCTL_DEADLINE, 5 s) and the drain capped at MAX_FRAME, with their timed and drain tests
@task_file n=4 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs
what: the test-only Hyprland fixture recorder tap - its verbs, its closed stderr codes and its string keep-list allowlist, spawning its own terminal for the move
@task_file n=5 path=crates/viola-e2e/tests/hypr_tap.rs
what: the recorder tap's rehearsal tests over planted hyprctl and terminal stubs
@task_file n=6 path=tests/contract_bounded_readers.rs
what: the bounded readers census - one Boundary row per product reader, the hyprctl reply row and its witness test
@task_file n=7 path=tests/contract_hyprland_seam.rs
what: the Hyprland seam contract - recorded replies replayed through the fake and parsed into their types
@task_file n=8 path=tests/contract_fixture_hygiene.rs
what: the recorded fixture hygiene walk - planted unclean or unknown-request Hyprland fixtures refused
@task_file n=9 path=schemas/hyprland-fixture.v1.json
what: the Hyprland fixture schema - its closed request word enum
@task_file n=10 path=fuzz/fuzz_targets/hyprctl_json.rs
what: the hyprctl JSON parser fuzz target, a parser surface in fuzz scope
@task_file n=11 path=fixtures/hyprland/0.56.2/Monitors.recorded-1.json
what: a recorded, scrubbed Hyprland monitors reply fixture - numbers and booleans only, every string the placeholder
@task_file n=12 path=fixtures/hyprland/0.56.2/MoveToWorkspace.recorded-1.json
what: a recorded Hyprland move-to-workspace dispatch fixture - its argv and the success text alone
@task_file n=13 path=fuzz/corpus/hyprctl_json/monitors-row
what: a fuzz corpus seed for the hyprctl JSON parser (a monitors row)
@task_item n=1 part=touches
decision/hyprctl-read-deadline: stale - its state still reads provisional; the deadline is now settled at 5 s, value unchanged, and its guard should name the timed test read_against_the_deadline_returns_unreachable_no_sooner_than_five_seconds beside the kill test (the deadline-5s control holds). Expected amendment.
@task_item n=2 part=touches
decision/hyprland-recorder-string-allowlist: add-to - the keep-list now covers two more request words: monitors keeps no string, move-to-workspace keeps only its success text; guard record_keeps_no_monitors_string (the monitors-keep-none control holds). Expected amendment.
@task_item n=3 part=touches
decision/hyprland-reply-text-dropped: a new reply type falls under it - the Monitor type holds no text field, and a move reply is read only as the success text.
@task_item n=4 part=touches
decision/bounded-readers-census and decision/bounded-external-readers: the hyprctl reply Boundary row's witness moved to read_of_a_reply_twice_the_cap_keeps_one_byte_past_it_and_is_oversize, and its types text now also names Monitor; needle and cap unchanged.
@task_item n=5 part=touches
decision/max-frame-cap, decision/max-frame-shared-cap and decision/reads-capped-at-max-frame: a new parser (parse_monitors) refuses one byte past MAX_FRAME as oversize, and the drain of the hyprctl child keeps MAX_FRAME plus one byte while the child writes (drain-cap control).
@task_item n=6 part=touches
decision/child-output-bounded and decision/child-output-tolerant: the hyprctl child's output for two new requests is read bounded and tolerantly (a key of another type read as absent, an element without an id stepped over, a non-array reply malformed).
@task_item n=7 part=touches
decision/parser-fuzz-scope: the hyprctl JSON fuzz target now also drives the monitors parse, with a size bound in place of an id check, and a monitors corpus seed.
@task_item n=8 part=touches
decision/json-default-depth-limit and decision/serde-json-bounded-depth: the monitors parse reads at serde's default depth - a new reader under the rule, no exception.
@task_item n=9 part=touches
decision/fixture-recording-scrubbed, decision/fixture-scrub-and-refusal, decision/fixture-hygiene and decision/fixtures-reviewed-before-commit: two new recorded Hyprland fixtures, each scrubbed by the keep-list, the monitors one read back by hand before landing; the hygiene walk's planted unknown request word moved from monitors to exec since the schema now admits monitors.
@task_item n=10 part=touches
decision/closed-types-for-protocol-fields and decision/upstream-text-is-content: the move dispatch argv admits only a closed Address and an i32; no reply text or user text enters the hyprctl argv.
@task_item n=11 part=touches
decision/new-env-read-needs-log-entry: the seam gained two requests and reads no new env var; its instrumentation is the standing hyprctl-probe spawn pair and parse-rejected log entry.
@task_item n=12 part=touches
decision/release-build-no-test-binaries and decision/release-build-without-test-features: the test-only recorder tap gained the record-monitors and record-move verbs; it stays a test-only binary outside the release build.
@task_item n=13 part=touches
decision/live-round-rig-keys-own-pty (by analogy): the tap's record-move moves only the window of a terminal it spawned itself, then ends and reaps that terminal and checks its window gone.
@task_fact n=1 slug=input-validation
fact: The Hyprland seam's move-to-workspace dispatch crosses to hyprctl as one argv element in which only a checked window address and an i32 workspace enter, never reply or user text; it rests on the overseer's delegated word (F3) and lands PROVISIONAL.
@task_fact n=2 slug=data-protection
fact: A recorded Hyprland monitors reply keeps its numbers and booleans (monitor ids, sizes, positions, scale, focus, active workspace) and no string; kept numbers are accepted as host facts, since the recorder allowlist governs strings only.
@task_fact n=3 slug=authentication-authorization
fact: The test-only Hyprland recorder's move acts only on the window of a terminal it spawned itself, ends and reaps that terminal after the move, and refuses with no-terminal-window when no window of its own terminal appears.
@task_item n=1 part=untouched
The IPC channel, its wrapper methods, sockets, socket directory and peer euid checks: no channel method, endpoint or socket was added.
@task_item n=2 part=untouched
The MCP server, its tools, stdin line cap and rmcp feature set.
@task_item n=3 part=untouched
Strict-modes checks of the viola home, the state tree, snapshots and stamps, and their refusal contracts.
@task_item n=4 part=untouched
Paste, send and answer text validation, control-character sets and their refusal words.
@task_item n=5 part=untouched
Hooks, revive, run, the launch pair and lineage: no product verb calls the seam.
@task_item n=6 part=untouched
Dependency pins, licences, advisories, toolchain pins, CI workflow permissions and the canary scan: no dependency, harness command or CI step changed.
@task_item n=7 part=untouched
Secret management and credential stripping of the claude child; error-handling contracts and exit codes, the seam's error set unchanged at five words.
@task_item n=8 part=untouched
The absences: no network listener, outbound call, secret or remote access was added.
