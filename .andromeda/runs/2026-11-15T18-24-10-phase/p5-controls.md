# P5 controls — chunk 2026-11-15-mutation-and-coverage-baselines-green

Every control fired AS THE ENTRY through `gate.py run --chunk 2026-11-15-mutation-and-coverage-baselines-green
--docs-root . --entry {n} --project-root {clone}` over one scratch clone, `git clone --shared` of the project at HEAD
`73b06303`: `$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/c1108f72-f24c-4690-b82a-a7327bbcf798/scratchpad/ctl-clone`
(role: the P5 control clone). Inputs lived in the clone only; the tree was not edited.

| entry | input in the clone | reading |
|---|---|---|
| 8 (`WITHIN` 12 s grep) | `tests/support/watch.rs` carrying `from_secs(12)` | green · exit 0 · last line 1 (must-pass) |
| 9 (the 15 s kill grep) | `.config/nextest.toml` `[profile.mutants]` at `terminate-after = 3` | green · exit 0 · last line 1 (must-pass) |
| 15, first spelling `crates/*/src` | the recorder file changed alone | green · last line 0 |
| 15, first spelling `crates/*/src` | plus `crates/viola-hyprland/src/lib.rs` changed | green · last line 0 — DOES NOT HOLD (a wildcard pathspec matches whole paths, never a directory prefix) |
| 15, corrected `crates/*/src/*` | the recorder file changed alone | green · last line 0 (must-pass) |
| 15, corrected `crates/*/src/*` | plus `crates/viola-hyprland/src/lib.rs` changed | red · last line 1 (must-fail) |

Entry 15's spelling was corrected in the plan (rev 3) and its tree baseline re-fired: green, last line 0.
