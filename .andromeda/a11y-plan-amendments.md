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

## 2026-09-27-browser-verdict-reachability — the browser pipe on three OSes, the a11y verdict ubuntu-judged, axe pin deferred
**Section:** §3 (CI integration Runner; the a11y harness Command; Bootstrap phases `a11y-tooling-install`) · §9 (the layer table's Unit/integration and E2E rows; Pipeline integration; the ubuntu-only sentence) · §11 (CI anti-pattern) · §12 (D-A11Y-12)
**Change:**
- `run --browser` runs the locked Playwright CLI (`node node_modules/@playwright/test/cli.js test` after `npm ci`; was `npx --prefix e2e-web playwright test`) on all three legs of the `test` job (was the ubuntu leg only, `browser-linux-only` elsewhere); the a11y verdict stays judged on the ubuntu leg only. `browser-missing` covers a failed `npm ci` as well as a missing Chromium.
- The test job's gate is `coverage,doctest,playwright` (was `coverage,doctest` / `playwright`).
- `a11y-tooling-install` adds `@axe-core/playwright@4.13.0` itself (was "already declared by tests"): tests declared only `@playwright/test@1.63.0`.
- §11 bans judging the a11y verdict on Windows or macOS (was: running `--browser` there). D-A11Y-12's reason is the ubuntu-judged verdict, not an ubuntu-only `--browser`.
**Why:** founder ruling W125 put the browser pipe on all three CI OSes before the a11y harness lands on it (chunk 2026-09-27-browser-verdict-reachability); the axe pin was deferred to Epoch 8.
**Kept:** §1 (the verbatim a11y-scope copy) and the §12 key-decision history ("already declared") stand as written.
**Ref:** .andromeda/runs/2026-09-27T19-50-23-wrap/
