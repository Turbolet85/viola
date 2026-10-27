# Task — obs-plan · 2026-10-27-pinned-copy-race-and-reproducible-builds

## The session
- Changed the pinned copy's absent-copy write: `viola_state::pin::pin_bytes`'s `NotFound` arm now calls the private `place_absent`, which uses the new public `viola_state::fs::place_private_new` (enum `Placed { Written, Present }`). It places a closed temp file without overwriting anything, so a concurrent first start can no longer exec the copy while another start still has it open for write (`ETXTBSY`).
- When the placement is lost (another start placed the copy first), the copy already there is re-hashed. If it is equal it is reused; if it differs the result is `PinError::HashMismatch` and the file is left as found. `replace_private_shared` keeps only the plugin files and the ConPTY companions, and its body is unchanged.
- Measured: the product race failed 4 of 24 two-instance boots before the fix (each `spawn_os_error:26`, `ETXTBSY`) and 0 of 24 after. Measurement B: one tree built to two different files whose loaded sections are equal. Its cause is not explained and it is the founder's item.
- Test-only changes: the fake agent's `hook` receipt gained `spawn_os_error` on a failed absolute spawn. The Windows plain-home oracle now reads the inherited untrusted write grant by its right, whichever SID holds it. Path 7's MCP leg now boots the default `overseer` + `builder` pair unstamped, which retires the test-side race workaround.
- Added no log line, field, span, `detail` code, metric, config key, env var or flag. `run.pin_copy` keeps `ok|pinned-hash-mismatch`, and `schemas/diag-line.v1.json` is unchanged. CI `ci#40439188053` is green 15/15. Red 1 (Path 1 timeout) is recorded as runner variance under a WATCH, PROVISIONAL.

## Reached files
- crates/viola-state/src/fs.rs
- crates/viola-state/src/pin.rs
- crates/viola-state/src/strict.rs
- src/bin/viola-fake-agent.rs
- crates/viola-e2e/tests/mcp_tools.rs
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/preconditions.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/repro-unstamped.py
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/race-before.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/race-after.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/red-green.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/repro-builds.sh
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/reproducible-builds.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/path1-phases.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/operator-pass.md

## Touches
- The `run.pin_copy` outcome in the `viola run` start-sequence scenario (span-trace-coverage) keeps `ok|pinned-hash-mismatch`. A lost placement whose standing copy re-hashes equal now reads `ok`, and one that differs reads `pinned-hash-mismatch`.
- The `run` start refusal for a tampered pinned copy (obs-plan.md §3, the `run` terminal rule): it is now also reached when a concurrent first start's standing copy re-hashes different. The `run` refusal site count (7) is unchanged because `src/cmd/run.rs` is unchanged.
- The refusal telemetry for pinned-exe hash mismatches (obs-scope-summary, the strict-modes / home-integrity required telemetry signal) gains the lost-placement path through `place_absent`'s `HashMismatch`. No new line.
- The start order naming the pinned copy at obs-plan-detail/log-coverage.md:27: re-read, no change.
- The tampered-copy refusal's text and output at obs-plan-detail/error-capture-reporting.md:8 and obs-plan.md:311: re-read, no change.
- The start order naming the pinned copy in registries/contracts/obs-plan/otel-sdk-init.md:21: re-read, no change.
- The Path 7 heading at obs-plan-detail/obs-scope-summary.md:329: re-read, still true. Its MCP leg now runs on the default two-instance pair, unstamped.
- The obs-scope path groups that map Path 1 and Path 7. Path 1 is unchanged: nothing was cut and no bound moved; red 1 is runner variance under a WATCH, not a spec fact.
- The `pinned_bin` never-logged rule (span-trace-coverage run-start scenario; pii-scrubbing §8 path-free home lines): `place_private_new` adds no line or span, so the rule holds unchanged.
- The `detail` code catalog and the log-field set (log-coverage §6): no code or field was added, and `schemas/diag-line.v1.json` is unchanged.
- The `sideload_fallback` `strict-modes-failed` line on a refused `conpty/` companion: `pin_companions` and `replace_private_shared`'s body are unchanged, and `pin.rs` now imports `replace_private_shared` under `cfg(windows)` only.
- The strict-modes failure line rules (obs-scope-summary, the `ledger-stamps` read and the companion check): the Windows plain-home change is only to a test oracle in a `cfg(windows)` test module. The product verdict `Err(Writable)` was right and no line changed.
- The fake-agent clippy print-lint exemption (obs-anti-patterns, the print rule): the fake agent gained the `spawn_os_error` receipt field. It is a receipt, not a product log line, and the exemption still covers it.
- The CI gate set (ci-integration §9): G2, the secret scan, schema-check and the uploads are unchanged. CI `ci#40439188053` is green 15/15, `g2-zero-panics.sh` and `agent-run schema-check` are green, and no CI step changed.
- The perf budgets (slo-invariants-telemetry-budgets §10): `run --perf` is green with 7 passed and 0 failed, and no budget or hyperfine gate moved.

## New facts
- F1 · span-trace-coverage · On a concurrent first start, the pinned copy's absent-copy write is a no-clobber placement of a closed temp file (`fs::place_private_new`). The start that loses the placement re-hashes the copy that stands, and `run.pin_copy` reads `ok` when it is equal or `pinned-hash-mismatch` when it differs. The placement adds no line, field or span.

## Untouched
- metric-coverage §5: no metric and no grouping key was added. The closed-enum label set is unchanged.
- telemetry-strategy §2: no OTel SDK, exporter or sink change. The harness `logs` / `status` documents are unchanged.
- The confirmed-send, `wait` / `last`, dialog-answer, wheel and budget-governor scenarios in span-trace-coverage: none of their code paths was changed.
- pii-scrubbing-compliance §8 redaction lists: no field crosses into a log. `spawn_os_error` is an integer on a test-only receipt.
- slo-invariants-telemetry-budgets' zero-panic invariant and its single `fake-agent` seam exemption at `src/cmd/hook/seam.rs`: no seam was added or moved.
- Measurement B (one tree builds two files and can key two `bin/<key>/` folders): no obs-plan site holds build reproducibility. It is the founder's item on the route.
- The `verify` / `plugin install` logging rules and the `revive` refusals: `src/cmd/verify.rs` and the `revive` and `plugin` code are unchanged.
- The §12 Obs Decisions Log: the session made no obs decision.
