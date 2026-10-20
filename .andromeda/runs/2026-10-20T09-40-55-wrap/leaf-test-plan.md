group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 101 · «- 2026-10-20: A probe word the right implementation also wri»
read: .claude/rules/verification-harness.md · 71 · «- 2026-10-14: A side effect added to a fake-agent turn (a sp»

- leaf: .claude/docs/tests-summary.md:32   class: retired
  leaf-text: «E4 (SSE `Last-Event-ID` resume) deferred with the frontend to 0.2.0»
  source: .andromeda/test-plan.md:826 «#### Scenario: E4 — none in 0.1.0»
  replace-with: «E4 none in 0.1.0 (N/A: no frontend in 0.1.0; the frontend is deferred to 0.2.0, its technology not chosen)»

- leaf: .claude/docs/tests-summary.md:20   class: omission
  leaf-text: «and on the Linux dev host it is an operator-set link to an owner-only tmpfs directory under `/tmp`: the path is unchanged»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:6 «and refused when its filesystem reads less than the 9 GiB headroom floor free (`statvfs`; the message and the floor: §3 Test data bootstrap)»
  replace-with: «a linked base whose filesystem reads less than the 9 GiB headroom floor free (`statvfs`) is refused, and on the Linux dev host it is an operator-set link to an owner-only tmpfs directory under `/tmp`: the path is unchanged»

- leaf: .claude/rules/testing.md:33   class: omission
  leaf-text: «prepared and checked by `tests/support/home.rs` `prepare_home_base`; no CI runner has the link»
  source: .andromeda/registries/contracts/test-plan/test-data-bootstrap.md:13 «A linked base whose filesystem reads less than `HEADROOM_FLOOR_MIB` (9 GiB) free by `statvfs` (`f_bavail × f_frsize`) is refused too»
  replace-with: «prepared and checked by `tests/support/home.rs` `prepare_home_base` (a linked base whose filesystem reads less than `HEADROOM_FLOOR_MIB`, 9 GiB, free by `statvfs` is refused with a fixed message holding the two counts and no path; a plain base reads no headroom); no CI runner has the link»

- leaf: .claude/rules/verification-harness.md:28   class: omission
  leaf-text: «made again 0700 when it is gone, else `build-failed`)»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:6 «and refused when its filesystem reads less than the 9 GiB headroom floor free (`statvfs`; the message and the floor: §3 Test data bootstrap)»
  replace-with: «made again 0700 when it is gone, and refused when its filesystem reads less than the 9 GiB headroom floor free (`statvfs`), else `build-failed`)»

- leaf: .claude/rules/verification-harness.md:41   class: stale
  leaf-text: «first waiting for a hook still running and starting no other: it is its PTY's session leader, so its exit would hang up an in-flight hook mid-exit (a truncated coverage profile, measured).»
  source: .andromeda/test-plan.md:963 «and on Unix every command it spawns (each hook, and the status line command) runs in a process group of its own (`process_group(0)`, as the `--exit-no-eof` holder does)»
  replace-with: «first waiting for a hook still running and starting no other; on Unix every command it spawns (each hook, and the status line command) runs in a process group of its own (`process_group(0)`, as the `--exit-no-eof` holder does), because a hangup of its PTY's foreground group, at its exit as session leader or when a killed wrapper closes the PTY, cuts a `viola hook` short in its exit-time coverage write (a cut-short profile, measured).»
