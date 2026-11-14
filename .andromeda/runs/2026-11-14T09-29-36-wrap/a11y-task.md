@task_prose part=session
The session added viola-hyprland, a new internal sync crate: the seam behind which viola reads the Hyprland compositor (clients, active window, workspaces, version) and dispatches window or workspace focus, each through a direct hyprctl spawn with a closed argv, no shell, stdin null, stdout capped at the shared frame cap.
Every failure is one closed code word with a fixed message that interpolates nothing: unsupported, unreachable, oversize, malformed, refused; reply text such as window titles and classes is dropped at the parse and reaches no line and no error.
A provisional 5 s deadline falls on viola's own hyprctl child, killed and reaped past it; no product verb calls the seam yet, so no CLI or TUI surface prints anything new.
It measured Hyprland 0.56.2 live on the founder's host (dispatch runs as Lua, the plan's dispatch form disproved) and recorded a scrubbed fixture set under a closed per-request keep-list, replayed in CI by the fake agent answering as hyprctl.
It added a test-only recorder binary, a contract test binary in the E2E layer, a fuzz target, and two new diag-line words (subject hyprctl-probe, parser hyprctl-json); nothing was retired.
@task_file n=1 path=crates/viola-hyprland/Cargo.toml
what: the manifest of the new internal crate viola-hyprland, the Hyprland compositor seam, a sync crate with no tokio
@task_file n=2 path=crates/viola-hyprland/src/lib.rs
what: the Hyprland seam's closed request set, its window address and workspace id forms, and its error enum of five code words whose thiserror messages interpolate nothing
@task_file n=3 path=crates/viola-hyprland/src/reply.rs
what: the parsers of hyprctl replies into closed types that drop every reply text (titles, classes, names), refusing an over-cap reply as oversize
@task_file n=4 path=crates/viola-hyprland/src/read.rs
what: the hyprctl read: a direct spawn of viola's own child with no shell, the provisional 5 s deadline that kills and reaps it, the frame cap, and its codes-only diagnostic lines
@task_file n=5 path=crates/viola-hyprland/proptest-regressions/reply.txt
what: the committed property-test regressions of the hyprctl reply parsers
@task_file n=6 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs
what: the test-only recorder of live hyprctl replies, scrubbing every string off a closed keep-list before a fixture is written
@task_file n=7 path=src/bin/viola-fake-agent/hyprctl.rs
what: the fake agent's hyprctl personality on the test path, replaying recorded replies byte for byte
@task_file n=8 path=schemas/hyprland-fixture.v1.json
what: the schema of a recorded hyprctl fixture envelope
@task_file n=9 path=fixtures/hyprland/0.56.2/ActiveWindow.recorded-1.json
what: a recorded, scrubbed hyprctl active-window reply
@task_file n=10 path=fixtures/hyprland/0.56.2/Clients.recorded-1.json
what: a recorded, scrubbed hyprctl clients reply
@task_file n=11 path=fixtures/hyprland/0.56.2/FocusWindow.recorded-1.json
what: a recorded hyprctl focus-window dispatch reply
@task_file n=12 path=fixtures/hyprland/0.56.2/Version.recorded-1.json
what: a recorded hyprctl version reply
@task_file n=13 path=fixtures/hyprland/0.56.2/Version.absent-1.json
what: a recorded hyprctl reply with no compositor instance
@task_file n=14 path=fixtures/hyprland/0.56.2/Workspace.recorded-1.json
what: a recorded hyprctl workspace dispatch reply
@task_file n=15 path=fixtures/hyprland/0.56.2/Workspaces.recorded-1.json
what: a recorded, scrubbed hyprctl workspaces reply
@task_file n=16 path=tests/contract_hyprland_seam.rs
what: the E2E contract tests of the Hyprland seam: recorded replay, codes-only failure lines, and a canary proving no window text reaches a line or an error
@task_file n=17 path=fuzz/fuzz_targets/hyprctl_json.rs
what: the fuzz target over the hyprctl reply parsers
@task_file n=18 path=fuzz/corpus/hyprctl_json
what: the synthetic seed corpus of the hyprctl reply fuzz target
@task_file n=19 path=Cargo.toml
what: the workspace manifest: its members and shared dependencies
@task_file n=20 path=Cargo.lock
what: the workspace lockfile
@task_file n=21 path=crates/viola-e2e/Cargo.toml
what: the manifest of the test-only binaries crate
@task_file n=22 path=scripts/sync-crates.txt
what: the list of sync crates each checked with its tokio ban, now five
@task_file n=23 path=src/obs.rs
what: the log setup and the set of viola log targets, now eight
@task_file n=24 path=schemas/diag-line.v1.json
what: the diagnostic line schema: its subject words and parse-rejected parser words
@task_file n=25 path=src/bin/viola-fake-agent.rs
what: the fake agent on the test path, standing in for the vendor binary
@task_file n=26 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the fake agent's own unit tests of its modes
@task_file n=27 path=tests/support/fake.rs
what: the test support that places the fake agent on the test path
@task_file n=28 path=tests/contract_bounded_readers.rs
what: the census of bounded readers, its boundary rows and product roots
@task_file n=29 path=tests/contract_error_bodies.rs
what: the census of error bodies, each with a writer and a witness, and its list of error types that are not error bodies
@task_file n=30 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene walk over recorded fixtures, now with a Hyprland class that checks the keep-list
@task_file n=31 path=fuzz/Cargo.toml
what: the fuzz manifest and its targets
@task_file n=32 path=fuzz/Cargo.lock
what: the fuzz lockfile
@task_file n=33 path=.gitattributes
what: the repository's file attributes
@task_item n=1 part=touches
decision/time-bounds-never-on-human: a new time bound, the provisional 5 s hyprctl deadline, falls on viola's own hyprctl child, killed and reaped past it, never on the human; the row may need to name this bound beside the others.
@task_item n=2 part=touches
decision/time-bounds-fall-on-the-driver: the new hyprctl deadline is a bound on a child process viola drives, not on a person; its value 5 s is provisional, to be settled when the first product caller arrives.
@task_item n=3 part=touches
absence/user-time-limit and decision/no-unwarned-timeouts: the new deadline adds no time limit on the user; it bounds a compositor query.
@task_item n=4 part=touches
decision/thiserror-messages-interpolate-nothing: a new thiserror enum, HyprlandError, with five fixed messages that interpolate nothing, pinned by literal in its own tests.
@task_item n=5 part=touches
contract/code-word/format and decision/codes-beside-plain-word: five new failure code words (unsupported, unreachable, oversize, malformed, refused) and six request words (clients, active-window, workspaces, version, focus-window, workspace), each a kebab code word.
@task_item n=6 part=touches
decision/error-bodies-have-writer-and-witness: HyprlandError is recorded as not an error body, since no verb calls the seam yet and no surface writes one.
@task_item n=7 part=touches
decision/no-assertions-for-internal-crates: a new internal crate, viola-hyprland, with no user-facing output; it is asserted only in its own crate tests and the contract binary.
@task_item n=8 part=touches
decision/peer-replies-screened-to-code-words and decision/no-screen-content-parsing: hyprctl replies are parsed into closed types, every text field (window titles, classes, names) dropped, failures reduced to closed code words.
@task_item n=9 part=touches
decision/run-diagnostics-off-terminal: the hyprctl read writes codes-only diagnostic lines (process-start and process-exit with subject hyprctl-probe, parse-rejected with parser hyprctl-json) to the log, none to the terminal.
@task_item n=10 part=touches
decision/paths-not-carried: the hyprctl program path is resolved by the caller and carried into no message and no line.
@task_item n=11 part=touches
decision/screen-reader-target-tui: the target Orca with foot on Hyprland now sits beside a viola seam that queries and focuses Hyprland windows; at 0.56.2 dispatch runs as Lua; nothing yet moves a window the human uses.
@task_item n=12 part=touches
decision/live-readings-never-gate and decision/ci-test-gate-no-skips: live Hyprland readings were operator legs on the founder's host; CI replays recorded fixtures through the fake hyprctl, no live compositor in CI, no test skipped.
@task_item n=13 part=touches
decision/a11y-inside-existing-suites and decision/a11y-cases-inside-coverage-suite: a new contract test binary joins the E2E layer of the existing suites, and a new fuzz target joins the fuzz replay; no separate a11y suite.
@task_item n=14 part=touches
decision/single-vendor-content-boundary: compositor reply text is a second outside source, dropped at the parse; it never reaches the terminal or the log.
@task_item n=1 part=untouched
TUI passthrough contracts (entry, human keystrokes, focus, mouse and resize sequences, vendor content, viola terminal bytes, feed panic): no passthrough behaviour changed.
@task_item n=2 part=untouched
CLI refusal, exit code, --json failure, fault and hint contracts, and every verb's output contract (list, wait, verify, revive, link, plugin install, help): no verb calls the new seam, so no printed text changed.
@task_item n=3 part=untouched
Colour and SGR decisions, NAME truncation, board captions and visual design rows: no terminal output changed.
@task_item n=4 part=untouched
Keyboard navigation, the wheel and send-refusal rules: the new focus dispatch is a compositor window request with no product caller, not terminal focus or human keystrokes.
@task_item n=5 part=untouched
Manual screen-reader and keyboard pass setup, the tier, WCAG and compliance rows, and the absences of a web frontend, touch surface and internationalization.
@task_item n=6 part=untouched
CI a11y gate rules, flakiness and budget rows: no gate threshold, floor or bound changed.
