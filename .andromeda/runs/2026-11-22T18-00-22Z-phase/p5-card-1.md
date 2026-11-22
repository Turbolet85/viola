# P5 review — «Epoch 4 cleanup» (`2026-11-22-epoch-4-cleanup`), plan rev 3

**Open the plan:** `.andromeda/cache/docs/2026-11-22-epoch-4-cleanup/plan.md` (rev 3, the render). No plan stands in
the chunk's folder; writing in that file changes no record until a `put`. A diff of the chunk's records:
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-22-epoch-4-cleanup`.
Research: `doc:2026-11-22-epoch-4-cleanup/research` rev 2. Scope: rev 3. The founder-forks block:
`.andromeda/runs/2026-11-22T18-00-22Z-phase/founder-forks.md` — no fork of the operator's.

## Goal
The Epoch 4 audit's ranked defects mended in its order, each worded by what the next boundary's audit reads: a claimed
`session-start` whose snapshot write fails settles unconfirmed (the send closes `no-prompt-submitted` at its window,
instead of holding the slot forever); verify's agents read takes a bound of its own at the measured 5 s; eight of the
ten survivors killed by cases in their own crate, two named equivalents; the boot test's binaries hard-linked and
`ABORT_DEADLINE` 20 s → 15 s; the two drift replay tests into the `verify_window_` class; `testing.md`'s `paths:`
taking the `test_`-stemmed source files. The chrono advisory: no safe upgrade (`patched = []`, 2026-11-22), the ignore
stays. A SIGKILLed verify: leaves its own one-key trust record, a residual already named.

## Top acceptance criteria
- step 2's case green, and red with step 1's settle removed (`claimed-settle`);
- step 4's two cases green, `agents-bound` red with the bound raised;
- each of the eight killable survivors' cases fails with its mutant applied by hand (seven controls), recorded in
  `evidence/remove-the-guard.md`, the two equivalents named there;
- `binary(harness_lifecycle)` green; `binary(contract_fake_agent_drift)` green under `--profile mutants`;
- `rec.py check --no-heal` clean after the `testing.md` edit; the unit, integration and e2e layers `"ok":true`.

## Touchpoints (10, all modify)
`src/run/send/slot.rs` · `src/run/agents.rs` · `crates/viola-agent-claude/src/ledger/test_cases_checks.rs` ·
`crates/viola-agent-claude/src/ledger/test_cases_stamps.rs` · `crates/viola-agent-claude/src/screen.rs` ·
`crates/viola-pty/src/pump.rs` · `crates/viola-e2e/tests/harness_lifecycle.rs` · `crates/viola-e2e/src/harness/boot.rs`
· `.config/nextest.toml` · `.claude/rules/testing.md`.

## Test commands (22 entries; baselines on the untouched tree)
fmt · clippy · `viola-agent-claude` whole (green, 793) · `viola-pty` whole (green, 37) · root bins (green, 1199) · three
control entries `ctl-root` / `ctl-claude` / `ctl-pty` (red exit 4, 0 selected — vacuous until their cases exist) ·
`binary(harness_lifecycle)` (green, 10) · `binary(contract_fake_agent_drift)` under `mutants` (green, 13; the replay
tests 6.41 s) · hostwatch (recorded: CONTENDED 15 s, this phase's own builds) · `run --unit` / `--integration` /
`--e2e` (seen green at the last wrap) · `rec.py check --no-heal` (green; its must-pass control both sides,
`controls-entry-15.md`) · operator: boot smoke trio, pre-push, hygiene, push, `ci.py conclusion`.

## Capabilities
None claimed: the pool's twelve unclaimed caps (v2-07 … v2-15, v2-28, v2-30, v2-31) are Epoch 5–7 fleet work this
chunk does not build. No concretization preview.

## Reads the letters ask the card to name
- **Rows files read by search, not whole:** their sum is 318 793 B (architecture 63 721 · security 39 116 · design
  19 963 · layouts 10 784 · tests 89 280 · obs 80 036 · a11y 15 893), past 250 000 B; every handle the plan cites was
  searched in the rows files (`grep -F` per handle).

## The run's own record (the directive's §13, verbatim where a tool printed it)
- **The union, per master and its line:**
  `architecture: rows 983 · packs 10 · picked 196 (96 by every draw) · standing 25 · 63721 B` ·
  `security-plan: rows 633 · packs 7 · picked 118 (72 by every draw) · standing 8 · 39116 B` ·
  `design-system: rows 326 · packs 4 · picked 80 (59 by every draw) · standing 2 · 19963 B` ·
  `layout-templates: rows 225 · packs 3 · picked 49 (34 by every draw) · standing 1 · 10784 B` ·
  `test-plan: rows 1371 · packs 14 · picked 337 (173 by every draw) · standing 65 · 89280 B` ·
  `obs-plan: rows 1037 · packs 11 · picked 337 (213 by every draw) · standing 12 · 80036 B` ·
  `a11y-plan: rows 237 · packs 3 · picked 52 (44 by every draw) · standing 21 · 15893 B` ·
  `rows-union: 7 master(s) · picked 1169 of 4812 · rule any`.
- **Every cited row in the union or its standing list?** No — 13 of the 14 handles searched stand in a rows file;
  test-plan `decision/verify-window-80s-kill` stands in none and was looked up by `rec.py find` (the letter's small
  lookup), cited by its handle. The rows the founder-forks block lists as «read as not reached» were found by
  `rec.py find` over all seven masters, not by the union.
- **`[RETYPED]` lines:** none printed. **Re-spawns:** one — `tests-pick-p2.d3 · [ABSENT]` (pick-p2), re-spawned as
  `pick-p2-r1`: `VERDICT: 1 of 1 pass`. No dead draw; every other batch `VERDICT: N of N pass`.
- **Non-zero exits and refusals:** the Bash guard refused three calls (two pipes of a tool's output into a filter,
  one `cd` that would move the cwd), each re-issued bare once; `cargo deny … check advisories --config …` exit 2 twice
  (`unexpected argument '--config'`, the flag's position), then the probe ran as `cargo deny --config … check
  advisories` (exit 1 by design: the un-ignored chrono advisory, `advisories FAILED`); `command grep -n -i 'config'`
  over the help text exit 1 (no match); `hostwatch.py` first called at `~/.claude/skills/andromeda-tools/scripts/`
  (`can't open file`, exit 2); the baselines of entries 6–8 `exit 4` (no tests to run, by design); entry 15's control
  red on the edited clone (by design).
