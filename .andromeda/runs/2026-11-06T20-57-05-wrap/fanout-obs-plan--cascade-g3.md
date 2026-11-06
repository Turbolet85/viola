@proposal n=1 detector=D-cascade severity=warning file=error-capture-reporting anchor=fct-j5jj3gux basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-u65xlcax
section: ## 7. Error Capture & Reporting
change: The product's `thiserror::Error` derives read 9: `AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError`, `ServeError` (as measured at chunk 2026-10-31-sanitised-error-surfaces, a grep of `thiserror::Error` over the product sources, `src` and `crates` outside `crates/viola-e2e`, whose one derive at `crates/viola-e2e/src/harness/mod.rs:25` is test-only). `AgentError`'s variants include `ProgramNotFound`, its message fixed.
sidecar: 2026-11-06-version-done-check — the thiserror census rule re-worded to the product sources (the grep reads 10 rows, the 10th test-only in crates/viola-e2e); the count stays 9.
rationale: the report's Counts moved states the grep over src and crates reads 10 rows, 9 outside crates/viola-e2e, and the rule is re-worded to the product sources with the count staying 9 (inputs#I7 item 4).
@remark
fct-nwrbbhk2 (body §7 platform pick) names the 9 types with no grep rule and still holds; T1's five ratified divergences name no picked g3 entity, so their obs-plan follow-up, if any, is the body's or another category's; F1 is span-trace-coverage's, not g3's.
