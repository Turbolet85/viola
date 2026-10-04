# Tests Summary — viola

_Distilled from `.andromeda/test-plan.md` by `/andromeda-setup-project`. wrap-session does not modify._

## Test tier

**Tier:** Comprehensive (2)

**Justification (one sentence):** nine security vectors needing negative tests, seven parser surfaces handed over for property/fuzz coverage, eight agent-driven surfaces across three required OSes and seven cross-surface critical paths — plus founder-mandated mutation testing from chunk 1, crash-safe-state chaos, hook-deadline perf budgets and multi-version CLI compat.

## Harness contract (§3)

The harness is the agent-driven verification surface. `scripts/agent-run.{sh,ps1}` are thin shims over `cargo run -q -p viola-e2e --bin viola-harness -- <command>`; all logic lives in Rust. See `.claude/rules/verification-harness.md` for path-scoped enforcement.

- **Test runner:** cargo-nextest 0.9.146 (process-per-test, JUnit, `retries = 0`) + `cargo test --doc`; Playwright 1.63.0 headless Chromium (all three CI OSes and the native Linux `pre-push`, `run --browser` only; under the pinned Node v24.21.0) for the page
- **5-command discipline:** boot / run / status / cleanup / logs — each prints one JSON document `{"v":1,"cmd":…,"ok":…}`, exit 0/1/2
- **Status:** `agent-run status` aggregates `viola list --json`, `GET /ready` and cookie-gated `/api/sessions` (`state: ready|degraded|down`, `api_sessions_equal_list`)
- **PID file:** none as a product file — pids live in `instances/<name>/snapshot.json`; the harness record is `target/agent-run/<session>/session.json`
- **Log format:** JSON-per-line — `events.ndjson` (arch event line) + process logs in `<home>/diagnostics/*.ndjson`; required fields `timestamp level target message event process instance corr` (bound to obs-plan §3)
- **Tempdir convention:** a not-yet-existing home under `target/e2e-home/viola-session-*/home` (harness) or `target/e2e-home/viola-test-*/home` (root rstest chain, `tests/support/home.rs`) — viola creates it, except on Windows x64 where `seed_conpty` first hard-links the ConPTY companions into a root test home's `bin/<key>/conpty/` from `target/conpty-seed/<key>/` (`Wrapper::boot`, piped and outer-PTY starts; never `run_viola_unseeded` or `conpty_sideload`, which keep the product's write; the Epoch 6 entry owns closing it), and except `tests/chaos_feed_panic.rs`'s `TestHome::outside_scan()` home under the system temp dir, outside G2 / G4 / the secret scan (the founder's ruling, 2026-10-04); kept in CI (`AGENT_RUN_KEEP_HOMES=1`; never under `run --mutants`) until obs gates and the secret scan have read it, a root home's leftover removed only once its recorded owner process is gone (removal deletes `owner.json` last, `remove_owned`), and on a failing test under `AGENT_RUN_KEEP_FAILED=1` (viola-e2e's booted lifecycle tests through a `Booted` drop guard that stops the session first, and removes the home of a passing test)
- **Fake agent:** `viola-fake-agent` (root `[[bin]]`, feature `fake-agent`) replays `fixtures/claude/<cli-version>/<Event>.<variant>.json` recorded by `viola verify`, runs absolute exec-form hooks from `<plugin-dir>/hooks/hooks.json`, gates script steps on `--control`, and writes an ndjson `--receipt`; its print mode `-p/--print <prompt>` fires the four spine fixtures, prints `ok` and exits 0 (what `viola verify` drives: `tests/cli_verify.rs` pins verify's lines by literal asserts); CI stamps homes only by running `viola verify` against it at the recorded `DEFAULT_CLI_VERSION` 2.1.283 (the root `stamped_home` and harness `boot` step 4, unless `--unstamped`; `StampedHome::unstamped` for the unverified-path tests); the real `claude` never runs in CI — only the local `run --local-live`, refused under `CI` (`live-in-ci`). The committed `fixtures/claude/*/*.json` (first set `2.1.283`) are walked for hygiene by a run-time directory walk against `schemas/claude-fixture.v1.json` (rstest 0.27 `#[files]` refuses an empty glob at compile time); the same walk drives the drift contract `contract_fake_agent_drift`, which compares each print-mode hook's receipt `stdin_hex` byte for byte with the recorded fixture and the hook order with the spine literal — no insta snapshot

## E2E coverage (§6)

- **Path 1 — `run` start sequence** — event order `wheel{start}` → `budget-gate` → `session-start` (the third with "Hooks to normalised events"); duplicate / squatted / tampered-exe starts exit 1; plugin files rewritten with absolute pinned paths
- **Path 2 — confirmed `send` + CL-1 records** — `cursor` = pre-paste offset, `send-issued` then `prompt-submitted{driver}`; `no-prompt-submitted` → exit 13 + `send-refused`; a second send in flight → exit 13 `turn-running`; local command → `not-delivered` until `:82` lands the local-command rows, then `unconfirmable`; MCP / SSE / web halves owed to `:102` / `:131` / `:139`; readback `open → read`
- **Path 3 — `wait` / `last`** — wakes only on driver-relevant kinds, returns already-logged events at once, typed timeout, exit 21 on a vanished wrapper
- **Path 4 — dialog → `answer`** — insta-pinned decision bodies (S3/S7/S8), one pending dialog, `unknown-dialog`, no decision without stamp + wheel `driver`
- **Path 5 — the wheel** — human key → `human-typing` (exit 10), `pause` → `manual-pause`, `release` back, `release` with `from` → exit 20, harness turns never flip it
- **Path 6 — budget governor** — 90/85 thresholds, exit 11, per-instance `release --budget` override, statusline pass-through
- **Path 7 — unverified CLI** — transport works, dialog answers withheld (exit 12)
- **E1–E5** — unwrapped hooks no-op · R8 env strip · link/unlink markers · SSE `Last-Event-ID` resume · snapshot corruption → replay
- **Non-path suites** — security sweep, secret scan + canary, schema conformance (G4 body), exit-cause matrix, security-control negatives, `list` row integrity, cross-surface parity, bay layout states, chaos, property (7 parsers), contract

## Quality gates (§10)

| Gate | Threshold | Tool |
|---|---|---|
| Coverage (line) | ≥ 85 % per OS | cargo-llvm-cov 0.9.1 `--fail-under-lines` |
| Coverage (branch → region) | ≥ 80 % | `--fail-under-regions` |
| Coverage (function) | ≥ 95 % | `--fail-under-functions` |
| Mutation | 0 missed, 0 timeout, unviable ≤ caught at a `run --mutants` (the epoch-boundary audit, or on demand) | cargo-mutants 27.1.0 `--in-diff`; more unviable than caught mutants is red (`unviable-exceeds-caught`: the builds failed, nothing was tested); a Rust delta reads a fresh `outcomes.json` (`verdict:"counted"`), a diff with no `.rs` path passes as `verdict:"no-rust-delta"`, and one whose `.rs` paths are all test targets (`tests/`, `benches/`, `examples/`) as `verdict:"test-only-rust-delta"` (cargo-mutants mutates none of them); a mixed diff is `counted`; `--file <path>` is the scoped inner loop (`verdict:"scoped"`); `--package <member>` scores one whole member, the boundary tier's form (`verdict:"package"`, viola-e2e included); a mutant the host cannot compile or reach is "not measured here; owed to {route entry}" by coordinate, never "equivalent", and `missed == 0` reads over the measurable set. On the Linux dev host every mutation run takes a NOCOW btrfs `TMPDIR` (`<repo parent>/viola-mutants-scratch`). On a Windows host the run's temp copies and `mutants.out/` live in the host mutation scratch `<repo parent>/viola-mutants-scratch` (guarded, wiped first, `scratch_bytes` reported); each run's `outcomes.json` and JUnit are archived in `target/run-archive/<n>`. No chunk, pre-push or CI mutation gate since 2026-09-28: `run --mutants` is named-only, and `/andromeda-code-audit` runs mutation at the epoch boundary, its Windows leg the dispatch-only, report-only `windows-mutants.yml` (`--package <member> --file …` per package on `windows-2025`); the `mutants` nextest profile is `terminate = "wait"`; the two real-cargo-mutants harness self-tests are compiled out on macOS (runner-side, measured) |
| Flakiness budget | zero — no retries, a flake keeps the chunk red | nextest `retries = 0`, Playwright `retries: 0` |
| Performance budget | hook `max` < `viola_core::SPINE_DEADLINE` (1.0 s), the constant `gate.rs` `perf()` imports (SessionEnd and the spine hooks; the hook's own connect deadline is the provisional 750 ms `CONNECT_DEADLINE`, asserted below it) | hyperfine 1.20.0 `-N --warmup 3 --runs 30` via `run --perf` (four rows; `pre-tool-use` untimed until the dialog tier), gated on `max` by `gate --require perf` in the per-OS `perf` job; the fail-open matrix also asserts `< 1.0 s` per case |
| Per-job verdict | every required suite present, 0 failed, 0 skipped, artifacts present | `viola-harness gate --require …` |

## Universal anti-patterns

- NEVER use `sleep(N)` for synchronization, a retry budget, or `#[ignore]` / `test.skip` as a parking place.
- NEVER parse the rendered child screen; verdicts come from events, payloads, receipts and DOM attributes.
- NEVER run the real `claude` or real-CLI `viola verify` in CI; NEVER treat a green fake-agent run as proof of real-CLI behaviour.
- NEVER hand-write stamps, snapshots or `budget.json`; NEVER use `std::env::set_var`.
- NEVER write a test that auto-approves a dialog.

## Critical decisions

- `crates/viola-e2e` (test-only, `publish = false`) hosts `viola-harness` and the Tokio clients, keeping the sync root package tokio-free.
- Stamps conflict resolved: `viola verify` against the fake agent is the only writer of `ledger/stamps.json` in tests and CI.
- Branch coverage is enforced as LLVM region coverage; mutation testing covers branch strength.
- hyperfine is the perf gate (criterion does not exit non-zero on regression).
- Arch requests pending: `viola ui --port 0`. (The spine deadline is `viola_core::SPINE_DEADLINE`; the confirmation window's built-in fallback is the provisional `CONFIRM_WINDOW_FALLBACK` = 10 s, its per-version ledger row held.)

---

**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: `.claude/rules/testing.md`, `.claude/rules/verification-harness.md`.
