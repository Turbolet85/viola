# Classified — test-plan · quality-gates-coverage-targets.b1

## Answers
quality-gates-coverage-targets#1.1 · yes · stale · tests/chaos_wait_vanish.rs now uses support::start::kill_wrapper with its own kill gone, so the kill-shaped test the census measured reads a different kill
quality-gates-coverage-targets#2.0 · yes · stale · the ci profile's kill line in .config/nextest.toml:24 gained binary(cross_exit_causes), so the 120 s kill now reaches the new root binary, while the 120 s value and retries = 0 hold
quality-gates-coverage-targets#3.1 · no · the perf session times the hook path, which the change does not reach: hook.rs is untouched, hook-decision is unmoved and hooks stay exit 0 with empty stderr
