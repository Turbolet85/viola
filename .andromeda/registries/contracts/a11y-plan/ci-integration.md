### CI integration

- **Pipeline integration:** a11y runs inside the tests' pipeline per the upstream-context Section 5 Test Harness Contract Summary (binding 5-command discipline: `boot`, `run`, `status`, `cleanup`, `logs`).
  - The CLI output-discipline and TUI boundary cases are ordinary nextest cases, so they run in the existing suites (CI: the per-OS `test` job's `coverage` suite).
  - There is no separate driver and no new `suite` enum value.
- **Command:**
  - `scripts/agent-run.sh run` (POSIX) or `scripts/agent-run.ps1 run`, followed by `gate --require coverage,doctest` (skips never allowed), on every leg of `ci.yml`'s `test` job.
  - 0.1.0 runs no browser a11y leg; the frontend's a11y tooling is a 0.2.0 input, its technology not chosen.
- **Artifact:**
  - No a11y artifact of its own: a cli / tui failure is a nextest failure in the harness `run` JSON (`suites[].failures[]`).
