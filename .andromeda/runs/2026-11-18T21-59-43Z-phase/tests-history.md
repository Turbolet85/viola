@history_item n=1 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (5. Integration Test Strategy, 2 rows) — `contract/verify-output/step-line` became `pass|absent|fail`, absent on browser-tools-prompt alone, and `contract/verify-output/summary-line`'s n counts an absent row. This is the output contract that verify's new browser-tools-prompt line sits beside. The new line must leave the step lines, the summary and the exit unchanged. PROVISIONAL (delegate overseer).

@history_item n=2 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (4. Unit Test Strategy; 3. Test Harness Contract; 1. Test Scope Summary) — `decision/version-gate-needs-all-relied-rows` now reads verified when every relied row counts, absent counting on browser-tools-prompt only. `contract/local-live-suite/pass` accepts one absent line on that row, and `decision/ledger-row-verify-probe` reads pass/absent/fail. The live 24-row re-stamp's pass predicate and the re-pinned product's reading both rest on these. PROVISIONAL.

@history_item n=3 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (7. Test Data & Fixtures) — added `contract/fake-agent-cli/start-screen`: under `--screens`, `--start-screen <phase>` draws that screen in place of the ready screen. The scope names this option as how verify's browser-tools-prompt line is built and proved on the fake agent with no live start.

@history_item n=4 marker=2026-11-18-the-2-1-325-recorded-fixture-set
**2026-11-18-the-2-1-325-recorded-fixture-set** — `decision/record-home-names-no-home` was widened: the live rig's record arm refuses a home-named scratch base (record-home, exit 3, uncounted) before any credentials read or ledger line. This is the precedent for rig live-start refusals. Running verify on the real `~/.viola`, which is named by path, has to respect this rule or name the difference.

@history_item n=5 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — it rewrote `contract/fake-agent-cli/trust-config` to the CLI's walk and added three rows: `decision/trust-start-folders-outside-repository` (under /tmp), `decision/run-under-test-owned-user-home` and `decision/harness-wrappers-under-session-user-home`. They exist because a start writes into the user's Claude config. Every test start in this chunk must stay off the real `~/.claude.json`. Only the founder-capped live starts touch it.

@history_item n=6 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** — `decision/record-only-at-zero-failing-rows` gained the rig-only first-screen exception on a failed recording verify. `decision/verify-window-60s-kill` was replaced by `decision/verify-window-80s-kill` (ci and mutants). That kill bounds any verify_window_ case added for the browser-tools line. The scope's hypothesis that a failed live verify still writes its stamp entry also touches the record-only rule.

@history_item n=7 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — `contract/verify-stop/signal` now removes verify's own trust record too. It added `decision/no-verify-start-outside-test-home` (census, guard `every_test_verify_runs_under_a_test_home`), `decision/verify-under-test-owned-user-home`, the fake agent's `--trust-config` / `--trust-record-blank`, and `decision/live-failing-reading-not-verification`. Any new test-started verify must sit under a test-owned home, or the census goes red. A failing or confounded live reading is recorded as read, never as verification, which binds the live 24-row verify and the /context and /mcp readings.

@history_item n=8 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — it replaced `decision/live-readings-not-a-test-layer` with `decision/standing-live-rig`: one committed rig, rehearsals in CI, live starts by hand under the founder's cap, each ledgered with its clock time first, and the live arm refused under CI. It rewrote `decision/real-claude-local-only` to name the rig's live starts. It added the rig's screen-reading exceptions to `no-child-screen-assertions`, `never-parse-rendered-child-screen`, `discipline-at-every-layer` and `no-screen-parsing`, plus the trust-dialog exception to `no-blanket-approval-tests` and the fake agent's trust-form and keystroke rows. Every live start in this chunk runs through this rig and its rules. PROVISIONAL rows were later ratified.

