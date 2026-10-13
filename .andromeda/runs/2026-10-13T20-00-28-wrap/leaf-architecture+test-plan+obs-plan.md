group: architecture+test-plan+obs-plan
read: .claude/docs/commands.md · 95 · «- `scripts/agent-run.sh logs --instance <name> | jq -c 'sele»

- leaf: .claude/docs/commands.md:48   class: stale
  leaf-text: «nine `mutants (<label>)` jobs, one per matrix item (the root package `viola` is four items, split by file)»
  source: .andromeda/registries/contracts/architecture/ci-cd-approach.md:2 «over a ten-item `matrix.include` (counted as its `- package:` items; each item is a `package`, a `label` and Windows-gated `files`). The root package `viola` is five of the ten, split by file»
  replace-with: «ten `mutants (<label>)` jobs, one per matrix item (the root package `viola` is five items, split by file, the fifth `viola-cmd-verify-stop` over `src/cmd/verify/stop.rs`)»

- leaf: .claude/docs/commands.md:26   class: omission
  leaf-text: «exit 1 on a failing row»
  source: .andromeda/architecture.md:139 «`viola verify` stopped by Ctrl-C or TERM before its rows are checked (no stamp written, stdout empty, stderr exactly `unable: verify was stopped before it finished`»
  replace-with: «exit 1 on a failing row; a run stopped by Ctrl-C or TERM kills its child with no key, removes only the probe dirs it created, writes no stamp, leaves stdout empty, prints `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI` and exits 1 (SIGKILL, a crash or a Windows close leaves the dirs to remove by hand)»
