# a11y-plan — amendments

## 2026-09-24-supply-chain-and-workflow-gates — Platform: ci.yml is the one push/PR workflow
**Section:** §9 Pipeline integration → Platform
**Change:** "one workflow `ci.yml`" now reads "one push/PR workflow `ci.yml`". The scheduled `nightly.yml` carries no a11y step, and the E2E/a11y leg stays in `ci.yml`.

**Why:** chunk 2026-09-24-supply-chain-and-workflow-gates added `nightly.yml`, the weekly `cargo deny check advisories` run. The orchestrator raised this site because the same workflow-count grep hit it; no detector proposed it. Sweep: see architecture-amendments.md, same entry heading. For this master, :1108 was amended; the other "all three OS legs" hits were left unchanged as unrelated.

## 2026-09-24-diagnostics-plane — stale a11y-violation resolved-question bullet
**Section:** §12 A11y Decisions Log → Resolved questions
**Change:** The `a11y-violation` bullet now states what Z7 / D-A11Y-09 decided: it is not a product `event` value. It is a harness-only row validated by the tests-owned `e2e-web/schemas/a11y-row.v1.json`, and the shipped `ObsEvent` and `diag-line.v1.json` enum (19 values) carry none.
**Why:** chunk 2026-09-24-diagnostics-plane, report Spec claims disproved #3 (contract test green). The D-a11y-obs-schema proposal was applied as routine.

Sweep: `accepted as a tests \+ obs enum` over all seven masters gives 1 remaining hit, a11y :1389. It is a Decisions-Log history entry, left unchanged. The §3 wording it names ("new closed-enum value") has 0 hits outside that entry.

## 2026-09-24-quality-gates — CLI output-discipline evidence reports under the CI `coverage` suite
**Section:** §1 (CLI output discipline bullet) · §1 cli surface (Automated tool reach) · §9 Per-pipeline-stage table (Unit / integration row) · §10 Standard+ invariants
**Change:** the CLI output-discipline tests still run on all three OS legs. Locally they report under `nextest-integration` / `nextest-e2e`; in CI they report inside the per-OS `test` job's single instrumented `coverage` suite, gated by `gate --require coverage,doctest`. The run JSON `suites[]` enum in the table row gains `coverage`.
**Why:** chunk 2026-09-24-quality-gates replaced the CI unit and integration runs with one `run --coverage` (report Harness / gate surface). Raised by the orchestrator at Validate: the a11y detector flagged it as outside both D-a11y invariants.
**Sweep:** `nextest-integration|nextest-e2e` 4 hits, all amended (`:280, :499, :1115, :1153`). `:604` ("failures surface only as nextest failures in `suites[].failures[]`") stays true under `coverage`, no change. `.claude/docs/a11y-summary.md` and `.claude/rules/a11y.md`: 0 hits, no change.

## 2026-09-25-pty-wrapper-on-windows — Windows zero-viola-bytes oracle is literal absence
**Section:** §3 A11y Assertion Harness Contract → Keyboard test harness → Tooling · §6 State color tokens → CLI equivalent
**Change:** the zero-viola-bytes clause means viola's own literals absent on every leg; "no SGR or cursor control the child did not emit" (byte-for-byte against an unwrapped run) holds on Linux and macOS only. On Windows ConPTY emits its own `ESC[?9001h ESC[?1004h ESC[?25l ESC[2J ESC[m ESC[H`, an OSC 0 title and `ESC[?25h` on every spawn and re-renders nested output, so the windows-2025 `viola run` check is literal absence; §6 "zero SGR under `viola run`" becomes zero SGR of viola's own.
**Why:** chunk 2026-09-25-pty-wrapper-on-windows report Spec claims disproved 4 (research fact 4, measured on the Windows host), expected amendment 7. Rejected (E4): the §1 edits at :98 and :179 — §1 is verbatim from a11y-scope.md per D-A11Y-15 and those lines are not deferral clauses; §3 and §6 carry the truth. Not amended (E3): "ConPTY swallows focus reports" — no measured basis in the chunk's artifacts; carried as a labelled HYPOTHESIS on the route entry that builds focus/mouse-sequence handling.
**Sweep:** `no SGR or cursor control|zero SGR|byte for byte|viola-originated` over the 7 masters: a11y :624, :947 amended; :98, :179 (§1 verbatim) no change per E4; no hit in the other six. Leaves: `.claude/docs/a11y-summary.md`, `.claude/rules/a11y.md` recomputed — neither states the zero-bytes oracle: no change.
