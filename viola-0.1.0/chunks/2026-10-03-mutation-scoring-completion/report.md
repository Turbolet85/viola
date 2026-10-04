# Report — 2026-10-03-mutation-scoring-completion

**Chunk:** Mutation scoring completion: viola-e2e and the twelve cfg(unix) mutants scored natively at the boundary tier;
pre-push and gate tools moved to the Linux host; the harness diff-prefix pin.
**Date:** 2026-10-04
**Commits:** `80b69cd chore(2026-10-03-mutation-scoring-completion): operator pre-CI commit, for the run this chunk's verdict reads` (the only commit since `last_wrap` 2026-10-03T22:17:49Z; parent `aa300a4`)

## Changes (structured — detectors read this)
- **Files** (basis: `git diff --name-status aa300a4 HEAD` plus the one uncommitted evidence edit):
  - Rust:
    - `crates/viola-e2e/src/harness/run/mutants/base.rs`, `run/mutants.rs`, `run/mutants/scratch.rs` (tests only)
    - `crates/viola-e2e/src/harness/run.rs`, `bin/viola-harness.rs`
    - `crates/viola-e2e/src/harness/pre_push.rs`, `pre_push/linux.rs` (both rewritten)
    - `crates/viola-e2e/src/harness/boot.rs`, `cleanup.rs`, `logs.rs` (tests only)
    - `crates/viola-pty/src/lib.rs` (tests only)
    - `crates/viola-channel/tests/channel_frames.rs`
  - Scripts deleted: `scripts/wsl-exec.sh`, `scripts/wsl-provision.sh`.
  - Comment-only: `scripts/install-node.sh` (:4), `.github/workflows/ci.yml` (:10). No step, pin or permission changed.
  - Chunk folder:
    - `report.md`
    - `evidence/{c3,m3,cfg-unix,tui,operator-pass}.md`
