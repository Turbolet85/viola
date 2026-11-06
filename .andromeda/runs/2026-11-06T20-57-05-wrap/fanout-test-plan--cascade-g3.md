@proposal n=1 detector=D-cascade severity=escalate file=e2e-test-strategy anchor=fct-4m5t4osf basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-ngwjb7xx
section: ## 6. E2E Test Strategy
change: In the cli (verbs + `--json`) Driver cell, replace «trycmd 1.2.1 for human layouts other than the `viola list` board, `viola --help` and the `viola link` / `viola unlink` result lines, pinned by literal in …» with the literal pins: «assert_cmd 2.2.2 + predicates 3.1.4; human layouts pinned by literal in their test files (the `viola list` board in `tests/cli_list.rs`, `viola --help` in `tests/cli_output_plain.rs`, the link lines in `tests/cli_links.rs` `link_verbs_print_the_design_lines`)», no trycmd case file named.
sidecar: e2e-test-strategy#1.1 — the cli driver's trycmd wording re-worded to the literal pins (CARRY 1, version done-check).
rationale: the report's expected amendment 10 carries «the trycmd/assert_cmd text re-worded to the literal pins (CARRY 1)» over test-plan §6; the exact form of the pins is the orchestrator's to confirm, hence escalate.
@proposal n=2 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-tr6c4sml basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-xc746owh
section: ## 6. E2E Test Strategy
change: In the tui (`viola run` passthrough) Driver cell, «portable-pty `=0.8.1` outer PTY» reads «portable-pty `=0.9.0` outer PTY»; the rest of the row stands (openpty, exit only via `child.wait()`).
sidecar: e2e-test-strategy#1.3 — the outer-PTY driver's portable-pty pin moved =0.8.1 → =0.9.0 (version done-check).
rationale: Dependencies moves `portable-pty` `=0.8.1` → `=0.9.0`, Retired names the `=0.8.1` pin; Outcome (pty) reads viola-pty 33/33 and the outer-PTY e2e binaries green on `=0.9.0`, `pty_backend()` `openpty`.
@proposal n=3 detector=D-cascade severity=escalate file=e2e-test-strategy anchor=fct-qbytqn26 basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-ngwjb7xx
section: ## 6. E2E Test Strategy › #### Scenario: Path 2 — confirmed `send` with CL-1 events
change: In the «Human mode» bullet, «verified with trycmd-redacted assert_cmd» reads «verified with assert_cmd against literal lines in the test file» (the literal pins); the lines it lists stand.
sidecar: e2e-test-strategy#3.3 — Path 2's human-mode check re-worded off trycmd to the literal pins (CARRY 1, version done-check).
rationale: the report's expected amendment 10 re-words the trycmd/assert_cmd text of test-plan §6 to the literal pins (CARRY 1); the exact wording is the orchestrator's to confirm, hence escalate.
@proposal n=4 detector=D-cascade severity=warning file=e2e-test-strategy anchor=fct-ja62ftpj basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-q2lwjk3d
section: ## 6. E2E Test Strategy › #### Scenario: E2 — R8 `CLAUDE*` env strip
change: In the third bullet, the mechanism reads «portable-pty closes every inherited descriptor in the child (`close_random_fds()`, every fd above 2 at spawn, re-read on `=0.9.0`), so a planted leak cannot go red either way», and a clause follows: «As measured on live `claude` 2.1.287 on the Linux dev host at chunk 2026-11-06-version-done-check (`evidence/live-round.md`; a local live measurement, no suite): the wrapped child's `/proc/<child>/fd` held 31 fds (pty 7, socket 14, other 10), fds 0–2 on the pty, no socket that is viola's endpoint or the wrapper's, and no file under the home.»
sidecar: e2e-test-strategy#9.2 — the fd-inheritance mechanism worded as the founder ruled and the 0.9.0 live child fd table recorded (version done-check).
rationale: the report's expected amendment 6 sets the wording «portable-pty closes every inherited descriptor in the child» (`close_random_fds()`), witnessed by E2 and the live table (`inputs#I7`); Cross-project claims record session 2's fd table on 2.1.287.
@proposal n=5 detector=D-cascade severity=warning file=e2e-test-strategy anchor=fct-hhwc4gim basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-ngwjb7xx
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: Remove the sub-bullet «Its two controls, on test-owned `CreateNamedPipeW` pipes read through the same client-end instrument, … `security_negative_control_reject_remote_pipe_reads_the_bit_set` (`PIPE_REJECT_REMOTE_CLIENTS`) reads it set.» under Single writers; the rest of the Security control negatives entity stands.
sidecar: e2e-test-strategy#15.1 — the Windows named-pipe controls removed from the security control negatives (Linux only, version done-check).
rationale: the report's expected amendment 10 names fct-hhwc4gim among test-plan's Windows wording to move; the product and its tests are Linux only, and the named-pipe controls describe no Linux test.
@proposal n=6 detector=D-new-detail severity=warning file=e2e-test-strategy anchor=fct-sn6e6lxw fact=F1 basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-zmvefc57
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: In the first sub-bullet, the comparator clause reads «its `drift` comparator names a planted order swap and a planted one-byte fixture difference, and `dialog_drift_names_a_changed_annotation` plants a change to a forwarded `annotations` byte of `synthetic_dialog_set` and reads `dialog_drift` red (chunk 2026-11-06-version-done-check)».
sidecar: e2e-test-strategy#19.1 — the contract suite gains the planted annotation-drift witness `dialog_drift_names_a_changed_annotation` (version done-check).
rationale: Symbols / APIs names the new test `tests/contract_fake_agent_drift.rs::dialog_drift_names_a_changed_annotation` (lines 807-839); Coverage of new surfaces places it at tests e2e (contract binary).
@proposal n=7 detector=D-cascade severity=escalate file=test-data-fixtures anchor=fct-6wjlyq3w basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-ngwjb7xx
section: ## 7. Test Data & Fixtures
change: In the Human-mode expected output row, drop «trycmd `tests/cmd/*.toml`, with snapbox redactions for timestamps, cursors and paths;» so the row reads that the human layouts are literals in their test files (the literal pins), each named literal and table kept as it stands.
sidecar: test-data-fixtures#1.11 — the human-mode store re-worded off trycmd case files to the literal pins (CARRY 1, version done-check).
rationale: the report's expected amendment 10 re-words the trycmd text of test-plan §7 to the literal pins (CARRY 1), and amendment 3 removes architecture's `cmd/*.toml` trycmd line; the exact wording is the orchestrator's to confirm, hence escalate.
@remark
The same CARRY 1 trycmd re-wording reaches body facts outside this group (§5 fct-klrvzf5j, §6 fct-gnzxb6ex, §7 fct-puo2hazj, §2 fct-gcer5i4i, §5 fct-w5xcngvx); the contract body line fct-q6ltykaz and trigger fct-fvjvvfmg may take a short F1 clause; F2 is test-scope-summary's, not this group's.
