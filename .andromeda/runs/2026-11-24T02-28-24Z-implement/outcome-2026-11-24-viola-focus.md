# Outcome — 2026-11-24-viola-focus (implement run 2026-11-24T02-28-24Z-implement)

**Outcome: green** · smoke skipped — no boot-path / UI-surface change · 0 fix iterations · Next → the OPERATOR PASS
(entries 14 pre-push, 15 hygiene, 16 push, 17 CI read; 18 the build entry and 19 R3 NOT fired — they wait on the
operator's own word, `inputs#I5`), then `/andromeda-wrap-session`.

## Files changed (base 081d4604, `cites.py added` → `.andromeda/runs/2026-11-24T02-28-24Z-implement/new-text-2.md`)
- New: `src/cmd/focus.rs` (393 lines) · `tests/cli_focus.rs` (508) · `scripts/r3-focus.sh` (280).
- Modified: `src/cmd/mod.rs` · `src/human/hints.rs` · `src/human.rs` · `src/bin/viola-fake-agent/hyprctl.rs` ·
  `src/bin/viola-fake-agent/test_cases_modes.rs` · `schemas/diag-line.v1.json` · `tests/cli_output_plain.rs` ·
  `tests/cross_exit_causes.rs` · `tests/contract_error_bodies.rs` · `tests/cli_error_bodies.rs` ·
  `tests/contract_diag_schema.rs`.

## Gates (trail `.andromeda/runs/2026-11-24T02-28-24Z-implement/gate-2026-11-24-viola-focus.json`)
- 1 `cargo fmt --all --check` green · 2 clippy green · 3 root-binaries units green · 4 the filtered new units green
  (25 selected) · 5 `test(/^focus_/)` green (9 cases of `tests/cli_focus.rs`) · 6 the seven reader binaries green
  · 7 `bash -n scripts/r3-focus.sh` green · 8 the R3 rehearsal green (`r3: green hand=focused opened=focused`, last
  line `r3: rehearsed — not a live reading`; fired alone first, then in 1-9, then after the fix below) · 9 the host
  read recorded · 10 `run --unit` green · 11 `run --integration` green · 12 `run --e2e` green.
- 13 `not run — defer` (gated: R3) · 14-19 `leg operator`, the pass's.
- Controls (`evidence/control-1.txt`): `control: 3 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone
  removed` — focus-reuse, focus-order, focus-no-client, each on entry 4, `--timeout 900`.
- Host: entries 10-12 waited for the overseer's quiet window (`inputs#I6`): my read 02:25:53Z..02:40:53Z was
  CONTENDED (escher 65 s alive in a 70 s IO stall); 02:46:08Z..02:46:21Z QUIET.

## Deviations (with justification)
- Scope record: `src/human.rs` · companion · serves `src/human/hints.rs` · self (the re-export of `focus_pair`).
  `gate.py scope`: `scope: clean — changed 14 · listed 13 · recorded 1`.
- Unit cases beyond the plan's list, under the plan's `::focus_` filter: `focus_maps_a_reply_of_another_kind_to_malformed`
  (a reply of another kind → `malformed`, step 2e) and a `zero` row in
  `focus_skips_the_workspace_dispatch_for_an_unread_or_special_workspace` (step 2e's `w >= 1`).
- `tests/cli_error_bodies.rs`' loosened-socket-directory table: focus's row reads exit 1
  `{"v":1,"error":"unsupported","detail":null}`, not `socket-dir-refused` — focus connects to no socket (step 2c
  reads liveness alone), and that table's PATH is an empty bin dir. The plan named the table, not the document.
- `scripts/r3-focus.sh`: under `--rehearse` the `hyprctl locked` precondition is not read (the fake answers no
  `locked`; the lock is the real compositor's), and every child runs under a scratch HOME and XDG_RUNTIME_DIR so no
  rehearsal reads the real `~/.claude*`. Precondition words: `scratch` · `jq` · `fake-agent` (rehearse only) · `foot` ·
  `hyprctl` · `locked` · `stand-in`. A hand wrapper never live reads `hand=unread` (fixed after the first green:
  the first draft wrote `not-live`, a word outside the plan's set; entries 7 and 8 re-fired green).
- `v2-11` takes no ref: it is gated on R3.

## What stands unproven
- Behind R3 (founder-forks F-D, the operator's word owed): the real raise on Hyprland 0.56.2, for a hand-started and
  a viola-opened session; whether `hl.dsp.focus({window})` alone switches workspace; a special workspace; a
  footclient/foot-server terminal (refuses `window-not-found` by construction). If R3 is granted while R2 is still
  unfired, R3b is the first live `viola open`, and its red reads `opened=open-<code>`.
- Behind the operator's forks F-A to F-E, each built on its PROVISIONAL arm: F-A the widening (CLI-only, no MCP tool),
  F-B the help row `board:    list, focus`, F-C the six refusal pairs' wording, F-D R3, F-E `unsupported` only with
  no hyprctl reachable.

## Surfaced / stuck
- none.

## Process hygiene (`gate.py census --comm foot --comm viola --comm viola-fake-agent --comm hyprctl --args r3-focus --args andromeda-control`, this run dir)
| process | started by | final state |
|---|---|---|
| the rehearsal's stand-in foot, wrappers, fake claude (3 firings of entry 8) | this run (entry 8) | terminated — none in the census |
| the control clone's builds and tests | this run (`gate.py control`) | terminated — `clone removed` |
| `cli_focus` wrappers, stand-in feet, fake agents | this run (entries 5, 11) | terminated — none in the census |
| foot overseer1 / viola-overseer / escher-overseer, the escher-builder foot and its prototype `viola run`, `~/.viola/bin/…/viola mcp`, the product `viola wait viola-builder` | NOT DERIVED — the host's own sessions | left running: not this run's |

Left on disk outside the tree: `$TMPDIR/tmp.FmH6CggFOf/r3`, `$TMPDIR/tmp.OBAmiWi8SY/r3`, `$TMPDIR/tmp.sPoX7PuwAF/r3` (entry 8's own
`mktemp -d`, one per firing).

## Decisions & corrections
- The directive's host read stopped the whole layers: CONTENDED, escher alive in the stall (`inputs#I5`, The host);
  the overseer's quiet window resumed them (`inputs#I6`).
- `hostwatch.py read --to now` is refused (`not a UTC stamp: 'now'`): read `--to $(date -u +%FT%TZ)` instead.
- Bash guard refusals, each re-issued once: a session `cd`; a `cat` heredoc appending to a file (the Edit tool
  anchored on the file's last lines).
- Sweep hazard: none met; the new detail words (`unsupported` · `unreachable` · `oversize` · `malformed` · `refused`)
  were swept over the test tree's planted unknown-word cases, no hit.
