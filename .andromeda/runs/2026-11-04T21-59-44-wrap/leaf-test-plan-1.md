@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: 
@leaf_read leaf=.claude/rules/testing.md lines=107
last: - 2026-10-31: A case that runs on every OS must not call a h
@leaf_edit n=1 leaf=.claude/rules/testing.md line=18 class=omission
leaf-text: the SGR parser `sgr_attributes` (with `redraws` and `sets_a_foreground`) in `sgr.rs`, its self-check case staying in `tests/cli_output_plain.rs`, and a root test file defines no copy of its own)
source: fct-hckuzazj «`frozen.rs` holds `Frozen`, the one freeze guard (`attach(pid)` and `Drop`): on Unix `kill -STOP`, `ps -o stat=` polled until `T` within `WITHIN`, and `kill -CONT` on drop»
replace-with: the SGR parser `sgr_attributes` (with `redraws` and `sets_a_foreground`) in `sgr.rs`, its self-check case staying in `tests/cli_output_plain.rs`, the one freeze guard `Frozen` in `frozen.rs` (`attach(pid)` and `Drop`: on Unix `kill -STOP`, `ps -o stat=` polled until `T` within `WITHIN`, and `kill -CONT` on drop; on Windows a debugger attach, detached on drop), and a root test file defines no copy of its own)
@leaf_edit n=2 leaf=.claude/rules/testing.md line=19 class=omission
leaf-text: and its `cross_` binary (`tests/cross_exit_causes.rs`, sync, `[[test]]` `required-features = ["fake-agent"]`);
source: fct-gldxhydz «root `contract_os_gated_tests.rs`: every OS-gated test function and gated file under `tests/` and `crates/*/tests/` a `GATES` row with its reason, an unnamed or stale gate failing the run»
replace-with: and its `cross_` binary (`tests/cross_exit_causes.rs`, sync, `[[test]]` `required-features = ["fake-agent"]`), and its `contract_` binary `tests/contract_os_gated_tests.rs` (auto-discovered, no `[[test]]` table; every OS-gated test function and gated file under `tests/` and `crates/*/tests/` a `GATES` row with its reason, an unnamed or stale gate failing the run);
