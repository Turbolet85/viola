# Scope — Mutation scoring completion

**Marker:** `2026-10-03-mutation-scoring-completion` · **Version:** viola-0.1.0 · **Epoch:** Epoch 3 — Windows slice II: driving verbs and live proof
**Working entry:** `viola-0.1.0/working-route.md:68` — "Mutation scoring completion — viola-e2e harness and twelve cfg(unix) mutants scored natively at the boundary tier; pre-push and gate tools moved to the Linux host", with six CARRY blocks (all six folded below; `route.py pins` indexed 6 freight blocks on :68, 927 · 1022 · 500 · 1165 · 1009 · 590 chars).
**Host:** Linux (Omarchy, btrfs) since 2026-10-03; Windows is witnessed only by the `windows-2025` CI runner.

## What this chunk builds

Closes the mutation-scoring gaps the Epoch 2b boundary code audit
(`.andromeda/runs/2026-10-01T09-18-50-code-audit/proposals.md`) left, now that the dev host is Linux, and moves
the local gate tooling (pre-push and its stages) off the retired Windows + WSL arrangement onto the Linux host.

### 1. M3 — viola-e2e enters the boundary mutation tier (CARRY 1)
- viola-e2e (727 mutants at the Epoch 2b audit) is unscored at both boundaries: 32 of 246 harness tests fail in
  cargo-mutants' copied tree because they spawn the root package's `viola-fake-agent` bin, which a
  `--package=viola-e2e` build never produces (audit M3 + §Skips; measured on the Windows host, `.exe` names).
- Give the boundary tier a viola-e2e form that has the root bins in each copied tree: the project's own
  `run --mutants` prebuild (`viola --features fake-agent`) + `--copy-target=true` form
  (`crates/viola-e2e/src/harness/run/mutants.rs:155`, prebuild at :657–:689), **its copy size measured first**, or a
  viola-e2e test seam that builds what it spawns. Then score the unit and dispose its survivors the way M1's were
  (killed by a new/strengthened test, or recorded equivalent with its argument).
- The 32-of-246 baseline failure reproduces on the Linux host in the same shape (verified at P3, git config
  isolated: the audit form reads 216/245 run, 32 failed, each `ENOENT` on the copied tree's
  `target/debug/viola-fake-agent`; the prebuild + `--copy-target=true` form reads 245/245 and 22/22 caught — research.md
  §Measured). With the host git config in place both forms read red on the C3 tests first, so C3 lands before any
  viola-e2e scoring on this host.
- Overseer direction (founder-delegated, 2026-10-01): if this chunk sizes over one window, raise a split of this
  item and the cfg(unix) leg at P1. **P1 disposition: no split raised** — leaned on the overseer's own C2 sizing
  (adaptation record `.andromeda/runs/2026-10-03T22-10-25-wrap/adaptation-record.md` :25: the Windows boundary
  mutation workflow became its own entry, `:70`, to keep `:68` within one window).

### 2. The cfg(unix) mutants scored natively (CARRY 2)
- 13 mutants were never compiled on the former Windows host: 12 `cfg(unix)` — viola-pty `HostTerminal::enter` ×4
  and `host_size`; viola-channel `open_by`, `host_socket_dir`, `listen`, `Guard::drop`; `src/panic_frames.rs`
  `raw_frames` and `module_of` ×2 — and 1 `cfg(all(windows, not(target_arch = "x86_64")))` (`src/cmd/run.rs`
  `sideload_outcome`).
- The 12 compile natively on Linux: the boundary tier measures them in-repo and their survivors are disposed.
  The WSL leg and its `TMPDIR` carve-out question are retired.
- The 13th is recorded **not measurable**, with the reason (no Windows non-x86_64 host or runner; `windows-2025` is
  x64), in the form M1's equivalent-mutant exemptions take.
- Out of scope: the cfg(windows) x86_64 mutants — the next entry, `:70` "Windows boundary mutation workflow"
  (founder ruling C2).
- The 12-mutant set still matches HEAD (verified at P3: every audit coordinate lists unchanged under
  `cargo mutants --list --workspace --line-col=true`; the unix arms carry further mutants beyond the 12, which a native
  run grades too).

### 3. The in-repo TUI boundary cases on the host (CARRY 3, M2's rider)
- M2 is closed (the D: volume is gone; witness set 63/0 on Linux; CI green on `9e3b850` and `d60f3d6`). Its rider
  stays: the in-repo TUI boundary cases. The Linux host's 5/0 (the overseer's relay) covers openpty only; the
  ConPTY cases stay witnessed on the `windows-2025` CI runner. This chunk measures the in-repo set on the host.
