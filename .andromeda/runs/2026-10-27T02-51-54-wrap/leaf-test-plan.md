group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 105 · «- 2026-10-26: An OS-gated test module in product code stacks»
read: .claude/rules/verification-harness.md · 72 · «- 2026-10-20: `target/debug/viola` is a hard link cargo re-p»

- leaf: .claude/rules/testing.md:33   class: stale
  leaf-text: «Windows by an inherited Users write ACE»
  source: .andromeda/test-plan-detail/test-scope-summary.md:117 «the instance and home-tree checks refuse a `create_dir_all` home (Unix: its 0755 mode fails the read rule; Windows: an inherited write grant to a SID strict-modes does not trust, whichever SID holds it).»
  replace-with: «Windows by an inherited write grant to a SID strict-modes does not trust, read by its right whichever SID holds it»

- leaf: .claude/rules/testing.md:39   class: omission
  leaf-text: «compare kebab-case serde values and integer exits, never `Debug` strings.»
  source: .andromeda/test-plan-detail/integration-test-strategy.md:43 «An oracle for a runner-inherited grant therefore names no SID: it reads the grant by its right, beside a control that cannot pass vacuously (§4 viola-state, the plain-home oracle).»
  replace-with: «compare kebab-case serde values and integer exits, never `Debug` strings. An oracle for a runner-inherited Windows grant names no SID (`windows-2025` runners differ by host in which SID holds a temp dir's inherited write): it reads the grant by its right, beside a control that cannot pass vacuously.»

- leaf: .claude/rules/verification-harness.md:40   class: omission
  leaf-text: «`hook` (with `stdin_hex`, the exact bytes offered, when it ran)»
  source: .andromeda/test-plan-detail/test-data-fixtures.md:59 «and `spawn_os_error` when the spawn of an absolute `command` failed: the raw OS error as an integer, `null` when the OS gives none}»
  replace-with: «`hook` (with `stdin_hex`, the exact bytes offered, when it ran, and `spawn_os_error` when the spawn of an absolute `command` failed: the raw OS error as an integer, `null` when the OS gives none)»

- leaf: .claude/docs/tests-summary.md:31   class: omission
  leaf-text: «transport works, dialog answers withheld (exit 12)»
  source: .andromeda/test-plan-detail/e2e-test-strategy.md:203 «the MCP leg is `mcp_tools_path7_unverified_cli_answer` (`crates/viola-e2e/tests/mcp_tools.rs`), which boots the default `overseer` + `builder` pair unstamped»
  replace-with: «transport works, dialog answers withheld (exit 12); as landed, the MCP leg is `mcp_tools_path7_unverified_cli_answer` (`crates/viola-e2e/tests/mcp_tools.rs`), booting the default `overseer` + `builder` pair unstamped (`SessionOptions { unstamped: true, ..SessionOptions::default() }`), so it also witnesses two first starts on one fresh unstamped home both landing (green on all three CI OSes, ci#40439188053)»
