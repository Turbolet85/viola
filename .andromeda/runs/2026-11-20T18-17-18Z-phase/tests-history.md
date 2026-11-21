@history_item n=1 marker=2026-09-24-fake-agent-and-test-data-fixtures
**2026-09-24-fake-agent-and-test-data-fixtures** — (fixture naming) fixtures are named `<Event>.<variant>.json` with the CLI's PascalCase hook event name and no mapping table; the fake agent joins `<cli-version>` under `--fixtures`. Any new probe fixture this chunk records (harness-prefix or env-names) must take this form (decision/fixture-named-by-hook-event).

@history_item n=2 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — replaced the 14-name S6 literal with the 11-name identity floor plus the prefix / persistent-set rule; "the product's own strip list" became `IDENTITY_FLOOR`; E2 asserts `env_stripped_count` / `env_stripped_known` / `env_kept`, 0 canaries. The origin of the floor this chunk turns into a ledger row, and of the literal-copy oracle (decision/identity-floor-tested-against-literal-copy).

@history_item n=3 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — verify's `[NN/MM]` step lines, `stamped` summary and `unable:`/`hint:` pairs pinned by literal asserts in `tests/cli_verify.rs`; the fixture hygiene walk is a run-time directory walk, not rstest `#[files]` (rstest refuses an empty glob). The step-line and summary literals grow with every new relied row.

@history_item n=4 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — root `stamped_home` stamps only through `viola verify` against the fake agent, `stamped` true only on exit 0 and a `… 0 fail` last line; `run --local-live` checks literal row ids, failing `row-missing`, and claims no live measurement beyond that; fake-agent default CLI version equals the recorded set's version. The register this chunk extends with two row ids and the stamp source it must keep.

@history_item n=5 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** — (harness prefixes) widened the M2 / Path 5 harness prefixes to four: `<agent-message from=`, `<task-notification>`, `<\cross-session-message`, `<cross-session-message`, on the raw start with no trim; witness `prompt_origin_files_the_cross_session_tag_as_harness`. The exact compiled set the harness-prefix ledger row's probe must read.

@history_item n=6 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** — (drift contract) `contract_fake_agent_drift` compares each recorded set's hook `stdin_hex` byte for byte, walked at run time, an empty walk failing; insta stays for decision bodies only. A new probe's recorded payloads must be replayed under this contract, never snapshotted.

@history_item n=7 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — the default CLI version moved across `DEFAULT_CLI_VERSION`, `RECORDED_CLI_VERSION`, `stamped_home` and `Wrapper::boot` together; relayed fixtures kept and later superseded. Precedent for moving the default-version constants (both read 2.1.328 now) if 2.1.329 is recorded.

@history_item n=8 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — ten ledger rows; boot step 4 runs verify with `--screens --turn-stop --trusted-root`; `contract_ledger_probes` holds stamped and drift-only sets as literal lists, every dir on exactly one list; screen fixtures are signature-rows-only and scrub-as-detector, refused whole with a named code. The pattern every row addition follows (decision/ledger-probe-contract-suite, decision/screen-fixture-scrub-as-detector).

@history_item n=9 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — fourteen rows; `LEDGER_ROWS` and `[NN/MM]` grow; `[profile.ci]` gains the `verify_window_` override and the verify-driven 20 s binary line; a verify a test drives has no test-side bound. Trap: added verify runs raise the designed floor, which the kill lines must absorb.

@history_item n=10 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — seventeen rows; the framing tier joins the recorded set and the drift contract replays it under `--framing`; a set without them must exit 1; the 20 s kill stands on a measured reading. Precedent for a new probe's variants entering the recorded set and replay.

@history_item n=11 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — `prompt_origin_live_shape_files_as_harness` pins two measured harness raw starts (a `<task-notification>` at the prompt's very start; the unescaped three-attribute cross-session start); a differing Linux dev-host reading on 2.1.287 (12 names in a hook's environment, four outside the eleven, three of the eleven absent) stands beside the floor, the literal oracle staying eleven; the `--local-live` id list was stale and re-listed. Directly bears on the identity-floor probe: a live env-names capture will not equal the eleven.

@history_item n=12 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — the `verify_window_` kills moved in proportion to a by-design floor, each with a planted-hang control both ways. Standing rule: a kill line moves only for a floor that is by design, the control recorded both ways; applies if the new probes add verify runs.

@history_item n=13 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the seventeen-row live firing through `--local-live`; `verify` with no program stamps the first `claude` on `PATH`; live readings of a real session are a local measurement, not a suite. Bears on "stamped live" and which `claude` a live verify reads.

