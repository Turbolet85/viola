@task_prose part=session
The session widened the Hyprland compositor seam's closed request set from 6 to 8: a monitors query and one move-a-window-to-a-workspace dispatch, the move a single closed argv element holding only a checked window address and an integer workspace.
It added a closed Monitor reply type that keeps numbers and booleans only and drops every compositor text, and recorded one monitors reply and one move reply from the live compositor as fixtures (the move measured live once, on one host).
It settled the compositor read deadline at its unchanged 5 s (no longer provisional) and witnessed the read's drain cap with a new test.
The test-only recorder tool gained two verbs (record-monitors, record-move) with closed stderr failure codes; the fixture schema's request enum grew to 8 words.
The seam still has no product caller: no viola verb, CLI output, exit code, IPC method, socket, env var or crate was added, and nothing user-facing changed.
@task_file n=1 path=crates/viola-hyprland/src/lib.rs
what: the Hyprland compositor seam: its closed request set, each request's argv and word list, and its error words
@task_file n=2 path=crates/viola-hyprland/src/reply.rs
what: the parsers of the compositor's JSON replies into closed reply types that keep no compositor text
@task_file n=3 path=crates/viola-hyprland/src/read.rs
what: the bounded, time-limited read of the compositor control tool's reply (the read deadline and the reply size cap)
@task_file n=4 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs
what: the test-only recorder that captures compositor replies into scrubbed fixtures, its verbs and its closed stderr failure codes
@task_file n=5 path=crates/viola-e2e/tests/hypr_tap.rs
what: the recorder's rehearsal tests over planted stub compositor and terminal programs
@task_file n=6 path=tests/contract_bounded_readers.rs
what: the contract check listing every bounded reader with its cap and its witness test
@task_file n=7 path=tests/contract_hyprland_seam.rs
what: the contract check that the seam's recorded fixtures replay and parse through the seam
@task_file n=8 path=tests/contract_fixture_hygiene.rs
what: the contract check that recorded fixtures hold no host strings and only known request words
@task_file n=9 path=schemas/hyprland-fixture.v1.json
what: the schema of a recorded compositor fixture, with its closed request-word enum
@task_file n=10 path=fuzz/fuzz_targets/hyprctl_json.rs
what: the fuzz target over the compositor JSON reply parsers
@task_file n=11 path=fixtures/hyprland/0.56.2/Monitors.recorded-1.json
what: a recorded compositor monitors reply, numbers and booleans only, every string a placeholder
@task_file n=12 path=fuzz/corpus/hyprctl_json/monitors-row
what: a fuzz seed input for the compositor reply parsers
@task_file n=13 path=fixtures/hyprland/0.56.2/MoveToWorkspace.recorded-1.json
what: a recorded compositor move-window-to-workspace dispatch, its argv and its success reply
@task_item n=1 part=touches
Indirect: any row on time bounds, waits or the absence of a spinner or elapsed-time counter, through the compositor read deadline now settled at 5 s; the seam has no CLI caller yet, so no verb shows or waits on it, and the bound lands beside a verb's own bounds only when the future viola open verb arrives.
@task_item n=2 part=touches
Indirect: rows that keep foreign text out of human output (control characters escaped, no internals in errors or hints), through the new monitors reply type that drops every compositor string at the parse; nothing of it reaches any CLI line.
@task_item n=3 part=touches
Indirect: rows on closed refusal or failure words and the stderr refusal line, through the test-only recorder's closed stderr codes (version-unread, no-terminal-window, move-refused, move-unseen, window-left); these are a test tool's codes, not viola CLI refusals, hints or exit codes.
@task_item n=4 part=touches
Indirect: the command-name rows and the flat CLI verb set, only to confirm no name is taken: the move capability exists in the seam, but no viola verb places a window and the open verb is not yet added.
@task_item n=1 part=untouched
Brand identity, the flight-progress strip, readback box, send readback and handoff rows: no strip, board or readback behaviour changed.
@task_item n=2 part=untouched
Colour palette, typography, spacing, depth, border radius and iconography rows, and the CLI colour tokens and decision order: no output styling changed.
@task_item n=3 part=untouched
The viola list board and table, revive, run, verify, wait, last, link and unlink output contracts, and their refusal hints: no verb's output or refusal changed.
@task_item n=4 part=untouched
CLI exit codes, exit-21 causes, revive JSON refusal codes, wrapper-fault detail, peer reply screening and wrapper socket rows: no exit code, cause, IPC reply or socket changed.
@task_item n=5 part=untouched
The web-spa and rendered-page absences and the self-validation protocol: no surface was added.
