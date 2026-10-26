group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 105 · «- 2026-10-26: An OS-gated test module in product code stacks»
read: .claude/rules/verification-harness.md · 72 · «- 2026-10-20: `target/debug/viola` is a hard link cargo re-p»

- leaf: .claude/rules/verification-harness.md:41   class: omission
  leaf-text: «cuts a `viola hook` short in its exit-time coverage write (a cut-short profile, measured).»
  source: .andromeda/test-plan.md:670 «The agent itself, its PTY's session leader, cannot leave that group (`setpgid` refuses a leader), so on Unix its exit ignores SIGHUP from its first step (`\x03` or stdin's end)»
  replace-with: «cuts a `viola hook` short in its exit-time coverage write (a cut-short profile, measured); the agent itself, its PTY's session leader, cannot leave that group (`setpgid` refuses a leader), so on Unix its exit ignores SIGHUP from its first step (`\x03` or stdin's end): a hangup that lands while it exits (a killed wrapper, or portable-pty's SIGHUP-first kill) no longer cuts its own exit-time coverage write, and one that lands before the exit begins still ends it (measured).»

- leaf: .claude/rules/verification-harness.md:41   class: omission
  leaf-text: «as the live process their snapshot records, reaped by each file's `Held` `Drop` guard)»
  source: .andromeda/test-plan-detail/test-data-fixtures.md:69 «as the live process whose pid the impostor's snapshot records, killed and reaped by each file's `Held` `Drop` guard.»
  replace-with: «as the live process whose pid their snapshot records, killed and reaped by each file's `Held` `Drop` guard)»

- leaf: .claude/rules/testing.md:18   class: omission
  leaf-text: «`empty_bin`, `claude_bin` and `claude_in` in `fake.rs`, the one place a test plants the fake agent as `claude`,»
  source: .andromeda/test-plan-detail/test-data-fixtures.md:33 «A test that plants more than one fake-agent copy plants every copy (each recorded set's bin) one after another before any thread starts a child: on the tmpfs `target/e2e-home` the hard link fails»
  replace-with: «`empty_bin`, `claude_bin` and `claude_in` in `fake.rs`, the one place a test plants the fake agent as `claude` (a test that plants more than one fake-agent copy plants every copy, each recorded set's bin, one after another before any thread starts a child: on the tmpfs `target/e2e-home` the hard link fails, so each copy is written, and a child one thread forks inherits another thread's open write handle, ETXTBSY),»
