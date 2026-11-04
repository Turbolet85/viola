@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/services/viola.md lines=58
last: - `.andromeda/architecture.md` · `.claude/rules/api.md` · `.
@leaf_read leaf=.claude/docs/stack.md lines=66
last: 5. Commit as `chore: bump {tool} to {version}`
@leaf_read leaf=.claude/docs/workflow.md lines=67
last: This project uses a **reference-based** CLAUDE.md architectu
@leaf_edit n=1 leaf=.claude/docs/workflow.md line=48 class=omission
leaf-text: CI (three OSes) is the authority for OS-specific code: a Linux-only or Windows-only local run is not proof for the other OSes.
source: fct-xxlqtlbc «A new OS gate on a test therefore lands with its `GATES` row and reason, or the census goes red.»
replace-with: CI (three OSes) is the authority for OS-specific code: a Linux-only or Windows-only local run is not proof for the other OSes. Every test held off a runner by an OS gate names its reason: a new OS gate lands with its `GATES` row (`Gate { file, test, gate, reason }`) in the E2E contract binary `tests/contract_os_gated_tests.rs` (run under `run --e2e`), or its census goes red.
@leaf_edit n=2 leaf=.claude/docs/workflow.md line=11 class=omission
leaf-text: → the guarded push → the CI reads.
source: fct-jyuckxyp «`scripts/junit-os-sets.py` (the python 3 standard library and the host's `gh`): the operator pass's per-OS JUnit reading.»
replace-with: → the guarded push → the CI reads (per OS through `scripts/junit-os-sets.py --run <id>`, an operator-pass instrument and never a CI step: it downloads the three `junit-<os>` artifacts and prints each OS's case, failed and skipped counts and the cases one OS ran that another did not).
