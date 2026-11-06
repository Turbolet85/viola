@proposal n=1 detector=D-cascade severity=warning file=error-handling anchor=fct-gnmmvxa5 basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-gtooygvh
section: ## Error Handling
change: Named exceptions to the error-enum convention, ratified on the founder's own word (2026-11-06, T1): `ServeError`, `StampsError`, `PinError`, `StampsReadError` and `Refused`; no CARRY owns them.
sidecar: error-handling fct-gnmmvxa5 — the five open error-enum divergences ratified as named exceptions on the founder's word (T1, inputs#I2); the Version done-check CARRY owner dropped.
rationale: the report records the founder's 19:34Z word confirming T1, ratifying the five one-enum divergences as named exceptions (Decisions fct-gtooygvh; plan entry 8, fct-ngwjb7xx), so «open» and the owner on route entry Version done-check, now closed, are contradicted.
@proposal n=2 detector=D-cascade severity=warning file=error-handling anchor=fct-hzzqncjb basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-u65xlcax
section: ## Error Handling
change: The `thiserror::Error` derives over product sources are nine: `AgentError` · `ChannelError` · `StateError` · `PinError` · `StampsError` · `StampsReadError` · `Refused` · `McpError` · `ServeError` (the test-only `crates/viola-e2e` harness's derive is outside the count); the census in `tests/contract_error_bodies.rs` reads their messages.
sidecar: error-handling fct-hzzqncjb — the thiserror census rule re-worded to the product sources, the test-only viola-e2e harness derive named outside it; the count stays 9.
rationale: the report's Counts read 10 grep rows, 9 outside crates/viola-e2e, the tenth at crates/viola-e2e/src/harness/mod.rs:25 test-only; the rule is re-worded to the product sources and the count stays 9 (inputs#I7 item 4, CARRY 16).
@remark
The body broad line fct-scytqusw still lists the same nine thiserror enums plus PtyError and holds; task facts F1-F2 (dependency-security) and F3 (authentication-authorization) belong to other groups.
