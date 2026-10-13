# obs extract

## Relevance
partial — the chunk adds tests and behaviour-preserving restatements and records survivors for the Epoch 5 boundary audit; obs binds through the mutation gate on obs code, the "no emitted-line change" rule for restatements, and the scan/G2/G4 coverage any new test home must sit under.

## Constraints
- A surviving cargo-mutants mutant in obs code is a build failure condition at a `run --mutants` (the epoch-boundary audit); "surviving" is the harness document's `survived`, so a missed mutant the host never compiles is left out (per obs-plan §10 Build / deploy failure conditions). Whether any of the 47 sites is obs code — candidates by name only: `src/run/dialog.rs` `millis` (a `duration_ms` / `deadline_ms` source?), `src/cmd/verify/typed.rs` `Run::finish` `:671:53` (a `verify-pty-probe` `process-exit.duration_ms` source?), `viola-state` `board.rs` `SkippedTotal::add` (the `parse-rejected{parser:"state-entry"}` `count` or the `list --json` `skipped` gauge?) — is research's question.
- Mutation runs only at the epoch boundary through `/andromeda-code-audit` or on demand, never as a push/PR job; the Windows leg is the dispatch-only, report-only `windows-mutants.yml` (per obs-plan §9 Pipeline integration, Mutation row). This matches the chunk's no-dispatch boundary: a "not measured here" record names that audit as its reader.
- A restatement may not change any emitted line: the `event` values, the additive fields and the closed `detail` codes stay exactly as catalogued, and a field outside the catalog is a G4 failure (per obs-plan §6 Additive field catalog and §6 `detail` code catalog).
- `board.rs` `check_if_present` / `scan` (the `NotFound` guards): `board::read` writes one `parse-rejected{parser:"state-entry", detail:"symlink-ignored"|"malformed", count}` per nonzero count after its loop, naming no entry, and `board::scan` itself logs nothing (per obs-plan §6 `detail` code catalog, Filesystem refusals); a refused `viola list` scan writes no line at all (per obs-plan §10 Bounded exemptions, item 7). A test that kills these mutants must not expect a line on a refused scan.
- `viola-agent-claude` `statusline.rs` `shell_argv`: a `statusline-shell` `process-start` / `process-exit` pair exists only when a shell is built (never on Windows, where `shell_argv` is `None`), and `statusline_command` never appears in any span or home-level line (per obs-plan §4 Scenario: Budget governor).
- `viola-e2e` `harness/run/mutants/scratch.rs` `prepare`: the host mutation scratch's path is never printed — documents carry only the byte count `scratch_bytes` — and `mutants.out/` is never uploaded (per obs-plan §8 PII Scrubbing, Integration points item 6, Unscanned uploads).
- A new integration test's viola home lives under `target/e2e-home/` so G2, G4, the secret scan and the `diag-<os>` upload cover it; the one carve-out is `tests/chaos_feed_panic.rs`'s `TestHome::outside_scan()` (per obs-plan §9 Pipeline integration, Integration tests row).

## Patterns to follow
- Unit tests that check emitted lines use an in-memory `Vec<u8>` writer and parse the lines with serde_json against `schemas/diag-line.v1.json` (per obs-plan §9 Pipeline integration, Unit tests row).
- Every E2E and integration input is synthetic and embeds the tests-owned canary, which may appear only in `detail-*.ndjson` (per obs-plan §8 PII Scrubbing, Integration points item 6).
- Spans stay `#[instrument(skip_all, fields(...))]` with `skip_all` first, and log lines go only through `obs_event!` (per obs-plan §8 PII Scrubbing, Integration points item 1; G1 in §9 Gate commands) — a restatement that moves an instrumented function keeps its attribute form.
- `viola verify`'s Run B driver logs each interactive PTY run as one `verify-pty-probe` `process-start` / `process-exit{child_exit_status, duration_ms}` pair, with no row text, prompt or path (per obs-plan §4 Edge flows, `verify` / `plugin install`); a fake-PTY test of `typed.rs` keeps that pair's shape.

## Anti-patterns to avoid
- NEVER use `print!` / `println!` / `eprint!` / `eprintln!` / `dbg!` in the product crates' `hook`, `run` or `mcp` code paths, and NEVER call raw `tracing` level macros or `event!` outside `viola_core::obs` — a test-only seam in `src/` included (per obs-plan §11 Logs).
- NEVER add a retry-once policy for a flaky assertion (per obs-plan §11 SLO); a test shown failing under its hand-applied mutant is shown once, under `retries = 0`.
- NEVER put paths or pids inside `detail` codes or a home-level line (per obs-plan §11 PII Scrubbing) — including a "not measured here" record's wording if it lands in a harness document.

## Contract bindings
- obs ↔ tests: the mutation gate's verdict and the host-excluded rule are test-plan §10 Mutation gate and §3 `run` step 4; obs only names obs code as in scope (per obs-plan §9 Pipeline integration, Mutation row).
- obs ↔ tests: G2 / G4 / secret scan read every home under `target/e2e-home/`; the home layout and the canary are tests-owned (per obs-plan §9 Pipeline integration, Integration tests row; §8 Integration points item 6).
- obs ↔ security: `statusline_command` integrity and the scratch path's non-disclosure (per obs-plan §8 Data classification rules).

## Acceptance criteria contributions
- (obs) No restatement changes an emitted line: `cargo nextest` over the touched packages stays green and G4 reports zero schema-conformance failures on the chunk's CI run (per obs-plan §10 Error budget).
- (obs) Any of the 47 sites that research finds to be obs code is disposed by a killing test or a restatement, never by a "not measured here" record, unless the host cannot compile it (per obs-plan §10 Build / deploy failure conditions).
- (obs) Every new test home that writes `diagnostics/` sits under `target/e2e-home/`, and G2 reads zero non-seam `event:"panic"` lines on the chunk's CI run (per obs-plan §9 Pipeline integration, Integration tests row).
- (obs) A new `board.rs` test asserts no process-log line on a refused scan, and a new `scratch.rs` test or record prints no scratch path (per obs-plan §10 Bounded exemptions, item 7; §8 Integration points item 6).
