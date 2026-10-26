# Apply — test-plan · part 2 of 2

## Verdicts

- fanout-test-plan--cascade-g1.md 14 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:235#1
- fanout-test-plan--cascade-g1.md 16 · already stands · .andromeda/test-plan-detail/e2e-test-strategy.md:242–242 · «As landed it holds the fail-open matrices `hook_fails_open_silently_within_the_spine_bound` and `hook_dialog_tier_fails_open_silently`»
- fanout-test-plan--cascade-g1.md 17 · already stands · .andromeda/test-plan-detail/e2e-test-strategy.md:246–246 · «are `case::impostor` in `hook_fails_open_silently_within_the_spine_bound` and `question_impostor` and `permission_impostor` in `hook_dialog_tier_fails_open_silently`»
- fanout-test-plan--cascade-g1.md 18 · already stands · .andromeda/test-plan-detail/e2e-test-strategy.md:244–244 · «missing `VIOLA_NAME` and, on Linux and Windows, a server that is not the recorded wrapper (`server-verify-failed`)»
- fanout-test-plan--cascade-g1.md 19 · already stands · .andromeda/test-plan-detail/e2e-test-strategy.md:248–248 · «Not a refusal case: a hook reaching an exiting wrapper's own Windows pipe (ci#36532038635) passes the server check by design»
- fanout-test-plan--cascade-g1.md 21 · already stands · .andromeda/test-plan-detail/e2e-test-strategy.md:266–266 · «As landed, `cross_exit_causes.rs` does not exist and no rstest table joins the causes»
- fanout-test-plan--cascade-g1.md 22 · already stands · .andromeda/test-plan-detail/e2e-test-strategy.md:270–270 · «As landed, its Linux and Windows case is the six-verb server-identity negative and its control under Security control negatives (`tests/security_negatives_server_identity.rs`), not a `cross_exit_causes.rs` row.»
- fanout-test-plan--cascade-g1.md 23 · already stands · .andromeda/test-plan-detail/e2e-test-strategy.md:269–269 · «are not built, one CARRY on the route entry "Unix endpoint and home hardening"»
- fanout-test-plan--cascade-g1.md 24 · already stands · .andromeda/test-plan-detail/e2e-test-strategy.md:271–271 · «Its hint is its own, cause `server-verify` in `src/human/hints.rs`»
- fanout-test-plan--cascade-g1.md 26 · already stands · .andromeda/test-plan-detail/e2e-test-strategy.md:305–305 · «**Server identity** (`#![cfg(not(target_os = "macos"))]`, IPC client-side server verification)»
- fanout-test-plan--cascade-g1.md 27 · already stands · .andromeda/test-plan-detail/e2e-test-strategy.md:306–306 · «Its control `security_negative_cli_verb_reaches_the_recorded_wrapper` runs each of the six verbs, in both modes, against the recorded wrapper itself, which receives one frame each.»
- fanout-test-plan--cascade-g2.md 1 · already stands · .andromeda/test-plan-detail/test-data-fixtures.md:27–27 · «The server-identity impostor cases (Linux and Windows: `served` in `tests/security_negatives_server_identity.rs`, `Impostor::serve` in `tests/hook_fail_open.rs`) write an `InstanceSnapshot` themselves»
- fanout-test-plan--cascade-g2.md 3 · already stands · .andromeda/test-plan-detail/test-data-fixtures.md:69–69 · «re-exec mode `--hold-stdout-internal` (it sleeps 10 s and writes nothing) is also spawned directly by the server-identity impostor cases»
- cascade-compare-test-plan.md 2 · already stands · .andromeda/test-plan-detail/test-data-fixtures.md:33–33 · «A test that plants more than one fake-agent copy plants every copy (each recorded set's bin) one after another before any thread starts a child»
- fanout-test-plan--cascade-g2.md 4 · already stands · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:18–18 · «a held live process whose pid an impostor's snapshot records is a real process, never a double»
- cascade-compare-test-plan.md 1 · already stands · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:19–19 · «in the server-verification negatives (Linux and Windows) an impostor»
- fanout-test-plan--cascade-g2.md 5 · already stands · .andromeda/test-plan-detail/ci-integration.md:11–11 · «Beside it, a check-only macOS-target clippy is a recipe on the Linux dev host»
- fanout-test-plan--cascade-g2.md 7 · already stands · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:26–26 · «Every timed hook also makes its server check (the peer pid and its OS start time against the one snapshot read) before its first frame»
- cascade-compare-test-plan.md 3 · already stands · .andromeda/test-plan-detail/test-scope-summary.md:12–12 · «**As landed (server verification):** the pid/start-time server verification is asserted on Linux and Windows»

## Edits

- line: .andromeda/test-plan-detail/e2e-test-strategy.md:235
  old: «`src/cmd/hook/statusline.rs`) stay on `read_snapshot` by the founder's own ruling, and their move»
  new: «`src/cmd/hook/statusline.rs`) stay on `read_snapshot`, and their move»

## Patterns

(none)
