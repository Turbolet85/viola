# Classified — tests · ci-integration.b1

## Answers
ci-integration#1.1 · no · the lint stage's fmt, clippy and orphans gates rule over all code alike, a re-check every change owes; the chunk adds no crate to the cargo-check list
ci-integration#1.2 · yes · the peer-credential code in viola_channel, including the TokioClient euid check, must keep the channel's normal graph tokio-free (channel-tokio-free) and add no dependency that fails deny
ci-integration#1.3 · yes · the unit-layer cases Needs names (injected-euid peer check, viola-state strict-modes unit layer) run here under nextest process-per-test
ci-integration#1.4 · yes · the integration binaries Needs names (cross_exit_causes, cli_list, contract_error_bodies, security negatives, tests/support/home) run here, in parallel
ci-integration#1.5 · yes · the E2E run on all three OSes carries the macOS exit-cause case, the Unix harness cleanup measurement and the v1-53 suite on Linux and macOS
ci-integration#1.6 · no · standing S8 · each OS job holds lines 85, functions 95 and regions 80 under the per-OS coverage run
ci-integration#1.7 · no · mutation runs at the epoch-boundary audit with no push job and no chunk gate; Not needed names run --mutants
ci-integration#1.8 · yes · the hook's time bound over the added checks is measured by the hyperfine max rows Needs names
ci-integration#1.9 · no · the 1.96 MSRV compile and unit run is a toolchain gate every change owes; the scope names no API that moves the floor
ci-integration#1.10 · no · the chunk adds no parser surface, and Not needed drops the cargo-fuzz targets and replay
ci-integration#1.11 · no · release-check judges the release build's features and binaries, which the chunk does not add or change
ci-integration#1.12 · yes · the perf gate reads the hyperfine JSON that carries the hook's max rows the scope names
