@history_item n=1 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (3. Test Harness Contract) — added decision/verify-under-test-owned-user-home: every test-started viola verify and harness boot stamping runs under a test-owned user home whose config file is seeded empty and owner-only, the fake agent told that config file; a --record verify keeps the real HOME and points a kept CLAUDE_CONFIG_DIR at the test-owned home. The precedent for this chunk: a product write into the child's Claude config file at run/revive must likewise land only in a test-owned file (HOME or a kept CLAUDE_CONFIG_DIR), never the host's ~/.claude.json.

@history_item n=2 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (5. Integration Test Strategy) — contract/verify-stop/signal now removes verify's own trust record on a stop, and decision/no-verify-start-outside-test-home added a census holding every test verify start tied to a test-owned home, red on a planted untied start. The writer this chunk reuses (trust.rs / trust_record.rs) came from this chunk. A new census, or a widening of this one, is needed to keep every test run/revive start that writes a record tied to a test-owned home.

@history_item n=3 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (7. Test Data & Fixtures) — added contract/fake-agent-cli/trust-config (the fake agent reads a trust record from the named config by its own key rule, nearest git ancestor else canonical cwd, through a bounded read, any failure untrusted) and --trust-record-blank. This is what lets an e2e case of the product write go red when the write is gone. A change to the fake agent's trust reading (the parent walk, say) moves these rows and puts cli_verify in the gate.

@history_item n=4 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (6. E2E Test Strategy) — added decision/live-failing-reading-not-verification: a failing live reading is recorded as read and never as verification, and a confounded one leaves the row unverified with its re-read owed. This governs how S1 and S2 are recorded.

@history_item n=5 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (3. Test Harness Contract) — retired decision/verify-window-60s-kill for verify-window-80s-kill (the verify_window_ class kill at 80 s in ci and mutants). Any new verify-driven or verify_window_ case this chunk adds sits under these kill lines.

@history_item n=6 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (7. Test Data & Fixtures) — decision/record-only-at-zero-failing-rows gained the rig-only exception: a failed recording verify writes first-screen files of whole scrubbed rows, refused when unclean, never committed. This is how the S1/S2 first screens can be read, and the row read live as `pass` on 2.1.325 (S0B) that this chunk's verified-CLI gate rests on.

@history_item n=7 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (6. E2E Test Strategy) — retired decision/live-readings-not-a-test-layer for decision/standing-live-rig: live rounds reuse the one committed viola-live-rig. Its rehearsals run in CI over the fake agent, its live starts run by hand under the founder's cap with clock time first, and its live arm refuses under CI. S1 and S2 must be live starts of this rig, rehearsed on the fake agent first.

