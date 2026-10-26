# tests · standing conventions

- S1 · A gate's verdict is its printed ending line and exit code, read from the bare command, never a pipe's last command · .andromeda/test-plan-detail/ci-integration.md
- S2 · Each tool runs at the one version ci.yml pins; the local pre-push checks the host against those pins and installs nothing · .andromeda/test-plan-detail/ci-integration.md
  (.andromeda/test-plan-detail/e2e-test-strategy.md:15) e2e
- S3 · Zero flakiness: nextest `retries = 0`, a red stays red until its root cause is fixed, no `#[ignore]` parking · .andromeda/test-plan-detail/quality-gates-coverage-targets.md
  (.andromeda/test-plan-detail/test-strategy.md:75) strategy
- S4 · No gate is widened, skipped or retried to reach green (the coverage ignore regex, a threshold, a retry budget) · body §11 Test Anti-Patterns (CI, Quality)
- S5 · A test added for a forced source change follows `<subject>_<condition>_<expected>` naming and the layer prefixes (`cli_`, `mcp_`, …) · .andromeda/test-plan-detail/test-strategy.md
