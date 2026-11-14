@task_prose part=session
The session added viola's own seam over the Hyprland compositor: a new sync library crate viola-hyprland that spawns hyprctl directly (no shell, no PATH lookup, closed argv) for four queries and two focus dispatches, parses the JSON reply tolerantly and drops every text field (titles, classes, names).
Every failure is one of five closed code words with a fixed message that interpolates nothing: unsupported, unreachable, oversize, malformed, refused; a reply over the shared frame cap is refused whole, a child past a provisional 5 s deadline is killed and reaped.
No product caller yet: no viola verb calls the seam, so no CLI output, refusal line, hint, exit code or --json document changed; the release build still carries viola only.
It measured Hyprland 0.56.2 live on the founder's host: dispatch now runs as Lua, so the focus dispatches use the measured hl.dsp.focus form; recorded replies were scrubbed by a closed per-request keep-list and replayed by the fake agent acting as hyprctl in CI.
It added a test-only recorder binary viola-hypr-tap, a fixture schema, a fuzz target hyprctl_json, a fixture-hygiene class for Hyprland fixtures, and two new diagnostic words: subject hyprctl-probe and parser hyprctl-json; log targets 7 to 8, sync crates 4 to 5.
@task_file n=1 path=crates/viola-hyprland/Cargo.toml
what: the manifest of the new Hyprland seam crate, a sync library free of tokio and the agent, state and channel crates
@task_file n=2 path=crates/viola-hyprland/src/lib.rs
what: the Hyprland seam's closed request set, the window-address and workspace-id newtypes, and its one error enum of five closed code words with fixed messages
@task_file n=3 path=crates/viola-hyprland/src/reply.rs
what: the tolerant parsers of hyprctl replies into window, workspace and version types that keep no text field
@task_file n=4 path=crates/viola-hyprland/src/read.rs
what: the bounded direct spawn of hyprctl under a deadline and the frame cap, with its spawn diagnostic lines
@task_file n=5 path=crates/viola-hyprland/proptest-regressions/reply.txt
what: the committed property-test regressions of the hyprctl reply parsers
@task_file n=6 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs
what: the test-only recorder of live hyprctl replies into scrubbed fixtures, by a closed keep-list
@task_file n=7 path=src/bin/viola-fake-agent/hyprctl.rs
what: the fake agent's hyprctl personality that replays recorded replies in tests
@task_file n=8 path=src/bin/viola-fake-agent.rs
what: the test fake agent binary, now switching to its hyprctl personality by its file name
@task_file n=9 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the fake agent's unit tests of its modes
@task_file n=10 path=schemas/hyprland-fixture.v1.json
what: the schema of a recorded Hyprland reply fixture envelope
@task_file n=11 path=schemas/diag-line.v1.json
what: the diagnostic log line schema, its subject and parse-rejected parser word lists
@task_file n=12 path=src/obs.rs
what: the list of viola log targets
@task_file n=13 path=fixtures/hyprland/0.56.2/ActiveWindow.recorded-1.json
what: a recorded, scrubbed Hyprland 0.56.2 reply fixture
@task_file n=14 path=fixtures/hyprland/0.56.2/Clients.recorded-1.json
what: a recorded, scrubbed Hyprland 0.56.2 reply fixture
@task_file n=15 path=fixtures/hyprland/0.56.2/FocusWindow.recorded-1.json
what: a recorded, scrubbed Hyprland 0.56.2 reply fixture
@task_file n=16 path=fixtures/hyprland/0.56.2/Version.recorded-1.json
what: a recorded, scrubbed Hyprland 0.56.2 reply fixture
@task_file n=17 path=fixtures/hyprland/0.56.2/Version.absent-1.json
what: a recorded Hyprland 0.56.2 reply fixture for the no-instance case
@task_file n=18 path=fixtures/hyprland/0.56.2/Workspace.recorded-1.json
what: a recorded, scrubbed Hyprland 0.56.2 reply fixture
@task_file n=19 path=fixtures/hyprland/0.56.2/Workspaces.recorded-1.json
what: a recorded, scrubbed Hyprland 0.56.2 reply fixture
@task_file n=20 path=tests/contract_hyprland_seam.rs
what: the end-to-end contract tests replaying recorded hyprctl replies through the fake and the read
@task_file n=21 path=tests/support/fake.rs
what: the test support that places the fake agent on the test path, now also as hyprctl
@task_file n=22 path=tests/contract_bounded_readers.rs
what: the census of bounded readers over the product roots
@task_file n=23 path=tests/contract_error_bodies.rs
what: the census of error bodies over the product roots and its list of errors no surface writes
@task_file n=24 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene walk, now with a Hyprland fixture class and its keep-list check
@task_file n=25 path=fuzz/fuzz_targets/hyprctl_json.rs
what: the fuzz target over the hyprctl reply parsers
@task_file n=26 path=fuzz/corpus/hyprctl_json
what: the synthetic seed corpus of the hyprctl reply fuzz target
@task_file n=27 path=fuzz/Cargo.toml
what: the fuzz workspace manifest and its target list
@task_file n=28 path=fuzz/Cargo.lock
what: the fuzz workspace lockfile
@task_file n=29 path=Cargo.toml
what: the root workspace manifest, its members, dev-dependencies and test targets
@task_file n=30 path=Cargo.lock
what: the root lockfile
@task_file n=31 path=crates/viola-e2e/Cargo.toml
what: the manifest of the test-only e2e crate and its binaries
@task_file n=32 path=scripts/sync-crates.txt
what: the list of sync crates each checked under its own tokio ban
@task_file n=33 path=.gitattributes
what: the repository's file attributes
@task_item n=1 part=touches
decision/no-internals-in-errors: a new error enum stands under it; its five fixed messages name no path, pid or reply text, though no surface prints them yet.
@task_item n=2 part=touches
contract/peer-reply-screening/code-word and the code-word rows: the seam's failures are closed kebab code words checked as code words (unsupported, unreachable, oversize, malformed, refused), not yet a refusal any verb prints.
@task_item n=3 part=touches
decision/escape-controls-in-human-output and decision/list-rows-escape-all-controls: indirectly, compositor text never enters viola; window titles and classes are dropped at the parse and a window address refuses any control character.
@task_item n=4 part=touches
decision/linux-terminal-platform: indirectly, the Linux platform now has a measured Hyprland 0.56.2 compositor seam beside the terminal, with no CLI expression yet.
@task_item n=5 part=touches
contract/viola-verify-refusals/unclean-fixture: indirectly, a second test-only recorder now refuses an unclean fixture whole by closed code, beside viola verify's own recording; the verify verb's refusal itself is unchanged.
@task_item n=6 part=touches
decision/stdout-data-stderr-messages and decision/no-stack-traces: no verb calls the seam yet, so no new stdout or stderr line, hint or exit code exists for a Hyprland failure; a row stating the full set of refusal causes stays whole.
@task_fact n=1 slug=new
fact: viola reaches Hyprland only through its own seam, which keeps no window title, class or other reply text and fails only as one of five closed code words with a fixed message; no viola verb uses it yet.
@task_item n=1 part=untouched
Brand identity, colour palette, typography, spacing, depth, border radius and iconography: no glyph, colour, column or strip changed.
@task_item n=2 part=untouched
CLI verb output and refusal contracts (list board, send readback, link, release, revive, run, wait, last, verify output lines): no verb's output, refusal pair, hint or --json document changed.
@task_item n=3 part=untouched
CLI exit codes and exit 21 causes: no exit code or cause was added or moved.
@task_item n=4 part=untouched
Command names and the help output: the new recorder is a test-only binary, not a viola verb; the release build carries viola only.
@task_item n=5 part=untouched
Colour decision order and colour depth: no environment variable or terminal check was added; the seam reads no env var.
@task_item n=6 part=untouched
Anti-patterns and the self-validation protocol: no human-facing output was designed in this session.