@history_item n=8 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (7. Test Data & Fixtures) — added contract/fake-agent-screens/trust-dialog (2.1.325's trust form, selector on No, Enter on No exits 1). It also changed session-start (fired only after Enter on the trust row when the cwd is untrusted) and exit-triggers. With viola writing the record, the fake agent must start trusted with no key, so these rows are the e2e oracle that the dialog never rose.

@history_item n=9 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (1. Test Scope Summary · 2. Test Strategy · 11. Test Anti-Patterns) — added exceptions to never-parse-rendered-child-screen, no-child-screen-assertions, discipline-at-every-layer, no-screen-parsing and no-blanket-approval-tests: only the live rig reads the child's screen, by compiled literals into closed words, and passes a read trust dialog under its key grant. real-claude-local-only now names the rig's live starts. S1's "first screen reads trust" predicate must use this exception. No other test may read the screen or answer the dialog.

@history_item n=10 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (6. E2E Test Strategy) — contract/event-log/start-wheel-record: where folder-trust keys pass through the wrapper, a wheel{human, human-input} record now precedes session-start, measured live. decision/start-lines-read-from-start-offset was added: a start's lines are read from its own offset, since a reused name keeps earlier starts' lines. This chunk removes the typed trust keys on a written record, so the human-input line should vanish on a trusted start. A case asserting that reads from the start's offset.

@history_item n=11 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (4. Unit Test Strategy) — decision/version-gate-needs-all-relied-rows: a version reads verified only when its stamp holds every relied row as pass, and a retired row it also holds changes nothing. The run-start events order also lost budget-gate. "On a verified CLI" is this gate. A stamp taken at 22 rows reads unverified to a 23-row build, so that run is transport-only and writes no record: a skip case to prove.

@history_item n=12 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (6. E2E Test Strategy) — added contract/viola-run-start/settings: a start hands its child no --settings, writes no settings.json, leaves the user's settings byte- and mtime-equal, and makes no default home under --home. This is the standing promise that a start leaves the user's Claude files alone. This chunk's write into the user's .claude.json is a bounded exception beside it, and the row must not be read as broken by it.

@history_item n=13 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** (3. Test Harness Contract · 5. Integration Test Strategy) — added decision/one-verify-stamp-per-build (stamped homes copy one stamp per pair of built binaries, under a lock). fresh-home-per-test now says a stamped home is a copy, never a link. verify-driven-tests-20s-kill rests on that one stamp. The cases for a verified write start from stamped_home, which holds all 23 rows of this build, so the write path is exercised.

@history_item n=14 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (3. Test Harness Contract) — added decision/stamping-verify-passes-transcript: every stamping verify passes the fake agent --transcript, or the stamp misses a row. A new stamping helper for this chunk's cases must keep it, or the home reads unverified and the write is skipped.

@history_item n=15 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (3. Test Harness Contract) — root-wait-bound and WITHIN moved from 7 s to 12 s, and the mutants default kill to 15 s, below cargo-mutants' 20 s floor. decision/record-home-names-no-home was added. Any wait in this chunk's tests takes WITHIN, under the 20 s lint the scope names.

@history_item n=16 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** (5. Integration Test Strategy) — wrapper-drop-guard now ties every long-lived viola child to its test thread by a parent-death kill. Two rows were added: no-untied-viola-spawn (a census over every test source, red on a planted positive) and tied-child-spawn-thread (spawn only on a thread that outlives the child). New run/revive starts in this chunk's cases must spawn through the tied builder.

@history_item n=17 marker=2026-11-17-supply-chain-green-and-booted-tests-end
**2026-11-17-supply-chain-green-and-booted-tests-end** (2. Test Strategy · 3. Test Harness Contract · 5. Integration Test Strategy) — in-test harness boots are now tied to their test thread (the supervisor by SIGKILL, the stamping verify by SIGTERM). Rows were added: no-untied-harness-boot (a census) and tied-verify-kill-witness (a process-and-dir witness cannot tell a tied verify from an untied one). These are the constraints for any viola-e2e case of the write.

@history_item n=18 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a start's socket directory refusal) — Path 1's start gained the socket directory check after the version gate and before the spawn, shared by revive's start. A refused directory exits 1 before any snapshot, child or bind, with one process-exit line, held by cross_exit_causes and security_negatives cases. It is the precedent for placing a step in that same window and for "left as found" refusal witnesses. The trust write's refusals are no start refusal: they write nothing and the start goes on.

@history_item n=19 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola revive --json`'s ten codes and the closed gap) — revive's start shares run's start causes, which reach revive --json as table rows, while run takes no --json. The start-exit-1 rows in the picked set descend from here. A trust-write refusal must add no exit-1 code or document: the dialog stays the human's, and the start is not refused.

@history_item n=20 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the bare temp-dir home and the liveness-check case) — the trap recorded: an oracle moved by a shared body can hide a test that never reached its subject. Each "nothing written" refusal case must show it reached the write path (a control that does write), not only that the file is unchanged.

@history_item n=21 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the root `cross_` binary and the shared start helpers) — tests/support/start.rs shares the start setups (first_life, stopped_life, revive, kill_wrapper, …), so no root test file copies them. The new start cases reuse these helpers, per one-shared-test-helper and shared-e2e-helpers.

@history_item n=22 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the start's and verify's home-tree checks before any log) — the start runs check_home_tree before the wrapper log opens. A refused tree exits 1 with no log, no line and no child (run_refuses_a_widened_home_tree_before_its_log), and run-checks-before-child-spawn gives the order. The trust write slots into this order after the version gate, so the existing order cases must stay green.

@history_item n=23 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (remove-the-guard witnesses bounded by `WITHIN`) — a control whose neutralised guard lets a product process start needs a witness bounded by WITHIN (an unbounded one hung 27 minutes). The scope requires each control with --timeout and a note of what the subject does with the guard gone. A neutralised trust-write guard still starts a wrapper.

@history_item n=24 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader enumeration in the Contract suite) — added tests/contract_bounded_readers.rs: every product reader of external input is bounded in-statement or named in BOUNDARIES with a witness, and the census is red on an unbounded read. The product read of the CLI's .claude.json (oversize past MAX_FRAME refused) must be a BOUNDARIES row with a cap-driving witness, or reuse a read already named there.

@history_item n=25 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded compare read of `replace_private_shared`) — the shared-target compare reads at most one byte past the compared bytes (decision/bounded-compare). The atomic replace and read-back in trust_record.rs keep this bound if they touch replace_private_shared.

@history_item n=26 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error-body enumeration in the contract suite) — added tests/contract_error_bodies.rs, which checks every product error writer is named (product_error_writers_are_each_named) and that thiserror messages interpolate nothing. The write's closed code words and any new error type must name no path and pass that census. "Nothing of the file anywhere" leans on this check.

@history_item n=27 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (4. Unit Test Strategy) — decision/bounded-readers-keep-the-cap now lists the version gate drain, the hyprctl reader and the dialog-hold and env-keep readers, each keeping no more than the frame cap. A new reader of the Claude config file joins this row.

@history_item n=28 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the R8 strip by exact name, the registry reader retired) — the persistent-environment exception is config.json claude_env_keep, stripped by exact name, with the eleven-name floor. The scope's file choice (<CLAUDE_CONFIG_DIR>/.claude.json only when claude_env_keep keeps that variable, else ~/.claude.json) rests on this rule. Cases for both arms set it per child.

@history_item n=29 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS-gate census reads another OS as a fault) — the census of OS-gated tests counts any gate naming windows or macOS as a fault, even when a row names it. Every OS-gated test needs its own GATES row. New cases stay ungated, or Linux-gated with a row.

@history_item n=30 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (§7 Fake agent) — wrapper boots pass the fake agent --screens --trusted-root <cwd>, and boot_untrusted passes no root (decision/wrapper-boots-pass-trusted-screens). A case of the product write must boot untrusted, with no --trusted-root, so the fake agent reads trust only from the config record viola writes.

@history_item n=31 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (test data: the fake agent's agents files, the test-scoped PATH) — the fake agent answers from files beside its exe with no env var, and every test-started list or revive runs under a test-scoped PATH (tests-never-read-host-sessions, fake-agent-on-test-path). Revive cases of the trust write keep this PATH discipline.

@history_item n=32 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the state-file ban scoped) — decision/state-files-never-hand-written binds booted-wrapper tests to viola's own state files, with only a reader's unit cases planting malformed bytes through the writer. The CLI's .claude.json is no viola state file, so refusal cases (not an object, projects not an object, oversize, a link) may plant it by hand without breaking this ban.
