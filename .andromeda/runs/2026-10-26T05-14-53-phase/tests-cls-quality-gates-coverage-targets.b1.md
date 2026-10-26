# Classified — tests · quality-gates-coverage-targets.b1

## Answers
quality-gates-coverage-targets#1.1 · yes · the chunk re-runs `agent-run run --coverage`, and the E2E MCP stdio path spawns a `viola mcp` child whose profile the merge must accept, so the env_clear re-add rule and the refused-profile mechanism bind
quality-gates-coverage-targets#2.0 · no · the mutation gate is judged only at the epoch boundary through `/andromeda-code-audit`, never by a chunk (the task's Not needed list), and the chunk runs no `run --mutants`
quality-gates-coverage-targets#3.1 · no · the `--perf` session times the sync hook path, which no chunk change reaches, and the task's Not needed list drops the perf job's hook-deadline rows
