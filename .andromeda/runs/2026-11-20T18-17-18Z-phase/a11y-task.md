@task_prose part=chunk
Turns two behaviours of the claude CLI the product relies on with no ledger row into relied capability-ledger rows, each with a viola verify probe.
The harness prompt prefixes: the compiled raw-prefix set that files a prompt as a harness turn that never moves the wheel, probed by a second session sending into a verify child and reading its prompt's raw start.
The R8 identity floor: the eleven CLAUDE* parent-identity environment names always stripped from the child, probed by capturing env names, never values.
The new rows are stamped live, counted in a viola verify stamp from a live run in the rig's scratch home only; the live home's stamps and config.json and the pinned product stay untouched.
Re-reads the facts the tree can show: the relied-row count, the stamped fixture set and the default-version constants.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the capability ledger of the claude agent: its relied rows, their verify probes and the stamp's relied-id set — edits: the relied-row list, the stamp's relied-id set
@task_file n=2 path=crates/viola-agent-claude/src/ledger/test_cases.rs mark=edits
what: the capability ledger's tests: its rows, runs, checks and stamps
@task_file n=3 path=crates/viola-agent-claude/src/hook.rs mark=edits
what: the hook handling of the claude agent, home of the harness-prefix literals by which a prompt is filed as a harness turn, not a human one — edits: the harness-prefix literals
@task_file n=4 path=crates/viola-agent-claude/src/lib.rs mark=edits
what: the claude agent crate's root, home of the identity floor of parent-identity environment names stripped from the child's environment — edits: IDENTITY_FLOOR
@task_file n=5 path=fixtures/claude/2.1.328
what: the recorded fixture set of one claude CLI version that viola verify --record writes and the fake agent replays in CI
@task_item n=1 part=needs
Two new relied rows in the capability ledger, harness prompt prefixes and the R8 identity floor, each with a viola verify probe: every dependence on an undocumented claude behaviour is a ledger row with a verify probe.
@task_item n=2 part=needs
The harness-prefix probe: a second session sends into a verify child and the prompt's raw start is read to tell the harness-origin prefix; a harness prompt is never a human turn and never moves the wheel.
@task_item n=3 part=needs
The identity-floor probe: environment names, never values, are captured; no value may reach any file, fixture, stamp, log line or screen, and a failing test proves a captured value is refused.
@task_item n=4 part=needs
The hidden hook --capture arm of viola verify and its closed set of capture ids, which the new probes may extend.
@task_item n=5 part=needs
viola verify --record and its scrub, including the handling of a captured environment name that itself carries private text.
@task_item n=6 part=needs
The viola verify output: each new probe's step line and the verified count it adds, printed as plain linear output with no spinner and no SGR off a terminal.
@task_item n=7 part=needs
An interrupted viola verify run still keeps its exit and its stderr pair with the new probes in its child set.
@task_item n=8 part=needs
The verify stamp's relied-id set and count, stamped from a live run in the rig's scratch home; live readings never gate CI.
@task_item n=9 part=needs
The fake agent replays the new probes from the recorded fixture set in CI, so the CI gate holds them without a live claude.
@task_item n=10 part=needs
Indirect, through the probe reading the child's prompt: the boundary check never inspects claude content beyond its signatures; only the raw prefix is read.
@task_item n=11 part=needs
Indirect, through anything the probes print: C0 and C1 control characters in a captured name or prompt start are printed escaped, never raw.
@task_item n=12 part=needs
Indirect, through the probe's run diagnostics and log lines: diagnostics never reach the terminal, and log identity tags stay as they are; no environment value is logged.
@task_item n=13 part=needs
Indirect, through any refusal or failure the new probes report: the refusal or failure word comes with its reason code and hint, never a stack trace or bare status, and is never coloured.
@task_item n=14 part=needs
Indirect, through the probe's wait on the verify child: time bounds fall on the driver, never on the human.
@task_standing n=1
convention: each new check runs inside the existing test suites, with no separate a11y driver, suite or artifact
home: 1-a11y-scope-summary · a11y-scope-summary
@task_standing n=2
convention: each new test fails as a nextest failure in the CI gate, which allows no skips
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=3
convention: each new test is free of flakiness, with no retries
home: 10-slo-invariants-a11y-budgets
@task_standing n=4
convention: each new fixed CLI string is pinned by its literal in a test
home: 8-cognitive-accessibility · cognitive-accessibility
@task_standing n=5
convention: the CI gate's coverage and doctest requirements hold for each new test
home: ci-integration
@task_item n=1 part=not_needed
The viola list board rows, its caption, colour, stale-row dimming and NAME truncation: the chunk changes no board output.
@task_item n=2 part=not_needed
The viola revive rows, its --json refusals and its --list output: revive is not touched.
@task_item n=3 part=not_needed
The viola link, unlink and plugin install rows: neither verb is touched.
@task_item n=4 part=not_needed
The viola --help rows, its usage line, group labels and handoff row: no verb is added or relabelled.
@task_item n=5 part=not_needed
The screen-reader target and setup rows and the manual Orca and keyboard passes: the chunk adds no surface those passes read.
@task_item n=6 part=not_needed
The structured violation JSON rows: no a11y report shape changes.
@task_item n=7 part=not_needed
The absences of a web or graphical frontend, a touch surface and internationalization: nothing here adds one.
