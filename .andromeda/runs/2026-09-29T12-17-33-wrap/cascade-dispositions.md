# Cascade dispositions — wrap of 2026-09-29-sideloaded-conpty

**The search:** `cascade.py sweep` over `cascade-patterns.toml` (20 patterns, every control fired over the pre-pass
masters at `fb78ddc9`; `backend-const` = `PTY_BACKEND` dropped — its control never fired, since no master ever named the
const — and controlled by hand: `grep -rn PTY_BACKEND CLAUDE.md .claude/ .andromeda/*.md` → 1 hit, obs-plan D-36's own
"replaces the const" clause). Listing: `sweep-out.txt` (this run dir). Patterns cover the retired claims' wording AND
mechanism: the inbox-only preamble and "ConPTY emits"; the `conpty|openpty` backend set and "hosted in ConPTY"; the
start order without the sideload (`pin_copy › version_gate`, "pinned copy and plugin, version gate", the stale
"strip plan → collision"); the code-bearing list and the re-hash mechanism; the interim-gap set; the fresh /
not-yet-existing home and "never hand-written" convention; the lone 13/200 H2 figure; the out-of-graph audit list;
PATH/CWD DLL resolution; the Windows matrix row; the first-start bounds; the `bin/` layout. 0 rows in the curation
homes and the two judgment bases.

## Master rows
| row | disposition |
|---|---|
| a11y :624 inbox-preamble (edited) | amended this pass (Y1) — the inbox bytes now one of two hosts |
| arch :47 inbox-preamble (new) | this pass's own text ([PTY] as-built) |
| obs :863, :1060 backend-values (new) | this pass (R2) |
| a11y :94 hosts-conpty | §1 verbatim copy — rejected (playbook "Verbatim upstream copy"); "ConPTY/openpty" stays true |
| test-plan :225 pin-then-gate | **amended this pass** — the Critical Path 1 start sequence gains the sideload (routine, dependent of R1) |
| obs :307 pin-then-gate | obs §1 verbatim copy — rejected (playbook "Verbatim scope copy"); §4 wins, amended (R2) |
| arch :93 start-order (edited) | amended this pass (R1; both sites) |
| test-plan :1020, :1205; obs :170, :240, :305, :723, :830, :851; a11y :357 start-order | no change — name the sequence, state no step list (obs :305 is §1) |
| security :114, :38 code-bearing / bin-layout | Threat Model Summary verbatim copy — rejected (playbook "Verbatim upstream copy"); §Data Protection wins, amended (S3) |
| security :207, :229, :509, :731, :747, :751 code-bearing | :207 amended (S5); the rest name the Epoch 6 entry by title — true, no change |
| security :239, :766-769; test-plan :932 code-bearing / interim-gap (new) | this pass |
| arch :23, :99 rehash-bin (edited) | amended this pass (A12, A10) |
| security :269, :533 rehash-bin | :533 amended (S4); :269 is the exe's re-hash — true for the exe, the companions have their own bullet (S3) |
| security :725 interim-gap | the 2026-09-28 Log entry's title — history, no change |
| test-plan :747 fresh-home | "every test gets a fresh home" — still true (seeded homes are still per-test fresh); no change |
| test-plan :932, :1353 fresh-home / hand-written (edited) | amended this pass (T4, T6) |
| test-plan :1735 fresh-home | inside §12 Test Decisions Log (the `2026-09-24` overseer fix-pass entry, :1711) — history, never modified (an edit made in this pass was reverted); the ruling lands as a new `2026-09-29` Log entry instead |
| test-plan :1174 | a verify scenario's fresh home — true, no change |
| test-plan :1295 | a named Windows test with a not-yet-existing `--home` under `RUNNER_TEMP` — a harness-style start, not a seeded one; true, no change |
| obs :583 fresh-home | "when viola creates the home (a not-yet-existing `--home`, as the tests harness passes)" — the harness never seeds; true, no change |
| arch :47, test-plan :928 h2-13 (edited) | amended this pass (A7, T1) — 13/200 now stands beside the pair |
| security :313 out-of-graph | the npm graph's own sentence — true; the vendored binaries got their own bullet (S7) |
| security :396 audit-list (edited) | amended this pass (S10) |
| security :239, :585, :765 path-dll (new) | this pass |
| security :587 path-dll | exec-form `command` PATH ban — a different claim, true, no change |
| test-plan :1461 matrix-conpty | "windows-2025 (ConPTY, …)" — still true (both ConPTYs); no change |
| obs :1142 first-start | the hook perf spine bound — a different claim, no change |
| arch :372, :388, :458 bin-layout | :388 gained its sibling row (A1); :458 amended (A4); :372 is the hook exec path — true |
| security :263, :268, :388-390, :531, :587, :614; test-plan :1296 bin-layout | the pinned exe's mode, path and re-hash — true for the exe; no change |

## Leaf rows (step 3 re-derived — every leaf of the five changed masters enumerated by provenance)
| leaf | disposition |
|---|---|
| `.claude/docs/stack.md` :13, :19, :42 | re-derived (PTY row, content hash, audit) |
| `.claude/docs/services/viola-pty.md` :6, :22, :30 | re-derived (sideload module, `pty_backend()` values, the H2 pair, DA1 preamble, entry point) |
| `.claude/docs/services/viola.md` :3, :20 | re-derived (start order with the sideload step; the `main` restriction) |
| `.claude/docs/services/viola-state.md` :20, :22 | re-derived (third interim gap; the `pin` companions bullet); :20 exe mode true |
| `.claude/docs/gotchas.md` H2 | re-derived (the pair) + a new DA1 gotcha from arch [PTY] |
| `.claude/docs/security-summary.md` :18, :36 | re-derived (third gap, companions, the planting threat, the vendored audit); :36 PATH ban true |
| `.claude/docs/tests-summary.md` :20, :25 | :20 re-derived (the carve-out); :25 Path 1 names no step list |
| `.claude/rules/testing.md` :34 | re-derived (the carve-out) |
| `.claude/rules/verification-harness.md` :28 | harness homes never seed — true, no change |
| `.claude/rules/security.md` :16, :30 | re-derived (third gap, the DLL rule, the vendored audit); :16 exe mode and :30 fuzz sentence true |
| `.claude/docs/commands.md` | re-derived (the `conpty-vendor.sh` line) |
| `CLAUDE.md` GENERATED:setup:modules | re-derived (viola-pty `sideload`); overview · warnings · pointer-table · architecture recomputed from arch — no other change (the warnings' "human always wins" already covers the silent degrade) |
| `.claude/docs/obs-summary.md`, `a11y-summary.md`, `.claude/rules/{observability,a11y,events}.md` | read — no line states a retired claim (the obs summary lists only selected decisions; D-36 is not one of them) |
