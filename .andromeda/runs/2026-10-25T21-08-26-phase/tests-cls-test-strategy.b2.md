# Classified — tests · test-strategy.b2

## Answers
test-strategy#3.17 · no · the mutation gate runs only at the epoch boundary or on demand, never per chunk (the task's Not-needed list), so the mutation row reaches no chunk code or test
test-strategy#3.18 · yes · the no-flakes discipline binds: case_4 must go green through its root cause with retries = 0, no #[ignore], no serialising and a signal-based log capture, never a sleep
