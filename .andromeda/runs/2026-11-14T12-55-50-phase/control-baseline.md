# P5 check 4 (9) controls — 2026-11-14-hyprland-exact-frame-bound-pinned

Clone: `git clone --shared` of the project, detached at 2414270c1ce5e8e3fb2df262a1aef2467758d113, at
`$TMPDIR/{scratchpad}/ctl-clone` (role: the control clone). Each entry fired as itself:
`gate.py run --chunk 2026-11-14-hyprland-exact-frame-bound-pinned --docs-root . --entry {n} --project-root $TMPDIR/{scratchpad}/ctl-clone`.

## Unmodified (must read what the tree read)
- entry 7 — green · exit 0 (last line 1), as on the tree
- entry 8 — green · exit 0 (last line 2), as on the tree
- entry 9 — red · last line 0, as on the tree

## Input A — the two mutants: `read.rs:77` and `reply.rs:151` `>` → `>=` (uncommitted in the clone)
- entry 7 — red · exit 1 (grep -c reads 0): the known-positive control holds
- entry 8 — red · last line 1 (expected 2): the known-positive control holds
- entry 9 — green · last line 2: the must-pass reading (the two listed files changed)

## Input B — Input A plus one comment line appended to `src/main.rs`
- entry 9 — red · last line 3: the fail reading

## Entry 5 — typed control (an input operand alone differs: the two test names)
Fired on the tree as `/usr/bin/bash -o pipefail -c 'cargo nextest run -p viola-hyprland --no-fail-fast -E "test(=read::tests::read_of_a_reply_past_the_cap_is_oversize) | test(=reply::tests::parse_clients_takes_max_frame_bytes_and_refuses_one_more_as_oversize)"'`:
exit 0 · `Summary [ 0.136s] 2 tests run: 2 passed, 80 skipped` — the filter form and the `2 tests run: 2 passed` atom read
green over two tests that stand.
