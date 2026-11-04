@amendment marker=2026-11-04-linux-and-macos-parity
title: the per-OS JUnit reading of one CI run
section: §9 CI Integration (ci-integration.md: the telemetry artifact rows, a new row after the JUnit row)
change:
  - A new row: the three `junit-<os>` artifacts of one run (`junit-windows-2025`, `junit-macos-latest`, `junit-ubuntu-latest`), read by `scripts/junit-os-sets.py --run <id>` (python 3 standard library and the host's `gh`), on the operator pass only, never a CI step.
  - Its handling: `gh run download` into a `tempfile.mkdtemp()` dir removed on exit; nothing written in the tree, no environment variable read.
  - Its output: per OS `{os}: {n} cases · {f} failed · {s} skipped`, then `## {label}: {count}` and the sorted `classname::name` lines for `ubuntu-not-macos`, `macos-not-ubuntu`, `windows-not-ubuntu` and `ubuntu-not-windows`; exit 0, 1 for a report missing or unparsable (named), 2 for usage.
why: the chunk adds `scripts/junit-os-sets.py`, an operator-pass instrument run by the plan's `leg = 'operator'` entry and never by CI, reading exactly the `junit-<os>` uploads; a new fact with no earlier master site.
ref: .andromeda/runs/2026-11-04T21-59-44-wrap/
@amendment marker=2026-11-04-linux-and-macos-parity
title: the OS-gated test census in the E2E layer
section: §9 CI Integration (ci-integration.md: the pipeline integration rows, a new row after the E2E tests row)
change:
  - A new row: the OS-gated test census (`tests/contract_os_gated_tests.rs`, a contract binary in the E2E layer, `run --e2e`) requires every OS-gated test function and whole gated file directly under `tests/` and each `crates/*/tests/` to be named by a `GATES` row with a non-empty reason.
  - It fails on an unnamed gate or a row that no longer stands; its self-check reads eight planted cases; its verdict is the nextest JUnit and the `run --e2e` exit code.
why: the chunk adds the root contract binary `contract_os_gated_tests`, auto-discovered and selected by the E2E layer's `contract_` prefix (`run --e2e`, suite `nextest-e2e`), green on the three OSes on the final pushed sha; a new fact with no earlier master site.
ref: .andromeda/runs/2026-11-04T21-59-44-wrap/
