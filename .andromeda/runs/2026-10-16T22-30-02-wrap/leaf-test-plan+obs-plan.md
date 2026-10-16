group: test-plan+obs-plan
read: .claude/rules/observability.md · 46 · «- 2026-09-24: `obs_event!` expands to `::tracing::event!`, s»

- leaf: .claude/rules/observability.md:40   class: stale
  leaf-text: «each preflight refusal — `already-live`, `no-session`, `cwd-missing` — one `process-exit{exit_code:1, detail}`)»
  source: .andromeda/obs-plan.md:754 «and from `viola revive`'s start-arm preflight `no-session` (no logged session, or an `--id` the log does not hold), `session-live` (one `claude agents --json` read lists the logged session id live»
  replace-with: «each preflight refusal — `already-live`, `no-session`, `session-live` (the thirteenth `process-exit` `detail` value), `cwd-missing` — one `process-exit{exit_code:1, detail}`; its `session-live` reading, skipped under `--fork` and passing on a failed read, writes one `process-start` / `process-exit{subject:"agents-probe"}` pair there, plus one `parse-rejected{parser:"claude-agents-json"}` on an oversize or malformed answer)»

- leaf: .claude/rules/observability.md:39   class: omission
  leaf-text: «one `state-recovered` per replayed snapshot and the `config-json` threshold rejections into `cli-<name>.ndjson`»
  source: .andromeda/obs-plan.md:788 «the `agents-probe` pair is written by `run::agents::read_logged` for `viola list` (into `cli-<name>.ndjson`, only under a parsed `VIOLA_NAME`)»
  replace-with: «one `state-recovered` per replayed snapshot, the `config-json` threshold rejections and one `agents-probe` spawn pair around its `claude agents --json` read (plus one `parse-rejected{parser:"claude-agents-json"}` on an oversize or malformed answer) into `cli-<name>.ndjson`»
