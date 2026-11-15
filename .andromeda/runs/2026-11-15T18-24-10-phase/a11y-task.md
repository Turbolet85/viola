@task_prose part=chunk
Test-only corrective chunk: makes a green coverage suite and a green mutation baseline for viola reachable, measuring neither (no mutation run, no coverage run, no live claude start).
It gives the fail-open hook test a channel socket name that does not grow with the pid and event, and proves its length bound under the mutation scratch with a 7-digit pid.
It widens the nextest mutants profile's slow-case kill and the receipt wait together, the wait staying strictly below the kill and the kill below cargo-mutants' 20 s floor, re-measuring each slow case's wall time with the host load beside it.
It kills the ten surviving mutants of the hyprland recorder binary by tests that fail on each mutated site, or states in the test-plan why the recorder's main is outside the tier.
It enforces the record-home naming rule (no path component named home in a test home given to viola verify --record) in the rig's home-naming helper.
@task_file n=1 path=tests/hook_fail_open.rs mark=edits
what: the hook fail-open integration test, which binds a test IPC channel endpoint and drives viola hook event and dialog calls against it — edits: the test channel's socket name and a test of its length bound
@task_file n=2 path=.config/nextest.toml mark=edits
what: the nextest test-runner config, the CI test gate's profiles — edits: the mutants profile's slow-timeout kill, and any override for the slow-case class
@task_file n=3 path=tests/support/watch.rs mark=edits
what: the shared test-support receipt wait the integration tests sequence on records with — edits: the WITHIN bound and its doc comment, or a per-call bound for its wait_for callers
@task_file n=4 path=tests/support/home.rs mark=edits
what: the test-support home fixture every viola verify --record caller takes its test home from — edits: the home-naming helper (TestHome::new, the home fixture, scratch) to refuse a path component named home
@task_file n=5 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs mark=edits
what: the test-only hyprland recorder binary of the live end-to-end rig — edits: its tests for main's argument guard and record arms, version_of and texts; main only if a test proves a recorder line wrong
@task_item n=1 part=needs
The test suites' determinism: the chunk exists to remove flaky reds (a receipt wait timing out in the coverage suite, slow cases killed in the mutation baseline) — zero flakiness with no retries is what its margins must deliver, not a retry or a skip.
@task_item n=2 part=needs
The mutation audit over invariants held in crate and integration tests: the chunk makes the mutation baseline reachable and kills recorder survivors, so the mutation-audit decision binds what each new killing test must prove (fails on the mutated site, proven by a remove-the-guard control whose case is the test's name and which ran and failed).
@task_item n=3 part=needs
The coverage suite the a11y cases run inside: the receipt-wait change must leave that suite green, and the gate that requires coverage and doctest is the one the next audit reads.
@task_item n=4 part=needs
Time bounds fall on the driver only: the receipt wait and the slow-case kill are test-harness bounds on the driver and the rig, never a time limit on the human; widening them must keep that.
@task_item n=5 part=needs
Boundary tests sequenced on records: the receipt wait is the record-sequenced wait the boundary tests rely on; its bound may be raised or made per-call but tests still wait on the record, not on sleeps or screen content.
@task_item n=6 part=needs
The hook call path the fail-open test exercises over the test channel endpoint: viola hook event and dialog calls and their socket-directory, strict-modes and server-verify refusals keep their asserted behaviour; only the test endpoint's name changes.
@task_item n=7 part=needs
The CI test gate allows no skips and a gate breach fails the build: the new tests (socket bound, recorder kills, home-name refusal) run in the existing nextest suites, are not ignored, and fail as ordinary nextest failures.
@task_item n=8 part=needs
Live readings never gate and no live desktop step: the recorder's survivors are killed by unit tests of the test-only binary, not by a live hyprland or live claude run; the standing live rig is out.
@task_item n=9 part=needs
No product behaviour changes: every product crate line is out unless a test proves one wrong, and any red or defect found folds into this chunk as an inferred item with its own acceptance.
@task_standing n=1
convention: each new test is an ordinary nextest case inside the existing test suites, its failure a nextest failure, with no separate a11y driver or suite
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=2
convention: each new test and each changed bound is deterministic — zero flakiness, no retries, no skips
home: 10-slo-invariants-a11y-budgets
@task_item n=1 part=not_needed
Visual design and colour rows (SGR on stale rows, bold names, amber dialog word, name truncation, colour decision inputs): the chunk changes no rendered output.
@task_item n=2 part=not_needed
The viola list board, caption, unwrapped separator and revive --list contracts: no board or list output is touched.
@task_item n=3 part=not_needed
viola revive --json refusals, start refusals, link and unlink, plugin install and --help contracts: no CLI verb's output, exit code or refusal document changes.
@task_item n=4 part=not_needed
Screen-reader targets and setups, the manual keyboard and Orca passes: no manual pass and no live desktop step is in this chunk.
@task_item n=5 part=not_needed
TUI passthrough contracts (viola terminal bytes, human keystrokes, focus/mouse/resize sequences, vt100 feed panic): the PTY passthrough is product code the chunk does not touch.
@task_item n=6 part=not_needed
Absences (internationalization, touch surface, web or graphical frontend, rendered page) and the N/A web harness areas (contrast, focus management, WCAG mapping): nothing the chunk adds is a frontend.
@task_item n=7 part=not_needed
Cognitive-accessibility copy rules (fixed CLI strings from design, one hint per code, reason codes beside unable, error word-detail-hint): no user-facing string is written or changed.
