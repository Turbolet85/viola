# Entry 14's predicate, typed beside it in the tool's shell form (the tool fires no deferred entry)

Run 2026-11-24 at P5 by `bash -o pipefail -c "test -s '<f>' && grep -c '^r5[ab]: green ' '<f>'"` over three scratch
records in the session scratchpad:

- both lines green (`r5a: green resumed=yes lineage=yes turn=ended`, `r5b: green refused=session-live started=none`):
  exit 0 · last line 2 — the entry's expect holds;
- `r5a` green, `r5b` red: exit 0 · last line 1 — the entry reads red on its `last line 2` atom;
- no record: exit 1 · no line — red.
