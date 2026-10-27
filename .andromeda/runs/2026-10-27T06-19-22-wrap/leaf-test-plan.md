group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 105 · «- 2026-10-26: An OS-gated test module in product code stacks»
read: .claude/rules/verification-harness.md · 72 · «- 2026-10-20: `target/debug/viola` is a hard link cargo re-p»

- leaf: .claude/rules/testing.md:38   class: omission
  leaf-text: «verdicts come from hook events, `events.ndjson`, channel payloads, the fake-agent receipt and exit codes.»
  source: .andromeda/test-plan.md:473 «and for the forced vt100 panic chaos case a byte search of the drained master reader's stream (the child's own bytes on Unix, no viola literal on every OS), never a parse of the rendered screen»
  replace-with: «verdicts come from hook events, `events.ndjson`, channel payloads, the fake-agent receipt and exit codes, and in the forced vt100 panic chaos case (`tests/chaos_feed_panic.rs`) a byte search of the outer terminal's drained stream (the child's own bytes on Unix, no viola literal on every OS), never a parse of the rendered screen.»
