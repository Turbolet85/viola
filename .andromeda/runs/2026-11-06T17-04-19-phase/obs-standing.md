# obs · standing conventions

- S1 · every integration-test viola home lives under `target/e2e-home/`, so G2, G4, the secret scan and the `diag-<os>` upload cover it · ci-integration
  (obs-plan/ci-integration fct-uqr3kd37) ci
- S2 · every log line goes through `obs_event!` with typed fields named in the §6 catalog only, and conforms to `schemas/diag-line.v1.json` (default-deny); no raw `event!` or level macro outside `viola_core::obs` · log-coverage
  (obs-plan/log-coverage fct-iy4rx7dp) log-details
- S3 · no retry-once policy for panics or flaky telemetry assertions; `retries = 0`, and any nonzero error-budget count makes the chunk red until fixed within it · slo-invariants-telemetry-budgets
- S4 · every CI gate is its own `run:` step with `shell: bash` on the `ubuntu-latest` leg; G2, G4 and the secret scan run `if: always()`; uploads are gated on the secret-scan outcome and Actions are pinned by commit SHA · ci-integration
  (obs-plan/ci-integration fct-u6lpuily) ci
- S5 · every test input and fixture is synthetic, and no `CLAUDE*` value, `Cookie` header or content canary may reach a home-level `diagnostics/` file · pii-scrubbing-compliance
  (obs-plan/obs-scope-summary fct-6un2lrdt) scope-entities
  (obs-plan/pii-scrubbing-compliance fct-cthr64ip) pii
  (obs-plan/pii-scrubbing-compliance fct-rp7rj7wv) pii
- S6 · no `print!` / `println!` / `eprint!` / `eprintln!` / `dbg!` in the `hook`, `run` or `mcp` code paths of product crates · obs-anti-patterns
  (obs-plan/obs-anti-patterns fct-hfdtlj6j) logs