@history_item n=9 marker=2026-11-17-supply-chain-green-and-booted-tests-end
**2026-11-17-supply-chain-green-and-booted-tests-end** — `decision/supervisor-ordinary-child` now ties the in-test supervisor by SIGKILL and its stamping verify by SIGTERM. It added `decision/tied-verify-kill-witness`: a witness must read the stamp's absence, not processes and dirs alone. This applies if the chunk adds a stamping verify test or touches verify's stop path.

@history_item n=10 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** — `decision/wrapper-drop-guard` now has the parent-death tie. It added `decision/no-untied-viola-spawn`, a census over every test source that counts any spawn of the viola binary constant outside the tied builder, and `decision/tied-child-spawn-thread`. A new verify-driving test file must spawn viola only through the tied builder.

@history_item n=11 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — `decision/local-command-post-condition` gained /mcp, confirmed by a new stdout tag in the transcript. `contract/send-result/unconfirmable` and `decision/local-command-unconfirmable` gained the no-readable-transcript case. `contract/driver-send/local-command` now classifies by name, and `decision/local-command-exact-match` was retired. It added `decision/stamping-verify-passes-transcript` and the fake agent's `--transcript` and echo-command rows. Context: the 22nd row `local-command-echo` (Run B's `/mcp disable viola-probe-absent`) made every older stamp unverified, which is the root of C3. Measuring `/context`'s post-condition live (C6) extends this rule.

@history_item n=12 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** — the founder ratified the local-command-delivery rows: `contract/driver-send/local-command`, `decision/local-command-post-condition`, `contract/send-result/unconfirmable` and the retired `local-command-exact-match`. They stand on his own word, so changing them for /context would need his word too.

@history_item n=13 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — added `decision/one-verify-stamp-per-build` (one stamp per pair of built binaries, under target/e2e-home, behind a lock). `decision/fresh-home-per-test` now says a stamped home is a copy, never a link. The `decision/verify-driven-tests-20s-kill` why now rests on the single stamp. A change to verify's output or behaviour flows through every stamped-home test by this one stamp.

@history_item n=14 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — `decision/root-wait-bound` and `decision/within-7s-wait-bound` moved to 12 s, and the mutants default kill moved to 15 s (`decision/mutants-kill-below-cargo-mutants-timeout`). It added `decision/record-home-names-no-home`. These are the wait bounds any waiting test in this chunk must respect, under the scope's 20 s lint.

@history_item n=15 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — a live start measured trust keys passing through the wrapper and writing a human-input wheel line before session-start. It rewrote `contract/event-log/start-wheel-record` and added `decision/start-lines-read-from-start-offset`. Precedent: readers of a reused instance's log on a live home must read from the start's own offset.

@history_item n=16 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (4. Unit Test Strategy) — `decision/version-gate-needs-all-relied-rows` gained «a retired row it also holds changing nothing», because statusline-rate-limits was retired. The live home's older stamps can hold retired rows. The 24-row reading counts only the relied rows.

@history_item n=17 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — the status line variants were dropped from `decision/fixtures-recorded-by-live-verify`, `decision/fake-agent-contract-suite` and `decision/recorder-scrub`. A start no longer writes an instance settings.json. This narrows what a live verify records and what the live home's start writes under `~/.claude*`.

@history_item n=18 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — it extended these rows to cover hyprctl: `decision/fixtures-recorded-by-live-verify`, `fake-agent-on-test-path`, `fake-agent-as-claude`, `fake-agent-argv-only`, `contract/fake-agent-cli/argv-only-options`, `fakes-for-external-services`, `ci-never-real-claude` and `no-production-data-in-tests`. They state the current form of the fake-agent and never-real-CLI-in-CI rules the chunk's tests must keep.

@history_item n=19 marker=2026-11-14-hyprland-exact-frame-bound-pinned
**2026-11-14-hyprland-exact-frame-bound-pinned** — `decision/fakes-for-external-services` gained the planted-stub exception for viola-hyprland's own unit tests. It is part of the standing form of a picked row.

