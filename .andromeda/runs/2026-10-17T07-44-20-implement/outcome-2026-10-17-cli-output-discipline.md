# Implement outcome — 2026-10-17-cli-output-discipline

**Outcome: green + smoke.** The whole `## Test Commands` block read `entries 21 · green 18 · red 0 · recorded 0 ·
timeout 0 · not-run 3` on its first run (trail `.andromeda/runs/2026-10-17T07-44-20-implement/gate-2026-10-17-cli-output-discipline.json`);
the three `not run — leg operator` entries (19 hygiene, 20 push, 21 CI read) are the operator pass's. No source-delta
deferral (every language touched; walk-class rust 4 · uncommitted 73 voids none, nothing deferred). Smoke fired
(boot path: `viola --help`, `viola list`). `v1-40` → `implemented` with its ref (matrix trail in this run dir). No
live `claude` start was made. No mutation run of any kind was made.

Next → the OPERATOR PASS (`.claude/rules/ci.md`): entry 19 (`gate.py hygiene`, as written), the pre-CI commit,
entry 20 (the push, through `gate.py run --operator 20`), entry 21 (`ci.py conclusion`, as written); then
`/andromeda-wrap-session`.

## Files changed (14, all on research's lists; `gate.py scope`: clean — changed 14 · listed 14 · recorded 0)
- New: `src/human/style.rs` (tokens, `Depth`, `Facts`, `decide`, the fact reader, the Windows VT enable and its
  restoring guard, the `colour_` units); `tests/support/sgr.rs` (`sgr_attributes` moved, plus `redraws` and
  `sets_a_foreground`).
- Modified: `src/cmd/mod.rs` (usage override + grouped help template); `src/human.rs` (`mod style`, `read_colour`);
  `src/human/board.rs` (width + depth parameters, the unwrapped-NAME cut, the spans, `board_row_width_` /
  `board_row_style_` units, the six standing callers updated); `src/cmd/list.rs` (`host_size()` width, colour
  decision, VT guard held across the one write; `--json` untouched); `tests/cli_output_plain.rs`;
  `tests/support/mod.rs`; `tests/support/events.rs` (`raise` moved); `tests/cli_answer.rs` (imports `raise`, no case
  changed); `tests/cross_list_rows.rs` (styled, no-SGR and truncation cases; `NO_COLOR` on the standing case's
  outer-PTY mode); `tests/cli_wait_last.rs`; `tests/cli_controls_not_disableable.rs`;
  `.github/workflows/windows-mutants.yml` (`src/human/style.rs` under the `viola-main` label).

## Red readings (recorded before each fix)
- `evidence/red-01-help-on-the-current-tree.txt` — step 1/2 red on the current tree (clap's `Commands:` help). The
  same output showed the test's own `\n\` continuation literal dropping the `Options:` lines' leading spaces: the
  literal was rewritten with `concat!` before any product change; the `Options:` lines copied from that output.
- `evidence/red-02-units-on-stubs.txt` — steps 5, 8, 10 on stubs: 81 run, 34 failed; the absence cases passed.
- `evidence/red-03-remove-the-guard.txt` — 13 controls, each red on its own case(s), each restored byte for byte. The
  paste-control control's first firing read all 8 cases red (the host's `COLORTERM=truecolor` was inherited by every
  case); `assert_send_paste_control_holds` now removes `NO_COLOR` / `TERM` / `COLORTERM` before setting the case's
  own; the second firing read exactly the three new cases red.

## Deviations from plan, with justifications
1. **Step 13, the self-check case.** `sgr_attributes` moved to `tests/support/sgr.rs`, but its self-check case stays
   in `tests/cli_output_plain.rs` (calling the support reader): a `#[test]` inside `tests/support/` compiles into
   every binary declaring `mod support` and would run in each of them. The case also checks the new
   `sets_a_foreground`.
