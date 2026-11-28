@history_item n=1 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — the GUI surfaces and their tests left 0.1.0 (the `viola ui` boot, `/api/*`, SSE E4, Playwright, axum-test), E4 kept as a stub «The frontend is deferred to 0.2.0; its technology is not chosen», and the obs vocabulary's `ui` process / `ui-<port>` marked reserved. This is the stub the console's surface section replaces; a console role must not resurrect the retired web vocabulary by accident.

@history_item n=2 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (Log format) — the `ui` process and the four web events left the closed Log format: process logs are `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`, `process` is `run|hook|mcp|cli`, and the key is kept equal to obs-plan §3's copy in the same pass. A new console diagnostics role extends exactly this closed enum (contract/log-line/process, decision/process-logs-via-tracing-subscriber) and must move obs-plan in lockstep; adding an enum value needs a Decisions Log entry (decision/closed-enums-need-decisions-log).

@history_item n=3 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (§2, §5, §7, §9–§11; §3 harness) — no browser runner, `run` step 3 reads none, `logs` prints the per-process logs of `run`/`hook`/`mcp`/`cli`; nextest `retries = 0`, serialising tests (`max-threads = 1`) banned. The `logs` command's role list (contract/test-harness/logs) is what the console's own role must join.

@history_item n=4 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — the 0.2.0 version census rewrote the founder direction's process list «(run/hook/mcp/ui)» to (run/hook/mcp), and E4 to «N/A, viola has no web or graphical frontend». The `ui` word is retired; the scaffold's role and E4/absence rows (Scenario E4 overview, absence/browser-suite) are the texts this chunk revisits.

@history_item n=5 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (E4 retitled; headroom floor) — E4 read «none in 0.1.0» over an `N/A` gap; the e2e-home headroom floor `HEADROOM_FLOOR_MIB` with the refusal `e2e-home backing: {free} MiB free, below the {floor} MiB floor` (contract/e2e-home-refusal/message), one compiled constant set by no variable, flag or config key. Console tests run in these homes.

@history_item n=6 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (OS scope) — Linux only: CI on `ubuntu-latest`, «the window features of 0.2.0 on Hyprland»; Windows/macOS code out of the tree. Binds decision/linux-only and decision/tests-run-on-linux-ubuntu-latest: the console's tests run on the one Linux leg, and its window belongs to Hyprland.

@history_item n=7 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (census fault) — the OS-gate census (`tests/contract_os_gated_tests.rs`) reads a gate naming `windows` or `target_os = "macos"` as a fault even when named (decision/linux-only guard `contract_os_gated_tests`). Any console test gate is held by it.

@history_item n=8 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (PTY seam) — viola-pty is openpty alone, exit detected on the handle; §5 Module ↔ PTY reads real openpty spawns, a key after a resize sent once the child reports the size; one carve-out home (`tests/chaos_feed_panic.rs`). The console's keystroke driver sits on this seam (decision/pty-tested-on-real-openpty, decision/real-pty-in-integration).

@history_item n=9 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (portable-pty `=0.9.0`) — the viola-pty seam, the tui outer-PTY driver and harness boot's supervisor moved to portable-pty `=0.9.0`; `pty_backend()` still openpty. The driver's PTY is this pinned version; a terminal-drawing/reading dependency beyond it is the scope's fork.

@history_item n=10 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (CLI tests over `std::process::Command`) — no manifest holds `assert_cmd`, `predicates`, `trycmd` or `snapbox`, no `tests/cmd/`; human layouts are literals in their test files; the E2E ban: never `.output()`/`.status()` on a long-lived wrapper, use `.spawn()` + a Drop guard (`tests/support/cli.rs` `Running`). Binds decision/no-cli-snapshot-crates, decision/human-output-pinned-by-literals and decision/wrapper-spawned-with-guard for the console process.

@history_item n=11 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (supply-chain advisories) — `[advisories] ignore = []` was reached (RUSTSEC-2026-0441 closed by the portable-pty move); the ignore list then narrowed to none. Precedent for carry `cry-h6vzhlce`: an ignore leaves when its cause leaves.

