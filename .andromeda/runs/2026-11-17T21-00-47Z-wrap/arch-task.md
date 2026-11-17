@task_prose part=session
Added the 23rd relied capability-ledger row, trust-record: a trust record another process writes into the Claude config opens an untrusted folder with no trust modal.
viola verify gained a sixth interactive run, Run F, after Run E: it writes a trust record for its own temp folder into the config file its child reads, starts the child there, checks the frame for the trust modal and the input box, then removes that one record (a drop guard removes it on a stop too).
The record's shape (the config file choice, the record key, add and remove, a closed set of 14 trust code words) lives pure in the agent crate; the locked, bounded, mode-keeping file writer lives in the root bin, with verify as its only caller for now.
Counts moved: relied rows 22 to 23, step lines [NN/23], summary 23 pass, interactive runs five to six (A to F), spawn pairs eight to nine; stamps gain a trust-record value that may be a writer word and measured.trust_probe.
The fake agent reads the record from a named file; every test-started verify now runs under a test-owned user home (a new census test guards it); the live rig gained a v arm.
Measured live once on claude 2.1.325: trust-record fail, 7 of 23 pass, confounded; the row stands built and proven on the fake agent and unverified on the real CLI, the live re-read owed.
@task_file n=1 path=crates/viola-agent-claude/src/trust.rs
what: new pure module of the agent crate holding the Claude config's trust-record shape - config file choice, record key, add and remove of one record, the closed trust code words
@task_file n=2 path=src/trust_record.rs
what: new root-bin writer and remover of verify's trust record in the Claude config file - git root lookup, lock file, bounded read-back, atomic replace keeping the file mode, three attempts
@task_file n=3 path=tests/contract_verify_home.rs
what: new contract census test that every test-started verify runs under a test-owned user home
@task_file n=4 path=crates/viola-agent-claude/src/lib.rs
what: the agent crate's module list, edited to add the trust module
@task_file n=5 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger's relied rows, their checks and the stamp merge - edited to add the trust-record row, its check and its stamp values
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: unit cases of the capability ledger's row list, edited for the 23rd row
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit cases of the ledger rows' checks, edited to add the trust-record check cases
@task_file n=8 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit cases of the ledger stamp merge, edited for the trust-record value and the trust probe measure
@task_file n=9 path=src/main.rs
what: the viola binary's module root, edited to add the trust-record writer module
@task_file n=10 path=src/cmd/verify.rs
what: the viola verify command - its inputs, environment strip and recording scrub - edited to pick the Claude config file the trust record goes to
@task_file n=11 path=src/cmd/verify/typed.rs
what: verify's typed interactive-run sequence, edited to add Run F and its results
@task_file n=12 path=src/cmd/verify/typed/runs.rs
what: verify's interactive runs, edited to add Run F, the trust-record run, with its record guard
@task_file n=13 path=src/run/version_gate.rs
what: the start's version gate over the ledger stamps, edited for the 23-row ledger
@task_file n=14 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent, edited to read a trust record from a named config file and to take a blank-start hold
@task_file n=15 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: unit cases of the fake agent's argv options, edited for the trust-config and blank-hold options
@task_file n=16 path=tests/support/verify.rs
what: test support that starts viola verify, edited to tie every start to a test-owned user home and hand the fake agent the trust config
@task_file n=17 path=tests/cli_verify.rs
what: integration tests of viola verify, edited to add the trust-record row cases
@task_file n=18 path=tests/cli_verify/runs.rs
what: integration tests of verify's interactive runs, edited for the sixth run
@task_file n=19 path=tests/cli_verify/stamps.rs
what: integration tests of verify's stamps, edited for 23 rows
@task_file n=20 path=tests/contract_ledger_probes.rs
what: contract test of the ledger's probes and the stamped summary, edited for 23 rows
@task_file n=21 path=tests/contract_bounded_readers.rs
what: contract list of bounded readers at every boundary, edited to add the Claude config file reads
@task_file n=22 path=crates/viola-e2e/src/harness/boot.rs
what: the test harness's session boot, edited to stamp under the session's own user home and hand the fake agent the trust config
@task_file n=23 path=crates/viola-e2e/src/harness/run.rs
what: the test harness's run step, edited for the 23-row verify output and ledger row count
@task_file n=24 path=crates/viola-e2e/src/rig/mod.rs
what: the live rig's arms and summary, edited to add the verify arm v and its summary keys
@task_file n=25 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's isolation of the child's config and temp root, edited to read whether verify's record was left
@task_file n=26 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the test-only live rig binary, edited for the v arm
@task_file n=27 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: rehearsal tests of the live rig, edited to add the v arm rehearsal
@task_item n=1 part=touches
decision/verify-child-set: verify's interactive runs go from five (A to E) to six (A to F); Run F runs last; spawn pairs eight to nine.
@task_item n=2 part=touches
decision/permission-request-question-stays-null: its count of adding a 22nd row is stale; the ledger now holds 23 relied rows.
@task_item n=3 part=touches
contract/ledger-stamps/rows: the trust-record row's value may now be the trust writer's code word when nothing was written, besides pass and fail.
@task_item n=4 part=touches
contract/ledger-stamps/measured: adds trust_probe with write and remove, additive, no version bump.
@task_item n=5 part=touches
decision/env-strip-names-only and decision/env-strip-reads-names-only: verify now reads one value, CLAUDE_CONFIG_DIR, when the strip keeps that name, else HOME, to find the config file.
@task_item n=6 part=touches
decision/no-environment-configuration and decision/env-var-viola-prefix: a new non-VIOLA environment read (CLAUDE_CONFIG_DIR value, HOME) by viola verify only.
@task_item n=7 part=touches
contract/config-json/claude-env-keep: keeping CLAUDE_CONFIG_DIR now also decides which config file verify writes its trust record into.
@task_item n=8 part=touches
decision/verify-removes-only-own-dirs and decision/verify-probe-dirs-drop-guarded: verify now also removes its own trust record through a drop guard; a SIGKILL leaves it.
@task_item n=9 part=touches
decision/claude-shapes-in-one-crate, decision/claude-shapes-isolated and decision/agent-claude-pure: the Claude config trust-record shape lives in the agent crate, pure; the file I/O is in the root bin; the fake agent and the live rig read the record test-only with their own key rule.
@task_item n=10 part=touches
decision/no-typing-into-trust-dialogs, decision/verify-never-types-into-dialog and decision/verify-types-nothing-into-dialogs: verify opens a folder by writing a trust record, never by typing into the trust dialog.
@task_item n=11 part=touches
contract/viola-verify-output/step-line and contract/viola-verify-output/summary-line: the step counter becomes [NN/23] and the summary 23 pass 0 fail; trust-record is the last step.
@task_item n=12 part=touches
decision/capability-ledger, decision/cli-capability-ledger, decision/capability-ledger-gate and decision/version-verified-all-relied-rows: a 23rd relied row joins; a version verifies only when trust-record passes too, and live 2.1.325 does not yet.
@task_item n=13 part=touches
contract/viola-verify-exit-codes/1-relied-row-failed: a trust-record fail now exits 1 like any relied row.
@task_item n=14 part=touches
decision/bounded-readers: new boundaries - the Claude config file read by verify's writer, the fake agent and the rig, each capped at MAX_FRAME.
@task_item n=15 part=touches
decision/separate-lock-files: the trust writer takes its own lock as a .lock directory beside Claude's config file.
@task_item n=16 part=touches
decision/atomic-replaced-state-files, decision/crash-safe-disk-writes and decision/private-mode-before-write: the Claude config is replaced atomically keeping that file's own mode, not the private mode.
@task_item n=17 part=touches
decision/stamps-single-writer, decision/verify-sole-stamps-writer and decision/stamps-merge-own-version-only: the stamp merge writes the new row value and measure; verify stays the one writer.
@task_item n=18 part=touches
decision/version-gate-strict-stamps-read and decision/malformed-stamps-unverified: the version gate reads a 23-row stamp set.
@task_item n=19 part=touches
decision/fake-agent-test-seams and decision/fake-agent-after-double-dash: the fake agent gains argv options trust-config and trust-record-blank, test-only.
@task_item n=20 part=touches
decision/test-isolated-viola-home and decision/test-homes-cleaned-by-owner: every test-started verify now also runs under a test-owned user home, guarded by a census; a recording verify keeps the real HOME and points CLAUDE_CONFIG_DIR at the test home.
@task_item n=21 part=touches
decision/real-cli-probe-local-only and decision/no-live-claude-in-ci: the live rig gained a verify arm run locally once; the trust-record row stays unverified on 2.1.325.
@task_item n=22 part=touches
decision/screen-rows-read-only-by-verify and decision/screen-signatures-compiled: Run F reads its frame for the trust modal literal and the input box.
@task_item n=23 part=touches
decision/verify-transcripts-accepted-residual and decision/verify-record-only-clean: a recording verify now runs Run F too, through a kept CLAUDE_CONFIG_DIR.
@task_fact n=1 slug=established-decisions
fact: viola verify writes a trust record for its own temp folder only into the Claude config file its child reads and removes that one record after, the bounded exception to viola writing nothing into Claude's config; run and revive do not write it.
@task_fact n=2 slug=established-decisions
fact: The relied ledger row trust-record (a trust record written for an untrusted folder opens it with no modal) is the 23rd row, built and proven on the fake agent, unverified on the real CLI.
@task_fact n=3 slug=standard-contracts
fact: The trust writer answers in a closed set of 14 code words (written, removed, config-absent, config-unreadable, config-link, config-not-regular, config-foreign, config-oversize, config-malformed, config-dir-unusable, home-unknown, key-present, lock-held, lost); an absent config is never created.
@task_fact n=4 slug=occupied-resources
fact: viola verify reads the value of CLAUDE_CONFIG_DIR only when claude_env_keep keeps that name, else the user home, to locate the Claude config file - the one value read beside the names-only strip.
@task_fact n=5 slug=occupied-resources
fact: The trust writer holds the lock name of the Claude config file plus .lock, a 0700 directory taken only when free, with three attempts before lost.
@task_fact n=6 slug=cross-cutting-patterns
fact: A trust record's key is the nearest ancestor holding a git entry, else the canonical folder.
@task_item n=1 part=untouched
The wheel, its claim, release and human take: no rule over the wheel moved.
@task_item n=2 part=untouched
Hooks, the dialog hold and hook output tiers: no hook or dialog behaviour changed.
@task_item n=3 part=untouched
The wrapper channel protocol, its frames, methods, fault codes and refusals: unchanged.
@task_item n=4 part=untouched
MCP tools, their results and the MCP server: unchanged.
@task_item n=5 part=untouched
The events log, normalised event kinds and event data: no event kind or field added; no new log event.
@task_item n=6 part=untouched
list, the board and session liveness: unchanged.
@task_item n=7 part=untouched
Hyprland seam and the window features: unchanged.
@task_item n=8 part=untouched
CI workflows, release, fuzzing, coverage floors and dependency policy: no dependency or crate added, no workflow edited.
@task_item n=9 part=untouched
Send, paste and typed text normalisation and the readiness gate timings: unchanged.
