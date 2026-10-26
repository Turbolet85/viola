group: test-plan+obs-plan
read: .claude/rules/observability.md · 46 · «- 2026-09-24: `obs_event!` expands to `::tracing::event!`, s»

- leaf: .claude/rules/observability.md:40   class: stale
  leaf-text: «the six CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`)»
  source: .andromeda/obs-plan-detail/log-coverage.md:29 «`server-verify-failed` is written by the eight CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`)»
  replace-with: «the eight CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`; `link` / `unlink` into the driven instance's `cli-<driven>.ndjson`)»

- leaf: .claude/rules/observability.md:40   class: omission
  leaf-text: «exit 21, the stderr pair or under `--json` the `instance-unreachable` document), opens no role file»
  source: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:17 «or a refused `check_instance` of the driven instance on `viola link` / `viola unlink`: it opens no role file and writes no line»
  replace-with: «exit 21, the stderr pair or under `--json` the `instance-unreachable` document), or `viola link` / `viola unlink`'s refused `check_instance` of the driven instance (run before their `cli` role file opens; exit 21, the fixed instance-state pair or under `--json` the `instance-unreachable` document), opens no role file»

- leaf: .claude/rules/observability.md:41   class: omission
  leaf-text: «The implicit link record on a `send` or an `answer`, the link set's restore at start and the replay's link rule carry no span and no process-log line either;»
  source: .andromeda/obs-plan.md:392 «the explicit `link` / `unlink` methods the driven wrapper serves have no process-log line, no `event` value and no span of their own»
  replace-with: «The implicit link record on a `send` or an `answer`, the explicit `link` / `unlink` methods the driven wrapper serves (their trace the call's `channel-request` / `channel-response` pair, which holds neither name), the link set's restore at start and the replay's link rule carry no span and no process-log line of their own;»

- leaf: .claude/rules/observability.md:21   class: omission
  leaf-text: «and exit 1 with no log opened.»
  source: .andromeda/registries/contracts/obs-plan/otel-sdk-init.md:23 «run `check_instance` on the home and `instances/<driven>` BEFORE the log init (`client::start`)»
  replace-with: «and exit 1 with no log opened. `viola link` / `viola unlink` run `check_instance` on the home and `instances/<driven>` before the log init too: a refusal is exit 21, the fixed instance-state pair or under `--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document, with no role file opened and the directory's mode left as found.»

- leaf: .claude/rules/observability.md:39   class: omission
  leaf-text: «so the tree check and its refusal run outside it; no path in any line.»
  source: .andromeda/obs-plan-detail/obs-scope-summary.md:158 «`viola link` / `viola unlink` log as `cli` into the driven instance's `cli-<driven>.ndjson` (the instance their `<driven>` argument names, never the caller's `VIOLA_NAME`)»
  replace-with: «so the tree check and its refusal run outside it; no path in any line. `viola link` / `viola unlink` log as `cli` into the driven instance's `cli-<driven>.ndjson` (never the caller's `VIOLA_NAME`), their request inside spans `cli.link` / `cli.unlink`, once their instance check of it has passed.»
