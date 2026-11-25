@history_item n=1 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (6. E2E Test Strategy) — added + decision/window-placement-proven-live: a real terminal window's placement on Hyprland is proven only by a live reading; the fakes prove the placement words and the window records only. Why it matters: this is the tier rule v2-07's new active-workspace clause extends; the fakes cannot witness «active workspace unchanged», so that clause's proof is a gated live leg (R2 read placement only, which is the defect's cause), while the fake/tap prove only the new dispatch form's words.

@history_item n=2 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (6. E2E Test Strategy: 1 row) — ~ decision/window-placement-proven-live widened to «or on the workspace its snapshot recorded when viola up re-opens it», live reading gated on R4. Why it matters: `viola up` is one of the three callers of the move; R4a is the reading that measured the active workspace 1 → 5, so the row must carry the active-workspace clause for up's re-open too.

@history_item n=3 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (6. E2E Test Strategy: 2 rows) — + decision/window-reopen-proven-live (a real fleet re-open proven only by the gated live reading R4; R4b refuses until granted; its rehearse form reads each window from the instance's own window record and is never a live reading, because the fake compositor lists no app-id) and ~ decision/hint-names-its-cause except (boot-time-unread held by its unit pair alone). Precedent for the gated, operator-fired leg with a rehearsal form this chunk's v2-07 half and its fixture re-record must take.

@history_item n=4 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (6. E2E Test Strategy: 3 rows) — ~ decision/strict-modes-before-any-log (viola up's board scan joins), + decision/child-fd-table-read-after-launch-settles (fd guard read after the launch settles; a red closed by a settle wait), + decision/live-rig-root-outside-repositories (live run refuses a root inside a repository, PROVISIONAL). Traps for any new up/open test and for the live leg's rig root.

@history_item n=5 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (7. Test Data & Fixtures) — + contract/fake-foot/spawn-record: the fake foot appends one line of pid and argv per spawn beside itself. The open/up/revive-under-origin cases this chunk re-asserts read placement through this record and the fake hyprctl, not a real window.

@history_item n=6 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (10. Quality Gates & Coverage Targets) — + decision/stand-in-terminal-tree-teardown: one shared teardown ends a stand-in terminal's process tree (kill a leaf only while its recorded parent runs, then wait, kill by start time at the deadline), because killing a fake agent mid-exit cut coverage profiles. Any new caller test that spawns the foot stand-in must use it.

@history_item n=7 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (6. E2E Test Strategy: 2 rows) — ~ decision/strict-modes-before-any-log (viola focus joins) and + decision/window-raise-proven-live: a real raise by viola focus is proven only by the gated live reading R3 beside R2; its rehearse form over the fakes is never a live reading and its verdict words come only from the plan's closed set (PROVISIONAL on fork F-D). Model for how v2-07's live active-workspace reading is shaped: gated, rehearsal form, closed verdict words.

@history_item n=8 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (the operator's answer to F-A ratified) — ratified the strict-modes rows of the focus entries; Kept: decision/window-raise-proven-live stays provisional on fork F-D (his word granted R3's firing, not its shape). A live-reading row whose shape the operator has not answered stays PROVISIONAL; this chunk's new gated leg will likely land the same way.

@history_item n=9 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (1. Test Scope Summary) — ~ contract/run-start-events/order: a start under --origin appends one window line with source wrapper after launch, before session-start. A revive under --origin is one of the move's callers; any change to the move must keep this one-window-line order.

@history_item n=10 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (1. Test Scope Summary: list items) — ~ contract/viola-list-json/items gains window? {address, workspace} and origin? cli|agent (PROVISIONAL on the delegate's card). The window record is what the fakes prove; a no-focus move must leave that record's workspace unchanged.

@history_item n=11 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (7. Test Data & Fixtures) — ~ decision/fake-agent-on-test-path: the fake agent also stands as foot, the terminal stand-in, by its file name, answering from files beside its exe with no env var. The open/up cases' fake terminal and fake hyprctl are the same binary.

@history_item n=12 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (5. Integration Test Strategy) — ~ decision/fake-agent-as-claude: the only external boundary is served by viola-fake-agent as claude, as hyprctl in a test-scoped bin dir, and as foot. A new move form reaches tests only through this hyprctl personality.

@history_item n=13 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (8. Mocking & Stubbing Discipline) — ~ decision/fakes-for-external-services: the real foot terminal is replaced by the same binary as foot, beside claude and hyprctl; tests never reach the real services. The phase's read-only compositor queries are measurement, not tests.

@history_item n=14 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (1. Test Scope Summary: replay) — ~ decision/snapshot-replay-fields: the replay recovers window too, the last such line winning. The window record's workspace is replayed state; the move-form change must not alter what is recorded.

@history_item n=15 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (6. E2E Test Strategy: live rig) — + decision/live-rig-clean-start (PROVISIONAL): a live rig's stand-in takes every argument the wrapper puts first, the rig unsets every inherited VIOLA_* name, and every board read runs with no claude on PATH — the three faults behind R2's red. The gated re-record and the v2-07 live leg (one terminal window, no claude) must keep these.

@history_item n=16 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (11. Test Anti-Patterns) — + decision/echo-oracle-skips-allowed-word: a sweep hazard met in the open's refusal oracles. Trap for any refusal oracle this chunk adds.

@history_item n=17 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (1. Test Scope Summary) — ~ decision/fixtures-recorded-by-live-verify: the recorded Hyprland replies are refreshed only by the test-only viola-hypr-tap. The move fixture's re-record (keeping only `ok` today) goes through the tap, live, on the operator's grant — never hand-edited.

@history_item n=18 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (7. Test Data & Fixtures: 4 rows, 1 of 2) — ~ decision/fixture-hygiene-walk (each Hyprland reply held to its closed per-request string keep-list), ~ decision/fake-agent-on-test-path (hyprctl by file name), ~ contract/fake-agent-cli/argv-only-options (as hyprctl its mode is read from a file beside its exe), ~ decision/recorder-scrub (viola-hypr-tap applies the same scrub). A new dispatch form needs its keep-list entry, its fake reply and its scrubbed recording to line up.

@history_item n=19 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (7. Test Data & Fixtures: 1 row, 2 of 2) — ~ decision/dirty-payload-refuses-recording: a Hyprland reply set that fails the unclean check refuses the whole recording. The one granted live re-record can be refused whole; plan for that outcome.

@history_item n=20 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (7. Test Data & Fixtures: 2 rows) — ~ decision/fixtures-walked-for-hygiene (Hyprland fixtures walked against their schema with planted cases) and ~ decision/fake-agent-argv-only except (as hyprctl it reads its mode and reply set from files beside its exe). The re-recorded move set joins the hygiene walk; the fake's reply set stays what the real compositor answers.

@history_item n=21 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (5. Integration Test Strategy) — ~ decision/fake-agent-as-claude: hyprctl became a second external child process, served by the fake agent in a test-scoped bin dir. The seam's closed request set is tested only through this stand-in.

@history_item n=22 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (8. Mocking & Stubbing Discipline) — ~ decision/fakes-for-external-services: the real hyprctl is replaced by the same binary as hyprctl. No double of the seam; a new move form lands in the fake, never a second compositor seam.

@history_item n=23 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (2. Test Strategy) — ~ decision/ci-never-real-claude: CI never touches the real claude or the real hyprctl; the seam is held in CI only by the recorded set replayed through the fake. The active-workspace clause cannot be a CI predicate over a real compositor.

@history_item n=24 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (4. Unit Test Strategy) — ~ decision/bounded-readers-keep-the-cap: the hyprctl reader keeps at most the frame cap, a cut reply read as oversize, held by its BOUNDARIES row. Any new reply read for the move form stays under that reader.

@history_item n=25 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (6. E2E Test Strategy: property-tests-on-parsers) — ~ decision/property-tests-on-parsers: the hyprctl reply parsers are a 512-case proptest surface with committed regressions. A changed reply shape for the move must keep its property.

@history_item n=26 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (1. Test Scope Summary: parser-property-fuzz-coverage) — ~ decision/parser-property-fuzz-coverage: the hyprctl reply parsers joined the property tests and the fuzz replay (hyprctl_json). A new reply form needs its seed/corpus considered.

@history_item n=27 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (6. E2E Test Strategy: property-tests-with-proptest) — ~ decision/property-tests-with-proptest: proptest-regressions committed for the Hyprland reply suites. Keep regressions committed if the reply parser changes.

@history_item n=28 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (11. Test Anti-Patterns) — ~ decision/no-production-data-in-tests: fixtures come only from viola verify probe text or a test-only recorder's capture (viola-hypr-tap), scrubbed. The re-recorded move fixture is such a capture.

@history_item n=29 marker=2026-11-14-hyprland-exact-frame-bound-pinned
**2026-11-14-hyprland-exact-frame-bound-pinned** (8. Mocking & Stubbing Discipline) — ~ decision/fakes-for-external-services except: viola-hyprland's own unit tests serve hyprctl by a planted shell stub, because cargo-mutants runs only the mutated package's tests, so a root-package test cannot kill that crate's mutants. A change to the seam's request set needs its kill in the crate's own tests.

@history_item n=30 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (6. E2E Test Strategy: 3 rows) — + decision/adopt-proven-live (adopt proven over the fake only, the real-CLI clause left to the gated live reading R5) and ~ decision/session-live-read-fails-open. R5a's first firing is where the folded item was found: the automatic verify ran in an untrusted scratch folder and stamped a failing verdict.

@history_item n=31 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (1. Test Scope Summary) — ~ decision/real-claude-local-only: the real claude is reached only locally through viola verify, the standing live rig's live starts and the gated live-reading scripts, never in CI. The v2-07 live leg and any live re-read of the folded item sit on this route.

@history_item n=32 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (3. Test Harness Contract) — ~ decision/verify-driven-tests-20s-kill except: a verify-driven test named verify_window_ takes that class's 80 s kill instead. A witness for the automatic verify's place that drives whole verifies belongs in that class.

@history_item n=33 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** (6. E2E Test Strategy) — ~ decision/single-writer-state-files: only verify writes stamps.json, the verify a keyed start runs included. The folded item changes what that start-run verify stamps on its own failure; it must stay the single stamps writer.

@history_item n=34 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (1. Test Scope Summary: 4 rows) — ~ decision/human-never-blocked except (the start's automatic verify window, before any child exists: a typed key waits in the terminal's buffer), ~ decision/readiness-gate-tested-by-refusals, ~ decision/no-child-screen-assertions, + decision/live-rig-reads-compiled-shapes. Moving where the automatic verify runs must keep the human-never-blocked exception true.

@history_item n=35 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (1. Test Scope Summary: 2 rows) — + decision/run-start-verify-and-trust-after-bind: between the exclusive bind and the first snapshot a start runs any automatic verify, then on a verified final verdict writes an untrusted spawn folder's trust record. The folded item's fork (where verify runs, what it stamps) sits exactly on this order.

@history_item n=36 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (ratified: 2026-11-20-a-new-cli-version-stamped-automatically, single-writer-state-files) — the delegate's confirm of the start-run verify as the one stamps writer stands on the founder's word. A change to that writer is a ratified-surface change, not a free move.

@history_item n=37 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (6. E2E Test Strategy) — + decision/live-failing-reading-not-verification: a live reading that fails a ledger row is recorded as read, never as verification; a confounded reading leaves the row unverified and its live re-read owed. The 9 pass · 19 fail stamp taken in an untrusted folder is a confounded reading; this row is the standing rule the fix answers.

@history_item n=38 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (5. Integration Test Strategy) — ~ contract/verify-stop/signal (a stop also removes verify's own trust record) and + decision/no-verify-start-outside-test-home (a census holds every test-started verify start tied to a test-owned home, closed exempt list, red on a planted untied start). A new auto-verify witness must pass this census.

@history_item n=39 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (3. Test Harness Contract) — + decision/verify-under-test-owned-user-home: every test-started verify runs under a test-owned user home with an empty owner-only config the fake agent is told; verify writes a trust record into that config. A witness for verify in an untrusted folder needs this setup.

@history_item n=40 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (7. Test Data & Fixtures) — + contract/fake-agent-cli/trust-config and + contract/fake-agent-cli/trust-record-blank. The fake's trust read is how a test makes «an untrusted folder» for the folded item's witness.

@history_item n=41 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (7. Test Data & Fixtures) — ~ contract/fake-agent-cli/trust-config (the CLI's walk: cwd's exact key then each parent up to the git root) and + decision/trust-start-folders-outside-repository (trust-start test folders under /tmp, since a folder inside the workspace keys at its git root). Trap: an untrusted-scratch-folder witness placed under target/e2e-home reads trusted.

@history_item n=42 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (3. Test Harness Contract) — + decision/run-under-test-owned-user-home and + decision/harness-wrappers-under-session-user-home: the start's trust step writes the Claude config, so test starts run under a test-owned user home. Any start-driven auto-verify witness must keep this.

@history_item n=43 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (7. Test Data & Fixtures) — + contract/fake-agent-screens/trust-dialog and the session-start / exit-trigger amendments: under --screens with an untrusted cwd the fake draws the trust form and fires SessionStart only after Enter on the trust row. The fake's untrusted start is the stand-in for the folder verify could not read.

@history_item n=44 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (6. E2E Test Strategy) — − decision/live-readings-not-a-test-layer → + decision/standing-live-rig: live rounds reuse one committed test-only rig, rehearsals in CI, live starts by hand under the start cap, each ledgered, live arm refused under CI. The re-record and v2-07's live half run through it or a gated script beside it.

@history_item n=45 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** (6. E2E Test Strategy) — ~ decision/standing-live-rig except: the SF arm's contained rehearsal is a local entry that skips by its printed sandbox probe. Precedent for a rehearsal form that cannot run in CI.

@history_item n=46 marker=2026-11-21-key-after-a-resize
**2026-11-21-key-after-a-resize** (6. E2E Test Strategy: 3 rows) — + decision/refusal-rehearsal-reads-own-effects (a refusal rehearsal reads its own effects, not the shared teardown words), with the KR settle rows. A gated leg that refuses before its grant needs its refusal rehearsed this way.

@history_item n=47 marker=2026-11-21-key-after-a-resize
**2026-11-21-key-after-a-resize** (6. E2E Test Strategy: 1 row) — + decision/kr-seams-rehearse-only: rehearse-only seam options are refused on a live start or another arm, usage exit 2, before any ledger line. Model for any rehearse-only option the live leg gains.

@history_item n=48 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (4. Unit Test Strategy) — ~ decision/version-gate-needs-all-relied-rows: verified only when the stamp holds every relied row as counting (pass, or absent on browser-tools-prompt). This is why a kept failing stamp leaves the version unverified for every later start — the folded item's consequence.

@history_item n=49 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (5. Integration Test Strategy) — ~ contract/verify-output/step-line and ~ contract/verify-output/summary-line (stamped <ver>  <n> pass  <m> fail, n counting absent on browser-tools-prompt). The 9 pass · 19 fail · 1 absent reading is in this shape; a witness of the fix reads these literals.

@history_item n=50 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (3. Test Harness Contract) — − decision/verify-window-60s-kill → + decision/verify-window-80s-kill. A witness that runs whole verifies takes the 80 s class.

@history_item n=51 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (7. Test Data & Fixtures) — ~ decision/record-only-at-zero-failing-rows except: only the live rig's failed recording verify writes first-screen files, never committed. A failed verify writes nothing else; relevant to what the auto verify may leave on its own failure.

@history_item n=52 marker=2026-11-22-epoch-4-cleanup
**2026-11-22-epoch-4-cleanup** (3. Test Harness Contract) — ~ decision/verify-window-80s-kill now also covers the fake agent dialog and framing replay tests; + decision/harness-abort-wait-below-ci-kill. Kill-line classes a new verify-driven witness joins.

@history_item n=53 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** (3. Test Harness Contract: 2 rows) — + decision/one-verify-stamp-per-build: stamped homes copy one verify stamp per build under target/e2e-home under a lock. A witness that needs an unstamped or failed-stamp home must not reuse the shared stamp.

@history_item n=54 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (3. Test Harness Contract) — + decision/stamping-verify-passes-transcript: every stamping verify gives the fake agent --transcript, else a row fails. A verify that a witness expects green must keep it.

@history_item n=55 marker=2026-11-21-cross-session-prefix-ledger-row
**2026-11-21-cross-session-prefix-ledger-row** (3. Test Harness Contract) — + decision/stamping-verify-test-owned-runtime-dir: every stamping verify runs with a short owner-only runtime dir the test owns. Trap: without it a stamp reads a failing row for reasons the folded item is not about.

@history_item n=56 marker=2026-11-17-supply-chain-green-and-booted-tests-end
**2026-11-17-supply-chain-green-and-booted-tests-end** (2. Test Strategy) — + decision/tied-verify-kill-witness: a kill witness for a tied stamping verify waits for a child inside a viola-verify run dir and reads the stamp's absence, since processes and dirs alone cannot tell tied from untied. Relevant to any witness of the start-run verify's own failure.

@history_item n=57 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** (5. Integration Test Strategy: 3 rows) — ~ decision/wrapper-drop-guard, + decision/no-untied-viola-spawn (census over every test source, proven on a planted positive), + decision/tied-child-spawn-thread. Every new open/up/revive/auto-verify test spawns viola tied.

@history_item n=58 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (6. E2E Test Strategy: start-wheel-record) — ~ contract/event-log/start-wheel-record: where folder-trust keys pass through the wrapper a wheel{human, human-input} line precedes session-start. An untrusted spawn folder changes a start's records; witnesses must allow it.

@history_item n=59 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (6. E2E Test Strategy: start offset) — + decision/start-lines-read-from-start-offset: a start's first lines are read from its own offset, never line 1, since a reused name keeps earlier starts' lines. Trap for up/revive re-open cases over a reused instance.

@history_item n=60 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (3. Test Harness Contract: root-wait-bound) — ~ decision/root-wait-bound: root waits bounded at 12 s, below the 15 s mutants kill and cargo-mutants' 20 s floor. Waits in new cases stay under it.

@history_item n=61 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (3. Test Harness Contract: 2 rows) — ~ decision/within-7s-wait-bound (WITHIN = 12 s) and the paste-hint kill's why. The WITHIN literal moved; the row key still reads 7s.

@history_item n=62 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (3. Test Harness Contract: mutants kill) — ~ decision/mutants-kill-below-cargo-mutants-timeout: the 15 s mutants default kill, a stated departure from fixing slow cases by removing slow work. No new default-kill raise without that standard.

@history_item n=63 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (remove-the-guard witnesses bounded by WITHIN) — a control whose neutralised guard lets a product process start needs a bounded witness (an unbounded one hung 27 minutes). The folded item asks for a witness that cannot pass vacuously; its remove-the-guard control must be bounded.

@history_item n=64 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the start's and verify's home-tree checks before any log) — run and verify refuse a widened home tree before their log (run_refuses_a_widened_home_tree_before_its_log; verify_refuses_a_widened_ledger_before_any_step). Moving the automatic verify inside the start must keep the check ahead of any log.

@history_item n=65 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (tree, diagnostics, stamps and pin checks at the unit tier) — update_stamps_refuses_a_widened_ledger_and_leaves_it (checked before the create and under the lock). Any change to what the stamp writer writes on failure keeps these refusals.

@history_item n=66 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (no-clobber placement: forced-window witness, test parameter) — place_private_new_placed_copy_execs_at_once and the `placed` hook as a test parameter, not a seam. Pattern for any forced-window witness of the move form; adds no env, flag or feature seam.

@history_item n=67 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader enumeration in the Contract suite) — tests/contract_bounded_readers.rs holds BOUNDARIES and EXCEPTIONS literal tables and a census of product reads. A new hyprctl reply read for the move form must be a row there.

@history_item n=68 marker=2026-10-25-windows-endpoint-admission
**2026-10-25-windows-endpoint-admission** (scoped log captures through with_scoped_default) — every root-bin scoped capture goes through the pinned-dispatch helper, witness scoped_capture_sees_a_line_whose_callsite_another_thread_registered_first. Unit cases that read the window step's or the auto verify's log lines use it.

@history_item n=69 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a start's socket directory refusal) — the start's socket directory check after the version gate and before the spawn, refused 1 with socket-dir-refused before any snapshot, child or bind. Re-ordering the start's verify step must keep this order and its witnesses.

@history_item n=70 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket directory's security negatives) — setting_does_not_disable_the_socket_dir_check runs every SETTINGS row against the check. Any new config key or flag for the move form or the verify's place must become a SETTINGS row and not disable a control.

@history_item n=71 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS scope: Linux only, one CI leg) — Linux only, the window features of 0.2.0 on Hyprland; Windows/macOS code out of the tree at multi-os-final. The seam's move form is Hyprland's alone.

@history_item n=72 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS-gate census reads another OS as a fault) — a gate whose predicate names windows or macos is a fault even when a row names it (contract_os_gated_tests). A new gated case must not name another OS.

