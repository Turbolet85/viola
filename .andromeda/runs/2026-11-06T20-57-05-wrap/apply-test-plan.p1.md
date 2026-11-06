@apply_row n=1 ref_file=fanout-test-plan.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-test-plan.md ref_n=2 v=refused
why: fct-zxaumenb stands in the keyed contract 5-command-implementation, a key file outside this part's four read files (body, test-scope-summary, test-strategy, unit-test-strategy); its rev was not read, so the edit goes to the part that reads that key
@apply_row n=3 ref_file=fanout-test-plan.md ref_n=3 v=applied edit=E2
@apply_row n=4 ref_file=fanout-test-plan.md ref_n=4 v=applied edit=E3
@apply_row n=5 ref_file=fanout-test-plan.md ref_n=5 v=applied edit=E4
@apply_row n=6 ref_file=fanout-test-plan.md ref_n=6 v=applied edit=E5
@apply_row n=7 ref_file=fanout-test-plan.md ref_n=7 v=applied edit=E6
@apply_row n=8 ref_file=fanout-test-plan.md ref_n=8 v=applied edit=E7
@apply_row n=9 ref_file=fanout-test-plan.md ref_n=9 v=applied edit=E8
@apply_row n=10 ref_file=fanout-test-plan.md ref_n=10 v=refused
why: fct-da3wgwes stands in the keyed contract bootstrap-phases, a key file outside this part's four read files; its rev was not read, so the edit goes to the part that reads that key
@apply_row n=11 ref_file=fanout-test-plan.md ref_n=11 v=applied edit=E9
@apply_row n=12 ref_file=fanout-test-plan.md ref_n=12 v=applied edit=E10
@apply_row n=13 ref_file=fanout-test-plan.md ref_n=13 v=applied edit=E11,E12,E13,E14,E15,E16,E17,E18,E19,E20,E21,E22,E23,E24,E25,E26,E27
@apply_row n=14 ref_file=fanout-test-plan.md ref_n=14 v=refused
why: fct-upolcsa3 names no «Version done-check» owner to re-point: its survivor clause ends «an always-admit survives every committed test», with no owner; the known-limit status is written once, at fct-cz5jn5fs (fanout-test-plan--cascade-g1.md 3, E29)
@apply_row n=15 ref_file=fanout-test-plan--cascade-g1.md ref_n=2 v=applied edit=E28
@apply_row n=16 ref_file=fanout-test-plan--cascade-g1.md ref_n=3 v=applied edit=E29
@apply_row n=17 ref_file=fanout-test-plan--cascade-g1.md ref_n=4 v=applied edit=E30
@apply_row n=18 ref_file=fanout-test-plan--cascade-g1.md ref_n=5 v=applied edit=E31
@fact id=fct-njspb5nv expect_rev=2
old: the seam over portable-pty `=0.8.1` (spawn
new: the seam over portable-pty `=0.9.0` (spawn

@fact id=fct-hlx2zkbt expect_rev=1
old: `cargo deny check` (only ignore RUSTSEC-2017-0008),
new: `cargo deny check` (no advisory ignored, `[advisories] ignore = []`),

@fact id=fct-engo5ir7 expect_rev=1
old: never taken as a prompt (§6 rewritten-path warning)
new: never taken as a prompt

@fact id=fct-u3dkdc2w expect_rev=2
old: identity floor measured on the Windows host;
new: identity floor measured on the retired Windows host;

@fact id=fct-vxnii5yh expect_rev=2
old: (Windows dev host, release build)
new: (the retired Windows dev host, release build)

@fact id=fct-yoazurel expect_rev=1
old: live wrapper), per OS |
new: live wrapper) |

@fact id=fct-2ztzeoct expect_rev=1
old: threshold for tier (per OS)
new: threshold for tier (the `test` job on `ubuntu-latest`)

@fact id=fct-plbmhdnw expect_rev=1
old: (any level, any OS)
new: (any level)