@history_item n=20 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** — the Cookie clause was dropped from `decision/logs-carry-no-secrets`, `decision/no-secrets-in-logs` and `decision/secrets-never-in-logs`, and the cookie scan class was retired. These are the secret rules a live verify of the real home and its diagnostics must keep. Read key names only, never values.

@history_item n=21 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — `contract/readiness-gate/gate-max-wait` now reads the input box up by its rule and prompt rows with no footer literal. It added `decision/screen-fixture-box-rows-as-literals`. A start screen standing on the browser-tools prompt is a modal, not the input box. That interacts with the 8.5 s gate wait the new verify line is proved under.

@history_item n=22 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — the owner of the H2 key-after-resize question in `--local-live` was re-pointed to the route entry «Key after a resize». This is the standing wording of the local-live harness key.

@history_item n=23 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the statusline-rate-limits arm's live red) — a live `viola verify` on unchanged 2.1.287 read `23 pass  1 fail` because the account's plan state, not the CLI, decided the row. It is a precedent of a live home verify failing on an environmental row. The row was later retired.

@history_item n=24 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the CLI tests over std::process::Command) — the trycmd and assert_cmd wording was retired. `tests/cli_verify.rs` pins verify's human lines by literal asserts over `std::process::Command` against `CARGO_BIN_EXE_viola`. Each fixed string is a literal in its test file (`decision/human-output-pinned-by-literals`, `cli-fixed-strings-pinned-by-literal`). Verify's new browser-tools line must be pinned this way.

@history_item n=25 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set) — `LedgerRow::relied()` is all 24 rows on every build. `STEPS_PASS`, `ROW_IDS` and `LEDGER_ROWS` are `[&str; 24]`. The live re-stamp's 24-row target and the literal tables in `tests/cli_verify.rs`, `tests/contract_ledger_probes.rs` and the harness local-live come from here.

@history_item n=26 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — at the time this made 24 relied rows on Unix: `[NN/24]`, the `cli_verify` no-framing and tag-turn summaries, the twenty-three-id older-stamp oracle, and `LEDGER_ROWS` / `LIVE_PASS`. It also added `/compact`'s confirmation by the same session inside `COMPACT_CONFIRM_WINDOW` (`decision/compact-confirm-window`, provisional). It shows where the row-count literals live, which matters if the browser-tools line shifts any summary.

@history_item n=27 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — when a ledger row was added, the tmpfs live home stamped at seventeen read `row-missing` until its next `run --local-live` re-stamped it. This is the direct precedent for C3/C4: a new relied row makes the live home's older stamps unverified until a live re-stamp.

@history_item n=28 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — step lines `[NN/MM]` are one per relied row with MM the relied count. `contract_ledger_probes` lists each relied id once. The 21-row live firing was recorded (`passed 1`). This is how a live stamp's firing is recorded in the plan.

@history_item n=29 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — the 22-row live firing took six starts and read `22 pass  0 fail`. The no-screen `verify_window_` case went from 34.42 s to 42.98 s and no kill moved. This is a precedent for the start count of a live re-stamp and for verify-window timing.

@history_item n=30 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (tests: agents-join) — the relied rows grew to 23. It set the `cli_verify` summary literals per replay omission and the eighth spawn pair (`agents-probe`). This is part of the count history that verify's line literals carry.

@history_item n=31 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — `contract_ledger_probes`' stamped sets became literal lists (2.1.287, 2.1.302). It added the fake agent's `--long-turn-screen` beside `--tag-turn-screen`, a second input-box literal, and the 2.1.302 set to the hygiene walk. This is the pattern for a new screen option and for `decision/ledger-probe-contract-suite` (each stamped set on exactly one list).

@history_item n=32 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the mutants profile's paste-hint class) — the `verify_window_` override was moved to 60 s with planted-hang controls both ways (later 80 s). The standing rule: a kill moves only for a by-design floor, with its control recorded both ways.

@history_item n=33 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (the mutants override order) — the `verify_window_` override stands first in `[profile.mutants]`, first match winning. The order is pinned by `mutants_verify_window_override_stands_before_the_harness_package_override`. A new verify_window_ case joins this class by name.

