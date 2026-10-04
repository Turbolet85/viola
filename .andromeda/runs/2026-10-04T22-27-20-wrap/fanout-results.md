# Fan-out results — 2026-10-04-running-turn-refusal

Seven Explore doc-agents, one parallel batch, the amendment-flow prompt sent verbatim (15 detectors: arch 2 ·
security-plan 3 · design-system 1 · layout-templates 1 · test-plan 3 · obs-plan 3 · a11y-plan 2 = the drift-base's 15
`doc:` names). Stripping removed only `#` commentary lines after each YAML list (their substance below). Entity
probe: the test-plan return carried the harness's neutralised `<\` (for `<`) inside step 8's change text; the applied
text is re-derived from the doc, never pasted. No raw twin warranted (no `proposals: []` return was changed by
stripping, no parse failure).

## Verdicts

- **architecture** — 2 proposals (D-arch-decisions ×2). Commentary: D-arch-resources no drift (no new IPC / param /
  event / detail / snapshot field / env / flag / seam; `turn-running` registered at `:133` / `:136`, `:136` already
  names both causes); `:84` (the open "Turn end without Stop" ledger item) stays true; no fake-agent claim in arch.
- **security-plan** — `proposals: []`. Commentary: no new external-input surface; `release` param checks unchanged;
  `:206` driver-held no-op still holds; the closed seam list (`:233-234`, `:607`) untouched; no deps.
- **design-system** — `proposals: []`. Commentary: every new surface `tokens n/a`; `:163` · `:572` · `:762` · `:820`
  stay true.
- **layout-templates** — `proposals: []`. Commentary: no new surface; `:522` ("No hint names `viola release`") holds.
- **test-plan** — 7 proposals (D-tests-coverage ×6, D-tests-obs-harness ×1). Commentary: D-tests-framework no drift;
  `:587` refusal ordering agrees.
- **obs-plan** — 1 proposal (D-obs-instrumentation). Commentary: D-obs-stack, D-obs-pii no drift; `:314`, `:836` list
  the detail only and stay true; the §4 Human-takes-the-wheel scenario (`:680-689`) is not contradicted.
- **a11y-plan** — `proposals: []`. Commentary: no interactive UI element; schemas unchanged; the fake-agent mentions
  (`:573`, Keyboard test harness → Tooling) concern byte comparison, untouched.

## Proposals and dispositions

### architecture
1. D-arch-decisions · warning · Established Decisions → [Human Takeover / Wheel] · retire "as built, no running-turn
   state exists beyond the in-flight `send` slot … so `release` returns the wheel alone"; state the running-turn
   state, its marks, the rung, the release clear, the human-typing reading, the two residuals. basis `architecture.md:70`.
   → **apply** (check 1: playbook "Accurate this-chunk addition" — `Turn`, `WheelSlot::turn_*`, the rung and the
   `apply` clear are in the report's Changes; check 5: the plan's first expected amendment; check 6: disproved claim 1,
   and disproved claim 2's measured fact; text re-derived from the report, not pasted).
2. D-arch-decisions · warning · same section · "(`WheelSlot`: holder and cause under one lock)" → holder, cause and
   the running turn · dependent-of D-arch-decisions · basis `architecture.md:70`.
   → **apply** (check 1 as above; a same-line duplicate of the retired two-field model, applied atomically with 1).

### test-plan
3. D-tests-coverage · §6 Path 5 "As landed" (`:808`) — names the harness-turn `turn-running` refusal and acceptance
   after `turn-ended`, and `cli_wheel::path5_a_turn_left_running_is_cleared_by_pause_then_release`.
   → **apply** (check 1 routine; check 5: expected amendment "test-plan §6 Path 5").
4. D-tests-coverage · §6 Path 5 step 8 (`:817`) — a `send` during the harness turn, then after its scripted Stop ·
   dependent-of 3. → **apply** (atomic with 3).
5. D-tests-coverage · §6 Path 5 verification signal (`:826`) — the refusal shape, receipt, `send-refused` without
   `cursor`, exit 0 after `turn-ended` · dependent-of 3. → **apply** (atomic with 3).
6. D-tests-coverage · §6 Path 2 "As landed" (`:734`) — the `path3.json` boot and turn end between its sends; the
   driver-own-turn `cli_send` case. → **apply** (check 1; check 5: expected amendment "test-plan §6 Path 2").
7. D-tests-coverage · §5 CLI controls "as landed" (`:660`) — the turn-running row.
   → **apply** (check 1; check 5: expected amendment "§5 CLI controls table").
8. D-tests-coverage · §5 CLI controls planned negatives (`:661`) — the list grows to five with `send` during a running
   turn · dependent-of 7. → **apply** (atomic with 7).
9. D-tests-obs-harness · §7 Fake agent (`:1077`) — "It exits on `\x03`." → on `\x03` or stdin EOF, waiting for a
   running hook and starting no other first. → **apply** (check 1: the fact is the report's Harness / gate surface
   bullet, this chunk's recorded widening — test infrastructure, no product boundary, so not the "Boundary widening"
   class; check 3: the widening carries the overseer's founder-delegated word. The detector's own pairing (§3 ↔
   obs §3) is unaffected — no verb, status shape, log format or CI step changed — and the edit is the §7 fact the
   detector surfaced).

### obs-plan
10. D-obs-instrumentation · §4 Scenario: Confirmed `send` (`:644`) — `turn-running` covers a running turn or another
    `send` in flight, same closed detail, no new field; the turn state writes nothing of its own.
    → **apply** (check 1; check 5: expected amendment "obs-plan §4 Scenario: Confirmed `send`").

## Validate — the six checks

1. Playbook — all 10 routine ("Accurate this-chunk addition"); none in the "Boundary widening" class.
2. Cross-contradiction — none (the three docs edit disjoint sections; arch 1 and 2 are one line, applied together).
3. Intent-consistency — the report diverges from the plan's acceptance in one place: the human-origin turn reads
   `human-typing`, not `turn-running` (Deviation 1). Justified (a measured rung order) → the intent is amended: the
   operator's directive (2) amends the plan's "The turn's life" acceptance line as measured. The scope record's one
   line, the `widening` of `src/bin/viola-fake-agent.rs`, carries the overseer's founder-delegated word — the
   justified branch; its `serves` (the folded WATCH, pre-push gate 17) holds.
4. Absence needs evidence — arch `:70` is 6 000+ chars: its two edit points are located by a short unique anchor each
   (`grep -c` = 1) and read in bounded windows before the edit; test-plan and obs-plan sites are short lines read whole.
5. Expected amendments — all 6 entries covered: arch (proposals 1–2) · test-plan Path 5 (3–5) · Path 2 (6) · §5
   controls (7–8) · obs §4 (10) · `matrix#v1-32 notes` → P7.3 ledger note · `matrix#v1-29 notes` → already written at
   phase P5.
6. Disproved claims — (1) arch `:70` "as built" → proposal 1 · (2) the plan's human-origin acceptance → intent
   amendment (directive 2) + arch proposal 1's human-typing sentence · (3) the scope's WATCH hypothesis → P5's
   recurrence-watch row (the WATCH closed, directive 3) and the report's Insufficient-fixes bullet.

Escalations: 0.
