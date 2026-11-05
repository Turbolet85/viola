# P4 dialog — the closed words that lose their only emitter

Asked at phase P4 of chunk 2026-11-05-linux-only-code-and-ci, one question, technical.

## The question, as asked

The Windows and macOS code goes, but some closed words only that code emitted would stay in the wire contracts and
schemas: `batch-script-child` (revive --json and verify codes, the process-exit detail enum,
AgentError::BatchScriptChild), `sideload_fallback` with its five words on process-start, and `os`'s `windows` ·
`macos` in schemas/diag-line.v1.json. Linux never emits any of them today (resolve_program never refuses on Unix). Do
they leave with the code, or stay as reserved words with no emitter?

1. Remove with the code (Recommended) — each word leaves its schema enum, closed code list and pinned test literal in
   the same change, and the wrap's obs/arch amendments record the removal (the 2026-10-16 browser-leg precedent).
2. Keep as reserved, no emitter — the schemas and closed lists stay byte-identical; only the code that produced the
   words goes (the 2026-10-16T14-58-10-wrap precedent).

## The answer, verbatim

Chosen: «Remove with the code (Recommended)».

Notes: «The overseer, technical answer (founder-delegated), 2026-11-05 11:08Z, within the founder's own ruling of
2026-11-04 21:50Z (remove the Windows and macOS code, recoverable at the tag): a closed word no build can emit leaves
with its emitter. The scope's «no closed word changes» boundary is narrowed to these words by this answer; record it
so.»

## Authority

delegate the Viola overseer, 2026-11-05 11:08Z (a technical answer, founder-delegated), resting on the founder's own
ruling of 2026-11-04 21:50Z. A narrowing, no boundary widening: no escalate pattern of the playbook names it.