@history_item n=34 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `GATE_MAX_WAIT` 8 500 ms became the gate's bound (`contract/readiness-gate/gate-max-wait`), and the verify_window_ class waits it out four times by design. Standing rule: a kill line moves only for a floor that is by design.

@history_item n=35 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the seventeen-row live firing was made once. Because `verify` names no program, it stamps the first `claude` on `PATH`. Live readings of a real session are that chunk's own local measurement, not a suite. Trap for this chunk: the live verify of 2.1.325 must name its claude by path, since a bare `claude` is 2.1.327.

@history_item n=36 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — `run --local-live` runs the keeper after the build and before the verify, and `verify-exit-none` covers a refused base. A stalled-start red is a finding about the backing and is never re-run for green. These are the local-live failure codes (`contract/local-live-failure/*`).

@history_item n=37 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (run --local-live specified) — under `CI`, `--local-live` refuses `live-in-ci` with exit 2 before any build. Otherwise it builds, then verifies against the real `claude` through the runner seam. Suite `local-live` closes the codes `build`, `verify-exit-<n>`, `verify-exit-none` and `row-missing`. This is the origin of `decision/live-verify-never-in-ci`, `local-live-runner-seam` and the picked `contract/local-live-*` / `run-reason/live-in-ci` rows.

@history_item n=38 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (stamped root seam) — the root `stamped_home` runs verify against the fake agent. It is stamped only on exit 0 and a last line `stamped <ver>  <n> pass  0 fail`. This is the origin of `decision/stamps-from-fake-agent-verify` / `verify-sole-stamp-writer`, which an extra output line must not break.

@history_item n=39 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — `tests/cli_verify.rs` pins verify's step lines, the `stamped` summary and the `unable:`/`hint:` pairs by literal asserts. It added the fake agent's print mode and the hygiene walk over the recorded set. These are the literal sites a new verify line joins.

@history_item n=40 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — verify's interactive runs and the screen fixture class landed. Screens are signature-rows-only and checked scrub-as-detector (`decision/screen-fixture-scrub-as-detector`). The fake agent takes `--trusted-root` and `--screens`. `--local-live` checks literal row ids. These are the foundations of screen recording, which the browser-tools prompt screen meets.

@history_item n=41 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — `[profile.ci]` gained the `verify_window_` and verify-driven 20 s kill overrides. A verify a test drives has no test-side bound (`decision/verify-driven-tests-20s-kill`). The 7 s bound gave way to a designed floor.

@history_item n=42 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — the 20 s kill stands on a measured reading (the longest verify-driven test at 9.98 s). It added the framing tier and the fake agent's `--framing` and `--paste-hint-ms`. A set without the framing variants must exit 1. This is a precedent for keeping the verify-driven class under its kill when verify's output grows.

@history_item n=43 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — `local` became exit 0 `{confirmed:false, detail:"unconfirmable", cursor}`, with the human line `[  ] unconfirmable <name> local command, no measured post-condition`. This is the origin of the picked `contract/viola-send/unconfirmable*` and `viola-send-human-mode/unconfirmable` rows. A `/context` whose post-condition stays unmeasured falls under them, or stays an ordinary send per C6.

