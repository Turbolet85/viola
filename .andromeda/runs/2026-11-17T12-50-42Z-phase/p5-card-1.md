# P5 review — 2026-11-17-supply-chain-green-and-booted-tests-end

**Chunk:** «The supply-chain gate green again, and every booted test process ends with its test» · viola-0.2.0 · Epoch 4
**The plan to read:** `.andromeda/cache/docs/2026-11-17-supply-chain-green-and-booted-tests-end/plan.md` (369 lines,
plan rev 3). No plan stands in a chunk's folder. Writing in that file changes no record until a `put`. A diff of the
chunk's records: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-17-supply-chain-green-and-booted-tests-end`.

## Goal
- **A — `supply-chain` green again.** Add one ignore to `deny.toml`, for RUSTSEC-2026-0543 (chrono unmaintained, no
  patched version). Its reason names rmcp 3.4.1 and schemars 1.2.2 (which pull chrono in through viola-mcp), viola's
  own use beside them, the founder's widened rule, and the re-read at each epoch boundary. The comment above the list
  is re-worded so it states both grounds. No other advisory is admitted. The widened rule's standing security-plan row
  is the wrap's, as an Expected amendment.
- **B — the rest of v2-37.**
  - `harness_lifecycle.rs` `options()` and viola-e2e's `cli.rs` `booted()` boot tied.
  - The stamping `viola verify`'s parent-death signal becomes **SIGTERM**; the supervisor keeps SIGKILL.
  - A new witness kills a test mid-stamp and reads that the verify's processes, run dirs, probe dir and Run A dir are
    all gone.
  - A census of viola-e2e's own test sources reads that no boot stays untied.
- **Leaned per the rows (no question asked).** A SIGKILLed verify leaves its dirs «to be removed by hand»
  (`architecture · decision/verify-removes-only-own-dirs`). A test-side remover of them would be a fifth removal way
  (`security-plan · decision/test-removal-own-dirs-only`). SIGTERM runs verify's own stop, which removes only its own
  dirs.

## Top acceptance criteria
- `cargo deny check` reads `advisories ok, bans ok, licenses ok, sources ok`. The fuzz audit reads
  `advisories ok, sources ok`. The ignore list is exactly the three rmcp ids plus RUSTSEC-2026-0543, and every ban
  still fires.
- The chunk's own pre-CI run reads `verdict: green` with `supply-chain` among its checks (`ci.py conclusion --sha HEAD
  --wait 1800`).
- `killed_test_leaves_no_stamping_verify` and `viola_e2e_census_finds_no_untied_boot` pass. They read red under the
  controls `vfy-term`, `vfy-tie` and `e2e-census`, each fired with `--timeout 600`.
- Every boot of `harness_lifecycle`, viola-e2e's `cli` and the MCP binaries passes tied.

## Touchpoints
`deny.toml` · `crates/viola-e2e/src/harness/boot.rs` · `crates/viola-e2e/tests/harness_lifecycle.rs` ·
`crates/viola-e2e/tests/cli.rs` · `crates/viola-e2e/tests/harness_tied.rs`. No new file, and no product source.

## Test commands (15 entries)
- 1–2 lint, seen green.
- 3–6 deny probes, baselines `red` · `green` · `red` · `green`. 3 and 5 are red on exactly the missing id, and both
  have a must-pass green in a scratch clone carrying the ignore (`p5-controls.md`).
- 7 viola-e2e lib, green.
- 8–11 harness suites, green: 2, 20, 11 and 7 passed.
- 12–15 operator: pre-push, hygiene, push, CI read.

## What would turn the gate red again
- Any new advisory, or a yanked crate, on the root or fuzz graph.
- The ignore list gaining or losing any id (entry 5).
- **Not** RUSTSEC-2026-0543 gaining a safe upgrade. cargo-deny's ignore is keyed on the id alone, so that change turns
  nothing red; the founder's epoch-boundary re-read is its control. This is recorded as an owed reading, unmeasured.

## Concretization preview
No capability is claimed. The claimable pool holds none this chunk proves. v2-37 reads `verified`, claimed by
2026-11-17-booted-wrapper-does-not-outlive-its-test, and the ledger tool refuses a verified cap's claim. So the
directive's «claim it» becomes an Expected amendment instead: a dated `matrix#v2-37` note naming the two new witnesses,
so the wrap can lift the narrowing. Its acceptance text is untouched, never weaker.

