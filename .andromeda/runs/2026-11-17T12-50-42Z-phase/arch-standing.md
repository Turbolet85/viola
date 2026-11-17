# arch · standing conventions

- S1 · every new test that waits takes a bound under the 20 s lint and uses its own viola home, its sockets under the short /tmp root · occupied-resources
- S2 · every new test runs against the fake agent, never a live claude, and is selected by at least one checked filter · ci-cd-approach
- S3 · a new test that is OS-gated names its reason and names no other OS · cross-cutting-patterns
  decision/no-other-os-test-gates · No test gate naming another OS — A test gate whose predicate names windows or macOS is a fault even when the gates table names it. · guard: census_reads_a_gate_naming_another_os_as_a_fault_even_when_named
  decision/os-gated-tests-name-reason · OS-gated tests name their reason — Every OS-gated test names its reason in the census, and a gate whose predicate names windows or macos reads as a fault even when a row names it. · guard: contract_os_gated_tests
  decision/os-gated-tests-named · OS-gated tests named with their reason — Every test held off a runner by an OS gate is named in the gates table with a non-empty reason, and the table names no gate that no longer stands. · guard: contract_os_gated_tests: os_gated_tests_are_each_named_with_their_reason
