# Founder forks — wrap 2026-11-16T03-24-03-wrap (chunk 2026-11-16-wait-wake-condition)

The founder is away (inputs#I16 item 1). Each fork below was resolved by the delegate's standing direction,
PROVISIONAL, the most conservative option; nothing is ratified.

## Q1 — security-plan decision/test-removal-own-dirs-only (escalated by rule «Boundary widening»)
The ban lists how a test-side removal under the e2e home knows a directory is its own: by its owner record, its own
temp dir or its session name prefix. This chunk's stamped-home fixture also removes ANOTHER build's
`viola-stamp-<key>` dir under `target/e2e-home`, only when it takes that stamp's lock with `try_lock` (never a stamp a
running build holds). The dirs removed are still dirs the test side itself made.
- 1. (Recommended — taken, PROVISIONAL) Amend the list with the fourth way (another build's stamp dir whose lock it
  takes with try_lock); the row carries `state: provisional` until your word.
- 2. Keep the list as it stood and have the fixture stop sweeping other builds' stamps (a code change, a later
  chunk; per-build stamps then pile up on the tmpfs, about 96 MB each).
- 3. Keep the list and accept the sweep as unlisted (the master then under-states the code).

## Standing forks carried, not re-asked
- F1 (the remove-the-guard reading of «a test that fails before the fix», cap v2-17) — option 1 by the delegate,
  PROVISIONAL: `.andromeda/runs/2026-11-16T00-29-34-phase/founder-forks.md`.
- `session_crons` carried as a count beside `background_tasks` (F1 option 2) — a candidate boundary widening, his to
  give; no route entry minted.
