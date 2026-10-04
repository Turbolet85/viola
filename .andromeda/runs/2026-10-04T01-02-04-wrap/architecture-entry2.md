
## 2026-10-03-mutation-scoring-completion — the mutation scratch dir on the Linux host
**Section:** §Occupied Resources (`<repo parent>/viola-mutants-scratch/`)
**Change:** The harness arm stays Windows-only (`HOST_SCRATCH = cfg!(windows)`), and its wipe now runs before every counted, scoped or package run (was "counted or scoped"). New: on the Linux dev host the same-named dir is the operator's `TMPDIR` for every mutation run, an environment fact rather than harness code, and it must be NOCOW (`chattr +C`). The reasons: cargo-mutants 27.1.0 copies the tree (`target/` included, ~14 GB) into the temp dir; `/tmp` there is a 32 GB `usrquota` tmpfs (a full viola-e2e run died at 460/709); and a COW btrfs reflink copy drops the exec bit of the prebuilt `viola-fake-agent`. The body carries this as measured at the chunk's `evidence/m3.md`. No document prints the path.
**Why:** overseer direction (founder-delegated): every mutation run on this host takes `TMPDIR` on btrfs. The NOCOW requirement is the measured remedy for the reflink mode loss (`reflink 0.1.3` creates the clone with `create_new` and copies no mode).
**Ref:** .andromeda/runs/2026-10-04T01-02-04-wrap/
