# security · standing conventions

- S1 · each new test runs on its own `--home`, never the user's real home · Secret Management, Development
  (.andromeda/security-plan-detail/secret-management.md:15) secrets
- S2 · a child launched by the pre-push gate gets exactly `HOME` and `PATH`; a host-sourced assignment needs its own Decisions Log entry · Secret Management, Development
  (.andromeda/security-plan-detail/secret-management.md:15) secrets
- S3 · every home and file a test builds is owner-only, 0700 and 0600, never the default umask · Security Anti-Patterns, Data Protection
- S4 · each new log line holds nothing on the NEVER-log floor · Bootstrap phases, `logging-redaction-wire` (.andromeda/security-plan-detail/bootstrap-phases.md)
- S5 · each new test's artefacts under `target/agent-run/` are synthetic and stay readable by the artifact canary scan · Secret Management (.andromeda/security-plan-detail/secret-management.md)
  (.andromeda/security-plan-detail/bootstrap-phases.md:24) bootstrap
- S6 · a test-side removal under `target/e2e-home` takes only a dir the test side itself created · Security Anti-Patterns, Code Patterns
  (.andromeda/security-plan-detail/security-anti-patterns.md:23) bans
- S7 · test-only features `fake-agent` and `test-support` stay out of the release build · Dependency Security, CI integration
  (.andromeda/security-plan-detail/input-validation.md:48) input
  (.andromeda/security-plan-detail/input-validation.md:51) input
- S8 · no new test seam or environment read without its own Decisions Log entry · Security Anti-Patterns, Universal
  (.andromeda/security-plan-detail/security-anti-patterns.md:39) bans
- S9 · no new exemption to the G2 zero-panics gate without its own Decisions Log entry · Dependency Security, CI integration
  (.andromeda/security-plan-detail/dependency-security.md:48) deps-supply-chain