- **`UNPARSED` / `INDETERMINATE` / `NOT DERIVED`:** none printed by any tool this run.
- **Where a letter was wrong or silent:** (1) the directive names `hostwatch.py read` without its path; a search of
  the skills dir and the repository missed the overseer's tools path, so research first wrote the tool absent — found
  at P4 in the previous plan's entry, research corrected (rev 2), the founder-forks host line rewritten before P4 read
  it, the friction records retracted (`2026-11-22T18:45:07Z-a`). (2) `fanout-contract.md` was read to line 401 of 487
  (the Read cap; §Status and the tail of the instruction table unread) — no step of this run needed them. (3) The
  directive's §2 asks for a stalled-disk measurement of the agents read «without loading the neighbours»; on this host
  no stall can be produced so: the block says what stands unmeasured.

## Beyond the plan
| # | Proposal the word would approve | Owner / place |
|---|---|---|
| 1 | The advisory carry re-pinned: RUSTSEC-2026-0543's re-read moves to the Epoch 5 boundary (the carry's «none» branch, the operator's own word) | the Epoch 5 boundary, by this chunk's wrap's route adaptation |
| 2 | (8) `framing_turns`' step function and `send_through`'s section — not taken | the next boundary's code audit (its M1) |
| 3 | (9) the relied-row count from one source and the stamped-set table held once — not taken | the next boundary's code audit (its M3) |
| 4 | Whether the real CLI writes its own `projects` entry per verify probe cwd, and the found entry's writer — unmeasured (no live start here) | «Version done-check» (a chunk where a live start is allowed), by the wrap's route adaptation |
| 5 | The scratch clone `$TMPDIR/{scratchpad}/ctl-clone` (entry 15's control) left in place, no recursive delete | the operator, by hand |

Apply? (yes / review / cancel)
