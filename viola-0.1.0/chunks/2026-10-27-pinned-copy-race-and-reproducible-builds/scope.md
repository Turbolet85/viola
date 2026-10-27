# Scope — Pinned copy race and reproducible builds (2026-10-27-pinned-copy-race-and-reproducible-builds)

**Working entry (viola-0.1.0/working-route.md, Epoch 6 — Security hardening):** Pinned copy race and reproducible
builds — the pinned copy of the binary never replaced under a concurrent first start (the `pin_exe` race on a fresh
home), and whether a rebuild of one tree can read as another binary (two clean release builds of one tree, compared by
file and by section); carved out of «Home and code-bearing file integrity» with its two CARRYs verbatim by the
founder's own word at its P4 (`viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/inputs/I3-relay-1.md.txt`,
Q4, read at this take-up: «Вынести оба (Рекомендую)», option 1, carve both out into this entry right after that one in
Epoch 6) and his P5 «Да» on the card (`viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/inputs/I4-relay-2.md.txt`,
read: the word on `.andromeda/runs/2026-10-26T16-59-13-phase/p5-card-1.md`, its Beyond-the-plan row 1 this entry).

**Invocation:** the operator's `/andromeda-phase Proceed.`, with the overseer's directive for this phase (`inputs#I1`):
an ordinary chunk, no forced sample (the natural sample draw stands); every subagent and Workflow transcript and the
run dir kept; a hard cascade failure (a Workflow launch, a batch or a P2 step failing hard, agents writing nothing, or
agents doing the orchestrator's work, F352) stops the phase with the run dir reported and no re-spawn around it; a
founder fork (a boundary widening, a live-session cap, a route split) is asked as a P4 dialog, the overseer holding it
in the away form, a delegate's answer recorded PROVISIONAL; standing: no epoch boundary inside Epoch 6, and
`df -h /tmp` read before a stamped-home gate.

## What it builds

### A. The pinned copy never replaced under a concurrent first start (CARRY 16)
- **The product race closed.** Two first `viola run`s of one version starting together on a fresh home must not both
  replace `bin/<key>/viola(.exe)`, and no agent's hook spawn of the pinned path may fail because another start's write
  of it is in flight. The surface is `viola_state::pin::pin_bytes` / `pin_exe` (`crates/viola-state/src/pin.rs`; the
  absent-copy arm calls `replace_private_shared(&path, bytes, DIR_MODE)`), reached from `run`'s start
  (`src/cmd/run.rs`, `pin_bytes`) and `viola verify` (`src/cmd/verify.rs`, `pin_exe`). The home copy of the binary is
  this entry's by the CARRY ("this entry owns the home copy of the binary").
- **The measured fact it rests on** (folded verbatim from the CARRY, `viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/evidence/unstamped-boot-race.md`,
  read at this take-up): two first `viola run`s of one version on a fresh home both find `bin/<key>/viola` absent and
  replace it; one agent's first hook spawn then fails (receipt `ran:false`) and its `session-start` never lands —
  6 of 24 concurrent unstamped two-instance harness boots failed, 0 of 24 single-instance.
- The mechanism, the folded hypothesis "the kernel error is not read — hypothesis: `ETXTBSY`" re-derived at P3
  (research.md, Mechanism re-derivation): viola's half verified at HEAD — `replace_private_retrying` renames through
  `NamedTempFile::persist` while it still holds the temp file's write handle (`crates/viola-state/src/fs.rs:288–303`,
  `tempfile-3.27.0/src/file/mod.rs:767–779`), so after a second start's rename `bin/<key>/viola` is an inode that
  process holds open for write; the kernel's half verified by a scratch control on the Linux dev host (an exec racing
  that window: `ETXTBSY` 4059 of 16283; the handle closed before the rename: 0 of 13066). Still not measured: the
  errno of the CARRY's own failed product boots (the fake agent receipts `ran:false` only) — the plan's first reading.
- A two-sided witness: the race forced open, never sampled (`.claude/rules/testing.md` 2026-09-27), red
  before the fix and clean after it, plus the CARRY's unstamped two-instance product reproduction re-read red at HEAD
  with its errno and clean after, beside the single-instance control — a witness that cannot pass vacuously.
  Verified feasible at P3: `replace_private_with` already takes its decision from the caller (`fs.rs:271`), the form
  a forced-window hook takes; `harness_session` boots unstamped two-instance sessions (`SessionOptions`, `session.rs:16`).
- The Windows arms, read at P3: the exe arm's rename over a running image is refused and retried, then accepted as
  same bytes, but the placing start still holds its write handle on the inode it placed, so the closed-handle,
  no-clobber placement covers Windows too (read on the `windows-2025` runner only); `pin_companions` holds each
  companion open `FILE_SHARE_READ`-only before any launch and is left unchanged.
- `plugin/<key>/`, read at P3: `hooks.json`, `.mcp.json` and `plugin.json` are read, never executed, and a rename
  gives a reader either whole file; the per-start rewrite keeps `replace_private_shared` unchanged.
- The test-side workaround stands: "a stamped home is pinned by `viola verify` first, and the test-side
  workaround (Path 7 boots one instance) stands" (the CARRY); verified at P3: `mcp_tools_path7_unverified_cli_answer`
  boots `builder` alone with the race comment at `crates/viola-e2e/tests/mcp_tools.rs:673–674`, and the evidence file
  says "Path 7 needs no other instance"; whether Path 7 returns to two instances once the race is closed is P4's.
