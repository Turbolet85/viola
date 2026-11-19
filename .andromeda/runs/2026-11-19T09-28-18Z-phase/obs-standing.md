# obs · standing conventions

- S1 · each recorded fixture file is scrub-required, passes the recorder's hygiene, and is never a telemetry source · 1-obs-scope-summary · obs-scope-summary
- S2 · each new or re-run test runs under a test-owned home inside the scanned integration-test root, kept until the gates run, and is gated by the panic, schema and secret-scan gates · 9-ci-integration · ci-integration
- S3 · each test that waits on a log or process uses a bounded wait under the lint, never a sleep, with no retry · 10-slo-invariants-telemetry-budgets · slo-invariants-telemetry-budgets
- S4 · each fixture or artifact uploaded from CI holds only synthetic or scrubbed inputs and passes the secret scan first · 8-pii-scrubbing-compliance · pii-scrubbing-compliance
  decision/detail-upload-synthetic-inputs-only · Detail-file upload only with synthetic inputs — Detail files and the harness capture are uploaded only while every input is synthetic; a non-synthetic source first drops them from the upload paths. · why: every input is synthetic, so detail content carries no real data
