@task_prose part=session
The Epoch 4 audit's ranked defects mended; no capability claimed, no row of coverage flipped, nothing retired, no dependency moved.
The send: a session-start the confirmation claimed whose snapshot write then fails now settles the slot empty, so the send ends at its existing deadline as not-delivered / no-prompt-submitted instead of hanging.
The claude agents --json read now has its own kill bound, AGENTS_READ_DEADLINE, still 5 s (the version probe's value, kept, never measured against the real CLI); the version probe keeps VERSION_DEADLINE. The stalled-host false red of verify's agents read stays a carry, not mended.
The harness abort wait after a failed readiness (ABORT_DEADLINE) moved 20 s to 15 s, below the 20 s ci kill of the harness lifecycle binary; the 20 s readiness bound is unchanged.
The nextest verify_window_ override of both the ci and the mutants profile now also selects the two fake-agent dialog and framing replay tests by name; no kill value moved.
Mutation survivors of the audit got killing tests (env-name fault, stamps v1 shape, dot-led screen row, pump exit drain, enqueue wait, end-writer wait), each shown red by hand in a 10-control remove-the-guard run; two equivalent survivors named and argued.
@task_file n=1 path=src/run/send/slot.rs
what: the send confirmation slot of the wrapper — the confirmation window that claims a session-start, writes the snapshot and settles the send (confirmed, not-delivered, no-prompt-submitted); edited
@task_file n=2 path=src/run/agents.rs
what: the claude agents --json read (the agents listing used by list, revive and verify), its bounded capped read and its kill deadline; edited
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit cases of the ledger checks, here the env-names fault check of the recorded Claude fixtures (the user word as a name part); edited, tests added
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit cases of the verify stamps (the attempted stamp document refusing a v1 document of a failing shape); edited, tests added
@task_file n=5 path=crates/viola-agent-claude/src/screen.rs
what: the compiled screen-signature read of the child screen (option-shaped rows, a dot-led row read unnumbered); edited, tests only
@task_file n=6 path=crates/viola-pty/src/pump.rs
what: the PTY pump — output drain after the child exit, the input enqueue wait for room under its deadline, the end of the input writer within its wait; edited, tests and test doubles only
@task_file n=7 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: the harness lifecycle E2E binary (boot, status, cleanup; the stuck-boot supervisor case), its temp binaries now hard-linked into a temp dir beside the build directory instead of copied into /tmp; edited
@task_file n=8 path=crates/viola-e2e/src/harness/boot.rs
what: the harness boot command — readiness wait and the abort wait after a failed readiness (ABORT_DEADLINE 15 s), the readiness bound INSTANCE_DEADLINE 20 s; edited
@task_file n=9 path=.config/nextest.toml
what: the nextest profiles (ci and mutants) and their per-class kill overrides, the verify_window_ class filter; edited
@task_file n=10 path=.claude/rules/testing.md
what: the project's testing rule, its frontmatter path scope; edited
@task_file n=11 path=tests/contract_bounded_readers.rs
what: the bounded-readers contract check that every external reader passes a named kill bound to its capped read; edited (needle for the agents read)
@task_file n=12 path=tests/contract_lints.rs
what: the lint contract tests pinning the nextest verify_window_ override filter line; edited
@task_item n=1 part=touches
decision/verify-window-80s-kill reads stale: the verify_window_ class in both the ci and mutants profiles now also holds the two fake_agent dialog and framing replay_matches_every_recorded tests by name, each running a whole viola verify per recorded set; the 80 s kill value is unmoved.
@task_item n=2 part=touches
decision/boot-deadline-not-a-budget and the harness boot rows (contract/harness-boot/timeout, contract/harness-boot-reason/readiness-timeout, contract/harness-boot/readiness-failure): the abort wait after a failed readiness moved 20 s to 15 s; the 20 s readiness deadline is unchanged — check which bound each row's value names.
@task_item n=3 part=touches
decision/ci-stamped-home-kill-line, decision/root-wait-bound and decision/viola-e2e-30s-mutants-kill: the harness abort wait now sits below the 20 s ci kill of the harness lifecycle binary (2 s readiness plus 15 s, 17 s at worst).
@task_item n=4 part=touches
decision/peer-admitted-survivor-known-limit: two equivalent mutation survivors of the Epoch 4 audit are now named and argued as unkillable (one in the ledger, one in the screen read); read whether the row names them.
@task_item n=5 part=touches
decision/bounded-readers and decision/bounded-readers-keep-the-cap: the agents listing read is killed at its own named bound AGENTS_READ_DEADLINE, 5 s, no longer the version probe's VERSION_DEADLINE; the bounded-readers contract check's needle moved with it.
@task_item n=6 part=touches
decision/agents-json-fallback-unknown and decision/agents-parser-fails-to-unknown: the agents read killed at its bound still reads unknown; failure semantics unchanged, the bound now its own constant (indirect, through the agents read).
@task_item n=7 part=touches
decision/send-confirmation-window-expiry, decision/clear-confirmed-by-new-session and decision/compact-confirmed-by-same-session: a claimed session-start whose snapshot write fails now settles the send empty and the confirmation deadline closes it unconfirmed.
@task_item n=8 part=touches
contract/viola-send/no-prompt-submitted, contract/cli-send-json/no-prompt-submitted, contract/viola-send/clear-confirmed and contract/viola-send/compact-confirmed: the failed snapshot write on a claimed /clear or /compact line now answers not-delivered / no-prompt-submitted.
@task_item n=9 part=touches
decision/snapshot-read-through-replay, decision/single-writer-state-files and decision/pending-dialog-before-its-line: the snapshot write's failure path on the send's claim was changed (indirect, a rule over the snapshot writer the send calls).
@task_item n=10 part=touches
decision/mutation-gate-at-epoch-boundary, decision/mutation-run-verdict and decision/mutants-survived-count: the Epoch 4 boundary's killable survivors now each have a killing case; the survivor count those rows read moves.
@task_item n=11 part=touches
decision/bounded-remove-the-guard-witness: ten remove-the-guard controls, each case shown red with its mutant applied by hand (a tenth beyond the plan's nine).
@task_item n=12 part=touches
decision/forced-window-hook-is-test-parameter and decision/no-sleep-synchronization: the pump exit-drain case forces its window with a test-only hold released on the writer's drop or after a fifth of the drain bound; a read-bound case spawns a sleeping program to outlive the bound.
@task_item n=13 part=touches
decision/pty-exit-from-wait-not-eof, decision/child-exit-read-on-the-process-handle and decision/outer-pty-reader-drained: new unit cases assert the pump drains the output after the exit is seen and the input writer ends within its wait.
@task_item n=14 part=touches
decision/tempfile-test-dirs and decision/trust-start-folders-outside-repository: the stuck-boot supervisor case makes its temp dir beside the build directory and hard-links its binaries rather than copying into /tmp.
@task_item n=15 part=touches
decision/no-untied-harness-boot and decision/supervisor-spawned-only-by-boot: the stuck-boot case of the harness lifecycle binary changed its setup (indirect).
@task_item n=16 part=touches
decision/test-layer-filtersets, decision/mutants-kill-below-cargo-mutants-timeout, decision/verify-driven-tests-20s-kill and decision/send-window-paste-hint-30s-kill: a nextest kill override's filter widened; their kill values unmoved — read only whether a row lists the class members.
@task_item n=17 part=touches
decision/stamps-refuse-widened-ledger, decision/stamps-written-only-by-verify and decision/version-gate-needs-all-relied-rows: a new case refuses a v1 attempted stamp document of a failing shape; the relied-row count stays 29.
@task_item n=18 part=touches
decision/screen-signatures-only and decision/screen-fixture-box-rows-as-literals: a new case reads a dot-led option row unnumbered.
@task_item n=19 part=touches
contract/claude-fixtures/env-names: new cases read the user word as an env name part, a user word inside a part as no part, and no user word as nothing.
@task_item n=20 part=touches
decision/time-as-argument and decision/within-7s-wait-bound: the pump enqueue now has a case that waits for room until its deadline (indirect, over the pump's waits).
@task_fact n=1 slug=6-e2e-test-strategy
fact: The claude agents --json read is killed at its own bound AGENTS_READ_DEADLINE, 5 s, separate from the version probe's VERSION_DEADLINE; the value is the version probe's, kept, not measured against the real CLI's agents read.
@task_fact n=2 slug=unit-test-strategy
fact: A session-start the send confirmation claimed whose snapshot write fails settles the send empty; the confirmation deadline then ends it not-delivered / no-prompt-submitted, never confirmed and never hung.
@task_fact n=3 slug=5-command-implementation
fact: The harness abort wait after a failed readiness is 15 s, so a stuck boot ends within 17 s, below the 20 s ci kill of the harness lifecycle binary; the readiness bound stays 20 s.
@task_item n=1 part=untouched
CI workflow jobs, artifacts, SHA pins, permissions, caches and the supply-chain gates: no workflow, action or deny configuration changed; the advisory ignore stands, re-read with no safe upgrade.
@task_item n=2 part=untouched
Coverage floors, thresholds and exclusions, and the performance budgets and hyperfine gate: no threshold, floor or perf budget moved.
@task_item n=3 part=untouched
MCP server, MCP tools and their invalid-params, refusal and reply-screen contracts: no MCP surface changed.
@task_item n=4 part=untouched
Wrapper channel JSON-RPC errors, link and unlink methods, and instance-unreachable causes: no channel method or error body changed.
@task_item n=5 part=untouched
run and revive start refusals, exit codes and preflight order, strict-modes and socket-directory checks: no start or refusal path changed.
@task_item n=6 part=untouched
Hook exec entry points, hook deadlines, decision bodies and dialog answers: no hook or dialog path changed.
@task_item n=7 part=untouched
Process log line format, log event corr and the status document shape: no log field, event or status field added or changed.
@task_item n=8 part=untouched
Fuzz targets, corpora and nightly fuzzing, and the fake-agent command line and receipts: none changed.
@task_item n=9 part=untouched
viola verify output, exit details and stamped lines: verify's own outputs unchanged; only the bound of the agents read it calls has a new name, its value the same.
