---
paths:
  - "tests/**"
  - "crates/*/tests/**"
  - "crates/viola-e2e/tests/**"
  - "e2e-web/tests/**"
  - "fixtures/**"
  - "fuzz/**"
  - "**/proptest-regressions/**"
  - ".config/nextest.toml"
---

# Testing Rules

Path-scoped rules for test authoring (what tests assert, fixtures, coverage). Loaded when working on the files above. Authoritative source: `.andromeda/test-plan.md` (tier Comprehensive). Harness driver / invocation rules live in `verification-harness.md`.

## Framework
- **Unit:** libtest `#[cfg(test)] mod tests` inline in every crate + root `src/`, run through cargo-nextest 0.9.146; rstest 0.27 `#[case]` tables, proptest 1.11.0 (`cases: 512`), insta 1.48.0 (check mode only), mockall 0.15.0 only on seam traits the product defines (`Pty`, liveness probe, `Clock`).
- **Integration:** crate `tests/<topic>.rs` + root `tests/{cli,hook,tui,channel,chaos,contract}_<topic>.rs` (sync, tokio-free) with assert_cmd 2.2.2, trycmd 1.2.1, axum-test 21.1.0, jsonschema 0.57.0.
- **E2E:** `crates/viola-e2e/tests/{path,mcp,http,sse,cross}_<topic>.rs` (rmcp client, reqwest, eventsource-client) + Playwright 1.63.0 specs `e2e-web/tests/<bay-layout-type>.spec.ts` (ubuntu only).
- **Doctests:** `cargo test --workspace --doc` (nextest cannot run them); examples that must not run use `no_run` / `text`, never `ignore`.

## Naming
- `<subject>_<condition>_<expected>`; critical paths `path<N>_<slug>`; rstest case labels readable (`#[case::esc_refused]`); Playwright `test('<layout type>: <expected state>')` with the scenario id in the title.
- Fixtures `fixtures/claude/<cli-version>/<hook-event>.<variant>.json`; fake scripts `fixtures/fake-scripts/<scenario>.json`.

## Determinism (zero-flake budget)
- NEVER `sleep` for synchronization — wait on an `events.ndjson` byte offset, `viola wait --after`, an SSE `id:`, a status field or an auto-waiting locator. nextest `retries = 0`, Playwright `retries: 0`; no `#[ignore]` / `test.skip` / `test.fixme` as a parking place.
- Time only through the injected `viola_core::Clock` / mock_instant (sync) or `start_paused` (Tokio); never use an in-process clock to control another process's time.
- NEVER `std::env::set_var` (unsafe in edition 2024) — `Command::env` per child; `.env_clear()` must re-add `LLVM_PROFILE_FILE`.
- Commit `proptest-regressions/`; chaos faults only from explicit process/file operations or mockall seams.

## Test data
- Every test owns a fresh home: a not-yet-existing `home` inside `tempdir_in("<workspace>/target/e2e-home")`, so viola creates it (Windows protected DACL) and CI's G2/G4/secret scan cover it.
- Never hand-write `ledger/stamps.json` (only `viola verify` against the fake agent), `snapshot.json` (only the wrapper) or `budget.json` (only `viola hook statusline`).
- Fixtures are synthetic probe text only, scrubbed of absolute paths and usernames; every input embeds the tests-owned canary string.

## What to assert
- Never parse the rendered child screen; verdicts come from hook events, `events.ndjson`, channel payloads, the fake-agent receipt, exit codes and DOM attributes.
- Oracles are literals in the test (the 14 S6 names, refusal order, exit codes) — never import the product's own list; compare kebab-case serde values and integer exits, never `Debug` strings.
- A multi-surface path asserts every surface it lists; a lower-layer case (parser, refusal order, threshold, clock) belongs in unit tests, not E2E.
- No test auto-approves a dialog; a decision body without a verify stamp and wheel `driver` fails the test.
- Web selectors: roles + `data-*` + exact text; never CSS classes (`.band`, `.rb`), inline `style` or xpath; never `bypassCSP` or `toHaveScreenshot`.

## Running tests
- **Everything:** `scripts/agent-run.sh run --all` · **one Rust test:** `scripts/agent-run.sh run --e2e --filter 'test(/path2_send_confirms/)'`
- **Unit / integration:** `run --unit` / `run --integration` · **browser (ubuntu):** `run --browser` · **one spec:** `npx --prefix e2e-web playwright test --grep "<title>"`
- **Coverage:** `run --coverage` (lines 85 / functions 95 / regions 80 per OS) · **mutants:** `run --mutants` (zero missed, zero timeout; verdict from `mutants.out/outcomes.json`, never the exit code alone)

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run._
