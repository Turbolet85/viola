proposals:
  - detector: D-tests-coverage
    severity: warning
    section: §4 Unit Test Strategy (viola-agent-claude, the Screen signatures bullet)
    change: "SIGNATURES names two input-box literals, `for agents` and `(shift+tab to cycle)` (the modal literals unchanged); the 2.1.302 resting footer is pinned in `screen.rs` by `holds_any_reads_the_resting_footer` (8 rows: the measured row and the 2.1.287 row true; the rule, blank, tip and Skills-hint rows false), `verdict_over_the_2_1_302_resting_footer_alone_is_ready` and `verdict_over_the_resting_footer_beside_a_modal_is_input_not_ready`; the rest of the bullet stands."
    sidecar: "§4 viola-agent-claude: SIGNATURES.input_box is two compiled literals (`for agents`, `(shift+tab to cycle)`), with the three resting_footer unit cases (chunk 2026-10-19-newer-claude-cli-rows)."
    rationale: "Report Symbols / APIs: `SIGNATURES.input_box` `[for agents]` -> `[for agents, (shift+tab to cycle)]` and the new unit cases at screen.rs:449-475; Spec claims disproved #1 and Counts F1-literal name test-plan.md:508 as stale; Deviations run 2 item 2 names the rule, blank, tip and Skills-hint rows read false."
    basis: "test-plan.md:508; crates/viola-agent-claude/src/screen.rs:41, :449-475"
  - detector: D-tests-coverage
    severity: warning
    section: §4 Unit Test Strategy (viola-agent-claude, beside the ledger arms bullets)
    change: "Add a bullet: the `input-box-signature` arm (`check(InputBoxSignature)`) reads three Run B screens, `ready`, `turn` and `long_turn` (`TypedRun::long_turn`, the rows verify settles on after the long paste's turn, `None` when that turn's box wait fails or the turn never ends; never written to `measured`), each required to hold a compiled input-box literal and no modal; its row words unchanged; cases `check_input_box_signature_long_turn_holding_the_literal_passes`, `check_input_box_signature_long_turn_holding_a_modal_fails` and `check_input_box_signature_long_turn_of_none_fails` (`ledger.rs`)."
    sidecar: "§4 viola-agent-claude: the input-box-signature arm reads ready + turn + the long turn's settled screen, three long_turn unit cases (chunk 2026-10-19-newer-claude-cli-rows)."
    rationale: "Report Symbols / APIs: `TypedRun::long_turn` (ledger.rs:603-604, :610), `check(InputBoxSignature)` reading ready, turn and long_turn (ledger.rs:1011-1013), the unit cases at ledger.rs:2429-2467, typed.rs `framing_turns` keeping the screen; Counts F2-ibsig; Expected amendment 2 carries the long_turn cases into test-plan §4."
    basis: "crates/viola-agent-claude/src/ledger.rs:1011-1013, :2429-2467"
  - detector: D-tests-coverage
    severity: warning
    section: §5 Integration Test Strategy (CLI, the `tests/cli_verify.rs` bullet)
    change: "Beside the tag-turn modal case, add the long-turn modal case: Run B pastes nothing more once the screen drawn after the long paste's turn alone shows a modal (`19 pass  5 fail` on Unix, `16 pass  5 fail` on Windows, the receipt holding the probe prompt and the long paste only; the fake agent's `--long-turn-screen`; `verify_pastes_nothing_more_once_the_long_turn_screen_shows_a_modal`)."
    sidecar: "§5 CLI: cli_verify gains the long-turn-screen modal case and the fake agent's --long-turn-screen (chunk 2026-10-19-newer-claude-cli-rows)."
    rationale: "Report Symbols / APIs: the new case at tests/cli_verify.rs:1486-1542 and the fake agent's `--long-turn-screen`; Outcome (arch) gives its 19 pass 5 fail (Unix) / 16 pass 5 fail (Windows) and the receipt; Counts F4-fakeopt (@c2053) and F8-repro (@c2072) name test-plan.md:589 stale by omission."
    basis: "test-plan.md:589; tests/cli_verify.rs:1486-1542"
  - detector: D-tests-coverage
    severity: warning
    section: §5 Integration Test Strategy (CLI, the `contract_ledger_probes.rs` bullet)
    change: "For each stamped set (`fixtures/claude/2.1.287/` and `fixtures/claude/2.1.302/` today; the `STAMPED` literal `[2.1.287, 2.1.302]`), viola verify ... must end `stamped <ver>  <n> pass  0 fail`; the rest of the bullet stands."
    sidecar: "§5 CLI: contract_ledger_probes' stamped list is 2.1.287 and 2.1.302 (chunk 2026-10-19-newer-claude-cli-rows)."
    rationale: "Report Symbols / APIs: `tests/contract_ledger_probes.rs:69` `STAMPED` is `[2.1.287, 2.1.302]`; Schema / config: the new recorded set `fixtures/claude/2.1.302/` at `24 pass  0 fail`; Counts F3-stamp names test-plan.md:601 stale."
    basis: "test-plan.md:601; tests/contract_ledger_probes.rs:69"
  - detector: D-tests-coverage
    severity: warning
    section: §7 Test Data & Fixtures (Fake agent, the Modes bullet)
    change: "The fake agent's argv options grow by one (thirteen; the eleven that follow, nine of them for verify's interactive runs and their tests), and the one test-only screen switch becomes two: beside `--tag-turn-screen <phase>`, `--long-turn-screen <phase>` (with `--framing` and `--turn-stop`: after the long paste's turn alone, after its Stop, the agent draws `Screen.<phase>.json` of its set in place of `Screen.turn`; `cfg(feature = fake-agent)`; parse cases `opts_long_turn_screen_takes_the_phase_it_names_alone` and `opts_long_turn_screen_defaults_to_none`). The count's rule stays architecture's list of them."
    sidecar: "§7 Fake agent: --long-turn-screen joins --tag-turn-screen as the second test-only screen switch (chunk 2026-10-19-newer-claude-cli-rows)."
    rationale: "Report Symbols / APIs: the fake agent's test-only `--long-turn-screen <phase>` (src/bin/viola-fake-agent.rs:99, :144, :670-671, :722-725, :736-741), acting as `--tag-turn-screen` does after the tag-like turn, parse cases at :1328-1342; Counts F4-fakeopt (1 -> 2) names test-plan.md:960 @c3290 stale by omission; Harness / gate surface lists it."
    basis: "test-plan.md:960; src/bin/viola-fake-agent.rs:1328-1342"
  - detector: D-tests-coverage
    severity: warning
    section: §7 Test Data & Fixtures (Fixture hygiene, the walk bullet)
    change: "The walked hook-payload sets are `2.1.283`, `2.1.287`, `2.1.288` and `2.1.302`: the four spine payloads in each, the 12 recorded dialog variants under each of `2.1.287`, `2.1.288` and `2.1.302`, the six recorded framing variants and the one resume variant under each of `2.1.287` and `2.1.302`; the two status line variants and the one agents fixture of `2.1.287` and `2.1.302`; and the `Screen.{modal,ready,turn}.json` screens of `2.1.287`, `2.1.288` and `2.1.302`; schemas and planted cases unchanged."
    sidecar: "§7 Fixture hygiene: the walk covers the recorded 2.1.302 set (29 files) beside 2.1.283, 2.1.287 and 2.1.288 (chunk 2026-10-19-newer-claude-cli-rows)."
    rationale: "Report Schema / config: the new recorded set `fixtures/claude/2.1.302/` (29 files: spine, dialog, framing, status line, resume, compact, agents and screen variants), hygiene-green (Outcome, Gated); Counts F3-stamp and F7-hygiene name test-plan.md:968 stale by omission; the 29 files are listed under New text, by line."
    basis: "test-plan.md:968; report New text, by line: fixtures/claude/2.1.302/ (29 new files)"
  - detector: D-tests-coverage
    severity: warning
    section: §7 Test Data & Fixtures (Seed strategies table, the Recorded hook payloads row)
    change: "The framing variants (six), the status line variants (two), the resume variant (one) and the agents fixture (one) are recorded under `2.1.287` and `2.1.302`, no longer under `2.1.287` only; the 2.1.287 file count stands as written."
    sidecar: "§7 Seed strategies: the recorded framing, status line, resume and agents variants stand under 2.1.287 and 2.1.302 (chunk 2026-10-19-newer-claude-cli-rows)."
    rationale: "Same retired claim as the hygiene-walk proposal (2.1.287 the only recorded full set): report Schema / config records `fixtures/claude/2.1.302/` with the framing, status line, resume, compact and agents variants; the row's four `under 2.1.287 only` phrases are now false. The report's sweep keyed on `stamped` and gives no line here."
    basis: "report Schema / config; New text, by line: fixtures/claude/2.1.302/"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §2 Test Strategy (Test pyramid table, the Contract row)
    change: "The dialog-tier variants number 12 under each of `2.1.288`, `2.1.287` and `2.1.302`, and the framing-tier variants six under each of `2.1.287` and `2.1.302` (none under `2.1.288`); the rest of the row stands."
    sidecar: "§2 Contract row: the dialog and framing variant counts include the recorded 2.1.302 set (chunk 2026-10-19-newer-claude-cli-rows)."
    rationale: "Same retired claim (2.1.287 and 2.1.288 the only recorded dialog sets, 2.1.287 the only framing set): report Schema / config and New text, by line list 12 dialog files (PermissionRequest 2, PreToolUse 5, PostToolUse 5) and six framing files under fixtures/claude/2.1.302/. No line is given in the report."
    basis: "report New text, by line: fixtures/claude/2.1.302/"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §5 Integration Test Strategy (beside the `tests/support/home.rs` boot helpers)
    change: "Add: `tests/support/home.rs` `wait_ready` writes a stall report at its deadline, before the panic: the stalled wrapper's last `run-<name>.ndjson` event; on macOS `/usr/bin/sample <pid> 1`'s call-graph lines (bounded at 4 s, at most 120 lines); on Linux `/proc/<pid>/wchan` and the `stat` state (`note_stall`, `wrapper_pid` by one sysinfo read, `process_state` per OS); it signals nothing and adds nothing to a passing boot; it is the witness for the macOS readiness stall, whose phase is not yet named."
    sidecar: "§5: wait_ready's test-side stall report (last run-<name> event, macOS sample, Linux wchan/stat) as the macOS readiness stall's witness (chunk 2026-10-19-newer-claude-cli-rows)."
    rationale: "Report Symbols / APIs (Test support, home.rs:645-664, :896-911, :913-970) and Counts F5-stall: new, no master states it, its home is test-plan; Expected amendment 2 carries the stall report in wait_ready; Outcome: the macOS readiness criterion UNMET, the stall report its witness."
    basis: "tests/support/home.rs:645-664, :896-911, :913-970"
note: "D-tests-framework and D-tests-obs-harness find no drift (nextest/rstest unchanged; the report states no agent-run verb, status shape, CI step or log format changed). The macOS readiness stall left UNMET and routed to a WATCH sits against §10's zero-flakiness budget (a flake keeps the chunk red until its root cause is fixed within it): no Change edits §10, so this is for the orchestrator's judgment, not a proposal. §7's fake-agent --resume launch clause names `2.1.287, 10 key names` as the set holding the resume variant; 2.1.302 now holds one too, but the report gives no key count for it, so no proposal is made."
