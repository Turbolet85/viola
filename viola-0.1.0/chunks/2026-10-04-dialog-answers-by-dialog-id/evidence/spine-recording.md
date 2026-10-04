# Spine recording — the operator leg (plan step 7)

## Run 1 (fired once by /implement, 2026-10-04 ~14:23Z) — RED on its version atom
- Run: `H="$HOME/.viola-record-$(date -u +%Y%m%dT%H%M%SZ)" && cargo run -q --bin viola -- --home "$H" verify --record fixtures/claude`,
  preceded by the `command -v claude` probe (green: the mise `latest` claude on PATH).
- Exit 0. Six step lines `[01/06]` … `[06/06]`, all `pass`, then `stamped 2.1.288  6 pass  0 fail`.
- Atoms: `exit 0` held · `contains stamped 2.1.287  6 pass  0 fail` **did not hold**.
- Cause, measured: mise installed `claude` 2.1.288 at ~14:19Z (`~/.local/share/mise/installs/claude/2.1.288/`, its
  `latest` / `2.1` / `2` links moved to it at that time), between the plan's revision (~14:17Z) and this leg. `claude
  --version` now prints `2.1.288 (Claude Code)`; the 2.1.287 binary is still installed beside it and answers
  `2.1.287 (Claude Code)`.
- Artifact: the four spine fixtures were written to `fixtures/claude/2.1.288/` (`SessionStart` · `UserPromptSubmit` ·
  `Stop` · `SessionEnd` `.default.json`), outside research's derived set (`fixtures/claude/2.1.287/*.default.json`).
  Nothing was written under `fixtures/claude/2.1.287/` by this leg. Left in place for the operator's decision.
- The recording home is `~/.viola-record-20261004T142325Z` (left in place).

## The operator's ruling (2026-10-04, overseer measurement)
Option 1: mise installed 2.1.288 at 14:19Z, but every running `claude` — this builder and every prototype-driven
session included — still runs the 2.1.287 binary, so 2.1.287 is the version the relayed captures and the live host
share. Re-record the spine against the pinned 2.1.287 binary, remove `fixtures/claude/2.1.288/` and the stray recording
home, and record the run-text deviation. Stamping 2.1.288 is not this chunk: the operator carries it as an owed step.

## Run 2 (re-fired once on that ruling, 2026-10-04 ~14:24Z) — GREEN
- **Run-text deviation:** the plan's entry runs `… verify --record fixtures/claude` against `claude` on PATH; run 2
  names the program after `--`:
  `H="$HOME/.viola-record-$(date -u +%Y%m%dT%H%M%SZ)" && cargo run -q --bin viola -- --home "$H" verify --record fixtures/claude -- "$HOME/.local/share/mise/installs/claude/2.1.287/claude"`.
  `viola verify` takes "the CLI to verify and its arguments, after `--`" (`src/cmd/verify.rs` `VerifyArgs::program`),
  so the R8 strip, the probe and the scrub are the same code path.
- Exit 0. `[01/06]` … `[06/06]` all `pass`, then `stamped 2.1.287  6 pass  0 fail`. Atoms: `exit 0` held ·
  `contains stamped 2.1.287  6 pass  0 fail` held. Output kept as
  `.andromeda/runs/2026-10-04T14-20-29-implement/record-2.out`.
- Artifact (fresh, ~14:24Z): `fixtures/claude/2.1.287/{SessionStart,UserPromptSubmit,Stop,SessionEnd}.default.json`
  (317 · 442 · 465 · 364 bytes), beside the four relayed dialog fixtures.
- Recording home: `~/.viola-record-20261004T142437Z`.
- Cleanup of run 1's residue (`fixtures/claude/2.1.288/`, `~/.viola-record-20261004T142325Z`): the `rm -r` was
  refused by this session's permission layer; the removal goes to the operator.
- Owed, outside this chunk (the operator carries it): a `viola verify` stamp for the installed 2.1.288.
