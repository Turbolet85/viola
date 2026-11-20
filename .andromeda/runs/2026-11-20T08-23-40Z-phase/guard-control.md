# Entry 12 (the preservation guard) — its known-positive control, P5 check 4 (9)

The clone is `git clone --shared` of the project at HEAD `9e2b7e7d`, made in the session scratchpad
(`$TMPDIR/{scratchpad}/guard-clone`). The entry was fired through `gate.py run --entry 12 --docs-root {root}
--project-root {clone}`.

- Unmodified clone: `12 probe green · exit 0` with last line `guard: clean`. This matches the real tree's baseline
  reading.
- Clone with one trailing line appended to `crates/viola-agent-claude/src/ledger.rs`, left uncommitted (the input
  the guard must fail on): `12 probe red · last line 'guard: clean' ✗ (last: 'guard: outside 1 files')`.

Verdict: the guard reads red on a change to a path it protects and green on no change. The control holds.
