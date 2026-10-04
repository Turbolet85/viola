
## 2026-10-03-mutation-scoring-completion — pre-push document and mutation scratch wording
**Section:** §8 PII Scrubbing → Integration points, item 6 (the pre-push document bullet; the `mutants.out/` bullet)
**Change:** The pre-push document carries no absolute path, only codes and counts. The gate runs natively on the Linux host in the working tree (no clone), so neither the repository root nor the passwd home reaches it, as `pre_push_document_carries_no_absolute_path` asserts; was "no Windows repository or home path, no Linux clone or home path (`~/viola-pre-push`)". The `mutants.out/` bullet: the harness scratch is Windows-host-only (`HOST_SCRATCH = cfg!(windows)`); on the Linux dev host the same-named dir is only the operator's NOCOW `TMPDIR`, printed by no document, and `mutants.out/` stays at the repository root.
**Why:** the WSL clone retired with the Windows dev host; the no-path guarantee now names the native gate's paths at risk.
**Ref:** .andromeda/runs/2026-10-04T01-02-04-wrap/
