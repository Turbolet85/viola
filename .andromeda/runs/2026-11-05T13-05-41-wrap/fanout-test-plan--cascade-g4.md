@proposal n=1 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-4m5t4osf basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-iol4iyl6
section: ## 6. E2E Test Strategy
change: In the cli row's Mode column, drop «; on windows-2025 the parsed attribute set», and change «under the outer PTY on `ubuntu-latest` and `macos-latest` (`cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal`, `tests/cross_list_rows.rs`, `cfg(unix)`; `list` row integrity)» to «under the outer PTY on `ubuntu-latest` (`cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal`, `tests/cross_list_rows.rs`, ungated since chunk 2026-11-05-linux-only-code-and-ci; `list` row integrity)»; the rest of the row stays as it is.
sidecar: e2e-test-strategy cli row — the dim-stale witness runs on ubuntu-latest, ungated; the windows-2025 attribute-set arm and the macos-latest leg are gone (Linux-only, multi-os-final).
rationale: The report's deviation 11 makes cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal unconditional, and its Retired bullet drops the windows-2025 and macos-latest CI legs.
@proposal n=2 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-tr6c4sml basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## 6. E2E Test Strategy
change: In the tui row, the Mode reads «direct (openpty)», and the panic witness reads «a byte search of what the drained master reader read (the child's own bytes reach it, and no viola literal appears)»; drop «on Unix» and «on every OS».
sidecar: e2e-test-strategy tui row — mode openpty alone; the panic witness loses its Unix / every-OS split (ConPTY retired, Linux-only).
rationale: The report's Counts give pty_backend's words as 3 → 1 (openpty), and its Retired bullet retires the ConPTY sideload and the Windows / macOS target. The witness's Unix half is now the whole claim.
@proposal n=3 detector=D-retired severity=warning file=body anchor=fct-ktlsbc3h dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. E2E Test Strategy
change: The tui driver line's parenthesis reads «(the child's own bytes, no viola literal)», dropping «on Unix» and «on every OS».
sidecar: §6 tui driver broad line — the panic byte-search witness loses its Unix / every-OS split (Linux-only).
rationale: This is the broad line that e2e-test-strategy#1.3 details. The same OS split leaves with the Windows and macOS target (report Retired).
@proposal n=4 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-4khqx2uf basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. E2E Test Strategy › #### Scenario: Path 1 — `run` start sequence
change: The settings.json bullet reads «The restart rewrites the instance's `settings.json` whole at 0600 as `{"statusLine":{"type":"command","command":"<pinned path> hook statusline"}}` with the absolute pinned path, and only when every character of that path is an ASCII letter, a digit or one of `_ - . / :`. Landed as `run_rewrites_the_settings_override_each_start` (`tests/cli_instance_state.rs`) and the unit `statusline_override_of_a_start_is_rewritten_whole_at_owner_only` (`src/cmd/run.rs`).» This drops «The `settings.json` half of step 5 is Unix's. On Unix», «On Windows no `settings.json` is written.» and «, with a `cfg!(windows)` branch». In the socket-directory bullet, «On Unix the start checks» becomes «The start checks».
sidecar: e2e-test-strategy Path 1 signal — the settings.json override is unconditional; the Windows no-write clause and the cfg!(windows) branch are gone (Linux-only).
rationale: The report's Retired bullet and Symbols / APIs (WRAPPED_HERE folded, the OS arms removed, tests/cli_instance_state.rs among the changed files) leave no Windows arm of the start's settings.json write.
@proposal n=5 detector=D-retired severity=warning file=body anchor=fct-mwv2sqc7 dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. E2E Test Strategy › #### Scenario: Path 1 — `run` start sequence
change: The Path 1 broad signal ends «… never a PATH lookup, and the `settings.json` override», dropping «on Unix».
sidecar: §6 Path 1 broad line — the settings.json override is no longer qualified to Unix (Linux-only).
rationale: This is the broad line that e2e-test-strategy#2.2 details. The Windows no-override arm left with the Windows code (report Retired).
@proposal n=6 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-n6muav3b basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. E2E Test Strategy › #### Scenario: Path 2 — confirmed `send` with CL-1 events
change: «covers cli, ipc-internal (the wrapper channel) and tui (the receipt) on all three CI OSes» becomes «covers cli, ipc-internal (the wrapper channel) and tui (the receipt) in CI on `ubuntu-latest`»; the rest stays.
sidecar: e2e-test-strategy Path 2 surfaces — coverage on ubuntu-latest, not three CI OSes (Linux-only, multi-os-final).
rationale: The report retires the windows-2025 and macos-latest CI legs. The CI matrix is [ubuntu-latest] (Counts: win-ci).
@proposal n=7 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-qbytqn26 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. E2E Test Strategy › #### Scenario: Path 2 — confirmed `send` with CL-1 events
change: In the newline-ending bullet, «on all three CI OSes» becomes «on `ubuntu-latest`»; drop «; its Windows reading is owed to "Windows-only live measurements"» from the paste-newline parenthesis; in «Not measured:» drop «an LF typed inside a paste on Windows,»; the rest of the bullet stays.
sidecar: e2e-test-strategy Path 2 signal — the three-OS claim reads ubuntu-latest; the Windows paste-newline reading owed to the retired Windows-only live measurements entry and its Windows not-measured item leave with the Windows claim.
rationale: The report's Retired bullet retires the Windows target and the route entry «Windows-only live measurements» (wolm sites are re-pointed or retired with the Windows claim they owe, report Expected amendments). A Linux-only product owes no Windows reading.
@proposal n=8 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-mdqnbyad basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. E2E Test Strategy › #### Scenario: Path 3 — `wait` / `last` event-driven readback (§6 live flow)
change: «covers cli and ipc-internal (the wrapper channel) on all three CI OSes» becomes «covers cli and ipc-internal (the wrapper channel) in CI on `ubuntu-latest`»; the rest stays.
sidecar: e2e-test-strategy Path 3 surfaces — coverage on ubuntu-latest, not three CI OSes (Linux-only).
rationale: The report retires the windows-2025 and macos-latest CI legs. The matrix is [ubuntu-latest].
@proposal n=9 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-lhsyucqg basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 6. E2E Test Strategy › #### Scenario: Path 3 — `wait` / `last` event-driven readback (§6 live flow)
change: The bullet reads «`tests/run_cli.rs` `run_child_ends_when_its_wrapper_is_terminated` terminates the wrapper (`kill -KILL <wrapper pid>`, its exit asserted) and asserts that the child, its pid and start time read from `snapshot_data`, is gone within `WITHIN`.» This drops «on every OS» and the Windows `OpenProcess` + `TerminateProcess` arm.
sidecar: e2e-test-strategy terminated-wrapper case — kill -KILL alone; the Windows TerminateProcess arm is gone.
rationale: The report's Symbols / APIs (Test support) says tests/run_cli.rs keeps `kill -KILL` only, and windows-sys is removed.
@proposal n=10 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-p2i2onck basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. E2E Test Strategy › #### Scenario: Path 4 — dialog → `answer` (question / permission / plan)
change: «covers cli and ipc-internal (the wrapper channel) on all three CI OSes» becomes «covers cli and ipc-internal (the wrapper channel) in CI on `ubuntu-latest`»; the rest stays.
sidecar: e2e-test-strategy Path 4 surfaces — coverage on ubuntu-latest, not three CI OSes (Linux-only).
rationale: The report retires the windows-2025 and macos-latest CI legs. The matrix is [ubuntu-latest].
@proposal n=11 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-3r2a46vd basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. E2E Test Strategy › #### Scenario: Path 5 — human takes the wheel, refusal, `release`
change: «cover tui, cli and ipc-internal (the wrapper channel) on all three CI OSes; on `windows-2025` the focus case asserts the platform fact of the founder's live ruling F-W3 (an injected mouse report takes the wheel, focus reports are swallowed), Unix the full assertion.» becomes «cover tui, cli and ipc-internal (the wrapper channel) in CI on `ubuntu-latest`, the focus case holding the full assertion.»; the rest stays.
sidecar: e2e-test-strategy Path 5 surfaces — ubuntu-latest alone; the windows-2025 F-W3 platform-fact arm of the focus case leaves with the Windows leg.
rationale: The report's Retired bullet drops the windows-2025 leg, and tests/tui_wheel.rs is among the files that lost their Windows arms (report Changes, Files).
@proposal n=12 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-kn7ryhs7 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-mjkivgjx
section: ## 6. E2E Test Strategy › #### Scenario: Path 6 — budget governor
change: The first signal bullet reads «`budget.json` holds 91. Statusline stdout equals the marker byte-for-byte, with exit 0 and empty stderr: `run` writes the instance's `settings.json` override, and the arm runs the recorded command as `/bin/sh -c <command>` (`shell_argv` returns `[&str; 3]`) under a 5 s bound (PROVISIONAL). `budget.json` is written only when the payload holds a `rate_limits` object.» This drops «The marker equality is Unix's: there», the whole Windows sentence («`override_document` and `shell_argv` return `None` for every input») and «holds on every OS».
sidecar: e2e-test-strategy Path 6 signal — the statusline pass-through runs on every build; shell_argv returns [&str; 3]; the Windows no-command arm is gone.
rationale: The report's Symbols / APIs and deviation 3 say statusline::shell_argv returns [&str; 3] (it was Option, always Some once WRAPPED_HERE was folded), so no build returns None for it.
@proposal n=13 detector=D-retired severity=warning file=body anchor=fct-jtonrpse dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-mjkivgjx
section: ## 6. E2E Test Strategy › #### Scenario: Path 6 — budget governor
change: The Path 6 broad signal reads «`budget.json` holds the fed reading and the statusline output passes through byte for byte; …», dropping «, on Unix,».
sidecar: §6 Path 6 broad line — the statusline pass-through is not limited to Unix (Linux-only, shell_argv unconditional).
rationale: This is the broad line that e2e-test-strategy#7.2 details. The pass-through now runs on every build (report deviation 3).
@proposal n=14 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-kylcwwrw basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. E2E Test Strategy › #### Scenario: Path 6 — budget governor
change: «green on the three CI runners» (first sentence) becomes «green in CI», and «green on all three CI runners, `test (windows-2025)` included» becomes «green in CI»; «on Unix the statusline arm prints the marker output and files the marker, and on Windows (`cfg(not(unix))`, run on `windows-2025` in CI) it prints nothing and files no marker» becomes «the statusline arm prints the marker output and files the marker»; «(green on the three CI runners, run 38098238731)» becomes «(green on that run's three CI runners, run 38098238731, before the Linux-only cut at `multi-os-final`)»; the rest stays.
sidecar: e2e-test-strategy Path 6 as-landed — the three-runner greens read as CI or as a dated reading; the Windows cfg(not(unix)) no-marker arm is gone.
rationale: The report retires the windows-2025 and macos-latest legs. A dated measurement on the retired matrix is re-worded as history and not left as current (report Counts, win-ci class).
@proposal n=15 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-5eczp6gf basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## 6. E2E Test Strategy › #### Scenario: Path 7 — unverified CLI version gate
change: «(green on all three CI OSes, as measured at ci#40439188053)» becomes «(green on that run's three CI OSes, as measured at ci#40439188053, before the Linux-only cut at `multi-os-final`; CI is now `ubuntu-latest` alone)».
sidecar: e2e-test-strategy Path 7 as-landed — the three-OS green reads as a dated reading of the retired matrix.
rationale: The report's Counts (win-ci) say a dated measurement on windows-2025 / macos-latest is re-worded as history and never left as current.
@proposal n=16 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-ja62ftpj basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. E2E Test Strategy › #### Scenario: E2 — R8 `CLAUDE*` env strip
change: In the first bullet, «On Unix a `claude_env_keep` name survives.» becomes «A `claude_env_keep` name survives.»; in the third bullet, «On Unix, the child holds nothing of viola's» becomes «The child holds nothing of viola's», and the closing sentence «On Windows, handle non-inheritance is asserted at the product unit boundary.» is dropped.
sidecar: e2e-test-strategy E2 signal — the Windows handle non-inheritance clause is retired; the keep-name and fd clauses lose their Unix qualifier (registry reader gone, Linux-only).
rationale: The report's Retired bullet retires the Windows named-pipe controls and the registry environment reader. Symbols / APIs keeps claude_env_keep as the one persistent-environment source.
@proposal n=17 detector=D-retired severity=warning file=body anchor=fct-tenptmlu dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. E2E Test Strategy › #### Scenario: E2 — R8 `CLAUDE*` env strip
change: The E2 broad signal reads «… carries the strip counts and names only, a `claude_env_keep` name survives and the child holds no viola socket or home path, …», dropping «on Unix».
sidecar: §6 E2 broad line — the keep-name and fd clauses lose their Unix qualifier (Linux-only).
rationale: This is the broad line that e2e-test-strategy#9.2 details. The Windows arms left with the code (report Retired).
@remark
F1 (the census's another-OS fault class) has no carrier among g4's picked entities; it belongs beside the §6 Contract suite body line fct-q6ltykaz, for the new-detail pass. Other §6 body lines still name Windows or macOS outside this group's picked set: the skipped list fct-lkgqv65p, fct-dcxdz6xw and fct-x7oaiapn; the Security sweep fct-4aawmv2h; the Exit-cause matrix fct-ndx7lm2p; the Security control negatives fct-4uc5ehox; the Chaos `.cmd` clause fct-47fi4ugg.
