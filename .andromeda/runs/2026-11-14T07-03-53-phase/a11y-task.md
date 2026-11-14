@task_prose part=chunk
Builds viola's own Hyprland seam: the one place the binary talks to the compositor, carrying only the relied-on window queries and dispatches the later window entries need, no general Hyprland client.
Each compositor reply is read capped, treated as untrusted upstream output, parsed tolerantly into closed viola types, and every seam call ends in a closed outcome code logged codes-only.
Outside Hyprland the seam answers a closed unsupported outcome; a fake compositor replays scrubbed, live-recorded replies so the seam's tests run green in CI where no compositor runs.
Measures the relied-on reply shapes and effects live once each as named operator steps (read-only queries; dispatches that focus or move a window or show a workspace), which record the fixtures the fake replays.
Builds no verb and adds no CLI output: open, focus, up, adopt and the fleet console are their own entries.
@task_item n=1 part=needs
A new internal component, the compositor seam: the sole place any Hyprland request, reply shape or socket is named, as the pty and channel seams are for their crates; it prints nothing of its own to any terminal.
@task_item n=2 part=needs
Compositor replies are untrusted peer output: read under a frame cap and refused past it, parsed into closed types; window titles, classes and all other reply text are content, never a command, and reach no log line, error or refusal — the reply is screened down to closed outcome code words.
@task_item n=3 part=needs
Codes-only diagnostics: each seam call emits one closed outcome code (ok, unsupported, unreachable, malformed, ...) through the observability event macro, carrying no reply text, window address, pid, path or socket name; these diagnostics go to the log, never to the terminal.
@task_item n=4 part=needs
The seam's error types and messages interpolate nothing taken from a reply or the environment; a failure surfaces only as its closed outcome code.
@task_item n=5 part=needs
State: no compositor reachable — the seam returns a closed unsupported outcome, never a panic and never a guess.
@task_item n=6 part=needs
Linux only; window features target Hyprland only; CI stays on ubuntu-latest.
@task_item n=7 part=needs
The fake compositor replay: the seam's tests replay recorded replies on ubuntu-latest and must run green there — no test skipped or gated on a live compositor being present.
@task_item n=8 part=needs
Live measurement is operator-only: the named live-desktop steps record facts and fixtures but never gate CI; the lock state is read first before any dispatch and the run stops on locked, never unlocking.
@task_item n=9 part=needs
Recorded replies are fixtures scrubbed like every fixture: no addresses, paths, socket names or other host detail carried into the repository.
@task_item n=10 part=needs
Every test or control that spawns viola run, mcp, a wrapper or a compositor client carries a kill deadline; time bounds fall on the test driver, never on a human.
@task_item n=11 part=needs
No mutation run, local or dispatched; a guard test's one-off control runs through the gate's control step under a kill deadline, holding only when the control's own case ran and failed with the guard removed.
@task_item n=12 part=needs
No verb is built: the chunk adds no CLI verb, help row, refusal, exit code or --json document, and claims no capability whose acceptance needs a verb.
@task_item n=13 part=needs
A red or defect found here folds into the chunk as an inferred item with its own acceptance; one found outside the scope is carried forward to its owner entry.
@task_standing n=1
convention: each new test runs inside the existing nextest suites, its failure a nextest failure, with no separate a11y driver or suite
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=2
convention: the CI test gate allows no skipped test
home: 1-a11y-scope-summary
@task_standing n=3
convention: each new test is deterministic — zero flakiness, no retries, a breach fails the build
home: 10-slo-invariants-a11y-budgets
@task_standing n=4
convention: each new log line stays off the terminal; run diagnostics never reach it
home: a11y-scope-summary
@task_standing n=5
convention: each new error type's messages interpolate nothing; peer replies are screened to code words
home: cognitive-accessibility
@task_item n=1 part=not_needed
Colour and SGR rows — amber, bold, dim, NAME truncation, colour decision inputs, word beside every SGR span: the seam prints nothing styled.
@task_item n=2 part=not_needed
The CLI verb output contracts — help rows, list board and captions, revive, link and unlink, verify, wait, send, plugin install: no verb is built here.
@task_item n=3 part=not_needed
Screen-reader targets and setups (Orca with GNOME Terminal or with foot on Hyprland): manual passes over the CLI and TUI, not over the compositor seam.
@task_item n=4 part=not_needed
Human keystroke, wheel, focus/mouse/resize and vt100 feed panic rows of the TUI passthrough: the seam does not sit on the outer PTY.
@task_item n=5 part=not_needed
Keyboard navigation of the fleet console: that belongs to the fleet a11y verification entry, not this seam.
@task_item n=6 part=not_needed
Browser, web, touch, contrast, focus-management, WCAG mapping and structured violation JSON rows: viola has no graphical frontend.
@task_item n=7 part=not_needed
Budget gate, dialog hold, config.json values and MCP tool contracts: the seam neither sends nor answers dialogs nor serves MCP.
