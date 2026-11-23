@task_prose part=session
The session widened the Hyprland seam's closed request set from 6 to 8: a monitors query and one move-to-workspace dispatch that names a window by address and an integer workspace.
It added a typed monitor reply that keeps numbers and booleans and drops all text, and it records and replays both new requests as Hyprland 0.56.2 fixtures, which now number 9.
It settled the compositor read deadline at 5 s, a value it already had, and added a witness test showing the drain keeps one byte past the frame cap and reports oversize.
The test-only fixture tap gained two recording verbs, so it now has 4 verbs. No viola CLI verb, MCP tool, output line, refusal, exit code, hint or flag was added or changed.
The seam still has no product caller. No viola verb places a window yet; the first one is meant to be viola open, which is not yet built.
@task_file n=1 path=crates/viola-hyprland/src/lib.rs
what: the Hyprland seam crate: its closed request set, each request's argv and its word list
@task_file n=2 path=crates/viola-hyprland/src/reply.rs
what: the Hyprland seam's typed reply parsers for hyprctl JSON replies, now including the monitor reply
@task_file n=3 path=crates/viola-hyprland/src/read.rs
what: the Hyprland seam's bounded and timed hyprctl read: the read deadline and the frame-cap drain
@task_file n=4 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs
what: the test-only Hyprland fixture recorder (tap) and its recording verbs and string keep-list
@task_file n=5 path=crates/viola-e2e/tests/hypr_tap.rs
what: the rehearsal tests of the test-only fixture recorder over planted stubs
@task_file n=6 path=tests/contract_bounded_readers.rs
what: the bounded-readers contract check: the Boundary row for the hyprctl reply and its witness
@task_file n=7 path=tests/contract_hyprland_seam.rs
what: the Hyprland seam contract check: replay and parse of every recorded reply
@task_file n=8 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene check over the recorded Hyprland fixtures
@task_file n=9 path=schemas/hyprland-fixture.v1.json
what: the Hyprland fixture schema and its closed request-word enum
@task_file n=10 path=fuzz/fuzz_targets/hyprctl_json.rs
what: the fuzz target for the hyprctl JSON reply parsers
@task_file n=11 path=fixtures/hyprland/0.56.2/Monitors.recorded-1.json
what: a recorded Hyprland 0.56.2 monitors reply fixture
@task_file n=12 path=fixtures/hyprland/0.56.2/MoveToWorkspace.recorded-1.json
what: a recorded Hyprland 0.56.2 move-to-workspace dispatch reply fixture
@task_file n=13 path=fuzz/corpus/hyprctl_json/monitors-row
what: a fuzz corpus seed for the hyprctl JSON parsers
@task_item n=1 part=touches
decision/cli-holds-every-action: the seam can now issue a window placement, but no viola verb issues it. The CLI gains no action, so check only that no row already counts window placement as a CLI action.
@task_item n=2 part=touches
contract/cli-verbs and contract/viola-help/verb-groups: viola open is named as the seam's first product caller but is not built. No verb row or help group changes; check that no row lists viola open as a standing verb.
@task_item n=3 part=touches
decision/flat-verbs and decision/targets-are-wrapped-names: the new dispatch targets a compositor window by address and a workspace number, not a ViolaName. This is a seam request, not a CLI target, so the rules hold as written; check only that neither claims seam requests.
@task_item n=4 part=touches
decision/hints-never-quote-or-locate and decision/hint-for-every-cause: the tap's new failure codes on stderr (no-terminal-window, move-refused, move-unseen, window-left) belong to a test-only tool. They are no viola refusal or hint, so check only that no row's cause list takes them.
@task_item n=1 part=untouched
The viola list board, BAY line, LINKS block and their columns, widths and colours: no board or list output changed.
@task_item n=2 part=untouched
viola send, wait, last, answer, revive, run and plugin install output, refusals and --json documents: none changed.
@task_item n=3 part=untouched
CLI exit codes, the exit-21 detail words and the frame-verb hints: no code, word or hint was added.
@task_item n=4 part=untouched
viola mcp tools and their results: no tool was added and no result shape changed.
@task_item n=5 part=untouched
CLI token rendering, SGR styling, TTY detection and the plain output modes: no styling changed.
@task_item n=6 part=untouched
viola verify output and refusals, and absence/web-surface and absence/cli-deep-links: unchanged.
