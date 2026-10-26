# Task — obs-plan · 2026-10-25-windows-endpoint-admission

## The session
A test-only chunk: no product file changed, and no product behaviour, wire contract, exit code, refusal word, process-log line, `event` value or log field moved.
It added five Windows-only `#[test]` cases in `tests/security_negatives_channel.rs`. They assert that the viola pipe listener rejects remote clients (a pipe-type read-back of `FILE_PIPE_REJECT_REMOTE_CLIENTS`) and that its DACL denies an anonymous-token open (`ERROR_ACCESS_DENIED`). Each has controls on test-owned pipes, and all are green on CI `test (windows-2025)` (ci#40247500584).
It added the root-bin test helper `test_support::with_scoped_default` in `src/main.rs`. The helper pins one always-interested, no-layer `tracing_subscriber::registry()` Dispatch in a `OnceLock`, never as the global default, and the nine root-bin scoped log captures now go through it. The witness test `scoped_capture_sees_a_line_whose_callsite_another_thread_registered_first` covers the fix: under tracing-core 0.1.36, a scoped capture that was the only dispatcher lost a line whose callsite a subscriber-less thread had registered first.
It added a root `[target.'cfg(windows)'.dev-dependencies]` windows-sys table with the features `Wdk_Storage_FileSystem` and `Win32_System_IO`. `Cargo.lock` and the release graph are unchanged.
Measured: the full CI row is red on `supply-chain` alone (RUSTSEC-2026-0386 against the unchanged tokio 1.53.1), and the new route entry "tokio RUSTSEC-2026-0386 lock bump" owns it. Under plain `cargo test --bin viola` (an unsupported runner), two known-cause unit-test reds are recorded with no owner. Every other gate is green.

## Reached files
- tests/security_negatives_channel.rs
- Cargo.toml
- src/main.rs
- src/run/send/test_support.rs
- src/run/dialog.rs
- src/run/budget.rs
- src/run/gate.rs
- src/obs.rs
- src/cmd/run.rs
- src/cmd/hook/statusline.rs
- viola-0.1.0/verification-matrix.json

## Touches
- Any text on how the root bin's unit tests capture process-log lines: captures now go through `crate::test_support::with_scoped_default`, not `tracing::subscriber::with_default`. The census `grep -rn 'subscriber::with_default' src` moved from 9 to 1, the helper's own at `src/main.rs:287`.
- Any census of `obs_event!` call sites or of `ObsEvent::StateRecovered` / `torn-line-healed` emitters over `src`: the witness's private `emit_witness_line` in the `#[cfg(test)]` module of `src/main.rs` adds one test-only site. It is not a product emitter, and the torn-append rule (the healing appender writes one line per heal) is unmoved.
- The logging-stack and logger-install claims: the binary's one global `set_global_default` (`src/run/gate.rs:429`) is untouched, and the helper installs no global default. Re-read, unmoved.
- The tracing crate versions: tracing-core 0.1.36 and tracing 0.1.44 were read from the pinned sources, and tracing-subscriber 0.3.23 stands. No version moved and no obs dependency was added.
- The supply-chain trigger (security Vector 9: obs dependencies Tokio-free in the sync crates, no C crates, `cargo deny` passing): `cargo deny check bans licenses sources` and `bash scripts/channel-tokio-free.sh` are green. The CI `supply-chain` red is a tokio advisory owned by a route entry, not an obs dependency. Claim unmoved.
- The windows-sys dev-dependency features reach no release build under `resolver = "3"`. Any obs statement about the release graph or the sync crates' dependency set is unmoved.
- CI integration: the new Windows cases run inside the existing `test (windows-2025)` instrumented nextest run. The `junit-windows-2025` artifact carries them (2988 tests, 0 failures). No CI step, upload, gate condition or retention changed.
- G1 (bare `#[instrument]` rg), G2 (`scripts/g2-zero-panics.sh`), G4 (`agent-run schema-check`), `scripts/lint-probes.sh` and clippy for the host and `x86_64-pc-windows-msvc` targets: all green at this chunk, and no gate command changed.
- The vt100 feed rule (one `parse-rejected{parser:"vt100-feed"}` per poisoning episode): its unit test `run::gate::tests::feed_panic_writes_one_parse_rejected_line_until_the_size_changes` fails only under plain `cargo test` and is green under nextest. The product claim is unmoved; the red is recorded with no owner.
- The config-rejection WARN line: its unit test `obs::tests::log_config_rejection_writes_one_warn_line` likewise fails only under plain `cargo test`. The claim is unmoved; the red is recorded with no owner.
- The `send-refused` line (each deciding side's line carries no text): case_4's missing capture under libtest threads in `src/run/send/test_support.rs` was a test-capture loss, now fixed in the capture. The product line and its fields are unmoved.
- The obs-line assertions in the root-bin tests of the run start, budget gate, dialog, statusline, readiness gate and obs modules: only their capture call moved. No asserted event, field or level changed.
- The ipc-internal channel surface (`viola-channel` on a Windows named pipe): listener admission is now asserted on CI. No channel log line, `corr` / `conn` field or `channel-request` / `channel-response` shape changed.
- Test-only pipe names: the new cases use the `viola-test-chan-<pid>-<label>` namespace with labels `remote`, `accept-remote`, `reject-remote`, `anon` and `anon-default`. Any registry or path citation of that namespace is still true.
- Any restatement that remote pipe clients or the Windows pipe DACL cannot be driven on hosted runners is now stale for those two, measured on `test (windows-2025)`. The cross-euid peer and the second OS account still stand.
- The obs acceptance as asserted: the witness's line and every capture's lines go through `obs_event!`, G2 reads zero panics and G4 reads conformant.
- The crate-level scoped captures `crates/viola-state/src/lib.rs:87`, `crates/viola-channel/src/test_support.rs:184` and `crates/viola-channel/src/lib.rs:199` share the mechanism but were left unchanged and have no owner. Any text naming those captures is unmoved.

## New facts
- (none)

## Untouched
- Metric Coverage: derived `viola.<area>.<measure>` metrics, the label-cardinality discipline and the `v_rejected` over-count. No metric source line or grouping key changed.
- PII Scrubbing: veil `Redact` and its banned `toggle` feature, the default-deny schemas `schemas/diag-line.v1.json` and `schemas/diag-detail.v1.json`, the secret-scan inputs and the hit report. No field, schema or scan path changed.
- CI Integration step order, upload gating on `steps.secret-scan.outcome`, `retention-days: 7`, `nightly.yml` and `windows-mutants.yml`. No workflow file was edited.
- SLO perf budgets (hyperfine `max` through `gate --require perf`) and the heartbeat liveness flip at >5 s. No hook timing, perf row or heartbeat code changed.
- The obs tier (Standard with Minimal exporter carve-outs) and its justification. No tier driver moved.
- Error-reporter admissibility (sentry rejected, the four preconditions for any future reporter). No dependency was added.
- The edge flows E3 (implicit link only) and E5 (`state-recovered{detail:"snapshot-replayed"}` from `viola_state::replay`). No viola-state or links code changed.
- The `wait` / `last` readback scenario and the MCP surface (`viola mcp`, the `mcp.ndjson` sink, codes-only `isError` lines). No viola-mcp or wait/last code changed.
- The `verify` / `plugin install` edge flow and its uninstrumented capture arms. Neither verb's code changed.
- The home and `diagnostics/` DACL / 0600-0700 rule. The session's DACL assertions concern the pipe listener's descriptor, not diagnostics files.
- The not-instrumentable entities (`viola-core`, `config.json` and plugin files, build and CI infrastructure).
- The Universal anti-patterns (no proprietary APM, no dashboards, no runtime service identity from env, Normal mode, not inverted).
- The Obs Decisions Log. The session made no obs decision and widened no event enum.
