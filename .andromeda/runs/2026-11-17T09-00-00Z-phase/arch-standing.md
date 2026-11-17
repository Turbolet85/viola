# arch · standing conventions

- S1 · each new test runs in its own viola home · established-decisions
  decision/test-isolated-viola-home · Each test in its own viola home — Each test runs in its own viola home (`--home`), so parallel tests never share an endpoint or log.
- S2 · each new test's channel or thread wait is bounded under the shared root wait bound and the 20 s lint · occupied-resources
- S3 · each new test socket goes under the short `/tmp` root, outside the product namespace · occupied-resources
- S4 · each new harness knob carries the AGENT_RUN_ prefix and each new fake-agent seam follows the fake-agent seam naming · conventions
  decision/harness-env-var-prefix · Test-harness knobs carry the AGENT_RUN_ prefix — Test-harness knobs use the `AGENT_RUN_` prefix and are never read by the `viola` binary; the `FAKE_AGENT_*` seams, read only by fake-agent builds, and each crate's test-private self-spawn markers (`PTY_SEAM_TEST_*`, `MCP_*_TEST_*`) keep their own names.
- S5 · each new OS-gated test names its reason and no test gate names another OS · cross-cutting-patterns
  decision/no-other-os-test-gates · No test gate naming another OS — A test gate whose predicate names windows or macOS is a fault even when the gates table names it. · guard: census_reads_a_gate_naming_another_os_as_a_fault_even_when_named
- S6 · each new end-to-end test leaves zero panic lines · project-directory-structure
