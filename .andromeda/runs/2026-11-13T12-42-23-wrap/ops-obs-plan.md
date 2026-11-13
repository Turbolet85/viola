@row_set n=1 row=decision/secret-scan-before-upload cell=rule seen=1 fact=F1 detector=none
value: CI runs the secret scan over every diagnostics/ file before any artifact upload, its classes being CLAUDE* values, the content canary, mode 0600 and an unreadable file; a Cookie header line is no class and no hit.
basis: fct-xhrefo2i, fct-45cfy2kk, fct-qlmrr4ao
because: The scan's cookie class is retired and an unreadable-file class now has its planted case, so the row naming the Cookie header among the scanned classes is false.
@row_set n=2 row=decision/no-secrets-in-log-lines cell=rule seen=1 detector=none
value: No log line may contain any stripped CLAUDE* value.
basis: fct-xhrefo2i, fct-qlmrr4ao
because: The cookie class is retired and the plan's amendment drops the Cookie clause of this row, so its Cookie header promise no longer stands.
@row_set n=3 row=decision/secrets-never-logged cell=rule seen=1 detector=none
value: Signing credentials and R8-stripped CLAUDE* values are never logged.
basis: fct-xhrefo2i, fct-qlmrr4ao
because: The cookie class is retired and the plan's amendment drops the Cookie clause of this row.
@row_set n=4 row=decision/content-only-in-instance-detail-file cell=rule seen=1 detector=none
value: Content-bearing detail goes only to the instance's detail-<process>.ndjson file; home-level log files are codes-only and carry no user content, token or CLAUDE* value.
basis: fct-xhrefo2i, fct-qlmrr4ao
because: The report's sweep names this row as one more Cookie site to drop with the retired cookie class.
@row_set n=5 row=decision/os-gated-test-census cell=rule seen=1 fact=F4 detector=none
value: Every OS-gated test function or file must be named in the census with a non-empty reason, and a gate predicate naming windows or macos is a fault even when named; the census walks only files directly under a tests directory, so a split test binary keeps every named gated case in its root file.
basis: fct-dqmoaoss, fct-lpwgvmaf
because: The census reads only files directly under a tests directory, which binds every split of a test binary to keep its census-named gated cases in the root file.
@row_set n=6 row=decision/error-bodies-clean cell=except seen=1 fact=F3 detector=none
value: The fake agent's whole module directory, its bin root and every file under it, is left out of the product-source census.
basis: fct-ud4xtspq
because: The error-bodies census filter now leaves out the fake agent's whole module directory, not only its single bin file, and the row names no exception.
@row_retire n=1 row=decision/cookie-header-never-logged seen=1 detector=none
why: The secret scan has no cookie class any more and a Cookie header line is no hit, so no decision keeps a Cookie header out of diagnostics.
basis: fct-xhrefo2i, fct-45cfy2kk, fct-qlmrr4ao
because: The cookie class and its planted hit are retired, so the whole row on the Cookie header no longer stands.
@row_retire n=2 row=decision/claude-env-and-cookie-never-logged seen=1 heir=decision/claude-env-never-logged detector=none
why: The Cookie header clause is dropped with the retired cookie class, and the row's key names the cookie.
basis: fct-xhrefo2i, fct-qlmrr4ao
because: The plan's amendment drops this row's Cookie clause, and a key made from the cookie cannot carry the remaining CLAUDE* rule.
@row_add n=1 row=decision/claude-env-never-logged area=11-obs-anti-patterns-never-do-these detector=none
name: CLAUDE* values never logged
rule: CLAUDE* values are never logged, at any level, in any file, detail files included.
basis: fct-xhrefo2i, fct-qlmrr4ao
because: The CLAUDE* half of the retired cookie row still stands and needs its own row once the Cookie clause is dropped.
@row_add n=2 row=decision/library-test-scoped-subscriber area=bootstrap-phases-derive-for-route-setup-project fact=F5 detector=D-obs-stack
name: Scoped test subscriber in a library crate
rule: A library crate's unit test may observe a line it emits through its own minimal tracing Subscriber scoped to that test, never installed globally and with no tracing-subscriber dependency, so tracing-subscriber stays in the root binary only.
basis: fct-pw5a6wc6, fct-r22lxaiu
because: The viola-mcp WARN case observes its line through a test-scoped Subscriber with no manifest change, a convention for library tests that no row states.
@row_none n=1 fact=F2 type=4
relied: The three test-source censuses read code through one masker that blanks comments, or comments and literals, and the OS-gated census reads a blanked line, block comments included, as a comment line.
