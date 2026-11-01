proposals:
  - detector: D-new-detail
    severity: warning
    fact: F23
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 33
    section: "## 5. Integration Test Strategy"
    change: "- **Harness `cleanup` witnesses** (the `endpoint_gone` reading on Unix, §3 `cleanup` step 4): `cleanup_after_a_force_killed_wrapper_reads_the_endpoint_gone` (`crates/viola-e2e/tests/harness_lifecycle.rs`) on `ubuntu-latest`, and the unit `unconnectable_reads_a_leftover_socket_file_no_one_listens_on` (`crates/viola-e2e/src/harness/cleanup.rs`)."
    sidecar: "2026-11-01-unix-endpoint-and-home-hardening: §5 gains the witnesses of harness `cleanup`'s `endpoint_gone` on Unix, one integration case and one unit case."
    rationale: "The report names both witnesses of the measured `cleanup` change: the force-killed-wrapper case on `ubuntu-latest` (report.md:57, report.md:114, its file at report.md:182) and the unit in the harness source (report.md:179), `unconnectable` being the reading it holds (report.md:20)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:57
