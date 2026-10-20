group: test-plan-2
read: .claude/rules/verification-harness.md · 71 · «- 2026-10-14: A side effect added to a fake-agent turn (a sp»

- leaf: .claude/rules/verification-harness.md:38   class: stale
  leaf-text: «the set's recorded `SessionStart.resume-1.json` when the set holds it (2.1.287), else»
  source: .andromeda/test-plan.md:936 «Run E's resume payload as `SessionStart.resume-<n>.json` (one under each of `2.1.287` and `2.1.302`)»
  replace-with: «the set's recorded `SessionStart.resume-1.json` when the set holds it (2.1.287, 2.1.302), else»

- leaf: .claude/rules/verification-harness.md:41   class: omission
  leaf-text: «`Screen.<phase>` of the set after the tag-like paste's turn alone, in place of `Screen.turn`),»
  source: .andromeda/test-plan.md:962 «the test-only screen switch `--long-turn-screen <phase>` (with `--framing` and `--turn-stop`: after the long paste's turn alone, after its Stop, the agent draws `Screen.<phase>.json` of its set in place of»
  replace-with: «`Screen.<phase>` of the set after the tag-like paste's turn alone, in place of `Screen.turn`), the test-only screen switch `--long-turn-screen <phase>` (with `--framing` and `--turn-stop`: `Screen.<phase>` of the set after the long paste's turn alone, after its Stop, in place of `Screen.turn`),»

- leaf: .claude/rules/verification-harness.md:42   class: omission
  leaf-text: «`Wrapper::stop` waits for the endpoint to be unconnectable, not only the exit)»
  source: .andromeda/test-plan.md:563 «**Readiness stall report:** when `tests/support/home.rs` `wait_ready` reaches its deadline, it appends the stalled wrapper's state to the watch report before the panic»
  replace-with: «`Wrapper::stop` waits for the endpoint to be unconnectable, not only the exit; `wait_ready` at its deadline appends the stalled wrapper's state — its last `run-<name>.ndjson` event, on macOS `/usr/bin/sample <pid> 1`'s stack lines, on Linux `/proc/<pid>/wchan` and the `stat` state field (`note_stall`) — to the watch report before the panic, signalling nothing and adding nothing to a passing boot)»
