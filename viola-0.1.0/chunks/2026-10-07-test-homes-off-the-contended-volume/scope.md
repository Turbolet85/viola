# Scope — 2026-10-07-test-homes-off-the-contended-volume

**Working entry** (`working-route.md:94`, Epoch 3 — Windows slice II: driving verbs and live proof):
Test homes off the contended volume — on the dev host a test start no longer stalls behind other builders' writes,
the homes' path and CI's own fsync unchanged.

## Intent
On the Linux dev host a test that starts `viola` no longer waits behind another builder's writes: the bytes a start
copies into its home land on a backing the shared volume's dirty-page budget does not pace. The homes' path stays
`<workspace>/target/e2e-home`, and CI keeps writing its homes to a real disk through the real `sync_all()`. The
chunk ends with a contended reading that is red on the present backing and green on the new one.

It is a small chunk (inputs#I1): one backing, its documents, one proof.

## Authority
- The overseer's own item, added to the 2026-10-07 0-pending wrap's route adaptation: **not a founder ruling**
  (inputs#I2 §4; the entry's CARRY says the same). It was placed ahead of "Live rows and paste shapes on the dev
  host" (`working-route.md:96`) so that entry's capped live round never meets a contended gate.
- The operator's directive at this take-up (inputs#I1, a verbatim copy):
  - no live `claude` session;
  - a backing that needs no root exists (`/tmp` is tmpfs), and a mount needs the founder at the desk: **both are
    priced at P4**, each with what it does to the homes-under-the-workspace statement and to the CI globs;
  - the proof must be a contended reading, red before and green after;
  - a synthetic writer on the shared volume also stalls four other builders: the proof is planned small and
    bounded, **and the operator is told before it runs**; or it is measured inside a natural link with overseer1's
    probe (F167 design slot, `start_probe.py`: inputs#I4);
  - size it as a small chunk.
- The P4 answers (inputs#I6, a verbatim copy; added at P5's validation-1, the scope having not named the
  widening):
  - the backing is a link with a test-side keeper. The harness then creates and deletes outside the working
    directory through the link, a boundary widening: ratified by the founder's own live answer of
    2026-10-07T07:25Z, given after the widening was shown to him, relayed by the overseer;
  - the proof is one natural window, capped at 60 minutes; when the cap runs out the builder stops and asks the
    overseer before any synthetic writer (the overseer's answer, a technical fork).

## What it builds (the entry's freight folded)

### W1 — the backing (the entry's text and its CARRY)
- On the dev host the test homes under `<workspace>/target/e2e-home` are backed by tmpfs, the path unchanged.
- How the backing is done is this chunk's research (the CARRY: "a mount the founder runs once, a link, a harness
  option"). P4 prices at least the two the operator names: a backing that needs no root, and a mount the founder
  runs at the desk. Decided at P4 (inputs#I6): `target/e2e-home` is a link to a per-user owner-only directory on
  tmpfs, with a keeper in the root fixture chain and in the harness that re-creates and verifies the link's
  target, and G2's walk written to descend a linked scope.
- CI is unchanged in behaviour: its homes stay on the runner's disk and the pinned copy still ends in `sync_all()`.
- `replace_private`'s `sync_all()` stays (the entry's hypothesis says removing it would not remove the stall, and
  it is a product durability step, not a test seam).

### W2 — the proof (inputs#I1)
- One contended reading on the present backing (red: the start stalls) and one on the new backing in the same
  contention (green), each bounded in size and time. Decided at P4 (inputs#I6): inside one natural window, the red
  is the start probe's stalled write on the shared volume, where the homes were, beside the recorded 2026-10-06
  reds of real starts; the green is the verify-driven test binary on the new backing in the same seconds.
- Before any writer this chunk starts on the shared volume runs, the operator is told: what it writes, where, how
  much, for how long. The alternative the operator names is a reading inside another builder's natural link with
  `start_probe.py` (inputs#I4), where this chunk adds no load beyond the probe's own writes.
- The reading is recorded in the chunk's `evidence/`.

### W3 — the documents that state where homes live
- Every statement that the homes live under the workspace, and every CI glob that reads `target/e2e-home/**`, is
  read at P2/P3 and either holds as written under the chosen backing or is named for the wrap's amendment.

## Premises carried by the freight (closed at P3, `research.md`)
- "measured at `af179a9`": a test start copies the running exe into its home, `pin_exe` through the private replace
  that ends in `sync_all()`; the exe is 52 118 312 B; `TestHome::new` puts every home under
  `<workspace>/target/e2e-home`, on the btrfs volume every builder shares; the host's `vm.dirty_bytes` is
  268 435 456. Verified at HEAD `b329083`: `crates/viola-state/src/pin.rs:71-83`,
  `crates/viola-state/src/fs.rs:267`, `tests/support/home.rs:117-126`; `ls -la target/debug/viola` reads
  52 118 312; `sysctl vm.dirty_bytes` reads 268 435 456; `findmnt -T target/e2e-home` reads btrfs
  `/dev/mapper/root[/@home]`. Both `viola run` (`src/cmd/run.rs:316`) and `viola verify` (`src/cmd/verify.rs:133`)
  pin, so a stamped home takes its copy at the fixture's `viola verify`.
- "Relayed, measured by overseer1 on 2026-10-07 inside two natural links of another builder and not re-run in this
  repository" (inputs#I3): one 50 MB write alone waited 13.0 s (0.006 s on a quiet host); 25 at once took 14.7 s
  with no fsync at all; the same 25 on tmpfs in the same seconds took 0.48 s. Verified against the snapshot (a
  pointer at andromedaV3 `1ed4a6ab`: its second-window table reads 13.01 s, 14.69 s and 0.48 s, its quiet-host row
  0.005 to 0.006 s). Still not re-run in this repository.
- "the reds of the 2026-10-06-local-command-and-paste-framing-rows wrap read ~18 s against nextest's 20 s kill".
  Verified: that chunk's `evidence/block-reds-host-contention.md` records a stall of about 18 s to 19 s per start
  and 22 tests at the 20 s kill; `.config/nextest.toml:31-33` gives the twelve verify-driven binaries 10 s × 2.
- **hypothesis:** "backing `target/e2e-home` with tmpfs on the dev host, the path unchanged, ends the stalled-start
  reds, and removing `sync_all()` would not (the no-fsync reading above)". What P3 re-established: the pinned copy
  is one buffered write of the whole exe per home, and inputs#I3 measured such a write stalled by another builder's
  link and unstalled on tmpfs. What P3 did not establish: that the pinned copy is the only write of a real start
  that stalls (the 2026-10-06 evidence says it "was not separated here", and a verify-driven start also writes
  small probe dirs under its cwd, on the shared volume). So it stays this chunk's hypothesis, W2's reading on real
  starts is its test, and a red there is a finding to report, not a result to work around.
- `/tmp` on this host is tmpfs (inputs#I1). Verified with `findmnt`: 31.3 G, 24.3 G available at 06:22Z (25.5 G at
  06:14Z: other sessions write there), options `rw,nosuid,nodev,usrquota` (no `noexec`), aged by systemd-tmpfiles
  (`q /tmp 1777 root root 10d`). Beside it: `/dev/shm` tmpfs 31.3 G, 31.1 G available, the same options, not aged;
  the per-user runtime dir tmpfs 6.3 G, mode 0700, owned by the user, cleared at logout.
- A batch of about 25 starts puts 25 × 52 118 312 B = 1.30 GB into the backing at once, transient (inputs#I3).
  Kept homes (`AGENT_RUN_KEEP_HOMES` / `AGENT_RUN_KEEP_FAILED`) stay there until removed: a kept home's owner
  record is deleted (`tests/support/home.rs:173-176`), so no sweep takes it.
- The headless harness puts its homes under the same path. Verified: `crates/viola-e2e/src/harness/mod.rs:84-86`
  (`e2e_home`), `boot.rs:128-131` (`viola-session-*`), `run.rs:370-373` (`viola-live-<pid>`). Both test-side
  homes of the path derive it with no environment read (`tests/support/home.rs:228-230`, a compile-time manifest
  dir; the harness's workspace root), so a path-level backing reaches `pre-push`'s `env -i` children unaided.
- `[premise-corrected: the three text hits are test code and a comment, no product branch reads the path]` Product
  code does not name `e2e-home`: `crates/viola-state/src/strict.rs:372-384` and `fs.rs:461-473` are Windows-only
  tests that create a home there, `fs.rs:456` a path literal in a unit test, `src/cmd/run.rs:925` a test comment.
  On Unix no product code canonicalises a home (`canonicalize` appears only in Windows arms and in the fake
  agent's cwd / trusted-root compare), and the strict-modes reader follows a link like `stat`
  (`strict.rs:118-122`) and reads `ledger/` and `ledger/stamps.json` only, never an ancestor of the home.
- The backing is a Linux dev-host matter only: Windows, macOS and CI runners keep the present one. Verified for a
  path-level backing (a mount or a link exists on no runner; `ci.yml` is not edited).
- What is in `target/e2e-home` today survives the change or is dispositioned by the operator. Verified by listing:
  13 entries, 545 MB (8 `viola-record-*`, `viola-session-UIPQKC`, `viola-test-SZkaMS`, `viola-test-WYNVH7` from
  the handoff's operator desk, 2 `vt-*`). The plan moves the directory aside whole and deletes nothing.

## Boundaries
- No live `claude` session, no ledger row, no fixture.
- No change to what `viola` does on a start: `pin_exe`, `replace_private`, the modes, the strict-modes checks.
- No host configuration is changed by the builder: a mount, an `/etc/fstab` or systemd unit, a sysctl is the
  founder's hand at the desk.
- Nothing in the Andromeda tree or another builder's tree; F167's host-wide candidates (a gate lease, a dirty
  budget per builder, a standing host record) are the founder's and not this chunk's.
- The proof starts no writer on the shared volume without the operator's word first.

## CI read at Setup (what shipped since the last flip)
- `b329083e0266` — green · checks 15/15 · wall 409 s · ci#37579512236.
- `af179a9c0cfd` — green · checks 15/15 · wall 462 s · ci#37550551341.
No red, nothing to disposition.

## Capabilities
None named by the entry. The claimable pool is read at P4.
