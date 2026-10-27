# Apply — obs-plan · part 1 of 6

## Verdicts
- fanout-obs-plan.md 5 · applied · .andromeda/obs-plan.md:119#1
- fanout-obs-plan.md 6 · applied · .andromeda/obs-plan.md:215#1
- fanout-obs-plan.md 7 · applied · .andromeda/obs-plan.md:311#1
- fanout-obs-plan.md 8 · applied · .andromeda/obs-plan.md:311#2, .andromeda/obs-plan.md:311#3, .andromeda/obs-plan.md:311#4
- fanout-obs-plan.md 9 · applied · .andromeda/obs-plan.md:361#1
- fanout-obs-plan--cascade-g3.md 19 · folded into fanout-obs-plan.md 9
- fanout-obs-plan.md 10 · applied · .andromeda/obs-plan.md:537#1
- fanout-obs-plan.md 11 · applied · .andromeda/obs-plan.md:394#1
- fanout-obs-plan.md 12 · applied · .andromeda/obs-plan.md:400#1

## Edits
- line: .andromeda/obs-plan.md:119
  old: «without `VIOLA_NAME` the hook is a silent no-op;»
  new: «without `VIOLA_NAME`, on an instance the instance check refuses, or on a `VIOLA_DIR` naming an instance directory that does not exist, the hook is a silent no-op (exit 0, no file opened, no line, no frame);»
- line: .andromeda/obs-plan.md:215
  old: «a strict-modes failure has a line only on the `ledger-stamps` read in `run`'s version gate.»
  new: «a strict-modes failure has a line only on the `ledger-stamps` read in `run`'s version gate and on a `conpty/` companion its owner/DACL check refuses (`process-start`'s `sideload_fallback:"strict-modes-failed"`, the child on the inbox ConPTY, exit unchanged); a refused `diagnostics/` folder or file is a failed open, its line dropped.»
- line: .andromeda/obs-plan.md:311
  old: «called by `run` (5 sites), `verify` (5), `list` (2), `revive` (1), `plugin install` (1) and `link` (1, the one site `viola link` / `viola unlink` share), as re-derived by `grep -rcE 'human::refuse\(' src` at chunk 2026-10-26-explicit-session-links»
  new: «called by `run` (7 sites), `verify` (6), `list` (2), `revive` (1), `plugin install` (1) and `client` (1, `client::refuse_untrusted`, the one site `viola link` / `viola unlink` and the six frame verbs `send`, `wait`, `last`, `answer`, `pause` and `release` share), as re-derived by `grep -rcE 'human::refuse\(' src` at chunk 2026-10-26-home-and-code-bearing-file-integrity»
- line: .andromeda/obs-plan.md:311
  old: «tampered pinned copy, squatted endpoint)»
  new: «tampered pinned copy, squatted endpoint, a home tree another user could write or read: the home pair for a home-level refusal, the instance pair for an instance-level one, before any log opens)»
- line: .andromeda/obs-plan.md:311
  old: «its start arm logs as process `run`)»
  new: «its start arm logs as process `run` and makes `run`'s home-tree check before that log)»
- line: .andromeda/obs-plan.md:311
  old: «to `run`'s four:»
  new: «to `run`'s five:»
- line: .andromeda/obs-plan.md:361
  old: «; `strict-modes-failed` cannot occur on them, its check owed to the route entry "Home and code-bearing file integrity".»
  new: «; first, a home or instance tree the instance check refuses ends a CLI `wait` / `last` before any role file opens (exit 21, the instance pair, or exactly `{"v":1,"error":"instance-unreachable","detail":null}` under `--json`; no line), and MCP's returns `instance-unreachable` with no line.»
- line: .andromeda/obs-plan.md:537
  old: «`instance-unreachable` — the liveness pre-check's, or a frame tool's server-verification mismatch on Linux and Windows —»
  new: «`instance-unreachable` — a frame tool's refused instance check (made before the liveness pre-check), the liveness pre-check's, or a frame tool's server-verification mismatch on Linux and Windows —»
- line: .andromeda/obs-plan.md:394
  old: «one line per call; the other snapshot readers write no `state-recovered` line.»
  new: «one line per call; its six product readers are `revive` (one read for both its collision check and its recorded-directory reading), `list`, the hook `connect`, the statusline `serve`, `run`'s start (one read handed to its collision check) and `liveness::live_endpoint`, and `read_snapshot` has no product caller under `src/`; over a replayed or absent read `live_endpoint` and the hook `connect` give no endpoint, so no frame, and the statusline runs no command.»
- line: .andromeda/obs-plan.md:400
  old: «in CI `viola verify` runs only against the fake agent.»
  new: «in CI `viola verify` runs only against the fake agent; before its log `viola verify` holds the home, `ledger/`, `bin/` and `plugin/` to the strict-modes check, and a refused tree is exit 1 with the home pair and no line, while a `ledger/stamps.json` its stamp write refuses prints the same pair and exits 1 through its own `process-exit`.»

## Patterns
[[pattern]]
id = "obs1-01"
fixed = "a strict-modes failure has a line only on the"

[[pattern]]
id = "obs1-02"
fixed = "`run` (5 sites), `verify` (5)"

[[pattern]]
id = "obs1-03"
fixed = "tampered pinned copy, squatted endpoint)"

[[pattern]]
id = "obs1-04"
fixed = "three refusals to `run`'s four"

[[pattern]]
id = "obs1-05"
fixed = "`strict-modes-failed` cannot occur on them"

[[pattern]]
id = "obs1-06"
fixed = "the liveness pre-check's, or a frame tool's server-verification mismatch"

[[pattern]]
id = "obs1-07"
fixed = "the other snapshot readers write no `state-recovered` line"
