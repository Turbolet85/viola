# P5 review card 1 — 2026-11-05-linux-only-code-and-ci: "Linux-only code and CI"

Written 2026-11-05T12:01:39Z, at the resume of P5 in this run dir (`.andromeda/runs/2026-11-05T08-59-20-phase/`), from
`p5-review.md`. Plan rev 5 (tx 220) · research rev 3 · scope rev 6. sampler: off.

**Open the plan in an editor:** `.andromeda/cache/docs/2026-11-05-linux-only-code-and-ci/plan.md` (its render, written
just now). No plan stands in a chunk's folder. Writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-05-linux-only-code-and-ci`
prints a diff of the chunk's records.

## Goal
Viola becomes Linux-only, on the founder's own ruling of 2026-11-04. Every Windows and macOS code arm, test, test-support
twin, vendored binary, manifest table and CI job leaves the tree, and all of it stays recoverable at the tag
`multi-os-final`. The tag is set on the last three-OS commit (`ecbe7ab3`) and pushed before any removal commit. No Linux
behaviour moves, with one narrowing: the closed words no Linux build can emit leave with their code (`inputs#I2`).
The chunk also fixes the open `msrv` red of ci#41930831242: the two e2e-home backing cases will decide on a free figure
the test supplies, never the runner's disk. 0 live sessions.

## Top acceptance criteria
- The tag `multi-os-final` resolves on origin to `ecbe7ab3`, which holds `src/conpty.rs` and `windows-mutants.yml`. The
  pre-CI commit's parent is the tagged commit.
- The carry's pattern `cfg!?\(.*(windows|target_os = "macos")` reads no line over the tracked `.rs` files. No manifest
  or deny config names `windows-sys` or a `cfg(windows)` table. The thirteen named paths are gone. No workflow holds a
  Windows or macOS job, OS condition or `pwsh` step.
- CI on `ubuntu-latest` reads `verdict: green` for `test`, `lint`, `msrv`, `perf`, `release` and `fuzz-replay`, each
  read by its own `--name` entry. **`supply-chain` is recorded, not asserted**: its one expected red is
  RUSTSEC-2026-0441, owned by a CARRY on «Version done-check» (`inputs#I3`). A red naming anything else is this chunk's.
- Locally, `cargo deny check bans licenses sources`, the fuzz lockfile audit, the tokio bans, `deny-probes`, zizmor,
  `rmcp-features`, `release-check --probe` and `orphans-check` pass. `cargo deny check advisories` is recorded, red on
  that one advisory alone. No ignore is widened.
- `pre-push` reads `ok:true` through `linux-tests` at the §10 coverage floors (85/95/80), with `COVERAGE_IGNORE` unchanged.
- The re-pointed census `tests/contract_os_gated_tests.rs` passes. A `windows`/`macos` predicate reads as a fault.
- Linux exit codes, wire contracts and human output stand byte for byte. Revive's codes go from 11 to 10, verify's from
  6 to 5, and the `process-exit` `detail` enum from 20 to 19 (predicted, re-measured by rule).

## Touchpoints
- New files: none.
- Files to modify: research's `## Files to modify`, 105 items, all present at HEAD (deletions included). None is a master.
- Steps 1–15: the msrv fix first; then manifests and deny targets, the five crates, the root bin and the closed words;
  then the Windows-only test files, the tests' arms, the census, CI and scripts, and the bounded-readers census.

## Test commands (49 entries)
- Lint: fmt, clippy, G1, G3, lint-probes, check.
- The four removal guards (8–11): predicted red on the untouched tree, must-pass green in a scratch clone
  (`p5-controls.md`).
- msrv: the injected-figure cases (12); the unit run, its gate and check on 1.96 (13–15).
- Dependency policy (16–26): entry 16 asserts bans, licenses and sources; entry 17 is advisories, report-only.
- Then unit, the e2e census filter, perf and its gate, the smoke cleanup/boot/status, G2, G4 and the second cleanup.
  `pre-push` closes the local block.
- Operator pass (38–49): the tag, then the tag check, hygiene, the push and the parent check. Then seven CI reads, one
  per check-name prefix; supply-chain's is report-only.
