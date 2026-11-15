# Outcome — 2026-11-15-api-error-turn-end · implement run 2026-11-15T22-51-16-implement

**Written:** 2026-11-15T23:06:53Z

## Outcome word
green + smoke — next: the OPERATOR PASS (plan entries 19 `gate.py hygiene`, 20 the push, 21 `ci.py conclusion`), then
`/andromeda-wrap-session`.

## Files changed (18 modified, 0 new — every one on research's «Files to modify» list)
`crates/viola-core/src/lib.rs` · `crates/viola-agent-claude/src/hook.rs` · `crates/viola-agent-claude/src/ledger.rs` ·
`crates/viola-agent-claude/src/ledger/test_cases_stamps.rs` · `crates/viola-agent-claude/src/lib.rs` ·
`plugin/hooks/hooks.json` · `src/cmd/run.rs` · `src/run/send/slot.rs` · `src/bin/viola-fake-agent.rs` ·
`schemas/diag-line.v1.json` · `schemas/claude-fixture.v1.json` · `schemas/fake-script.v1.json` ·
`tests/hook_events.rs` · `tests/cli_wheel.rs` · `tests/cli_list.rs` · `tests/hook_fail_open.rs` ·
`tests/security_negatives_strict_modes.rs` · `crates/viola-state/tests/state_events.rs`

## Gates (the block's second full firing, `entries 21 · green 18 · red 0 · recorded 0 · timeout 0 · not-run 3`)
All 18 non-leg entries green, `bash scripts/agent-run.sh pre-push` included (95.81 s). The three `operator` entries
(19–21) not fired here: the operator pass's. First firing read `red 4 (8,9,11,18)`; two causes, both test-side in this
chunk's own new cases (see Deviations); fixed and re-run `--only 8,11`, `--only 1,2,8,9`, then the whole block.

## Controls (plan step 14) — `control: 5 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`
registration · text-drop · text-drop-e2e · revalidation · data-keys: each `holds`. Lines: `evidence/control-1.txt`,
table `evidence/controls.md`.

## Smoke — ✓
By hand: cleanup → boot (`ok:true`) → status (`ready`, builder `live`/`idle`) → the written `hooks.json` holds the
`StopFailure` entry (`args ["hook","stop-failure"]`, no `@@`) → cleanup (`processes_gone`, `endpoint_gone`).
`evidence/smoke.md`.

## Matrix
`v2-16` planned → implemented, ref the cli_wheel, cli_list and hook_events cases and the registration control.

## Deviations (with justification)
- `tests/cli_wheel.rs` and `tests/cli_list.rs`: the new cases build a scratch fixtures root as a COPY of the recorded
  `fixtures/claude/2.1.287` set plus one synthetic `StopFailure.default.json`, not a fully synthetic set. A synthetic set
  holds no `Screen.*` rows, so the fake agent draws no input box and the next `send` reads `input-not-ready` (the first
  firing's red of entries 8/9/18). Plan step 10 says «the synthetic set of that test», and that test uses the committed
  set; the copy is that set. No fixture is committed (entry 13 green, `last line 0`).
- `tests/hook_fail_open.rs`: the spine-bound case's two `hook_event` asserts read the case's own event argument
  (`args[1]`) in place of the literal `"stop"`, which the new `stop-failure` row broke (first firing's red of entries
  11/18). In the listed file.
- `tests/security_negatives_strict_modes.rs`: the `HOOKS` array's length literal 9 → 10 beside its new row.
- `api_error_class` carries an `as_str` per class (kebab words) so the table test can hold each class to
  `is_code_word`; no class is written anywhere (plan step 2, F2 held).
- scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 18 · listed 18 · recorded 0`).

## Surfaced / stuck
none.

## Process hygiene
| process | started by | final state |
|---|---|---|
| the gate block's cargo / nextest / viola / fake-agent children | this run (gate tool) | terminated — census shows no process under `projects/viola/target` |
| smoke session `p-api-error-smoke` (harness supervisor, wrapper, fake agent) | this run (by hand, and entries 14–16) | terminated — cleanup `processes_gone:true`, `endpoint_gone:true` |
| control clone `~/.cache/andromeda-control/8c642676` firings | this run (gate tool) | terminated, clone removed |
| the operator's overseer `claude` sessions, the viola-lab prototype wrappers, `~/.viola/bin/…/viola mcp`, the product `viola wait` | NOT DERIVED (not this run's; other trees and the operator's sessions) | left running: not this run's — the operator's |

Census: `gate.py census --comm viola --comm claude --comm viola-fake-agent --comm viola-harness --args
projects/viola/target` → `census: 16 running`, none of this repository's `target/`.

## Decisions & corrections
- The operator's directive for this run: no live `claude` start (the cap is spent), no mutation run; journal each long
  run before a wait (`evidence/journal.md`); a red that moves a plan entry stops before the operator pass (none did).
- Guard refusals: the Bash guard refused two `cat` heredocs with a file target (re-issued through the Edit tool) and one
  `cd` that would move the session cwd (re-issued as a Read) — standing rules `host-linux.md` 2026-09-29 and the cwd rule.
- Sweep hazard: a fake-agent fixtures root built only from synthetic hook payloads holds no `Screen.*` rows, so any
  case that sends after the first turn reads `input-not-ready`; a case that needs a send copies the recorded set first.
