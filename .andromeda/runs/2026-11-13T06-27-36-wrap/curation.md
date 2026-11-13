# Curation — 2026-11-13-verify-readiness-on-the-input-box (wrap 2026-11-13T06-27-36)

Source: this conversation and the report's Decisions & corrections (12 entries weighed).

| # | candidate | filters | score | disposition |
|---|---|---|---|---|
| 1 | `holds_any` also matched modals: a predicate swap drops the modal settle | F4 | 0.6 (measured +0.4, detail +0.2; no other-home signal — architecture decision/verify-box-wait-for-literal now states «or a modal literal») | rejected — exactly 0.6 |
| 2 | vt100 keeps a fed trailing space; an oracle over a fed row is copied from what was fed | pass | 0.8 (real gate failure +0.4 — entries 8, 9, 29 red, `left: "❯ ", right: "❯"`; detail +0.2; no other home +0.2) | **T2 `testing.md`** (test authoring) |
| 3 | a kept box rule's seam is the live prompt row: `--record` refuses identifying prompt text | F4 | 0.6 (now in architecture decision/screen-fixtures-signature-rows-only and security-plan's row) | rejected — exactly 0.6, a master carries it |
| 4 | the fuzz workspace has no gate: compile-check a type change on the pinned nightly, offline, no install, kill deadline | pass | 0.8 (operator's direction +0.4 — his request after the gate block; detail +0.2; no other home +0.2 — the proposer's test-plan row was rejected at Validate) | **T2 `testing.md`** |
| 5 | the newest CLI moved between plan and proof; a version literal is re-read against `mise ls claude`, moved only on the operator's word | pass (F1: additive facet of verification-harness 2026-10-10) | 0.8 (measured +0.4 — entry 30; the operator's ruling +0.4, `inputs#I8`) | **T2 `verification-harness.md`, extended in place** |
| 6 | a live-start cap counts every `claude` process; read versions via `mise ls`, never by running the binary | F3 | — | **conflict — user review**: the standing verification-harness 2026-10-10 entry says «reads `claude --version` by path before its first start»; new: «never run the binary for a version under a cap». To the handoff. |
| 7 | `gate.py respell` refuses a bare root; respell such a line by hand to `.` | F2/F4 | 0.4 (one event, tool quirk, task-specific −0.2) | rejected |
| 8 | `rec.py render --section` refused a listed heading; read by `--page` | scope | — | rejected — tool telemetry (an evolve record), not a learning |
| 9 | a sweep pattern's zero is about its wording (advisories missed three rows) | F1 | — | duplicate of the letter (amendment-flow §Tool: «a zero-row pattern is a statement about that pattern») |
| 10 | Bash guard refusals (`cd`, pipes into filters) | F4 | — | rejected — the guard is the home |
| 11 | the operator's directions for this implement and pass | F2 | — | task-specific |
| 12 | the founder's words (cap, F1, F2) | scope | — | rejected — recorded in `inputs#I7`, `inputs#I9` and the masters |

Writes: T1 0 · T2 3 (testing.md 2 new; verification-harness.md 1 extended: `Extended 2026-11-13` on the 2026-10-10 entry) · T3 0. Filtered 9 (duplicate 1 · task-specific 1 · confidence 4 · scope/tool 2 · conflict 1 → handoff).
Load-bearing: none. No-other-home: #2 «vt100 keeps a fed trailing space», #4 «fuzz workspace compile check».
