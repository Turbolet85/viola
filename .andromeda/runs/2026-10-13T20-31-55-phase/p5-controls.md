# P5 controls — 2026-10-13-paste-newline-ledger-row

Read at 2026-10-13T21:16:56Z (`date -u`). Each control fired AS THE ENTRY through `gate.py run --plan … --entry {n}
--project-root {clone}` over a `git clone --shared` of the project at HEAD `19483111dbe9`, in the session scratchpad
(`$TMPDIR/{scratchpad}/clone-*`), by the script `$TMPDIR/{scratchpad}/p5-controls.sh`. The inputs lived only in the
clones; only the verdict lines are copied here.

## Entry 10 — the fixture guard
- unmodified clone (must read red): `red · exit 0 ✗ (exit 1)` — as on the real tree
- `paste-3` minted, untracked (must pass): `green · exit 0`
- `paste-3` minted + `paste-2` modified (must fail): `red · exit 0 ✗ (exit 1)`
- `paste-3` + an extra `paste-4` minted (must fail): `red · exit 0 ✗ (exit 1)`
- `paste-3` committed, clone HEAD `f72c7f44b260` (must pass — the light gate's side): `green · exit 0`

## Entry 12 — the literal probe
- unmodified clone (must read red): `red · last line '0' ✗ (last: '75')` — as on the real tree
- the six files emptied (must pass): `green · exit 0`, last line `0`

## Entry 13 — the preservation guard
- unmodified clone (must pass): `green · exit 0` — as on the real tree
- `Cargo.toml` edited (must fail): `red · exit 0 ✗ (exit 1)`
- an untracked `src/new_file.rs` (must fail): `red · exit 0 ✗ (exit 1)`
- `crates/viola-state/src/fs.rs` edited (must fail): `red · exit 0 ✗ (exit 1)`
- the modify-set files edited + `paste-3` minted (must pass): `green · exit 0`

## Entry 6 — the named row cases
- typed as the tool types an entry, differing by its pattern operand alone:
  `/usr/bin/bash -o pipefail -c "bash scripts/agent-run.sh run --unit --filter 'test(/check_paste_rows_fail_without_a_matching_capture/)'"`
  → exit 0, `"suite":"nextest-unit","passed":1,"failed":0` (the filter form selects an existing case).
