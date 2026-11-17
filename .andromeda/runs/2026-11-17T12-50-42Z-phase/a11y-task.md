@task_prose part=chunk
Turns the red supply-chain CI gate green again by admitting one advisory ignore (RUSTSEC-2026-0543, chrono unmaintained, no safe upgrade, brought in by upstream crates) with its reason and its re-read at each epoch boundary, under the founder's widened advisory rule.
Re-words the config comment above the advisories ignore list so it states both admission grounds; admits no other advisory and leaves Viola's own chrono use as it is.
Re-measures every config and job that reads advisories (root graph, fuzz lock audit, weekly nightly advisories run, per-sync-crate runs) for which of them read this id and which fail on it.
Makes the in-test boots of the end-to-end crate's harness-lifecycle and cli test files tied, so their supervisors end with their test even when the test is SIGKILLed.
Adds a test that kills the tied stamping viola verify and reads that nothing of it is left, closing the narrowed v2-37 ledger note.
@task_file n=1 path=deny.toml mark=edits
what: the cargo-deny supply-chain config the CI supply-chain job reads for advisories — edits: the advisories ignore list (one entry for RUSTSEC-2026-0543), the comment above the ignore list
@task_file n=2 path=crates/viola-e2e/tests/harness_lifecycle.rs mark=edits
what: the end-to-end harness lifecycle test suite and its in-test boot options — edits: the options boot (BootOptions built untied, made tied)
@task_file n=3 path=crates/viola-e2e/tests/cli.rs mark=edits
what: the end-to-end CLI test suite and its in-test booted helper — edits: the booted boot (BootOptions built untied, made tied)
@task_file n=4 path=fuzz/Cargo.toml mark=re-reads
what: the fuzz graph's advisories and sources audit (cargo deny check advisories sources over the fuzz lock), re-measured for whether it reads this advisory id
@task_file n=5 path=.github/workflows/nightly.yml mark=re-reads
what: the weekly nightly advisories run, re-measured for whether it reads and fails on this advisory id
@task_file n=6 path=deny-sync.toml mark=re-reads
what: the per-sync-crate cargo-deny runs' config, re-measured for whether they read and fail on this advisory id
@task_item n=1 part=needs
The supply-chain CI job, one job of the GitHub Actions push/PR workflow, must read green on the chunk's own pushed sha with the whole run green; a red job fails the build.
@task_item n=2 part=needs
The CI test gate stays whole: no test skipped, ignored or filtered out to make the run green, and no skip or allow entry added by the way.
@task_item n=3 part=needs
The in-test boots of the end-to-end harness-lifecycle and cli suites become tied, so a booted wrapper and its fake agent end with their test, killed or not.
@task_item n=4 part=needs
A new test kills the tied stamping viola verify and proves nothing of it is left running; it kills only a process it owns and leaves nothing running after it.
@task_item n=5 part=needs
Every test that waits on a channel or a thread takes a time bound under the project's 20 s lint; time bounds fall on the driver and the test, never on a human.
@task_item n=6 part=needs
Tests must be deterministic: zero flakiness, no retries, every acceptance predicate one that can fail, and every test filter checked to select at least one test.
@task_item n=7 part=needs
Failures of the new and changed tests surface as ordinary nextest failures inside the existing test suites, with no separate driver, suite value or artifact of their own.
@task_item n=8 part=needs
The killed viola verify is a kill, not an interrupt: the test reads the process census, not the interrupted-verify stderr pair or exit code, and asserts no output of the killed process.
@task_item n=9 part=needs
No live claude start in any test: boots use the fake agent, and the test socket goes under the short /tmp root.
@task_item n=10 part=needs
The gate's coverage and doctest requirements still hold over the changed and new test code.
@task_standing n=1
convention: each new test lives inside an existing test suite and runs in the existing nextest run, its failure a nextest failure
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=2
convention: each new test is deterministic with no retries, and a gate breach fails the build
home: 10-slo-invariants-a11y-budgets
@task_standing n=3
convention: each new test's time bound falls on the driver, never on a human
home: cognitive-accessibility
@task_item n=1 part=not_needed
TUI passthrough contracts (entry, human keystrokes, focus/mouse/resize sequences, vendor content, viola terminal bytes, vt100 feed panic, refused peer): the chunk adds no terminal bytes and changes no passthrough.
@task_item n=2 part=not_needed
The viola list board and its caption, colour, SGR, dim, bold, amber and NAME truncation rules: no board or colour output changes.
@task_item n=3 part=not_needed
Refusal and fault copy contracts (unable/hint lines, code words, hints, --json refusal and failure documents, revive refusals, exit codes 10 to 21, instance-unreachable details): no refusal or fault text is added or changed.
@task_item n=4 part=not_needed
viola --help, link/unlink, plugin install, wait, send and revive --list output contracts: no CLI verb's output changes.
@task_item n=5 part=not_needed
Screen-reader targets and setups, and the manual Orca and keyboard passes: nothing user-facing changes, and no manual pass is owed.
@task_item n=6 part=not_needed
Absences (internationalization, touch surface, web or graphical frontend, rendered page, user time limit) and the WCAG, compliance and tier decisions: the chunk introduces none of these.
@task_item n=7 part=not_needed
The structured violation JSON rows and the a11y tool-pick decisions: the chunk adds no a11y assertion or tool.
