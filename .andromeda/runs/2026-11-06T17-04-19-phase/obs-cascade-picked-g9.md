@account n=1 entity=ci-integration#1.1 v=carried
why: the wrap's CI read takes the test job's scan-gated `diag-<os>` as its diagnosis input, and its detail-file upload is admissible only for synthetic homes, a fork over where the live real-`claude` sessions' homes sit
@account n=2 entity=ci-integration#1.2 v=not_in_scope standing=S4
why: the failure-only `harness-<os>` capture and its scan-gated upload is the upload convention every CI step keeps; the Setup red is `supply-chain`, not a test failure, and the chunk adds no step
@account n=3 entity=ci-integration#1.3 v=carried
why: v1-53's claim reads the `ubuntu-latest` nextest JUnit alone, the `junit-<os>` artifact this row defines
@account n=4 entity=ci-integration#1.4 v=carried
why: the `perf` job is one of CI's seven jobs re-read green at the wrap, and its gate reads these per-hook hyperfine files
@account n=5 entity=ci-integration#1.5 v=not_in_scope
why: it records only that v1 has no OTel exporter; the chunk adds, removes and re-asserts no exporter
@account n=6 entity=ci-integration#2.1 v=carried
why: the lint stage's clippy, lint-probe, G1, Tokio-free check and `cargo deny` gates must read green at the wrap over the moved `portable-pty` graph and the remnant edits
@account n=7 entity=ci-integration#2.3 v=not_in_scope standing=S1
why: every integration-test viola home lives under `target/e2e-home/` so G2, G4, the secret scan and the `diag-<os>` upload cover it
@account n=8 entity=ci-integration#2.4 v=carried
why: the E2E job's verdict at the wrap is read through G2 and G4 exit codes, and the live sessions' evidence through `agent-run logs` / `status`
@account n=9 entity=ci-integration#2.5 v=carried
why: the chunk re-reads the OS-gated census v1-53's claim reads, and the Windows-remnant removal must leave no gate naming `windows`
@account n=10 entity=ci-integration#2.6 v=carried
why: the chunk edits the `windows` test-host key in `host.rs`, the host exclusion this row's mutation verdict rests on
@account n=11 entity=ci-integration#3.1 v=carried
why: G1 is one of the gates the chunk re-asserts green at the wrap
@account n=12 entity=ci-integration#3.2 v=carried
why: G2 is the zero-panic gate the chunk re-asserts green at the wrap in the `test` and `perf` jobs
@account n=13 entity=ci-integration#3.3 v=carried
why: G4 is re-asserted green at the wrap, and G3 scans `Cargo.toml`, which the chunk edits for the `portable-pty` pin
@account n=14 entity=ci-integration#4.1 v=carried
why: the kept homes and the perf job's step order are what make the `test` and `perf` jobs' green re-read at the wrap non-vacuous, and the chunk edits the harness files that honour `AGENT_RUN_KEEP_HOMES`
@account n=15 entity=ci-integration#5.1 v=carried
why: it states the same kept-homes and `if: always()` scan behaviour the wrap's green secret scan reads
@account n=16 entity=slo-invariants-telemetry-budgets#1.1 v=carried
why: the zero-panic invariant is the error budget the done-check must clear, and the `portable-pty` bump moves under the `run` pump and handle-wait threads that must keep it
@account n=17 entity=slo-invariants-telemetry-budgets#2.1 v=carried
why: the `session-end` budget is a gated value of the `perf` job re-read green at the wrap
@account n=18 entity=slo-invariants-telemetry-budgets#2.2 v=carried
why: the spine-hook and `pre-tool-use` budgets are gated values of the `perf` job re-read green at the wrap
@account n=19 entity=slo-invariants-telemetry-budgets#2.4 v=carried
why: the hook obs-overhead gate is part of the `perf` job's green, holding only under its stated home and `VIOLA_NAME` conditions
@account n=20 entity=obs-anti-patterns#1.1 v=not_in_scope standing=S6
why: no `print!` / `println!` / `eprint!` / `eprintln!` / `dbg!` in the `hook`, `run` or `mcp` code paths of product crates
