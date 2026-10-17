group: test-plan+obs-plan
read: .claude/rules/observability.md · 46 · «- 2026-09-24: `obs_event!` expands to `::tracing::event!`, s»

- leaf: .claude/rules/observability.md:40   class: retired
  leaf-text: «`viola revive --list` opens no process log (the sixth bounded exemption, standing only until it gains `--json` on the route entry "CLI machine contract")»
  source: .andromeda/obs-plan.md:530 «`viola revive <name> --list` (chunk 2026-10-17-cli-machine-contract) is a `cli` verb that, after its strict-modes check passes and only under a parsed `VIOLA_NAME`, opens the CALLER's `cli-<VIOLA_NAME>.ndjson`»
  replace-with: «`viola revive --list` is role `cli` (chunk 2026-10-17-cli-machine-contract; the sixth exemption retired): after its strict-modes check passes and only under a parsed `VIOLA_NAME` it writes `process-start` / `process-exit` (`exit_code` 0 with no detail; `exit_code` 1 `detail:"no-session"` on an empty chain) into the caller's `cli-<VIOLA_NAME>.ndjson`, with no name no log, and its dispatch `Err` or panic reaches the `cli` catch site»

- leaf: .claude/rules/observability.md:40   class: omission
  leaf-text: «A refused instance check on the start arm or on `viola hook statusline` opens no role file and writes no line»
  source: .andromeda/obs-plan.md:989 «7. a refused instance check on either `viola revive` arm (start or `--list`) or on `viola hook statusline` (chunk 2026-10-10-budget-governor)»
  replace-with: «A refused instance check on either `viola revive` arm (start or `--list`) or on `viola hook statusline` opens no role file and writes no line»

- leaf: .claude/rules/observability.md:21   class: omission
  leaf-text: «`viola revive`'s start arm runs its instance check before the log init as well: a refused check is the stderr pair and exit 1 with no log opened.»
  source: .andromeda/obs-plan.md:983 «the instance check of both `viola revive` arms (the start arm and `--list`, whose check runs before its `cli` role file opens)»
  replace-with: «Both `viola revive` arms (the start arm and `--list`) run their instance check before the log init as well: a refused check is the stderr pair, or under `--json` the `{"v":1,"error":"strict-modes-failed","detail":null}` document, and exit 1 with no log opened.»

- leaf: .claude/rules/observability.md:39   class: omission
  leaf-text: «one `state-recovered` per replayed snapshot»
  source: .andromeda/obs-plan.md:656 «the line carries `instance` the caller and `scanned` the scanned instance's name (the instance dir's own last component when it parses as a `ViolaName`, absent otherwise; chunk 2026-10-17-cli-machine-contract)»
  replace-with: «one `state-recovered` per replayed snapshot (its `instance` the caller, its `scanned` the scanned instance dir's own last component when it parses as a `ViolaName`, absent otherwise — never a path or a literal `null`)»
