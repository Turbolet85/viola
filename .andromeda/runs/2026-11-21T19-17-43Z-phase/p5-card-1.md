# P5 card 1 — 2026-11-21-mcp-calls-bounded («MCP calls bounded»)

**Open the plan in an editor:** `.andromeda/cache/docs/2026-11-21-mcp-calls-bounded/plan.md` (the render of
`doc:2026-11-21-mcp-calls-bounded/plan` rev 3). No plan stands in a chunk's folder; writing in that file changes no
record until a `put`. A diff of the chunk's records: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py
diff --root . 2026-11-21-mcp-calls-bounded`.

## Goal
Every MCP frame tool (`send · wait · last · answer`) returns by a compiled MCP-side deadline of 55 s
(`MCP_CALL_DEADLINE`), above viola's longest calls (send 48.5 s + paste, wait 50 s) and below the tightest client bound
read from the 2.1.328 and 2.1.330 bundles (the 120 s auto-background); its expiry is `instance-unreachable` /
`call-deadline`, a new named member of `UnreachableCause`. The client's bound is measured live through a new rig arm
`MB`: two long calls (2.1.330 `MB`, 2.1.328 `MB8`) returning whole at viola's 50 s cap, one cut call (2.1.330 `MC`)
cut by a private 5 000 ms per-server `timeout`. The operator's own answers: `inputs#I2` items 1–3.

## Top acceptance criteria
- matrix#v2-24 — the 55 s deadline pinned between the wrapper bounds and the client read by a unit case; its expiry on
  each frame tool through an injected deadline signal (the `call-deadline` control); the three live starts' atoms.
- `call-deadline` a member of `UnreachableCause`, written by the MCP client alone; the error-bodies census green.
- tokio's `time` feature in viola-mcp alone: the channel tokio-free and rmcp-features gates green, no new package.
- the rig's `MB` arm: its reader's cases and its rehearsals green; every start keeps nothing but its ledger lines (kept
  products and the live home stamp unchanged, census 0).

## Touchpoints
New `crates/viola-e2e/src/rig/mcpcall.rs`. Modified: viola-core `lib.rs`; viola-agent-claude `lib.rs`; viola-mcp
`tools.rs`, `lib.rs`, `result.rs`, `Cargo.toml`, `tests/handlers.rs`; `tests/contract_error_bodies.rs`; viola-e2e
`rig/mod.rs`, `bin/viola-live-rig.rs`, `tests/rig_rehearsal.rs`. No master, no fixture, no stamp, no plugin file.

## Test commands (36 entries)
Phase A 1–9 (fmt, clippy, core, agent, viola-mcp, the `call-deadline` control entry, the error-bodies census, the two
tokio/rmcp gates) · phase B 10–13 (rig lib cases, the `MB` rehearsals, `run --unit`, `run --e2e`) · the round 14–26
(`round` and `live` only) · 27 census `operator` · 28 report-only ledger read · 29 `run --integration` · 30–36 the
operator pass. One-shot control `call-deadline` on entry 6, `--timeout 300`.

**Whole-round dry-run** (`gate.py run --chunk 2026-11-21-mcp-calls-bounded --live-legs --dry-run`):
```
 14 probe       not run — dry-run · python3 ~/dev/projects/additional/andromedaV3/andromeda-ov… (103 chars)
 15 probe       not run — dry-run · test -x "$HOME/.local/share/mise/installs/claude/2.1.330/c… (134 chars)
 16 probe       not run — dry-run · test -x "$HOME/.local/share/mise/installs/claude/2.1.328/c… (134 chars)
 17 probe       not run — dry-run · mise ls claude
 18 probe       not run — dry-run · cargo run -q -p viola-e2e --bin viola-live-rig -- census
 19 build       not run — dry-run · CARGO_TARGET_DIR=target/harness cargo build -q --features … (262 chars)
 20 probe       not run — dry-run · f=.andromeda/cache/chunks/2026-11-21-mcp-calls-bounded/evi… (583 chars)
 21 probe       not run — dry-run · f=.andromeda/cache/chunks/2026-11-21-mcp-calls-bounded/evi… (209 chars)
 22 e2e         not run — dry-run · cargo run -q -p viola-e2e --bin viola-live-rig -- live MB … (238 chars)
 23 e2e         not run — dry-run · cargo run -q -p viola-e2e --bin viola-live-rig -- live MB8… (239 chars)
 24 e2e         not run — dry-run · cargo run -q -p viola-e2e --bin viola-live-rig -- live MC … (238 chars)
 25 probe       not run — dry-run · f=.andromeda/cache/chunks/2026-11-21-mcp-calls-bounded/evi… (591 chars)
 26 probe       not run — dry-run · f=.andromeda/cache/chunks/2026-11-21-mcp-calls-bounded/evi… (257 chars)
not run — excluded · 23: 1-13,27-36
round: DRY-RUN · legs 3 · entries 36 · green 0 · red 0 · recorded 0 · timeout 0 · indeterminate 0 · not-run 36
```
The live home's stamp sha is taken right before the round (entry 21) and read after it (entry 26); the kept products
(`HF` `floor-stamp.json`, `HG` `floor-stamp-2.json` + `HG/`, `XS` `cross-stamp.json`, `XS-2` `cross-stamp-2.json` +
`XS-2/`, `CS` `context-stamp.json`, `PQ` `question-stamp.json` + `PQ/`, `WS` `wake-stamp.json`, `R4/`, `R4-2/`, `R4M/`,
from `ls target/live-rig/`) before (20) and after (25), both keyed to the round.

