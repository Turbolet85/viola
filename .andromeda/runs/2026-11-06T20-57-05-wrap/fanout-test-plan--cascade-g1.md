@proposal n=1 detector=D-new-detail severity=warning file=test-scope-summary anchor=fct-zgqp2syf fact=F2 basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-npz56o6m
section: ## 1. Test Scope Summary
change: add under the viola-agent-claude entity: «- **Measured (chunk 2026-11-06-version-done-check):** the ledger row `statusline-rate-limits` reads the account's status-line plan state, not the CLI: a live `viola verify` on an unchanged `claude` 2.1.287 read `stamped 2.1.287  23 pass  1 fail` (`[21/24] statusline-rate-limits`) when the payload carried `five_hour` `unknown` and only `seven_day`; owner 0.2.0 (finding F-06i).»
sidecar: test-scope-summary viola-agent-claude entity gains the measured statusline-rate-limits finding (the row reads the account's plan, not the CLI; owner 0.2.0, F-06i)
rationale: the report's Spec claims disproved records the rig home's verify red 23 pass 1 fail on unchanged 2.1.287 because the payload held only seven_day; this entity covers the ledger and verify probes and their local-only live reach
@proposal n=2 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-b2j3wr36 basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-uufjzm4k
section: ## 1. Test Scope Summary
change: the tui surface's Notes read «traced to arch excerpt, Stack (portable-pty, vt100) and creator excerpt, Must-Work (R2, R3/S1, exit detected on the process handle, never on EOF).» — the «Windows ConPTY» reason dropped; the no-screen-parse and readiness-gate sentences unchanged
sidecar: test-scope-summary tui surface Notes lose the Windows ConPTY trace; exit on the handle, never on EOF, stands
rationale: the report retires the =0.8.1 pin and its Windows rationale (ConPTY EOF) and the product is Linux-only on portable-pty =0.9.0 with pty_backend openpty; the handle-exit rule the reason guarded stands
@proposal n=3 detector=D-cascade severity=warning file=test-strategy anchor=fct-cz5jn5fs basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-ngwjb7xx
section: ## 2. Test Strategy
change: the Mutation level's last sentence ends «…survives every committed test by construction (§1 `viola-channel`, **As landed (the euid wiring, untestable)**), and stands as the version's known limit, read at the epoch-boundary audit (matrix#v1-44).» in place of «and the mutation audit reads that survivor at the route entry "Version done-check".»
sidecar: test-strategy Mutation level: the always-admit survivor is 0.1.0's known limit read at the epoch-boundary audit (matrix#v1-44), no longer owed to the Version done-check entry
rationale: the report's expected amendment 14 makes the always-admit survivor the version's known limit read at the epoch-boundary audit (CARRY 8) and names fct-cz5jn5fs among the three sites that name this entry as owner
@proposal n=4 detector=D-cascade severity=warning file=test-strategy anchor=fct-vekwchud basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-zmvefc57
section: ## 2. Test Strategy
change: the Contract level's replay sentence adds, after «replays every dialog variant byte for byte under `--dialogs`», «(its synthetic witness `dialog_drift_names_a_changed_annotation`: a planted change to a forwarded `annotations` byte of `synthetic_dialog_set` reads `dialog_drift` red)»
sidecar: test-strategy Contract level names the planted-annotation drift witness dialog_drift_names_a_changed_annotation (chunk 2026-11-06-version-done-check)
rationale: the report's Symbols / APIs adds tests/contract_fake_agent_drift.rs::dialog_drift_names_a_changed_annotation (lines 807-839), a planted forwarded-annotations byte read red by dialog_drift; the level lists the suite's synthetic drift witnesses beside the statusline one
@proposal n=5 detector=D-cascade severity=warning file=unit-test-strategy anchor=fct-hpqcf7r7 dependent_of=D-new-detail basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-npz56o6m
section: ## 4. Unit Test Strategy
change: after the statusline arms' relied-set clause add «Measured at chunk 2026-11-06-version-done-check: on an unchanged `claude` 2.1.287 a live `viola verify` read `statusline-rate-limits` red (`23 pass  1 fail`) because the account's status-line payload carried `five_hour` `unknown` and only `seven_day`; the row reads the account's plan state, not the CLI. The arm and its nine cases stand; owner 0.2.0 (finding F-06i).»
sidecar: unit-test-strategy viola-agent-claude: the statusline-rate-limits arm's live reading recorded (account plan state, not CLI; owner 0.2.0, F-06i)
rationale: the report disproves the row as a CLI behaviour relied on every build (Spec claims disproved); the ledger stays 24 rows and the fixture-stamped expectations are unchanged, so only the relied-every-build reading moves
@remark
F1 (dialog_drift witness) is e2e-test-strategy's; the body lines fct-njspb5nv, fct-u3dkdc2w, fct-hlx2zkbt, fct-3mj52n2x, fct-b4avhaux, fct-vxnii5yh, fct-yoazurel, fct-2ztzeoct carry =0.8.1 / Windows / RUSTSEC-2017-0008 wording outside group 1's picked entities
