@task_prose part=chunk
Builds the verb `viola open`: a terminal window (foot, started directly, no shell) per session beside its parent, from a closed flag allowlist, with `claude` alone after `--`.
The started wrapper finds its own window through the Hyprland seam (its first product caller), moves it with the one closed move dispatch to the target workspace, and records window and origin in its own state, logged codes-only.
Adds tests for the recorder's three untested `record-move` failure codes over the existing planted stubs, and lands the settled 5 s compositor read bound as a time-bound row of its own.
Adds a test-only script for R2, the one live reading that gates v2-07; implement stops before R2.
@task_file n=1 path=src/cmd/open.rs
what: the new `viola open` verb — its closed argument allowlist, the terminal spawn without a shell, the liveness wait and its refusal and --json output
@task_file n=2 path=crates/viola-e2e/tests/hypr_tap.rs mark=edits
what: the end-to-end tests of the test-only recorder over planted compositor stubs — edits: new tests for the `record-move` failure codes move-refused, move-unseen and window-left
@task_file n=3 path=Cargo.toml mark=edits
what: the workspace manifest declaring the Hyprland seam crate as a dependency — edits: the seam's entry, moved from the dev-dependencies table to a product dependency
@task_file n=4 path=scripts/r2-open.sh
what: the new test-only live-reading script for R2 — starts a parent, runs one `viola open … -- claude`, reads the verdict, closes both windows on every exit path
@task_item n=1 part=needs
The closed flag allowlist of `viola open`: a flag outside it, or a word after `--` other than `claude`, is refused before anything starts with exit 2 — clap's usage error, plus one fixed line beside the existing dashes-pair line.
@task_item n=2 part=needs
The new exit-1 refusals `terminal-not-found` and `not-live` (window left standing): each a kebab-case code word beside the unable word, with its one hint, never coloured, its fixed copy pinned by literal.
@task_item n=3 part=needs
The start-against-a-holder refusals `already-live` and `stale-heartbeat`, read before any spawn, reused by `viola open`.
@task_item n=4 part=needs
The `strict-modes-failed` refusal, raised before any log line or write; and no exit 11 from `viola open`.
@task_item n=5 part=needs
The `--json` output of `viola open`: one document, `{"v":1,"ok":{"name","parent","window","placement"}}` on success or `{"v":1,"error":"<code>","detail":null}` on refusal, with no hint and no stray bytes.
@task_item n=6 part=needs
The `viola --help` setup group label line, gaining `open`: `setup:  open, run, revive, verify, plugin install`; help lists served verbs only, and the two new hidden flags on `viola run` stay out of help.
@task_item n=7 part=needs
The time bounds `viola open` adds: liveness wait at most 30 s, window appearance 5 s polled at 100 ms, then `unplaced`; each as a time-bound row of its own beside the full lists, falling on the driver, never on the human.
@task_item n=8 part=needs
The settled 5 s compositor read bound `HYPRCTL_DEADLINE` joining the a11y time-bound rows (bounds fall on the driver, never on the human) as a row of its own.
@task_item n=9 part=needs
The placement outcome values recorded as codes: placed, `unplaced` (no parent window or no window seen in time), skipped off Hyprland with the seam's own `unsupported` / `unreachable` saying why; the open proceeds on a locked desktop.
@task_item n=10 part=needs
The move of the new window to its parent's workspace makes no focus change: the human's focused window and keystrokes are untouched by the open.
@task_item n=11 part=needs
The new log-only event kind `window {address?, workspace?, placement, origin}` and the snapshot's `window?`, replayed last line winning; logged codes-only, carrying no paths or free text.
@task_item n=12 part=needs
`list --json` and MCP `list` items gain `window?` and `origin?`; the six-column human board stays as it is.
@task_item n=13 part=needs
The origin (`cli` · `agent`) and the target workspace crossing onto the started `viola run`'s argv as two hidden checked flags, validated by the wrapper.
@task_item n=14 part=needs
The parent defaulting to the caller's `VIOLA_NAME`; names printed in output are the parsed arguments, with control characters escaped.
@task_item n=15 part=needs
The terminal argv built without any shell line, whatever characters the arguments hold; the wrapped session in its foot window keeps viola's own bytes off the terminal and run diagnostics off it.
@task_item n=16 part=needs
The recorder's failure code words `move-refused`, `move-unseen` and `window-left`, asserted by tests in the code-word format with no live window.
@task_item n=17 part=needs
New error types of the open path: errors as word, detail and hint; thiserror messages interpolate nothing; no stack trace or bare status as error text.
@task_item n=18 part=needs
R2 is a live reading under a scratch home: it gates v2-07 but never gates CI; manual and live checks stay supplemental, the fake (a stand-in foot and a keyed clients reply) carries every CI test.
@task_standing n=1
convention: each new fixed CLI string — refusal line, hint, help line, fixed-message line — is pinned by a literal test, its copy from design
home: 8-cognitive-accessibility · cognitive-accessibility
@task_standing n=2
convention: each new a11y assertion lives inside the existing test suites and fails as a nextest failure, with no separate a11y driver or artifact
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=3
convention: each new test is deterministic, zero flakiness and no retries; a breach fails the build
home: 10-slo-invariants-a11y-budgets
@task_standing n=4
convention: each new refusal ends with a hint and carries a kebab-case code beside a plain word, never coloured
home: 11-a11y-anti-patterns-never-do-these
@task_standing n=5
convention: each new output line prints no SGR off a terminal and escapes control bytes
home: aria-patterns-roles
@task_item n=1 part=not_needed
Dialog rows — dialog pending and answer, the dialog hold, its config value and the dialog-expired event: `viola open` raises and answers no dialog.
@task_item n=2 part=not_needed
Send and answer rows — input-not-ready, no-prompt-submitted, send refused while the human types, confirmed send and readback: the chunk sends nothing to a session.
@task_item n=3 part=not_needed
`viola link` and `viola unlink` rows — exits 20 and 21, result line, --json output: unchanged by this chunk.
@task_item n=4 part=not_needed
`viola revive` specific rows — revive --list, and revive refusals cwd-missing, no-session, pinned-hash-mismatch, squatted-name, session-live: revive is not changed.
@task_item n=5 part=not_needed
`viola verify` and `viola wait` output rows — step line, interrupted pair, waiting line, timed_out, the MCP wait timeout: not touched.
@task_item n=6 part=not_needed
Human board rendering rows — colour, stale-row dim, NAME bold and truncation, captions, the unwrapped separator: the board's columns and rendering are not changed.
@task_item n=7 part=not_needed
Absence rows of a web or graphical frontend, a rendered page, a touch surface and internationalization, and the browser, contrast and WCAG mapping rows: viola has none of these.
@task_item n=8 part=not_needed
The TUI passthrough's stdin classifier, human-key wheel taking and focus, mouse and resize sequence rows: the passthrough of `viola run` is not changed.
@task_item n=9 part=not_needed
Hook event and dialog call rows and peer-reply screening: the chunk adds no hook call.
@task_item n=10 part=not_needed
`viola plugin install` output rows: the verb itself is unchanged; only the help line naming it moves.