2. **Step 14, the Windows exact sequence.** The plan says the `windows-2025` styled reading's exact sequence is copied
   from the first `windows-2025` run. That run cannot be made from this Linux host. The Windows leg asserts the
   parsed attribute set (holds `1`, sets a foreground, lacks `2`) as the plan's first sentence says; the exact
   sequence is not pinned. SURFACED for the wrap: pinning it needs the CI job log of the operator pass's run.
3. **Step 15, the helper's environment.** Beyond the three cases, `assert_send_paste_control_holds` stops inheriting
   `NO_COLOR` / `TERM` / `COLORTERM` (listed file, in-intent): without it the remove-the-guard run could not tell the
   colour cases from `no_setting` on a host whose terminal sets `COLORTERM`.
4. **Step 18, the CR reading.** It is a shared `redraws` helper in `tests/support/sgr.rs`, used by
   `wait_human_lines_are_linear` and `help_is_plain_on_a_terminal` (Unix only).
5. **Units beyond the plan's list.** `colour_decision_a_terminal_with_every_rung_passing_is_coloured` (the positive
   beside the six rungs) and `colour_vt_mode_adds_only_the_processing_bit` (pins `0x0004` alone, never `0x000c`;
   testing.md 2026-09-27). `vt_mode` and its constant are `cfg(any(windows, test))`, so no dead code on Unix and no
   `#[allow]`.
6. **`Colour` re-export.** `human.rs` re-exports only `read_colour`; `Colour` is never named outside `human`.
7. **The terminal values** are read with `to_string_lossy` (a non-UTF-8 `TERM` reads as its lossy text, never as
   `dumb` or `256color` unless it holds them).

## Surfaced
- Deviation 2 above: the Windows exact SGR sequence is unpinned; the attribute-set reading stands.
- No spec↔reality gap in the product. The plan's Expected amendments (wrap) stand as written; as built, the
  `NO_COLOR` / `TERM` / `COLORTERM` reads live only in `src/human/style.rs::read`.

## Process hygiene
| process | started by | final state |
|---|---|---|
| gate block's smoke session `p-cod-smoke` (supervisor, 2 wrappers, 2 fake agents) | a leg (entries 13 boot, 17 cleanup) | terminated — entry 17 green, `processes_gone:true`, `endpoint_gone:true` |
| P3 smoke session `p-cod-p3` (supervisor, 2 wrappers, 2 fake agents) | this run | terminated — `cleanup` `processes_gone:true`, `endpoint_gone:true`, `home_removed:true` |
| the remove-the-guard batches (`controls.py`, `controls2.py`) and their `agent-run.sh run` children | this run | terminated — both exited 0 |
| the backgrounded gate block and its wait loop | this run | terminated — both exited 0 |
| `script(1)` + the harness `viola list` under it | this run | terminated (exited with the command) |
| one ad hoc `target/debug/viola --home $TMPDIR/{scratchpad}/nohome list` | this run | terminated; it spawned the host's `claude agents --json` once (a read, no session started) |

Measured after cleanup with `ps -eo pid,comm,args`: every running `viola` on the host is the overseer bridge's
prototype build (`~/dev/projects/additional/viola-lab/prototype/target/debug/viola`), none this repository's.

## Decisions & corrections
- Guard refusal: the Bash guard refused a `cd` into the chunk's `inputs/` dir (re-issued with absolute paths) and a
  `cat` heredoc into a scratch file (re-issued through the Edit tool).
- Measurement on this host: an ad hoc `viola list` run from the session's own shell, with no test `PATH`, reads the
  host's real `claude agents --json` (11 live sessions listed). Probes of `list` outside the test suite put the fake
  agent first on `PATH` (testing.md's `empty_bin` / `claude_bin` rule binds tests; it bound this probe too).
- Measurement: a test that runs `viola` without clearing `NO_COLOR` / `TERM` / `COLORTERM` takes the host terminal's
  values (`COLORTERM=truecolor` here); every colour-sensitive case sets or removes them per child.
- Hazard met (no standing learning names it): a Rust `\n\` string continuation strips the next line's leading whitespace, so a
  literal of indented output lines is written with `concat!` (caught on the first red reading).