@history_item n=73 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the census of OS-gated tests) — tests/contract_os_gated_tests.rs: every OS-gated test function must be a GATES row with its reason. A new cfg-gated case joins the table.

@history_item n=74 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the CI matrix [ubuntu-latest] and two workflows) — CI runs one ubuntu-latest leg. No compositor exists on that leg, so the active-workspace clause is held there only through the fake.

@history_item n=75 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — mutation moved to the epoch boundary and on demand; no per-chunk, pre-push or CI mutation gate. This chunk is Epoch 5's last entry, so the boundary audit reads its seam and verify changes next.

@history_item n=76 marker=2026-10-10-windows-mutation-grade
**2026-10-10-windows-mutation-grade** (host-excluded missed mutants) — a missed mutant wholly under a cfg proved false for the host leaves the missed count. Mutants of the move form are reachable on Linux and are counted.

@history_item n=77 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the always-admit survivor as 0.1.0's known limit) — the peer-admitted always-admit survivor stands as the version's known limit, read at each epoch-boundary audit. Precedent for naming a survivor a known limit rather than leaving it unowned.

@history_item n=78 marker=2026-11-22-epoch-4-cleanup
**2026-11-22-epoch-4-cleanup** (10. Quality Gates & Coverage Targets) — + decision/equivalent-survivors-known-limit: two survivors named equivalent and left as code for the next boundary's audit. Same handling available for an equivalent mutant in the move's dispatch.

@history_item n=79 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** (2. Test Strategy) — + decision/os-gated-cases-in-binary-root: a split binary keeps its cfg-gated cases in the root file. Applies if a caller's test binary is split.

@history_item n=80 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** (6. E2E Test Strategy) — ~ decision/unverified-cli-send-only: on an unverified CLI answer and switch are refused unverified-cli. A kept failing auto-verify stamp drives every later start into this degraded mode — the folded item's user-visible cost.

@history_item n=81 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (1. Test Scope Summary: 2 rows) — ~ contract/run-start-events/order gained the lineage line (PROVISIONAL F3) and the replay's lineage. One of the steps that built today's start-record order the window line now follows.

@history_item n=82 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (1. Test Scope Summary) — ~ contract/run-start-events/order gained the launch line after lineage. The window line under --origin sits after it.
