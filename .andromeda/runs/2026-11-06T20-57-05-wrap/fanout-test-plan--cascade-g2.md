@proposal n=1 detector=D-retired severity=warning file=integration-test-strategy anchor=fct-vmzc7an2 basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-xc746owh
section: ## 5. Integration Test Strategy
change: the Tools line reads «portable-pty `=0.9.0` via viola-pty seam» (was `=0.8.1`); the rest of the PTY boundary stands (openpty spawn, exit on the handle, resize rules)
sidecar: integration-test-strategy#1.5 Module ↔ PTY tools line moved from portable-pty =0.8.1 to =0.9.0 (2026-11-06-version-done-check dependency move)
rationale: the report's Dependencies move portable-pty `=0.8.1` → `=0.9.0` and its Retired bullet names the `=0.8.1` pin; viola-pty runs 33 of 33 on 0.9.0 and pty_backend() stays openpty
@proposal n=2 detector=D-retired severity=warning file=integration-test-strategy anchor=fct-lzdm6e7w basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-zmvefc57
section: ## 5. Integration Test Strategy
change: in the home-resolution item, «backed by the `src/cmd/mod.rs` units `resolve_home_*`, `canonical_home_*` and `plain_drive_*`» becomes «backed by the `src/cmd/mod.rs` units `resolve_home_*` and `canonical_home_*`»
sidecar: integration-test-strategy#5.1 the retired plain_drive_* unit table dropped from the home-resolution backing units
rationale: the report removes `plain_drive` and its rstest table `plain_drive_strips_only_the_verbatim_drive_prefix`; canonical_home keeps its signature (Symbols / APIs, Retired)
@proposal n=3 detector=D-retired severity=warning file=integration-test-strategy anchor=fct-lzdm6e7w basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-zmvefc57
section: ## 5. Integration Test Strategy
change: in the `tests/cli_output_plain.rs` item, «its usage program read as `viola` or `viola.exe` followed by the verb's words» becomes «its usage program read as `viola` followed by the verb's words»
sidecar: integration-test-strategy#5.1 the help_lists_only_served_verbs usage program reads plain viola; the .exe form left with the retired .exe trim
rationale: the report removes every EXE_SUFFIX join and `tests/cli_output_plain.rs` `.trim_end_matches(".exe")`; the binaries are named plain `viola` (Symbols / APIs, Retired)
@proposal n=4 detector=D-retired severity=warning file=integration-test-strategy anchor=fct-lzdm6e7w basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-uufjzm4k
section: ## 5. Integration Test Strategy
change: the `tests/cli_slash_args.rs` item's head drops the retired warning: «`tests/cli_slash_args.rs` (Vector 6: a leading-slash argument is never taken as a prompt)»; its cases (exit 2 usage error, no event bytes, no receipt prompt) stand
sidecar: integration-test-strategy#5.1 cli_slash_args no longer points at the retired §6 rewritten-path warning; the never-a-prompt rule stands
rationale: the report retires the Git Bash rewritten-path warning (its emitter, unit table and CLI witness; T2), and the rule that a leading-slash argument reaches viola only as stdin or --file stays
@proposal n=5 detector=D-cascade severity=warning file=integration-test-strategy anchor=fct-lzdm6e7w basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-npz56o6m
section: ## 5. Integration Test Strategy
change: the `contract_ledger_probes.rs` sub-item «The same check runs against the real CLI under `--local-live`» gains: «; there the `statusline-rate-limits` row reads the account's status-line plan state, not the CLI, so an account whose payload carries only `seven_day` reads it red (`23 pass  1 fail` on an unchanged 2.1.287), owner 0.2.0 (finding F-06i)»
sidecar: integration-test-strategy#5.1 the local-live ledger check notes the measured statusline-rate-limits red as an account-state reading owned by 0.2.0
rationale: Spec claims disproved: the rig home's live verify read `23 pass  1 fail` because the row reads the account's plan, not the CLI; the fixture-stamped `0 fail` and the 24-row ledger are unchanged
@remark
expected amendment 10 (CARRY 1, the trycmd text in fct-lzdm6e7w's `tests/cmd/*.toml` mirror line and its verify step-counter sentence) is not proposed: the report names no literal pin that replaces it; F2 (slug test-scope-summary) is outside this group