## Baselines (P5, untouched tree, HEAD `53669b68`)
3 green 3.13 s · 5 green 75.91 s · 6 red, 0 selected (vacuous; the `call-deadline` control) · 7 green 9.04 s · 8 green
· 9 green · 11 red, 0 of 600 selected (vacuous; its filter's control selected 1 existing rehearsal in that layer and
passed — `p5-controls.md`) · 28 recorded exit 1 (the ledger absent).

## Checks
Mechanical: sections, paths, the fence (dry-run exit 0), planlint 0 hits, research lists parsed (12 items), inputs 2
unchanged / 0 uncited. **WARN, check 7:** no code-graph trace — research records derived-without-graph (the one changed
symbol `frame` and its four callers read directly). Validation-1: aligned after an intent-incomplete amendment (scope
rev 4: the operator's answer added the live rig arm). Rows: the union's sum 419 797 B, past 250 000 B — the rows files
read by search; every row the plan cites found by `rec.py find`.

## Concretization preview — matrix#v2-24
- **Now:** «The MCP client's tool-call timeout is measured on the current CLI, and every MCP tool call, the longest send
  included, returns below it; a test holds each call's bound.»
- **Proposed:** «On claude 2.1.330 and 2.1.328 a real print-mode client's call of viola's longest tool call (an MCP
  `wait` with no `timeout_ms`, 50 s) returns whole (the rig's `MB` and `MB8` ledger lines: `call-error: false`,
  `timed-out: true`, `elapsed-band: at-cap`), and on 2.1.330 a per-server `timeout` of 5 000 ms is applied as read
  (`MC`: `call-error: true`, `cut-word: tool-timeout`, `elapsed-band: at-bound`); the default's value stays a reading per
  version (1e8 ms hard, 30 min stdio idle, 120 s auto-background, from the bundles' strings); every MCP frame tool
  returns by `MCP_CALL_DEADLINE` 55 s, held below `MCP_CLIENT_BOUND_READ` and above the wrapper's bounds by
  `mcp_call_deadline_sits_between_the_wrapper_bounds_and_the_client_read`, its expiry `call-deadline` held by
  `handlers_frame_tools_end_at_the_call_deadline` and the `call-deadline` control.» «Verified» rests on the three live
  evidence lines plus those unit tests (`inputs#I2` item 3).

## Forks shown before the wrap (founder-forks.md §P4, relay item 10)
- **P4-F1** architecture · contract/unreachable-cause/cause-set — CallDeadline added as one named member, written by
  the MCP client alone (recommended), at the wrap.
- **P4-F2** security-plan · decision/live-rig-mcp-arm-keeps-no-output — a new row for the `MB` arm beside it, the
  ratified shape word for word (recommended); the wrap halts on it once as the answer card (`rul-bznresq5`).

## Beyond the plan
- A route insertion by the wrap's route-resolve: the paste's missing time bound (`crates/viola-pty/src/pump.rs:77`,
  one blocking `write_all`, `inputs#I2` item 9), proposed as a new markerless entry «Paste write bounded» placed in
  Epoch 4 after «Key after a resize».
- Carry `cry-ujze4atl` re-pinned whole on the next entry if its red is not met during this chunk.

Apply? (yes / review / cancel)
