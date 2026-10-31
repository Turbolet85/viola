group: security-plan
read: .claude/docs/security-summary.md · 77 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 44 · «_This section is owned by `/wrap-session`. setup-project pre»
- leaf: .claude/rules/security.md:40   class: omission
  leaf-text: «thiserror `Display` impls use fixed messages (no paths, no payloads)»
  source: .andromeda/security-plan.md:340 «The `Display` impls on the product error enums (`thiserror`, or hand-written for `PtyError`; §Error Handling lists them) must not interpolate upstream text or absolute paths.»
  replace-with: «The `Display` impls on the product error enums (`thiserror`, or hand-written for `PtyError`) use fixed messages (no paths, no payloads)»
- leaf: .claude/rules/security.md:40   class: omission
  leaf-text: «MCP `isError` results carry codes only (`refusal`/`detail`, or `instance-unreachable`, `wrapper-fault`, `invalid-params`), text `refused: <code>`.»
  source: .andromeda/security-plan.md:413 «a `refusal` that is no code word (`viola_core::is_code_word`) reading `unknown` and a string `detail` that is none reading null, `of_reply` here and `reply_of` on CLI `--json` alike»
  replace-with: «MCP `isError` results carry codes only (`refusal`/`detail`, or `instance-unreachable`, `wrapper-fault`, `invalid-params`), text `refused: <code>`. A client reads a reply through `reply_of` (CLI `--json`) and `of_reply` (MCP): a `refusal` or a string `detail` only as a code word (`viola_core::is_code_word`; a failing `refusal` reads `unknown`, a failing `detail` null; a word outside the closed list stays visible by design), a fault only through `viola_channel::sanitised_fault`, which closes its `message` and `data` (only two `-32602` bodies keep `data`).»
- leaf: .claude/rules/security.md:41   class: omission
  leaf-text: «Full detail goes only to `instances/<name>/diagnostics/detail-*.ndjson` (0600) when an instance resolves.»
  source: .andromeda/security-plan-detail/error-handling.md:29 «`product_error_writers_are_each_named` fails on an unnamed product error writer and `thiserror_messages_interpolate_nothing` on an interpolating `thiserror` message.»
  replace-with: «Full detail goes only to `instances/<name>/diagnostics/detail-*.ndjson` (0600) when an instance resolves. Every product error writer is named in `tests/contract_error_bodies.rs`: `product_error_writers_are_each_named` fails on an unnamed one and `thiserror_messages_interpolate_nothing` on an interpolating `thiserror` message.»
- leaf: .claude/rules/security.md:19   class: stale
  leaf-text: «`release` carrying `from` → `-32602 release-from-driver`»
  source: .andromeda/security-plan-detail/authentication-authorization.md:30 «The wrapper rejects a `release` whose `params` carry a string `from` with `-32602`»
  replace-with: «`release` carrying a string `from` → `-32602 release-from-driver` (a `from` of another type → `-32602` with `data: null`)»
- leaf: .claude/docs/security-summary.md:42   class: stale
  leaf-text: «`release` with `from` → `-32602 release-from-driver`»
  source: .andromeda/security-plan.md:116 «the wrapper refuses a `release` whose params carry a string `from` (`-32602`, `release-from-driver`)»
  replace-with: «`release` with a string `from` → `-32602 release-from-driver`»
