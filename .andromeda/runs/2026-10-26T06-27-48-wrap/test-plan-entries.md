## 2026-10-26-tokio-rustsec-2026-0386-lock-bump — viola-e2e's tokio pin in the bootstrap phases
**Section:** §3 Test Harness Contract (the key file Bootstrap phases (derive for route / setup-project): the test-runner-install Dev-deps, the `in viola-e2e` bullet)
**Change:**
- The `viola-e2e` bullet reads tokio `=1.53.3` with default features off and `rt` + `process` (no `test-util`) (was `=1.53.1`).
**Why:** the report's Dependencies fact: the `[workspace.dependencies]` exact pin and the lock moved tokio `=1.53.1` → `=1.53.3`, a version RUSTSEC-2026-0386 lists as patched, `default-features = false` kept; its Crates / modules fact: tokio's per-crate features unmoved (`viola-e2e` `rt` + `process`).
**Kept:** every other pin on the bullet (rmcp `>=3.4.1, <3.5` with `client` only, syn `=2.0.119`, proc-macro2 `=1.0.107`, libc `=0.2.189`) and the bullet's no-`test-util` clause.
**Ref:** .andromeda/runs/2026-10-26T06-27-48-wrap/

## 2026-10-26-tokio-rustsec-2026-0386-lock-bump — the tokio test-util version in the time-mocking discipline
**Section:** §8 Mocking & Stubbing Discipline (mocking-stubbing-discipline.md: the Time item, the Time mocking row)
**Change:**
- The Time item reads `#[tokio::test(start_paused = true)]` from tokio 1.53.3 `test-util` (was 1.53.1).
- The Time mocking row reads tokio 1.53.3 `test-util` (was 1.53.1).
**Why:** the report's Dependencies fact: the workspace pin and the lock moved tokio 1.53.1 → 1.53.3; Crates / modules and Schema / config changed nothing, so no manifest gained `test-util`.
**Kept:** the row's qualifier "in no manifest yet, as landed at chunk 2026-10-17-mcp-server-for-drivers", and the rest of the item and the row.
**Ref:** .andromeda/runs/2026-10-26T06-27-48-wrap/

## 2026-10-26-tokio-rustsec-2026-0386-lock-bump — the tokio pin in the time-sensitive fixtures row
**Section:** §7 Test Data & Fixtures (test-data-fixtures.md: the Time-sensitive data row)
**Change:**
- The row's parenthesis reads "(as landed at chunk 2026-10-17-mcp-server-for-drivers; tokio `=1.53.3` with default features off, …)" (was `=1.53.1`, joined by a colon); no history clause is written into the body.
**Why:** the report's Dependencies fact: the workspace exact pin and the lock moved tokio `=1.53.1` → `=1.53.3`; its Crates / modules fact: tokio's per-crate features unmoved.
**Kept:** the features clause (`viola-mcp` `rt`, `viola-channel` `io-util` and dev `rt`, `viola-e2e` `rt` + `process`), the no-manifest-names-`test-util` fact, and `viola-mcp` owning no timer (`wait_timeout(given)` = `min(given, 50 000)`, a unit case), so no `start_paused` test exists.
**Ref:** .andromeda/runs/2026-10-26T06-27-48-wrap/
