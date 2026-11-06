@apply_row n=1 ref_file=fanout-test-plan--cascade-g2.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-test-plan--cascade-g2.md ref_n=2 v=applied edit=E2
@apply_row n=3 ref_file=fanout-test-plan--cascade-g2.md ref_n=3 v=folded_into to_file=fanout-test-plan--cascade-g2.md to_n=2
@apply_row n=4 ref_file=fanout-test-plan--cascade-g2.md ref_n=4 v=folded_into to_file=fanout-test-plan--cascade-g2.md to_n=2
@apply_row n=5 ref_file=fanout-test-plan--cascade-g3.md ref_n=1 v=applied edit=E3
@apply_row n=6 ref_file=fanout-test-plan--cascade-g3.md ref_n=2 v=applied edit=E4
@apply_row n=7 ref_file=fanout-test-plan--cascade-g3.md ref_n=3 v=applied edit=E5
@apply_row n=8 ref_file=fanout-test-plan--cascade-g3.md ref_n=4 v=applied edit=E6
@apply_row n=9 ref_file=fanout-test-plan--cascade-g3.md ref_n=5 v=refused
why: a hand removal · fct-hhwc4gim (the Single writers sub-line naming the `CreateNamedPipeW` controls is a whole line an in-line change cannot empty, and the fact takes no whole rewrite)
@apply_row n=10 ref_file=fanout-test-plan--cascade-g3.md ref_n=6 v=applied edit=E7
@apply_row n=11 ref_file=fanout-test-plan--cascade-g3.md ref_n=7 v=applied edit=E8
@apply_row n=12 ref_file=fanout-test-plan--cascade-g4.md ref_n=1 v=applied edit=E9
@apply_row n=13 ref_file=fanout-test-plan--cascade-g4.md ref_n=2 v=applied edit=E10
@apply_row n=14 ref_file=fanout-test-plan--cascade-g4.md ref_n=3 v=applied edit=E11
@apply_row n=15 ref_file=fanout-test-plan--cascade-g4.md ref_n=4 v=applied edit=E12
@apply_row n=16 ref_file=fanout-test-plan--cascade-g4.md ref_n=5 v=applied edit=E13
@fact id=fct-vmzc7an2 expect_rev=2
old: portable-pty `=0.8.1` via viola-pty seam
new: portable-pty `=0.9.0` via viola-pty seam

@fact id=fct-lzdm6e7w expect_rev=4
old: units `resolve_home_*`, `canonical_home_*` and `plain_drive_*`;
new: units `resolve_home_*` and `canonical_home_*`;
old: read as `viola` or `viola.exe` followed by
new: read as `viola` followed by
old: `tests/cli_slash_args.rs` (Vector 6, §6 rewritten-path warning):
new: `tests/cli_slash_args.rs` (Vector 6: a leading-slash argument is never taken as a prompt):

@fact id=fct-4m5t4osf expect_rev=3
old: trycmd 1.2.1 for human layouts other than the `viola list` board, `viola --help` and the `viola link` / `viola unlink` result lines, pinned by literal in `tests/cli_list.rs`, `tests/cli_output_plain.rs` and `tests/cli_links.rs` (`link_verbs_print_the_design_lines`)
new: human layouts pinned by literal in their test files (the `viola list` board in `tests/cli_list.rs`, `viola --help` in `tests/cli_output_plain.rs`, the `viola link` / `viola unlink` result lines in `tests/cli_links.rs` `link_verbs_print_the_design_lines`)

@fact id=fct-tr6c4sml expect_rev=2
old: portable-pty `=0.8.1` outer PTY
new: portable-pty `=0.9.0` outer PTY

@fact id=fct-qbytqn26 expect_rev=2
old: Human mode, verified with trycmd-redacted assert_cmd:
new: Human mode, verified against literal lines in the test file:

