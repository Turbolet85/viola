group: test-plan-1
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/observability.md · 46 · «- 2026-09-24: `obs_event!` expands to `::tracing::event!`, s»
read: .claude/rules/testing.md · 104 · «- 2026-10-20: A test that ends a served duplex session (`tok»

- leaf: .claude/docs/tests-summary.md:15   class: omission
  leaf-text: «(process-per-test, JUnit, `retries = 0`) + `cargo test --doc`»
  source: .andromeda/test-plan-detail/test-strategy.md:11 «- **Runner:** nextest is the runner; a plain `cargo test` run (`cargo test --bin viola`) is not supported»
  replace-with: «(process-per-test, JUnit, `retries = 0`) + `cargo test --doc`; nextest is the runner, and a plain `cargo test` run (`cargo test --bin viola`) is not supported»

- leaf: .claude/rules/testing.md:17   class: omission
  leaf-text: «mockall 0.15.0 only on seam traits the product defines (`Pty`, liveness probe, `Clock`).»
  source: .andromeda/test-plan-detail/unit-test-strategy.md:63 «Scoped log captures: every scoped capture in the root bin's unit modules goes through `crate::test_support::with_scoped_default` (`src/main.rs`)»
  replace-with: «mockall 0.15.0 only on seam traits the product defines (`Pty`, liveness probe, `Clock`); every scoped log capture in the root bin's unit modules goes through `crate::test_support::with_scoped_default` (`src/main.rs`), never a bare `tracing::subscriber::with_default`.»
