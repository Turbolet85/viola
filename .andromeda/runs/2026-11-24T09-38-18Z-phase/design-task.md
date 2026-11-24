@task_prose part=chunk
Adds a new flat CLI verb `viola adopt <session-id> --name <n> [--parent <p>]` that takes a plain claude session started outside viola, by its session-shaped id, and starts a viola wrapper whose claude child resumes it under an operator-given name and optional parent, so it stands on the board as a named instance with its lineage and is driven like any wrapped session.
It is built on revive's start path: the resume argv, the claude agents --json live-elsewhere preflight and its closed session-live refusal, and the start shared with run; a malformed id and a session live in another claude are refused with closed words before any child is spawned, and viola never takes over a running process's terminal.
Conservative arm: adopt never ends or signals a process it did not start; it resumes only a session live nowhere and refuses a live one with a hint to end it by hand.
Folded CI item: re-measures the two env-capture security-negative cases that wait by design against the test runner's kill line, and gives them a margin by their test class or reduced work, never by raising the kill line for every test.
@task_file n=1 path=src/cmd/revive.rs
what: the viola revive verb's start path — the resume argv, the claude agents --json live-elsewhere preflight and its session-live refusal, and the start shared with viola run, which adopt is built on
@task_file n=2 path=.config/nextest.toml mark=edits
what: the test runner's configuration of per-test kill lines and test classes — edits: the class or override that gives a case that waits by design its margin
@task_item n=1 part=needs
The new verb adopt joins the flat CLI verb table and the command-name register as a verb of its own, with its row and group in the viola --help verb groups.
@task_item n=2 part=needs
adopt's start refusals: a malformed session id refused with a closed word, a session listed live by the claude agents --json read refused as session-live, a name already held (squatted-name / already-live) and a second adopt of the same id each with a defined closed outcome — every refusal printed in the fixed unable/hint two-line refusal form on stderr, showing no path or pid.
@task_item n=3 part=needs
adopt's --json documents: a result is one document on stdout, a refusal a clean failure document carrying a closed refusal code from the revive --json exit-1 code set or a new code added beside them, no hint line under --json, and no document once the child has held the terminal.
@task_item n=4 part=needs
adopt's exit codes and exit-21 causes follow the CLI exit-code table: refusals at exit 1, strict-modes and socket-directory failures at exit 21 with their closed causes and the fixed instance-state and home-state refusal pairs.
@task_item n=5 part=needs
adopt's output after the child starts matches viola run and revive output: the child-started line, the automatic-verify line forms, and nothing printed while wrapped.
@task_item n=6 part=needs
The session-live refusal of the live-elsewhere guard and its hint: the hint tells the operator to end the session by hand, never quotes text, names no internals and never names viola release.
@task_item n=7 part=needs
Unwrapped sessions are never a target: adopt reaches a plain session only by its session id to resume it once it is live nowhere, never sending to, signalling or taking over a bare session's process or terminal.
@task_item n=8 part=needs
Nothing announces success before confirmation: adopt reports the session adopted only once the wrapper and its resumed child are confirmed started.
@task_item n=9 part=needs
adopt prints no interactive prompt or pager, no spinner, and only the step counter where it shows progress.
@task_item n=10 part=needs
The adopted instance's name and parent land as its first snapshot and the lineage record the board reads, so it shows in viola list as a wrapped strip in a stable slot; a parent the operator gives that is malformed or unknown has a closed outcome.
@task_item n=11 part=needs
adopt's home-tree and strict-modes checks run before any home read beyond the strict ones, as revive's refusal order sets them.
@task_item n=12 part=needs
adopt's human output is ASCII-only, escapes every control character in any echoed id or name, never wraps lines, and uses colour only as a second cue through the hand-written SGR module under the CLI colour decision order.
@task_item n=13 part=needs
The two security-negative env-capture cases that wait by design are given a measured time per case against their kill line, locally and under coverage, and a margin by their class or reduced work, never by raising the kill line for every test.
@task_standing n=1
convention: every new verb gets its command name, its --help row in a verb group, and its verb-output contract
home: surface-cli · surface
@task_standing n=2
convention: every new refusal carries a fixed unable line and, in human mode only, one hint line, and a closed --json code
home: surface-cli · surface
@task_standing n=3
convention: every new message names no internals, no stack trace, no path or pid, and keeps data on stdout and messages on stderr
home: anti-patterns-never-do-these
@task_standing n=4
convention: every new line of human output is ASCII text with no decorative glyph, check-mark or emoji prefix
home: typography · iconography
@task_item n=1 part=not_needed
The readback box, send readback and viola send mirror contracts: adopt sends no prompt and reads back no send.
@task_item n=2 part=not_needed
The flight-progress strip fields, dialog-pending cocked strip, handoff transfer marker and pause/release control transfer of brand-identity: adopt does not change how a strip is drawn or how control is handed over.
@task_item n=3 part=not_needed
The colour palette's arrival amber accent and the colour-depth tokens: adopt adds no dialog-pending state and no new colour.
@task_item n=4 part=not_needed
viola verify, viola wait, viola last, viola link and unlink, viola answer, viola plugin install, and viola focus exit-1 codes: adopt reuses none of those verbs' outputs.
@task_item n=5 part=not_needed
The wrapper fault --json detail, peer reply screening and wrapper socket refused-peer contracts: adopt talks to no running wrapper over its socket.
@task_item n=6 part=not_needed
The web-spa surface, rendered-page absence, border radius, depth and self-validation protocol tests: viola renders no page.
@task_item n=7 part=not_needed
Model and effort switching and viola up's reboot reading are out of scope; the fleet console is no verb's output and is not built here.
