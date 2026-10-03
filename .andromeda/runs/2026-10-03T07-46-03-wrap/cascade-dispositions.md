# Cascade dispositions — 2026-10-02-epoch-2b-cleanup

**Search:** `cascade.py sweep` over `cascade-patterns.toml` (baseline `e0fbc724`, the oldest pre-CI commit's parent). It
covered the seven masters, every `.andromeda/registries/**` file, the three curation homes, the two judgment bases and the
leaf bodies. Every pattern's control fired.
- `tempdir-removes` (fixed `TempDir` drop removes per-test homes): the retired wording.
- `tempdir-keep` (fixed `TempDir::keep()`): the keep path named by its retired mechanism.
- `tempdir-drop` (regex): any restatement that removal is a `TempDir` drop.
- `owner-gone` (regex): every restatement of the owner-gone sweep, re-read for the removal mechanism.

| row | disposition |
|---|---|
| `tempdir-removes` 0 rows, `tempdir-keep` 0 rows | the pass's own edit removed both from `test-data-bootstrap.md:15`; nothing else carried them |
| `test-plan.md:730 · 828 · 875 · 889 · 902 · 945` (tempdir-drop) | **no change**: §6 critical-path cleanup cells for the paths' own homes (H · H2 · H3 · per-path), which are viola-e2e / harness homes, not the root `TestHome` (each line read at Validate check 4) |
| `registries/contracts/test-plan/5-command-implementation.md:33` (@c18/3166) | **no change**: the `viola_e2e::fixtures` chain for Paths 1 · 5 · E1 · E2 · E5, not the root `TestHome` (read) |
| `obs-plan.md:1044` (tempdir-drop) | **amended** (routine, a cross-master citation): "neither harness `cleanup` nor an rstest `TempDir` drop deletes a home" → "… nor a test home's drop deletes a home". The behaviour (CI keeps every home until G2 · G4 · the secret scan · the uploads) is unchanged; only the deleter's name was stale |
| `architecture.md:416` (owner-gone, edited) | **amended** this pass (P-1) |
| `registries/contracts/test-plan/test-data-bootstrap.md:15` (owner-gone, edited) | **amended** this pass (P-2); the line was re-read for an intra-line duplicate: the two `TempDir::keep()` statements were restated as behaviour in the same edit |
| `.claude/rules/verification-harness.md:43` (leaf) | **re-derived**: the clause "the drop and that sweep both go through `remove_owned`, which deletes `owner.json` last" was added; the `## Session Additions` were untouched |
| `.claude/docs/tests-summary.md:20` (leaf) | **re-derived**: "(removal deletes `owner.json` last, `remove_owned`)" was added |

**Leaves re-computed per changed source:**
- architecture §Occupied Resources: CLAUDE.md's GENERATED blocks re-read. No block states the test-home removal (the
  sweep finds no CLAUDE.md hit), so they are unchanged. No docs leaf derives from §Occupied Resources' home entry.
- obs-plan: `obs-summary.md` and `.claude/rules/observability.md` carry no home-removal statement (0 `tempdir-drop` /
  `owner-gone` leaf rows), so they are unchanged.
- test-plan (key file): `tests-summary.md` and `verification-harness.md` were re-derived (above); `testing.md` carries no
  removal statement (0 rows).

**Lateral binds:** test-plan §3 ↔ obs-plan §3: obs carried the one stale deleter name, now aligned. a11y ↔ obs schema:
untouched.

**Not looked for:** the D: remnants' mechanism (not established, so no claim to sweep), and the home DACL wording
(routed to `working-route.md:107`, no amendment).
