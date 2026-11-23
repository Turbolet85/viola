@task_prose part=session
The internal Hyprland seam crate's closed request set grew from 6 to 8: a monitors query and one move-to-workspace dispatch (one Lua element holding only a checked window address and an integer workspace).
The seam still has no product caller: no viola verb places a window, prints anything new, or reaches the human's terminal; no CLI verb, refusal, exit code, IPC method or event was added.
The compositor read deadline stays at 5 s and moved from provisional to settled; a test now times it, and a second test witnesses the read's drain cap on an oversize reply.
The test-only recorder tap gained two verbs (record-monitors, record-move); the move was recorded live once on Hyprland 0.56.2 by the tap's own foot window, and the monitors reply keeps numbers and booleans only.
The recorded Hyprland fixture set grew from 7 to 9 files, the fixture schema's request enum from 6 to 8 words; CI green 7/7, no capability claimed.
@task_file n=1 path=crates/viola-hyprland/src/lib.rs
what: the internal Hyprland seam crate's closed request set, its words and its dispatch argv
@task_file n=2 path=crates/viola-hyprland/src/reply.rs
what: the internal Hyprland seam's compositor reply parsers, now including the monitor rows and the move's success text
@task_file n=3 path=crates/viola-hyprland/src/read.rs
what: the internal Hyprland seam's bounded compositor read with its 5 s deadline, a driver-side time bound
@task_file n=4 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs
what: the test-only recorder tap that records live compositor replies into fixtures, spawning its own foot terminal for the move
@task_file n=5 path=crates/viola-e2e/tests/hypr_tap.rs
what: the recorder tap's rehearsal tests over planted stub compositor and terminal programs
@task_file n=6 path=tests/contract_bounded_readers.rs
what: the bounded-reader Boundary list check, its compositor reply row and the witness test it names
@task_file n=7 path=tests/contract_hyprland_seam.rs
what: the contract check that replays every recorded compositor reply through the fake and the seam's parsers
@task_file n=8 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene walk check that refuses kept host strings and unknown request words in recorded fixtures
@task_file n=9 path=schemas/hyprland-fixture.v1.json
what: the recorded compositor fixture schema, its closed request word list
@task_file n=10 path=fuzz/fuzz_targets/hyprctl_json.rs
what: the fuzz target over the compositor reply parse
@task_file n=11 path=fuzz/corpus/hyprctl_json/monitors-row
what: a fuzz corpus seed for the compositor reply parse, one monitor row
@task_file n=12 path=fixtures/hyprland/0.56.2/Monitors.recorded-1.json
what: a recorded compositor fixture, the monitors reply on Hyprland 0.56.2 with every string a placeholder
@task_file n=13 path=fixtures/hyprland/0.56.2/MoveToWorkspace.recorded-1.json
what: a recorded compositor fixture, the move-to-workspace dispatch and its success text on Hyprland 0.56.2
@task_item n=1 part=touches
decision/time-bounds-fall-on-the-driver: the compositor read deadline is now settled at 5 s, a bound on the driver only; by the phase card it is deliberately not listed yet, because the seam has no product caller, and it lands beside viola open's own bounds when that verb calls the seam.
@task_item n=2 part=touches
decision/time-bounds-never-on-human: the settled 5 s compositor read deadline falls on no human input; a picker should confirm the row stays true and that no row needs the bound until a product caller exists.
@task_item n=3 part=touches
absence/user-time-limit: a new settled time bound (the compositor read deadline) exists in code; it is not a time limit on the user, so the absence stands, but the row is the one to check against it.
@task_item n=4 part=touches
decision/no-unwarned-timeouts: the compositor read now returns unreachable after a settled 5 s; it has no product caller, so no human sees it yet.
@task_item n=5 part=touches
decision/no-assertions-for-internal-crates: the internal Hyprland seam crate gained requests, a reply type and tests, and no a11y assertion was added for it, as this row rules.
@task_item n=6 part=touches
decision/screen-reader-target-tui and decision/tui-screen-reader-test-setup: their stack is Orca with foot on Hyprland; the seam can now move a window to a workspace, but only the test tap uses it, on its own foot window, so the TUI screen-reader setup is unchanged.
@task_item n=7 part=touches
decision/live-readings-never-gate: the move was measured live once on the operator's word and recorded; only the recorded fixture replay gates, the live firing does not.
@task_item n=8 part=touches
decision/human-keystrokes-never-blocked and decision/tui-terminal-belongs-to-child: a window-placing dispatch now exists in the seam but no viola verb issues it, so the wrapped terminal and the human's keys are not reached.
@task_item n=9 part=touches
decision/a11y-cases-inside-coverage-suite and decision/ci-test-gate-no-skips: new seam, tap and contract tests run inside the existing nextest coverage suite with no skips; CI green 7/7.
@task_item n=10 part=touches
decision/paths-not-carried: the recorder keeps no monitors string and keeps the move only as its success text, so no host name or path reaches a fixture.
@task_item n=1 part=untouched
CLI output rows (help, list board, refusal layout, hint and unable lines, exit codes, json failure documents): no verb, refusal word, exit code or printed string changed.
@task_item n=2 part=untouched
Colour and SGR rows (amber dialog, bold names, dim stale rows, no SGR off a terminal): nothing prints.
@task_item n=3 part=untouched
TUI passthrough contract rows (entry, own bytes, vendor content, vt100 feed panic, focus/mouse/resize sequences): the wrapper's terminal path was not touched.
@task_item n=4 part=untouched
Revive, link/unlink, plugin install, hook call, MCP frame tool and socket or strict-modes refusal rows: none of those verbs or their refusals moved.
@task_item n=5 part=untouched
Dialog hold, wait timeout and verify stop grace bounds: none of those bounds or their config values changed.
@task_item n=6 part=untouched
Absences of a web or graphical frontend, a touch surface, a rendered page and internationalization: still absent.
