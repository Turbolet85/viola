# a11y-plan — amendments

## 2026-09-24-supply-chain-and-workflow-gates — Platform: ci.yml is the one push/PR workflow
**Section:** §9 Pipeline integration → Platform
**Change:** was "one workflow `ci.yml`"; now "one push/PR workflow `ci.yml`". The scheduled `nightly.yml` carries no a11y step; the E2E/a11y leg stays in `ci.yml`.
**Why:** the chunk added `nightly.yml`, the weekly `cargo deny check advisories` run, so `ci.yml` is no longer the only workflow; raised by the orchestrator, not by a detector.
**Ref:** .andromeda/runs/2026-09-24T09-41-13-wrap/

## 2026-09-24-diagnostics-plane — stale a11y-violation resolved-question bullet
**Section:** §12 A11y Decisions Log → Resolved questions
**Change:** the `a11y-violation` bullet now states what Z7 / D-A11Y-09 decided: it is not a product `event` value. It is a harness-only row validated by the tests-owned `e2e-web/schemas/a11y-row.v1.json`, and the shipped `ObsEvent` and `diag-line.v1.json` enum (19 values) carry none. Retired: "accepted as a tests + obs enum" / a "new closed-enum value".
**Why:** the chunk's contract test (green) disproved the stale claim; the D-a11y-obs-schema proposal was applied as routine.
**Kept:** the Decisions-Log history entry still reading "accepted as a tests + obs enum" stays unchanged, as history.
**Ref:** .andromeda/runs/2026-09-24T10-40-06-wrap/

## 2026-09-24-quality-gates — CLI output-discipline evidence reports under the CI `coverage` suite
**Section:** §1 (CLI output discipline bullet) · §1 cli surface (Automated tool reach) · §9 Per-pipeline-stage table (Unit / integration row) · §10 Standard+ invariants
**Change:** the CLI output-discipline tests still run on all three OS legs. Locally they report under `nextest-integration` / `nextest-e2e`; in CI they report inside the per-OS `test` job's single instrumented `coverage` suite, gated by `gate --require coverage,doctest`. The run JSON `suites[]` enum in the table row gains `coverage`.
**Why:** the chunk replaced the CI unit and integration runs with one `run --coverage`; raised by the orchestrator at Validate, since the a11y detector flagged it as outside both D-a11y invariants.
**Kept:** "failures surface only as nextest failures in `suites[].failures[]`" stays true under `coverage`, unchanged.
**Ref:** .andromeda/runs/2026-09-24T14-48-15-wrap/

## 2026-09-25-pty-wrapper-on-windows — Windows zero-viola-bytes oracle is literal absence
**Section:** §3 A11y Assertion Harness Contract → Keyboard test harness → Tooling · §6 State color tokens → CLI equivalent
**Change:** the zero-viola-bytes clause means viola's own literals absent on every leg; "no SGR or cursor control the child did not emit" (byte-for-byte against an unwrapped run) holds on Linux and macOS only. On Windows ConPTY emits its own `ESC[?9001h ESC[?1004h ESC[?25l ESC[2J ESC[m ESC[H`, an OSC 0 title and `ESC[?25h` on every spawn and re-renders nested output, so the windows-2025 `viola run` check is literal absence. §6 was "zero SGR under `viola run`"; now zero SGR of viola's own.
**Why:** measured on the Windows host: ConPTY's own sequences disproved the byte-for-byte oracle on Windows.
**Kept:** §1 stays unedited — it is verbatim from a11y-scope.md per D-A11Y-15 and its lines are not deferral clauses; §3 and §6 carry the truth. "ConPTY swallows focus reports" is not amended — no measured basis; it is carried as a labelled HYPOTHESIS on the route entry that builds focus/mouse-sequence handling.
**Ref:** .andromeda/runs/2026-09-25T17-43-18-wrap/
