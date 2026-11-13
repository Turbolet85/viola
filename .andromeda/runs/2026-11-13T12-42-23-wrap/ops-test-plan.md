@row_set n=1 row=decision/logs-carry-no-secrets cell=rule seen=1 detector=D-tests-count-literals
value: No log contains a stripped CLAUDE* value, and the fixed canary appears in no home-level diagnostics line.
basis: fct-xhrefo2i, fct-qlmrr4ao
because: The secret scan's cookie class is retired and a Cookie header line now reads as no hit, so the rule guarded by the secret scan can no longer name the Cookie header.
@row_set n=2 row=decision/no-secrets-in-logs cell=rule seen=1 detector=D-tests-count-literals
value: No log line and nothing in diagnostics/ holds a stripped CLAUDE* value, and diagnostics/ is mode 0700 with its files 0600.
basis: fct-xhrefo2i, fct-qlmrr4ao
because: The cookie class of the secret scan is retired, so the Cookie clause guarded by the secret-scan test no longer stands.
@row_set n=3 row=decision/secrets-never-in-logs cell=rule seen=1 detector=D-tests-count-literals
value: No stripped-variable value appears in any log or diagnostics file.
basis: fct-xhrefo2i, fct-qlmrr4ao
because: The secret scan no longer holds a cookie class, so the row's Cookie header clause names a retired surface.
@row_set n=4 row=decision/one-shared-test-helper cell=rule seen=1 fact=F1 detector=none
value: Root tests share the events reader and wait, the child guard and runner, the freeze guard, the result struct, the private-dir helper and the one source-masking scanner of the census tests from the test support module; no root test file or census defines its own copy.
basis: fct-ykh3ebsp, fct-7qarizhb, fct-qlmrr4ao
because: The three census tests now mask source through one shared scanner and test homes and endpoint directories are made through one shared private-dir helper, so the shared-helper rule must name them.
@row_retire n=1 row=contract/secret-scan-class/cookie seen=1 detector=D-tests-count-literals
why: The secret scan's cookie class is retired; a Cookie header line now reads as no hit.
basis: fct-xhrefo2i, fct-45cfy2kk, fct-qlmrr4ao
because: The closed list of secret-scan hit classes lost cookie, so its contract row no longer stands.
@row_add n=1 row=decision/os-gated-cases-in-binary-root area=test-strategy fact=F2 detector=none
name: OS-gated cases stay in a split binary's root file
rule: The OS-gated census walks only files directly under tests/ and crates/*/tests/, so an integration binary split into a module directory keeps every gated case and its gated helpers in its root file.
why: A gated case moved into the module directory would leave its gates row reading as no longer standing.
guard: contract_os_gated_tests
basis: fct-dqmoaoss, fct-lpwgvmaf
because: The cli_verify split kept its cfg(unix) cases in the root file by this rule, which binds every later split of a test binary and no standing row says it.
@row_none n=1 fact=F3 type=5
what: The census filters' left-out file list (the fake agent binary and its module directory) is inventory the census code holds; the bounded-readers and error-writer rows already scope themselves to product readers and writers.
@row_none n=2 fact=F4 type=5
what: The planted case of each hit class is inventory the secret-scan tests hold; the four standing secret-scan-class rows already state the class list, and the cookie class's removal is the retire of its row.
@row_none n=3 fact=F5 type=5
what: The tap's e2e cases are a test inventory held by crates/viola-e2e/tests/mcp_tap.rs; the tap's behaviour is already stated by the standing MCP tap rows.
