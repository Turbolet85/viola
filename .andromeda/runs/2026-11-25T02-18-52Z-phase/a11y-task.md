@task_prose part=chunk
Changes the wrapper's window step so a placed window reaches its workspace while the compositor's active workspace stays where it was (the operator's F4 «no focus change»), in the form P3 measures from the compositor's own help, inside the Hyprland seam's closed request set, never a second compositor seam.
Every caller of the move — viola open, viola up, viola revive under --origin — reads the same answer; the seam's tap, fake and recorded move fixture follow the form, the fixture re-recorded only in one operator-granted, operator-fired live leg.
Adds to v2-07's acceptance a gate-read clause: the active workspace read before a placing open equals the one read after; re-proves v2-07, its live half gated on the operator's reading.
Folded item: says from the code where the automatic verify of a new CLI runs at a start and what it stamps when its own preconditions fail (an untrusted folder, a modal, a red it caused itself), with a fix whose form is a fork, its own acceptance and a witness that cannot pass vacuously.
@task_file n=1 path=src/run/window.rs mark=edits
what: the wrapper's window step, the one that places an opened window on its workspace through the Hyprland seam and waits for it — edits: the move-to-workspace request it sends
@task_file n=2 path=crates/viola-hyprland mark=edits
what: the Hyprland seam, the compositor boundary with its closed request set, its tap and its fake — edits: the move request's form, the fake's reply set
@task_file n=3 path=src/run/auto_verify.rs mark=edits
what: the automatic verify of a new claude CLI version run at a start, and the verify stamp it writes
@task_file n=4 path=src/cmd/run.rs mark=edits
what: the start of viola run, where the automatic verify is triggered — edits: the start
@task_item n=1 part=needs
The window step's move of a placed window: a behaviour changed — the window reaches its workspace and the active workspace does not move, for viola open, viola up and a viola revive under --origin.
@task_item n=2 part=needs
The window step's wait stays inside its bound of 5 s; the changed move adds no wait that falls on the human.
@task_item n=3 part=needs
Each compositor read the chunk adds or uses (the active workspace before and after, the seam's queries) is bounded at 5 s, and every time bound falls on the driver, never on the human.
@task_item n=4 part=needs
viola open's live wait stays bounded at 30 s with the changed move in its path.
@task_item n=5 part=needs
The output lines of the move's callers — viola up's instance line, viola open's and a revive's human and --json output — keep their fixed form; a placement that still takes the screen along is said to the operator as a defect against F4, in a fixed string pinned by literal, never a coloured or bare word.
@task_item n=6 part=needs
The automatic verify's stamp on its own failed preconditions: a behaviour changed — what a start stamps when the verify runs in an untrusted folder, meets a modal or causes its own red; viola verify's step lines and its stamp keep their contract.
@task_item n=7 part=needs
Start's verify and its stop grace never fall on the human; whatever the fork's fix adds to the start (a refusal, a skipped or re-run verify) keeps the refusal form — reason code beside unable, one hint per code, no stack trace or bare status, --json one clean document.
@task_item n=8 part=needs
The live re-record of the move fixture and the live reading of v2-07's active-workspace clause are operator-fired, gated legs that never gate the build; manual checks stay supplemental.
@task_item n=9 part=needs
The new tests (the move's form through the fake, the active-workspace predicate, the folded verify witness) run inside the existing suites as nextest failures under the CI gate: no skips, zero flakiness, no retries, coverage and doctest required.
@task_item n=10 part=needs
Error text the changed seam or verify code raises interpolates nothing and keeps its code word form.
@task_standing n=1
convention: each new test case runs inside the existing test suites as a nextest case, and its failure fails the build with no skip or retry
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=2
convention: each new or changed fixed CLI string is pinned by its literal in a test
home: cognitive-accessibility
@task_standing n=3
convention: each live reading is an operator-fired leg that never gates CI
home: keyboard-test-harness
@task_item n=1 part=not_needed
The TUI passthrough contracts (entry, human keystrokes, focus, mouse and resize sequences, vendor content, vt100 feed panic, refused peer): the chunk moves a window on the compositor, it does not touch the passthrough or the bytes on the terminal.
@task_item n=2 part=not_needed
The viola list board rows (caption, order, colour, stale-row dim, NAME truncation, the unwrapped separator): the chunk prints no board.
@task_item n=3 part=not_needed
viola link and unlink, viola plugin install and viola revive --list contracts: none of those verbs is reached.
@task_item n=4 part=not_needed
Frame verb, hook call and MCP frame tool refusals (socket-dir, strict-modes, server-verify): the chunk adds no frame path.
@task_item n=5 part=not_needed
viola send refusals, the dialog hold and the human-takes-the-wheel decisions: the chunk sends nothing into a session.
@task_item n=6 part=not_needed
The screen-reader targets and setups, and the absences of a web, graphical or touch frontend and of i18n: no surface of that kind is touched.
