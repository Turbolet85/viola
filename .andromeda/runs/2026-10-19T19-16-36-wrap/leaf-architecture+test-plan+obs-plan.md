group: architecture+test-plan+obs-plan
read: .claude/docs/commands.md · 89 · «- `scripts/agent-run.sh logs --instance <name> | jq -c 'sele»

- leaf: .claude/docs/commands.md:33   class: stale
  leaf-text: «`logs [--session] [--instance] [--process]`»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:84 «Assertions pipe into `jq -e` (jq 1.8.2) or jaq 3.1.1, for example `agent-run logs --instance builder --kind send-issued | jq -e '.record.data.cursor'`.»
  replace-with: «`logs [--session] [--instance] [--kind] [--process]`»

- leaf: .claude/docs/commands.md:49   class: stale
  leaf-text: «`print_stdout`, `print_stderr` and `dbg_macro` are denied workspace-wide»
  source: .andromeda/registries/contracts/architecture/build-system.md:4 «`viola-e2e` opts out and denies only `dbg_macro`, because the harness prints its JSON document.»
  replace-with: «`print_stdout`, `print_stderr` and `dbg_macro` are denied in every product member (`viola-e2e` opts out and denies only `dbg_macro`)»

- leaf: .claude/docs/commands.md:13   class: omission
  leaf-text: «21 when its strict-modes check refuses the home or any existing `marketplace/` tree dir (nothing created, no log opened)»
  source: .andromeda/architecture.md:277 «`viola plugin install --json`, no path; its refused strict-modes check prints the `instance-unreachable` document (exit 21) through `client::refused_document`.»
  replace-with: «21 when its strict-modes check refuses the home or any existing `marketplace/` tree dir (nothing created, no log opened; under `--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document)»
