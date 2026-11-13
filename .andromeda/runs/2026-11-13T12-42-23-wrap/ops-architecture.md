@row_add n=1 row=decision/os-gated-cases-in-root-test-file area=cross-cutting-patterns fact=F2 detector=none
name: OS-gated cases stay in the root test file
rule: When a test binary splits into a module directory, every OS-gated case it holds stays in the binary's root test file.
why: The OS-gated census reads only the files directly under each test directory, so a gated case moved into a module file leaves its gates-table row reading no longer stands.
guard: contract_os_gated_tests
basis: fct-dqmoaoss, fct-lpwgvmaf
because: The chunk measured that the OS census walks only top-level test files and kept the cli_verify unix-gated cases in the root file, a convention binding every later split of a test binary that no standing row states.
@row_none n=1 fact=F1 type=4
what: How the source census checks are built — one shared test-support masking scanner and a census filter that leaves out the test-only fake agent's module; the bounded-readers and sanitised-error-surfaces rows already scope their rule to product code and stay true, and the test plan's shared-helper row holds the scanner.
@row_none n=2 fact=F3 type=5
what: The secret scan's class list is an inventory held by the code and the security plan's scan-class contract rows; no architecture row names the cookie class, and the chunk-diff skip rows stay true.
@row_none n=3 fact=F4 type=5
what: A file layout of the test-only fake agent's module directory and the split test suites' sibling module files — the code holds it; no architecture row lists those paths.
