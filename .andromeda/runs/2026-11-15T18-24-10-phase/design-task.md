@task_prose part=chunk
Test-only corrective chunk that makes a green coverage suite and a green mutation baseline for viola reachable, without running either.
It gives the hook fail-open test a test-channel socket name that does not grow with the pid and the event label, and proves its length bound under the mutation scratch.
It resizes the mutants profile's slow-case kill and moves the receipt wait with it, re-measuring the slow cases' wall time against the new margins.
It kills the ten surviving mutants of the recorder test binary with tests, or rules the recorder's main out of the tier with its reason.
It enforces the record-home naming rule, that a home given to viola verify --record holds no path component named home, through the rig's home-naming helper.
@task_file n=1 path=tests/hook_fail_open.rs mark=edits
what: the hook verb's fail-open test, which binds the test channel's endpoint socket in a temp dir — edits: the test socket name and its join and bind
@task_file n=2 path=tests/support/watch.rs mark=edits
what: the test support's receipt wait, the bound every fake wait-for caller shares — edits: the WITHIN constant and its doc comment
@task_file n=3 path=tests/support/home.rs mark=edits
what: the test rig's home-naming helper that gives each test, including every viola verify --record caller, its test home — edits: TestHome::new, the home() fixture, scratch()
@task_file n=4 path=.config/nextest.toml mark=edits
what: the test runner's config, its mutants profile and the slow-case kill — edits: the mutants profile's slow-timeout and any override for the slow-case class
@task_file n=5 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs mark=edits
what: the recorder, a test-only binary of the end-to-end crate — edits: its tests over main's argument guard and its record and record-dispatch arms, version_of's Version arm, and texts' Array arm
@task_item n=1 part=needs
The hook fail-open test's channel socket: a name that does not grow with the pid or the event label, with a computed or asserted bound under sun_path's 107 bytes for a 7-digit pid under the mutation scratch. The hook verb's exit code and stderr that the test asserts stay as they are asserted now.
@task_item n=2 part=needs
The mutants profile's slow-case kill: a margin the chunk sizes for the ~5 s class (instance-state tampered pinned copy and plugin-folder rewrite, revive after a killed wrapper, snapshot replay over an unreadable snapshot), staying below cargo-mutants' 20 s auto-timeout floor.
@task_item n=3 part=needs
The receipt wait moves with the kill and stays strictly below it. It is shared by every wait-for caller, and the narrower change (a per-call or a scaled bound) is chosen so that the verify stopped-during-resume case stops timing out waiting for its run's start.
@task_item n=4 part=needs
Each of the slow cases' wall time is measured against the new margins, with the host's load recorded beside it.
@task_item n=5 part=needs
The ten recorder survivors: each one killed by a test that fails on the mutated site, or the test plan's tier text states, with its reason, why the recorder's main is outside the tier.
@task_item n=6 part=needs
Each survivor-killing test is proved by a one-off remove-the-guard control run under a kill deadline. The control's case is the test's name alone, and it holds only when that case ran and failed.
@task_item n=7 part=needs
The record-home naming rule: a test home given to viola verify --record never holds a path component named home, and the rig's home-naming helper enforces it so that a plan cannot name the home wrong.
@task_item n=8 part=needs
Test-only bound: no product line changes unless a test proves one wrong. Every viola verb's output, exit codes and refusals stay as they are, and no requirement or capability claim is added.
@task_item n=9 part=needs
No mutation run (local or dispatched) and no coverage run serve as acceptance; no live claude start and no live desktop step.
@task_item n=10 part=needs
Statements about how a tool or crate behaves (the test runner's profile inheritance and slow-timeout form, the temp-dir crate's use of TMPDIR) are measured and cited, not assumed. Absence probes are checked against the plan's own test names and fixture keys.
@task_item n=11 part=needs
A red result or a defect found inside the pinned scope folds into this chunk as an inferred item with its own acceptance. A defect found outside the scope is carried forward and names its owner entry.
@task_item n=1 part=not_needed
Brand identity, the strip-board expression, the readback box and the flight-progress strip: the chunk adds no display and changes no strip.
@task_item n=2 part=not_needed
Colour palette, CLI colour tokens, colour depth and the colour decision order: no styled output is added or changed.
@task_item n=3 part=not_needed
Typography, spacing, iconography, depth, border radius and the rendered-page and web-surface absences: viola renders no page, and the chunk touches no glyph or column.
@task_item n=4 part=not_needed
The viola list board and table rows (columns, NAME cut, skip counts, strict-modes scan): the chunk changes no listing.
@task_item n=5 part=not_needed
viola revive and viola run outputs, start refusals and --json refusal codes: no product start path changes.
@task_item n=6 part=not_needed
Send readback, the send mirror, refusal hints for not-delivered sends, and viola wait outputs: no send or wait behaviour changes.
@task_item n=7 part=not_needed
viola link and unlink outputs, wrapper fault --json detail, peer reply screening and the exit-21 causes: none of these surfaces is reached.
@task_item n=8 part=not_needed
The self-validation protocol's squint, swap and readback-box signature checks: these are design reviews of displayed output, which this chunk adds none of.
