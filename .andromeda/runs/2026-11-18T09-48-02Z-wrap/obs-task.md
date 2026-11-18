@task_prose part=session
The relied trust-record ledger row was read live on claude 2.1.325 in the standing live rig (start S0B) and passed 23/23, so the row is now verified on 2.1.325 under the rig's private config only; nothing is claimed for 2.1.326 or 2.1.327.
The cause that stopped verify's interactive runs B to F short of the input box was measured: 2.1.325's browser-tools prompt (ChromeAutoEnableDialog), not the first-run config and not trust.
viola verify now reads and stamps each interactive run's first screen: the stamp's measured gains closed first-screen words, whole-number settle walls and closed row facts (literal ids, rule above prompt, first-row family, last column), with no screen text.
A hidden rig-only verify option (--record-first-screens) writes, on a failed record verify only, one local first-screen file of whole scrubbed rows per run that missed the box; an unclean set is refused whole; a plain record and a passing verify write no such file; it is absent from help.
The rig's seed moved from six used-config markers to five bundle keys (including claudeInChromeDefaultEnabled false), the rig gained the VC arm, S0B labels and new summary lines, and its version probe now runs in the scratch root so no coverage profile lands in the tree.
The verify-window nextest class kill moved 60 s to 80 s in the ci and mutants profiles; viola run and the browser-tools prompt as a modal are unchanged and become a new route entry.
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: edited — the screen read against compiled literals (the signatures): adds the first-screen classifier (trust, imports, box, none, exited) and the closed row facts of a first frame
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: edited — the capability ledger and its stamps: the typed runs' first readings and the stamp's measured first-screen words, settle walls and facts
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: edited — unit cases of the capability ledger
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: edited — unit cases of the ledger stamps' merge, now over the six first screens, settles and facts
@task_file n=5 path=src/cmd/verify.rs
what: edited — the viola verify command, its record of fixtures with scrub and whole-set refusal, and the hidden rig-only failed-verify first-screen record
@task_file n=6 path=src/cmd/verify/typed.rs
what: edited — verify's typed interactive runs A to F over the PTY
@task_file n=7 path=src/cmd/verify/typed/run.rs
what: edited — one typed verify run: keeps its first settled screen and reading at each end (kill, ctrl-c, finish)
@task_file n=8 path=src/cmd/verify/typed/runs.rs
what: edited — the dialog, plan and resume typed runs of verify and how each run's first reading is kept
@task_file n=9 path=tests/cli_verify.rs
what: edited — integration tests of viola verify against the fake agent, including the record and first-screen cases
@task_file n=10 path=tests/cli_verify/runs.rs
what: edited — integration tests of verify's typed runs and their first-screen words over the fake agent
@task_file n=11 path=tests/support/verify.rs
what: edited — test support that starts viola verify in a test home
@task_file n=12 path=crates/viola-e2e/src/rig/isolate.rs
what: edited — the standing live rig's isolated child scratch home: its git-root layout, its five-key config seed and the kept first screens
@task_file n=13 path=crates/viola-e2e/src/rig/mod.rs
what: edited — the standing live rig's arms (V, VC), labels and summary lines, and its version probe's working directory
@task_file n=14 path=crates/viola-e2e/src/rig/screen.rs
what: edited — the live rig's screen model, now sized by rows and columns
@task_file n=15 path=crates/viola-e2e/src/rig/ledger.rs
what: edited — the live rig's start ledger and its caps of spare starts
@task_file n=16 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: edited — the live rig binary and its live verb
@task_file n=17 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited — the live rig's rehearsals over the fake agent, including the no-credential and no-host-path record check
@task_file n=18 path=.config/nextest.toml
what: edited — the nextest profiles and their per-class kill timeouts for ci and mutants
@task_file n=19 path=.gitignore
what: edited — the repository ignore list, now ignoring the recorded first-screen fixture files
@task_item n=1 part=touches
The rows that verify lines carry no content and that no telemetry comes from screen content: verify now classifies each run's first screen and stamps closed words and closed facts from it; the stamp is no log line, but a picker must check these rows still hold and say so.
@task_item n=2 part=touches
The rows that recorded Claude fixtures are scrub-required and that fixtures are no telemetry source: a new rig-only first-screen file of whole scrubbed rows is recorded beside the fixtures, scrubbed and refused whole when unclean, and kept local.
@task_item n=3 part=touches
The rows that the capability ledger path and stamps are never logged and that verify is the only stamps writer: the stamp's measured gains first_screen, first_settle_ms and first_facts, still written by verify alone.
@task_item n=4 part=touches
The rows that CI verifies against the fake agent only and that real-CLI verify never runs in CI: the trust-record row was verified live on 2.1.325 in the rig, outside CI, and CI still ran on the fake.
@task_item n=5 part=touches
The rows that the PTY byte stream and typed keys are never logged: verify's typed runs now keep their first settled rows in memory and the rig prints first-screen words and facts; no key was typed on any live start.
@task_item n=6 part=touches
The rows on the trust-record write staying uninstrumented and an unreadable ledger counting as unverified: the trust-record row is now verified on 2.1.325, with its writer unchanged.
@task_item n=7 part=touches
The verify stdout step and summary line contracts and verify exit codes: the summary still reads the relied row count of 23; a picker should confirm no verify stdout or exit shape changed (only a hidden option was added, absent from help).
@task_item n=8 part=touches
The verify child spawn subject version-probe: the rig's own version probe now runs with the scratch root as its working directory, so an instrumented child's coverage profile no longer lands in the tree.
@task_item n=9 part=touches
The rows that test homes are kept until gates run and that test homes live under the scanned root, and the secret-scan scope: the rig rehearsal's no-credential and no-host-path record check read red on stray coverage profiles and was fixed by the probe's working directory.
@task_item n=10 part=touches
Any row whose cell holds the verify-window test class kill timeout: it moved from 60 s to 80 s in the ci and mutants nextest profiles.
@task_item n=11 part=touches
The row that wrapped claude internals are not a contract and that transcripts are not a contract: the 2.1.325 bundle's browser-tools prompt and its config key were read from the bundle and used only as the rig's seed.
@task_item n=12 part=touches
The rows on the closed event enum and closed detail words: the new first-screen words (trust, imports, box, none, exited) and row families (rule, banner, text) are closed kebab-case words in the stamp, not log events.
@task_fact n=1 slug=obs-scope-summary
fact: The rig-only hidden verify first-screen record (whole scrubbed rows of each run that missed the box, written only on a failed record verify, refused whole when unclean, kept local and git-ignored) is a fixture-side record, never a telemetry source and never a log line.
@task_fact n=2 slug=span-trace-coverage
fact: The stamp's measured first-screen words, settle walls and row facts are closed words, whole numbers and literal ids only, hold no screen text, and are never logged like the rest of the ledger stamps.
@task_item n=1 part=untouched
The logging stack, the file sinks, the diagnostics line schema, its fields, levels and corr: no log line, event, field or target was added or changed.
@task_item n=2 part=untouched
The hook, channel, MCP, send, wait, wheel, link and dialog paths and their contracts: viola run and the other verbs are unchanged.
@task_item n=3 part=untouched
Panic capture, error reporting, the detail file and the fixed stderr messages: untouched.
@task_item n=4 part=untouched
The CI workflow, its jobs, artifact uploads and gates: no agent-run command, status shape or CI step changed; CI ran green 7 of 7.
@task_item n=5 part=untouched
The perf and hook latency budgets, the SLO invariants, metrics derivation, heartbeat and liveness rows: no budget or threshold moved.
@task_item n=6 part=untouched
The OTel, exporter, error-reporter and network-egress absences: nothing was added that exports or sends.
