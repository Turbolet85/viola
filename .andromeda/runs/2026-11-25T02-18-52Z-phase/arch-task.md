@task_prose part=chunk
Changes the wrapper's window step so a placed window reaches its Hyprland workspace while the compositor's active workspace stays unchanged (the operator's F4 «no focus change»), in the form P3 measures from the compositor's own help, through the one Hyprland seam's closed request set, its tap and its fake.
Every caller of the move — viola open, viola up, viola revive under --origin — takes the same form or a named difference; the recorded move fixture is re-recorded only in an operator-granted, operator-fired live leg.
Adds to v2-07's acceptance a gate-read predicate: the active workspace read before a placing open equals the one read after; v2-07 is claimed and proved again, its live half gated on the operator.
Folds in the automatic verify of a new CLI at a start: says where it runs and what it stamps when its own preconditions fail (an untrusted folder, a modal, a red it caused itself), fixes that by the form the fork decides, with its own acceptance and a witness that cannot pass vacuously.
@task_file n=1 path=src/run/window.rs mark=edits
what: the wrapper's window step that places a started instance's window on its workspace through the Hyprland seam — edits: the move-to-workspace request it sends
@task_file n=2 path=src/run/auto_verify.rs mark=edits
what: the start's automatic verify of a new CLI version and the stamp it leaves — edits: where the automatic verify runs and what it stamps when its own preconditions fail
@task_file n=3 path=src/cmd/run.rs mark=edits
what: the viola run start path, the start order that keys the automatic verify of a new CLI version — edits: the start's call into the automatic verify
@task_item n=1 part=needs
The Hyprland seam crate as the one compositor seam: the window-move request in its closed request set gains or changes to a form that leaves the active workspace where it was; no second compositor seam.
@task_item n=2 part=needs
hyprctl spawned directly with a closed argv: the new move dispatch form must stay a closed argv built by the seam, with no shell-out.
@task_item n=3 part=needs
The seam's error answers (refused, unsupported, malformed, unreachable, oversize) and the window-not-found and terminal-not-found process-exit details: what the new move form returns on each, for its callers.
@task_item n=4 part=needs
The viola open, viola up and viola revive --origin verbs as callers of the move: each one's placement now keeps the active workspace, or carries a named difference P3 decides; their exit codes on a failed move.
@task_item n=5 part=needs
The window event appended by the wrapper and the instance snapshot's window field, fed by the placement the move makes.
@task_item n=6 part=needs
The seam's tap binary viola-hypr-tap and the fake compositor: they follow the new move form, and the fake's reply set is kept to what the real compositor answers.
@task_item n=7 part=needs
The recorded Hyprland fixtures per compositor version: the move fixture re-recorded only through the operator's grant (one terminal window, no claude, the host unlocked), scrubbed and hygiene-walked like every recorded fixture.
@task_item n=8 part=needs
Window features target Hyprland: the move form is measured against the compositor's version (0.56.2) through read-only queries of its help, version and state only; the phase dispatches nothing.
@task_item n=9 part=needs
The v2-07 acceptance clause as a predicate a gate entry reads: the active workspace read before and after a placing open are equal, witnessed against the fake in CI and by a gated operator-fired live leg with a rehearsal form.
@task_item n=10 part=needs
Real claude and live compositor runs are local only: no live claude or live window in CI; the live re-record and v2-07's live reading are operator-fired legs, never fired by the phase.
@task_item n=11 part=needs
Start verifies a new CLI version when keyed by the verify_new_cli config key: where that automatic verify runs (its working directory, versus the start's own folder) and in what home tree.
@task_item n=12 part=needs
Verify's own trust record and its refusal of an untrusted home tree: the automatic verify must not read a trust dialog of an untrusted folder as the CLI's red, and types into no trust dialog.
@task_item n=13 part=needs
The ledger stamps and their one writer: what the automatic verify stamps when its own preconditions fail — a failing stamp is kept and the version reads unverified for every later start; stamps merge replaces only their own version; a version is verified only when every relied row counts.
@task_item n=14 part=needs
No automatic verify of an earlier red version: a self-caused red stamped by the automatic verify would block every later automatic try; the fix's form decides how that red is told apart or not stamped.
@task_item n=15 part=needs
The automatic verify's own attempt lock and verify's probe folders: verify removes only the probe folders it created, kept under its own run dir.
@task_item n=16 part=needs
The folded item's witness cannot pass vacuously: its test must show a verify placed in an untrusted folder or with a failed precondition no longer stamps a red that a trusted verify would not, against the fake agent.
@task_item n=17 part=needs
Codes-only role logs and the run diagnostics role file: any line the automatic verify or the move adds on its own failure is a code, never content.
@task_standing n=1
convention: each new test runs in its own isolated viola home, removed by its owner record
home: established-decisions
@task_standing n=2
convention: tests run against the fake agent on ubuntu in CI; real claude and live compositor readings run only locally
home: ci-cd-approach
@task_standing n=3
convention: each recorded fixture is scrubbed and hygiene-walked before it is kept, and recorded only from a clean run
home: occupied-resources
@task_standing n=4
convention: each new OS-gated test names its reason
home: cross-cutting-patterns
@task_standing n=5
convention: each new process log line holds codes only, emitted through the one event emitter
home: cross-cutting-patterns · build-system
@task_standing n=6
convention: each new error stays inside its crate's one thiserror enum, its message interpolating nothing
home: conventions
@task_standing n=7
convention: coverage floors and zero panic lines in end-to-end runs hold over each new test and line of code
home: ci-cd-approach · project-directory-structure
@task_item n=1 part=not_needed
The MCP server's tools, results and stdio limits: the chunk adds no tool and changes no MCP result.
@task_item n=2 part=not_needed
The send path, paste bounds, typed-text normalisation and send confirmation: the move and the automatic verify type nothing into a session.
@task_item n=3 part=not_needed
The wheel, its claim, release and human take: placing a window does not move the wheel.
@task_item n=4 part=not_needed
Hook tiers, hook-to-kind map and dialog hold and answer rows: neither the move nor the verify's place touches a hook or a dialog answer.
@task_item n=5 part=not_needed
Channel frames, fault codes, link and unlink methods and their events: no channel method changes.
@task_item n=6 part=not_needed
The board and list payload rows, and the agents listing: the chunk changes no listed field.
@task_item n=7 part=not_needed
viola adopt, switch and plugin install rows: none of them places a window or keys the automatic verify.
@task_item n=8 part=not_needed
The PTY seam, vt100 readiness gate and screen signatures: the chunk reads no screen of its own.
@task_item n=9 part=not_needed
Release tooling, deploy stage, licensing and crates.io publishing rows: the chunk ships nothing.
@task_item n=10 part=not_needed
Fuzz workspace and weekly fuzzing rows: the chunk adds no fuzz target.
