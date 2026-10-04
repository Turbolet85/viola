# Operator pass — 2026-10-04-readiness-gate-and-timing-constants

Run 2026-10-04 on the overseer's word ("Run the operator pass now (entries 23-25 …); windows must be green as the
passthrough witness. Fold any red now. Report and stop before the wrap."), on the Linux dev host. The native
`pre-push` (entry 22) read `ok:true`, stage `linux-tests`, on this tree at /implement (gate trail
`.andromeda/runs/2026-10-04T04-59-18-implement/`, both full runs); only evidence and run-dir files changed after it.

| entry | command (as the plan lists it) | exit | reading |
|---|---|---|---|
| 23 | `python -X utf8 …/andromeda-tools/scripts/gate.py hygiene` | 0 | `hygiene: clean — read 37 (runs 34 · evidence 3) · trails 13 not read · binary 0 not read by P1` |
| — | the pre-CI commit (`git add -A`, then `chore(2026-10-04-readiness-gate-and-timing-constants): operator pre-CI commit, for the run this chunk's verdict reads`) | 0 | `afef92f8885f` on `build/viola-0.1.0` |
| 24 | `git diff --quiet && git diff --cached --quiet && git push origin HEAD` | 0 | `5bcb0a0..afef92f  HEAD -> build/viola-0.1.0`; 0 ahead after |
| 25 | `python -X utf8 …/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` | 0 | `afef92f8885f verdict: green · checks 15/15 · wall 364 s · runs ci#37179459192 completed/success` |

## The three-OS passthrough witness (ci#37179459192)
Every job `success` (15/15), `fuzz-replay` included. Read from each test job's log (`gh run view --job <id> --log`):

| job | `tui_passthrough` PASS / FAIL | `run::gate::tests` PASS | `screen::tests` PASS |
|---|---|---|---|
| `test (windows-2025)` 111368878617 | 5 / 0 | 3 | 21 |
| `test (macos-latest)` 111368878618 | 5 / 0 | 3 | 21 |
| `test (ubuntu-latest)` 111368878653 | 5 / 0 | 3 | 21 |

The five Windows cases: `tui_child_output_passes_through_without_viola_bytes`, `tui_hooks_firing_add_no_viola_bytes`,
`tui_keys_reach_the_child_as_typed_and_ctrl_c_ends_it`, `tui_host_resize_reaches_the_child`,
`tui_host_resize_in_the_pump_start_window_reaches_the_child` — each driving the built `viola run` with the tee and the
feed thread on its pump. No red was met, so nothing was folded; no fix commit sits above the pre-CI commit.

## Overseer direction for the wrap
"route the unbounded tee mpsc to :74 as a named item ("bound every input")" — the tee → feed channel
(`src/run/gate.rs` `start`, `mpsc::channel`) is unbounded as the plan specified; `:74` (Confirmed send, the gate's
first verdict consumer) owns bounding it.