@history_item n=14 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — eighteen rows (`LedgerRow::ALL`, new row last); `cli_verify` partial-fail summaries and `contract_ledger_probes` move with the count; Run B gains a paste recorded as a new variant walked by hygiene; counts carry their rule; a live home stamped at fewer rows reads `row-missing` until re-stamped. The template for adding a ledger row and its probe.

@history_item n=15 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — relied set via `LedgerRow::relied()`, `MM` its length; `cli_verify` per-row summaries; a probe's `--settings` on Run B; new fixture class with its own schema; a `workspace.repo` kept-name exception to the scrub (later retired). Precedent for a new fixture class and for a scrub exception needing the founder's word (cf. a captured env name carrying private text).

@history_item n=16 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — Run E added; relied 22; the older-stamp oracle widened; stop case plants the new run dir; `verify_window_` no-screen case grew 34.42 s to 42.98 s with no kill moved; the case count dropped from the master. A new run/probe moves every one of these registers; masters state no case count.

@history_item n=17 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (test data) the fake agent placed as `claude` in a test-scoped bin dir answers from files beside its exe with no env var; every test-started `list` / `revive` runs under a test-scoped `PATH` so no test reads the host's live sessions; a new fixture class (`Agents.recorded-1.json`) schema-checked and hygiene-walked with planted cases. Constraint for a probe that captures environment or a second session: no new env seam, no host reach (decision/tests-never-read-host-sessions).

@history_item n=18 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (agents-join row) relied 23; Run B's agents read placed after its first turn settles, before the framing pastes; an uninterrupted verify closes one more spawn pair; arm cases `check_agents_join_*`. Precedent for a probe that runs a second process against the verify child and for where it sits in a run.

@history_item n=19 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** — `boot --agents-mode` is written after step 4 because Run B's agents read goes through that copy (a mode written first failed `agents-join`). Trap: harness boot ordering versus a probe's inputs during stamping.

@history_item n=20 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — the controls-table completeness case and the row `config_key_does_not_disable_the_identity_strip`; a test-started `viola` drops `VIOLA_NAME` / `VIOLA_DIR` / `VIOLA_BIN`. The identity strip's not-disableable negative this chunk's floor row rests on (decision/no-switch-disables-a-control, decision/test-started-viola-drops-env).

@history_item n=21 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — 24 rows, the new row appended last; `STEPS_PASS` / `ROW_IDS` / `LEDGER_ROWS` / `LIVE_PASS`; Run B's added paste; the older-stamp oracle; partial-fail summaries. The most complete list of literals a row addition must move.

@history_item n=22 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — the 2.1.302 set joined as a second stamped set in `contract_ledger_probes` and the hygiene walk; the fake agent gained `--long-turn-screen`. Precedent for adding a newly recorded CLI version (2.1.329) to the stamped list.

@history_item n=23 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — `verify_window_` kill at 60 s in `ci` and `mutants`, with planted-hang pairs; the no-screen case at ~43 s. The verify-time budget any added probe run must fit (since moved to 80 s).

@history_item n=24 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** — a test-only recorder (`viola-mcp-tap`) runs through the `--record` scrub and refusal with four closed codes naming the file only; its fixtures get their own schema and walk with planted cases. Precedent if the second-session probe needs a test-only recorder rather than verify itself.

@history_item n=25 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — `tests/contract_bounded_readers.rs` enumerates every external reader (`BOUNDARIES`, `EXCEPTIONS`) with a census that fails an unnamed unbounded read. A new env-names or child-output reader this chunk adds must be capped and named there (decision/bounded-readers).

@history_item n=26 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — `booted_with_the_value` plants a value into one child through the PTY spawn's `env_set` (as `tests/tui_env_strip.rs`); the error-body check reports place and class `planted`, never bytes. The mechanism for a failing test that plants a CLAUDE* value and proves no value reaches any file (decision/error-body-check-reports-place-and-class, decision/scan-hits-never-echo-bytes).

@history_item n=27 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — `viola verify` writes its refusal code on `process-exit` under a parsed `VIOLA_NAME`, `unclean-fixture` among them; a failing row is no refusal and no code; the fixture's file name enters no code. Holds for a new probe's scrub refusal (contract/verify-exit-detail/unclean-fixture, decision/verify-failing-row-no-refusal).

@history_item n=28 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — `secret-scan` classes reduced to `claude-stripped` (twelve), `content-canary`, `mode`, `unreadable` (cookie later retired). The scan that must stay green over any captured env-names artifact (contract/secret-scan-class/claude-stripped).

@history_item n=29 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** — `decision/logs-carry-no-secrets`, `decision/no-secrets-in-logs` and `decision/secrets-never-in-logs` lost the Cookie clause, standing on stripped CLAUDE* values alone. The current wording of the no-value rule the identity-floor probe must satisfy.

