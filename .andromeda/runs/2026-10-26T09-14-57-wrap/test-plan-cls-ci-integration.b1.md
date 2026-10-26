# Classified — test-plan · ci-integration.b1

## Answers
ci-integration#1.1 · yes · add-to · the macOS-target check-only clippy joins the Windows-target one beside it as a host recipe, not a gate (F13)
ci-integration#1.2 · no · no .github file changed and no dependency pin moved, so viola-channel's normal graph stays tokio-free on the changed channel files and the tokio ban still holds
ci-integration#1.3 · no · no harness script or command changed (the §3 Test Harness Contract is untouched), so the Unit stage's command and nextest parallelism stand
ci-integration#1.4 · no · no harness script or command changed (the §3 Test Harness Contract is untouched), so the Integration stage's command, parallelism and cache stand
ci-integration#1.5 · no · no .github file changed and §3 untouched, so run --e2e still runs on all three OSes and the macOS compile-out of the new negatives does not move that matrix
ci-integration#1.6 · no · no .github file changed and neither the conpty_sideload binary nor the ConPTY verification step is reached, so the per-OS coverage run and its gate steps stand
ci-integration#1.7 · no · no .github file changed, so the ten-job windows-mutants matrix and its five root items stand
ci-integration#1.8 · no · no .github file changed and the perf rows and gate are unchanged, so the perf job's steps and gate stand (the hook rows now timing the server check is not a claim this job makes)
ci-integration#1.9 · no · no .github file changed and the toolchain pin and the 1.96 floor are unchanged, so the MSRV job's steps stand
ci-integration#1.10 · no · no fuzz/ file is reached and no .github file changed, so the fuzz-replay job and its cargo-fuzz pin stand
ci-integration#1.11 · no · no .github file changed and no release script is reached, so the release job and its probes stand
ci-integration#1.12 · no · the coverage thresholds, the mutation gate and the perf gate are unchanged, so the gate's inputs and breach rule stand