- **Symbols / APIs** (all in the test-only `viola-e2e` harness; no product crate's public API changed):
  - **`run --mutants --package <member>`** (clap `--package`, `requires = "mutants"`; `Selection.package: Option<String>`):
    - It mutates one whole member: no base, no `chunk.diff`, no classification, no `--in-diff`.
    - Its argv is `cargo mutants --package <member> --features fake-agent [--file <p>]… --test-tool=nextest
      --copy-target=true --caught --unviable --build-timeout-multiplier=5`. It runs after the existing root prebuild
      (`cargo build --package viola --features fake-agent`, `CARGO_TARGET_DIR=<repo>/target/mutants`), with relative
      `CARGO_TARGET_DIR=target/mutants`, `NEXTEST_PROFILE=mutants`, `AGENT_RUN_KEEP_HOMES=0` and
      `AGENT_RUN_KEEP_FAILED=0`. A stale `outcomes.json` is deleted first, and the counts are read via `mutants_suite`.
    - Document: `"mutants":{"tested":N,"verdict":"package","package":"<member>"}`, plus `files` with `--file` and
      `scratch_bytes` on a Windows host. `archived` is unchanged.
    - **Input bound:** `package_member(root, member)`. The member must be named in lowercase ASCII, digits, `-` and
      `_`, and be `viola` (the root manifest) or `crates/<member>`, whose `Cargo.toml` `[features]` declares
      `fake-agent` (a line parse; viola-e2e carries no toml crate). Anything else is `{"v":1,"cmd":"run","ok":false,
      "reason":"package-refused"}`, exit 2, before any cargo runs (checked first in `run_with`).
    - The shared tail moved into a private `mutate()`. The diff arm's argv, env and verdicts (`counted` / `scoped` /
      `no-rust-delta` / `test-only-rust-delta`) are unchanged. `mutants()` gained a `package: Option<&str>` parameter;
      its sole caller is `run.rs` `tool_arms` (graph: 1 site).
  - **`chunk_diff`** (C3): both `git diff` argvs pin `--src-prefix=a/ --dst-prefix=b/`, the merge-base diff and the
    per-untracked-file `--no-index` diff. No signature change. `diff_paths` and `diff_files` are unchanged.
  - **`pre-push`, reshaped to native Linux, in place:**
    - `PRE_PUSH_HOST_SUPPORTED = cfg!(target_os = "linux")`. Off Linux the `host` stage refuses with
      `pre-push-linux-only`, exit 2.
    - The stages are `tools → linux-tests`, and the pass ends `ok:true` when `linux-tests` is green.
    - Every child runs through `/usr/bin/env -i HOME=<home> PATH=<home>/.cargo/bin:<home>/.local/viola-node/bin:
      /usr/local/bin:/usr/bin:/bin <argv>`, with `current_dir(ws.root)` (the working tree itself, no clone).
    - `<home>` is the passwd entry's field 6, read through `/usr/bin/env -i PATH=/usr/bin:/bin id -u`, then
      `… getent passwd <uid>`. A non-numeric uid, a failed probe or a non-absolute field is `tool-missing`
      `passwd-home` at stage `tools`. The harness's `$HOME` is never read.
    - `tools` keeps `cc`, `rust-toolchain.toml`'s channel, ci.yml's test-job `tool:` pins and `NODE_PIN_VERSION`.
    - **Closed values now emitted** (basis: `grep -noE 'Stop::new\("…' pre_push.rs pre_push/linux.rs` plus the three
      name-carrying sites):
      - `reason`: `pre-push-linux-only` (exit 2), `tool-missing` (detail `cc` · `node` · `passwd-home` · a tool name),
        `tool-pin-mismatch` (detail `node` · `pins-unreadable` · a tool name), `linux-document-unreadable` (detail the
        harness verb);
      - `stage`: `host` · `tools` · `linux-tests`.
      - **Retired with the stages:** `sync-failed` and its step details, `sync-mismatch`, `pre-push-windows-only`,
        `wsl-distro-ubuntu`, and the stages `sync` · `cache` · `vm-release` · `windows-tests`.
    - `linux-tests` keeps `run --coverage` → `run --browser` → `gate --require coverage,doctest,playwright`.
    - **Removed:** `pre_push_with` (`pre_push` is the one entry; signature unchanged), `wsl_path`, `distro_home`,
      `sync`, `cache`, `target_bytes`, `dir_bytes`, `release_vm`, `host_free_kib`, `windows_tests`, `CACHE_CAP_BYTES`,
      `WSL`, `DISTRO`, `CLONE_DIR`, `HOST_BUILD_JOBS`. `ci_pins` and `last_document` stay `pub`.
  - **`viola-harness` `boot_options`** (a new private fn): the `!unstamped` moved out of `main`. Boot's behaviour is
    unchanged.
  - Item 7: the `channel_frames` test tolerates `ErrorKind::NotConnected` beside `BrokenPipe | ConnectionReset` on the
    oversized frame's write, and every later assertion is unchanged.
- **Crates / modules:** none added or removed.
- **Dependencies:** none added or bumped (no `Cargo.toml` / `Cargo.lock` change; `git diff aa300a4 --stat -- '*Cargo*'`
  is empty).
- **Schema / config:** no config key, schema or env var added. The `pre-push` document's closed values changed (see
  Expected amendments). The `run` document gained `verdict:"package"`, field `package` and `reason:"package-refused"`.
- **Spec-master edits:** none this chunk (`git diff aa300a4 -- .andromeda/*.md .andromeda/registries` shows only the
  route and master files, which phase writes).
- **Counts / qualifiers moved:**
  - The viola-e2e mutant unit reads 711 (`cargo mutants --list` inside run 3; it was 727 at the Epoch 2b audit and 709
    at run 1).
  - Unit tests: 732 (gate entry 9 Summary). Coverage suite: 959 (gate entry 18).
  - No master states these counts (`grep -rn "727\|724 tests" .andromeda/*.md` → 0 hits).