- [premise-corrected: the in-repo TUI set re-measured on the host reads 12/0 across `tui_pty_seam` 3,
  `tui_env_strip` 3, `tui_passthrough` 5, `tui_channel_fds` 1 — openpty only, as relayed; the relay's "5/0" was a
  narrower selection] None of the 12 asserts focus / mouse / wheel (a11y-plan §4 P4 clause 3 has no case at HEAD — owned
  by the wheel entries, recorded, not built here).

### 4. Pre-push and the gate tools moved to the Linux host (CARRY 5, relay D and B)
- Pre-push and the gate tools still carry the Windows host's stages: the host `windows-tests` stage and the WSL
  `linux` stage (`crates/viola-e2e/src/harness/pre_push.rs` — stage order `tools → sync → cache → linux-tests →
  vm-release → windows-tests`, per its module doc; `crates/viola-e2e/src/harness/pre_push/linux.rs`),
  `scripts/wsl-exec.sh`, `scripts/wsl-provision.sh`.
- The Linux stage runs **natively** on the host; Windows is witnessed only by the `windows-2025` CI runner.
- The native stage keeps the `env -i` HOME+PATH boundary. **Anything wider is a widening that goes to the founder
  live** (the C3 red below is a host value reaching a gate).
- [premise-corrected: under `env -i HOME PATH` with exactly the WSL PATH shape (`~/.cargo/bin`, `~/.local/viola-node/bin`,
  system dirs) every ci.yml pin and Playwright's Chromium resolve, and the pre-push leg spawns no python — the venv is
  needed for the code-graph refresh only, not on the gate's PATH (research.md §Measured)] Operator note at this
  phase's invocation (2026-10-04): the host's gate tools sit off the default PATH —
  `scip-typescript` on the pinned Node at `~/.local/viola-node/bin`, python `duckdb`/`protobuf` in the venv at
  `~/.local/viola-venv/bin` (measured at Setup: both resolve with the two dirs prepended; `rust-analyzer` is already on
  PATH). Prepend both for the code-graph refresh and the gates. The former WSL `env -i` PATH already admitted the
  distro-derived constant `<home>/.local/viola-node/bin` (security rules); whether the native stage's PATH gaining
  `~/.local/viola-venv/bin` (or any other entry) is a boundary widening is **shown to the founder at P4**.
- Re-judged with it: the `Web test toolchain and a11y harness` entry's (`working-route.md:133`, cited `:131` in the
  adaptation record before the `:70` insert) `windows-tests` `run --browser` mirror item. Phase never edits a route
  line: the re-judgement is recorded for wrap's route-resolve.
- The spec bodies naming the WSL distro or the pre-push stages (architecture, security-plan, test-plan, obs-plan,
  `.claude/docs/commands.md`, rules `testing.md` and `verification-harness.md`) reconcile at **this chunk's wrap**
  (amendments), not in phase. Also naming WSL (grep at P1): `.claude/docs/security-summary.md`, `stack.md`,
  `gotchas.md`, `.claude/rules/security.md`.

### 5. The WSL-provision root install (CARRY 4)
- Before the WSL distro is next re-provisioned, `scripts/wsl-provision.sh --install-deps` must stop running
  user-writable code as root (apt-get only, over an allowlisted dry-run list; overseer live ratification, chunk
  2026-09-27-browser-verdict-reachability).
- The Linux-host gate adaptation (item 4) **retires** it with the WSL tooling; **any WSL path kept keeps it**. The
  chunk's disposition of `wsl-provision.sh` decides which.

