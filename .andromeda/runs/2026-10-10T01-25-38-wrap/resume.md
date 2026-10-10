# Wrap resume point — 2026-10-09-epoch-3-cleanup-ii

Written at 2026-10-10T01:29Z, at the end of Phase 1, on the operator's word (`inputs#I4`, `relay-1.md` in this run
dir): this window ran Setup and P1 only. **NEXT IS PHASE 2 (Reconcile docs).** The wrap is resumed in a cleared
window; the operator gives its route items there.

## How to resume

- Setup step 2a will find `viola-0.1.0/chunks/2026-10-09-epoch-3-cleanup-ii/report.md` and HALT with its offer.
  The operator's answer is already given: **resume, in this run dir** (`.andromeda/runs/2026-10-10T01-25-38-wrap/`).
  The report is reused as it stands. This dir holds no `fanout-results.md`, so P2 starts from its first step: the
  citation sweep (`cites.py apply --dry-run`, then without), then the fan-out.
- No spec master and no sidecar was touched (`git status --short -- .andromeda/` shows only run-dir files and the
  friction ledger). P2's apply has not begun.
- Setup step 7's code-graph refresh was **not fired** in this window. The resumed Setup fires it.
- A resume re-fires P1's scope read and both new-text calls (`cites.py added`, then the `splice.py` paste):
  `unchanged` and `skipped` are the expected words. If the listing differs, source moved: HALT.

## What Phase 1 did (all in this run dir's trails)

- Chunk detected: one `pending` record, `2026-10-09-epoch-3-cleanup-ii`. The operator pass ran: the base is
  `14f1fb5f588d`, the parent of the one pre-CI commit `632f6a7edf29` (pushed; 0 ahead; CI `ci#38012420489` green,
  15 of 15, `run_attempt` 1).
- Scope read: `scope: clean — changed 20 · listed 20 · recorded 0`. No `scope-record.md` exists and none is needed.
- Inputs: the wrap directive snapped as `I4`; `inputs.py verify` after the report: 4 entries, n/a 4, uncited 0.
- New-text listing: `new-text.md` (89 lines, 44 blocks, 20 files), pasted as the report's last section by
  `splice.py` (`report.md` lines 274 to 362). No line of that section was edited. One line ABOVE it was corrected
  after the paste (the evidence file count, 16 → 15).
- Report authored: `viola-0.1.0/chunks/2026-10-09-epoch-3-cleanup-ii/report.md`.
- Telemetry: the `report` step record and one friction record (2026-10-10T01:28:39Z, ids a and b).

## What P2 will meet (from the report)

- **Three expected amendments, all carried**, each with its site search in the report:
  - test-plan §2: the helper-file list at `test-plan.md:456` names `events.rs` and not `cli.rs`.
  - test-plan §10 Mutation gate: the first measured whole-unit `viola-e2e` score (718 mutants, 5356 s; 656 / 2 / 0 /
    60). "78 min" reads 0 in the seven master bodies and 1 each in two test-plan key files under
    `.andromeda/registries/contracts/test-plan/`.
  - architecture §Occupied Resources: `architecture.md:407` says `WITHIN` is "used by the 9 root waits on a
    child". The report measured `Instant::now() + WITHIN` sites, 30 → 22, and did **not** re-derive the master's
    9. Before amending that number, read how it was counted (the sidecar entry that wrote it).
- **No spec claim was measured false in a master.** Spec-master edits: none. Dependencies, schema, harness: none.
- The intent reference for Validate: the frozen working-route line (`viola-0.1.0/working-route.md:109`) and the
  plan's 21 acceptance criteria, all re-asserted met in the report's Outcome.

## For P3 (curation) — the conversation will be gone

P3 is conversation-sourced and this window's conversation does not survive the clear. Its raw material is the
report's **Decisions & corrections** section, which was written for that: the operator's two words (`I3`, `I4`), the
findings (cargo-mutants keeps one earlier run's per-mutant logs; a mutation run of `viola-e2e` leaves session homes
on the shared tmpfs base; a formatter width above its range panics at run time), the two sweep hazards over
cargo-mutants' outcome lines, and two recurrences of carried learnings (a time written ahead of the clock; a
heredoc with a file target). Anything not in that section was not carried.

## For P5 (route-resolve) — the operator gives the route items in the resumed window

What the chunk's outcome puts on the table, for the operator's items to be read against:

- Four root mutants graded `caught` by the `mutants` profile's 10 s kill alone, no failing assertion, on lines this
  chunk did not edit: `src/bin/viola-fake-agent.rs:378:16`, `:378:38`, `:97:17`; `src/cmd/verify.rs:284:9`. No
  route owner yet (`evidence/survivors.md`, its last-but-one section).
- The two `viola-e2e` mutants the score read missed (`harness/run/mutants/scratch.rs:48:5`, `:54:8`), reachable only
  on a Windows host: "owed to Windows mutation grade", the next markerless entry.
- Standing from before this chunk and untouched by it: the six `replace_private_with` rows and the `viola-e2e`
  Windows grade, also "Windows mutation grade"'s.
- No gate was deferred, so no `PREREQ` is owed. No `watch:` was folded.

## Operator desk (left on disk by the implement run, not removed)

- 34 `.tmp*` directories in `<repo parent>/viola-mutants-scratch` (250 MB).
- 26 `viola-session-*` homes of the score run on the tmpfs behind `target/e2e-home` (gone at a reboot).
- `mutants.out/` and `mutants.out.old/` at the repository root (ignored by git).
- The four raw mutation-run stderr captures, in the implement session's scratchpad (`raw-stderr/`).

## Tree at this point

- HEAD `632f6a7edf29` = upstream. Uncommitted, all for this wrap's P7 commit: `report.md`; three evidence files
  completed after the pre-CI commit (`operator-pass.md`, `survivors.md`, `e2e-score.md`); the chunk's
  `inputs/` (the `I4` copy and the manifest); the implement run dir's journal and gate trail; this run dir;
  `.andromeda/friction-log.ndjson`.
- The master record is still `pending`. Nothing was flipped, committed or pushed by this wrap.
