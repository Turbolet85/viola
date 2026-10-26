# obs · CI Integration — category read

## Carried
- (.andromeda/obs-plan-detail/ci-integration.md:24) Every viola home the chunk's new root cases make (`tests/channel_links.rs` and any test of the `link` / `unlink` verbs) must live under `target/e2e-home/`, with no second outside-scan home: the chaos panic test's home is the one carve-out.
  > `diagnostics/*.ndjson` from fake-agent runs. Their viola homes must live under `target/e2e-home/` (the directory layout is tests-owned), so G2, G4, the secret scan and the `diag-<os>` upload cover them. A `hook` panic exits 0, so a home outside that root would hide it. The one carve-out: `tests/chaos_feed_panic.rs`'s `TestHome::outside_scan()` home under the system temp dir, outside G2, G4, the secret scan and the upload, whose deliberate panic line and `parse-rejected` line the test asserts present itself (the founder's ruling, live, 2026-10-04, relayed by the overseer).
- (.andromeda/obs-plan-detail/ci-integration.md:22) The new verbs', the wrapper's two served methods' and the tests' code must pass the lint-stage gates: no raw tracing level macro outside `viola_core::obs`, no print / dbg macro, no raw `event!`, no bare `#[instrument]`.
  > clippy `-D warnings` incl. `disallowed-macros` (raw `tracing::{info,warn,error,debug,trace}!` outside `viola_core::obs`) and the `print_stdout` / `print_stderr` / `dbg_macro` bans; the fail-closed raw-`event!` grep and both-ways ban probes in `scripts/lint-probes.sh`, Linux leg (mechanism in §3 logger-stack-install); bare-`#[instrument]` gate G1 (below);
- (.andromeda/obs-plan-detail/ci-integration.md:34) Any span the chunk adds (the verbs' client span, the wrapper's `link` / `unlink` dispatch) must take `skip_all` as its first `#[instrument]` argument, or G1 fails.
  > `skip_all` must be the first argument (the §4 form).
- (.andromeda/obs-plan-detail/ci-integration.md:36) The chunk's runs must leave zero `event:"panic"` lines in the home-level role files G2 counts, in the `test` and `perf` jobs.
  > **G2, zero panics.** It runs as `scripts/g2-zero-panics.sh` (fail-closed; `tool-missing: jq` without jq), `--probe` first, in the `test` and `perf` jobs; the snippet below is its logic.
- (.andromeda/obs-plan-detail/ci-integration.md:39) No new panic exemption may be added for the chunk's tests: the only line G2 leaves out is the `fake-agent` seam's.
  > One exemption, ratified by the founder live on 2026-09-28 (security-plan Decisions Log): a line whose `panic_location` is exactly `src/cmd/hook/seam.rs:<digits>` — the `fake-agent` seam's forced panic — is not counted;
- (.andromeda/obs-plan-detail/ci-integration.md:41) Every process-log line the new verbs (`cli`) and the wrapper's `link` / `unlink` methods (`run`, `channel-request` / `channel-response`) write must conform to `schemas/diag-line.v1.json`, and any detail line to `schemas/diag-detail.v1.json`, or G4 fails.
  > Scope: every line in the G2 file scope is checked against `schemas/diag-line.v1.json`, and every `instances/*/diagnostics/detail-*.ndjson` line against `schemas/diag-detail.v1.json` (§8).
- (.andromeda/obs-plan-detail/ci-integration.md:42) If the new lines need a schema change (a `method` value, an `event` value or a field), the schema moves in the chunk, owned by obs, with the validator body tests-owned.
  > The check body and the JSON Schema validator crate are tests-owned; obs owns both schemas.
- (.andromeda/obs-plan-detail/ci-integration.md:59) The chunk's test homes must survive until G2, G4, the secret scan and the uploads have read them: a test home's drop must honour `AGENT_RUN_KEEP_HOMES=1`.
  > **Homes are kept until the gate steps have run** (overseer fix pass 2, B1): `ci.yml` sets `AGENT_RUN_KEEP_HOMES=1`, so neither harness `cleanup` nor a test home's drop deletes a home before G2, G4, the secret scan and the scan-gated uploads have read it (test-plan §3 `cleanup` step 6).
- (.andromeda/obs-plan.md:612) The new verbs' and methods' log lines must carry no CI env metadata: the product reads no env as configuration.
  > **CI-specific resource attributes:** none are read by the product. Env vars are not a configuration channel, so `deployment.environment`, `ci.run.id` and `git.commit.sha` are **never** injected into viola log lines.

## Bindings
- **Integration-test homes ↔ test-plan §3 (directory layout, tests-owned)**: the homes of `tests/channel_links.rs` sit under `target/e2e-home/` per the tests-owned layout, so G2, G4, the scan and `diag-<os>` cover them (.andromeda/obs-plan-detail/ci-integration.md:24), (.andromeda/obs-plan.md:600)
- **G4 ↔ obs §6 Log Coverage (field catalog, default-deny) and §4 E3**: the served `link` / `unlink` methods end E3's "no process-log line" state for the explicit methods, so their `channel-request` / `channel-response` and verb `process-start` / `process-exit` lines must sit inside the §6 catalog and the schema G4 checks (.andromeda/obs-plan-detail/ci-integration.md:41), (.andromeda/obs-plan.md:392)
- **G1 ↔ obs §4 span form**: `#[instrument(skip_all, ...)]` with `skip_all` first, the form §4 and §11 Spans name (.andromeda/obs-plan-detail/ci-integration.md:34), (.andromeda/obs-plan.md:604)
- **Kept homes ↔ test-plan §3 `cleanup` step 6**: `AGENT_RUN_KEEP_HOMES=1` keeps the chunk's homes for the gates (.andromeda/obs-plan-detail/ci-integration.md:59)
- **Zero panics ↔ obs §10 error budget**: G2's exit code is the panic count the §10 zero budget reads (.andromeda/obs-plan-detail/ci-integration.md:36), (.andromeda/obs-plan.md:655)
