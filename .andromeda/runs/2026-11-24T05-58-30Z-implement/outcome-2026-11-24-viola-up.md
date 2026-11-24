# Outcome — implement 2026-11-24-viola-up (re-entry, plan rev 10)

**Written:** 2026-11-24T06:02:42Z · **Run:** 2026-11-24T05-58-30Z-implement

## Outcome: green

Step 21 is done. Entry 16 (`ctl-fds`) is green, and its `fds-held-red` control holds. The three whole layers
(entries 13–15) and the linter are green on the final tree. The scope reading is clean.

The open `tui_channel_fds` red of run `2026-11-24T05-12-02Z-implement` was folded into this chunk through the
revision door (plan rev 10, step 21). It is closed by its fix, not by a green re-run. The case had read a launch-time
open of `hooks.json` as a hold. It now reads a whole table again while a non-fixture home file is open, inside its
2 s deadline. The control shows the guard still bites: a fake that holds the file for its life reads the case red.

matrix#v2-12 stays GATED on R4, with no ref. The gated entry is deferred, not voided.

**Next:** the overseer's word on the pre-push red (below). Then the rest of the pass, still without R4: hygiene, the
pre-CI commit and the push, and `ci.py conclusion`. None of R4's entries is fired: the two builds, R4a, and R4b's
plant and after.

## The operator pass (after the report landed, rev 2) — STOPPED RED at pre-push
Recorded at 2026-11-24T06:06:15Z, from `evidence/operator-pass.md` and `evidence/operator-18-pre-push.txt`.
- `bash scripts/agent-run.sh pre-push` was red: exit 1, 96.15 s, history unmoved.
  - Every test passed (4320 of 4320). The coverage merge failed: `llvm-profdata` refused
    `viola-3061597-…_29.profraw` (32232 B, written 06:04:53Z) for a corrupt header.
  - The profile's module signature is the fake agent's (it plays `claude`, `foot` and `hyprctl`). No record names
    the test that started pid 3061597.
  - Host QUIET over the firing. Cause NOT KNOWN. Not re-run.
- Not fired: `gate.py hygiene` (entry 19), the pre-CI commit, the push, and the CI read.
- The overseer answered (`inputs#I11`): measure first. The census (`evidence/profraw-census.md`, 192 runs over 16
  workers per case):
  - this chunk's cases: 7 of 14 refused profiles, 54 in all;
  - the `cli_open` control, untouched by this chunk: 9 and 6.
  The red is not shown to be this chunk's (step 4 of `inputs#I11`). No mend; STOPPED, waiting on the overseer. The
  corrupt file stays at `target/llvm-cov-target/viola-3061597-5781154654293742489_29.profraw`.

## Files changed (this run)
Modified: `tests/tui_channel_fds.rs` (step 21; listed by the plan's revision).
The chunk's earlier files stand as run `2026-11-24T05-12-02Z-implement` left them.

## Gates (this run; trail `.andromeda/runs/2026-11-24T05-58-30Z-implement/gate-2026-11-24-viola-up.json`)
- `cargo fmt --all --check` green.
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` green. It took 0.34 s, cached: the
  edit hook's own lint had already built the edited file. pre-push re-runs it under `env -i`.
- `bash scripts/agent-run.sh run --e2e --filter 'binary(tui_channel_fds)'` (ctl-fds) green: 1 test across 1 binary,
  84 binaries skipped, passed.
- `bash scripts/agent-run.sh run --unit` green.
- `bash scripts/agent-run.sh run --integration` green.
- `bash scripts/agent-run.sh run --e2e` green.
- Every other non-operator, non-defer entry was not re-fired. Each last read green on run
  `2026-11-24T05-12-02Z-implement`'s tree. This run's only edit is `tests/tui_channel_fds.rs`, which none of those
  entries selects.
- The gated `evidence/r4.md` entry: not run — defer (gated: R4); no `--void`.
- Operator entries: next, in this window (R4's excluded).

Control (`evidence/control-2.txt`, `--timeout 1800` on entry 16): `control fds-held-red · entry 16 · holds — in
place green, removed red`. The removed log reads `fd 4 holds a viola home file`, the case's own assertion.
`control: 1 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`.

Host reads: `hostwatch.py read --last 15` QUIET twice (05:59Z, 06:00Z), 0 s stalled. The only build alive was this
repository's own. `df -m /tmp`: 31070 MB free.

## Deviations (with justification)
- No deviation from step 21. The settle wait is bounded by the case's existing 2 s deadline, and every assertion is
  unchanged. Where no whole table was ever read by the deadline, the case still panics «no consistent fd table
  within 2 s».
- The control's `fds-held-red` edit is `std::mem::forget` of an open of `<plugin dir>/hooks/hooks.json` at the head
  of `launch`, as the plan's table names it. It is written in `control-1.toml` and fired in the tool's clone, never
  in the tree.
- Scope record, `gate.py scope`: `clean — changed 23 · listed 20 · recorded 3 (companion 2 · mechanical 0 ·
  in-intent 1 · widening 0)`. These are the three lines the earlier run recorded; this run adds none.

## Unproven — behind R4 and the operator's forks
Unchanged from run `2026-11-24T05-12-02Z-implement`:
- R4a and R4b have not run, and no real window, compositor or `claude` was touched.
- R4b refuses until a revision built on U6.
- U1 and U3–U8 stand PROVISIONAL. U2 is the operator's own word, arm 1 (`inputs#I6` item 3).
- U8 is read from the code only.
- Unmeasured live: SessionEnd on a graceful shutdown, real foot windows after a compositor restart, and a real
  `claude --resume` after a reboot.

## Process hygiene
| process | started by | final state |
|---|---|---|
| the ctl-fds and layer test wrappers, fake agents, stand-in `foot` and `hyprctl` | this run (its gate entries) | terminated (census: none of this run's left) |
| the control's clone build and firings | this run (`gate.py control`) | terminated, clone removed |
| 8 listed by `gate.py census`: the overseer feet (overseer1, viola-overseer, escher-overseer), escher-builder's foot and wrapper, `~/.viola`'s `viola mcp`, the product's `viola wait viola-builder`, the prototype's `viola wait escher-builder` | NOT DERIVED: not this run's | left running — the operator's fleet |

Left on disk outside the tree: nothing new from this run. The earlier run's leftovers stand: `$TMPDIR/tmp.6JQv5Ve6YK`
and `$TMPDIR/tmp.meET73vc55`, `target/r4/rehearse-20261124T052802Z` and `target/r4/rehearse-20261124T053311Z`, and the
lock files in `$XDG_RUNTIME_DIR/viola/`.

## Decisions & corrections
- The `tui_channel_fds` red is closed by its fix (step 21) and its control, never by a green re-fire. The rule is
  CLAUDE.md's: a red stays open until its cause is known.
- The run dir's name `2026-11-24T05-58-30Z` was set at the mkdir from a `date -u` read 40 s earlier (05:57:50Z), not
  read at the moment of the mkdir. Every stamp written inside a record was read from `date -u` when it was written.
- Guard refusals: none.