- **Dev-tool versions:** none changed. Re-read on the dev host 2026-10-04: rustc 1.98.1, cargo-nextest 0.9.146,
  cargo-mutants 27.1.0, cargo-llvm-cov 0.9.1, node v24.21.0 (pinned, `~/.local/viola-node`), git 2.55.0, GCC 16.2.1
  (`cc`).
- **Harness / gate surface:**
  - The `run --mutants --package` arm and the native `pre-push` (as above).
  - `scripts/wsl-exec.sh` and `scripts/wsl-provision.sh` are deleted.
  - **Mutation runs on this host** take `TMPDIR=<repo parent>/viola-mutants-scratch`, a NOCOW (`chattr +C`) btrfs dir
    (overseer direction). This is an environment fact, not harness code: the harness's scratch arm stays Windows-only
    (`HOST_SCRATCH = cfg!(windows)`).
  - **CI:** no step change (ci.yml :10 is a comment).
- **Cross-project / external claims:**
  - **CI run ci#37166444247 on `80b69cd`:** green, 15/15 checks, 267 s. Job `test (macos-latest)` (id 111330205294)
    `success`, its log reading `PASS … channel_frame_one_byte_over_is_refused_and_closed` and 955/955.
  - **CI run ci#37157981452 on `aa300a42d15c`:** `failure`, job `test (macos-latest)`, with `write: Os { code: 57, kind:
    NotConnected }` at `channel_frames.rs:92:28`. This is item 7's origin, read at phase.
  - **cargo-mutants 27.1.0 `src/copy_tree.rs` `copy_file`** tries `reflink::reflink` first. `fs::copy` (which keeps
    modes) runs only after a reflink fails. Basis: the registry source at
    `~/.cargo/registry/src/index.crates.io-*/cargo-mutants-27.1.0`.
  - **`reflink 0.1.3` `src/sys/unix.rs`** creates the destination with `OpenOptions::create_new` and copies no mode.
    Measured: a raw `FICLONE` into a `create_new` file on btrfs leaves mode 0644 from a 0755 source, and
    `cp --reflink=always` into a `chattr +C` dir fails with EINVAL (`evidence/m3.md`).
  - **`kill(2)`:** PID 1 receives only signals for which it has installed a handler. The `cleanup` kill test rests on
    this.
- **Reverted / negative API facts:**
  - The first disposition of the two Windows-only `prepare` mutants, "equivalent mutant", was withdrawn on the
    overseer's correction. They are "not measured here; owed to `:70`".
  - `close` was neutralised once for the remove-the-guard reading and then restored byte-identical (insertions-only
    diff).
- **Insufficient fixes:** none.
- **Spec claims disproved by measurement:**
  1. **research.md §Measured:** "B's whole run incl. the copy took 64 s on btrfs". The copy did not go to btrfs.
     cargo-mutants copies into `std::env::temp_dir()`, which is `/tmp` (a 32 GB tmpfs with `usrquota`) on this host. A
     full viola-e2e run there died on `Disk quota exceeded` at 460/709. On btrfs, cargo-mutants' reflink copy drops the
     exec bit (above). Evidence: `evidence/m3.md` §Where the copy goes. The claim lives in a chunk research file
     (immutable), not in a master. The fact to carry: the boundary-tier form needs a NOCOW on-disk `TMPDIR` on this
     host.
  2. **plan.md Acceptance (M3):** "its confirming run's `outcomes.json` counts read `missed == 0`". Over the whole unit
     the confirming run reads `missed == 2`: the two `scratch.rs` `prepare` mutants behind the const
     `HOST_SCRATCH = cfg!(windows)`, which no Linux run can measure. Overseer ruling (founder-delegated): `missed == 0`
     reads over the MEASURABLE set (709: 649 caught, 0 missed, 0 timeout, 60 unviable), and the pair is owed to `:70`
     by coordinate. Evidence: `evidence/m3.md` §Run 4.
  3. **plan.md step 5, test letter:** "every runner call is `/usr/bin/env -i HOME=… PATH=…`". The two passwd probes
     run before HOME is known and carry `PATH=/usr/bin:/bin` only. That is narrower, not wider, and
     `pre_push_every_native_call_carries_only_home_and_path` asserts the narrower form.
  4. **The always-loaded `.claude/rules/security.md`** states that "the WSL2 `pre-push` distro installs only CI's own
     pins (`scripts/wsl-provision.sh` …)" and gives `wsl-exec.sh` an operator-only rule. Both scripts are now deleted,
     so those clauses describe retired tooling. That is a reconcile, not a falsified measurement.
- **Expected amendments (from plan):** each line gives the sites located by `grep -c` per file (basis in parentheses).
  - **test-plan §3 `pre-push` (stages `tools → linux-tests`; stage values `sync`, `cache`, `vm-release`,
    `windows-tests` retired; reason `pre-push-windows-only` → `pre-push-linux-only`; detail `wsl-distro-ubuntu`
    retired, `passwd-home` new; the document drops `sync`, `cache`, `vm`, `windows`): carried** (Symbols: pre-push).
    Sites: the body (`test-plan.md` `WSL` 2, `wsl-provision` 1) and the keyed contract
    `registries/contracts/test-plan/5-command-implementation.md` (`pre-push-windows-only` 3, `windows-tests` 2,
    `vm-release` 2, `wsl-distro-ubuntu` 1, `target/pre-push` 1). These are closed-enum changes and need a Decisions Log
    entry.
  - **test-plan §3 `run` (the `--mutants --package <member>` selector, `verdict:"package"`,
    `reason:"package-refused"`): carried** (Symbols: `run --mutants --package`). Site: the same keyed contract
    (`scoped` 10, `no-rust-delta` 3, `scratch-refused` 2: the run-verdict enumerations). Closed values, Decisions Log.
  - **test-plan §10 Mutation gate (the boundary tier's viola-e2e form is `run --mutants --package viola-e2e`): carried**
    (Symbols; plus disproved claim 1's NOCOW `TMPDIR` fact). Site: `test-plan.md` §10 (located by the P2 detector; the
    body holds the Mutation gate section).
  - **architecture CI/CD approach (the pre-push bullet becomes native Linux in place, `env -i` HOME (passwd) + PATH
    constants): carried** (Symbols: pre-push). Sites: `architecture.md` (`WSL` 3, `wsl-provision` 3) and
    `registries/contracts/architecture/ci-cd-approach.md` (`pre-push-windows-only` 1, `windows-tests` 1, `vm-release` 1,
    `WSL` 3).
  - **architecture Occupied Resources (retire the WSL test-side install sites, `target/pre-push/`, the `scripts/` rows
    for the two WSL scripts, the CI workflow data-line mention): carried** (Files: deleted scripts; ci.yml :10).
    Sites: `architecture.md` (`target/pre-push` 1, `wsl-provision` 3).
  - **architecture Project directory structure (drop the two script rows): carried** (Files). Site:
    `registries/contracts/architecture/project-directory-structure.md` (`wsl-provision` 1, `wsl-exec` 1).
  - **security-plan Secret Management, Development (the native `env -i` crossing; `wsl-exec.sh` and the root
    `--install-deps` rule retired with CARRY 4; founder told at P4, no widening chosen): carried** (Symbols:
    pre-push). Sites: `security-plan.md` (`WSL` 3, `wsl-provision` 2, `wsl-exec` 1).
  - **security-plan Dependency Security, Pinning (the gate host is the native Linux host): carried.** Site:
    `security-plan.md` (as above).
  - **security-plan Security Anti-Patterns → Code Patterns (the scratch bullet worded Windows-host-only, inert
    elsewhere): carried** (Harness: the scratch arm stays `cfg!(windows)`). Site: `security-plan.md` (`scratch-refused`
    1).
  - **obs-plan §8 item 6 (the pre-push document's no-absolute-path wording without the WSL clone; the scratch "Windows
    host" wording): carried** (Symbols: pre-push; the document carries no path, per
    `pre_push_document_carries_no_absolute_path`). Site: `obs-plan.md` (`pre-push|viola-mutants-scratch|clone` 2).
  - **The docs `commands.md`, `gotchas.md`, `security-summary.md`, `workflow.md` and the rules `testing.md`,
    `verification-harness.md`, `security.md` reconcile at wrap: carried.** Basis: `grep -c
    "WSL|wsl|windows-tests|vm-release|pre-push-windows-only"` → `gotchas.md` 3, `commands.md` 5, `security.md` 1,
    `security-summary.md` 1, `workflow.md` 1, `stack.md` 1, `verification-harness.md` 1, `testing.md` 1. `stack.md` was
    named at P1 but is not on the plan's list. `host-win32.md` is setup's (U04, directive 5): it stays, in the handoff.
  - **Route note (`working-route.md:133`'s `windows-tests` `run --browser` mirror item is moot): carried** to P5
    route-resolve.
  - **Matrix: no capability claimed; v1-07 stays pooled: not carried** (`matrix.py show --chunk` → claimed 0).
- **Coverage of new surfaces:**
  - `run --mutants --package <member>` (harness CLI input) → validation `package_member`✓ (name class + manifest feature,
    before any cargo) · instrumentation n/a (test-only harness; no `obs_event!`) · PII n/a · tests unit (5 named, runner
    seam) · a11y n/a · tokens n/a.
  - The native `pre-push` launcher (process crossing) → validation: the passwd field 6 must be absolute and the uid
    numeric✓ · instrumentation n/a · PII: the document carries codes and counts only; neither the home nor the repo
    path is printed✓ · tests: unit (Fake runner) plus one real launch (the canary pair: 0 `CLAUDE*` names through
    `env -i`, 1 through the control without `-i`) · a11y n/a · tokens n/a.

## Deviations from intent
- **Mutation-run scratch (overseer direction, founder-delegated).**
  - **What changed:** every mutation run used `TMPDIR=<repo parent>/viola-mutants-scratch`. Run 1 (on `/tmp`) was
    discarded whole: 25 of its 77 unviable came from the quota. Partial outcomes are never merged.
  - **NOCOW:** the scratch is marked `chattr +C` so cargo-mutants' reflink fails and `fs::copy` keeps the exec bit.
    Run 2's baseline broke without it (32 EACCES on the copied `viola-fake-agent`).
  - **Code:** no change. **Plan's word:** "the host scratch arm unchanged" holds.
- **The `!unstamped` moved out of `main` into a new private `boot_options`.** The survivor at `main` (`delete !`) was
  unkillable there. Boot is unchanged, and the mutant at its new site (`viola-harness.rs:151:16`) is caught.
- **The passwd probes carry PATH only** (disproved claim 3). This is narrower than the plan's letter.
- **M3 acceptance over the measurable set** (disproved claim 2; overseer ruling).
- **Kill tests beyond the provisional sites.** Run 3's four `wipe` mutants (`scratch.rs`) and viola-pty's
  `pty_backend` ×2 and `close` were killed in their own crates. All sit inside research's listed directories:
  `crates/viola-e2e/src/harness/` and `crates/viola-pty/src/lib.rs`.
- **The C3 test collects every failing case before asserting, rather than stopping at the first.** One red run then
  names each failing case; under the host's global config the `default` case is hostile too.
- **Operator pass, first push refused.** The OAuth token lacked the `workflow` scope (the commit touches `ci.yml`).
  The overseer granted it, and entry 23 was re-fired unchanged. No other remote or credential was tried.
- **Two operator cleanups were refused by the permission layer and left to the operator:**
  - the 21 leaked `/tmp/cargo-mutants-ws-*.tmp` dirs (325 MB);
  - the scratch's 8 GB of leaked test dirs.
- **Scope record:** none. `gate.py scope` reads clean at P1 (changed 16 · listed 16 · recorded 0 · absorbed 4 ·
  excluded 41).

## Decisions & corrections
- **Overseer correction:** a mutant that this host cannot compile or reach (`cfg(windows)`, or behind a const
  `cfg!(windows)`) is **"not measured here; owed to `{route entry}`"**, by coordinate, never "equivalent". An acceptance
  `missed == 0` then reads over the measurable set, and the wrap notes the premise.
- **Overseer direction:** every mutation run on this host takes `TMPDIR` on btrfs, the died unit is re-run in full, and
  partial outcomes are never merged. The check is that no unviable came from the cap.
- **Measured fact (Linux host):** cargo-mutants 27.1.0's reflink copy (via `reflink 0.1.3`) drops the exec bit on btrfs,
  so the mutation scratch must be NOCOW (`chattr +C`) to force the mode-keeping `fs::copy`. On tmpfs no reflink happens,
  but `/tmp` here is a 32 GB `usrquota` tmpfs, too small for a 14 GB `target/` copy plus build growth.
- **Measured fact:** the mutants nextest profile (`fail-fast … terminate = "immediate"`) kills every running test at
  once, so `TempDir` / `Booted` drop guards never run. One viola-e2e boundary run leaves about 25k dirs in `TMPDIR`,
  plus nested `cargo-mutants-ws-*` copies from the two real-cargo-mutants self-tests. Pre-existing; owner per
  directive 3, at P5.
- **Measured fact:** `kill(2)`: PID 1 receives only the signals it has a handler for. It is a safe target that outlives
  any SIGKILL in a cleanup-deadline test (Linux).
- **Measured fact:** on Linux, closing a PTY master hangs up a live child (SIGHUP) only when no reader or writer clone
  still holds the master fd.
- **Operator note:** the CLI auto-submitted a stale `/andromeda-phase` P3 prompt twice, 230 ms after a Stop hook with a
  background task running and no `viola:typed` before it (overseer measured from viola events). Both were declined.
- **Founder rulings carried by this wrap (directives 1 and 2):**
  - verbatim upstream copies are KEPT CURRENT, superseding playbook `:40` and `:44`;
  - the 2026-09-28 mutation rule goes into `testing.md`.
- **Sweep hazard:** `git diff` headers on this host read `c/ w/` (or `i/ w/`) under the global
  `diff.mnemonicprefix=true`. Any tool parsing `diff --git a/` must pin `--src-prefix=a/ --dst-prefix=b/`.

## Outcome
**Acceptance criteria, re-asserted against the diff:**
- **(tests) C3: met.** The `package(viola-e2e) & test(/mutants/)` selection reads `ok:true` under the host config (51/0;
  29/13 at P3). `chunk_diff_pins_its_prefixes_under_a_hostile_config` passes all three cases, and its remove-the-guard
  red is recorded (`evidence/c3.md`).
- **(tests) M3: met over the measurable set** (disproved claim 2). The witness passed its baseline (runs 3 and 4; no
  `mutants-exit-4`). The confirming run reads 711 tested, 649 caught, 2 missed, 0 timeout, 60 unviable; over the 709
  measurable, missed 0, timeout 0, unviable 60 ≤ caught 649. Every first-run survivor is killed or owed to `:70` by
  coordinate (`evidence/m3.md`).
- **(arch) `target/mutants` contract: met.** The unit test asserts the root prebuild into `<repo>/target/mutants`, the
  relative `CARGO_TARGET_DIR=target/mutants` with `--copy-target=true`, and no `--in-diff`. The witness baseline is
  green (runs 3 and 4).
- **(tests) cfg(unix): met.** The 12 coordinates grade 9 caught, 3 unviable (confirmed in each build log), none missed.
  The 13th is not measurable and never counted as caught (`evidence/cfg-unix.md`).
- **(obs) `panic_frames`: met.** Every Unix mutant is caught, and no obs-code mutant is missed or timed out in the
  measurable set.
- **(a11y) TUI set: met.** 12/0 on the host, openpty only, ConPTY not credited, and the clause-3 gap is recorded with
  its owner (`evidence/tui.md`).
- **(security) Native crossing: met.** Every launcher call is `env -i HOME=<passwd home> PATH=<constants>`, and the two
  passwd probes are PATH-only (narrower). Canary 0 vs 1. HOME comes from passwd. No PATH entry beyond the WSL shape;
  no venv.
- **(security) Pins, no uid-0 launch, scripts gone, CARRY 4: met.** `tool-missing` / `tool-pin-mismatch` stay. No uid-0
  launch anywhere. Both scripts are deleted. **CARRY 4 is retired with `wsl-provision.sh`**, not dropped.
- **(tests) Native `pre-push`: met.** It reads `ok:true` at `"stage":"linux-tests"` (coverage 959/0, Playwright 1/0,
  gate no breaches) and refuses `pre-push-linux-only` off Linux (unit). There is no mutation or perf stage.
- **(obs) No absolute path in the documents: met.** The `pre-push` document carries none, and the `--package` document
  names only `archived` (plus `scratch_bytes` on Windows).
- **(tests) Item 7: met.** The arm tolerates `NotConnected` and still asserts the reply and the end of stream.
  `test (macos-latest)` is green on the final pushed sha `80b69cd` (run ci#37166444247, job 111330205294).
- **(tests) No ignore / retries / skip / coverage-ignore widening, CI gains no mutation job, release-check refusals
  hold: met** (gate entries 13–16).

**Gates** (implement's final pass, `implement-2026-10-03T22-52-19`, on the tree the pre-CI commit froze):

| entry | verdict |
|---|---|
| `cargo fmt --all --check` | green |
| `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` | green |
| `run --unit --filter 'package(viola-e2e) & test(/mutants/)'` | green, 51/0 |
| `run --unit --filter 'test(/chunk_diff_pins_its_prefixes_under_a_hostile_config/)'` | green |
| `run --unit --filter 'test(/run_mutants_package_\|run_package_needs_mutants/)'` | green, 5/0 |
| `run --unit --filter 'package(viola-e2e) & test(/pre_push/)'` | green, 20/0 |
| `run --integration --filter 'binary(channel_frames)'` | green, 4/0 |
| `run --integration --filter 'binary(/^tui_/)'` | green, 12/0 |
| `run --unit` | green, 732/0 |
| `run` | green, 732 + 227 / 0 |
| `test ! -e scripts/wsl-exec.sh && …` | green |
| the WSL-token grep | green, no output |
| the `#[ignore]` / retries diff grep | green, no output |
| `git diff --quiet aa300a4 -- …/coverage.rs` | green |
| the ci.yml mutation grep | green, no output |
| `release-check.sh --probe` | green, 5/5 refused |
| `cargo deny check` | green |
| `pre-push` | green, `ok:true` `stage:linux-tests` |
| smoke `cleanup` → `boot` → `cleanup` | green; processes and endpoint gone |
| leg operator: `gate.py hygiene` | clean (evidence) |
| leg operator: push | `aa300a4..80b69cd` after the scope grant (evidence) |
| leg operator: `ci.py conclusion` | green 15/15, run ci#37166444247 (evidence) |

No `defer` entry. Smoke: boot (builder) → status `ready` → cleanup, driven by hand at P3.

**Watches:** none folded.

**Outcome basis:** the operator pass ran. Commits from `aa300a4`: `80b69cd` alone. The final HEAD's CI run
ci#37166444247 is recorded in `evidence/operator-pass.md`. The implement conversation is present in this session, and
its P4 report is the basis for everything else.

**Process hygiene:** implement P4's census; re-measured now: `pgrep` for viola / harness / cargo-mutants / nextest
finds none running. Left on disk for the operator:
- 21 `/tmp/cargo-mutants-ws-*.tmp` (325 MB; `rm` refused by the permission layer);
- `<repo parent>/viola-mutants-scratch`: about 8 GB, 25,275 `.tmp*` + 62 `cargo-mutants-ws-*` + `rustdoctest*`;
- two reflink probe files in `target/` (gitignored).
