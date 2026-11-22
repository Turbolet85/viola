# Outcome — 2026-11-22-epoch-4-cleanup · implement run 2026-11-22T19-03-55Z-implement

**Outcome: green** — every non-leg entry of the block green (`cargo fmt --all --check` through `rec.py check --root .
--no-heal`; the host read `recorded`), the ten remove-the-guard controls `holds` (`control: 10 holds · 0 timeout · 0
does not hold · 0 unreproduced · clone removed`). Next → the OPERATOR PASS (entries 16-22: the harness boot smoke
`agent-run.sh boot/status/cleanup --session cleanup-smoke`, `agent-run.sh pre-push`, `gate.py hygiene`, the push,
`ci.py conclusion --sha HEAD --wait 1800`), run in this window per the directive (`inputs#I4`), then the wrap in a fresh
window.

## Files changed
Listed (research's Files to modify, 9 of 10 touched by the scope read's set; `.claude/rules/testing.md` is excluded from
the read by rule): `src/run/send/slot.rs` · `src/run/agents.rs` ·
`crates/viola-agent-claude/src/ledger/test_cases_checks.rs` · `crates/viola-agent-claude/src/ledger/test_cases_stamps.rs`
· `crates/viola-agent-claude/src/screen.rs` · `crates/viola-pty/src/pump.rs` ·
`crates/viola-e2e/tests/harness_lifecycle.rs` · `crates/viola-e2e/src/harness/boot.rs` · `.config/nextest.toml` ·
`.claude/rules/testing.md` (through `rec.py harness land`, tx 4037, rev 54).
Recorded (scope record, `gate.py scope`: `scope: clean — changed 11 · listed 9 · recorded 2 (companion 2 · mechanical 0 ·
in-intent 0 · widening 0) · absorbed 0 · excluded 225`):
- `tests/contract_bounded_readers.rs` · companion · serves `src/run/agents.rs` · self
- `tests/contract_lints.rs` · companion · serves `.config/nextest.toml` · self

## Deviations, with their justifications
- **The block fired in parts** (`--only 1-11`, then `--only 12,13,14,15`), never whole: the directive (`inputs#I4`) has
  the host read before the three whole layers and a stop on a neighbour's contention, which one whole firing cannot
  honour. Full pass after the fixes: entries 1-11 green / recorded, 12-15 green (15 after the land, below).
- **A tenth control, `env-ne`**, beyond the plan's nine: the plan lets `env-user`'s case cover the `== → !=` mutant; the
  directive asks each survivor's case shown to kill its mutant by hand, so that mutant got its own run (`holds`).
- **Two companion test pins** the research sweep did not list read red in `run --e2e` (firing 1): the bounded-readers
  census needle spelled `VERSION_DEADLINE,` in `src/run/agents.rs`, and `contract_lints`' `VERIFY_WINDOW_FILTER` the old
  `mutants` filter line. Both re-pointed to the new text (the lint test's swapped-order control now built from the two
  constants); `run --e2e` green at firing 2 and 3.
- **`rec.py check --no-heal` (entry 15) red at firing 2** on `check: clean` — `records: docs · unlanded 2`, the two
  evidence files this run wrote — with `records: harness · render ok` holding; `rec.py land` (tx 4040), then firing 3
  green. The atom reads the whole store, not the harness render alone.
- No gate deferred; no `defer` in the block; no `gate.py guard` entry in the block (no guard pass line to copy).

## Surfaced / stuck
none.

## Process hygiene
`gate.py census --comm viola --comm viola-fake-agent --comm viola-harness --comm cargo-nextest --comm sleep --args
projects/viola/target` (this run dir's trail): `census: 10 running`, none this repository's.

| process | started by | final state |
|---|---|---|
| the gate entries' cargo / nextest / harness children (entries 1-15, 5 firings) | this run (through `gate.py run`) | terminated — none in the census |
| the control clone's builds and tests (`~/.cache/andromeda-control/7b01b3ab`) | this run (through `gate.py control`) | terminated; `clone removed` |
| `/bin/sleep 30` exec'd by `read_logged_kills_a_program_that_outlives_its_bound` | this run (the test) | terminated — killed at its 5 s bound in place; ran out its 30 s in the clone's `agents-bound` firing; none in the census |
| `viola` ×7 rows (viola-lab prototype `run` / `wait`, `~/.viola/bin/… mcp`, `~/.local/viola-product/bin/viola wait`) | NOT DERIVED — other trees' (the bridge, the pinned product) | left running: not this run's |
| `sleep` ×3 (parents `ctxwatch.sh`, `deploy.sh --wait`, a Bash-tool shell) | NOT DERIVED — the overseer's tool loops and another shell | left running: not this run's |

## Decisions & corrections
- Host read before the layers, firing 1 (`hostwatch.py read --last 15`, 19:09:26Z): `QUIET - 5 s stalled on IO` → the
  layers fired. Firing 2 (19:16:39Z): `CONTENDED - 10 s stalled on IO: 19:09:01Z..19:09:06Z 5 s; 19:09:46Z..19:09:51Z
  5 s … build tools alive in them: viola 10 s` — no neighbour named (escher alive 5 s, in neither span); the stall is
  this run's own gate run, so the layers fired (the directive stops on a neighbour; verification-harness.md 2026-11-19).
- Guard refusals, each re-issued once in the passing form: a heredoc to a scratch script file (→ Edit-tool edits); a
  `cd` outside the root, twice (→ absolute paths).
- Sweep hazard: a census needle in a test source spells a product argument verbatim (`"VERSION_DEADLINE,\n …"` in
  `tests/contract_bounded_readers.rs`), and a lint test pins a nextest filter by its exact whole line
  (`VERIFY_WINDOW_FILTER` in `tests/contract_lints.rs`): a sweep of a moved constant or filter reads the test tree's
  string literals too.
- The equivalents re-argued from the code at this run (`evidence/remove-the-guard.md`), matching card 2's argument.
