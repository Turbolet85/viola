# Adaptation record — 0-pending wrap, Epoch 2 boundary (2026-09-27)

Path: Setup step 6 no-op path (0 pending; tree dirty only with bookkeeping + untracked run dirs) with P5
route-resolve on the operator's route-adaptation request.

Request: `D:/dev/projects/additional/viola-overseer/e2-route-adaptation.md` (overseer, founder-delegated; item B
is the founder's ruling W125). Triage: `D:/dev/projects/additional/viola-overseer/epoch2-triage.md`. Sources:
`.andromeda/runs/2026-09-27T13-31-25-evolve-diagnose/proposals.md`, `.andromeda/runs/2026-09-27T13-39-34-code-audit/proposals.md`.

## Dialogue (one round, answered by the overseer)
- A scope → all relay items, one chunk, per the standing rule. Overseer sizing note for its P5: the plan states
  the diff mutant count and the estimated pre-push minutes; above ~60 min it names which splits to defer, as a
  decision for the operator.
- B position → after the cleanup, before Hooks to normalised events (pipeline infra, so the anchor "Hooks stays
  Epoch 2b's first FEATURE entry" holds).
- B pre-push half → rides the reachability chunk (one set of pins for the CI and WSL legs).

## Item dispositions (A — the Epoch 2 cleanup chunk)
| Item | Measured here | Disposition |
|---|---|---|
| Split 4 files below 800 | audit LOC 1335/1074/1029/869 (`c-sizes.json`); `wc -l` 1373/1218/1155/1025 | in scope |
| Clone dedupe: tracing capture layer; 2 endpoint-fixture pairs | per audit | in scope |
| Killing tests: channel 4, state 2 survivors | per audit | in scope |
| pty 5 survivors (`HostTerminal::enter` / `host_size`) | per audit | in scope: check the ubuntu leg first, killing tests only if not |
| Windows mutation scratch off `%TEMP%` (C:) to a wiped D: dir, counted in the cache report | ~30 GB per run, per triage (15:26 green finish) | in scope (harness code); leaked-fake-agent cause carried as a hypothesis |
| Evolve P3 / P7 / P10+L4 / P11 harness aids | no `scripts/wsl-exec.sh` today | in scope |
| cargo-machete `proc-macro2` false positive | machete runs only in the code audit; fix = `[package.metadata.cargo-machete] ignored` | in scope |
| cargo-mutants 27.1.0 facts → testing.md | a rules edit, not code | elsewhere: the cleanup chunk's own wrap curation |
| 2 recurrence watches (viola-pty hang, concurrent boot) | CARRY on the head entry | moved to the cleanup chunk; forced-window repro first; 3-green expiry kept |

## Item B — Browser verdict reachability
In, by founder ruling W125. Position: second entry of Epoch 2b. Its pre-push Playwright leg rides it. State
recorded on the entry: no `e2e-web/`, no tracked `package.json` / lockfile / `tsconfig.json` (TS code-graph plane
not yet detected). The full web harness and the a11y verdict gate keep their Epoch 8 positions.

## Route diff
`viola-0.1.0/working-route.md` +5/−1: two entries + two separators inserted under `### Epoch 2b — …`; the
recurrence-watch CARRY moved off Hooks to normalised events. Epoch 2 header untouched; no frozen line touched;
no PREREQ existed to re-pin. `route.py cursor`: next = working-route.md:43 · Epoch 2 cleanup; pins clean
(one INDETERMINATE on authored `P11:` prose fixed by rewording).
