# Curation — 2026-10-03-mutation-scoring-completion

CLAUDE.md ecosystem curated:
- **Tier 1** (CLAUDE.md `USER:session-learnings`): none.
- **Tier 2** (.claude/rules/*):
  - `testing.md`: "A chunk's `[[gate]]` block holds no mutation entry: a scoring or kill run is a one-off witness whose
    counts go to `evidence/` …" (confidence 1.2)
    - Proof: founder ruling 2026-09-28, curated on the overseer's directive 2 (`wrap68-directives.md`). Its cost
      measured twice at P5: this chunk's plan went from 28 to 24 gate entries, and the cleanup chunk's from 28 to 25.
      Signals: explicit curation request +0.5, "never / no" language +0.3, verified by two review rounds +0.4. Filter 1:
      the generated body's "no chunk, pre-push or CI mutation gate" matched; the gate-block and `evidence/` halves are
      the additive facet, so a new Session Additions entry (the generated-body rule).
  - Correction (exempt from the cap) `testing.md` 2026-10-03 entry, the clippy and cfg-import rule: "Linux clippy in
    pre-push's WSL clone can, before the push" → `[corrected 2026-10-04: pre-push's WSL clone is retired and the dev
    host is Linux]`.
    - Proof: this chunk deleted `scripts/wsl-provision.sh` / `wsl-exec.sh` and reshaped `pre-push` to native Linux
      (report Changes). The cascade sweep routed the row here (`cascade-dispositions.md`, curation row
      `testing.md:72`).
- **Tier 3** (.claude/docs/session-learnings.md): "A push touching a workflow file needs the `workflow` OAuth scope"
  (confidence 0.8).
  - Proof: entry 23's first firing refused at the remote (`refusing to allow an OAuth App to create or update workflow
    .github/workflows/ci.yml without workflow scope`); after `gh auth` gained `workflow` the unchanged push landed
    `aa300a4..80b69cd` (`evidence/operator-pass.md`). Signals: verified by a real failure +0.4, specific technical
    detail +0.2, reached no other home +0.2 (the exact-0.6 conditional).
- **Extended:** T2/host-win32.md: "2026-10-03: cargo-mutants copies the tree under `TMP` …" + "on the Linux dev host
  point `TMPDIR` at a NOCOW btrfs dir (quota tmpfs; reflink drops the exec bit)".
  - Proof: M3 run 1 died at 460/709 on `Disk quota exceeded`, with 25 unviable from the cap; run 2's baseline read 32
    EACCES on the copied `viola-fake-agent`. Two-sided probe: a raw `FICLONE` into a COW dir gives mode 0644, and a
    clone into the `chattr +C` dir fails EINVAL (`evidence/m3.md`). The overseer's directive 4 names the host rule as
    the home. Signals: measured +0.4, explicit curation request +0.5.
- **Filters:**
  - 0 duplicate · 2 task-specific (the CLI auto-submitted P3 prompt: a CLI behaviour, not a project rule; the scratch
    residue counts) · 0 conflict.
  - 3 deferred (→ handoff):
    - the "not measured here; owed to {route entry}" vocabulary (0.7; already amended into test-plan §10 and
      tests-summary);
    - the kill(2) PID 1 deadline target (0.8);
    - a PTY master close hanging up a live child only when no reader or writer clone remains (0.8).
- **No-other-home:** "A push touching a workflow file needs the `workflow` OAuth scope".
- **CLAUDE.md size:** read at P7 (health check 1).
