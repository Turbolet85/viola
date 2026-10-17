group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 100 · «- 2026-10-17: Write a literal of indented output lines with »
read: .claude/rules/verification-harness.md · 71 · «- 2026-10-14: A side effect added to a fake-agent turn (a sp»

- leaf: .claude/docs/tests-summary.md:26   class: stale
  leaf-text: «the MCP `send` step and its verification owed to `:118`»
  source: .andromeda/test-plan.md:651 «the MCP `send` step landed at chunk 2026-10-17-mcp-server-for-drivers in `crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_path2_path3_send_wait_last_match_the_cli`»
  replace-with: «the MCP `send` step landed at chunk 2026-10-17-mcp-server-for-drivers (`crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_path2_path3_send_wait_last_match_the_cli`, its result compared with the CLI's `--json` document less `v`)»

- leaf: .claude/docs/tests-summary.md:27   class: stale
  leaf-text: «the MCP `wait` / `last` steps owed to `:118`»
  source: .andromeda/test-plan.md:672 «The MCP `wait` / `last` steps landed at chunk 2026-10-17-mcp-server-for-drivers in `crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_path2_path3_send_wait_last_match_the_cli`»
  replace-with: «the MCP `wait` / `last` steps landed at chunk 2026-10-17-mcp-server-for-drivers (`crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_path2_path3_send_wait_last_match_the_cli`, each result compared with the CLI's `--json` document less `v`)»

- leaf: .claude/docs/tests-summary.md:28   class: stale
  leaf-text: «the MCP `answer` step owed to its surface chunk»
  source: .andromeda/test-plan.md:692 «The MCP `answer` step landed at chunk 2026-10-17-mcp-server-for-drivers in `crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_path4_path5_answer_by_id_and_a_paused_send`»
  replace-with: «the MCP `answer` step landed at chunk 2026-10-17-mcp-server-for-drivers (`crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_path4_path5_answer_by_id_and_a_paused_send`, its result and refusal compared with the CLI's `--json` document less `v`)»

- leaf: .claude/docs/tests-summary.md:29   class: stale
  leaf-text: «the MCP `send` step and its refusal checks owed to `:118`»
  source: .andromeda/test-plan.md:719 «The MCP `send` step and its refusal checks landed at chunk 2026-10-17-mcp-server-for-drivers in `crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_path4_path5_answer_by_id_and_a_paused_send`»
  replace-with: «the MCP `send` step and its refusal checks landed at chunk 2026-10-17-mcp-server-for-drivers (`crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_path4_path5_answer_by_id_and_a_paused_send`; no MCP result, body or tool description names `release`)»

- leaf: .claude/docs/tests-summary.md:30   class: stale
  leaf-text: «Still owed: the MCP form of the refusal to "MCP server for drivers".»
  source: .andromeda/test-plan.md:766 «The MCP form of the refusal landed at chunk 2026-10-17-mcp-server-for-drivers as `mcp_tools_path6_budget_paused_send` (`crates/viola-e2e/tests/mcp_tools.rs`)»
  replace-with: «The MCP form of the refusal landed at chunk 2026-10-17-mcp-server-for-drivers as `mcp_tools_path6_budget_paused_send` (`crates/viola-e2e/tests/mcp_tools.rs`: `isError:true`, `{"refusal":"budget-paused","detail":"five-hour"}`, equal to the CLI's document less `v`).»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «`StampedHome::unstamped` for the unverified-path tests)»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:26 «`stamp: !opts.unstamped` (`SessionOptions::unstamped`, default false, so the default session is stamped — the in-test form of `boot --unstamped`»
  replace-with: «`StampedHome::unstamped` for the unverified-path tests; `SessionOptions::unstamped`, default false, the in-test form of `boot --unstamped` for a harness session)»

- leaf: .claude/rules/testing.md:29   class: omission
  leaf-text: «`start_paused` (Tokio)»
  source: .andromeda/test-plan.md:935 «`viola-mcp` owns no timer — its `wait` bound is the pure `wait_timeout(given)` = `min(given, 50 000)`, a unit case — so no `start_paused` test exists»
  replace-with: «`start_paused` (Tokio; tokio `test-util` in no manifest yet, and `viola-mcp` owns no timer — its `wait` bound is the pure `wait_timeout(given)`, a unit case)»

- leaf: .claude/rules/verification-harness.md:30   class: omission
  leaf-text: «(a plain function and drop guard, never an rstest fixture; the library `boot` spawns the `supervise` process from the given binary dir)»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:26 «`stamp: !opts.unstamped` (`SessionOptions::unstamped`, default false, so the default session is stamped — the in-test form of `boot --unstamped`»
  replace-with: «(a plain function and drop guard, never an rstest fixture; the library `boot` spawns the `supervise` process from the given binary dir; the session is stamped unless `SessionOptions::unstamped`, default false, the in-test form of `boot --unstamped`)»
