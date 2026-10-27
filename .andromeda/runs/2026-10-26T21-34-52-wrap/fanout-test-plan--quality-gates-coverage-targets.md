proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 26
    section: '## 10. Quality Gates & Coverage Targets'
    change: 'Follow line 26 with one item: Every timed hook now also runs the instance strict-modes check first (`viola hook` takes `instance_of(…).and_then(canonical).filter(trusted)`, `check_instance` on the home and `instances/builder`, before `read_diagnostics_level` / `viola_obs_init`), so the perf home viola booted must pass it to stay on the real logging and channel path (a refused instance is the silent exit-0 no-op); with the check in the timed path the five hook rows read `max` 2.4–3.3 ms (2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms) on the Linux dev host, as measured at `viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md` (Outcome, `run --perf`).'
    sidecar: 'gate-perf: the hook instance strict-modes check added to the timed path, its perf reading (max 2.4–3.3 ms, five rows) recorded (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'The report adds the instance check to the hook event/dialog paths before obs init (Changes, Symbols, src/cmd/hook.rs canonical/trusted) and its Outcome re-asserts the perf criterion with that check in the timed path (run --perf green, max 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms); the gate-perf group states only the server-check reading of the previous chunk.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:137
note: 'No count literal, retired gap or read_snapshot / seed / single-writer / wheel-human claim the report moves stands in this file; coverage, mutation and harness surfaces unchanged (Harness / gate surface: none), so D-tests-coverage, D-tests-framework and D-tests-obs-harness hold here.'
