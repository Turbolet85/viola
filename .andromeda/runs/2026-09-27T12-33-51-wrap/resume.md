# Wrap resume point — 2026-09-27-wrapper-channel

- Run dir: `.andromeda/runs/2026-09-27T12-33-51-wrap/` (created 12:33:51Z). Chunk `2026-09-27-wrapper-channel`
  (the one master `pending` record); chunk_dir `viola-0.1.0/chunks/2026-09-27-wrapper-channel/`.
- **Done: P1** — `report.md` written (this session holds the implement + operator-pass conversation).
- **Next: P2** (fan-out → validate → apply → cascade). Resume via Setup 2a with this dir: the report is
  reused as is; no `fanout-results.md` yet, so P2 fans out.
- Owed from P1, not done (context 92%): the P1 evolve checkpoint (`references/evolve/report.md`); the
  report-template's pre-scan of `drift-base.md` detector `check` fields against the Changes bullets; the
  per-master grep + hit count behind each "Expected amendments" line (the report says so).
- Git basis: the operator pass ran — diff base is `17c99c9` (parent of the oldest pre-CI commit `3fca0ac`);
  HEAD `3efed41` == `origin/build/viola-0.1.0`; CI 36318398739 on `3efed41` success 15/15 (overseer-read).
  Untracked, to ride the wrap commit: `evidence/operator-pass-continued.md`, the implement run dir's
  `fix-pp*`/`op-23*` trail files, this run dir, `report.md`.
- The background code-graph refresh (Setup 7) was NOT fired — fire it at resume.
