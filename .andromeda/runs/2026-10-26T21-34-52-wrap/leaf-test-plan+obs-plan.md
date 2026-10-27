group: test-plan+obs-plan
read: .claude/rules/observability.md · 46 · «- 2026-09-24: `obs_event!` expands to `::tracing::event!`, so»

- leaf: .claude/rules/observability.md:21   class: stale
  leaf-text: «`viola hook statusline` runs its instance check BEFORE this init (the role-file open sets an existing home to 0700, so a check made after it passes a widened home) and a refused check writes no line and opens no role file (exit 0 at once)»
  source: .andromeda/registries/contracts/obs-plan/otel-sdk-init.md:23 «Every `viola hook` arm but the two `--capture` arms (the event and dialog paths and `viola hook statusline`) runs its instance check (`instance_of(…)`, then `canonical` and `trusted` in `src/cmd/hook.rs`»
  replace-with: «Every `viola hook` arm but the two `--capture` arms (the event and dialog paths and `viola hook statusline`) runs its instance check (`canonical`, then `trusted`: `check_instance`) BEFORE this init (the role-file open sets an existing home to 0700, so a check made after it passes a widened home) and a refused or absent instance (a `VIOLA_DIR` naming no existing instance directory included) writes no line and opens no role file (exit 0 at once; the event and dialog paths also read no stdin, send no frame and make no SessionEnd append)»

- leaf: .claude/rules/observability.md:21   class: omission
  leaf-text: «`viola link` / `viola unlink` run `check_instance` on the home and `instances/<driven>` before the log init too»
  source: .andromeda/registries/contracts/obs-plan/otel-sdk-init.md:26 «`send`, `wait`, `last`, `answer`, `pause` and `release` take the same check through the one `client::refuse_untrusted` that `link` / `unlink` share (`check_instance` on the absolute home and the target's»
  replace-with: «`viola link` / `viola unlink` (on `instances/<driven>`) and `send`, `wait`, `last`, `answer`, `pause` and `release` (on their named instance; `send` after its text read) run `check_instance` on the home and the instance through the one `client::refuse_untrusted` before the log init too»

- leaf: .claude/rules/observability.md:21   class: omission
  leaf-text: «with no role file opened and the directory's mode left as found.»
  source: .andromeda/registries/contracts/obs-plan/otel-sdk-init.md:21 «Before the log init, `run` and `viola revive`'s start arm (the latter after its `check_instance`) run `checked_exe`: the exe's bytes read once, their `pin_key`, then `strict::check_home_tree`»
  replace-with: «with no role file opened and the directory's mode left as found. `viola run` and `viola revive`'s start arm (after its instance check) run the home-tree check `strict::check_home_tree` (through `checked_exe`: the home, the `check_instance` set, `ledger/`, `bin/`, `plugin/`, `bin/<key>/`, the pinned copy, `plugin/<key>/` and its `hooks/` and `.claude-plugin/`, each that exists) before the wrapper log: a refusal is exit 1 with the home pair (the instance pair for an instance-level refusal; under `viola revive --json` the `strict-modes-failed` document), no log opened, no line and no child.»

- leaf: .claude/rules/observability.md:38   class: omission
  leaf-text: «A `viola verify` stopped by Ctrl-C or TERM»
  source: .andromeda/registries/contracts/obs-plan/otel-sdk-init.md:27 «`viola verify` runs `strict::check_home_tree` with no name and no key (the home to the read rule, `ledger/`, `bin/`, `plugin/`, each that exists) before its `cli` log opens: a refusal is exit 1 with the home pair»
  replace-with: «`viola verify` runs `strict::check_home_tree` on the home, `ledger/`, `bin/` and `plugin/` before its `cli` log opens: a refused tree is exit 1 with the home pair, no step line, no log and no line; a refused `update_stamps` (`StampsError::StrictModes`, nothing read or written) prints the same pair and exits 1 through its own `process-exit`. A `viola verify` stopped by Ctrl-C or TERM»

- leaf: .claude/rules/observability.md:40   class: omission
  leaf-text: «opens no role file and writes no line (the seventh exemption; on `viola hook statusline` the founder's own answer, 2026-10-13).»
  source: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:18 «8. a refused strict-modes check on the hook event and dialog paths (exit 0, nothing printed), on `send`, `wait`, `last`, `answer`, `pause` or `release` (exit 21, the instance pair or under `--json` the»
  replace-with: «opens no role file and writes no line (the seventh exemption; on `viola hook statusline` the founder's own answer, 2026-10-13). So does a refused strict-modes check on the hook event and dialog paths (exit 0, nothing printed), on `send`, `wait`, `last`, `answer`, `pause` or `release` (exit 21, the instance pair or under `--json` the `instance-unreachable` document), on `viola verify`'s home tree (exit 1, the home pair) or on the home tree of `viola run` or `viola revive`'s start arm (exit 1, the home or the instance pair) (the eighth exemption).»

- leaf: .claude/rules/observability.md:40   class: omission
  leaf-text: «MCP's frame tools return `instance-unreachable` with no line, as the liveness pre-check's»
  source: .andromeda/obs-plan-detail/error-capture-reporting.md:37 «`viola mcp`'s frame tools (`send`, `wait`, `last`, `answer`) run `check_instance` on the home and the instance directory in `frame` (`crates/viola-mcp/src/tools.rs`) before `live_endpoint`»
  replace-with: «MCP's frame tools return `instance-unreachable` with no line, as the liveness pre-check's and as their own refused `check_instance` of the home and the instance directory (made before `live_endpoint`, `detail: null`, no frame)»

- leaf: .claude/rules/observability.md:20   class: omission
  leaf-text: «A failed log write is swallowed — `hook` still exits 0.»
  source: .andromeda/obs-plan-detail/error-capture-reporting.md:9 «A role file or a detail file is opened only past `check_diagnostics` (`src/obs.rs`, in `open_role_file` and `write_detail`), which runs `strict::check_diagnostics` on the existing `diagnostics/` folder»
  replace-with: «A failed log write is swallowed — `hook` still exits 0. `open_role_file` and `write_detail` (`src/obs.rs`) run `strict::check_diagnostics` on the existing `diagnostics/` folder and on the role or detail file before `ensure_private_dir` narrows anything (a symlink refused, a folder held to the read rule, a file to the read rule on Unix and the write rule on Windows, an absent path passing): a refusal is a failed open — the role writer ends at `std::io::sink` and the detail line is dropped.»
