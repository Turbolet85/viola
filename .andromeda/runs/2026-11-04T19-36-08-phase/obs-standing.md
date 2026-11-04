# obs · standing conventions

- S1 · every integration-test viola home lives under target/e2e-home/, so G2, G4, the secret scan and the per-OS diag upload cover it (the one chaos_feed_panic home excepted) · ci-integration
  (obs-plan/ci-integration fct-uqr3kd37) ci
- S2 · every new log line goes through obs_event! with the closed event enum and catalog-named typed fields only: no raw tracing level macros, no print or eprint, corr passed as a typed value, no path or pid in detail · log-coverage
- S3 · every new span uses instrument with skip_all first, a static area.operation name and an explicit field allow-list; never err or ret, and never content in span fields · span-trace-coverage
  (obs-plan/span-trace-coverage fct-yj7tr26f) scenario-human-takes-the-wheel-automation-is-refused-release-returns-it
  (obs-plan/span-trace-coverage fct-6peizmat) scenario-budget-governor
- S4 · every new CI gate is its own run step with shell bash on all three OSes, if always where it guards diagnostics; uploads come only after the secret scan passes, actions are pinned by commit SHA, and retention is 7 days · ci-integration
  (obs-plan/ci-integration fct-auyn3syc) ci
  (obs-plan/ci-integration fct-u6lpuily) ci
  (obs-plan/ci-integration fct-t3izd7ip) ci
- S5 · every test and fixture input is synthetic, so a detail-file upload stays admissible; no CLAUDE value, Cookie header or real content reaches any diagnostics file · pii-scrubbing-compliance
  (obs-plan/obs-scope-summary fct-6un2lrdt) scope-entities
  (obs-plan/obs-scope-summary fct-rkannnby) triggers
  (obs-plan/pii-scrubbing-compliance fct-rp7rj7wv) pii
- S6 · no retry-once policy for panics or flaky telemetry assertions (retries 0), and no log-based wait relying on sleeps; emission keys on hook events and events.ndjson offsets · slo-invariants-telemetry-budgets
  (obs-plan/obs-scope-summary fct-jz5zhprx) triggers
