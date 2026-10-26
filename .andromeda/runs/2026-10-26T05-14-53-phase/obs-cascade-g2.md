# obs · cascade read · group 2 of 2

## Carried
- (.andromeda/obs-plan-detail/ci-integration.md:22) The lint stage's clippy `-D warnings`, the Tokio-free `cargo check` of the sync crates and `cargo deny check` with the veil `toggle` and banned-obs-crate bans must all exit 0 on the bumped tokio pin and lock, with no ban or ignore changed to reach green.
  > | Lint / typecheck | clippy `-D warnings` incl. `disallowed-macros` (raw `tracing::{info,warn,error,debug,trace}!` outside `viola_core::obs`) and the `print_stdout` / `print_stderr` / `dbg_macro` bans; the fail-closed raw-`event!` grep and both-ways ban probes in `scripts/lint-probes.sh`, Linux leg (mechanism in §3 logger-stack-install); bare-`#[instrument]` gate G1 (below); `cargo check` of sync crates without Tokio; `cargo deny check` (ubuntu) incl. veil `toggle` ban and bans on `opentelemetry-otlp` / `opentelemetry-stdout` / `sentry` / `tracing-appender` / tracing-subscriber `env-filter` feature | CI annotations + job exit code |
  (.andromeda/obs-plan.md:594)
  > beside the scheduled `nightly.yml`, which runs the weekly `cargo deny check advisories`
- (.andromeda/obs-plan-detail/ci-integration.md:23) The workspace tests the chunk re-runs must keep the in-memory writer tests passing against `schemas/diag-line.v1.json`, the witness that no emitted line changes under the bump.
  > | Unit tests | In-memory `Vec<u8>` writer tests parse emitted lines with serde_json against `schemas/diag-line.v1.json` (tests-owned bodies) | nextest JUnit |
- (.andromeda/obs-plan-detail/ci-integration.md:25) The CI row read at the wrap on the chunk's push must read the E2E stage's G2 (zero panics) and G4 (schema conformance) gates green, the run-time witness of the bumped tokio under `viola mcp`.
  > | E2E tests | `agent-run logs` / `status`; zero-`event:"panic"` gate G2 and schema gate G4 (below) | uploaded artifacts + gate exit codes |
  (.andromeda/obs-plan.md:606)
  > - **G2, zero panics:** `scripts/g2-zero-panics.sh`, fail-closed, `--probe` first, in the `test` and `perf` jobs, over the home-level role files only; one exemption, the `fake-agent` seam's forced panic at exactly `src/cmd/hook/seam.rs:<digits>`, compared as a whole string.
- (.andromeda/obs-plan-detail/ci-integration.md:40) G4 is a step of the CI row the chunk re-reads, and must exit 0 over every role-file and detail line of that run, unchanged in scope and schema.
  > - **G4, schema conformance.** §10 counts schema-conformance failures as a build failure. G1–G3 do not measure them, and the Unit tests row checks only in-memory lines, so G4 checks real process output.
  >   - Scope: every line in the G2 file scope is checked against `schemas/diag-line.v1.json`, and every `instances/*/diagnostics/detail-*.ndjson` line against `schemas/diag-detail.v1.json` (§8).
  >   - It runs as its own step, `id: schema-conformance`, `if: always()` after the last test step.
  (.andromeda/obs-plan.md:608)
  > - **G4, schema conformance:** every G2-scope line against `schemas/diag-line.v1.json` and every detail line against `schemas/diag-detail.v1.json`, its own step `if: always()`, printing file, line and keyword, never the line's content.
- (.andromeda/obs-plan-detail/obs-anti-patterns.md:8) Any source edit the bump forces (an `AsyncFd` deprecation in Tokio-using code) must keep the clippy `print_stdout` / `print_stderr` / `dbg_macro` deny lints green and add no `#[allow]`, as the clippy gate the chunk re-runs enforces.
  > - NEVER use `print!` / `println!` / `eprint!` / `eprintln!` / `dbg!` in the `hook`, `run` or `mcp` code paths of the product crates (`viola-harness` and the fake agent are exempt, §3 obs-ci-gate-wire). They bypass the file sink and break the Hook contract, the `run` terminal rule and MCP framing. Enforced by `[workspace.lints.clippy]` `print_stdout`, `print_stderr`, `dbg_macro` = `deny` under the §9 clippy step.
  (.andromeda/obs-plan.md:717)
  > - NEVER use `print!` / `println!` / `eprint!` / `eprintln!` / `dbg!` in the `hook`, `run` or `mcp` code paths of the product crates: they bypass the file sink and break the Hook contract, the `run` terminal rule and MCP framing; enforced by the clippy `print_stdout`, `print_stderr` and `dbg_macro` lints at `deny`

## Bindings
- **ci-integration (Lint / typecheck `cargo deny check`) ↔ §10 build failure conditions**: the `cargo deny` bans the lint row runs are the build failure condition the bump must keep passing, (.andromeda/obs-plan-detail/ci-integration.md:22) ↔ (.andromeda/obs-plan.md:669)
- **ci-integration (clippy step) ↔ obs-anti-patterns (print bans)**: the clippy `-D warnings` step of the lint row is the enforcement the print ban names, so a forced source edit is judged by the one re-run, (.andromeda/obs-plan-detail/ci-integration.md:22) ↔ (.andromeda/obs-plan-detail/obs-anti-patterns.md:8)
- **ci-integration (E2E G2 / G4) ↔ §10 error budget**: the gate exit codes read on the chunk's CI row are the measurement of zero panics and zero schema-conformance failures, (.andromeda/obs-plan-detail/ci-integration.md:25) ↔ (.andromeda/obs-plan.md:655)
