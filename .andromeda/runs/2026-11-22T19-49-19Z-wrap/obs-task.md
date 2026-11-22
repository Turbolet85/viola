@task_prose part=session
The session mended the Epoch 4 audit's ranked defects and re-measured the chrono advisory; no capability was claimed and coverage stays 23/37.
In the send path, a session-start line the confirm window claimed whose snapshot write then fails now settles the slot unclaimed, so the send goes back to waiting and the confirm window's existing deadline closes it as send-refused with detail no-prompt-submitted. Before, the error returned with the slot still claimed.
The claude agents --json read now has its own named kill bound, AGENTS_READ_DEADLINE, at 5 s, the version probe's value kept and not measured against the real CLI. The version probe keeps VERSION_DEADLINE, and verify's false red on the agents read under a stalled host stays an open carry.
The harness's abort wait after a failed readiness went from 20 s to 15 s, and the readiness deadline stays 20 s. Two drift replay tests joined the verify_window_ kill class in both the ci and mutants test profiles.
Tests were added for eight mutation survivors (env-names fault checks, the stamps document shape, screen option rows, the PTY pump's drain, enqueue and writer end), each shown red with its mutant applied. Two equivalent survivors are named and argued.
RUSTSEC-2026-0543 was re-read with no safe upgrade, so the ignore stands. A SIGKILLed verify was measured to leave one projects trust entry for its own temp folder. No new log line, event, detail word or span was added.
@task_file n=1 path=src/run/send/slot.rs
what: the wrapper's send confirm slot, which records hook events, claims a session-start line, writes the snapshot and settles the confirm window (run.confirm_window)
@task_file n=2 path=src/run/agents.rs
what: the claude agents --json read, the agents-probe child spawn with its kill bound, shared by list, revive and verify
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the capability ledger's check tests, here the environment-names fault check
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the capability ledger's stamps document tests, here the refusal of a v1 document of a failing shape
@task_file n=5 path=crates/viola-agent-claude/src/screen.rs
what: the claude screen reader that reads the vt100 screen for dialogs and option rows (tests only, a dot-led option row read unnumbered)
@task_file n=6 path=crates/viola-pty/src/pump.rs
what: the PTY seam's output pump and input writer (tests only, covering draining output after the child exits, enqueue waiting for room up to its deadline, and the writer end's bounded wait)
@task_file n=7 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: the agent-run harness lifecycle end-to-end tests, including the supervisor's stuck-boot case
@task_file n=8 path=crates/viola-e2e/src/harness/boot.rs
what: the agent-run harness boot supervisor, with its readiness deadline and its abort wait after a failed readiness
@task_file n=9 path=.config/nextest.toml
what: the test runner's ci and mutants profiles and their per-test kill overrides (the verify_window_ class)
@task_file n=10 path=.claude/rules/testing.md
what: the project's testing rule file, here its frontmatter path list
@task_file n=11 path=tests/contract_bounded_readers.rs
what: the contract test that pins every child read to a named kill bound
@task_file n=12 path=tests/contract_lints.rs
what: the contract lint tests, here the pinned filter line of the verify_window_ override
@task_item n=1 part=touches
contract/confirm-window-close/clear and contract/confirm-window-close/compact: a claimed session-start line whose snapshot write fails now ends the send unconfirmed, closing at the window's deadline as send-refused no-prompt-submitted instead of returning an error with the slot still claimed.
@task_item n=2 part=touches
contract/send-refused-detail/no-prompt-submitted: a new path to this detail word, the claimed session-start line whose snapshot write failed. The word itself is unchanged.
@task_item n=3 part=touches
decision/compact-confirm-window and contract/send-span-attributes/window-ms: the run.confirm_window close now also covers the failed-snapshot claimed line, through its existing deadline arm. No span attribute was added.
@task_item n=4 part=touches
contract/send-line/unconfirmable, contract/send-path-log-events/event-order and decision/send-refused-by-each-deciding-side: the wrapper side now ends with send-refused no-prompt-submitted where it once errored on a claimed line. Check whether the send path's event order or the unconfirmable case reads true for it.
@task_item n=5 part=touches
contract/process-log/child-and-probe-lifecycle and decision/agents-cli-instrumented-at-boundary: the agents-probe read is now killed at its own named bound, AGENTS_READ_DEADLINE (5 s), no longer the version probe's. The spawn and exit pair is unchanged.
@task_item n=6 part=touches
contract/diag-subject/agents-probe, contract/revive-process-log/agents-probe, contract/verify-spawn-subject/agents-probe and contract/child-spawn-log/subject: the subject's read has a separate bound of the same 5 s value. Flag any row cell that names the version probe's bound as the agents read's.
@task_item n=7 part=touches
decision/agents-cli-parse-failure-unknown, decision/agents-probe-parse-failure-unknown and contract/parse-rejected/claude-agents-json: an agents read killed at its bound still reads as a failed read (Malformed, unknown). Verify's false red on the agents read under a stalled host is not mended and stays a carry.
@task_item n=8 part=touches
decision/version-rejections-counted-by-detail and contract/run-process-log/version-probe: the version probe keeps VERSION_DEADLINE, and only the agents read left it. Flag any row that states one bound for both.
@task_item n=9 part=touches
decision/obs-deps-pass-cargo-deny and any row holding the RUSTSEC-2026-0543 ignore: re-read on 2026-11-22 with no safe upgrade (patched empty). The ignore stands on the unmaintained-upstream ground, and the advisory re-read moves to the Epoch 5 boundary.
@task_item n=10 part=touches
decision/spine-hook-budget: its wording «provisionally» reads stale because its premise was supplied on 2026-10-04. The operator's word keeps the seven pre-Epoch-4 provisional rows as they stand, and no row is changed here.
@task_item n=11 part=touches
decision/obs-code-mutation-tested and decision/surviving-obs-mutant-fails: eight killable survivors now have killing tests, and two equivalent survivors (in the ledger and the screen reader) are named and argued as a known limit.
@task_item n=12 part=touches
contract/agent-run-status/state-last-error, contract/agent-run-status/state and contract/agent-run-status/state-degraded: the harness's abort wait after a failed readiness is now 15 s, and the readiness deadline stays 20 s. Flag any cell that holds the 20 s abort value.
@task_item n=13 part=touches
contract/pty-seam-log/paste and contract/pty-seam-log/kill-fallback: the pump's drain after exit, the enqueue deadline and the writer end's wait gained tests only. No log event moved.
@task_item n=14 part=touches
decision/env-names-only, decision/r8-strip-logged-as-names-only, decision/unreadable-ledger-is-unverified and contract/run-log-parse-rejected/ledger-stamps: the ledger's environment-names fault check and stamps document shape gained tests only. Behaviour and logged fields are unchanged.
@task_fact n=1 slug=span-trace-coverage
fact: A session-start line the confirm window claimed, whose snapshot write then fails, ends that send unconfirmed. The slot settles unclaimed, and the window's own deadline closes it as send-refused with detail no-prompt-submitted.
@task_item n=1 part=untouched
Panic capture, panic lines, the panic hook and panic outcome by role: no panic path or panic record changed.
@task_item n=2 part=untouched
PII scrubbing, redaction, the field allow-list, veil and the NEVER-log floor: no logged field was added or widened.
@task_item n=3 part=untouched
The MCP stdio server, its tools and frame refusals, the channel JSON-RPC lines and their corr and conn join keys: no protocol, frame or correlation change.
@task_item n=4 part=untouched
Dialog and hook contracts (hook stdout, hook-decision detail words, hook fail-open, dialog log fields): no hook or dialog behaviour moved.
@task_item n=5 part=untouched
Exit codes and exit-cause detail words (exit 1, 20, 21, socket-dir-refused, strict-modes-failed, server-verify-failed): none was added, removed or re-meant.
@task_item n=6 part=untouched
Log file location, format, schema, levels, service identity and the logging stack (tracing, subscriber, sinks, modes): no change.
@task_item n=7 part=untouched
Metric coverage, heartbeat liveness, and the secret-scan, schema-conformance and panic gates of CI and their uploads: no counter, threshold or gate changed.
@task_item n=8 part=untouched
Hook performance budgets other than the stale wording of the spine hook row: no hyperfine budget value moved.
