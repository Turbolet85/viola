@apply_row n=1 ref_file=fanout-obs-plan--cascade.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-obs-plan--cascade.md ref_n=2 v=applied edit=E2
@fact after=fct-t3izd7ip as=n1
| the three `junit-<os>` artifacts of one run (`junit-windows-2025`, `junit-macos-latest`, `junit-ubuntu-latest`), read by `scripts/junit-os-sets.py --run <id>` (python 3 standard library and the host's `gh`) | the operator pass only, never a CI step | `gh run download` into a `tempfile.mkdtemp()` dir removed on exit; nothing written in the tree, no environment variable read | per OS `{os}: {n} cases · {f} failed · {s} skipped`, then `## {label}: {count}` and the sorted `classname::name` lines for `ubuntu-not-macos`, `macos-not-ubuntu`, `windows-not-ubuntu` and `ubuntu-not-windows`; exit 0, 1 for a report missing or unparsable (named), 2 for usage |

@fact after=fct-k3fxcuz7 as=n2
| OS-gated test census (`tests/contract_os_gated_tests.rs`, a contract binary in the E2E layer, `run --e2e`) | every OS-gated test function and whole gated file directly under `tests/` and each `crates/*/tests/` named by a `GATES` row with a non-empty reason; fails on an unnamed gate or a row that no longer stands; its self-check reads eight planted cases | nextest JUnit + `run --e2e` exit code |
