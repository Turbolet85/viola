# obs · standing conventions

- S1 · each new or changed test waits for a log or event receipt by a bounded poll, never a sleep-based log wait · slo-invariants-telemetry-budgets · 10-slo-invariants-telemetry-budgets
- S2 · each new test home lives under the scanned integration-test home root and is kept until the gates run · ci-integration · 9-ci-integration
- S3 · each new test that exercises verify uses the fake agent; real-CLI verify never runs in CI · span-trace-coverage · 4-span-trace-coverage
- S4 · each new test's verdict is machine-asserted by its exit, not read by a human · 11-obs-anti-patterns-never-do-these
  decision/machine-asserted-telemetry-verdicts · Machine-asserted telemetry verdicts — Log analysis is never human-review-gated; every CI telemetry verdict is a jq or hyperfine assertion that sets an exit code.
  decision/no-human-gated-log-analysis · No human-review-gated log analysis — Every log verdict is a jq or harness assertion that sets an exit code, never a human review.