- **Removed:** `agent-run.sh secret-scan`. It ran past its 1800 s bound on the untouched tree, reading the 12 GB
  `target/agent-run`, so the scan is CI's alone (`ci.yml:107`, `:191`, in the `test` and `perf` jobs). Those two jobs'
  CI reads assert it.
- Baselines: 28 entries `new = true` (no leg), each with its `baseline`. `gate.py baseline` lists 0 NOT SEEN, 9 seen
  green and 12 leg rows.

## P5 step 1 (re-run whole after the fold)
- `gate.py run --dry-run`: 49 entries parse, no `env` unset.
- `planlint.py check`: 0 hits. 12 (d) lists five «unchanged» claims, each read and each holds: `scratch.rs`'s Linux
  arm, `nightly.yml`, `HEADROOM_FLOOR_MIB`, `COVERAGE_IGNORE` and deny-probes 14/14.
- `gate.py scope --lists-only`: `parsed 105 items`.
- `inputs.py verify`: 3 entries, 1 unchanged, 2 n/a (messages), 0 uncited.
- Check 2's path read: 105 present, 0 masters.
- Validation-1: aligned with scope rev 6 (`inputs#I3` now folded into the plan).

## Concretization preview
- **No capability claimed.**
  - `v1-01` (Linux CI pipeline) is not claimed: its outcome is a CI run on every push. Only the operator pass's CI read
    sees it, never an implement-time ref. Its dependency-policy clause (advisories) reads red until the CARRY's remedy.
  - `v1-53` (the fake-agent suite and one live Linux run) needs a live run. This chunk runs 0, and the cap's own notes
    already name «Version done-check» as its owner.
  - Step 4 writes each a dated note.

## Refusals (the directive's item 10), with the letter line followed
In the earlier window of this run (as recorded in `p5-review.md`):
- Bash guard, `cd` ×2: host-linux.md «anchor every cross-call path absolutely, or `cd` only inside a subshell». Each was
  re-issued once with absolute paths.
- The harness grep refusing a bounded repetition, ×1: the global CLAUDE.md Shell rule «write `command grep …`».
- The Bash guard refusing a pipe into a filter, ×1: phase SKILL.md Setup 1, «a tool's stdout is never sent to a file and
  its output never through a filter».
- The `detail` v1.28 tool fault, and the first P2 that it stopped: the directive's item 5, «A tool fault stops the phase».
- The five first-attempt `EVERY_ENTITY` refusals: fan-out.md §The extracted masters. Each took the contract's one
  re-spawn (the directive's item 4).
- The ten strong-reader refusals: SKILL.md P2 1b. Eight were cleared by `fanout.py move` («moved into its owner group's
  extract … never re-spawned») and two by one re-spawn (the directive's item 4).

In this window:
- Bash guard, `cd` ×1 (a `cd` into the skill's `references/` dir to size files): host-linux.md, the same rule. It was
  re-issued once with absolute paths.

P5's `landed tx` line does not exist yet: step 5's `rec.py outputs land` runs after the word. It goes on the report.

## Owed outside this repository
- New-session health check 13 expects the `{sh,ps1}` pair, so after this chunk it reads ⚠. This is known upstream (F411).
  On the operator's word, no setup Phase 4 is run for it.

## Beyond the plan
- **A CARRY on «Version done-check»** (origin: this run, `2026-11-05T08-59-20-phase`), worded per `inputs#I3`:
  RUSTSEC-2026-0441 (`pin-utils` unmaintained, via `portable-pty =0.8.1` → `nix 0.25.1`) reads red in supply-chain's
  advisories family. The 0.9.0 Windows garbage-read bug that justified the `=0.8.1` pin leaves with this chunk, so
  done-check measures moving `portable-pty` off 0.8.1 to a release whose graph drops `pin-utils`. No ignore-list
  widening.
- **`v1-01`'s owner: «Version done-check»**. Its CI-run outcome and its advisories clause are provable at that entry's
  CI read, once the CARRY's remedy lands.

Apply? (yes / review / cancel)
