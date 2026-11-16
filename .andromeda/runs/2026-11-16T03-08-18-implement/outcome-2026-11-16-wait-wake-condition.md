# Outcome — 2026-11-16-wait-wake-condition · run 2026-11-16T03-08-18-implement (re-entry on plan rev 10)

**Outcome: green.** Entry 5 (new at rev 10) reads green, and the `held-turn` control fired on it under `--timeout 120`
reads `holds`. All six step-9 controls now read `holds`. The whole non-operator block (entries 1–13) re-fired once
and read green. Next → the operator pass: entries 14 (hygiene, as written), 15 (push, `--operator 15`) and 16
(`ci.py conclusion`, as written), run in this window on the operator's direction. Then `/andromeda-wrap-session`.

## Gates (this run)
- Entry 5 alone (`--entry 5`): `5 unit green · exit 0 · 0.23s` — `4 tests run: 4 passed, 1004 skipped`.
- Control `held-turn` on entry 5, `--timeout 120` (`.andromeda/runs/2026-11-16T03-08-18-implement/control-1.toml`;
  `evidence/control-3.txt`):
  - `entry 5 in place green · 16.0s → 5.2.log`
  - `control held-turn · entry 5 · holds — in place green, removed red · exit 0 ✗ (exit 100) · 16.0s · 2.21s → 5.without-held-turn.log`
  - `control: 1 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`
  - The removed log: the four cases `case_1_zero` · `case_2_one` · `case_3_three` · `case_4_null` each FAIL in 0.045 s,
    `4 tests run: 0 passed, 4 failed, 1004 skipped`, no TIMEOUT line.
- The whole block (trail `.andromeda/runs/2026-11-16T03-08-18-implement/gate-2026-11-16-wait-wake-condition.json`):
  `entries 16 · green 13 · red 0 · recorded 0 · timeout 0 · not-run 3`. Entry 9 44.98 s; entry 12 `run --unit`
  7.11 s; entry 13 `pre-push` 84.07 s. Entries 14–16 `not run — leg operator`.
- The six controls, all `holds`:
  - `held-turn-e2e`, `nofalsewake-e2e`, `copy-not-link`, `one-stamp` — `evidence/control-1.txt` (run 01-45-25);
  - `no-false-wake` — `evidence/control-2.txt` (run 02-56-58, 7.29 s);
  - `held-turn` — `evidence/control-3.txt` (this run).
- No deferral. No smoke (no boot-path touchpoint). No live start, no mutation run.
- Matrix: v2-17 `planned → implemented`, its ref naming the two booted cases, the two unit tests and the four wake
  controls.

## Files changed (whole chunk)
`src/run/wait.rs` (test module only) · `fixtures/fake-scripts/wait-held-turn.json` (new) · `tests/cli_wait_last.rs` ·
`tests/support/home.rs` · `tests/cli_verify.rs` · `.config/nextest.toml` (comment). This run edited no source; it
wrote only the run dir and `evidence/control-3.txt`.

## Deviations
- Entries 1–4 and 6–13 were re-fired, although rev 10 says "no other entry … is re-fired". The operator directed
  "re-run what the letter needs for a green outcome", and the test module changed (PARK_BOUND, run 02-56-58) after
  entries 7–9 and 13 last ran. All read green.
- Earlier runs' deviations stand, absorbed by plan revs 6, 8 and 10: the stamp tests at `tests/cli_verify.rs`'s root;
  `tests/support/verify.rs` unchanged; the other-stamp sweep; the control id `nofalsewake-e2e`.
- Scope: `scope: clean — changed 6 · listed 6 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) ·
  absorbed 0 · excluded 267`. Record line: ``- `tests/cli_verify.rs` · in-intent · serves step 6 · self``.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / nextest / viola / fake agent of entry 5, the block and the control (clone `~/.cache/andromeda-control/da582644`) | this run | terminated. `gate.py census --comm viola --comm cargo-nextest --comm viola-fake-agent --args projects/viola/target` at 2026-11-16T03:12:47Z: 7 running, none from this repository's `target/` (the product install `~/.local/viola-product`, the lab prototype's builds, `~/.viola/bin/0.1.0-…` mcp). The tool removed the clone |

## Decisions & corrections
- The operator's direction was followed: no live start, no mutation run. No plan-moving red was met, so the run went
  on to the operator pass.
- The Bash guard refused one `cd` into the skill's references dir. It was re-issued once with absolute paths.