### 6. C3 — the harness `git diff` prefix pin (CARRY 6)
- `run --mutants`' unit tests read 13 red on the host: 25/13 with `diff.mnemonicprefix=true`, 38/0 with the config
  isolated or the setting off (the relay's two-sided witness — a hypothesis until re-measured).
- Mechanism (re-verified at P1): the host git config sets `diff.mnemonicprefix=true`
  (`~/.config/git/config`), so `git diff` headers read `diff --git i/… w/…` (seen at this session's start), while
  `crates/viola-e2e/src/harness/run/mutants/base.rs` `diff_paths` (:152) matches only `diff --git a/`; its `git diff`
  calls (:126, :134) pin no prefixes.
- Fix: `--src-prefix=a/ --dst-prefix=b/` on those calls. Acceptance: a test that holds the git config hostile, plus
  the 38/0 witness.
- [premise-corrected: only `chunk_diff`'s two calls feed a header parser (`diff_paths`, and cargo-mutants' own
  `--in-diff`); `pre_push/linux.rs` `sync`'s `diff --cached --binary` feeds `git apply`, which strips one component
  whatever the prefix, and retires with the WSL sync] The mechanism is measured two-sided on the host: 29/13 under the
  host config, 42/0 with `GIT_CONFIG_GLOBAL=/dev/null` (a 42-test `test(/mutants/)` selection; the relay's 25/13 vs
  38/0 was another selection — the 13 match).

### 7. The macOS `channel_frames` red (CI fold, on the operator's word — not on the entry)
- Run **ci#37157981452** on sha **`aa300a42d15ccabc9f5621ef7d64d917a45ad20d`** concluded `failure`; its one
  failed job is **`test (macos-latest)`** (22:18:33Z → 22:21:15Z, 162 s; read at P2 via `gh run view`): viola-channel
  `channel_frames` `channel_frame_one_byte_over_is_refused_and_closed` panicked at
  `crates/viola-channel/tests/channel_frames.rs:92` with `write: Os { code: 57, kind: NotConnected }` (the panic text
  is the overseer's relay; the job name, sha, run id and the line's `written.expect("write")` arm re-verified at P2).
  VERIFIED at P3 against THAT RUN (runner-only subject): `gh run view 37157981452 --log-failed` reads
  `panicked at crates/viola-channel/tests/channel_frames.rs:92:28: write: Os { code: 57, kind: NotConnected, message:
  "Socket is not connected" }`, 948/949 passed; on the Linux host it does not reproduce (3 × 4/4, its expected state).
- [inferred — half verified at P3: the arm's shape is read at HEAD, `channel_frames.rs:90–:93` tolerates exactly
  `BrokenPipe | ConnectionReset`; the macOS kernel cause stays a hypothesis no host here can measure] Mechanism
  (overseer, hypothesis — unmeasured): the test tolerates `BrokenPipe | ConnectionReset` on the
  oversized frame's write but not `NotConnected`, which macOS can also return when the server closes first. `aa300a4`
  is records-only (`chore(route)`), so the red is a pre-existing flake, not a regression of that commit.
- Folded on the operator's word (overseer, founder rule: fold reds into the next chunk), 2026-10-04, by the HALT's
  third arm (promotion.md): its subject (viola-channel's frame bound) does not intersect this chunk's mutation /
  gate-tool work. It is a runner-only subject — P3 closes it against THAT RUN, never against HEAD; non-reproduction
  on the Linux host is its expected state.
- Own acceptance (non-vacuous): the test admits exactly the close-race write kinds the platforms report and still
  asserts the `-32600 invalid request` reply and the closed stream after them — a tolerated write error never
  skips the reply assertion, so widening the arm cannot pass vacuously; witnessed green on `test (macos-latest)` on
  this chunk's pushed sha, with the panic site's `NotConnected` arm named in the evidence. If P3 finds the reply
  itself unreadable after `NotConnected` on macOS, the acceptance is re-shaped at P4, never weakened to the write.

## CI read at Setup (Setup 5a; every commit from the last flip `d60f3d6` through HEAD)
- `aa300a42d15c` — read `in progress` at Setup (run ci#37157981452). **Settled since: `failure`** — job
  `test (macos-latest)` red (162 s to its failure; read at P2). Dispositioned as item 7 above, on the operator's word.
- `d60f3d6c1ba5` — green · checks 15/15 · wall 267 s · run ci#37120646288.

## Boundaries
- In: the viola-e2e boundary form + its scoring; the 12 cfg(unix) mutants + the 13th's not-measurable record;
  the in-repo TUI boundary set on the host; pre-push / gate-tool migration to native Linux (WSL stage and Windows
  host stage retired or re-shaped); C3's prefix pin + hostile-config test; the WSL-provision root CARRY's
  disposition; the macOS `channel_frames` close-race red (item 7, folded on the operator's word).
- Out: cfg(windows) x86_64 mutants (`:70`); spec-body edits (wrap reconciles them); any `env -i` boundary widening
  without the founder's live word; the installed `claude` 2.1.287 verification (no fixture set; not this chunk's).
