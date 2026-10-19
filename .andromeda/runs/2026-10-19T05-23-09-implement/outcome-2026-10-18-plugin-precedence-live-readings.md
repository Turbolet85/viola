# Implement outcome — 2026-10-18-plugin-precedence-live-readings

**Outcome: green.** Smoke: skipped — no boot-path / UI-surface change (the chunk writes no product code; the plan
lists no smoke entry). Run dir `.andromeda/runs/2026-10-19T05-23-09-implement/`.

## Gates (trail `gate-2026-10-18-plugin-precedence-live-readings.json`)
`entries 12 · green 8 · red 0 · recorded 0 · timeout 0 · not-run 4 · voided 4 (6,7,8,9)`:
1 fmt green · 2 `rig-controls.py` green (`controls: 10 of 10 as expected`) · 3 `cargo build --bin viola` green · 4
`binary(cli_plugin_install)` green · 5 clippy **not run — defer** (zero Rust delta; it reads no file at run time, so
neither void below reaches it) · 6 `run --unit` green, void: `.andromeda/master-route.md` read by
`crates/viola-e2e/src/harness/run/mutants/base.rs` (`gate.py delta --defer-check rust`) · 7 `run --integration`, 8
`run --e2e`, 9 `pre-push` green, void: walk-class rust 4 marked walkers (`tests/cli_controls_not_disableable.rs`,
`tests/contract_*.rs`), 60 uncommitted · 10–12 operator leg (the operator pass).
Deferral for the wrap's pin: clippy (entry 5), deferred since this chunk (zero Rust delta).

## The live round (`evidence/live-round.md`, `evidence/live-sessions.ndjson`)
3 starts of the founder's cap 3 (`inputs#I2` F2), each ledgered before it; 1 red, never re-fired; `claude plugin`
commands 4 of 4 in rig A and 4 of a fresh 4 in rig B (`inputs#I4`).
- **Precedence (start 2, wrapped):** only the `--plugin-dir` server started and answered `list` (`whome` `mcp.ndjson`:
  1 `process-start{subject:"self"}` + the `agents-probe` pair; `ihome`: no diagnostics file). The CLI logged one
  `plugin-viola-viola` connection. 1 `session-start`, 1 `hook-invoked{hook_event:"session-start"}`.
- **Exit-1 (start 3, unwrapped):** not blocking — the start's agents row read `idle`; the control exited 1 with both
  streams empty; `<ihome>` stayed 0770 with no `mcp.ndjson`; the CLI logged `Connection failed (CONNECTION_CLOSED)`.
- **Start 1 (red):** the arrow rule passed the trust dialog (default pointer one row above the trust row, 1 arrow, 1
  Enter), then a second modal, `Yes, allow external imports`, held the start (no key, F1). Inferred cause: `proj/`
  inside this work tree, whose `CLAUDE.md` imports `@.claude/session-handoff.md` from outside `proj/`'s git root.
- Residual: the CLI's orphaned plugin cache rewritten (same sizes), an unattributed `synced/` pair, two
  `~/.claude.json` trust entries (the CLI's writes on the accepted dialogs), one `~/.claude/projects/` dir.

## Matrix
`v1-42` planned → **implemented**, ref `tests/cli_plugin_install.rs` (gate entry 4) + `evidence/live-round.md` start 2
(trail `matrix-2026-10-18-plugin-precedence-live-readings.json`).

## Files changed
New, all under `viola-0.1.0/chunks/2026-10-18-plugin-precedence-live-readings/evidence/`: `live-pty.py`,
`live-ledger.py` (both `cmp` 0 against the plugin chunk's), `live-pty-arrow.py`, `rig-controls.py`, `live-round.md`,
`live-sessions.ndjson`. Inputs: `inputs#I4` (`relay-1.md`, snapped). Ledger: `viola-0.1.0/verification-matrix.json`
(v1-42). Modified product files: none. `gate.py scope`: `clean` (scope-record.md not needed).

## Deviations from plan, with justifications
1. **Rig B outside the work tree (plan steps 3–8).** Starts 2 and 3 ran from a fresh `proj/` on the e2e-home tmpfs
   (its own git root, no ancestor `CLAUDE.md`) with a fresh four `claude plugin` commands, not from the plan's
   `target/plugin-live-<stamp>/proj/`; start 2 took the precedence reading and start 3 the exit-1 reading, so no spare
   remained. Justification: start 1 met the external-imports modal from the plan's location; the founder's own word
   «Стенд вне репо» (`inputs#I4`, relayed by the operator), which also ordered rig A's removal first.
2. **A sixth verdict word, `unselected`** (the trust row with no single pointer row: no key, red). The plan's step 1
   list names no word for that screen; F1's "anything else no key and red" decides it. No start read it.
3. **The matrix write despite plan step 12** ("Matrix and route records are P5's and the wrap's, never
   implement's"): the implement contract's P2 sets `ref` + `implemented` on green for the chunk's claimed cap; the
   write was made through `matrix.py implement` and is surfaced here for the wrap.
4. **Waits by script, not by hand:** the 90 s `session-start` wait and start 3's "20 s after the first `absent`
   read" were bounded scratchpad scripts (`waitfor.py`, `s3wait.py`); start 3 read `absent` at its first read (no
   dialog), so its 20 s ran from there.

## Surfaced (for the wrap; nothing authored)
- The plan's premise "a start passes once the trust dialog is accepted" is falsified for a `proj/` inside this work
  tree on 2.1.287 (the external-imports modal), recorded in `evidence/live-round.md`.
- `viola mcp` ends with the CLI session without a `process-exit{subject:"self"}` line in `mcp.ndjson` (start 2) — a
  reading against obs-plan's process pair, for the wrap to weigh.

## Process hygiene
| process | started by | final state |
|---|---|---|
| `viola run driver` + its `claude` child, start 1 (rig A) | this run (the rig) | terminated — stop file, 3 Ctrl-C, exit 0 |
| `viola run driver` + its `claude` child + the `--plugin-dir` `viola mcp`, start 2 (rig B) | this run (the rig) | terminated — stop file, 3 Ctrl-C, exit 0 |
| `claude` (unwrapped), start 3 (rig B) | this run (the rig) | terminated — stop file, 3 Ctrl-C, exit 0 |
| the `viola mcp` control (start 3) | this run | exited 1 at once |
| the gate block's harness runs | a leg of this run (gate.py) | terminated (no survivors on any line) |
`gate.py census` (args: both rig dirs, both home dirs, `projects/viola/target/debug/viola`, `live-pty-arrow.py`,
`viola-harness`; trail `census-2026-10-18-plugin-precedence-live-readings.json`): `census: none running`.

## Decisions & corrections
- The operator's correction at invocation: this chunk's cap is 3 starts (`inputs#I2` F2), not the previous chunk's 12
  the session's dashboard had named — applied.
- The founder's ruling «Стенд вне репо» (`inputs#I4`) after start 1 — applied as deviation 1.
- Guard refusals: the Bash guard refused a `cd` that moved the cwd (re-issued with an absolute path), once.
- `hyprctl locked` read `true` before each start and did not stop the round (the plan's lean: the rig types no desktop
  key; host-linux.md 2026-10-08 stops a step that focuses a window or types with `wtype`).

## Next
The operator pass (plan entries 10–12, `.claude/rules/ci.md` order), then `/andromeda-wrap-session`.
