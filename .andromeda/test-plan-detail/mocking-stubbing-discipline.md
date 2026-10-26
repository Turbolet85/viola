# test-plan · Mocking & Stubbing Discipline
Read whole. The master's broad view is .andromeda/test-plan.md; this file holds its Mocking & Stubbing Discipline detail.

## 8. Mocking & Stubbing Discipline

<!-- detail: values · mocking -->

- Time: sync crates use mock_instant 0.6.1 (global mode) or the injected `viola_core::Clock` for heartbeat 1 s / 5 s and the send confirmation window. `budget_override_until` is no clock-seam case: the budget gate's decision takes `chrono::DateTime<Utc>` as an argument, and its expiry is a pure-function unit case (§4, chunk 2026-10-10-budget-governor). Async crates (`viola-mcp`) use `#[tokio::test(start_paused = true)]` from tokio 1.53.1 `test-util` for any timer the MCP adapter itself owns. The `wait` `timeout_ms` deadline runs in the wrapper process, and only the injected `viola_core::Clock` in the §4 root-bin case drives it. A paused tokio clock in a test never reaches a spawned wrapper (§11 Universal). `chrono::Utc::now()` is read only through the clock seam, with one exception as landed: the budget gate's three evaluation points read it directly and hand the instant to the pure decision (as read in `src/run/send.rs` and `src/cmd/run.rs` at the chunk 2026-10-10-budget-governor wrap; that read named the gate's call sites only and was no census of `Utc::now()`).

<!-- /detail -->

<!-- detail: values · mocking -->

| Concern | Library | Pattern |
|---------|---------|---------|
| HTTP mocking | N/A (no outbound HTTP) | none |
| Time mocking | mock_instant 0.6.1 (in no manifest yet, as measured at the chunk 2026-10-10-budget-governor wrap); tokio 1.53.1 `test-util` (in no manifest yet, as landed at chunk 2026-10-17-mcp-server-for-drivers) | Inject the `Clock` dependency through constructors and call `MockClock::advance(Duration::from_secs(6))`; or hand a pure decision its instant as an argument, as the budget gate's `decide` takes `chrono::DateTime<Utc>` |
| Process mocking | mockall 0.15.0 on the `viola-pty` `Pty` trait and the liveness / `Clock` traits; native stub binaries (`viola-fake-agent`) | `#[cfg_attr(test, mockall::automock)]` on seam traits only; external processes are real stub executables, resolved through `CARGO_BIN_EXE_viola-fake-agent` in root `tests/` only. `viola-e2e` tests and the harness use the assert_cmd target-dir lookup (§3 boot step 5) |
| Fake servers | interprocess 2.4.4 server API (squatter listener); `viola-fake-agent` as the fake CLI; windows-sys 0.61.2 `CreateNamedPipeW` (Windows only: test-owned control pipes) | The squatter binds the computed endpoint before `run`; in the listener-admission negatives the test-owned control pipes (accept-remote, reject-remote, the default descriptor) stand beside the real `viola_channel::Server::bind` listener, and `ImpersonateAnonymousToken` stands in for a foreign principal; no HTTP fake servers |

<!-- /detail -->
