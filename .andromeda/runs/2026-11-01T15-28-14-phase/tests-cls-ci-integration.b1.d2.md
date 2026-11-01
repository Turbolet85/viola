# Classified — tests · ci-integration.b1

## Answers
ci-integration#1.1 · yes · the chunk's new Rust code must pass fmt, clippy -D warnings and orphans-check, with its new functions reached
ci-integration#1.2 · no · the chunk adds no dependency, tokio edge or workflow, and its tokio-side euid check stays behind the existing tokio feature
ci-integration#1.3 · yes · the chunk's unit-layer cases (injected-euid peer check, viola-state strict modes, instance and home-tree checks) run in this nextest unit stage
ci-integration#1.4 · yes · the integration cases (cross_exit_causes rows, cli_list board cases, contract_error_bodies, handlers.rs test servers) run in this stage
ci-integration#1.5 · yes · the macOS exit-cause and server-verification cases must run on the macOS runner of the three-OS matrix, and the Unix harness cleanup measurement runs here
ci-integration#1.6 · yes · the one instrumented per-OS run carries the chunk's new unit, integration and E2E cases, and its coverage gate covers the new socket and euid code
ci-integration#1.7 · no · the task's Not needed names run --mutants and windows-mutants.yml, with no chunk gate
ci-integration#1.8 · yes · the hook's hyperfine max rows and the CONNECT_DEADLINE spine bound over the added checks are gated here
ci-integration#1.9 · yes · the new code must compile and its unit cases must pass on the 1.96 toolchain floor
ci-integration#1.10 · no · the task's Not needed names cargo-fuzz targets, and the chunk adds no parser surface
ci-integration#1.11 · no · the chunk adds no test-only feature or binary and no exit code, so the release artifact check has nothing new to judge
ci-integration#1.12 · yes · the quality gate reads the junit results and the hook's hyperfine max rows that the chunk's time bound rests on
