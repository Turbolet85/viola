group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 92 · «- 2026-10-11: A tree-equality assertion over a viola home co»
read: .claude/rules/verification-harness.md · 71 · «- 2026-10-11: On the Linux dev host `schema-check` (G4) read»

- leaf: .claude/docs/tests-summary.md:9   class: stale
  leaf-text: «founder-mandated mutation testing from chunk 1»
  source: .andromeda/test-plan.md:28 «_the founder mandates mutation testing: per chunk from chunk 1, and since 2026-09-28 at the epoch boundary through `/andromeda-code-audit` (the founder's ruling, 2026-09-28 17:59)_»
  replace-with: «founder-mandated mutation testing (per chunk from chunk 1, at the epoch boundary through `/andromeda-code-audit` since 2026-09-28)»

- leaf: .claude/docs/tests-summary.md:17   class: omission
  leaf-text: «**Status:** `agent-run status` aggregates `viola list --json`, `GET /ready` and cookie-gated `/api/sessions` (`state: ready|degraded|down`, `api_sessions_equal_list`)»
  source: .andromeda/registries/contracts/test-plan/status-endpoint-shape.md:19 «Interim, while `status` runs no `viola list` (built at chunk 2026-10-11-the-board-viola-list; the harness reading it is owed to the Epoch 5 entry "End-to-end path binary and harness legs")»
  replace-with: «**Status:** `agent-run status` aggregates `viola list --json`, `GET /ready` and cookie-gated `/api/sessions` (`state: ready|degraded|down`, `api_sessions_equal_list`); interim, while `status` runs no `viola list` (its harness reading owed to "End-to-end path binary and harness legs") and `viola ui` is unbuilt, `list`, `ui`, `pid`, `uptime_ms` and `api_sessions_equal_list` are `null` and `state` is `ready` when every recorded wrapper is alive, else `degraded`»

- leaf: .claude/docs/tests-summary.md:30   class: omission
  leaf-text: «and the web signals to Epoch 8.»
  source: .andromeda/test-plan.md:863 «the MCP form of the refusal to "MCP server for drivers"»
  replace-with: «the web signals to Epoch 8, and the MCP form of the refusal to "MCP server for drivers".»

- leaf: .claude/rules/verification-harness.md:34   class: omission
  leaf-text: «exit 0 only for `state:"ready"`.»
  source: .andromeda/registries/contracts/test-plan/status-endpoint-shape.md:19 «Interim, while `status` runs no `viola list` (built at chunk 2026-10-11-the-board-viola-list; the harness reading it is owed to the Epoch 5 entry "End-to-end path binary and harness legs")»
  replace-with: «exit 0 only for `state:"ready"`. Interim, while `status` runs no `viola list` (owed to "End-to-end path binary and harness legs") and `viola ui` is unbuilt: `list`, `ui`, `pid`, `uptime_ms` and `api_sessions_equal_list` are `null`, and `state` is `ready` when every recorded wrapper is alive, else `degraded`.»

- leaf: .claude/rules/verification-harness.md:35   class: stale
  leaf-text: «(endpoint/port/url fields `null` until those surfaces exist)»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:92 «`endpoint_gone` is reported from the wrapper channel on; `port_free` and `url_file_removed` are `null` until their surfaces exist.»
  replace-with: «(`endpoint_gone` reported from the wrapper channel on; `port_free` and `url_file_removed` `null` until their surfaces exist)»

- leaf: .claude/rules/verification-harness.md:29   class: omission
  leaf-text: «both pids alive by pid + start time; then»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:13 «(missing: `<name>:run-process-start`, `<name>:child-process-start`); a `process-exit` for `subject:"self"` first is `run-exited`»
  replace-with: «both pids alive by pid + start time (missing `<name>:run-process-start` / `<name>:child-process-start`; a `process-exit` for `subject:"self"` first is `run-exited`); then»

- leaf: .claude/rules/verification-harness.md:39   class: omission
  leaf-text: «with only `prompt` set for UserPromptSubmit and the fixture's trailing newline kept, so the recorded prompt yields the recorded bytes.»
  source: .andromeda/test-plan.md:1076 «under `--resume <id>` the SessionStart fired at launch is the recorded `SessionStart.default` payload with `source` set to `resume` and `session_id` set to the id given»
  replace-with: «with only `prompt` set for UserPromptSubmit and the fixture's trailing newline kept, so the recorded prompt yields the recorded bytes; under `--resume <id>` the launch SessionStart is the recorded `SessionStart.default` with `source` `resume` and `session_id` the id given (with `--fork-session`, the compiled fork id), and no other payload is built.»

- leaf: .claude/rules/verification-harness.md:41   class: omission
  leaf-text: «`size` (a watcher writes it at start and on every change, no key needed; interactive mode only), `step`.»
  source: .andromeda/test-plan.md:1084 «`hold` {`pid`}: the `--exit-no-eof` holder, at its start»
  replace-with: «`size` (a watcher writes it at start and on every change, no key needed; interactive mode only), `step`, `cwd`, `hold` (the `--exit-no-eof` holder, at its start), `statusline` (the `hook` kind's fields without `event`).»

- leaf: .claude/rules/verification-harness.md:42   class: omission
  leaf-text: «`agents --json` lands with its consumer.»
  source: .andromeda/test-plan.md:1093 «and two for `viola revive`'s tests, `--resume <id>` (the launch SessionStart reports source `resume` and that id) and `--fork-session` (beside `--resume`, the compiled fork id; alone it changes nothing)»
  replace-with: «Built for `viola revive`'s tests: `--resume <id>` (the launch SessionStart reports source `resume` and that id) and `--fork-session` (beside `--resume`, the compiled fork id; alone it changes nothing); and the capped test-only hold `--stop-receipt-hold-ms` (1 000 ms at most, holding the Stop hook's receipt to force a race window open). `agents --json` lands with its consumer.»

- leaf: .claude/rules/testing.md:19   class: omission
  leaf-text: «the started-child guard and runner in `cli.rs`,»
  source: .andromeda/test-plan.md:456 «`ndjson.rs` is the one complete-lines reader: every reader of a live-appended file parses only newline-terminated lines, because a Linux reader can observe an append part-way.»
  replace-with: «the started-child guard and runner in `cli.rs`, the one complete-lines reader in `ndjson.rs` (every reader of a live-appended file parses only newline-terminated lines),»

- leaf: .claude/rules/testing.md:24   class: omission
  leaf-text: «rstest case labels readable (`#[case::esc_refused]`)»
  source: .andromeda/test-plan.md:524 «its table-driven cases are a labelled case table (an array of `(label, …)` rows in one `#[test]`, each assertion naming its label)»
  replace-with: «rstest case labels readable (`#[case::esc_refused]`; `viola-e2e` carries no `[dev-dependencies]`, so its table-driven cases are a labelled case table of `(label, …)` rows in one `#[test]`, each assertion naming its label)»

- leaf: .claude/rules/testing.md:34   class: omission
  leaf-text: «so the product's first-start write stays covered.»
  source: .andromeda/test-plan.md:628 «so a case that drives a check-guarded path on every OS starts from `stamped_home` (§7 Seed strategies)»
  replace-with: «so the product's first-start write stays covered; a seeded home fails `hook statusline`'s instance check on Windows, so a case that drives a check-guarded path on every OS starts from `stamped_home`.»
