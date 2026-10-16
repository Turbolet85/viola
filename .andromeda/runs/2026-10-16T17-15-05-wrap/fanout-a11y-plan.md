proposals:
  - detector: D-a11y-obs-schema
    severity: warning
    section: §3 → Bootstrap phases (derive for route / setup-project) — the violation-json-emission-wire phase
    change: "violation-json-emission-wire: none in 0.1.0 — cli / tui emit no a11y violation rows (Section 3 → Structured violation JSON schema: output-discipline failures surface only as nextest failures in `suites[].failures[]`); the frontend is deferred to 0.2.0, its technology is not chosen, and its violation row shape is a 0.2.0 input (obs's closed `process` enum is `run · hook · mcp · cli`, no `ui`)."
    sidecar: "violation-json-emission-wire no longer writes `e2e-web/test-results/a11y/<test-id>.ndjson` rows with `process:\"ui\"` and `version` from `GET /health`: obs's `process` enum lost `ui` (chunk 2026-10-16-remove-the-browser-leg) and cli / tui carry no a11y rows."
    rationale: "The report's Schema / config bullet: `schemas/diag-line.v1.json`'s `process` enum is `run · hook · mcp · cli` and `schemas/diag-detail.v1.json`'s `process` enum is the four; Symbols / APIs: `ObsProcess::Ui` removed, `ObsProcess::ALL` the four. The phase still promises rows in the obs shape with `process:\"ui\"` (obs field names unrenamed), a value the obs schema now refuses under `unevaluatedProperties: false` / its closed enum — the a11y violation shape and obs §6 diverge. The Structured violation JSON schema key already says cli / tui have no a11y rows, and the report's Files bullet deletes `e2e-web/` whole."
    basis: "schemas/diag-line.v1.json (the `process` enum); .andromeda/registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md (violation-json-emission-wire bullet)"
  - detector: D-a11y-obs-schema
    severity: warning
    section: §3 → Bootstrap phases (derive for route / setup-project) — the a11y-tooling-install phase
    change: "drop the `violation-row writer` from the `e2e-web/fixtures/a11y.ts` bullet: 0.1.0 writes no a11y violation row (the fixture's directory `e2e-web/` left the tree)."
    sidecar: "a11y-tooling-install no longer names a violation-row writer: no a11y violation row is emitted in 0.1.0."
    rationale: "The same retired claim restated at its writer: the report deletes `e2e-web/` whole (Files) and the obs `process` enum holds no `ui` (Schema / config), so a writer of obs-shaped violation rows has no schema value to emit under."
    basis: ".andromeda/registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md (a11y-tooling-install bullet)"
    dependent-of: D-a11y-obs-schema
  - detector: D-a11y-obs-schema
    severity: warning
    section: §3 → Bootstrap phases (derive for route / setup-project) — the a11y-ci-gate-wire phase
    change: "drop the post-gate `jq` step's read of `e2e-web/test-results/a11y/*.ndjson` (the violation rows) together with the browser leg; no a11y violation NDJSON exists in 0.1.0."
    sidecar: "a11y-ci-gate-wire's aggregation no longer reads `e2e-web/test-results/a11y/*.ndjson` violation rows."
    rationale: "The consumer side of the retired emission: the rows it reads were the obs-shaped `process:\"ui\"` violation rows the report's Schema / config change leaves without a valid `process` value; `e2e-web/` is deleted (Files)."
    basis: ".andromeda/registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md (a11y-ci-gate-wire bullet)"
    dependent-of: D-a11y-obs-schema
  - detector: D-a11y-obs-schema
    severity: warning
    section: §3 → CI integration — the Artifact bullet
    change: "drop the `a11y-violations` NDJSON (and the rest of the Playwright attachments) from the Artifact bullet; cli / tui results land only as nextest failures in `suites[].failures[]`, with no a11y violation rows."
    sidecar: "CI integration's Artifact bullet no longer lists an `a11y-violations` NDJSON attachment."
    rationale: "The same violation-row emission restated as a CI artifact; the report removes the `ui` process from both diag schemas (Schema / config) and the browser suite that attached it (Harness / gate surface, Files)."
    basis: ".andromeda/registries/contracts/a11y-plan/ci-integration.md (Artifact bullet)"
    dependent-of: D-a11y-obs-schema
  - detector: D-a11y-obs-schema
    severity: warning
    section: §9. CI Integration — Per-pipeline-stage a11y artifacts table, E2E row
    change: "the E2E row leaves with the browser leg, its `a11y-violations` NDJSON artifact included; no stage emits an a11y violation row in 0.1.0."
    sidecar: "§9's E2E stage row and its `a11y-violations` NDJSON artifact leave."
    rationale: "Restates the obs-shaped violation-row emission as the E2E stage's artifact; the report's Schema / config bullet removes `ui` from the obs `process` enum and the Files bullet deletes `e2e-web/`."
    basis: ".andromeda/a11y-plan.md §9 (Per-pipeline-stage a11y artifacts, E2E row)"
    dependent-of: D-a11y-obs-schema
  - detector: D-a11y-obs-schema
    severity: warning
    section: §9. CI Integration — Per-pipeline-stage a11y artifacts table, Aggregation row
    change: "the Aggregation row leaves with the browser leg: no `jq` read of `e2e-web/test-results/a11y/*.ndjson`, no per-`violation_type` count and no `violation_type_counts` in an `sc-coverage-report.json`."
    sidecar: "§9's Aggregation row (the violation-row `jq` count and `violation_type_counts`) leaves."
    rationale: "The aggregation consumes the violation rows whose obs `process:\"ui\"` value the report's Schema / config change removed; `e2e-web/` is deleted (Files)."
    basis: ".andromeda/a11y-plan.md §9 (Per-pipeline-stage a11y artifacts, Aggregation row)"
    dependent-of: D-a11y-obs-schema
note: "D-a11y-surface: no drift — the report adds no interactive UI element (Coverage of new surfaces: none; a11y n/a on both listed rows). The proposals above cover only the violation-row / obs-schema claim; the wider browser-leg text in a11y-plan (§1 → CI integration's runner, results and `gate --require playwright` bullets, §3 CI integration's Pipeline integration / Command bullets, the other Bootstrap phases, §9's Lint row, Unit / integration row's `gate --require coverage,doctest,playwright`, Pipeline integration and Artifact retention, the §9 stand-until paragraph, §10's `run --browser` budget and `gate --require playwright` / ESLint failure conditions, §11 Strategy and CI's Playwright / `run --browser` / `browser-missing` bans) is the report's Expected amendments item for a11y-plan, matched by none of these two detectors' invariants; the tier `Standard (1)` is untouched per the report."
