group: obs-plan
read: .claude/docs/obs-summary.md · 76 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules:»

- leaf: .claude/docs/obs-summary.md:70   class: retired
  leaf-text: «The `method:"link"|"unlink"` channel pair of edge flow E3 belongs to the explicit methods, which are not built.»
  source: .andromeda/obs-plan.md:392 «the explicit `link` / `unlink` methods the driven wrapper serves have no process-log line, no `event` value and no span of their own»
  replace-with: «The explicit `link` / `unlink` methods the driven wrapper serves (chunk 2026-10-26-explicit-session-links) have no process-log line, no `event` value and no span of their own either: their trace is the `method:"link"|"unlink"` `channel-request` / `channel-response` pair (holding neither name; a bad name or pair is `-32602`, no line), the product `link` / `unlink` line in `events.ndjson` and the snapshot's `links`; the `viola link` / `viola unlink` verbs log into the driven instance's `cli-<driven>.ndjson`, their request inside `cli.link` / `cli.unlink`.»

- leaf: .claude/docs/obs-summary.md:72   class: omission
  leaf-text: «and `viola plugin install`'s refused check of its home or a `marketplace/` tree dir, which exits 21 (chunk 2026-10-17-optional-plugin-install); exemption 6 retired)»
  source: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:17 «or a refused `check_instance` of the driven instance on `viola link` / `viola unlink`: it opens no role file and writes no line»
  replace-with: «`viola plugin install`'s refused check of its home or a `marketplace/` tree dir, which exits 21 (chunk 2026-10-17-optional-plugin-install), and `viola link` / `viola unlink`'s refused `check_instance` of the driven instance, made before their `cli` role file opens, which exits 21 (the fixed instance-state pair, or under `--json` the `instance-unreachable` document); exemption 6 retired)»
