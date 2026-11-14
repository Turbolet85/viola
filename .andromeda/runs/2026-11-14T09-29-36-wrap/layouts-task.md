@task_prose part=session
The session added a new sync library crate, viola-hyprland, the seam over the Hyprland compositor client hyprctl: four relied-on queries (clients, active window, workspaces, version) and two dispatches (focus a window by address, focus a workspace by id).
The seam spawns hyprctl directly with a closed argv, no shell, stdin null, the reply read up to the shared frame cap and refused whole past it, a provisional 5 s deadline that kills and reaps the child; every failure is one of five closed code words with a fixed message (unsupported, unreachable, oversize, malformed, refused).
Reply parsing keeps no text field: window titles, classes and every other reply string are dropped at the parse; the diag-line schema gains the subject hyprctl-probe and the parser hyprctl-json.
Measured live on Hyprland 0.56.2: the plan's plain dispatch argv was refused, 0.56.2 runs dispatch as Lua, and the measured hl.dsp.focus form replaced it; the replies were recorded by a new test-only recorder, viola-hypr-tap, into a scrubbed fixture set the fake agent replays as hyprctl in CI.
No product verb calls the seam yet: no CLI verb, flag, output line, hint, exit code or MCP tool changed; nothing was retired.
@task_file n=1 path=crates/viola-hyprland/Cargo.toml
what: the manifest of the new Hyprland seam crate, a sync library crate and workspace member
@task_file n=2 path=crates/viola-hyprland/src/lib.rs
what: the Hyprland seam's closed request set, its argv per request, the window address and workspace id newtypes, and its closed error words
@task_file n=3 path=crates/viola-hyprland/src/reply.rs
what: the hyprctl reply parsers, one per request, that keep only relied-on fields and drop every reply text
@task_file n=4 path=crates/viola-hyprland/src/read.rs
what: the hyprctl spawn and bounded read with its deadline, its frame cap and its codes-only diagnostic lines
@task_file n=5 path=crates/viola-hyprland/proptest-regressions/reply.txt
what: the committed property-test regressions of the hyprctl reply parsers
@task_file n=6 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs
what: the test-only Hyprland recorder binary that records hyprctl replies into scrubbed fixtures under a closed string keep-list
@task_file n=7 path=crates/viola-e2e/Cargo.toml
what: the manifest of the end-to-end test crate that holds the test-only recorder binaries
@task_file n=8 path=src/bin/viola-fake-agent/hyprctl.rs
what: the fake agent's hyprctl personality that replays recorded Hyprland replies by argv and mode
@task_file n=9 path=src/bin/viola-fake-agent.rs
what: the fake agent test binary on the test path, now answering as hyprctl when its file name is hyprctl
@task_file n=10 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the fake agent's unit tests of its reply modes
@task_file n=11 path=schemas/hyprland-fixture.v1.json
what: the schema of a recorded Hyprland fixture envelope
@task_file n=12 path=schemas/diag-line.v1.json
what: the diagnostic line schema with its subject and parse-rejected parser word lists
@task_file n=13 path=src/obs.rs
what: the observability setup with the list of viola log targets
@task_file n=14 path=fixtures/hyprland/0.56.2/ActiveWindow.recorded-1.json
what: a recorded Hyprland reply fixture, the active window query
@task_file n=15 path=fixtures/hyprland/0.56.2/Clients.recorded-1.json
what: a recorded Hyprland reply fixture, the clients query
@task_file n=16 path=fixtures/hyprland/0.56.2/FocusWindow.recorded-1.json
what: a recorded Hyprland reply fixture, the focus-window dispatch
@task_file n=17 path=fixtures/hyprland/0.56.2/Version.recorded-1.json
what: a recorded Hyprland reply fixture, the version query
@task_file n=18 path=fixtures/hyprland/0.56.2/Version.absent-1.json
what: a recorded Hyprland reply fixture, the version query with no compositor instance
@task_file n=19 path=fixtures/hyprland/0.56.2/Workspace.recorded-1.json
what: a recorded Hyprland reply fixture, the workspace dispatch
@task_file n=20 path=fixtures/hyprland/0.56.2/Workspaces.recorded-1.json
what: a recorded Hyprland reply fixture, the workspaces query
@task_file n=21 path=tests/contract_hyprland_seam.rs
what: the end-to-end contract check of the Hyprland seam against the recorded replies, the no-text canary included
@task_file n=22 path=tests/support/fake.rs
what: the test support that places the fake agent on the test path, now also as hyprctl
@task_file n=23 path=tests/contract_bounded_readers.rs
what: the bounded-reader census check with its boundaries rows and its product roots
@task_file n=24 path=tests/contract_error_bodies.rs
what: the error-body census check with its product roots and the error types that write no body
@task_file n=25 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene walk over recorded fixtures, now with a Hyprland class and its keep-list check
@task_file n=26 path=fuzz/fuzz_targets/hyprctl_json.rs
what: the fuzz target over the hyprctl reply parsers
@task_file n=27 path=fuzz/Cargo.toml
what: the fuzz workspace manifest listing the fuzz targets
@task_file n=28 path=fuzz/Cargo.lock
what: the fuzz workspace lockfile
@task_file n=29 path=Cargo.toml
what: the root workspace manifest, its members, dev-dependencies and test targets
@task_file n=30 path=Cargo.lock
what: the root workspace lockfile
@task_file n=31 path=scripts/sync-crates.txt
what: the list of sync crates each checked alone under the tokio ban
@task_file n=32 path=.gitattributes
what: the repository's per-path attributes for committed files
@task_item n=1 part=touches
The rule that the CLI holds every action: the session adds compositor actions (focus a window, focus a workspace) in a library seam that no CLI verb exposes yet; no verb was added, so a row stating every action has a verb now stands beside a seam action with none.
@task_item n=2 part=touches
viola verify --record output and the recorded fixture contracts: a second test-only recorder, viola-hypr-tap, now records fixtures beside viola verify --record; it is no viola verb and prints no verify output, but a row that names the only recorder or the recorder set is a touch.
@task_item n=3 part=touches
viola verify refusal for a recorded fixture not clean: the fixture hygiene walk gains a Hyprland class with a closed string keep-list, and the recorder refuses a fixture whole by file and closed code; a row stating what the hygiene walk checks or what a clean fixture is may be reached.
@task_item n=4 part=touches
The hint-for-every-refusal-cause and the closed refusal word rules: five new closed failure words (unsupported, unreachable, oversize, malformed, refused) with fixed messages that interpolate nothing exist, but no verb surfaces them, so they carry no hint and no exit code yet; the seam's error stands among the error types that write no body.
@task_item n=5 part=touches
Hints never quote text or name paths and codes-only logging: the seam's messages interpolate nothing and its diagnostic lines carry codes only, no window title or class crossing; a row holding that no upstream text reaches output now covers a compositor reply too.
@task_item n=6 part=touches
The diagnostic line word lists: the subject list is now 8 words (hyprctl-probe added) and the parse-rejected parser list 11 words (hyprctl-json added); viola log targets are now 8; any row whose cell holds those lists or counts is touched.
@task_item n=7 part=touches
Window features targeting Hyprland: the dispatch form relied on is the Lua form measured on 0.56.2, hl.dsp.focus with a window address or a workspace id; a row naming the plain dispatch focuswindow or dispatch workspace form is stale.
@task_item n=8 part=touches
Time bounds: the seam's provisional 5 s deadline falls on viola's own hyprctl child, never on a human; a row bounding waits or progress is touched only where it rules over child-process deadlines.
@task_item n=1 part=untouched
The board and BAY line of viola list, its columns, widths, colours, truncation and json document: no list output changed.
@task_item n=2 part=untouched
viola send, wait, last, answer, the readback mirror and its vocabulary: no send or wait line, word or json field changed.
@task_item n=3 part=untouched
viola run and viola revive starts, refusals, preflight order and json error codes: no start path changed.
@task_item n=4 part=untouched
CLI exit codes, global flags, the refusal line form, internal-error output and the frame verb hints: no code or hint was added.
@task_item n=5 part=untouched
The token rendering rows (colour, spacing, radius, motion, roles) and the styling, banner, spinner, prompt, pager and footer decisions: nothing styled was added.
@task_item n=6 part=untouched
The MCP tools, their results and the hidden-verb decisions, viola --help usage and verb groups, viola plugin install, link and unlink, and wheel verb output: no verb, tool or help line changed; the recorder is test-only and the release build carries viola alone.
@task_item n=7 part=untouched
The web surface and deep link absences: the session adds no page and no URL coupling.
