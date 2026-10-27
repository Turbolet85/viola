group: obs-plan
read: .claude/docs/obs-summary.md · 76 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »

- leaf: .claude/docs/obs-summary.md:17   class: omission
  leaf-text: «stderr JSON only when that file cannot be opened, D-09)»
  source: .andromeda/obs-plan-detail/error-capture-reporting.md:9 «- A role file or a detail file is opened only past `check_diagnostics` (`src/obs.rs`, in `open_role_file` and `write_detail`), which runs `strict::check_diagnostics` on the existing `diagnostics/` fol»
  replace-with: «stderr JSON only when that file cannot be opened, D-09); every role or detail file opens only past `check_diagnostics` on the existing `diagnostics/` folder and the file (a symlink, or a folder or file another user may read, is refused as a failed open: the role writer a sink, the detail line dropped)»

- leaf: .claude/docs/obs-summary.md:31   class: omission
  leaf-text: «logger init = 1 open + appends»
  source: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:31 «so the timed instance must pass that check. Logger init's one `open` follows the instance check's and `check_diagnostics`' metadata reads.»
  replace-with: «logger init = 1 open + appends, after the hook's instance strict-modes check and `check_diagnostics`' metadata reads, all inside the gate (the timed instance must pass that check, else the gate passes vacuously)»

- leaf: .claude/docs/obs-summary.md:67   class: omission
  leaf-text: «its strict-modes refusal writing no line since chunk 2026-10-10-budget-governor (on `--list` too)»
  source: .andromeda/obs-plan-detail/telemetry-strategy.md:43 «a strict-modes refusal made before any process log opens (the six frame verbs and `link` / `unlink`, exit 21; `run`, `revive`'s start and `verify`, exit 1) writes no line and carries no `detail` code»
  replace-with: «its strict-modes refusal writing no line since chunk 2026-10-10-budget-governor (on `--list` too); a strict-modes refusal made before any process log opens (`run`'s, a revived start's and `verify`'s home-tree check, exit 1; the six frame verbs' and `link` / `unlink`'s instance check, exit 21) writes no line and carries no `detail` code»

- leaf: .claude/docs/obs-summary.md:72   class: omission
  leaf-text: «exemption 6 retired)»
  source: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:18 «8. a refused strict-modes check on the hook event and dialog paths (exit 0, nothing printed), on `send`, `wait`, `last`, `answer`, `pause` or `release` (exit 21, the instance pair or under `--json` the»
  replace-with: «exemption 6 retired; exemption 8: a refused strict-modes check on the hook event and dialog paths (exit 0), on `send`, `wait`, `last`, `answer`, `pause` or `release` (exit 21, the instance pair or the `instance-unreachable` document), on `viola verify`'s home tree (exit 1, the home pair) or on the home tree of `viola run` or `viola revive`'s start arm (exit 1, the home or the instance pair) likewise opens no role file and writes no line)»
