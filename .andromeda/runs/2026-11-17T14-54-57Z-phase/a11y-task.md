@task_prose part=chunk
Takes the first live readings on the committed standing rig, each written down as closed words in the chunk's evidence: a fired scheduled wake's prompt-submit `source` field, turn-ended's scheduled_wakes reading 1 on a real wake, and the hook-latency window before a send's paste.
Takes the live `viola verify --record`, whose record home comes from a helper like the tests' record_home() and holds no path component named `home`; each reading names the claude binary it ran on.
Reads live the external-imports modal's shape and re-reads R2's unexplained `holds-text` input box; the rig is extended only as far as a reading needs, its isolation standing as built.
Fixes the CI nextest kill line: cli and harness_tied join the 20 s ci override, and harness_lifecycle's slow case either shortens its own boot readiness deadline or that binary takes a bound of its own, with the measured wall after the change.
@task_file n=1 path=crates/viola-e2e/src/bin/viola-live-rig.rs mark=edits
what: the standing live rig binary that starts a real claude session in isolation and takes live readings — edits: its arms (r1/r2 today), extended only as far as a reading needs (a wake, latency, imports-modal or verify-record arm)
@task_file n=2 path=crates/viola-e2e/src/rig/mod.rs mark=edits
what: the live rig's library: isolation, keys, procs, screen kept locally, closed-word ledger — edits: only as far as a reading needs
@task_file n=3 path=.config/nextest.toml mark=edits
what: the nextest CI profile and its 20 s kill line for verify-driven test binaries — edits: the ci profile's second override's binary list, and a bound of its own for harness_lifecycle if that fork is taken
@task_file n=4 path=crates/viola-e2e/tests/harness_lifecycle.rs mark=edits
what: the harness lifecycle tests driving a stamping verify through boot — edits: the case boot_that_never_gets_ready_times_out_and_stops_its_supervisor, its readiness deadline, if that fork is taken
@task_file n=5 path=crates/viola-e2e/src/harness/boot.rs
what: the harness boot step and its 20 s readiness deadline, read for whether a case can shorten it without changing the product
@task_file n=6 path=crates/viola-e2e/tests/cli.rs mark=re-reads
what: the cli test binary driving a stamping verify through boot, its measured wall under the 20 s kill line
@task_file n=7 path=crates/viola-e2e/tests/harness_tied.rs mark=re-reads
what: the harness-tied test binary driving a stamping verify through boot, its measured wall under the 20 s kill line
@task_item n=1 part=needs
Live readings are evidence only: no live reading taken on the rig gates CI or the build.
@task_item n=2 part=needs
The rig's readings never inspect or record the agent's own content beyond the closed words a reading names; the screen is kept locally and only closed words reach a record, so a box-text or modal reading reads signatures, not screen content.
@task_item n=3 part=needs
The live `viola verify --record` and its output: the verify step lines it prints and the stamp it records, read live, with its record home chosen by a helper and holding no `home` path component.
@task_item n=4 part=needs
The human's terminal under a wrapped live start: any key typed into a CLI-native modal (the external-imports modal, the trust pass) is the founder's to grant, none granted; the rig types nothing into the session's terminal beyond what a reading needs.
@task_item n=5 part=needs
The hook-latency window bears on a send's paste before the wrapper has learned of a turn: the send-refusal states (no prompt submitted, input not ready) and the rule that a send is refused while the human types are what the reading's window is measured against; the reading changes neither.
@task_item n=6 part=needs
Time bounds fall on the driver, never on the human: the 20 s kill line, a bound of its own for harness_lifecycle and a shortened boot readiness deadline are driver-side bounds only.
@task_item n=7 part=needs
The CI kill line must never end a case to pass it: the harness_lifecycle case is not added to the 20 s line to be killed, the slow case's wall is re-measured after the change, and no case is skipped or retried.
@task_item n=8 part=needs
Run diagnostics and the rig's own log lines never reach the wrapped session's terminal; viola writes zero bytes of its own to the terminal under a live start.
@task_item n=9 part=needs
The scheduled wake's turn-ended event and its scheduled_wakes count read live as 1; the event line form it is read from is unchanged and no classification of a wake prompt is built.
@task_item n=10 part=needs
Each reading on PATH claude 2.1.326 and R4's stamping of it are counted apart from readings on the 2.1.325 install; the binary each reading ran on is named in its closed words.
@task_item n=11 part=needs
The isolation of a live start stands as built: a private config dir, a scratch root outside every repository, credentials copied under the token-window read and removed on every exit path.
@task_standing n=1
convention: every test or check the chunk adds or changes fails as a nextest failure in the existing suites
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=2
convention: no separate a11y driver, suite value or artifact of its own; a11y cases run inside the coverage suite in the push/PR workflow
home: 9-ci-integration
@task_standing n=3
convention: zero flakiness and no retries: a new or changed test passes deterministically, with no retry and no soft SLO
home: 10-slo-invariants-a11y-budgets
@task_standing n=4
convention: the CI test gate allows no skips; a changed nextest profile keeps that
home: 1-a11y-scope-summary
@task_standing n=5
convention: the gate requires coverage and doctest; a changed nextest profile keeps both
home: ci-integration
@task_item n=1 part=not_needed
The viola list board's colour, SGR, dim, bold, amber and NAME truncation rows: no reading or fix here prints or changes the board.
@task_item n=2 part=not_needed
The revive, link, unlink and plugin-install refusal and --json document rows: the chunk adds no refusal, exit code or failure document on those verbs.
@task_item n=3 part=not_needed
The --help group labels, usage line and served-verbs rows: no verb or help text changes.
@task_item n=4 part=not_needed
The manual Orca and keyboard passes and the screen-reader target setups: no manual a11y pass is taken in this chunk.
@task_item n=5 part=not_needed
The absences of a web or graphical frontend, a rendered page, touch, internationalization, and the WCAG mapping and contrast harness areas marked N/A.