@history_item n=30 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — `verify_refuses_a_widened_ledger_before_any_step`; `update_stamps_refuses_a_widened_ledger_and_leaves_it` (checked before the create and under the lock). Rows decision/widened-install-dirs-refused and decision/stamps-refuse-widened-ledger that any change to the stamp's relied-id set must keep.

@history_item n=31 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — the state-file ban scoped: booted-wrapper tests never hand-write `ledger/stamps.json`; only a reader's own unit cases may plant malformed bytes through the writer. Constraint on any test needing a stamp with the new rows (decision/state-files-never-hand-written).

@history_item n=32 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — `verify_stopped_by_a_signal_removes_only_its_own_dirs_and_exits_1` plants foreign dirs and asserts every spawn pair closed. A probe adding a child or run dir to verify must be removed by the stop path and counted here.

@history_item n=33 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (R8 strip) the persistent exception is `config.json` `claude_env_keep` alone, the strip by exact name, the floor and its literal staying eleven beside the differing Linux reading; E2 keeps a `claude_env_keep` name. The current shape of the floor the new row must capture (decision/r8-claude-env-strip, decision/claude-env-strip).

@history_item n=34 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (relied set) `LedgerRow::relied()` is every row on every build; `STEPS_PASS`, `ROW_IDS`, `LEDGER_ROWS` one ungated form; the older-stamp oracle re-read. No per-OS row split remains for the two new rows.

@history_item n=35 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (bootstrap) the `ci` kill line lists the verify-driven binaries by its `binary(…)` rule, its count a register a new verify-driven binary moves.

@history_item n=36 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (identity floor) the floor reads «measured on the retired Windows host»; (E2) the live child fd table on 2.1.287 read as a local measurement, no suite. The floor's provenance a live env-names reading would replace or confirm.

@history_item n=37 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (statusline-rate-limits) a live verify read the row red because it measured account state, not the CLI. Trap for the identity-floor probe: a row must read a CLI behaviour, not the host's environment, or it reads red per operator.

@history_item n=38 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master's text became rows (decision / contract / absence); every later entry names a row by handle. The form this chunk's amendments take.

@history_item n=39 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — `+ decision/screen-fixture-box-rows-as-literals`: box rows recorded as literals alone, the walk refusing more. Bears on any screen recorded by the harness-prefix probe.

@history_item n=40 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — `~ decision/recorder-scrub`, `~ decision/dirty-payload-refuses-recording`, `~ decision/fixture-hygiene-walk`, `~ decision/fixtures-recorded-by-live-verify`, `~ decision/no-production-data-in-tests`: each new recorder or fixture class is added to the scrub, refusal, walk and closed keep-list rows. The rows the env-names capture extends if it records a new fixture class.

@history_item n=41 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `~ decision/version-gate-needs-all-relied-rows`: a retired row a stamp also holds changes nothing; `− decision/budget-file-from-statusline-hook` and siblings. Bears on old 27-row stamps once the relied set grows: they read unverified.

@history_item n=42 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — `~ decision/fake-agent-contract-suite`, `~ decision/fixtures-recorded-by-live-verify`, `~ decision/recorder-scrub`: the variant families the drift contract replays and verify refreshes are a closed list. A new probe variant family must be added to those lists.

@history_item n=43 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — `~ decision/hook-payloads-recorded-by-verify` except (StopFailure with no recorded payload, no ledger row, no probe) and `+ decision/send-cases-copy-recorded-fixtures` (a synthetic-only fixtures root draws no input box). Precedent for a behaviour kept outside the ledger, and a trap for any synthetic probe fixture root.

@history_item n=44 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `+ decision/start-lines-read-from-start-offset`; `~ contract/event-log/start-wheel-record` (trust keys add a human wheel line). A reader of a verify child's or reused instance's prompt-submitted lines must start at its own offset.

@history_item n=45 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — `~ decision/root-wait-bound`, `~ decision/within-7s-wait-bound` (`WITHIN` 12 s), `~ decision/mutants-kill-below-cargo-mutants-timeout` (15 s), `+ decision/record-home-names-no-home`. The bounds and the record-home rule any new `--record` test must keep.

@history_item n=46 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — `+ decision/one-verify-stamp-per-build`, `~ decision/fresh-home-per-test`, `~ decision/verify-driven-tests-20s-kill`: stamped homes copy one stamp per build under a lock, which keeps each stamped-home test under the 20 s kill. A longer stamping verify (more probes) costs once per build, not per test.

@history_item n=47 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — `+ decision/stamping-verify-passes-transcript`: every stamping verify passes `--transcript` or a row fails. Precedent: a new probe input every stamping helper must pass for its stamp to hold the new rows.

