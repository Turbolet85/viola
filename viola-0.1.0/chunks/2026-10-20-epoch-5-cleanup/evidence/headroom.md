# Step 11: the keepers' headroom floor, and its basis

## The readings the floor is set from

- **Fresh `statvfs` reading** (before the floor was written, 2026-10-20T09:04:01Z): the e2e-home link's target
  (an owner-only directory on the `/tmp` tmpfs) reads `f_bavail` 3 562 765 × `f_frsize` 4 096 = 14 593 085 440 B = **13 917 MiB free**, of
  32 097 MiB. `df -h /tmp` at the same time: `tmpfs 32G 18G 14G 57%`.
- **The recorded EDQUOT reading** (`.andromeda/runs/2026-10-19T19-16-36-wrap/relay-3.md`): the switch wrap's light gate
  struck EDQUOT on the tmpfs's per-user quota with `$TMPDIR` at 24 G used of 32 G, about **8 G free by `df`**, a run's
  test homes in place. 13 G used after the founder's cleanup.
- **The run's footprint** (research §Mechanisms (9)): every test home pins a copy of the debug `viola`, 101 163 992 B
  (96.5 MiB). 32 test threads on this host means up to 32 × 101 163 992 B = 3 237 247 744 B (3 087 MiB) at once.
- **The quota itself is unreadable** from the session's shell (`quotactl_fd` → EPERM, research). `statvfs` reads the
  filesystem, so its free bytes stand in for the quota: the same user writes almost all of `/tmp` (other live sessions'
  task output), so the filesystem's free bytes fall as the user's usage rises.

## The floor

- `HEADROOM_FLOOR_MIB = 9 * 1024` (9 216 MiB, 9 GiB), one named constant in each keeper, equal in both:
  `tests/support/home.rs` (`prepare_home_base`'s link branch) and `crates/viola-e2e/src/harness/mod.rs`
  (`Workspace::ensure_e2e_home`'s link branch).
- **Why 9 GiB.** The EDQUOT reading already holds a run's homes, so the line to stay above is about 8 GiB free *with*
  the run in place. A home made while the backing reads below 9 GiB is refused. That leaves one GiB, about ten pinned
  `viola` copies in flight, above the recorded EDQUOT point.
- **Why not the whole footprint on top (8 GiB + 3.1 GiB).** Each check reads the backing with the run's own live homes
  already in it. Today's start, 13 917 MiB less a full run's 3 087 MiB, reads about 10 830 MiB mid-run. A floor of
  about 11.1 GiB (11 366 MiB) would refuse the run's own later homes at today's headroom, though the run then still
  stands about 2.6 GiB (10 830 − 8 192 MiB) above the EDQUOT point.
- The refusal is a fixed message with the two counts and no path: `e2e-home backing: {free} MiB free, below the
  {floor} MiB floor` (root: a panic, as the keeper's other checks; harness: `io::ErrorKind::InvalidInput`, which
  `boot` reports as its closed `build-failed`).
- A plain-directory base (every CI runner) reads no headroom. No env var, flag or config key reads or sets the floor.
- The decision `headroom_refusal(free, floor_mib)` is a plain function tested on every OS:
  `home_base_headroom_below_the_floor_is_refused_with_both_counts`,
  `home_base_headroom_at_or_above_the_floor_passes`, `home_base_headroom_floor_is_nine_gib`
  (`tests/cli_instance_state.rs`); `e2e_home_headroom_refuses_below_the_floor_and_passes_at_or_above_it` and the
  Unix reader's `e2e_home_headroom_reads_a_real_filesystem` (`crates/viola-e2e/src/harness/mod.rs`).

## Remove-the-guard (scratch clone `clone-h`: HEAD `d35644a58909aec12c4898e5a066bcfca5a8ebd2` + the chunk's diff)

The comparison was neutralised in both keepers, `(free_mib < floor_mib)` → `(free_mib < floor_mib && false)`
(`$TMPDIR/{scratchpad}/neutralise_headroom.py`).
Then `cargo nextest run -E 'test(/headroom/)'` was run over each package (`…/scratchpad/headroom_guard.sh`):

| package | neutralised | restored (the files re-applied from the diff, 0 neutralised lines) |
|---|---|---|
| `viola` | `3 tests run: 2 passed, 1 failed` — `home_base_headroom_below_the_floor_is_refused_with_both_counts` panicked at `tests/cli_instance_state.rs:598:5`, `left: None`, `right: Some("e2e-home backing: 9215 MiB free, below the 9216 MiB floor")` | `3 tests run: 3 passed` |
| `viola-e2e` | `2 tests run: 1 passed, 1 failed` — `e2e_home_headroom_refuses_below_the_floor_and_passes_at_or_above_it` panicked at `crates/viola-e2e/src/harness/mod.rs:406:13`, `one byte short`, `left: None` | `2 tests run: 2 passed` |
