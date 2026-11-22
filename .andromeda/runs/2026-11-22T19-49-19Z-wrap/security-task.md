@task_prose part=session
The session mended the Epoch 4 audit's ranked defects and re-measured the dependency advisory; no capability added or retired, no dependency changed.
The send path: a session-start line the send claimed whose snapshot write then fails now settles the slot, so the send ends unconfirmed as not-delivered with no-prompt-submitted instead of hanging; claim, snapshot, append, settle order unchanged.
The claude agents read now has its own named kill bound AGENTS_READ_DEADLINE, still 5 s (the version probe's value, kept, never measured against the real CLI's agents read); the version probe keeps VERSION_DEADLINE. The stalled-host false red of verify's agents read is NOT mended and stays a carry.
Measured: RUSTSEC-2026-0543 re-read 2026-11-22 with no safe upgrade, its ignore stands; a SIGKILLed verify leaves one projects entry for its own temp folder holding hasTrustDialogAccepted true alone, under a test-owned config.
Tests added for surviving mutants (env-name user-word fault, stamp document shape, option-shaped screen row, PTY pump drain, enqueue wait, writer end wait); eight survivors killed, two named equivalent. The e2e harness abort wait after a failed readiness went 20 s to 15 s and its stuck-boot test hard-links its binaries into a temp dir beside the build directory instead of copying into the system temp root.
@task_file n=1 path=src/run/send/slot.rs
what: the wrapper's send slot and delivery confirmation: the hook event append that claims a session-start, writes the snapshot and settles the send
@task_file n=2 path=src/run/agents.rs
what: the claude agents read, bounded and failing open, run through the capped child-output reader with its own deadline
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: tests of the verify stamp's env-name probe check, the user word read as a name part (names and counts only)
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: tests of the stamps file reader refusing a version-1 stamp document of a failing shape
@task_file n=5 path=crates/viola-agent-claude/src/screen.rs
what: the screen model read against compiled literals (input box and option rows); tests only changed
@task_file n=6 path=crates/viola-pty/src/pump.rs
what: the PTY pump: bounded child output queue, drain after the child exits, the child input writer's end; tests only changed
@task_file n=7 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: the e2e harness lifecycle tests, the stuck-boot supervisor case and its test-created temp directory
@task_file n=8 path=crates/viola-e2e/src/harness/boot.rs
what: the e2e harness boot supervisor and its test-only readiness and abort deadlines
@task_file n=9 path=.config/nextest.toml
what: the test runner profiles (ci and mutants) and their per-class kill overrides, the verify-window class
@task_file n=10 path=.claude/rules/testing.md
what: the project's testing rule file; its frontmatter paths only
@task_file n=11 path=tests/contract_bounded_readers.rs
what: the bounded-readers census test: every product reader bounded or named, its needle for the agents read's deadline
@task_file n=12 path=tests/contract_lints.rs
what: the lint contract tests pinning the test runner's verify-window override filter
@task_item n=1 part=touches
decision/claude-agents-read-fail-open: the agents read is still killed at 5 s, but the bound is now its own named constant AGENTS_READ_DEADLINE, separate from the version probe's VERSION_DEADLINE, its value kept and not measured against the real CLI's agents read; the stalled-host false red (7 of 48 rehearsals failing agents-join under a neighbour's IO stall) is not mended by it.
@task_item n=2 part=touches
The bounded-readers rows (decision/bounded-readers-census, decision/bounded-external-readers, decision/child-output-bounded): the agents read's bound is now named by its own constant and the census test's needle for it re-pointed; no reader gained or lost a bound.
@task_item n=3 part=touches
decision/claude-agents-resolved-from-path and decision/agents-probe-log-content-free: indirectly, through the agents read whose deadline moved owner; its resolution, logging and fail-open behaviour unchanged.
@task_item n=4 part=touches
decision/advisory-ignore-list and decision/advisory-ignore-grounds: RUSTSEC-2026-0543 re-read 2026-11-22 at the epoch boundary, no safe upgrade (no patched version), the ignore stands on the unmaintained-upstream ground; the deny config unchanged.
@task_item n=5 part=touches
decision/nothing-written-under-claude-home and its siblings (decision/no-writes-under-claude-home, decision/viola-writes-nothing-in-claude-dir, decision/verify-trusted-runs-risk): the SIGKILL residual of verify measured 2026-11-22 under a test-owned config as one projects entry for verify's own temp folder holding hasTrustDialogAccepted true alone; read whether a row says less.
@task_item n=6 part=touches
decision/delivery-confirmation-trailing-tolerance, decision/send-text-is-one-bracketed-paste and decision/bridge-send-one-bracketed-paste: indirectly, through the send slot: a claimed session-start whose snapshot write fails now ends the send not-delivered with no-prompt-submitted through the existing confirm deadline; no new refusal word and no new time bound.
@task_item n=7 part=touches
decision/snapshot-written-only-by-wrapper and decision/replayed-snapshot-inert: indirectly, through the wrapper's snapshot write whose failure now settles the claimed send; the writer and what it writes unchanged.
@task_item n=8 part=touches
decision/env-probe-names-and-counts-only, decision/env-fixture-names-only and contract/fixture-refusal-codes/username: the env-name fault check that reads the user word as a name part gained tests killing its surviving mutants; behaviour unchanged.
@task_item n=9 part=touches
decision/stamps-checked-under-lock, decision/stamps-strict-check-first and decision/version-gate-strict-stamps-read: the stamps reader's refusal of a version-1 document of a failing shape gained a test; behaviour unchanged.
@task_item n=10 part=touches
decision/verify-screen-read-against-literals and decision/input-box-readiness-from-literals: the screen model's option-shaped row read (a dot-led row read unnumbered) gained a test; behaviour unchanged.
@task_item n=11 part=touches
decision/vt100-feed-bounded-queue and decision/child-output-tolerant: the PTY pump's drain after exit, its enqueue wait for room until its deadline and the writer end's bounded wait gained tests; behaviour unchanged.
@task_item n=12 part=touches
decision/mutation-testing-at-audit: the Epoch 4 audit's survivors acted on: eight killable survivors each shown red by hand with a new test, two named equivalent and argued.
@task_item n=13 part=touches
decision/e2e-home-removal-own-dirs-only and decision/test-removal-own-dirs-only: the stuck-boot test now hard-links its three binaries into its own boot-stuck temp dir made in the build directory's parent instead of copying them into the system temp root.
@task_item n=14 part=touches
The e2e harness abort wait after a failed readiness (ABORT_DEADLINE) 20 s to 15 s, test-only; the 20 s readiness deadline unchanged; a row holding the abort wait's value names it.
@task_item n=15 part=touches
The test runner's verify-window kill class in both the ci and mutants profiles now also holds the two fake-agent dialog and framing drift replay tests by name; no kill value moved; a row holding the class membership names it.
@task_item n=1 part=untouched
Authentication and authorization: peer euid checks, server verification before any frame, socket directory and socket file modes, strict-modes checks and their refusal contracts; no check, order or refusal moved.
@task_item n=2 part=untouched
Secret management: the CLAUDE credential strip, hermetic child environment, canary scan and live rig isolation; nothing changed in what the child inherits or what is scanned.
@task_item n=3 part=untouched
API security and the MCP surface: rmcp features, MCP tools, tool inputs, stdin line cap and MCP error contracts.
@task_item n=4 part=untouched
Paste and typed text validation: character sets, control-character refusal, empty text, line-ending normalisation and the wrapper's re-check.
@task_item n=5 part=untouched
Channel protocol contracts: frame caps, JSON depth, error codes, wrapper channel methods and their params, link and unlink, release from the driver.
@task_item n=6 part=untouched
Pinned binary and plugin handling: content hash, no-clobber placement, re-hash before reuse, plugin files rewritten at start, the trust record writer at start.
@task_item n=7 part=untouched
Dependency pins, toolchain and MSRV floors, licence allow list, workflow permissions and action SHA pins, release build checks: no dependency, toolchain or workflow changed.
@task_item n=8 part=untouched
The absences (no network listener, no outbound calls, no secrets, no accounts, no database, no hosting): no capability added.
