# P5 check 4 (9) controls — 2026-10-13-epoch-4-cleanup

Each entry fired AS ITSELF through `gate.py run --plan … --entry {n} --project-root {clone}` (no run dir) over a
scratch clone of the project at `d9ad9f1f1d96` (`git clone --shared` into the session scratchpad, spelled
`$SCRATCHPAD/clone-e4c` here), first unmodified, then over a must-fail and a must-pass input; the clone was reset
between inputs. The cargo build of entry 9 ran with `CARGO_TARGET_DIR` at the project's own `target/`. The lines are
the tool's, as printed (2026-10-13; recorded at 07:32:40Z by `date -u`).

## Entry 10 — the sizes read (`tokei … | python3 …`, `expect = ['exit 0', 'last line 0']`)
- unmodified (both split dirs absent): `red · exit 0 ✗ (exit 1)` — tokei prints `Error: 'src/run/send' not found.`
  and no JSON, as on the real tree's baseline.
- must-fail — `src/human/` and `src/run/send/` present (empty), `src/human.rs` and `src/run/send.rs` whole:
  `red · last line '0' ✗ (last: '2')` — the two files over 800, the subject the split removes.
- must-pass — every file of both sets a one-line function: `green · exit 0`.

## Entry 11 — the human siblings' print/allow read (`expect = ['exit 1', 'no output']`)
- unmodified (`src/human` absent): `red · exit 1 ✗ (exit 2)` — `rg: src/human: No such file or directory`.
- must-pass — `src/human/x.rs` holding a clean function: `green · exit 1`.
- must-fail — the same file plus a function calling `eprintln!`: `red · exit 1 ✗ (exit 0)`.

## Entry 12 — the preservation guard (`git diff --quiet d9ad9f1f1d96 -- . ':!…'`)
- unmodified: `green · exit 0`.
- must-fail — a comment appended to `tests/cli_list.rs`: `red · exit 0 ✗ (exit 1)`.
- must-pass — a comment appended to `src/human.rs` and to `crates/viola-e2e/src/harness/run/mutants/scratch.rs`,
  and a new `src/human/x.rs`: `green · exit 0`.

## Entry 13 — the lockfile read (`expect = ['exit 1', 'last line 0']`)
- unmodified: `green · exit 1`.
- must-pass — `"viola-state",` inserted into viola-e2e's `dependencies` list in `Cargo.lock`: `green · exit 1`.
- must-fail — a `[[package]]` `name = "zz-control"` table appended: `red · exit 1 ✗ (exit 0)`.

## Entry 9 — the split names (`cargo nextest list … | sed -E … | sort | diff - evidence/split-test-names.txt`)
- must-pass — the record written by the entry's own pipeline (minus `diff`) on the clone's tree, 223 lines:
  `green · exit 0`.
- must-fail — the same record with its first line removed: `red · exit 0 ✗ (exit 1)`.
- The normalisation over post-split paths (a typed control differing from the entry's `sed` stage by its input
  lines only, run as `/usr/bin/bash -o pipefail -c`): four pairs — `human::tests::board_row_cli_cell` /
  `human::board::tests::board_row_cli_cell`; `human::tests::answer_hint_is_the_fixed_table::case_1_unverified` /
  `human::hints::tests::…::case_1_unverified`; a `run::send::tests::…::case_1` rstest case against itself;
  `run::send::tests::send_local_command_clear_is_confirmed_by_a_new_session` /
  `run::send::classify::tests::…` — each reduced to the same `{root} {name}` line (`same` ×4, exit 0).