@history_item n=44 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (/compact's same-session confirmation) — `/compact` on a verified CLI waits only for a session-start of cause compact with the same id inside its own 40 s window, never claimed by a prompt-submitted. This is the shape a measured `/context` post-condition would take.

@history_item n=45 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — `verify_stopped_by_a_signal_removes_only_its_own_dirs_and_exits_1` was added: the exact stderr pair, exit 1, no `stamps.json`, and a log ending `process-exit{detail:"interrupted"}` (`contract/verify-stop/signal`, `contract/verify-exit-detail/interrupted`). A stopped live verify on the real home must leave its stamps as found.

@history_item n=46 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola verify`'s refusal codes) — verify writes its refusal code (`pinned-hash-mismatch`, `program-not-found` and others) on `process-exit` under a parsed `VIOLA_NAME`. A failing row is no refusal and carries no code (`decision/verify-failing-row-no-refusal`), and verify takes no `--json` (`decision/verify-takes-no-json`). A browser-tools-prompt line must not be a refusal code.

@history_item n=47 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `cli` catch site under `--json`) — `json_asked` reads `--json` from argv only before a `--` (`contract/cli-arguments/json-flag`). This matters for verify's `-- <program>` form.

@history_item n=48 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the start's and verify's home-tree checks) — `run_refuses_a_widened_home_tree_before_its_log` and `verify_refuses_a_widened_ledger_before_any_step` were added. Under them, a widened `ledger/`, `bin/` or `plugin/` refuses verify with exit 1 before any step, `stamps.json` byte for byte unchanged (`decision/widened-install-dirs-refused`, `contract/viola-run/start-refusal-exit-1`). The real `~/.viola`'s modes must pass these checks before the live verify.

@history_item n=49 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the tree, diagnostics, stamps and pin checks; single writers) — added `update_stamps_refuses_a_widened_ledger_and_leaves_it` (`decision/stamps-refuse-widened-ledger`) and the single-writers case `only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps` (`decision/single-writers`). These hold the scope's hypothesis that only verify writes `stamps.json`, so no hand restore.

@history_item n=50 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (remove-the-guard witnesses bounded by `WITHIN`) — a remove-the-guard witness hung 27 minutes under nextest's default profile. Any control whose neutralised guard lets a product process start needs a bounded witness (`decision/bounded-remove-the-guard-witness`).

@history_item n=51 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — the pinned copy became a no-clobber placement of a closed temp file. It added `place_private_new_*` (`decision/private-placement-mode-first`, `decision/placed-copy-execs-at-once`, ETXTBSY) and `pin::place_absent` re-hashing a present copy. This bears on the live verify pinning its own binary under `~/.viola/bin/<key>/` and on `decision/pinned-exe-hash-checked`.

@history_item n=52 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (fake-agent copies planted one after another, ETXTBSY) — a test planting several fake-agent copies plants each one in turn before any thread starts a child. Trap for any new contract case that plants per-set bins.

@history_item n=53 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** — `contract_fake_agent_drift` compares the fake agent's hook stdin byte for byte with each recorded set walked at run time, never insta (`decision/fake-agent-matches-recordings`, `fake-agent-matches-verify-fixtures`). This is the contract that a fake-agent change for the browser-tools proof must keep.

@history_item n=54 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — a test-started `viola` drops `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN`, and a path oracle compares `resolved(home)` (`decision/test-started-viola-drops-env`, `test-viola-drops-session-vars`). Home resolution was pinned (`decision/home-resolution-order`). Relevant because the live verify runs from a builder session that holds those variables.

@history_item n=55 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** — the `VIOLA_DIR` home step and canonicalisation were ratified as a widening. This is the why of the picked `decision/home-resolution-order`.

@history_item n=56 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — `viola --help` was pinned by the `HELP` literal, and `cli_output_plain` holds no ESC under non-TTY, `NO_COLOR` and `TERM=dumb` (`decision/no-escape-in-plain-output`, `no-sgr-without-tty`, `contract/cli-human-output/no-sgr`). Verify's new fixed-words line on a named stream must carry no SGR.

@history_item n=57 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the state-file ban scoped) — the hand-written state-file ban binds every booted-wrapper test. Only a reader's own unit cases may plant a malformed file by altering the writer's bytes (`decision/state-files-never-hand-written`, `stamps-written-only-by-verify`).

@history_item n=58 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — `path4_unstamped_permission_is_left_to_the_human_and_its_allow_refused` and the unstamped-home answer refusal were added: exit 12, no body. This is the unverified-CLI behaviour (`contract/cli-exit-codes/12`, `viola-answer/unverified-cli`) that a tree-built binary shows on every 22- or 23-row stamp until the live re-stamp.
