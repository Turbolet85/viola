# Outcome — 2026-11-24-viola-adopt, GATED MODE (run 2026-11-25T00-27-47Z-implement)

**green** — block R5 cleared for R5a by `inputs#I9` (the overseer's relay `relay-r5a-gated.md`, the operator's word
it quotes); R5a fired ONCE and read green; R5b not re-fired, its green line of 2026-11-24 standing on the record.

## Files changed
- No source, test, script or plan file (`git status --short`: no source edit; `gate.py scope`: `clean — changed 0`).
- Records only: `.andromeda/cache/chunks/2026-11-24-viola-adopt/evidence/r5.md` (the R5a ledger line, verdict and
  post-reads appended), `evidence/rebase-gated.md` (new), the ledger row `matrix#v2-13` (`ref` +
  `implemented`, notes +1835 chars from `cleared-v2-13.md`), this run's dir.

## Gates
- Gated re-base: take-up base `e7703af0`, gated base `d3cd68f7` (HEAD). No `run` holds `e7703af`. Entry 7 fired at
  the gated base in a shared scratch clone: `  7 integration green · exit 0 · 37.0s · 7872 B → 7.log`, its atoms
  unchanged. Entry 19 (`last line 2`) is `leg = 'operator'`, refused by `--entry`: left to the operator pass. No plan
  edit (`evidence/rebase-gated.md`).
- Entry 20, `--operator 20`: ` 20 build       green · exit 0 · 2.84s · 134 B → 20.log · history: unmoved · cargo
  build -p viola`; `target/debug/viola` sha256 `50f5e60d…`, HEAD `d3cd68f7`.
- Entry 21 (R5a), `--operator 21`, ONCE at 2026-11-25T00:29:58Z: ` 21 e2e         green · exit 0 · 2.47s · 114 B →
  21.log · history: unmoved`; verdict
  `r5a: green resumed=yes lineage=yes pair=haiku/low turn=ended ended=yes start=ok`.
- Entry 22 (R5b): NOT fired; its 2026-11-24T18:34:07Z green line stands in `evidence/r5.md`.
- The block, `gate.py run … --void '14:block cleared — …'`: entries 1–9, 11–13 green; 10 `recorded`; 14 green under
  its void (last line `2`); 15–22 `not run — leg operator`. Summary
  `entries 22 · green 13 · red 0 · recorded 1 · timeout 0 · not-run 8 · voided 1 (14)`.
- No `gate.py guard` entry in the plan.
- Smoke: skipped — no boot-path / UI-surface change (no source edit; R5a itself booted the real start).

## Deviations
- The gated re-base could not fire entry 19 at the gated base (the tool refuses `--entry` on an operator leg); it
  reads a work-dir record, not the tree, and stays the operator pass's as written.
- The block's premise names «unlocked»; `hyprctl locked` read `true` before R5a (as the relay measured). The relay,
  the clearing artifact, cleared R5a with that reading; the script hosts its own pty and opens no window.
- The live verdict line carries a trailing `start=ok`, a field the later chunk «Adopt's resume folder» added beyond
  step 12's closed set; entry 21's atoms are `contains` and held.
- `claude --version` was run once to read the CLI version (no session started).
- Scope record: none — `gate.py scope` clean, 0 recorded.

## Surfaced / stuck
none.

## Process hygiene
| process | started by | final state |
|---|---|---|
| `script` + `target/debug/viola adopt` + its claude child (R5a) | a leg (entry 21, this run) | terminated — `ended=yes`; `gate.py census` after: none of this tree's `target/debug/viola`, no `script` |
| cargo / nextest / fake agent (block, entry 7 re-base) | this run | terminated — census: none |
| 5 `viola` processes listed (`~/.viola/bin/…/viola mcp`, viola-lab `run escher-builder` / `run andromeda-worker` / `wait`, `~/.local/viola-product/bin/viola wait`) | NOT DERIVED — not this run's (other trees and the product) | left running: not ours |

## Decisions & corrections
- The operator's word for this firing, relayed: «все норм пару часов роли не играет делаем как положено» — R5a
  through the gated door, ONE firing, a red final, R5b not re-fired (`inputs#I9`).
- Bash guard refused a `cd` into a non-root dir (twice) and a pipe of `rec.py` output into a filter (once); each
  re-issued in the passing form.

## Left outside the tree
- `~/.viola/instances/adopt-probe` (this firing's instance; his to remove by hand).
- `$XDG_RUNTIME_DIR/viola-r5a/` (empty `no-claude/`, `typed-in/`); the start's trust record for that `typed-in/` in
  his Claude config (persists by design, not read).
- Gate logs `$TMPDIR/andromeda-gate/2026-11-24-viola-adopt/implement-2026-11-25T00-27-47Z/`,
  `…/run-20261125T002846Z/`, `…/run-20261125T002854Z/`; the scratch clone
  `$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/3fa4c78b-62ea-4053-9dd7-588bf0b2eeb8/scratchpad/gated-base/`.

## Next
The OPERATOR PASS — entries 15 (`pre-push`), 16 (hygiene), 17 (push), 18 (CI), 19 (ci-fold) — then
`/andromeda-wrap-session` (the gated wrap flips `matrix#v2-13`).
