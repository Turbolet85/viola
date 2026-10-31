# security · standing conventions

- S1 · Each new test uses its own `--home`, and the local gate's children get no host environment value beyond `HOME` and `PATH` · Secret Management body, Development
  (.andromeda/security-plan-detail/secret-management.md:15) secrets
- S2 · Each new test's artefacts under `target/agent-run/` pass the canary scan, with synthetic inputs only and matched bytes never printed or written · Secret Management body, Secret scanning in CI
- S3 · A test-side removal under `target/e2e-home` takes only a dir the test side itself created · Security Anti-Patterns body, Code Patterns
  (.andromeda/security-plan-detail/security-anti-patterns.md:23) bans
- S4 · A test-only feature stays out of the release graph, refused by `scripts/release-check.sh`, and shared test helpers come only through root `[dev-dependencies]` · Dependency Security body, CI integration
- S5 · CI's G2 zero-panics gate keeps its one exemption, and a further one needs its own Decisions Log entry · Dependency Security body
  (.andromeda/security-plan-detail/dependency-security.md:48) deps-supply-chain
- S6 · A new test seam or environment read needs its own Decisions Log entry, is compiled out of release builds and switches off no control · Security Anti-Patterns body, Universal
  (.andromeda/security-plan-detail/input-validation.md:47) input
  (.andromeda/security-plan-detail/input-validation.md:50) input
- S7 · A new or changed log line holds nothing from the NEVER-log floor, its format owned by obs · Bootstrap phases body, `logging-redaction-wire`
