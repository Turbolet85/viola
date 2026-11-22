@task_prose part=session
The session mended the Epoch 4 audit's ranked defects; no capability was claimed, no crate, dependency, verb, event kind, refusal word or exit code was added or retired.
A send whose session-start line the wrapper claimed, but whose snapshot write then fails, now settles its slot empty before the error returns: the send goes back to waiting and the confirmation window's existing deadline closes it as not-delivered with no-prompt-submitted. No new time bound and no new closed word.
The claude agents --json read now has its own named kill bound, 5 s, the version probe's value kept and not measured against the real CLI; the version probe and verify's version read keep the version deadline. The stalled-host false red of verify's agents read is NOT mended and stays a carry.
The test harness's abort wait after a failed readiness dropped from 20 s to 15 s (the 20 s readiness deadline unchanged); the stuck-boot test hard-links its binaries into a temp dir beside the build directory instead of copying them into /tmp.
Two drift replay tests joined the verify-window class of the ci and mutants nextest profiles (no kill value moved); mutation survivors got killing test cases, two equivalents named.
Measured, no code change: the chrono advisory re-read with no safe upgrade (the ignore stands), and a SIGKILLed verify leaves its own one-key trust record under a test-owned config.
@task_file n=1 path=src/run/send/slot.rs
what: the send slot of the wrapper — the claim of a send's confirming hook line (session-start, prompt-submitted), the snapshot write and append order, and the confirmation window's close that ends a send confirmed or not-delivered
@task_file n=2 path=src/run/agents.rs
what: the claude agents --json read (the agents listing joined into the board by list, MCP list, revive and verify), its capped read and its kill bound
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: tests of the capability ledger's row checks, here the env-probe names check (a user word read as a name part)
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: tests of the ledger stamps envelope reader, here a v1 document of a failing shape refused as attempted
@task_file n=5 path=crates/viola-agent-claude/src/screen.rs
what: the screen signatures read over the vt100 screen model (option rows, readiness), tests only
@task_file n=6 path=crates/viola-pty/src/pump.rs
what: the PTY pump behind viola's pty seam — output drain after the child's exit, the bounded input enqueue and the writer's end wait, tests only
@task_file n=7 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: the end-to-end test of the test harness lifecycle (boot, status, cleanup, the stuck-boot supervisor case)
@task_file n=8 path=crates/viola-e2e/src/harness/boot.rs
what: the test harness boot — its readiness deadline per instance and its abort wait after a failed readiness, test-only
@task_file n=9 path=.config/nextest.toml
what: the nextest profiles (ci, mutants) and their per-class kill overrides, here the verify-window class filter
@task_file n=10 path=.claude/rules/testing.md
what: the project's testing rule, its frontmatter paths list
@task_file n=11 path=tests/contract_bounded_readers.rs
what: the bounded-readers contract test, a census pinning each boundary read's bound by its call
@task_file n=12 path=tests/contract_lints.rs
what: the lint contract tests, pinning the nextest verify-window override by its whole filter line
@task_item n=1 part=touches
Rows on send confirmation (every send confirmed after the fact, unconfirmed delivery reported as not-delivered, /clear confirmed by a new session, /compact confirmed by the same session): a claimed session-start whose snapshot write fails now ends the send not-delivered, no longer a dangling claim.
@task_item n=2 part=touches
The not-delivered detail no-prompt-submitted gains a cause: a claimed confirming line whose snapshot write failed, closed by the confirmation window's existing deadline.
@task_item n=3 part=touches
A row stating a hook line stays in the appended set even when the snapshot write then fails (the hook.event appended line, the instance snapshot writer, the snapshot writer): read it beside the send now settled empty on that failure.
@task_item n=4 part=touches
Rows on the confirmation window and the send time bound: no new bound was added; the existing deadline arm closes the settled send.
@task_item n=5 part=touches
Rows on the agents read (agents listing degrades never errors, the board and list payload agents read failure, agents join read ungated, MCP list reads claude agents, heartbeat first with sysinfo and claude agents enrich): the read is now killed at its own named bound, 5 s, no longer the version probe's deadline; its failure semantics unchanged.
@task_item n=6 part=touches
Any row whose cell holds the agents read's 5 s kill or names its owner as the version deadline: the value is unmoved, its owner is now the agents read's own constant, unmeasured against the real CLI.
@task_item n=7 part=touches
Bounded readers at every boundary: the guard census pinning the agents read's bound moved its needle to the new constant.
@task_item n=8 part=touches
Verify's bounded child set and verify's agents read: verify's agents read takes the new bound; verify's version read keeps the version deadline.
@task_item n=9 part=touches
A row whose guard names a verify_window_ test or the verify-window nextest class: the class now also holds the two fake-agent dialog and framing replay tests, in both the ci and mutants profiles; the lint guard pins the new whole filter.
@task_item n=10 part=touches
Mutation testing at the epoch boundary and the mutants profile rows: survivors of the epoch run now have killing cases, two named equivalent survivors argued.
@task_item n=11 part=touches
Rows on the test harness (viola-e2e test-only, harness environment, test-side removals and temp roots): the harness abort wait after a failed readiness is 15 s, the stuck-boot test's binaries hard-linked into a temp dir beside the build directory, not /tmp.
@task_item n=12 part=touches
Rows on the PTY pump and passthrough (portable-pty behind the pty seam, child exit on the process handle): new tests pin output drained after an exit, enqueue waiting for room until its deadline and the writer's end wait; no behaviour changed.
@task_item n=13 part=touches
Rows on the ledger stamps envelope (malformed envelope, malformed or untrusted stamps read unverified) and the env-probe stamp: new cases pin a v1 document of a failing shape refused and the user-word name-part check; no behaviour changed.
@task_item n=14 part=touches
Weekly advisories and the cargo-deny dependency policy: the chrono advisory was re-read with no safe upgrade; the ignore stands, no row value moved.
@task_item n=15 part=touches
Verify writes and removes its own trust record: a SIGKILLed verify was measured to leave one projects entry for its own temp folder holding only the trust-accepted key; read whether a row says less.
@task_fact n=1 slug=established-decisions
fact: A send whose confirming session-start line the wrapper claimed but whose snapshot write then failed settles its slot empty and ends not-delivered with no-prompt-submitted at the confirmation window's existing deadline, never left claimed.
@task_fact n=2 slug=standard-contracts
fact: The claude agents --json read is killed at its own bound, 5 s, separate from the version probe's deadline and not measured against the real CLI; under a stalled host it can still read a false failure, an open carry.
@task_item n=1 part=untouched
The wheel rows (claim, release, human take, interrupt): no wheel behaviour moved.
@task_item n=2 part=untouched
Dialog hold, dialog answers and the hook-to-kind map: no dialog path or hook mapping changed.
@task_item n=3 part=untouched
MCP tools, MCP results and the MCP stdin caps: no MCP surface changed.
@task_item n=4 part=untouched
CLI verbs, CLI exit codes, refusal order and the revive and verify refusal codes: none added, moved or retired.
@task_item n=5 part=untouched
The channel protocol, frames, fault codes and protocol version: unchanged.
@task_item n=6 part=untouched
Paste bound, paste cut mark and typed text normalisation: untouched by this session.
@task_item n=7 part=untouched
Names (binaries, crates, commands, environment variables, sockets): no name taken or freed.
@task_item n=8 part=untouched
Trust writer code words, plugin install, pinned copy and Hyprland seam rows: unchanged.
@task_item n=9 part=untouched
The version probe's own deadline and start's new-version verify: kept as they were.
