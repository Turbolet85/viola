# obs · standing conventions

- S1 · each CI gate is its own run step with shell bash, copied verbatim into ci.yml. G2, G4 and the secret scan run if: always(), and a failed test step never skips them. · ci-integration
- S2 · an artifact upload runs only after the secret-scan step passes, uses an action pinned by commit SHA and checked by zizmor, and keeps retention-days 7. A failed scan uploads the hit report instead. · ci-integration
- S3 · every new or changed integration test keeps its viola home under target/e2e-home/, so the gates and the secret scan cover it. · ci-integration
- S4 · tests and telemetry assertions run with retries = 0 and no retry-once policy. Every verdict is an exit-code-setting assertion, never human review. · slo-invariants-telemetry-budgets