@history_item n=48 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — `− decision/live-readings-not-a-test-layer → decision/standing-live-rig`, `~ decision/real-claude-local-only`, the screen-reading excepts on `decision/never-parse-rendered-child-screen`, `decision/no-child-screen-assertions`, `decision/discipline-at-every-layer`, `decision/no-screen-parsing`. «Stamped live» runs through the one committed rig, rehearsed over the fake agent in CI, its live starts by hand under the founder's cap.

@history_item n=49 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** — `~ decision/wrapper-drop-guard`, `+ decision/no-untied-viola-spawn`, `+ decision/tied-child-spawn-thread`. A probe that adds a second process to verify's child set, and any test spawning it, must stay tied and pass the census.

@history_item n=50 marker=2026-11-17-supply-chain-green-and-booted-tests-end
**2026-11-17-supply-chain-green-and-booted-tests-end** — `+ decision/tied-verify-kill-witness`, `~ decision/supervisor-ordinary-child`, `+ decision/no-untied-harness-boot`, `~ decision/rstest-case-tables`. The kill-witness design for a stamping verify a longer probe set must keep green.

@history_item n=51 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — the last ledger-row chunk with a probe that writes outside its run dir: `~ contract/verify-stop/signal` (stop removes its own trust record), `+ decision/no-verify-start-outside-test-home`, `+ decision/verify-under-test-owned-user-home`, `+ decision/live-failing-reading-not-verification`, fake agent `--trust-config`. The closest precedent for a probe touching the user's Claude config and for how a failing live reading is recorded.

@history_item n=52 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** — `~ decision/record-only-at-zero-failing-rows` (the rig-only first-screen exception) and `− decision/verify-window-60s-kill → decision/verify-window-80s-kill`. The current recording rule and verify-window budget.

@history_item n=53 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — `+ decision/run-under-test-owned-user-home`, `+ decision/harness-wrappers-under-session-user-home`, `+ decision/trust-start-folders-outside-repository`. A second session started by a probe must run under a test-owned user home and never write the real config.

@history_item n=54 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — `~ contract/verify-output/step-line`, `~ contract/verify-output/summary-line`, `~ decision/version-gate-needs-all-relied-rows`, `~ contract/local-live-suite/pass`, `~ decision/ledger-row-verify-probe`: the `absent` verdict, readable on browser-tools-prompt alone. The step-line and verified-rule registers the two new rows enter.

@history_item n=55 marker=2026-11-18-the-2-1-325-recorded-fixture-set
**2026-11-18-the-2-1-325-recorded-fixture-set** — `~ decision/record-home-names-no-home`: the rig's record arm refuses a home-named scratch base before any credentials read. Bears on re-recording 2.1.328 or recording 2.1.329 in the rig's scratch home.

@history_item n=56 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** — `~ decision/standing-live-rig` (SF arm contained rehearsal local only), the screen excepts widened to box-row cell attributes, fake agent `--turn-fail` / `--settings-hooks`, and the StopFailure exception settled on one contained live reading, nothing claimed beyond it. Precedent for how a single live reading is scoped and recorded.

@history_item n=57 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — `~ contract/fake-agent-input/echo-command` (the fake echoes an exact verify probe literal under `--framing`), `+ decision/private-fake-for-no-transcript`, `+ decision/live-rig-hint-cell-reading`. Pattern for the fake agent replaying a new probe in CI.

@history_item n=58 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** — `+ decision/fired-wake-keeps-wheel` (a keyless human-filed prompt relabelled harness on a verified CLI), `~ contract/fake-agent-hooks/stdin-payload`, `~ decision/fake-agent-replays-recorded-bytes` (the closed list of fields the fake sets). A second harness-origin path beside the prefixes; any field the fake sets for the new probe joins that closed list.

@history_item n=59 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — Path 5 as landed: `tests/tui_wheel.rs` holds the harness-turn case; `tests/cli_wheel.rs` the wheel verbs. The existing E2E witness of decision/harness-turn-keeps-the-wheel the harness-prefix row's probe backs.

@history_item n=60 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** — a harness-injected turn refuses `send` `turn-running` until its `turn-ended`, booted over the gated `path3.json`. Trap for a probe sending into a verify child: a harness turn it causes must end before the next send (decision/injected-turn-keeps-wheel).

@history_item n=61 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** — the four screen-reading excepts and `~ decision/screen-signatures-only` now name compiled shapes over compiled literals. The current bound on what the harness-prefix probe may read from the child's screen.

@history_item n=62 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — `~ decision/single-writer-state-files`: a keyed start's own verify is the stamps writer. Bears on a stamp carrying the new rows from a start's verify.
