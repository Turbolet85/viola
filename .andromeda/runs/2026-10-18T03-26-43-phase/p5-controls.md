# P5 controls — 2026-10-18-compact-post-condition-ledger-row

Every control fired AS THE ENTRY through `gate.py run --plan viola-0.1.0/chunks/2026-10-18-compact-post-condition-ledger-row/plan.md --entry {n} --project-root {clone}`,
no `--run-dir`, over a shared scratch clone at `$TMPDIR/{scratchpad}/p5-clone` (`git clone --shared` of the project at
HEAD `7f35c8ecec0a`; the control inputs live only in the clone). Driver: `$TMPDIR/{scratchpad}/p5-controls.sh`,
`p5-counts-moved.py`. 2026-10-18T20:46Z–20:47Z.

## The clone unmodified (reads what the real tree read)
- 12 the fixture guard — red · exit 1 (compact-1 not recorded) — real tree: red, exit 1
- 14 the literal probe — red · last line `106` — real tree: red, last line `106`
- 15 the preservation guard — green · exit 0 — real tree: green, exit 0

## Entry 14 (the literal probe)
- must-pass: every relied-count literal in the five files moved to the post-chunk counts (`/23]`→`/24]`,
  `; 23]`→`; 24]`, `23 pass`→`24 pass`, `/20]`→`/21]`, `[&str; 20]`→`[&str; 21]`) and `src/run/version_gate.rs` given the
  older-stamp words "the twenty-three rows of an older stamp" — green, last line `0`.
- must-fail: the clone unmodified — red, last line `106`.

## Entry 12 (the fixture guard)
- must-pass: `fixtures/claude/2.1.287/SessionStart.compact-1.json` minted untracked — green.
- must-pass: the same file committed (the post pre-CI-commit form; clone HEAD `89d3dbd`) — green.
- must-fail: compact-1 minted plus a modified `SessionStart.clear-1.json` — red, exit 1.
- must-fail: compact-1 minted plus an extra `SessionStart.compact-2.json` — red, exit 1.

## Entry 6 (the new cases by name)
- baseline red on the real tree: exit 1, `"ok":false`, nextest-unit `["nextest-exit-4"]`, 0 tests run of 2341 (the
  cases do not exist yet).
- must-pass control, typed as the tool types an entry (`/usr/bin/bash -o pipefail -c`), differing from the entry's `run`
  by the filter operand alone — `bash scripts/agent-run.sh run --unit --filter 'test(/check_local_command_clear_/)'` on
  the real tree: exit 0, `"ok":true`, nextest-unit 1 passed 0 failed (log `$TMPDIR/{scratchpad}/p5-control-6.log`).
- the entry's `contains "failed":0` atom was dropped at P5: the doctest suite prints `"failed":0` on every run, so the
  atom could not fail.

## Entry 15 (the preservation guard)
- must-pass: five modify-set files edited (the entry-14 input) — green.
- must-pass: compact-1 minted plus an evidence file under `viola-0.1.0/chunks/…/evidence/` — green.
- must-fail: `Cargo.toml` edited — red, exit 1.
- must-fail: an untracked `src/new_file.rs` — red, exit 1.
