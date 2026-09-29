### CI integration

- **Pipeline integration:** a11y runs inside the tests' E2E pipeline per the upstream-context Section 5 Test Harness Contract Summary (binding 5-command discipline: `boot`, `run`, `status`, `cleanup`, `logs`).
  - a11y specs are ordinary files in `e2e-web/`, so they run in the existing `playwright` suite.
  - There is no separate driver and no new `suite` enum value.
- **Command:**
  - `scripts/agent-run.sh run --browser` (POSIX) or `scripts/agent-run.ps1 run --browser`, which invokes the locked Playwright CLI (`node node_modules/@playwright/test/cli.js test` in `e2e-web/`), followed by `gate --require …playwright` (skips never allowed).
  - Per-test session lifecycle is `boot` → test → `cleanup`. `status` is the readiness precondition, and `logs` is the failure-triage read.
  - The browser suite runs on every leg of `ci.yml`'s `test` job; the a11y verdict is judged on the ubuntu leg only. `browser-missing` is a failure.
- **Artifact:**
  - Playwright JSON report at the suite `artifact` path, with `attachments[]` holding scrubbed axe JSON, token-pair JSON, html-validate JSON, VSR phrase logs and the `a11y-violations` NDJSON.
  - `e2e-web/test-results/` is uploaded by `actions/upload-artifact` v7.0.1, SHA-pinned per zizmor.
  - Per-PR new-violation diff: the verdict is binary (`violations: []` on base and head), so every violation on a PR is new by construction and fails. No fingerprint baseline file exists.
