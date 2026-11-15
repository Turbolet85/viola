@task_prose part=chunk
A test-only corrective chunk that makes a green coverage suite and a green viola mutation baseline under the mutation scratch reachable, measuring neither.
It gives the hook fail-open test's channel a socket name that does not grow with the pid and the event label, and proves its length bound under the scratch with a 7-digit pid.
It widens the nextest mutants profile's slow-case kill and moves the shared receipt wait bound with it, re-measuring the slow cases' wall time against the new margins with the host's load beside it.
It kills the ten surviving mutants of the test-only recorder binary viola-hypr-tap by tests that fail on each mutated site, or states in the test plan why the recorder's main is outside the tier.
It enforces the record-home naming rule: a test home given to viola verify --record never holds a path component named home, through the rig's shared home-naming helper.
@task_file n=1 path=tests/hook_fail_open.rs mark=edits
what: the hook fail-open integration test binary, which binds a test channel endpoint (a Unix socket under a tempdir) for viola hook to reach — edits: the test channel endpoint's socket name and the bind it feeds
@task_file n=2 path=tests/support/watch.rs mark=edits
what: the shared test wait helper whose WITHIN bound every receipt and event wait of the root tests waits under — edits: the WITHIN constant and its doc comment tying it below the mutants profile's kill
@task_file n=3 path=tests/support/home.rs mark=edits
what: the shared per-test home helper (TestHome::new, the home() rstest fixture, scratch()) every test home and every viola verify --record caller takes its home from — edits: the home-naming helper
@task_file n=4 path=.config/nextest.toml mark=edits
what: the nextest runner configuration holding the test profiles and their per-test kill lines — edits: the mutants profile's slow-timeout kill and its overrides
@task_file n=5 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs mark=edits
what: the test-only recorder binary in the E2E crate (its main argument dispatch, version_of and texts) — edits: its tests, and its main only if a test proves a recorder line wrong
@task_file n=6 path=tests/cli_verify/runs.rs mark=re-reads
what: the viola verify --record cases that take their test home from the shared home helper
@task_file n=7 path=tests/cli_verify/stamps.rs mark=re-reads
what: the viola verify --record stamp cases that take their test home from the shared home helper
@task_item n=1 part=needs
The test channel endpoint's socket path in the hook fail-open test: a name that does not grow with the pid or the event label, with a computed or asserted bound under sun_path's 107 bytes for the mutation scratch path and a 7-digit pid, recomputed at HEAD from the scratch path and the longest label the cases carry.
@task_item n=2 part=needs
The test endpoint's directory stays a private test-made directory after the rename; where the tempdir is placed (tempfile's use of TMPDIR) is measured at P3 and cited, not assumed.
@task_item n=3 part=needs
The mutants profile's per-test kill line for the ~5 s class (the tampered pinned copy refusal, the plugin folder rewrite on each start, revive after a killed wrapper, start over an unreadable snapshot replaying the log): a margin sized by this chunk that stays below cargo-mutants' 20 s auto-timeout floor.
@task_item n=4 part=needs
How nextest reads the mutants profile and its overrides (whether an override setting only slow-timeout keeps the profile's other keys, the first matching override winning, the slow-timeout period and terminate-after form) is measured at P3 and cited; the existing per-class kill lines (stamped-home, verify-driven, send-window, verify-window, harness package) keep their order against the new kill.
@task_item n=5 part=needs
The WITHIN wait bound moves with the kill: every root wait stays strictly below the mutants profile's kill; whether a per-call bound or a scaled shared bound is the narrower change is read across every wait_for caller, and the coverage-suite red on the verify stopped during the resume run case is the case it must clear.
@task_item n=6 part=needs
Each slow case's wall time is measured against the new margins with the host's load recorded beside it; no test retry and no sleep is added to make a case fit.
@task_item n=7 part=needs
The ten viola-hypr-tap survivors (main replaced by a default, the --program and --out match guard forced true or false, its two == flipped and its && turned ||, the deleted record and record-dispatch arms in main, the deleted version reply arm in version_of, the deleted array arm in texts): each killed by a test that fails on its mutated site.
@task_item n=8 part=needs
Where the recorder's main is ruled outside the tier instead, the test plan's tier text states it with its reason, as a card item.
@task_item n=9 part=needs
Each survivor witness is proven by a one-off remove-the-guard control run through the gate control under a kill deadline, the control's case being the test's name alone, and it holds only when that case ran and failed.
@task_item n=10 part=needs
The record-home naming rule: a test home handed to viola verify --record never holds a path component named home, enforced inside the shared per-test home helper so no test can name it wrong; P3 closes which helper the rule binds, and an absence probe for it is checked against the plan's own test names and fixture keys.
@task_item n=11 part=needs
Test homes stay per-test, fresh and under the one checked test-home base after the naming rule lands; the helper's existing callers keep taking their home from it.
@task_item n=12 part=needs
Test-only bound: no product crate line changes unless a test proves one wrong; a red or a defect found now folds into this chunk as an inferred item with its own acceptance, one found outside the pinned scope is a carry-forward naming its owner entry.
@task_item n=13 part=needs
No mutation run, local or dispatched, and no coverage run as the chunk's acceptance; no live claude start and no live desktop step; no capability claimed and no requirement added.
@task_item n=14 part=needs
The test-only recorder binary stays test-only: the E2E crate is never in a release build, and any change to its main stays inside the recorder's own job.
@task_standing n=1
convention: every new test function is named by the plan's test naming rule and is reachable by the layer filtersets through its binary's surface prefix
home: 2-test-strategy · test-strategy
@task_standing n=2
convention: every new test synchronises on logged events or receipts under a bounded wait, never on a sleep, and is never retried or parked with #[ignore]
home: 11-test-anti-patterns-never-do-these
@task_standing n=3
convention: every new test runs in parallel with the rest, owns its own home and tempdirs, shares no mutable state and mutates no process-wide environment
home: 11-test-anti-patterns-never-do-these
@task_standing n=4
convention: every new test helper exists once in the shared support module and is never copied into a test binary
home: test-strategy
@task_standing n=5
convention: every new test asserts against literal oracles on the observable behaviour, never on Debug strings or fixture internals
home: 11-test-anti-patterns-never-do-these
@task_standing n=6
convention: every new test or config change keeps the zero-flakiness budget: a failure turns the suite red and fails the build
home: 10-quality-gates-coverage-targets
@task_item n=1 part=not_needed
The MCP tool, CLI verb, exit code, JSON document and human output contracts: the chunk changes no product surface and asserts none anew.
@task_item n=2 part=not_needed
The send, wait, answer, release, revive and run-start refusal contracts and their refusal orders: no product behaviour is changed or added.
@task_item n=3 part=not_needed
The harness session, boot, status, cleanup, supervisor and pid-record contracts: no harness command is touched.
@task_item n=4 part=not_needed
The mutation run verdict, run document and gate breach contracts: the chunk runs no mutation and no coverage, and the next boundary's audit reads them.
@task_item n=5 part=not_needed
The CI workflow, artifact, permission, action pinning and supply-chain rows: no workflow and no dependency is changed.
@task_item n=6 part=not_needed
Performance budgets and hyperfine rows: no hook timing or perf job is touched.
@task_item n=7 part=not_needed
Fuzzing, proptest seed and corpus rows: no parser surface or generator is added.
@task_item n=8 part=not_needed
Process log line, log event corr and diagnostics schema contracts: no log line is added or changed.
@task_item n=9 part=not_needed
Secret scan classes and pre-push detail contracts: neither tool is changed.
@task_item n=10 part=not_needed
The fake agent's command line, script, input and receipt contracts: the chunk changes how long a test waits, not what the fake agent does or writes.
@task_item n=11 part=not_needed
Windows, macOS, browser, database and outbound HTTP absences: the chunk adds nothing on those axes.
