# Peak WSL `/tmp` during the pre-push Linux leg — 2026-09-27-instance-state-and-start-order

Overseer ruling 2026-09-27 (option 1), condition 3. `/tmp` in the WSL2 `Ubuntu` distro is a tmpfs of
16 801 394 688 B (16.8 GB). Sampled with `df --output=used -B1 /tmp` every 2 s from before the pre-push launch
until after it ended, under `env -i HOME=… PATH=/usr/bin:/bin`; per-minute maxima below.

## Before the fix (pre-push run 2, 2026-09-27 ~02:35Z)

`ubuntu-latest` leg: `mutants-exit-1` — cargo-mutants `No space left on device (os error 28)` writing its
scratch tree in `/tmp`, after 9 outcomes of 137 mutants (log of gate entry 21, that run). No sampler ran; the wall
is the tmpfs size itself (~16.8 GB) reached within ~10 mutants. Cause: homes of tests killed by the `mutants`
nextest profile's `fail-fast … terminate = "immediate"` are never dropped, and each held a 38 MB pinned copy of
the Linux debug `viola`.

## After the fix (final pre-push run, 03:45Z–04:02Z, gate entry 21 green)

| minute (UTC) | max `/tmp` used | phase |
|---|---|---|
| 03:45 | 0.02 GB | sync, Linux suites |
| 03:46 | 12.21 GB | cargo-mutants scratch created (`--copy-target=true` copies the clone's ~9 GB target) |
| 03:47 | 13.19 GB | baseline, then mutants |
| 03:48 | 13.97 GB | mutants |
| 03:49 | 13.98 GB | mutants |
| 03:50 | 14.02 GB | last mutants |
| 03:51 → end | 0.08 GB | scratch removed; the `windows-2025` leg runs on the host |

Peak **14 018 965 504 B (14.0 GB)** of 16.8 GB, with **all 142 mutants tested** on the leg (union green, 0
breaches). Growth across the whole mutation phase was 12.21 → 14.02 GB (~1.8 GB for 142 mutants, ~13 MB per
mutant) against ~1.2 GB per mutant before. An earlier post-fix run (03:23Z–03:28Z, 142 mutants) peaked at
14.31 GB with the same shape.

## After the second fold: the mutation scratch off the tmpfs (pre-push 04:27Z–04:47Z, entry 21 green)

Overseer ruling 2026-09-27 (second fold). Mechanism verified at the pinned version, not assumed: cargo-mutants
27.1.0 `--help` has no scratch-location option (it has `--copy-target` and `--in-place`, reported, not taken), and
its source builds each scratch copy with `tempfile::Builder::new().prefix(..).tempdir()` (`copy_tree.rs:81-84`),
i.e. in `std::env::temp_dir()`, which honours `TMPDIR`. pre-push now runs the Linux mutation leg with
`TMPDIR=$HOME/viola-pre-push-scratch` (beside the clone, on its ext4 disk, outside the copied tree), wiped and
recreated 0700 at the start of each run; the cache report carries `scratch_bytes` (before the wipe) and
`scratch_bytes_after`.

Sampled every 3 s: `df` of `/tmp` and `du -sb` of the scratch dir.

| minute (UTC) | `/tmp` max | scratch max | phase |
|---|---|---|---|
| 04:27 | 0.13 GB | 5.34 GB | Linux suites, then the scratch copy starts |
| 04:28 | 0.00 GB | 13.19 GB | cargo-mutants copy + baseline |
| 04:29–04:32 | 0.00 GB | 14.32 → 14.63 GB | 160 mutants |
| 04:33 | 0.00 GB | 12.87 GB | scratch copy removed |
| 04:34 → end | 0.00 GB | 0.32 GB | leftovers of fail-fast-killed tests' temp dirs; the host `windows-2025` leg runs |

**`/tmp` peak 133 431 296 B (0.13 GB)** for the whole run (sync, Linux suites and the mutation leg), against 14.0 GB
before this fold; **scratch peak 14 626 345 696 B (14.6 GB)** on the clone's disk; **all 160 mutants tested** on both
legs (the count grew with this chunk's code), union 0 breaches. The document read `scratch_bytes: 0`,
`scratch_bytes_after: 322 572 880`: the 0.32 GB is removed by the next run's wipe.

## Remaining headroom (pre-existing, not this chunk's) — superseded by the second fold

The ~12 GB floor is cargo-mutants' `--copy-target=true` copy of the clone's `target/` (9.27 GB after this run;
pre-push's cache cap is 40 GB). The leg keeps ~2.8 GB of the tmpfs free today; as the clone's target cache grows
toward its cap, the copy alone will outgrow the 16.8 GB tmpfs whatever the tests leave behind.
