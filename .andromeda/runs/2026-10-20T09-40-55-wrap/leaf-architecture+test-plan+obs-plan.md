group: architecture+test-plan+obs-plan
read: .claude/docs/commands.md · 89 · «- `scripts/agent-run.sh logs --instance <name> | jq -c 'sele»

- leaf: .claude/docs/commands.md:59   class: omission
  leaf-text: «no CI job, `pre-push` stage or harness command runs it)»
  source: .andromeda/architecture.md:395 «Its second form, `--case <binary stem> <test name> <runs> <workers> [<name>]`, runs one integration case of the same instrumented build the same way»
  replace-with: «no CI job, `pre-push` stage or harness command runs either form). Its second form, `bash scripts/profraw-census.sh --case <binary stem> <test name> <runs> <workers> [<name>]`, runs one integration case of the same instrumented build the same way, offers each profile alone to `llvm-profdata merge` and keeps a refused one under `refused/` (last line `profraw-census: runs N · passed N · profiles P · refused R`; exit 0 only when every run passed and R is 0; exit 2 with no instrumented binary or no `llvm-profdata`)»

- leaf: .claude/docs/commands.md:87   class: omission
  leaf-text: «or the next start puts a plain directory back on the shared volume with no signal)»
  source: .andromeda/architecture.md:388 «A linked base whose filesystem reads less than `HEADROOM_FLOOR_MIB` (9 GiB) free by `statvfs` (`f_bavail × f_frsize`) is refused too, with the fixed message `e2e-home backing: {free} MiB free, below the {floor} MiB floor`»
  replace-with: «or the next start puts a plain directory back on the shared volume with no signal; a start through the link is refused with `e2e-home backing: {free} MiB free, below the {floor} MiB floor` when its filesystem reads less than the 9 GiB headroom floor free, a compiled floor set by no variable, flag or config key)»
