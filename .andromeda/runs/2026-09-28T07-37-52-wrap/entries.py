"""Write this pass's sidecar entries: one payload file per entry (a leading blank line, then the entry)."""
from pathlib import Path

RUN = Path(__file__).resolve().parent
REF = "**Ref:** .andromeda/runs/2026-09-28T07-37-52-wrap/"
M = "2026-09-28-hook-perf-gate"

ENTRIES = {
    "architecture-1": f"""## {M} — the second fake-agent test seam `FAKE_AGENT_HOOK_PANIC`
**Section:** §Established Decisions [Naming]; §Conventions Environment variables; §Occupied Resources → Environment variables (Test seams); §Cross-cutting Patterns Config management
**Change:**
- Was one ratified exception (`FAKE_AGENT_PUMP_DELAY_MS`); now two test seams, each read only under `cfg(feature = "fake-agent")` and absent from release builds.
- `FAKE_AGENT_HOOK_PANIC`: `fn panic_if_asked` in `src/cmd/hook/seam.rs` (module declared without a cfg), called in `hook()` right after `viola_obs_init`, before the stdin lock; panics only on exactly `1` with the fixed 4 608 B payload `"forced-hook-panic ".repeat(256)` → one codes-only `panic` role line + one `detail-hook.ndjson` line over 4 KiB, exit 0, empty streams. Named in product source only in that file; configures nothing, disables no control, widens no redaction.
**Why:** the fail-open contract proven on a real panic in the real binary. A boundary widening, ratified by the founder live on 2026-09-28 (the seam at 06:21; the seam with G2's exact-path exemption at 09:52:07; relay the Viola overseer).
{REF}
""",
    "architecture-2": f"""## {M} — perf arm registry, G2 script, the `perf` CI job, hyperfine in §Stack
**Section:** §Stack and Technologies Code quality; §Occupied Resources Repository (`target/agent-run/`, `target/perf/`, `target/g2-probe/`); §Infrastructure Patterns directory tree (`scripts/`, `ci.yml`); §CI/CD Setup steps and Jobs wired today
**Change:**
- `target/perf/` builds with `--features viola/fake-agent` (was `fake-agent`); its hyperfine exports are `target/agent-run/artifacts/perf-<hook>.json`, four rows, all required by `gate --require perf` (was the separate `perf/*.json`); the perf session's `<session>/` holds its synthetic `payload-<hook>.json`.
- New `target/g2-probe/` (the `--probe` scope) and `scripts/g2-zero-panics.sh` (G2, fail-closed, exempting only `src/cmd/hook/seam.rs:<digits>`, + `--probe`).
- CI: 9 jobs / 18 check-runs (was 8 / 15, as measured at ci#36390764600); the per-OS `perf` job (hyperfine via `cargo install --locked`, `run --perf`, G2 probe then check, G4, its own scan, scan-gated `perf-<os>` / `diag-perf-<os>`, `gate --require perf`); the `test` job's G2 step runs the script; `test`, pre-push and WSL carry no perf step.
- §Stack lists hyperfine 1.20.0 (as measured on the Windows dev host).
**Why:** the perf gate lands in its own per-OS job (operator P4 fork 1), so CARRY 3 does not fire; the export path is the one `gate.rs` already read.
{REF}
""",
    "security-plan-1": f"""## {M} — test seam `FAKE_AGENT_HOOK_PANIC` and G2's exact-path exemption, ratified by the founder
**Section:** §Input Validation (test-seam rows); §Secret Management → Storage; §Security Anti-Patterns → Universal; §Security Decisions Log (`2026-09-28`, test seam)
**Change:**
- A second test-seam row: `FAKE_AGENT_HOOK_PANIC`, a closed value (panics only on exactly `1`), fixed synthetic payload, fired after `viola_obs_init` so `hook` still fails open; named only in `src/cmd/hook/seam.rs`; compiled out of release builds.
- Storage and Universal: "the one variable / the one carve-out" (`FAKE_AGENT_PUMP_DELAY_MS`) → two test seams; the ban now names another seam or another G2 exemption as needing its own entry.
- New Decisions Log entry: the seam, and G2 (`scripts/g2-zero-panics.sh`) not counting a panic line at exactly `src/cmd/hook/seam.rs:<digits>` (whole-string compare, never prefix, suffix, regex or home path); Conditions: `release-check` `viola only`, G2's `--probe` look-alike and empty-scope controls before every check.
**Why:** a boundary widening (a test build reads a new input; a zero-panics gate admits one location). Ratified by the founder, live: the seam at 06:21 on 2026-09-28, the seam with the G2 exemption (shown to him as new) at 09:52:07; relay the Viola overseer.
{REF}
""",
    "security-plan-2": f"""## {M} — CI integration: the per-OS `perf` job and its scan-gated uploads
**Section:** §Dependency Security → CI integration
**Change:** the `perf` job (`contents: read`) installs hyperfine with `cargo install --locked hyperfine@1.20.0` as its own step (no new action, no `github.event` value), runs `agent-run run --perf`, then G2, G4 and its own `secret-scan`; `perf-<os>` and `diag-perf-<os>` upload only on a successful scan, `secret-scan-perf-<os>` only on a failed one; every input is synthetic.
**Why:** an expected amendment no detector raised (Validate check 5); the uploads are the existing scan-gated, synthetic-input class (the operator's ruling, recorded in the obs sidecar).
{REF}
""",
    "test-plan-1": f"""## {M} — `run --perf` built, `gate` requires four rows, the per-OS `perf` job
**Section:** §2 pyramid (Performance); §3 `run` (`--perf`), `gate` (perf breach, detail codes), Bootstrap `ci-tool-install` (G2 / jq); §9 Pipeline Perf row, tools paragraph, Test report format; §10 Perf run rules (Binary under test, Perf session, Status) and the perf table (`session-end`, `pre-tool-use` rows)
**Change:**
- `--perf` is named-only: probe → `target/perf` release build (`--features viola/fake-agent`) → ONE session `perf-<harness pid>` through the `PerfSession` seam (was two sessions, stamped + unstamped) → four rows (`session-start`, `user-prompt-submit`, `stop`, `session-end`) with synthetic `--input` payloads (was fixture payloads) → zero-panic check → cleanup; suite `perf`, 6 passed on green. The verdict stays with `gate`.
- `gate --require perf` requires each row by name (`artifact-missing` per absent `perf-<hook>.json`; was the single `perf-*.json` breach).
- `pre-tool-use` untimed until the dialog-tier chunk; no async-tier row. Status: built (was "not built yet").
- CI: its own `perf` job (was "not yet in `ci.yml`"); G2 is `scripts/g2-zero-panics.sh` in the `test` and `perf` jobs; the `perf` job's scan-gated uploads join the report inventory, and no perf export lands in `harness-<os>`.
**Why:** the chunk built the arm and the job (operator P4 fork 1); measured: a kept-home run took the channel path for all 132 samples; host max 72.8–73.7 ms.
{REF}
""",
    "test-plan-2": f"""## {M} — forced panic on the real binary, the over-4 KiB concurrent half, the controls table's interim shape
**Section:** §1 Test Scope Summary (hook stdin); §5 Module ↔ DB concurrent-append check, CLI `cli_controls_not_disableable.rs`; §6 Security sweep (the fail-open matrix and its summary)
**Change:**
- `hook_fail_open.rs` gains `hook_forced_panic_fails_open_with_one_role_line_and_one_detail_line` and `hook_panics_append_whole_lines_over_4_kib_side_by_side` (was "11 cases and no forced panic", the forced panic pending); the seam `src/cmd/hook/seam.rs` carries its Decisions Log entry and is a row of the controls table.
- The concurrent-append check's over-4 KiB half landed: 8 forced panics, 8 whole role lines and 8 whole detail lines over 4 096 B, 3 OSes.
- `cli_controls_not_disableable.rs` as landed: the `FAKE_AGENT_HOOK_PANIC` rows `0` · `false` · `off` · empty × the oversize-stdin and malformed-json refusals, no panic line; the verb negatives and the completeness case join with `send` / `answer`.
**Why:** measured green at implement and in CI `test` on three OSes (ci#36390764600); the forced-panic case took 0.414 s on the Windows debug build; operator P4 fork 3 set the interim shape.
{REF}
""",
    "obs-plan-1": f"""## {M} — the `perf` job's uploads: the existing scan-gated, synthetic-input class
**Section:** §8 PII Scrubbing item 6 (Detail-file upload, Scan failure); §9 Telemetry artifact handling (the hyperfine row); §9 Step order, step 1
**Change:**
- The detail-file upload covers `diag-<os>` and the `perf` job's `diag-perf-<os>`; the `perf` job's inputs are synthetic (string-field payloads with the tests-owned canary, the seam's fixed text in `detail-hook.ndjson` only).
- The hyperfine exports are `target/agent-run/artifacts/perf-<hook>.json` (was `perf/*.json`), uploaded as `perf-<os>` only on a successful scan (was a bare `if: always()`); a failed scan withholds both and uploads `secret-scan-perf-<os>`.
- Step 1: perf no longer runs in the `test` job but in its own per-OS `perf` job with its own G2, G4 and scan (was "same per-OS job; a separate job would upload no diagnostics or collide").
**Why:** escalated (D-obs-pii) and resolved by the operator (overseer): the same channel, scan gate and synthetic-input basis as the ratified `diag-<os>`, only a second artifact name — no new crossing.
{REF}
""",
    "obs-plan-2": f"""## {M} — G2's exact-path seam exemption, the D-28 over-4 KiB half, the perf rows built
**Section:** §3 Logging stack (the detail files, D-28); §9 G2 and its snippet; §10 Always-required invariant (Counting rule), Performance budgets (status, `session-end` and spine rows), error budget Definition
**Change:**
- G2 runs as `scripts/g2-zero-panics.sh` (`--probe` first, `test` and `perf` jobs) and does not count a panic line whose `panic_location` is exactly `src/cmd/hook/seam.rs:<digits>`; the snippet carries the exemption; the invariant still holds for the seam's line (written exactly once), only its count is exempt; the error budget's "0 panic lines" reads as G2 counts them.
- D-28: the over-4 KiB half landed (8 concurrent forced panics, 3 OSes).
- §10: status built; rows judged by `gate --require perf` on `target/agent-run/artifacts/perf-<hook>.json` (was `jq -e … perf/hook-<event>.json`); `pre-tool-use` untimed until the dialog tier.
**Why:** the G2 exemption is the one the founder ratified live on 2026-09-28 at 09:52:07 (relay the Viola overseer; security-plan Decisions Log); the rest was measured at implement and in CI.
{REF}
""",
}

for name, text in ENTRIES.items():
    body = text.rstrip("\n") + "\n"
    (RUN / f"{name}-entry.md").write_bytes(body.encode("utf-8"))
    (RUN / f"{name}-payload.md").write_bytes(("\n" + body).encode("utf-8"))
    print(name, len(body.encode("utf-8")), "B", body.count("\n") + 1, "payload lines")
