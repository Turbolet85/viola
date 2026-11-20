@task_prose part=session
A viola run that meets a claude version with no verify stamp can now start viola verify itself, once, when the new config.json key verify_new_cli is true; the key is off by default and in every CI home.
viola verify stays the one writer of the stamps file: the start only spawns it as a direct child with a closed argv (the pinned copy, the canonical home, the resolved program), stdin null, the start's own credential strip plan applied and VIOLA_NAME, VIOLA_DIR and VIOLA_BIN removed, tied to the start by a parent-death SIGTERM.
A new attempt lock ledger/verify.lock (0600, taken only after the strict-modes check, never the stamps writer's lock) lets one start of two verify; the other runs none. An entry already attempted and red runs no verify again, so there is no loop; a stamp short of one relied id verifies again once.
The start order gains the verify step after the endpoint bind; the folder trust record is now written after it, on the final verdict. The version gate re-reads the stamps through the strict-modes check under the lock and after the verify.
A SIGINT or SIGTERM during the start's verify TERMs the verify child, waits a 30 s grace, then KILLs it, and ends the start by the signal with no stamp, no snapshot and no session. Five fixed ASCII stderr lines (L1 to L5) say what happened, carrying only the parsed version and a closed word.
Counted: the known config keys 4 to 5, the closed spawn subjects 7 to 8 (auto-verify added), the controls table 8 to 9 entries. No dependency added, no capability claimed, nothing retired.
@task_file n=1 path=src/run/auto_verify.rs
what: new: the start's automatic verify step — the verify_new_cli key reader, the decision to verify, the attempt under the lock, the verify child's spawn, environment strip and parent-death tie, and the SIGINT/SIGTERM stop that ends it
@task_file n=2 path=src/human/auto_verify.rs
what: new: the five human stderr lines of the start's verify (verifying, did not verify, earlier red, elsewhere, no stamp), one ASCII write each, no SGR, version and a closed word only
@task_file n=3 path=crates/viola-agent-claude/src/ledger.rs
what: edited: the Claude adapter's verify ledger — a pure read of whether a version's stamp entry holds every relied row (attempted), beside the verified check
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: edited: unit tests of the stamps ledger reads
@task_file n=5 path=crates/viola-state/src/stamps.rs
what: edited: the stamps file store — adds the verify attempt lock ledger/verify.lock, owner-only, taken by try-lock after the strict-modes check
@task_file n=6 path=src/obs.rs
what: edited: the config.json reader's known-keys list (now five, verify_new_cli added) and the diagnostics config parse
@task_file n=7 path=schemas/config.v1.json
what: edited: the config.json schema — the boolean property verify_new_cli
@task_file n=8 path=schemas/diag-line.v1.json
what: edited: the diagnostics line schema — the closed spawn subject list gains auto-verify
@task_file n=9 path=src/run/version_gate.rs
what: edited: viola run's version gate — the strict-modes stamps read, now also giving the entry state (unusable, unattempted, attempted)
@task_file n=10 path=src/run/mod.rs
what: edited: the run module list, naming the auto-verify step
@task_file n=11 path=src/cmd/run.rs
what: edited: viola run's and revive's start order — the verify step after the endpoint bind, the folder trust record written after it on the final verdict, the stop raised on a signal
@task_file n=12 path=src/human.rs
what: edited: the human output module list, naming the auto-verify lines
@task_file n=13 path=src/bin/viola-fake-agent.rs
what: edited: the test-only fake agent — a new fake-args file beside its executable, read bounded by MAX_FRAME, put ahead of argv
@task_file n=14 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: edited: unit tests of the fake agent's options
@task_file n=15 path=tests/support/fake.rs
what: edited: test support that writes the fake agent's files
@task_file n=16 path=tests/support/home.rs
what: edited: test support for test homes and wrapper starts — a start observed before it is ready
@task_file n=17 path=tests/support/outer_pty.rs
what: edited: test support for the outer PTY — the started child's pid
@task_file n=18 path=tests/cli_version_gate.rs
what: edited: integration tests of the unverified CLI gate — the start's automatic verify cases (green once, off by default, no loop, two starts, stop, unstamped)
@task_file n=19 path=tests/cli_controls_not_disableable.rs
what: edited: integration tests that no config setting switches off a control — verify_new_cli does not lift the unverified gate
@task_file n=20 path=tests/contract_bounded_readers.rs
what: edited: the bounded-readers census check — the config.json verify_new_cli reader added with its witness
@task_file n=21 path=crates/viola-e2e/src/rig/mod.rs
what: edited: the live round rig — a new AV arm that keys verify_new_cli in its own scratch home and reads stamp ids, the stamp verdict and the snapshot verdict, and a TERM stop
@task_file n=22 path=crates/viola-e2e/src/rig/isolate.rs
what: edited: the live rig's isolated scratch home and its seeded config
@task_file n=23 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: edited: the live rig binary's label set (AV, AV-2)
@task_file n=24 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited: the live rig's rehearsal tests against the fake agent — the AV arm
@task_item n=1 part=touches
decision/stamps-written-only-by-verify and decision/verify-children-direct-spawn: verify stays the only stamps writer; a viola run start now STARTS verify as a direct child when the key is on.
@task_item n=2 part=touches
decision/unverified-cli-gate, decision/dialog-answers-need-verify-stamp and contract/cli-exit-codes/12: an automatic green stamp now lifts the gate at the start; the rule and the exit code are unchanged, and the key itself never lifts the gate.
@task_item n=3 part=touches
decision/version-gate-strict-stamps-read and contract/strict-modes-refusal/version-gate: the strict stamps read now also runs under the attempt lock and again after the verify, still emitting its one parse-rejected ledger-stamps line.
@task_item n=4 part=touches
decision/stamps-checked-under-lock, decision/stamps-strict-check-first and decision/state-tree-strict-modes-before-use: a second lock, the attempt lock, is taken only after the strict-modes check and is never the stamps writer's own lock; a widened ledger refuses it and is left as found.
@task_item n=5 part=touches
decision/controls-not-switchable, decision/no-control-switch-off and decision/config-cannot-disable-controls: a config key now turns a behaviour on (a start-time verify); no control is switched off; the controls table grows from 8 to 9 entries.
@task_item n=6 part=touches
decision/config-tolerant-parse and decision/config-bad-value-handling: verify_new_cli is a JSON boolean only; any other value reads false with one parse-rejected config-json malformed line, and the document version must be 1.
@task_item n=7 part=touches
decision/bounded-readers-census, decision/reads-capped-at-max-frame and decision/max-frame-shared-cap: a new product reader of config.json for verify_new_cli, bounded by MAX_FRAME at the default JSON depth, added to the census.
@task_item n=8 part=touches
decision/real-cli-local-only and decision/ci-no-secrets-no-real-claude: the key is off by default, so no CI home ever starts a real-CLI verify.
@task_item n=9 part=touches
decision/start-writes-trust-record-flag-only and decision/start-trust-skips-trusted-folders: the trust record step moves after the bind and the start's verify, and is written on the final verdict, not the gate's first one.
@task_item n=10 part=touches
decision/strict-modes-at-start and decision/start-checks-home-tree-before-log-opens: the start's home-tree check precedes verify's own; decision/start-checks-socket-dir-before-bind still holds, the verify step coming after the bind.
@task_item n=11 part=touches
decision/claude-credentials-stripped-from-child, decision/r8-claude-env-strip and decision/parent-identity-env-stripped: the verify child a start spawns carries the start's own strip plan, with VIOLA_NAME, VIOLA_DIR and VIOLA_BIN removed.
@task_item n=12 part=touches
decision/run-resolves-absolute-program, decision/claude-resolved-from-path and decision/exec-commands-absolute-pinned-path: the verify child is the pinned viola copy given the start's own resolved program and canonical home, with no user argument.
@task_item n=13 part=touches
decision/verify-probe-dirs-cleanup and decision/probe-artifacts-cleanup: a stop of the start now reaches its verify (TERM, 30 s grace, KILL) and a dying start TERMs it by parent death; no verify probe directory is left.
@task_item n=14 part=touches
the verify residual rows (decision/verify-trusted-runs-risk, decision/verify-echo-probe-residuals, decision/verify-context-probe-residual, decision/verify-real-cli-transcripts-accepted, decision/verify-never-answers-import and decision/verify-never-answers-import-dialog): with the key on, these residuals arise with no human present, once per new version or relied set, in the start's own cwd.
@task_item n=15 part=touches
decision/driver-permission-policy-outside-viola: a start may now run the verify's real-CLI runs before the driven child, under the same accepted policy.
@task_item n=16 part=touches
decision/stderr-never-log-floor, decision/human-output-escapes-controls and decision/escape-terminal-controls: five new start stderr lines, fixed ASCII with no SGR, holding only the parsed version and a closed word; verify's own output is inherited, and silenced under revive --json.
@task_item n=17 part=touches
decision/refusals-never-block-human and decision/human-typing-takes-wheel: the start's verify window runs before the driven child exists, so the human's keystrokes reach no child during it (a named exception carried in the accessibility rows).
@task_item n=18 part=touches
decision/test-seams-fake-agent-only, decision/test-seams-compiled-out and decision/new-env-read-needs-log-entry: a new test-only seam, the fake agent's fake-args file, read bounded by MAX_FRAME.
@task_item n=19 part=touches
decision/owner-only-file-modes, decision/private-file-modes and decision/explicit-mode-before-write: a new file under the home, ledger/verify.lock, opened 0600 through the private lock helper.
@task_item n=20 part=touches
the live rig rows (decision/live-rig-kept-stamp-owner-only, decision/live-rig-isolated-child, decision/live-rig-seed-from-bundle-logic): a new AV arm keys verify_new_cli only in its own scratch home and keeps stamp ids, verdict words and counts.
@task_item n=21 part=touches
contract/strict-modes-refusal/verify, contract/strict-modes-refusal/run-revive and the contract/verify-refusal-codes rows: words unchanged, now reachable from a start's verify, a refused lock read as the L5 lock-refused word.
@task_fact n=1 slug=input-validation
fact: config.json verify_new_cli is a JSON boolean, default false; any other value reads false with one parse-rejected config-json malformed line, and it is read only from a version 1 document.
@task_fact n=2 slug=authentication-authorization
fact: With verify_new_cli true, viola run and revive start viola verify once for a claude version with no stamp entry; verify stays the only stamps writer, and an entry already attempted and red runs no verify again.
@task_fact n=3 slug=input-validation
fact: The verify attempt lock ledger/verify.lock is owner-only, taken by try-lock only after the stamps strict-modes check and never the stamps writer's lock; a held lock means that start runs no verify and reads the gate unverified.
@task_fact n=4 slug=threat-model-summary
fact: The start's verify child is a direct spawn with a closed argv (the pinned copy, the canonical home, the resolved program), stdin null, the start's strip plan applied, VIOLA_NAME, VIOLA_DIR and VIOLA_BIN removed, tied to the start by a parent-death SIGTERM.
@task_fact n=5 slug=data-protection
fact: A SIGINT or SIGTERM during the start's verify TERMs the verify child, KILLs it after a 30 s grace and ends the start by the signal, leaving no stamp, no snapshot and no session receipt.
@task_fact n=6 slug=error-handling
fact: The start's verify reports on stderr in five fixed ASCII lines, one write each, no SGR, holding only the parsed version and a closed word (spawn-failed, unstamped, lock-refused); none under revive --json.
@task_fact n=7 slug=data-protection
fact: The folder trust record is written after the start's verify, on the final verify verdict.
@task_item n=1 part=untouched
The channel and IPC rows: wrapper channel methods, frames, peer euid checks, the socket directory check and the socket chmod after bind are unchanged.
@task_item n=2 part=untouched
The MCP server rows: tools, stdio frame cap, tool inputs and MCP error shapes are unchanged.
@task_item n=3 part=untouched
The hook contract rows: hook stdin, hook-decision details, fail-open and silent exit are unchanged.
@task_item n=4 part=untouched
The send and answer text rows: paste character sets, control-character refusals and empty-text refusals are unchanged.
@task_item n=5 part=untouched
The dependency-security rows: no crate, tool pin, workflow or audit gate changed; no dependency added.
@task_item n=6 part=untouched
The CI artifact canary scan, secret management of messaging credentials and the pre-push hermetic environment rows are unchanged.
@task_item n=7 part=untouched
The absence rows: no listener, network call, account, secret or release stage was added; no capability is claimed or removed.
@task_item n=8 part=untouched
The revive --json codes and revive argument rows are unchanged.