@history_item n=12 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — decision/no-advisory-ignored and decision/no-ignored-advisories gained their first except (rmcp advisories in a banned/off feature, until rmcp leaves <3.5), on the founder's own answer; ratified at 2026-11-27T19-07-11Z-wrap. The exception cell carry `cry-h6vzhlce` re-reads.

@history_item n=13 marker=2026-11-17-supply-chain-green-and-booted-tests-end
**2026-11-17-supply-chain-green-and-booted-tests-end** (advisories) — both no-ignore rows' except gained the second ground «an unmaintained-upstream advisory with no safe upgrade, re-read each epoch» for RUSTSEC-2026-0543 (chrono); ratified at 2026-11-22T17-29-41Z-wrap. This is exactly carry `cry-h6vzhlce`'s subject: if a safe chrono upgrade exists the ignore leaves and the except's second ground may lose its only member; else it stands one more epoch on the operator's word.

@history_item n=14 marker=2026-11-17-supply-chain-green-and-booted-tests-end
**2026-11-17-supply-chain-green-and-booted-tests-end** (ties) — supervisor-ordinary-child tie split (supervisor SIGKILL, stamping verify SIGTERM); decision/no-untied-harness-boot (census `viola_e2e_census_finds_no_untied_boot` read red on a planted untied boot); decision/tied-verify-kill-witness; rstest-case-tables except for viola-e2e/viola-core. A console process spawned by a test must be tied by these rules.

@history_item n=15 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** — decision/wrapper-drop-guard rewritten to a parent-death tie plus Drop guard; + decision/no-untied-viola-spawn (census over every root test source counting any spawn of the viola binary constant outside the tied builder, `cross_killed_test_census_finds_no_untied_viola_spawn`); + decision/tied-child-spawn-thread. A keystroke driver that spawns the console binary in a PTY must go through the tied builder on a thread that outlives the child, or the census reads red.

@history_item n=16 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — root waits/`WITHIN` moved 7 s → 12 s (decision/root-wait-bound, decision/within-7s-wait-bound, whose key keeps the stale «7s»), mutants default kill 10 s → 15 s (decision/mutants-kill-below-cargo-mutants-timeout); + decision/record-home-names-no-home. The console tests' waits sit under these bounds and the scope's 20 s lint.

