# Classified — test-plan · e2e-test-strategy.b1

## Answers
e2e-test-strategy#1.1 · no · assert_cmd 2.2.2, predicates 3.1.4 and trycmd 1.2.1 still hold on the changed Cargo.lock, whose 4-line delta is the tokio entry alone
e2e-test-strategy#1.2 · no · assert_cmd 2.2.2 still holds on the changed Cargo.lock, whose delta is the tokio entry alone, and no hook surface changed
e2e-test-strategy#1.3 · no · portable-pty =0.8.1 still holds on the changed Cargo.lock, whose delta is the tokio entry alone, and no run passthrough surface changed
e2e-test-strategy#1.4 · no · interprocess 2.4.4 and jsonschema 0.57.0 still hold on the changed Cargo.lock, and the sync client this row names holds no tokio
e2e-test-strategy#1.5 · no · rmcp 3.4.1 and its client-only feature set still hold on the changed Cargo files (rmcp-features re-read unchanged), and no session fact reaches the TokioChildProcess rule
e2e-test-strategy#1.6 · no · cargo-nextest 0.9.146 still holds (nextest is named unmoved among the dev tools), and the row only cites §4
e2e-test-strategy#2.1 · no · Path 1 is untouched (no product behaviour changed), and its run-start steps name no version or moved surface
e2e-test-strategy#2.2 · no · Path 1's verification signal names no version or moved surface, and no product behaviour changed
e2e-test-strategy#2.3 · no · no session fact moves the as-landed Path 1 test or its 4.69 s wall reading
e2e-test-strategy#2.4 · no · the Path 1 cleanup names no version or moved surface, and its drop guards are unchanged
e2e-test-strategy#3.1 · no · Path 2's surfaces and as-landed tests name no version, and no product or test surface changed
e2e-test-strategy#3.2 · no · the Path 2 steps name no version, and the send surface the session did not change
e2e-test-strategy#3.3 · no · the recorded claude 2.1.287 readings and the confirm rules are unmoved, since the CLI build and send behaviour did not change
e2e-test-strategy#3.4 · no · the Path 2 cleanup is the harness cleanup command, which the session did not change
e2e-test-strategy#4.1 · no · Path 3's surfaces and the claude 2.1.296 reading name no moved version or surface
e2e-test-strategy#4.2 · no · the Path 3 steps name no version, and the wait and last surface the session did not change
e2e-test-strategy#4.3 · no · the wake, exit and instance-unreachable rules are product behaviour the session did not change
e2e-test-strategy#4.4 · no · sysinfo's entry is outside the tokio-only lock delta, and the cleanup names no version
e2e-test-strategy#5.1 · no · Path 4's surfaces and as-landed dialog tests name no version, and no dialog surface changed
e2e-test-strategy#5.2 · no · the Path 4 steps name no version, and the dialog surface the session did not change
e2e-test-strategy#5.3 · no · the per-cli-version insta snapshots and the dialog verification rules are product output the session did not change
e2e-test-strategy#5.4 · no · the Path 4 cleanup is the harness cleanup command, which the session did not change
e2e-test-strategy#6.1 · no · Path 5's surfaces and as-landed wheel tests name no version, and no wheel surface changed
e2e-test-strategy#6.2 · no · the Path 5 steps name no version, and the wheel and release surface the session did not change
e2e-test-strategy#6.3 · no · the wheel, refusal and release rules are product behaviour the session did not change
