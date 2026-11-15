# security · standing conventions

- S1 · each new or re-recorded fixture is scrubbed, refused whole if unclean, keeps only signature rows for Screen fixtures, and is reviewed before commit · data-protection
- S2 · each new test or rig uses its own test home, never the operator's viola or Claude home, and removes only what it created · secret-management
- S3 · each new test-only binary or seam stays behind the test-only feature and out of the release build · security-anti-patterns
  decision/fake-agent-hook-panic-seam · Hook panic test seam — FAKE_AGENT_HOOK_PANIC exists only in test-feature builds and forces a fixed-payload hook panic only when exactly 1; the hook still fails open; it configures nothing, disables no control and widens no redaction. · why: Ratified.
  decision/fake-agent-pump-delay-seam · Pump delay test seam — FAKE_AGENT_PUMP_DELAY_MS exists only in test-feature builds, a u64 of milliseconds capped at 5 000 that holds the run pump back after the child starts; it configures nothing, disables no control and widens no redaction. · why: Lets a test land a host resize in the spawn-to-pump window; ratified.
  decision/test-support-dev-only · test-support enabled only for development — viola-channel's test-only test-support feature is enabled only by the root package's dev-dependencies.
- S4 · each new CI job or upload holds no secrets, runs no real claude, and is gated by the artifact canary scan · bootstrap-phases
  decision/checkout-no-persisted-credentials · Checkout without persisted credentials — The CI checkout action runs with persist-credentials false.
  decision/perf-uploads-scan-gated · perf job uploads gated by secret scan — The perf job's hyperfine exports and diagnostics are uploaded only when its own secret scan succeeds, the scan output only when it fails; every perf input is synthetic.