@fact id=fct-ja62ftpj expect_rev=2
old: and portable-pty's `close_random_fds()` closes every fd above 2 at spawn, so a planted leak cannot go red either way.
new: and portable-pty closes every inherited descriptor in the child (`close_random_fds()`, every fd above 2 at spawn), so a planted leak cannot go red either way. As measured on live `claude` 2.1.287 on the Linux dev host at chunk 2026-11-06-version-done-check (`evidence/live-round.md`; a local live measurement, no suite): the wrapped child's `/proc/<child>/fd` held 31 fds (pty 7, socket 14, other 10), fds 0–2 on the pty, no socket that is viola's endpoint or the wrapper's, and no file under the home.

@fact after=fct-sn6e6lxw as=n1
- **The planted annotation drift:** `dialog_drift_names_a_changed_annotation` (`tests/contract_fake_agent_drift.rs`) plants a change to a forwarded `annotations` byte of `synthetic_dialog_set` and reads `dialog_drift` red.

@fact id=fct-6wjlyq3w expect_rev=1
old: | trycmd `tests/cmd/*.toml`, with snapbox redactions for timestamps, cursors and paths; `viola --help` (the `HELP` literal
new: | `viola --help` (the `HELP` literal

@fact id=fct-63urz52n expect_rev=2
old: shared through `tests/support/mod.rs`; on Unix a `SIGSTOP`)
new: shared through `tests/support/mod.rs`; a `SIGSTOP`)

@fact id=fct-bfdvmti3 expect_rev=1
old: **Parallel:** per OS
new: **Parallel:** one-leg matrix `[ubuntu-latest]` (the job name keeps its `${{ matrix.os }}` form)

@fact id=fct-2qyicniz expect_rev=1
old: **Parallel:** per OS
new: **Parallel:** one `ubuntu-latest` leg

@fact id=fct-zwsm4wxk expect_rev=1
old: **The recorded supply-chain red:** the Supply-chain stage's `supply-chain` job reads red on one advisory, RUSTSEC-2026-0441 (`pin-utils` unmaintained, through `portable-pty =0.8.1` → `nix 0.25.1`), as measured at ci#42030025050; owner: the CARRY on «Version done-check», its remedy moving `portable-pty` off 0.8.1 to a release whose graph drops `pin-utils`; the ignore list is not widened.
new: **The supply-chain advisories:** the Supply-chain stage's `supply-chain` job reads green with no advisory ignored: `deny.toml` `[advisories]` reads `ignore = []`, and `portable-pty` `=0.9.0` (through `nix 0.28.0`) puts no `pin-utils` in the graph (`cargo tree -i pin-utils` exit 101, no package), as measured at ci#42288148289.

@fact id=fct-ocdr3xdn expect_rev=2
old: The mutation audit reads that same survivor at the route entry "Version done-check".
new: That survivor stands as version 0.1.0's known limit, read at each epoch-boundary audit (matrix#v1-44).

@pattern id=tp2-01
fixed: portable-pty `=0.8.1`
@pattern id=tp2-02
fixed: `canonical_home_*` and `plain_drive_*`
@pattern id=tp2-03
fixed: or `viola.exe` followed by
@pattern id=tp2-04
fixed: §6 rewritten-path warning
@pattern id=tp2-05
fixed: trycmd 1.2.1 for human layouts
@pattern id=tp2-06
fixed: verified with trycmd-redacted assert_cmd
@pattern id=tp2-07
fixed: portable-pty's `close_random_fds()` closes every fd above 2
@pattern id=tp2-08
fixed: trycmd `tests/cmd/*.toml`, with snapbox redactions
@pattern id=tp2-09
fixed: on Unix a `SIGSTOP`
@pattern id=tp2-10
fixed: **Parallel:** per OS
@pattern id=tp2-11
fixed: reads red on one advisory, RUSTSEC-2026-0441
@pattern id=tp2-12
fixed: reads that same survivor at the route entry
