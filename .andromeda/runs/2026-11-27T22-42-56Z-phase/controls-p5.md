# P5 controls — 2026-11-27-epoch-5-cleanup-2

Fired as the entries, `gate.py run --chunk 2026-11-27-epoch-5-cleanup-2 --docs-root . --entry {n} --project-root
$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/70c72b6e-1c7e-4929-b2f6-77d700394705/scratchpad/clone`, the
clone a `git clone --shared` of the project at HEAD `509eeca148c3ef174c303a1051322de60bd85e91`, its inputs left
uncommitted in the clone.

## Unmodified (the clone reads what the tree read)
- 12 `sizes` · red · last line '0' ✗ (last: '2') — as the tree.
- 13 `cx` · red · last line '0' ✗ (last: '2') — as the tree.
- 16 `bodies` · red · exit 1 ✗ (exit 2) — the record absent, as the tree.

## Must-pass and must-fail inputs
- 12 `sizes` · input: the clone's `crates/viola-pty/src/pump.rs` and `src/cmd/run.rs` each replaced by one line ·
  green · exit 0 (last line 0).
- 13 `cx` · input: the clone's `src/cmd/up.rs` replaced by the research prototype (`scratchpad/proto/up.rs`, the
  three-way split) and `crates/viola-pty/src/pump.rs` by one line · green · exit 0 (last line 0).
- 16 `bodies` · input: a minted `evidence/moved-bodies.md` of two `equal` lines and its summary · green · exit 1
  (last line 0).
- 16 `bodies` · input: the same record with one `differs` line · red · exit 1 ✗ (exit 0) — the entry fails on a
  differing body.
