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
- [inferred] The mechanism, kept with its marker verbatim: "the kernel error is not read — hypothesis: `ETXTBSY`"
  (the evidence file: "`ETXTBSY` on an exec racing a write handle is the likely one, a HYPOTHESIS"). P3 reads what
  `replace_private_shared` → `replace_private` holds open across the rename (`crates/viola-state/src/fs.rs`: a temp
  file written, `sync_all`, then `persist`) and what the second start's replace does to a path the first start's
  agent may already be spawning; the fix rests on the mechanism P3 or a two-sided reproduction establishes, never on
  the hypothesis alone.
- [inferred] A two-sided witness: the race reproduced red before the fix (the CARRY's unstamped two-instance boot,
  or a narrower concurrent-pin control) and read clean after it on the same rig, with the 0-of-24 single-instance
  control beside it — a witness that cannot pass vacuously (the race window must be shown reachable by the control).
- [inferred] The same absent-then-replace shape in `pin_companions` (Windows x64, `bin/<key>/conpty/`) already waits
  out `ERROR_SHARING_VIOLATION` (measured 5–11 ms on two concurrent first starts); whether the exe's arm on Windows
  needs the same, or is already covered by `replace_private_shared`'s same-bytes acceptance, is P3's to read.
- [inferred] `plugin/<key>/` is rewritten whole at every start through the same `replace_private_shared`; whether a
  concurrent start's rewrite can fail a reader of `hooks.json` / `.mcp.json` is P3's to read (no measurement names it).
- [inferred] The test-side workaround stands: "a stamped home is pinned by `viola verify` first, and the test-side
  workaround (Path 7 boots one instance) stands" (the CARRY); whether Path 7 returns to two instances once the race
  is closed is P4's to decide, never assumed.
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
- [inferred] "The cause is not known, and no record guesses one" (the CARRY, verbatim): this chunk records no cause it
  has not measured; the measurement says whether two CLEAN builds (each from an empty target dir) of one committed
  tree give one file, and, where they do not, which sections and notes differ.
- [inferred] Why it matters to the product, from the CARRY: "the pinned binary's directory is named by that hash and
  the binary is re-hashed before reuse" — a rebuild of one tree that reads as another binary pins a second
  `bin/<key>/` and `plugin/<key>/` and is keyed apart from its own stamps or snapshots wherever those read the key;
  P3 reads which readers key on the hash.
- [inferred] Where it is measured: on the Linux dev host at least, through `scripts/release-check.sh`'s build (the
  release profile and `CARGO_TARGET_DIR` it uses); whether CI's three OSes are read too is P4's.
- [inferred] Any product or build change on the measurement's basis (a linker flag, a path remap, a profile key) is
  the founder's to act on — a fork at P4, never a decisive lean.
- The first build's copy stood only on a tmpfs (the CARRY, verbatim): it is gone; the measurement starts fresh.

### C. CI read at Setup (step 5a)
- `ci#40425866096` on `99977116182b` (the last wrap's flip commit, the only sha from that flip through HEAD): verdict
  `in progress` · checks 15/15 · its oldest running job `lint (ubuntu-latest)` at 200 s when read — `CI 99977116:
  verdict not yet available`, never read as green; no wall-clock yet. The prior pre-CI run `ci#40403389629` on
  `e3468500` read `verdict: green · checks 15/15` (the handoff).

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
