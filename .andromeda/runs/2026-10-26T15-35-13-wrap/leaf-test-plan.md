group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 105 · «- 2026-10-26: An OS-gated test module in product code stacks»
read: .claude/rules/verification-harness.md · 72 · «- 2026-10-20: `target/debug/viola` is a hard link cargo re-p»

- leaf: .claude/docs/tests-summary.md:32   class: stale
  leaf-text: «and the explicit `viola link` / `viola unlink` steps are not built (held on "Explicit session links")»
  source: .andromeda/test-plan.md:570 «- **As landed: built.** The driven wrapper serves `link` / `unlink`, and the scenario runs as `tests/cli_links.rs` `e3_two_links_and_a_send_leave_one_link_record` from `stamped_home`»
  replace-with: «and the explicit `viola link` / `viola unlink` steps are built: E3 itself is `tests/cli_links.rs` `e3_two_links_and_a_send_leave_one_link_record` from `stamped_home` (the driver a self-reported name, not booted, so only `builder` boots), and `links_an_unlink_survives_a_wrapper_restart` holds an `unlink` across a wrapper restart»

- leaf: .claude/rules/testing.md:28   class: omission
  leaf-text: «or an instant handed to a pure decision as an argument (the budget gate's `decide`); never use an in-process clock»
  source: .andromeda/test-plan.md:705 «except at the budget gate's three evaluation points and at `LinkSet::line`, which stamps a link line's `since` (root cases assert it by its shape only, `is_clock_time` in `tests/cli_links.rs`)»
  replace-with: «or an instant handed to a pure decision as an argument (the budget gate's `decide`); a link line's `since`, which `LinkSet::line` stamps from the wall clock outside the seam, is asserted by its shape only (`is_clock_time` in `tests/cli_links.rs`); never use an in-process clock»
