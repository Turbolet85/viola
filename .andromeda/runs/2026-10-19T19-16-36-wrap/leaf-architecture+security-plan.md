group: architecture+security-plan
read: .claude/rules/api.md · 42 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/api.md:4   class: tree
  leaf-text: «  - "crates/viola-ui/src/**"»
  source: .andromeda/architecture.md:185 «**GUI contracts.** The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «»

- leaf: .claude/rules/api.md:24   class: omission
  leaf-text: «`wait`'s `after` / `timeout_ms` are absent or a `u64` and `from` is absent, `null` or a valid `ViolaName`, anything else `-32602` with `data: null`»
  source: .andromeda/architecture.md:235 «each method that reads it (`send`, `wait`, `last`, `answer`, `pause`) takes it absent, `null` or a valid `ViolaName` and answers anything else `-32602` `invalid params` with `data: null`»
  replace-with: «`wait`'s `after` / `timeout_ms` are absent or a `u64`, and `from` (read by `send`, `wait`, `last`, `answer` and `pause`) is absent, `null` or a valid `ViolaName`, anything else `-32602` with `data: null`»

- leaf: .claude/rules/api.md:25   class: stale
  leaf-text: «`release` carrying `from` → `-32602` with `data:{"reason":"release-from-driver"}`.»
  source: .andromeda/architecture.md:245 «A string `from` is refused `-32602` `"invalid params"` with `data: {"reason":"release-from-driver"}` (`viola_channel::ProtocolError::ReleaseFromDriver`) plus one `release-from-driver` obs line»
  replace-with: «`release` carrying a string `from` → `-32602` with `data:{"reason":"release-from-driver"}`; a `from` neither absent, `null` nor a string, or a `budget` that is not a bool (`null` included), → `-32602` with `data: null`.»

- leaf: .claude/rules/api.md:30   class: omission
  leaf-text: «2 usage (never from `viola hook`)»
  source: .andromeda/architecture.md:143 «`2` usage error (clap, a second positional to `viola send` included; never produced by `viola hook`), and `viola send` text over `MAX_FRAME` or not UTF-8»
  replace-with: «2 usage (clap, a second positional to `viola send` included; never from `viola hook`), and `viola send` text over `MAX_FRAME` or not UTF-8»

- leaf: .claude/rules/api.md:35   class: stale
  leaf-text: «`response` through `Response::parse`, `validate_paste_text` (then the empty-text check) on `send`'s text and every free-text answer value; a failure is `isError:true` `{"error":"invalid-params","detail":"<field>"}` (a closed field word) with no frame written.»
  source: .andromeda/security-plan.md:199 «`validate_paste_text`, then the empty-typed-text check, each refused as `{"refusal":"not-delivered","detail":"control-character"|"empty-text"}` with one client-side `send-refused` line»
  replace-with: «`response` through `Response::parse`, a failure being `isError:true` `{"error":"invalid-params","detail":"<field>"}` (a closed field word) with no frame written; then `validate_paste_text` (and on `send` the empty-text check) on `send`'s text and every free-text answer value, a failure being the refusal `{"refusal":"not-delivered","detail":"control-character"|"empty-text"}` (on `send` with one client-side `send-refused` line, before any frame).»
