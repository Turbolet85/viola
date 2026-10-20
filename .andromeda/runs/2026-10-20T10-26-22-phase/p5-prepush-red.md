# P5 baseline — `bash scripts/agent-run.sh pre-push` read red on the untouched tree

- **Fired:** entry 27 through `gate.py run … --entry 27`, HEAD 844de97682c8, on 2026-10-20, with the gate's run line
  `27 probe red · exit 0 ✗ (exit 1) · exit 1 · 85.4s`.
- **What failed:** 78 cases, each panicking at `tests/support/home.rs:139` with
  `e2e-home backing: 8939 MiB free, below the 9216 MiB floor`. This is `prepare_home_base`'s headroom refusal from chunk
  2026-10-20-epoch-5-cleanup (`HEADROOM_FLOOR_MIB`), firing as designed.
- **The host:**
  - `df -h /tmp` read `tmpfs 32G, 19G used, 13G avail` before the run and again after it.
  - The run itself drew the tmpfs below 9 GiB. `target/e2e-home` links to `$TMPDIR/viola-e2e-home-1000`, the coverage run's
    homes sit there, and the floor reads that filesystem.
  - What holds the tmpfs (`du -xsh /tmp/*`): `$TMPDIR/claude-1000` 16G, which is Claude Code's own session data outside this
    repository, and `$TMPDIR/viola-e2e-home-1000` 1.9G, which is 80 entries, among them earlier live rounds' homes
    (`viola-live-*`, `viola-record-*`).
- **Disposition (validation.md check 4 (9)):** this is RED on an existing property, the host's headroom. It is not this
  chunk's subject. Freeing space on the tmpfs is the founder's call at his desk: the session data is outside the work tree,
  and the kept live homes are earlier chunks'. The owner is named at the review.
- **Measured draw:** the `pre-push` coverage run takes more than 4 GiB of the tmpfs, from 13G free to under 8.9 GiB.
  Step 1 of the plan reads `df -h /tmp` against the floor plus that draw.
