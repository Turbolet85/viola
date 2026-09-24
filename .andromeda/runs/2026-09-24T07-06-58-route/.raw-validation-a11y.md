# A11y validation — route draft

## Insert
- Between `Linux and macOS parity` and `Version done-check`: **"Web a11y CI gate — lit-a11y and html-validate lint step, per-SC coverage report with budgets, always-on upload of scrubbed violation NDJSON on the Linux leg"** (epoch: `Epoch 8 — Polish & ship (cross-OS completion)`)
  Reason: per a11y-plan §3 Bootstrap phases → a11y-ci-gate-wire and §9 Aggregation, no chunk wires the `ci.yml` lint stage, `sc-coverage.json`/`sc-coverage-report.json` budget step or `if: always()` artifact upload; it lands in Polish with the tests CI, after the contrast harness.

## Reorder
- Move `Contrast, forced colours, reduced motion` before `A11y verdict across page states`
  Reason: per a11y-plan §3 Contrast verification harness (contrast-verification-harness-setup), the token-pair and forced-colours checks provide the SC 1.4.3/1.4.11/2.3.1 evidence. The verdict chunk's claim that every applicable SC is tagged cannot pass until those checks exist.

## Rewrite
- `Web test toolchain and a11y harness`: "keyboard oracle, virtual screen reader, template lint, violation rows (per a11y-plan §3)" → "keyboard oracle, virtual screen reader, SR-pass template, template lint, scrubbed identity-tagged violation rows"
  Reason: per a11y-plan §3 Bootstrap phases, screen-reader-test-spec-setup also covers the `a11y/sr-pass/TEMPLATE.json` founder pass spec, and violation-json-emission-wire needs rows carrying `service_name`/`version`/`os` and passed through the scrubber.
- `Strip-bay live page`: whole line → "Strip-bay live page — ATIS header, cocked strip, transfer markers, silent tape with readback box, title and status-announcer attention, 503/TAPE-stopped strips, viola list parity"
  Reason: per a11y-plan §4 P5 and the State strips row, no feature chunk builds the polite `role="status"` announcer or the 503/404/405 and `TAPE stopped` strips, so path P5 has nothing to verify except the 401 strip.
- `A11y verdict across page states`: whole line → "A11y verdict across page states — zero violations per web-spa state, keyboard walk equals oracle, announcements once, no focus theft, WCAG 2.1 AA + SC 2.4.11/2.5.8 tagged"
  Reason: per a11y-plan §3 WCAG criteria mapping, the verification chunk must name the conformance label and its SC scope, and per §5 Focus restoration the `focus-lost` / no-theft invariant must be stated.
- `Contrast, forced colours, reduced motion`: "zero-motion mode" → "zero-motion mode (SC 1.4.3, 1.4.11, 2.3.1)"
  Reason: per a11y-plan §6 and the §3 per-SC map, the verification chunk must name the SC IDs it covers.
- `The wheel`: "harness turns ignored" → "harness turns and focus/mouse/resize sequences ignored"
  Reason: per a11y-plan §4 P4, tui boundary clause (3) requires that `\x1b[I`/`\x1b[O`, mouse and resize sequences never move the wheel, and no chunk states this.
- `Version done-check`: "fix-pass items honoured," → "fix-pass items honoured, founder NVDA/VoiceOver/Orca pass recorded on paths P1–P6,"
  Reason: per a11y-plan §3 Screen reader test pattern, a supplemental founder-owned manual pass on every §4 path is expected (it never gates), and no chunk records it.