@fact id=fct-3mj52n2x expect_rev=1
old: `COVERAGE_IGNORE`. llvm-cov reports Windows paths with `\`, so the forward-slash form never matched there (as measured
new: `COVERAGE_IGNORE`; its separator class is kept from the retired Windows target, where llvm-cov reported paths with `\` (as measured

@fact id=fct-b4avhaux expect_rev=1
old: The separator class that makes it match Windows paths is not a widening
new: The separator class, kept from the retired Windows target, is not a widening

@fact id=fct-a6gfb3bv expect_rev=1
old: driven as a subprocess by `assert_cmd` with a per-test `--home`
new: driven as a subprocess by `std::process::Command` with a per-test `--home`

@fact id=fct-hyr7b53t expect_rev=1
old: subprocess + `assert_cmd` feeding recorded hook payloads
new: subprocess (`std::process::Command`) feeding recorded hook payloads

@fact id=fct-4ignlk4w expect_rev=1
old: a real tempfile home and `assert_cmd`.
new: a real tempfile home and `std::process::Command`.

@fact id=fct-gcer5i4i expect_rev=1
old: (`INSTA_UPDATE=no`), and trycmd only in verify mode.
new: (`INSTA_UPDATE=no`).

@fact id=fct-s42l2rwd expect_rev=1
old: assert_cmd 2.2.2 + predicates 3.1.4 against `CARGO_BIN_EXE_viola`.
new: `std::process::Command` against `CARGO_BIN_EXE_viola`, its outputs pinned by literal asserts.

@fact id=fct-klrvzf5j expect_rev=1
old: trycmd 1.2.1 case files, except the `viola list` board, pinned by literal asserts in
new: literal asserts in their test files: the `viola list` board in
old: and `viola --help`, pinned by the `HELP` literal
new: and `viola --help` by the `HELP` literal

@fact id=fct-w5xcngvx expect_rev=2
old: the human layouts by trycmd or literal with no
new: the human layouts by literal with no

@fact id=fct-gnzxb6ex expect_rev=1
old: assert_cmd with predicates, and trycmd or literals for human layouts;
new: `std::process::Command`, with literals for the `--json` documents and the human layouts;

@fact id=fct-iqthlzz5 expect_rev=1
old: assert_cmd with an `Instant` deadline
new: `std::process::Command` with an `Instant` deadline

@fact id=fct-puo2hazj expect_rev=1
old: trycmd cases with snapbox redactions; `viola --help` and the `viola list` board are literals in their test files.
new: literals in their test files (`viola --help`, the `viola list` board).

@fact id=fct-rvltlmnj expect_rev=1
old: inside the assert_cmd hook tests,
new: inside the hook subprocess tests,

@fact id=fct-ajzn4txb expect_rev=1
old: use `assert_cmd` `.assert()` on the long-lived `viola run` wrapper. It blocks until exit; use std::process + a `Drop` guard.
new: use `std::process::Command` `.output()` or `.status()` on the long-lived `viola run` wrapper. It blocks until exit; use `.spawn()` + a `Drop` guard (`tests/support/cli.rs` `Running`).

@fact id=fct-qxrlql3x expect_rev=1
old: (`cargo insta review`, trycmd overwrite)
new: (`cargo insta review`)

@fact id=fct-fnukifof expect_rev=3
old: subprocess + `assert_cmd` / `Command::cargo_bin("viola")`, with
new: subprocess + `std::process::Command` over `CARGO_BIN_EXE_viola`, with

@fact id=fct-hlf5klew expect_rev=1
old: and assert_cmd against `CARGO_BIN_EXE_viola`.
new: and `std::process::Command` against `CARGO_BIN_EXE_viola`.

@fact id=fct-hckuzazj expect_rev=3
old: **trycmd cases:** `tests/cmd/*.toml`.
new: **Human-mode literals:** in the verbs' own test files (the board in `tests/cli_list.rs`, `viola --help` in `tests/cli_output_plain.rs`).

@fact id=fct-yhevpdmp expect_rev=1
old: integration (assert_cmd hook)
new: integration (hook subprocess)

@fact id=fct-b2j3wr36 expect_rev=2
old: Must-Work (R2, R3/S1, Windows ConPTY).
new: Must-Work (R2, R3/S1).

@fact id=fct-cz5jn5fs expect_rev=1
old: and the mutation audit reads that survivor at the route entry "Version done-check".
new: and stands as the version's known limit, read at the epoch-boundary audit (matrix#v1-44).

@fact id=fct-vekwchud expect_rev=1
old: replays every dialog variant byte for byte under `--dialogs` and
new: replays every dialog variant byte for byte under `--dialogs` (its synthetic witness `dialog_drift_names_a_changed_annotation`: a planted change to a forwarded `annotations` byte of `synthetic_dialog_set` reads `dialog_drift` red) and

@fact id=fct-hpqcf7r7 expect_rev=2
old: `agents-join` then `local-command-compact` appended last).
new: `agents-join` then `local-command-compact` appended last). On an unchanged `claude` 2.1.287 a live `viola verify` reads `statusline-rate-limits` red (`23 pass  1 fail`) where the account's status-line payload carries `five_hour` `unknown` and only `seven_day`: the row reads the account's plan state, not the CLI (as measured at `viola-0.1.0/chunks/2026-11-06-version-done-check/evidence/live-round.md`); the arm and its nine cases stand · owner: 0.2.0 (finding F-06i).

@pattern id=tp1-01
fixed: portable-pty `=0.8.1`
@pattern id=tp1-02
fixed: only ignore RUSTSEC-2017-0008
@pattern id=tp1-03
fixed: rewritten-path warning
@pattern id=tp1-04 ignore_case=true
regex: \((per OS|any level, any OS)\)|, per OS\b
@pattern id=tp1-05
fixed: llvm-cov reports Windows paths
@pattern id=tp1-06
fixed: makes it match Windows paths
@pattern id=tp1-07
regex: (subprocess (by|\+) |inside the |and |Driver:\*\* )`?assert_cmd|trycmd (1\.2\.1 case files|or literals?|only in verify mode|overwrite|cases)
@pattern id=tp1-08
fixed: Windows ConPTY
@pattern id=tp1-09
fixed: reads that survivor at the route entry
