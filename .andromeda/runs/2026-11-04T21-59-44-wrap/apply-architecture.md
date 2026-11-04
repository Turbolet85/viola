@apply_row n=1 ref_file=fanout-architecture.md ref_n=3 v=applied edit=E1
@apply_row n=2 ref_file=fanout-architecture--cascade-new-cross-cutting-patterns.md ref_n=1 v=applied edit=E2,E3
@apply_row n=3 ref_file=fanout-architecture--cascade.md ref_n=1 v=applied edit=E4
@apply_row n=4 ref_file=fanout-architecture--cascade.md ref_n=2 v=applied edit=E5
@apply_row n=5 ref_file=fanout-architecture--cascade.md ref_n=3 v=applied edit=E6
@apply_row n=6 ref_file=fanout-architecture.md ref_n=1 v=refused
why: the row's place is a new line inside the one code-block fact fct-irhgihut (the tree's `scripts/` listing, before `└── profraw-census.sh`), and an in-line change carries no line break (dry run: `a backslash escape the line did not carry`), so this batch cannot add the row; the script stands registered in occupied-resources (E6)
@apply_row n=7 ref_file=fanout-architecture.md ref_n=2 v=applied edit=E7
@fact id=fct-fa2klbw5 expect_rev=1
old: Every OS-specific branch compiles and is tested on its CI runner.
new: Every OS-specific branch compiles and is tested on its CI runner, and every OS-gated test names its reason in the census binary `tests/contract_os_gated_tests.rs`.

@fact after=fct-7ok4646z file=cross-cutting-patterns gid=new:os-gated-tests kind=procedure subject=crosscut-os-gated-tests
- **OS-gated tests named with their reason.** Every test held off a runner by an OS gate says why. The E2E contract binary `tests/contract_os_gated_tests.rs` (a `contract_` binary, auto-discovered with no `[[test]]` table, run under `run --e2e`, carrying `andromeda:walks-tree`) holds it: its `GATES` table, one `Gate { file, test, gate, reason }` row per OS-gated test function or gated file, names each gate with a non-empty reason. Its census walks every `.rs` directly under `tests/` and under each `crates/*/tests/`, taking a file with a leading `#![cfg(…)]` and a test function whose contiguous attribute run holds `#[test]`, `#[rstest]` or `#[tokio::test]` and a `#[cfg(…)]` or `#[cfg_attr(…)]` whose predicate names `windows`, `unix` or `target_os` outside its string values (so `feature = "unix-like"` is no hit). `os_gated_tests_are_each_named_with_their_reason` fails both ways, on a gate no row names and on a row whose gate no longer stands; `gates_table_rows_are_unique` holds one row per gate; and its self-check, `census_finds_an_unnamed_gate_and_passes_a_named_one`, reads eight planted cases. A new OS gate on a test therefore lands with its `GATES` row and reason, or the census goes red.

@fact id=fct-h3mcbocf expect_rev=1
old: crosscut-sanitised-error-surfaces ·
new: crosscut-sanitised-error-surfaces · crosscut-os-gated-tests ·

@fact id=fct-jv5gqhbl expect_rev=1
old: `Instant::now() + WITHIN` reads 28 sites in 20 files under `tests/`, as measured at chunk 2026-10-10-budget-governor's wrap on the Linux dev host's final tree (two more than at chunk 2026-10-10-viola-revive: one each in `tests/cli_budget.rs`, a new file, and `tests/cli_revive.rs`), the shared
new: `Instant::now() + WITHIN` reads 32 sites in 24 files under `tests/` (`grep -rn 'Instant::now() + WITHIN' tests/ | wc -l` and `grep -rln 'Instant::now() + WITHIN' tests/ | wc -l`), as measured at chunk 2026-11-04-linux-and-macos-parity's wrap on the Linux dev host, the shared
old: not a census of every wait on the bound.
new: not a census of every wait on the bound. `tests/support/frozen.rs` `Frozen` is a user of the bound too: on Unix its `attach` polls `ps -o stat=` under a `Watch` bounded by `super::watch::WITHIN` until the frozen wrapper reads `T`.

@fact after=fct-mimmga3r file=occupied-resources gid=new:frozen-guard kind=values subject=res-filesystem
- Test-only: `tests/support/frozen.rs` (`pub mod frozen;` in `tests/support/mod.rs`) holds the `Frozen` guard, `attach(pid)` and its `Drop`, one twin per OS under its own gate: on Unix `attach` sends `kill -STOP` and polls `ps -o stat=` until it reads `T`, and the drop sends `kill -CONT`; on Windows `attach` attaches a debugger with kill-on-exit off and waits for the first debug event, and the drop detaches. It is the guard's one home, the way a root test freezes a wrapper so that it reads stale; its callers are in `tests/cli_instance_state.rs` and `tests/cross_list_rows.rs`. `viola` never names it.

@fact after=fct-mtiut4fw file=occupied-resources gid=new:junit-os-sets kind=values subject=res-repository
- `scripts/junit-os-sets.py` (the python 3 standard library and the host's `gh`): the operator pass's per-OS JUnit reading. `--run <id>` downloads the `junit-windows-2025`, `junit-macos-latest` and `junit-ubuntu-latest` artifacts with `gh run download` into a `tempfile.mkdtemp()` dir removed on exit, and prints `{os}: {n} cases · {f} failed · {s} skipped` per OS, then `## {label}: {count}` and the sorted `classname::name` lines of `ubuntu-not-macos`, `macos-not-ubuntu`, `windows-not-ubuntu` and `ubuntu-not-windows`. It exits 0, 1 when a report is missing or unparsable (the report named), and 2 on a usage error; it reads no environment variable and writes nothing in the tree. It is an operator-pass instrument, never a CI step.

@fact id=fct-irhgihut expect_rev=1
old: sgr.rs: the SGR reader of a terminal stream (`sgr_attributes`, `redraws`, `sets_a_foreground`)
new: sgr.rs: the SGR reader of a terminal stream (`sgr_attributes`, `redraws`, `sets_a_foreground`); frozen.rs: the shared freeze guard `Frozen` (`attach(pid)` + `Drop`; Unix `kill -STOP` polled to `ps` state `T` within `WITHIN`, `kill -CONT` on drop; Windows a debugger attach, detached on drop), its one home, used by `tests/cli_instance_state.rs` and `tests/cross_list_rows.rs`

@pattern id=arch-01
fixed: reads 28 sites in 20 files
