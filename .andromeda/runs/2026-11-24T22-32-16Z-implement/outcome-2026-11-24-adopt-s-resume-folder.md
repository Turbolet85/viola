# Outcome — 2026-11-24-adopt-s-resume-folder (run 2026-11-24T22-32-16Z-implement)

**Outcome: green** — the operator pass next (entries 11-14), then the wrap.

## Files changed (9 modified · 0 new; `gate.py scope`: clean — changed 9 · listed 9 · recorded 0)
- `src/cmd/run.rs` — `pump_end` (the pump with the child's exit logged, its own exit left to the caller); `pump_child` written over it, mapping unchanged
- `src/cmd/adopt.rs` — `Refusal::NotResumed` (`not-resumed`, its fixed pair), `resumed()`, adopt's reading after the child ends
- `schemas/diag-line.v1.json` — `not-resumed` in `process-exit`'s detail enum
- `src/bin/viola-fake-agent.rs` — argv-only `--resume-absent`
- `src/bin/viola-fake-agent/test_cases_opts.rs` — `opts_parse_resume_absent`
- `tests/cli_adopt.rs` — the three step-6 cases
- `tests/cross_exit_causes.rs` — the `not-resumed` row
- `tests/contract_error_bodies.rs` — the new document's writer named
- `scripts/r5-adopt.sh` — `start=` on `r5a`, the rehearse-only `r5c`

## Gates (implement's run)
- 1 `cargo fmt --all --check` green · 2 clippy green · 3 root units green (1403 passed; 1401 before + 2 new) · 4 adopt / exit / error-body binaries green (78 passed; 75 before + 3 new) · 5 `test(/not_resumed/)` green (4 selected, 4 passed) · 6 `bash -n` green · 7 rehearsal green (all five atoms) · 8 hostwatch recorded (CONTENDED, escher) · 9 `run --unit` green · 10 `run --integration` green.
- 11-14 `leg = 'operator'`: the operator pass's (pre-push, hygiene, push, ci.py).
- Control `not-resumed` (entry 5, `--timeout 900`): `holds` — in place green, removed red, the case failed (`evidence/control-1.txt`).
- Smoke: skipped — the plan lists no `smoke` entry (the start sequence is unchanged); entry 7 drove the built `viola` through three adopt starts over the fake agent.

## Deviations (with justification)
- Step 6's cases run `viola adopt` piped, outside any terminal (as `tests/cli_up.rs`'s `sessionless` starts a child), not through `Wrapper::adopt`, which waits for readiness a not-resumed start never reaches; piped, stdout and stderr read apart, as the criterion asks.
- Step 6's `fake-args` carry `--fixtures <root>/fixtures/claude` and `--cli-version 2.1.328` beside `--resume-absent`: without `--fixtures` the fake has no SessionEnd payload to fire. No `--screens`, so the child draws nothing on stdout. No new flag or mode.
- Step 5: under `--resume-absent` the fake also closes its hooks and ignores SIGHUP before its exit 1, as its other exits do; `launch` returns whether it launched, so both launch sites (a trusted start and the trust dialog's accept) exit 1.
- Entries 1-8 and 9-10 were fired as two `--only` calls: the overseer directive (`inputs#I3`) holds the whole layers on a CONTENDED host; they ran after the overseer's quiet word (`inputs#I4`).

## Unproven, plainly
- The not-resumed end is proven on the fake agent alone. What the real CLI does with an id it cannot resume was seen once from outside (a child ended in under a second, one SessionEnd, no SessionStart — the R5a firing 2 evidence) and is not re-read here. No real `claude` ran in this run.

## Process hygiene
| process | started by | final state |
|---|---|---|
| gate entries 1-10 (cargo, nextest, the harness, test wrappers and fake agents) | this run | terminated — census lists no `projects/viola/target` process |
| the R5 rehearsal's `script` hosts and wrappers (3 arms) | this run (entry 7) | terminated — census lists no `r5-adopt` or `script` |
| the control's clone build and firings | this run | terminated, clone removed (`control-1.txt`) |
| 9 processes the census lists (3 overseer `claude` sessions, the prototype's `viola run` wrappers and their `claude`, a product `viola mcp`, a product `viola wait`) | NOT DERIVED — not this run's | left running — their owners' |

## Decisions & corrections
- Overseer directive `inputs#I3`: no real `claude` in any form; the R5 script only under `--rehearse`; hostwatch before the whole layers and pre-push; a CONTENDED read naming a neighbour stops the whole layers.
- The stop fired: 22:38:59Z and 22:40:20Z read CONTENDED (escher alive 330-335 s of the stalled seconds); entries 9-10 waited.
- Overseer word `inputs#I4`: quiet since 2026-11-24T23:06:31Z; read `--from` that instant, never `--last 15`. Read 23:06:41Z: QUIET.
- Guard refusals: two `cd` into the skill's references dir refused by the Bash guard; re-issued as Read calls.
