# Classified — tests · ci-integration.b1

## Answers
ci-integration#1.1 · yes · the chunk's new readers and validators in the sync crates must pass fmt, clippy -D warnings with fake-agent, the no-feature cargo check and orphans-check
ci-integration#1.2 · yes · the tokio ban and channel-tokio-free keep the sync crates and viola-channel's normal graph tokio-free under the chunk's bounded readers, and the fuzz deny check covers a new fuzz target
ci-integration#1.3 · yes · the chunk's unit witnesses (frame codec, paste validation, ViolaName, config bounds, proptest cases) run in the nextest unit suite
ci-integration#1.4 · yes · the chunk's integration cases (wrapper-side paste refusal, chaos_feed_panic, channel_links, fake-agent agents_mode reads) run in the integration suite
ci-integration#1.5 · yes · the Path 1 test path1_run_start_one_home_in_order and the other viola-e2e path_ cases the chunk adds run in the E2E suite on all three OSes
ci-integration#1.6 · yes · the per-OS test job is the one instrumented nextest run where the Path 1 CI reads are taken and the new cases run, so the WATCH tally reads it
ci-integration#1.7 · no · mutation testing runs at the epoch boundary and on demand, never as a per-chunk gate (task Not needed)
ci-integration#1.8 · yes · the perf job times each hook, the path whose stdin the chunk bounds, under the §10 perf gate and G2
ci-integration#1.9 · yes · the chunk's new code and its unit witnesses must compile and pass on the 1.96 MSRV floor in the msrv job
ci-integration#1.10 · yes · the fuzz-replay corpus replay and the nightly per-target fuzz run cover the cargo-fuzz target the chunk adds to fuzz/ as its parser lands
ci-integration#1.11 · no · the release check judges only the release build's binaries and features, which the chunk adds none of, and its test seams stay under fake-agent
ci-integration#1.12 · yes · the test job's G2 zero-panics scan, which the chaos test's forced vt100 panic must boot outside of, and the gate reading the chunk's junit and llvm-cov output