@history_item n=17 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (kill line; stale-dim witness; Frozen) — §9 gained the `ci` kill line (the stamped-home 20 s override, counted by the `binary(…)` names on `.config/nextest.toml`'s filter line: decision/ci-stamped-home-kill-line); `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` asserts exact SGR sequences under the outer PTY with `TERM=xterm-256color`; `Frozen` moved to `tests/support/frozen.rs`. A new console test binary that starts from `stamped_home` joins the filter line and moves its count; the dim witness is the precedent for asserting a rendering's escape bytes, never the screen.

@history_item n=18 marker=2026-11-27-epoch-5-cleanup
**2026-11-27-epoch-5-cleanup** — decision/verify-window-80s-kill and decision/verify-driven-tests-20s-kill now name members of the class by its filter line one by one; + decision/r2-refuses-before-anything-starts. The latest moves of the kill-line rows a new console binary may join.

@history_item n=19 marker=2026-11-27-epoch-5-cleanup-2
**2026-11-27-epoch-5-cleanup-2** — decision/unit-tests-inline gained the except: a too-large unit-test module splits into sibling `#[path]` files keeping its module path, cases in child modules, helpers beside their only users. The pattern the last wrap set for test-module layout this chunk's new code follows.

@history_item n=20 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — `tests/cli_output_plain.rs` lands: `--help` and a clap usage error under `CLICOLOR_FORCE` into a pipe carry no ESC; under the outer PTY no SGR bold/underline, SGR parameters parsed, never the screen text. Origin of decision/help-no-bold-underline and decision/no-escape-in-plain-output — the CLI's no-redraw/no-SGR rows the redraw-ban reconciliation must scope away from the console surface.

@history_item n=21 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — `viola --help` pinned by the `HELP` literal; the parser `sgr_attributes` (with `redraws`, `sets_a_foreground`) in `tests/support/sgr.rs`; `NO_COLOR`/`TERM`/`COLORTERM` are cases of `setting_does_not_disable_the_send_paste_control`; ratified at 2026-10-20T06-58-31-wrap. The `redraws` helper is the existing test-side witness of the CLI's redraw ban, and the help table (contract/viola-help/verb-table) moves if the console's start verb is a served verb.

@history_item n=22 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — `list` rows escape every control character (DEL, C1, `\n`, `\t`) as `\xHH`, `wait`/`last` keep `\n`/`\t`; the board pinned by literals (`CAPTION`, `list_human_board_is_the_literal_rows`), not trycmd. Origin of decision/list-row-control-escaping and decision/human-layouts-pinned-by-literal; the console's later rows are owed to «Fleet console view», but the scaffold's frame inherits the escaping rule.

@history_item n=23 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — the test-only live rig's screen reads became the named except of decision/no-child-screen-assertions, decision/never-parse-rendered-child-screen, decision/no-screen-parsing and decision/discipline-at-every-layer («closed words by compiled literals»); decision/real-claude-local-only names the rig's live starts; ratified 2026-11-22T17-29-41Z-wrap. The precedent and the trap: any test reading a screen needs a named except; the console driver reads STATE, so it must add none.

@history_item n=24 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (screen excepts) — the four screen-parsing excepts returned to their whole members and the rig's compiled-shapes reading moved to its own row decision/live-rig-reads-compiled-shapes, because the bound could not hold it; decision/human-never-blocked gained the start's verify-window except. Shows how a new reading is placed in its own row rather than lengthening a bound except — the form a console exception (redraw) would take.

@history_item n=25 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (feed panic; readers) — the forced vt100 panic case byte-searches the outer terminal's drained master stream (the except of decision/no-child-screen-assertions, decision/tui-passthrough-never-parses-screen, decision/tui-verdicts-from-records); `tests/contract_bounded_readers.rs` `BOUNDARIES`/`EXCEPTIONS` and the census `product_readers_are_each_bounded_or_named` (decision/bounded-readers). A console reading keys from its terminal or any external input is a new reader the census must find bounded or named.

@history_item n=26 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (chaos home) — the second test-data carve-out: `tests/chaos_feed_panic.rs` alone boots `TestHome::outside_scan()` outside G2/G4/the secret scan, on the founder's ruling. The except of decision/fresh-home-per-test; a console test needs its home inside the scans.

@history_item n=27 marker=2026-10-25-windows-endpoint-admission
**2026-10-25-windows-endpoint-admission** (scoped captures; runner) — every scoped log capture in the root bin's units goes through `crate::test_support::with_scoped_default` (a pinned always-interested Dispatch), the binary's one global install `set_global_default` in `src/run/gate.rs`; the three crate captures left with no owner; nextest the only supported runner. Origin of decision/scoped-capture-pinned-dispatch and context for decision/one-global-tracing-install: a console process's own logging init must keep the one-global-install rule and its unit captures go through the helper.

@history_item n=28 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (diagnostics opens) — a `diagnostics/` folder, role or detail file failing `strict::check_diagnostics` is a failed open, checked before `ensure_private_dir` narrows anything (`open_role_file_refuses_a_widened_diagnostics_path_and_leaves_it`, `write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it`). Origin of decision/diagnostics-checked-before-narrowing and decision/diagnostics-files-owner-only; a new console role file opens through the same path.

@history_item n=29 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (strict checks before any log; test homes) — starts, `verify` and the frame verbs check the home tree before any log (decision/widened-home-tree-refused-at-start, decision/instance-check-before-role-file, decision/home-read-rule); a test that makes a home before viola uses `create_private_dir` (decision/test-homes-made-private); widen helpers in `tests/support/home.rs`; trap: a remove-the-guard witness whose neutralised guard lets a process start must be bounded by `WITHIN` (one hung 27 minutes). A console start joins the strict-modes-before-any-log list and its control needs a bounded witness.

@history_item n=30 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — decision/strict-modes-before-any-log and contract/strict-modes-refusal/no-line gained `viola focus`; + decision/window-raise-proven-live (R3 gated live reading); F-A ratified 2026-11-24T17-14-08Z-wrap. Precedent for adding a new window-feature verb to these closed lists.

@history_item n=31 marker=2026-11-24-viola-up
**2026-11-24-viola-up** — strict-modes lists gained `viola up`'s board scan; + decision/window-reopen-proven-live, decision/live-rig-root-outside-repositories, decision/child-fd-table-read-after-launch-settles; + contract/fake-foot/spawn-record; + decision/stand-in-terminal-tree-teardown (one shared teardown; killing a fake agent mid-exit cut coverage profiles); all ratified 2026-11-27T19-07-11Z-wrap. A console test that spawns a terminal stand-in uses this teardown.

@history_item n=32 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** — contract/cli-human/internal-error's silent list gained adopt's start; contract/strict-modes-refusal/no-line gained adopt; decision/real-claude-local-only names the gated live-reading scripts; decision/verify-driven-tests-20s-kill gained its verify_window_ except. The latest form of the closed lists a console start verb would join.

@history_item n=33 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — the fake agent also stands as `foot`, the terminal stand-in (decision/fake-agent-on-test-path, decision/fake-agent-as-claude, decision/fakes-for-external-services); + decision/window-placement-proven-live; + decision/echo-oracle-skips-allowed-word; + decision/live-rig-clean-start (ratified 2026-11-27T19-07-11Z-wrap). The console's fake side (the terminal it is drawn in, if any) extends these rows; real window proof is live-only.

@history_item n=34 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — the fake agent as `hyprctl` by file name, the test-only recorder `viola-hypr-tap`, decision/ci-never-real-claude extended to hyprctl, fixture hygiene and recorder scrub extended, parser property/fuzz rows grown. The pattern for adding a new external child behind the fake agent with no env seam (decision/fake-agent-argv-only except).

@history_item n=35 marker=2026-11-14-hyprland-exact-frame-bound-pinned
**2026-11-14-hyprland-exact-frame-bound-pinned** — decision/fakes-for-external-services except: viola-hyprland's own unit tests serve hyprctl by a planted shell stub, because cargo-mutants runs only the mutated package's tests. A new console crate meets the same package-scope trap.

@history_item n=36 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (fake-args) — contract/fake-agent-cli/argv-only-options and decision/fake-agent-argv-only: a fake-args file beside the exe goes ahead of argv, argv winning, never the environment. The fake-agent configuration rule the console's fake side inherits.

@history_item n=37 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** (harness target dir; no-op feature) — harness cargo work in `target/harness` with `--features viola/fake-agent`; `viola-e2e` declares a no-op `fake-agent` feature for cargo-mutants' package-scoped runs. Origin of decision/members-declare-fake-agent-feature: a new console workspace member must declare it.

@history_item n=38 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — the Lint row's `cargo check` reads one `-p` per crate in `scripts/sync-crates.txt`, an empty list fails; the sole-root `deny-sync.toml` tokio ban; `viola-e2e` never a tokio-ban root. Origin of decision/sync-crates-without-tokio, decision/sync-crates-compile-without-features (a crate joins the list with its creation), decision/tokio-banned-per-sync-crate: a new sync console crate joins the list.

@history_item n=39 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (CI gates; harness keys) — sync-crate list is four after `viola-channel` left (no listed crate depends on it); `channel-tokio-free.sh`; deny-probes 13/13; viola-e2e takes rmcp `client` only and tokio `rt`+`process` (decision/viola-e2e-rmcp-client-only, decision/tokio-only-in-e2e-crate); `harness_session` `stamp: !opts.unstamped`. The dependency-graph rules a console crate and its test driver must keep.

@history_item n=40 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (deny ban) — serde_json `unbounded_depth` banned with a firing probe, deny-probes 14/14 (decision/serde-json-unbounded-depth-banned, decision/deny-bans-probed). A new terminal dependency ban would add a probe and move this count.

@history_item n=41 marker=2026-10-26-tokio-rustsec-2026-0386-lock-bump
**2026-10-26-tokio-rustsec-2026-0386-lock-bump** — viola-e2e's tokio pin `=1.53.3`, `rt` + `process`, no `test-util`. Constrains any tokio-based console test client to the e2e crate's existing features.

@history_item n=42 marker=2026-09-24-workspace-tree-and-code-graph-planes
**2026-09-24-workspace-tree-and-code-graph-planes** — the orphans gate `scripts/orphans-check.sh` (+ `--probe`) per lib/bin target, `--acyclic` an on-demand review, the release-check job judging `--bin viola` on its own artifact. Origin of decision/no-orphan-modules and decision/release-build-viola-only: console modules must not orphan and a test-only console driver binary must never reach the release build.

@history_item n=43 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** — the orphans probe reads `2/2 fired`; the units moved under `src/cmd/verify/typed/` with names kept (the Epoch 4 `typed.rs` dispositions re-cited); the `Mcp.` exchange recorded by the test-only `viola-mcp-tap`. Where carry `cry-boujwuyr`'s M1 code (`framing_turns`) now lives.

@history_item n=44 marker=2026-10-13-epoch-4-survivors-ii
**2026-10-13-epoch-4-survivors-ii** — the 16 `typed.rs` survivors (`framing_turns` 5, `SharedPty` 5, `Run`'s ending 6) disposed by behaviour-preserving restatements (`added_turn_ended`, `local_command_answered`, `probe_until`, a generic `SharedPty` with a recording double, `ctrl_c_ending`/`ended_by`), each kill shown by hand, no mutation run. Direct precedent for carry `cry-boujwuyr` M1 (one step function for `framing_turns`' per-paste probes): restatements must keep these kills.

@history_item n=45 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — step lines `[NN/MM]`, MM = `LedgerRow::relied()`'s length; `cli_verify`'s per-OS summary literals; `contract_ledger_probes` `ROW_IDS`; `LEDGER_ROWS` in `--local-live`. Every later row landing moved these literals by hand (2026-10-14, 2026-10-16, 2026-10-18, 2026-11-05 entries) — the drift carry `cry-boujwuyr` M3 (the relied-row count from one source) targets; note decision/literal-test-oracles forbids importing the product's ledger rows as the oracle.

@history_item n=46 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (relied set) — `LedgerRow::relied()` is all twenty-four rows on every build; `STEPS_PASS`, `ROW_IDS` and `LEDGER_ROWS` `[&str; 24]`, the literals ungated. The literal set M3 would collapse, against the literal-oracle rule.

@history_item n=47 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — `contract_ledger_probes`' stamped sets are `2.1.287` and `2.1.302`, with the drift-only list as test literals (decision/ledger-probe-contract-suite). The stamped-set table M3 wants held once.

@history_item n=48 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — contract/verify-output/step-line and summary-line read `pass|absent|fail`, absent on the browser-tools-prompt row alone; decision/version-gate-needs-all-relied-rows reads «counting»; contract/local-live-suite/pass the same; ratified 2026-11-22T17-29-41Z-wrap. The current wording the M3 count change must keep.

@history_item n=49 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — + decision/verify-under-test-owned-user-home and decision/no-verify-start-outside-test-home (census); verify-stop removes its own trust record. Console tests that stamp a home inherit these.

@history_item n=50 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — + decision/run-under-test-owned-user-home and decision/harness-wrappers-under-session-user-home, because a start writes the Claude config. The scope's boundary «reads nothing of the operator's real `~/.claude*` or `~/.viola`»: any console test starting a wrapper does so under a test-owned user home.

@history_item n=51 marker=2026-11-21-cross-session-prefix-ledger-row
**2026-11-21-cross-session-prefix-ledger-row** — + decision/stamping-verify-test-owned-runtime-dir (verify reads `XDG_RUNTIME_DIR` once); fixture-drift excepts for send-tool hooks; hygiene reach narrowed to home, drive-letter and user-folder paths. Stamped console test homes take a test-owned runtime dir.

@history_item n=52 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — + decision/stamping-verify-passes-transcript (every stamping verify passes the fake agent `--transcript`). A new stamping helper for console tests must keep it.

@history_item n=53 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — + decision/one-verify-stamp-per-build; decision/fresh-home-per-test: a stamped home is a copy of its build's stamp, never a link; decision/verify-driven-tests-20s-kill's why names the one-stamp-per-build. The fixture chain console tests start from.

@history_item n=54 marker=2026-11-25-placing-without-moving-the-view
**2026-11-25-placing-without-moving-the-view** — + decision/verify-failing-row-from-fixture-set (a failing relied row from a spine fixture set, never by moving the trusted root); decision/window-placement-proven-live names R7. The latest change to the window-proof row the console's Hyprland window will lean on.

@history_item n=55 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — the fake agent placed as `claude` in a test-scoped bin dir answers from files beside its exe, no env var; every test-started `list`/`revive` runs under a test-scoped PATH (`support::cli::spawn_on`/`viola_on`). Origin of decision/tests-never-read-host-sessions and decision/agents-stub-from-files-no-env: a console that reads the board must run under a test-scoped PATH in its tests.

@history_item n=56 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — the completeness case `every_flag_variable_and_config_key_is_a_row_of_the_table` (the `Options:` of `viola --help`, the `VIOLA_*` literals walked from product sources, `schemas/config.v1.json`); a test-started viola drops `VIOLA_NAME`/`VIOLA_DIR`/`VIOLA_BIN` and compares `resolved(home)`. A console start flag, verb or variable must become a row of `cli_controls_not_disableable` (decision/controls-not-disableable, decision/no-switch-disables-a-control); decision/test-started-viola-drops-env applies to the console process.

@history_item n=57 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — a test placing an endpoint in its own directory makes it with `create_private_dir` (decision/test-endpoint-dirs-made-private: a bare `tempfile::tempdir()` is 0755 and refused); `setting_does_not_disable_the_socket_dir_check` runs every `SETTINGS` row; `list` on the read rule. Rules a console that connects to wrappers inherits.

@history_item n=58 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — + decision/supervisor-spawn-strips-bridge-env (the harness wrapper spawn drops `VIOLA_NAME`/`VIOLA_DIR`/`VIOLA_BIN`); boot readiness reads the start records. A console spawned from a harness session must not inherit the bridge env.

@history_item n=59 marker=2026-10-09-epoch-3-cleanup-ii
**2026-10-09-epoch-3-cleanup-ii** — the shared root helpers `tests/support/{home.rs,outer_pty.rs,fake.rs,events.rs,cli.rs,ndjson.rs}`, no root test file defining its own copy (later `frozen.rs`, `start.rs`). Origin of decision/shared-e2e-helpers and decision/one-shared-test-helper: the keystroke driver reuses `outer_pty.rs`/`cli.rs`/`events.rs` rather than copying.

@history_item n=60 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — `tests/cross_exit_causes.rs` (a root `cross_` binary on the kill line) with `no_two_codes_share_a_hint`, `no_hint_names_release`, `every_code_is_a_code_word`; `tests/support/start.rs` shared start helpers; `json_asked` reads `--json` only before `--`; the cli catch site's internal-error document. Origin of decision/each-exit-cause-own-hint, decision/no-release-in-driver-hints, contract/cli-arguments/json-flag, contract/cli-human/internal-error — a console exit path joins this catalogue.

@history_item n=61 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — no after-every-test events helper; error bodies held by the three surface witnesses and `tests/contract_error_bodies.rs`' census `product_error_writers_are_each_named`; `v1-47`'s scan after each job's E2E run. Origin of decision/error-bodies-held-by-surface-witnesses, decision/error-writers-each-named, decision/error-bodies-hold-no-path: any console error writer must be named in the enumeration.

@history_item n=62 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** — `run --e2e` built; the E2E regex `^(path[0-9]*|tui|mcp|cross|chaos|contract)_`, the integration layer its complement; `harness_session` a plain function and drop guard. Origin of decision/e2e-layer-by-binary-prefix and decision/test-layer-filtersets: a console test binary's name decides its layer (a `tui_` prefix lands in E2E).

@history_item n=63 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (naming) — the root sync E2E suites read `tests/{cli,hook,tui,channel,chaos,contract,security_negatives}_<topic>.rs` (decision/integration-binary-surface-prefix); fake-agent copies planted one after another (ETXTBSY). Naming rule for the console's test files.

@history_item n=64 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — internal `schema-check` and `secret-scan` with closed class enum; scan-gated `harness-`, `diag-`, `junit-<os>` uploads. Origin of decision/diagnostics-schema-validation, decision/secret-scan-scope, contract/ci-artifacts/diag: a console role file's lines are schema-checked (diag-line.v1) and scanned like every role file.

@history_item n=65 marker=2026-09-26-ci-chunk-base-and-union-verdict
**2026-09-26-ci-chunk-base-and-union-verdict** — the secret scan covers every file under `target/agent-run/` except exactly `target/agent-run/chunk.diff`. The except of decision/secret-scan-scope.

@history_item n=66 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** — the secret scan's cookie class retired (decision/logs-carry-no-secrets, decision/no-secrets-in-logs, decision/secrets-never-in-logs lost the Cookie clause); + decision/os-gated-cases-in-binary-root. The current text of the secret rows a console role is held to.

@history_item n=67 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — no per-chunk, pre-push or CI mutation gate; `run --mutants` named-only, mutation at the epoch boundary. Behind contract/test-harness/run and the scope's «no mutation run»: kills of new console code are shown by hand, read at the next boundary audit.

@history_item n=68 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (one shim) — `scripts/agent-run.sh` the one thin shim; `Frozen` a SIGSTOP guard; `SOCKET_ROOT_VAR` reads `XDG_RUNTIME_DIR`. Behind decision/harness-logic-in-rust: any console harness affordance lives in the Rust harness, not the shim.

@history_item n=69 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — `target/e2e-home` on the dev host is a link to tmpfs, prepared by `ensure_e2e_home`/`prepare_home_base`; a stalled-start red is a finding about the backing, never re-run for green. Console test homes live there.

@history_item n=70 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — the pump's resize baseline is the spawn size (forced-window via the `fake-agent`-only `FAKE_AGENT_PUMP_DELAY_MS` seam). Origin of decision/resize-baseline-is-spawn-size; the quarter-screen frame's degrade rule meets resizes on this seam.

@history_item n=71 marker=2026-09-29-h2-conpty-resize-probe
**2026-09-29-h2-conpty-resize-probe** — a test that writes a key after a resize sends it only once the child reports the new size, never a timer, retry or skip. Origin of decision/key-after-resize-waits-for-size: a keystroke driver testing the frame at a smaller size waits for the size report before its key.

@history_item n=72 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — decision/pty-tested-on-real-openpty now reads a resize from the child's own size report alone, no key written after it. The current form of the PTY rule the driver follows.

@history_item n=73 marker=2026-11-21-key-after-a-resize
**2026-11-21-key-after-a-resize** — decision/key-after-resize-waits-for-size gained the live rig KR arm except (ratified 2026-11-22T17-29-41Z-wrap); + decision/refusal-rehearsal-reads-own-effects. The only sanctioned key-right-after-resize case; the console driver gets no such except.

@history_item n=74 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — `fs::place_private_new` no-clobber, mode-first placement and its forced-window witness through a test parameter (no env, feature or seam). Origin of decision/private-placement-mode-first; the form a console's test parameter takes instead of a seam.

@history_item n=75 marker=2026-11-20-harness-prefix-and-identity-floor-ledger-rows
**2026-11-20-harness-prefix-and-identity-floor-ledger-rows** — the identity floor grew to thirteen names, rows stop counting them (decision/r8-claude-env-strip, decision/claude-env-strip, decision/literal-test-oracles); + decision/strip-control-oracle-keeps-guarded-names. A console that starts or wraps agents keeps the strip; its tests hold literal oracles.

@history_item n=76 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** — decision/send-refusal-order gained the two paste-cut rungs; contract/viola-send/paste-cut and during-a-blocked-paste; contract/send-event-log/send-issued; ratified 2026-11-22T17-29-41Z-wrap. The current form of the send rows the picked set carries (owed to «Console actions», not built here).

@history_item n=77 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows (decision · contract · absence), each verified against the code, the fates by the founder. Explains every picked row's form: a surface section the scaffold writes lands as rows, and rows owed to «Fleet console view»/«Console actions» are named as owed, never as built.
