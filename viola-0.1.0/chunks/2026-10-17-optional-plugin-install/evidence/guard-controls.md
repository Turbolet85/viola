# Remove-the-guard control (testing.md 2026-09-25)

The guard: `src/cmd/plugin.rs` `install`'s strict-modes check, `if !tree_passes(&home) {` (line 52), before
anything under the home is created, written or opened.

| reading (2026-10-17, Linux dev host) | `bash scripts/agent-run.sh run --integration --filter 'binary(cli_plugin_install)'` |
|---|---|
| guard neutralised (`if false && !tree_passes(&home) {`, read back at line 52 before the run) | exit 1 · `nextest-integration` passed 8, failed 1 · `plugin_install_refuses_a_widened_home` red at `tests/cli_plugin_install.rs:265`: `left: Some(0)`, `right: Some(21)` |
| guard restored (`if !tree_passes(&home) {`, read back at line 52) | exit 0 · passed 9, failed 0 |

The unit `tree_passes_refuses_any_dir_another_user_could_write` holds the per-directory half: each of the five
directories set to 0770 in turn reads refused.