## Verbatim block (`inputs#I1` item 12)
```
architecture: rows 948 · packs 10 · picked 83 (51 by every draw) · standing 3 · 29504 B
security-plan: rows 606 · packs 7 · picked 64 (36 by every draw) · standing 5 · 20498 B
design-system: rows 315 · packs 4 · picked 13 (6 by every draw) · standing 0 · 5653 B
layout-templates: rows 222 · packs 3 · picked 6 (0 by every draw) · standing 0 · 1823 B
test-plan: rows 1336 · packs 14 · picked 159 (79 by every draw) · standing 46 · 41710 B
obs-plan: rows 1024 · packs 11 · picked 105 (28 by every draw) · standing 74 · 27657 B
a11y-plan: rows 235 · packs 3 · picked 17 (6 by every draw) · standing 2 · 6385 B
rows-union: 7 master(s) · picked 447 of 4686 · rule any
```
- **Rows read whole** (sum 132 230 B, under the 250 000 B line), with each standing list and history file read once.
- **Every row the plan cites is in the union** (each checked by handle in its rows file): security 5, test-plan 9,
  architecture 5, obs 1. None comes only from a standing list.
- **`[RETYPED]` lines, verbatim. No re-spawn anywhere: task 7/7, pick 156/156, history 7/7 on the first try.**
  ```
  layouts-pick-p1.d3 · [RETYPED] line 59: contract/cli-token-rendering/radius-* is no row of the pack — read as contract/cli-token-rendering/radius, the pack's row 59
  tests-pick-p7 · [RETYPED] line 56: decision/outer-pty-reader-drained-on-own-thread is no row of the pack — read as decision/outer-pty-reader-drained, the pack's row 56
  tests-pick-p9.d2 · [RETYPED] line 71: decision/synthetic-fixture-text-only is no row of the pack — read as decision/synthetic-fixture-text, the pack's row 71
  obs-history · not a marker: **2026-11-01T02-46-44-wrap** — history:46
  ```
- **Non-zero exits and refusals:**
  - `cargo deny check advisories` exit 1 at P3 (the red, reproduced).
  - `gh api …/logs` exit 1, «the response contains terminal escape sequences; pass --allow-escape-sequences», re-issued
    with the flag.
  - The Bash guard refused two `cd`-led reads and one pipe of `gate.py run` output into a filter; each was re-issued in
    the form it passes.
  - The stamp-ahead hook blocked the plan's estimated `Generated:` stamp; corrected from `date -u`, plan rev 2.
  - Baseline entries 3 and 5 red, as predicted.
  - No `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line was printed by any tool this run.
- **Where a letter was wrong or silent:**
  - The architecture history (item n=7) says the ignore list «was empty, so this chunk adds its first entry». At
    `4ef8f4b9` it holds three rmcp ids.
  - The directive's «claim it» (item 5) meets a verified cap the tool will not claim: carried as a ledger note.
  - The CARRY and the history say four sync crates; `scripts/sync-crates.txt` lists five.
  - The scope's premise that the last chunk's census covers viola-e2e is false: it walks the root `tests/` only.
  - The SIGKILL tie could never pass «nothing of it is left». Both were corrected at P3 (`[premise-corrected]`).
  - My first research slot read `none` beside a runner-only bullet. Check 9 made it a fetched read of the advisory page
    (no patched versions; alternatives jiff and time; no fork named).

## Beyond the plan
- **Owner for an out-of-scope finding:** `.config/nextest.toml`'s `ci` 20 s kill line omits `harness_lifecycle`,
  viola-e2e's `cli` and `harness_tied`, though each drives a stamping verify through `boot`
  (`test-plan · decision/verify-driven-tests-20s-kill`). Proposed owner: the existing entry «Version done-check»,
  pinned by the wrap's route adaptation as a CARRY.
