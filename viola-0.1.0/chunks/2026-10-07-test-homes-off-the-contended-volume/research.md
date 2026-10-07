# Codebase Research — 2026-10-07-test-homes-off-the-contended-volume

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 11
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, Session Additions included
  (8 additions; applied: 2026-09-25 never pipe `boot`, 2026-09-27 the `binary(<stem>)` filter form, 2026-09-29 the
  two-sided local reading and its 2026-10-06 extension on a host stall); `.claude/rules/testing.md` — read in full
  (31 additions; applied: 2026-09-27 force a timing window open and show red before and green after,
  2026-09-28 a timing red is never fixed by a bound, 2026-10-04 no mutation entry in a chunk's gate block).
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg.
- **External inputs:**
  - `inputs#I1` — the operator's take-up directive (a verbatim copy);
  - `inputs#I2` — the overseer's relay, §4 item (E), behind `working-route.md:94`;
  - `inputs#I3` — overseer1's F167 measurement (a pointer at andromedaV3 `1ed4a6ab`);
  - `inputs#I4` — `start_probe.py`, the F167 start probe (a pointer at the same commit);
  - `inputs#I5` — `window_watch.py`, the F167 natural-window watcher (a pointer at the same commit; snapped at P3);
  - `inputs#I6` — the P4 fork answers (a verbatim copy): the founder's live word for the link with the keeper,
    relayed by the overseer, and the overseer's answer on the proof.

## Measured facts
Host readings are of 2026-10-07T06:14Z to 06:30Z on the Linux dev host; scratch probes ran in the session
scratchpad, which is on tmpfs, and wrote nothing in the tree.

### Where a home's path comes from
- Root chain: `TestHome::new` joins `target/e2e-home` to a compile-time manifest dir and takes
  `tempdir_in(base)` with prefix `viola-test-` (`tests/support/home.rs:117-126`, `:228-230`). No environment read.
- Harness: `Workspace::e2e_home` is `<root>/target/e2e-home` (`crates/viola-e2e/src/harness/mod.rs:84-86`); `boot`
  creates it and takes `tempdir_in` with prefix `viola-session-` (`boot.rs:128-131`); `--local-live` joins
  `viola-live-<pid>/home` and creates nothing itself (`run.rs:370-373`).
- So a backing set at the path `target/e2e-home` is seen by every home class and by `pre-push`'s `env -i`
  children, and a backing chosen through a variable is seen by none of those children.
- Two homes are elsewhere by design and stay there: `TestHome::outside_scan` (`viola-chaos-*` under the system temp
  dir, `tests/support/home.rs:139-150`) and the socket-path dir of `tests/channel_endpoint.rs:45` (`/tmp`).
- Under `env -i` the Unix socket dir is already `/tmp/viola-<uid>/`
  (`crates/viola-channel/src/endpoint.rs:39-45`): test-side viola state lives on that tmpfs today.

### What a start writes
- `pin_exe` reads the exe whole and writes it through `replace_private_shared` when the copy is absent
  (`crates/viola-state/src/pin.rs:71-83`); the helper writes a temp file beside the target, sets its mode, writes,
  `sync_all`s (`fs.rs:267`) and renames. Callers: `pin_and_plugin @ src/cmd/run.rs:316` and
  `measure @ src/cmd/verify.rs:133` (graph, below). A stamped home is pinned by the fixture's `viola verify`.
- `target/debug/viola` is 52 118 312 B (`ls -la`). 25 starts at once are 1 302 957 800 B (25 × that size).

### The product and a linked or mounted home
- No product code canonicalises a home on Unix: `grep -rn canonicalize src crates` (viola-e2e excluded) reads
  `crates/viola-pty/src/sideload.rs:84,114` and `crates/viola-state/src/fs.rs:124,130` (Windows arms) and
  `src/bin/viola-fake-agent.rs:154` (the fake agent's cwd against its trusted root, neither a home). `pin_exe`
  takes `std::path::absolute`, which resolves no link (`pin.rs:75`).
- The strict-modes reader follows a link as `stat` does (`crates/viola-state/src/strict.rs:118-122`) and is called
  on `ledger/` and `ledger/stamps.json` only (`strict.rs:29-39`): no ancestor of a home is read.
- The three product-crate text hits for `e2e-home` are tests and a comment: `strict.rs:372-384` (inside the
  Windows-only `win` module's tests), `fs.rs:461-473` (`#[cfg(windows)]`), `fs.rs:456` (a literal in a unit test),
  `src/cmd/run.rs:925` (a test comment).
- Tests that compare a home path already canonicalise both sides where they read the kernel's view:
  `tests/tui_channel_fds.rs:43` (the `/proc/<pid>/fd` table) and `tests/cli_verify.rs:740`.
  `tests/cli_fake_agent.rs:789-790` compares the home with `workspace_path("target/e2e-home")` as strings, which a
  link leaves equal. Whether the whole suite passes over a link is not measured here; it is the plan's first
  reading under the link option.

### What walks `target/e2e-home`
- The owner sweep: `fs::read_dir(base)` then per-entry `file_type().is_dir()` (`tests/support/home.rs:48-75`). A
  `read_dir` of a link to a directory lists the target; the entries are real directories on either backing.
- G4 and the secret scan: `read_dir(root)` then `symlink_metadata` per entry
  (`crates/viola-e2e/src/harness/schema_check.rs:34-53`, `secret_scan.rs:209-221`). A link at the root is followed
  by the first `read_dir`; a link below it would not be.
- Harness `cleanup` removes `<e2e-home>/viola-session-*` after a string compare of the parent's parent with
  `ws.e2e_home()` (`cleanup.rs:155-168`): nothing canonicalises, so the compare holds on a link or a mount.
- Removals through a link (the security extract's question): yes. The sweep lists `base` and removes dead-owner
  dirs under it (`tests/support/home.rs:50-72`), a test home's drop removes its own dir (`:162-179`), and harness
  `cleanup` removes the session dir (`cleanup.rs:166`). With `target/e2e-home` a link, each of these creates and
  deletes in a directory outside the workspace. The harness's one sanctioned wipe outside the workspace is the
  guarded Windows mutation scratch (security extract, per security-plan §Security Anti-Patterns → Code Patterns);
  the removals here take only dirs the harness itself created (by owner record, by its own `TempDir`, by the
  `viola-session-` name), but the far side of the link is not the workspace. A mount keeps every such path inside
  the workspace as the kernel resolves it. `.andromeda/playbook.md:36-38` ("Boundary widening", verdict
  `escalate`) and `:49-51` (what ratifies it) are the patterns a link option is read against at P4.
- G2: `find "$scope" -path '*/diagnostics/*.ndjson' …` with `scope="$root/target/e2e-home"`
  (`scripts/g2-zero-panics.sh:18-28`, `:35`). Measured in the scratchpad with this host's `find` (bfs 4.1.1): a
  start point that is a link to a directory holding one matching file prints 0 files; the same start point with a
  trailing slash prints 1; with `-H` prints 1. So over a linked `target/e2e-home` a local G2 reads its fail-closed
  `empty scope`. G2 runs in CI's `test` and `perf` jobs (`ci.yml:136`, `:230`) and, locally, only by hand
  (`.claude/docs/commands.md:70-75`); `pre-push` does not run it (`grep -rn 'g2\|schema.check\|secret.scan'
  crates/viola-e2e/src/harness/pre_push.rs crates/viola-e2e/src/harness/pre_push/ crates/viola-e2e/src/harness/gate.rs`:
  0 hits).
- CI: `ci.yml` is the only workflow naming the path (`grep -ln e2e-home .github/workflows/*.yml`: `ci.yml`), at
  `:160` and `:253` (the `diag-*` upload globs), with `AGENT_RUN_KEEP_HOMES: "1"` at `:28` and `:211`. No runner
  has a link or a mount, so nothing CI executes changes under either backing.

### A link's failure modes
- A dangling link: measured in the scratchpad, `makedirs(link, exist_ok=True)` over a link whose target is gone
  raises `FileExistsError` (`mkdir(2)` EEXIST; the path is a link and not a directory). Rust's
  `fs::create_dir_all` takes the same arm (an `mkdir` error on a path that is not a directory is returned), so
  after a reboot clears a tmpfs target, `fs::create_dir_all(&base).expect("e2e-home")` at
  `tests/support/home.rs:119` and `boot.rs:128` fail every start until the target is re-created. This is the
  equality a keeper rests on, and its remove-the-guard reading is the keeper's own red case.
- `cargo clean` removes the link with `target/` (`.claude/docs/commands.md:91` names what it removes today); the
  next start then creates a plain directory on the shared volume and the stall returns with no signal.
- `/tmp` is aged by systemd-tmpfiles (`/usr/lib/tmpfiles.d/tmp.conf:11`: `q /tmp 1777 root root 10d`): a file
  untouched for ten days is removed, which reaches a home kept that long and no live one.

### The tmpfs candidates on this host (`findmnt`)
| mount | size | available | options | note |
|---|---|---|---|---|
| `/tmp` | 31.3 G | 24.3 G at 06:22Z (25.5 G at 06:14Z) | `rw,nosuid,nodev,usrquota` | aged 10 d; shared with every session's scratch; no `noexec` |
| `/dev/shm` | 31.3 G | 31.1 G | `rw,nosuid,nodev,usrquota` | not aged; no `noexec` |
| the per-user runtime dir | 6.3 G | 6.2 G | `mode=700`, the user's uid | cleared at logout; 5 batches' worth |

RAM is 62 G with 40 G available, swap 125 G (`free -g`). The host was booted 2026-10-04T19:09Z (`uptime -s`).

### What is in `target/e2e-home` today
13 entries, 545 MB (`ls -la`, `du -sh`): 8 `viola-record-*`, `viola-session-UIPQKC`, `viola-test-SZkaMS`,
`viola-test-WYNVH7` (the handoff's operator desk), `vt-21w84iv1`, `vt-i7sy6r2a`. A mount over the directory hides
them; a link needs the directory moved aside first.

### The recorded red
`viola-0.1.0/chunks/2026-10-06-local-command-and-paste-framing-rows/evidence/block-reds-host-contention.md`: entry
16 (the five binaries) read `117 tests run: 93 passed (5 slow), 2 failed, 22 timed out` at 21:17:39Z to 21:18:20Z
while a sibling build wrote 88 files, 10 606 MB to the volume; the stall was about 18 s to 19 s per start; "which
write of the start it is … was not separated here". `.config/nextest.toml:31-33` gives the twelve verify-driven
binaries a 20 s kill (10 s × 2); `tests/support/watch.rs` `WITHIN` is 7 s.

### The quiet reading of the verify binary
`bash scripts/agent-run.sh run --integration --filter 'binary(cli_verify)'` at HEAD `b329083e0266`, 07:28:18Z to
07:28:40Z, `Dirty` 880 kB before it: exit 0, `"ok":true`, `"suite":"nextest-integration","passed":29,"failed":0`,
22 s wall with the harness build fresh. Its document carries an absolute `artifact` path, so a record of it in
`evidence/` copies the numbers, never the document.

### The harness's closed failure lists a keeper must not extend
- `boot`: a failed `start` reads `build-failed` (`crates/viola-e2e/src/harness/boot.rs:112`), which is where the
  present `fs::create_dir_all(opts.ws.e2e_home())?` at `:128` already lands.
- `run --local-live`: `build`, `verify-exit-<n>`, `row-missing`, and `verify-exit-none` for a verify with no exit
  code (`crates/viola-e2e/src/harness/run.rs:353-387`).
- `viola-e2e` depends on `viola-core`, `viola-pty` and `viola-channel`, not on `viola-state` or `libc`
  (`crates/viola-e2e/Cargo.toml` `[dependencies]`); std's Unix metadata extensions are already used there
  (`secret_scan.rs:240-241`). `grep -rn 'ensure_e2e_home\|backed_base\|e2e_home_backing_\|home_base_backing_' src
  crates tests`: 0 hits, the names are free.

### The probe and the watcher (inputs#I4, inputs#I5)
- `start_probe.py <dir> --payload <file> [--n 25] [--rounds 3] [--no-fsync] [--label text]` mirrors `pin_exe` +
  `replace_private` (read, write a temp file, fsync, rename), N processes at once, writes only under
  `<dir>/probe-<pid>/` and removes it, and prints one JSON line per round with `write`, `fsync` and `total` as
  `min` / `med` / `max` seconds. At `--n 5` one round writes 5 × the payload: 260 MB for the viola exe.
- inputs#I3 read, inside one natural link: 25 starts with no fsync `write()` med 14.69 s on btrfs against 0.48 s
  total on tmpfs in the same seconds; one start alone `write()` 13.01 s; a batch of 5 beside a link 12.47 s ("the
  stall does not shrink with the batch").
- `window_watch.py` opens a window when `Dirty + Writeback` stand at or above 100 MB on two consecutive one-second
  reads of `/proc/meminfo` while a build process of another project is alive and viola has none; it then fires
  the probe variants and records a `/proc/pressure` and build-process snapshot before and after each. Its variants
  are fixed to F167's; this chunk's reading needs its own driver on the same trigger.
- At 06:30Z the host was quiet: `Dirty` 312 kB, io pressure `avg10=0.00`, no `rust-lld` alive. inputs#I3 counts 79
  gate runs of the heaviest sibling in 58 h, so a natural window is a wait of unknown length, not a certainty.

## Files inspected
- `tests/support/home.rs` (full) — `TestHome::new`, the sweep, `remove_owned`, the keep path, `workspace_path`.
- `crates/viola-state/src/pin.rs` (66-86) — `pin_exe`.
- `crates/viola-state/src/fs.rs` (248-270, 448-476) — the one replace helper; the Windows-only home tests.
- `crates/viola-state/src/strict.rs` (1-128, 364-388) — the strict-modes rule and its Unix reader.
- `crates/viola-e2e/src/harness/mod.rs` (70-95) — `Workspace::e2e_home`.
- `crates/viola-e2e/src/harness/boot.rs` (120-140) — the session home's creation.
- `crates/viola-e2e/src/harness/cleanup.rs` (150-175) — `remove_session_home`.
- `crates/viola-e2e/src/harness/schema_check.rs` (30-60), `secret_scan.rs` (205-230) — the two walkers.
- `crates/viola-e2e/src/harness/run.rs` (362-376) — `local_live`'s home.
- `scripts/g2-zero-panics.sh` (1-45) — G2's walk.
- `.config/nextest.toml` (full) — the kills.
- `.github/workflows/ci.yml` (grep) — the keep variable, the gate steps, the two upload globs.
- `src/cmd/run.rs` (920-931), `tests/channel_endpoint.rs` (36-48), `crates/viola-channel/src/endpoint.rs` (grep).
- `viola-0.1.0/chunks/2026-10-06-local-command-and-paste-framing-rows/evidence/block-reds-host-contention.md` (full).

## Graph impact (from the code-graph query; rust plane, trace `tree-query-2026-10-07-test-homes-off-the-contended-volume.json`)
- **e2e_home** — 12 call sites (query 1, `rows: 12`): `start @ crates/viola-e2e/src/harness/boot.rs:128` and
  `:131`, `remove_session_home @ cleanup.rs:164`, `local_live @ run.rs:371`, `schema_check @ schema_check.rs:82`,
  `of @ secret_scan.rs:50`, five test sites (`cleanup.rs:344,352,355`, `mod.rs:299`,
  `crates/viola-e2e/tests/cli.rs:95`, `tests/harness_lifecycle.rs:126`). A keeper beside `e2e_home` is called
  from the two creating sites (`boot.rs:128`, `run.rs:371`); the readers and the remover need none.
- **workspace_path** (`tests/support/home.rs`) — 67 call sites (query 2, `rows: 67`), one of them the home base
  (`new @ tests/support/home.rs:118`); the rest resolve fixtures and schemas. The function itself does not change.
- **pin_exe / replace_private_shared / sweep_gone_owners / remove_owned** — 30 rows (query 3): product callers
  `pin_and_plugin @ src/cmd/run.rs:316` and `measure @ src/cmd/verify.rs:133` for the pin, `pin_exe @ pin.rs:82`
  and `pin_and_plugin @ run.rs:329` for the shared replace; the sweep's one caller is
  `new @ tests/support/home.rs:121`, with `fixture_sweep_removes_only_homes_whose_owner_is_gone @
  tests/cli_instance_state.rs:449` as its test. None of these changes.

## Patterns detected
- **A start-time write too slow under load, answered test-side with no bound moved**
  (`tests/support/home.rs:182-225`, `seed_conpty`): the nearest precedent, recorded as a dated carve-out.
- **A host environment fact outside harness code** (architecture §Occupied Resources → Repository, the
  `viola-mutants-scratch` entry, per the arch extract): the Linux dev host's mutation `TMPDIR` is recorded with its
  measurement and no document prints the path.
- **A per-user 0700 dir under a world-writable parent, verified before use**
  (`crates/viola-channel/src/endpoint.rs:39-45` names it; security-plan §Authentication & Authorization sets the
  `lstat` rule): the shape a link's target takes.
- **Support code is tested from a root test file** (`tests/cli_instance_state.rs:449` tests the sweep of
  `tests/support/home.rs`).

## Conventions to follow
- **Homes are created by viola, never by the fixture** (`tests/support/home.rs:106-107`): a keeper creates the
  base directory's backing only, never a `home`.
- **A removal never follows a path it did not verify** (`cleanup.rs:155-168` refuses any path but a
  `viola-session-*` under `e2e_home`): a keeper creates and verifies, and deletes nothing.
- **Test-only environment reads carry `AGENT_RUN_`** (`tests/support/home.rs:27-29`): none is added.
- **A two-sided timing reading goes to `evidence/` with counts and seconds** (`.claude/rules/testing.md`,
  2026-09-27 and 2026-09-28 additions); no suite member carries an elapsed-time verdict.

## New files to create
- `viola-0.1.0/chunks/2026-10-07-test-homes-off-the-contended-volume/evidence/` — the readings: the home dir moved aside, the backing as set, the suite on it, the contended reading, the driver's text

## Files to modify
- `tests/support/home.rs` — the keeper: the base's backing re-created when the link's target is gone, verified before use
- `tests/cli_instance_state.rs` — the root keeper's cases
- `crates/viola-e2e/src/harness/mod.rs` — the same keeper beside the path function, its cases
- `crates/viola-e2e/src/harness/boot.rs` — the session home's creation calls it
- `crates/viola-e2e/src/harness/run.rs` — the local-live home's start calls it
- `scripts/g2-zero-panics.sh` — the walk's start point written so a linked scope is descended

## Open questions
- Which backing: a link the builder sets with no root (with or without a test-side keeper), or a mount the founder
  runs at the desk → blocks: plan-decision (the P4 fork the operator asked to have priced). Answered at P4
  (inputs#I6): the link with the keeper; the lists above are that option's files.
- What stands as the red side of the proof: the control probe on the shared volume in the same window beside the
  recorded 2026-10-06 reds, or a fresh real-suite red on the present backing in a window of its own → blocks:
  plan-decision. Answered at P4 (inputs#I6): one natural window, capped at 60 minutes.
