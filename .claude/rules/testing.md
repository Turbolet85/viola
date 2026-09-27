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
- **E2E:** `crates/viola-e2e/tests/{path,mcp,http,sse,cross}_<topic>.rs` (rmcp client, reqwest, eventsource-client) + Playwright 1.63.0 specs `e2e-web/tests/<bay-layout-type>.spec.ts` (the pipe stub `pipe-reachability.spec.ts`, titles `pipe:`, is the one exception) on all three CI OSes; render, contrast and a11y verdicts are judged on the ubuntu leg.
- **Doctests:** `cargo test --workspace --doc` (nextest cannot run them); examples that must not run use `no_run` / `text`, never `ignore`.

## Naming
- `<subject>_<condition>_<expected>`; critical paths `path<N>_<slug>`; rstest case labels readable (`#[case::esc_refused]`); Playwright `test('<layout type>: <expected state>')` with the scenario id in the title.
- Fixtures `fixtures/claude/<cli-version>/<Event>.<variant>.json` (the CLI's PascalCase hook event name, e.g. `PreToolUse.ask.json`; no mapping table); fake scripts `fixtures/fake-scripts/<scenario>.json` (schema `schemas/fake-script.v1.json`).

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
- Oracles are literals in the test (the 11 identity-floor names, refusal order, exit codes) — never import the product's own `IDENTITY_FLOOR`; compare kebab-case serde values and integer exits, never `Debug` strings.
- A multi-surface path asserts every surface it lists; a lower-layer case (parser, refusal order, threshold, clock) belongs in unit tests, not E2E.
- No test auto-approves a dialog; a decision body without a verify stamp and wheel `driver` fails the test.
- Web selectors: roles + `data-*` + exact text; never CSS classes (`.band`, `.rb`), inline `style` or xpath; never `bypassCSP` or `toHaveScreenshot`.

## Running tests
- **Everything:** `scripts/agent-run.sh run --all` · **one Rust test:** `scripts/agent-run.sh run --e2e --filter 'test(/path2_send_confirms/)'`
- **Unit / integration:** `run --unit` / `run --integration` · **browser (every OS, only when named — never under `--all`):** `run --browser` · **one spec:** `npx --no --prefix e2e-web playwright test --config e2e-web/playwright.config.ts --grep "<title>"` (from the repo root `--config` is required)
- **Coverage:** `run --coverage` (lines 85 / functions 95 / regions 80 per OS) · **mutants:** `run --mutants` (zero missed, zero timeout, unviable ≤ caught; verdict from a fresh `mutants.out/outcomes.json` — in the host mutation scratch `<repo parent>/viola-mutants-scratch` on a Windows host — never the exit code alone; `--file <path>` is the scoped inner loop, `verdict:"scoped"`, never a leg verdict; a diff with no `.rs` path passes as `verdict:"no-rust-delta"`; one whose `.rs` paths are all test targets — `tests/`, `benches/`, `examples/` — as `verdict:"test-only-rust-delta"`; a mixed diff is `counted`). In CI the verdict is the union of the ubuntu and windows legs (`run --mutants --leg` + `gate --mutants-legs`), each mutant judged only by the legs whose `#[cfg]`s compile its line, so a body gated to one OS is killed (or proven unviable) on the leg that compiles it and a miss there stays red; `pre-push` runs the same two legs and union locally before the push, so a Unix-only red surfaces on the Windows host · **fuzz:** `run --fuzz-replay` (Linux only; targets in the separate `fuzz/` workspace, synthetic corpus)

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run._
- 2026-09-24: Under the zero-missed mutation gate every function needs an effect a test can observe — an unobservable body (a flag nothing reads, an env var no process consumes) is an unkillable mutant, so leave it out until its consumer exists; test code that drives cargo (nextest, doctest, cargo-mutants, a build) against a throwaway temp Cargo project (package named `viola` so `viola/<feature>` resolves), never a nested build of this workspace. Extended 2026-09-27: a guard that repeats what the call already guarantees (a join after dropping the channel already ends the thread, a chmod after a create-with-mode the umask can only narrow) is just as unobservable — make it the only mechanism or leave it out.
- 2026-09-24: Wait on the exact line a test asserts (receipt, role-file or event line), never on an earlier sibling — a line written just after the awaited one races the assertion.
- 2026-09-24: Inside a `proptest!` body build strings outside the format macro — inline format captures (`format!("{a}{b}")`) fail to compile there (macro hygiene: "there is no argument named …").
- 2026-09-24: A test that sets a process-global `OnceLock` (the panic sink, `ProcessCtx`) is isolated only by nextest's process-per-test, which the harness and `run --mutants` use — keep one such test per global and never rely on a bare `cargo test` run.
- 2026-09-24: A test that forces an I/O error must fail at the same call on every CI OS — a directory opens fine on Linux/macOS (the error comes at read) but fails at open on Windows, while a path under a regular file fails at open with `NotADirectory` on Unix yet reads as `NotFound` on Windows — so cfg-scope the case per OS or pick an input whose failing call matches everywhere.
- 2026-09-24: A test wait that a mutant can reach must detect the watched process's exit (`try_wait`) and be bounded below cargo-mutants' 20 s auto-timeout floor, and so must the nextest mutants kill. Otherwise a mutant that makes the process exit silently spins to the bound and is graded Timeout instead of caught, depending on which tests the runner happens to schedule first. Extended 2026-09-27: a test's own assertion deadline must also sit below the nextest profile's kill line, or the kill preempts the test and its failure dump is lost. Extended 2026-09-27: a deadline EQUAL to the kill line is a race the test can lose — keep it strictly below, and have a spawned child stream its report to a known file outside the test's tempdir, so a runner kill still leaves the evidence.
- 2026-09-24: Keep a `#[cfg(unix)]`-only function to a minimal OS reader, and put its decision logic in a plain function tested on every OS. A Windows host cannot compile, so cannot kill, mutants of a unix-only body; they are killable only on the Linux CI mutants job. Extended 2026-09-24: the mirror holds too — a `#[cfg(not(unix))]` / `#[cfg(windows)]` body is unkillable on Linux, so CI gates the union of the ubuntu and windows legs, and a trivial other-OS stub belongs as a `#[cfg]` block inside one shared function body.
- 2026-09-24: cargo-mutants swaps an operator TOKEN in the text, so replacing the second `||` in `a || b || c` with `&&` re-parses as `a || (b && c)` — a test with `a` set can never kill it; cover each such condition with a case where the leading operand is false.
- 2026-09-25: Every new guard test carries its remove-the-guard run: neutralise the guard, show the test red, restore it, and record both readings. When the guard's own line is outside the chunk diff, run cargo-mutants on its file directly, because `--in-diff` never regenerates those mutants. Extended 2026-09-25: re-Read the guard's site before each neutralising edit and confirm the edit landed before reading the run — the rustfmt PostToolUse hook reflows files after every write, so an Edit anchored on pre-format text misses and the run reads green on the unmodified guard.
- 2026-09-25: Keep every `#[cfg(test)]` module inline (`mod tests { … }`). The orphans gate (`cargo modules orphans`, run without `--cfg-test`) reports an out-of-line `#[cfg(test)] mod x;` file as an orphan, so moving tests out of a file never shrinks it.
- 2026-09-27: To test a timing window, force it open with a test-only hold and show red before the fix and green after, never sample the race — a window that stays green across many isolated runs can still go red under a loaded gate, and a green proves no cause.
- 2026-09-27: A test that scans a home byte by byte (a canary or secret scan) skips files byte-identical to the built `viola` — every home now holds a pinned copy of it (38 MB as a Linux debug build), and scanning it once per canary crosses a CI runner's 10 s kill; remove the slow work, never raise the timeout.
- 2026-09-27: cargo-mutants runs only the mutated package's tests, so a crate's security property (a DACL, a mode, a redaction) needs a test inside that crate — a root integration test alone leaves its mutants missed.
- 2026-09-27: cargo-mutants mutates `const` initializers, and `|` → `^` over disjoint flags is an equivalent mutant — write a combined flag value as one literal and pin it with a test.
- 2026-09-27: A scoped tracing subscriber installed as the only dispatcher caches a callsite first hit on another thread as disabled, so a test asserting a log line emitted off the test thread captures through a process-global subscriber.
