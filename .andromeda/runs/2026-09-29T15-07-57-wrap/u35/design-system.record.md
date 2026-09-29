## 2026-09-29-t15-07-57-wrap — registry migration (U35): the design-system Decisions Log leaves the body
**Section:** §Design Decisions Log · §Brand Identity · §Color Palette · §Surface: web-spa (Component Patterns 1 and 7; Navigation Pattern) · §Self-Validation Protocol (Squint Test)
**Change:** the log moved verbatim to design-system-amendments-archive.md (10 entries, all 2026-09-24). Each lift:
- §Brand Identity — the library shortlist direction (Precision & Density + Utility & Function; Minimalism & Swiss Style #1 without its hover, E-Ink / Paper #56 surface reference; rejected presets #7, #31, #51).
- §Color Palette — a second deviation note: holder colour marks kind, with the two contrast-forced state exceptions (`stale` fill, cocked inset DIALOG cell) and how kind stays readable without the fill.
- §Surface: web-spa component 1 — a grid `<tr>` still exposes `row`; the aria snapshot asserts `table` / `row` / `cell` in every state.
- §Surface: web-spa component 7 — scope: "view-only" / "never on this page" are v1 statements; the v1.x reservations do not contradict them.
- §Surface: web-spa Navigation Pattern — content jumping (UX guideline 5, #19) is met for order, not pixel position.
- §Self-Validation Protocol Squint Test — `open` and `unconfirmable` look identical by design; only the word cell tells them apart.
**Why:** a Decisions Log is keyed by time — history, not current truth; its in-force items now stand in the body
**Ref:** .andromeda/runs/2026-09-29T15-07-57-wrap/