- The hash-before-reuse rule stands unchanged: a present copy is re-hashed and refused on any difference, never
  overwritten (security.md §Local trust boundary, Home; `pin_exe`'s doc: "a copy that no longer matches is never run
  or overwritten").

### B. Whether a rebuild of one tree can read as another binary (CARRY 11) — a measurement
- **Two clean release builds of one tree, compared by file and by section** — the entry's own wording; this entry's
  to measure and the founder's to act on (the CARRY: "this entry's to measure … and the founder's to act on").
- **The measured fact it rests on** (folded verbatim from the CARRY, `viola-0.1.0/chunks/2026-10-09-inner-cr-and-crlf-in-a-sent-text/evidence/live-preconditions.md`,
  read at this take-up, its §"The product build after the whole block"): the build was repeatable for a given source
  text (three builds at sha256 prefix `4057b91d84010eca`), the hash moved with comment text alone, and the first
  build's file (`b4659b98029d0f94`, 4 000 408 B) was not reproduced when its comment text was put back
  (`b4fc939b7085eb39`, 4 000 416 B). The two files differ in the linker's build-id note and in three compiler-named
  local symbols; `.text`, `.rodata`, `.data`, `.data.rel.ro`, `.eh_frame` and `.gcc_except_table` hash equal.
- "The cause is not known, and no record guesses one" (the CARRY, verbatim): this chunk records no cause it
  has not measured; the measurement says whether two CLEAN builds (each from an empty target dir) of one committed
  tree give one file, and, where they do not, which sections and notes differ.
- Why it matters to the product, read at P3: a rebuild of one tree that reads as another binary pins a second
  `bin/<key>/` and `plugin/<key>/`; the readers of the key are `checked_exe`'s home-tree check
  (`src/cmd/run.rs:199`), the plugin folder (`src/cmd/run.rs:460`), the snapshot's `pinned_bin` and the test-side
  `target/conpty-seed/<key>/`; `ledger/stamps.json` is keyed by CLI version, so no stamp keys apart.
- Where it is measured: on the Linux dev host at least, through `scripts/release-check.sh`'s build (the
  release profile and `CARGO_TARGET_DIR` it uses); verified at P3: the script builds `--release --locked --bin viola`
  and needs `jq` (host 1.8.2), `objcopy` / `readelf` 2.47 are on the host, `rustc 1.98.1` matches
  `rust-toolchain.toml`, and `[profile.release]` sets only `panic = "unwind"`; whether CI's three OSes are read too is
  P4's.
- Any product or build change on the measurement's basis (a linker flag, a path remap, a profile key) is
  the founder's to act on — a fork at P4, never a decisive lean.
- The first build's copy stood only on a tmpfs (the CARRY, verbatim): it is gone; the measurement starts fresh.

### C. CI read at Setup (step 5a), settled at P3
- `ci#40425866096` on `99977116182b` (the last wrap's flip commit, the only sha from that flip through HEAD): at Setup
  `in progress` (`CI 99977116: verdict not yet available`); re-read at P3: `verdict: red · checks 15/15 · first-fail
  +678 s test (windows-2025) · wall 720 s`, failed `test (windows-2025)` and `test (ubuntu-latest)`. The prior pre-CI
  run `ci#40403389629` on `e3468500` read `verdict: green · checks 15/15` (the handoff). The operator asked both reds
  be given a basis and an owner (`inputs#I2`; the overseer's reading `inputs#I3`).
- Red 1, intersecting this chunk (Path 1 drives `pin_bytes`'s re-hash at its step 5 and its absent-copy arm
  at its step 6): `ci#40425866096`, sha `99977116182b`, job `test (ubuntu-latest)`: `TIMEOUT [  20.089s] ( 550/3220)
  viola::path1_run_start path1_run_start_one_home_in_order`. Closed against that run: still failing there →
  VERIFIED, coordinates kept. [premise-corrected: the overseer's reading that "a first-start stall is the shape of
  CARRY 16" (`inputs#I3`) — the run's kept home shows one stamped home, no concurrent first start, and the flow in
  step 5 when the 20 s kill came, before the absent-copy arm; Path 1 reads 13.468 s green (`ci#40403389629`) against
  20.089 s red on a runner whose 3220 common cases summed 1784.5 s against 1631.6 s (research.md, CI reds)]. Its
  cause is Path 1's two-thirds use of its kill line under a runner-wide slow tail; owned here, its phases measured on
  the runner and the slow work removed, never the kill line raised (`.claude/rules/testing.md` 2026-09-28).
- Red 2, not intersecting what this chunk builds: `ci#40425866096`, sha `99977116182b`, job
  `test (windows-2025)`: `viola-state strict::win::tests::strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry`
  panicked at `crates\viola-state\src\strict.rs:778:13`. Closed against that run: still failing there → VERIFIED.
  Basis: the product's `Err(Writable)` held (line 764); the oracle names Users `S-1-5-32-545` where that runner's
  inherited ACL granted the write to Authenticated Users `S-1-5-11` (image `windows-2025-vs2026` 20261021.278.1 on
  both runs, regions northcentralus red · westus3 green). Folded here on the operator's word at the P4 fork
  (`inputs#I4`, delegate overseer1, PROVISIONAL): the oracle fixed test-only, with its own acceptance criterion and a
  witness that cannot pass vacuously (its remove-the-guard control keeps its red).

## Boundaries
- The pinned-binary re-hash, the per-start plugin rewrite and the home-tree strict-modes check landed at
  «Home and code-bearing file integrity»; this chunk changes none of their rules, only the absent-copy write's
  concurrency.
- Unix socket and socket-directory hardening, the client euid check and `board::scan`'s one home rule are
  «Unix endpoint and home hardening»'s.
- Bounded readers are «Bounded inputs at every boundary»'s; release packaging and the version's gates on three OSes
  are «Version done-check»'s.
- No Windows-host live measurement (none exists; «Windows-only live measurements» is BLOCKED-ON one); Windows is read
  on the CI runner only.
