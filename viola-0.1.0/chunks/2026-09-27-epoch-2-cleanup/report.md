# Report — 2026-09-27-epoch-2-cleanup

**Chunk:** Epoch 2 cleanup — four splits under 800, clone pairs gone, Windows survivors killed, mutation scratch on D:, harness loop aids, machete annotation
**Date:** 2026-09-27T17:25Z
**Commits:** `f0e6dbc` chore(2026-09-27-epoch-2-cleanup): operator pre-CI commit, for the run this chunk's verdict reads (the only commit since `last_wrap` 2026-09-27T14:03:12Z; parent `a0e6506`)

## Changes (structured — detectors read this)
- **Files:** (basis `git diff --name-only a0e6506` = 68 paths, plus the two post-commit evidence edits)
  - viola-pty: `crates/viola-pty/src/lib.rs` (modified), `crates/viola-pty/src/pump.rs` (new).
  - viola-channel: `crates/viola-channel/Cargo.toml`, `src/lib.rs`, `src/server.rs` (modified); `src/server/win.rs`,
    `src/test_support.rs` (new).
  - viola-e2e: `crates/viola-e2e/Cargo.toml`, `src/bin/viola-harness.rs`, `src/harness/pre_push.rs`, `src/harness/run.rs`,
    `src/harness/run/mutants.rs`, `tests/harness_lifecycle.rs` (modified); `src/harness/pre_push/linux.rs`,
    `src/harness/run/mutants/{base,leg,scratch}.rs` (new).
  - Root: `Cargo.toml`, `src/cmd/run.rs` (tests only), `tests/channel_endpoint.rs` (modified).
  - Scripts: `scripts/release-check.sh` (modified); `scripts/wsl-exec.sh` (new, index mode `100755`).
  - Chunk folder: `viola-0.1.0/chunks/2026-09-27-epoch-2-cleanup/` (scope, research, plan — plan entry 22 edited on the operator's
    direction — metrics.py, evidence/ ×9); run dirs `.andromeda/runs/2026-09-27T14-05-04-phase/`, `…14-42-16-implement/`;
    `.andromeda/friction-log.ndjson`; the phase's master record and working-route stamp.
- **Symbols / APIs:**
  - **viola-pty:** `pump` and `PumpEnd` moved to `pump.rs`, re-exported `pub use pump::{PumpEnd, pump}`, so every caller keeps
    `viola_pty::pump` / `viola_pty::PumpEnd`. The pump's `Worker`, `spawn_worker`, `copy`, `drain`, `kill_and_reap` and the consts
    `TICK`, `RESIZE_EVERY`, `DRAIN_WITHIN`, `KILL_WAIT` moved with it (crate-private). Public surface otherwise unchanged; `PtyError`
    `Display` unchanged.
  - **viola-pty tests only:**
    - `CHILD_MODE` value `hold` (`CHILD_HOLD` 750 ms), and the new test
      `tests::spawn_delivers_a_key_written_right_after_a_resize_to_a_child_not_yet_reading`.
    - `CHILD_WITHIN` 10 s → **7 s**.
    - Child reports now stream to `<temp dir>/viola-pty-watch/<test name>.report`, kept on panic or kill and removed on pass.
    - `impl Drop for Child` stops a still-running PTY child.
  - **viola-channel:** new `pub mod test_support`, compiled only `#[cfg(any(test, feature = "test-support"))]`:
    - `JsonFields` (a `tracing::field::Visit` into a `serde_json` map);
    - on Windows, `canonical_sddl`, `dacl_of(endpoint)` and `user_sid()` (independent of `server::win::user_sid`).
    - `server::win` moved to `server/win.rs` (crate-internal; `owner_only_sddl`, `user_sid`, `owner_only_descriptor` unchanged).
      Its tests moved with it: `owner_only_sddl_is_protected_user_and_system`, `bound_pipe_dacl_reads_back_protected_user_and_system`
      (now on the shared helpers and the independent SID), `pipe_handles_on_both_ends_are_not_inheritable`.
    - The listener controls stay at their call site: exactly 1 `security_descriptor(` in `crates/viola-channel/src` (gate entry,
      last line `1`); 0 `try_overwrite` / `ListenerOptionsExt::mode` uses (gate entry, exit 1 no output).
  - **viola-e2e harness (`harness::run`):** `Selection` gains `pub files: Vec<String>` and loses `Copy` (keeps `Clone`).
    `run_with` refuses `files` together with a leg: `Outcome::usage(Some("run"), "scoped-leg")`.
  - **viola-e2e harness (`harness::run::mutants`):**
    - The `mutants()` arm returns the `outcomes.json` path it read, for the archive.
    - New submodules: `base` (base resolution + `chunk_diff`, plus the diff classifiers `diff_paths`, `diff_files`,
      `rust_delta`, `rust_paths`, `test_target`), `leg` (`leg_verdict`, `leg_verdict_path`, `write_leg_verdict`) and `scratch`
      (`HOST_SCRATCH` const `= cfg!(windows)`, `scratch_allowed`, `prepare`, `host_scratch_bytes`). Every former
      `harness::run::` export is kept by `pub use`.
  - **viola-e2e harness (`harness::pre_push`):** the distro side moved to the `linux` submodule (`Linux`, `distro_home`, `tools`,
    `sync`, `dir_bytes`, `target_bytes`, `cache`, `linux_harness`, `summary`, `leg`, `linux_tests`, `linux_leg`, `wsl_path`,
    `ci_pins`, `last_document`). `pub use linux::{ci_pins, last_document, wsl_path}` keeps the external paths; the only external
    user, `viola-harness`, uses `pre_push::{PRE_PUSH_HOST_SUPPORTED, pre_push}`, unchanged.
  - **viola-harness CLI:** `run --mutants --file <path>` (repeatable, clap `requires = "mutants"`).
  - **Env vars:** no new name. `AGENT_RUN_KEEP_FAILED` gains a reader: `crates/viola-e2e/tests/harness_lifecycle.rs`'s `Booted`
    guard (it already had `tests/support/home.rs` `keep_decision` and the `run --mutants` override to `0`, which is unchanged).
    `TMP`/`TEMP` are SET on the `cargo mutants` child on a Windows host (not read by viola).
  - **Paths/resources:**
    - `<repo parent>/viola-mutants-scratch/` (host mutation scratch: cargo-mutants' temp copies and `--output`; wiped before every
      counted/scoped run; never printed; on this host `D:\dev\projects\viola-mutants-scratch`, 9 818 023 B after the last run).
    - `target/run-archive/<n>/` (per-run archive of this run's JUnit files + the `outcomes.json` read; newest 10 kept;
      gitignored, `git check-ignore -q target/run-archive/1` exit 0).
    - `<temp dir>/viola-pty-watch/` (test-only report files).
    - `mutants.out/` at the repo root is no longer written by any run on Windows (gate `test ! -e mutants.out` green after the
      scoped run and the pre-push).
- **Crates / modules:**
  - Modules added: `viola_pty::pump` (private mod, re-exported items), `viola_channel::server::win` (was inline),
    `viola_channel::test_support` (feature-gated), `viola_e2e::harness::pre_push::linux`,
    `viola_e2e::harness::run::mutants::{base, leg, scratch}`.
  - No workspace member added or removed; no product crate depends on `viola-e2e`.
  - viola-pty normal deps unchanged (portable-pty, tracing, windows-sys, libc).
- **Dependencies:** none added or bumped (basis: `git diff a0e6506 -- Cargo.lock` empty). The root `[dev-dependencies]` gains
  `viola-channel = { path = "crates/viola-channel", features = ["test-support"] }` (an existing normal dependency, now also a
  dev-dependency, to enable the feature for tests). `cargo deny check` green; the sole-root tokio ban over viola-channel green;
  `cargo check` of the sync crates green.
- **Schema / config:**
  - Cargo feature `viola-channel/test-support = []`, enabled only by the root dev-dependency (`grep -rn test-support --include
    Cargo.toml` = 2 hits: its declaration + the root dev-dependency).
  - `crates/viola-e2e/Cargo.toml` `[package.metadata.cargo-machete] ignored = ["proc-macro2"]`.
  - `scripts/release-check.sh` `judge` refuses a `compiler-artifact` record whose `features` hold `test-support` or `fake-agent`,
    with the line `release-check: FAILED — test-only feature {feature} in {target}`.
- **Spec-master edits:** none. The plan was edited (entry 22, operator-directed); no master.
- **Counts / qualifiers moved:**
  - `release-check --probe`'s last line `3/3 refused, control clean` → **`5/5 refused, control clean`**. Basis: `grep -rn '3/3
    refused'` over `.andromeda/*.md`, `CLAUDE.md`, `.claude/` = 3 hits — `test-plan.md:1450` (Release-build row),
    `test-plan.md:1484` (release-build errors) and `.claude/docs/commands.md:73`. All three are stale.
  - viola-pty `CHILD_WITHIN` 10 s → 7 s: no master states it (the `sites.py` token sweep: 0 hits in all seven).
  - The four split files read (tokei `code`, `metrics.py size` at `f0e6dbc`):
    - `pre_push.rs` 654 (was 1335), `viola-pty/src/lib.rs` 590 (1074), `run/mutants.rs` 697 (1029), `server.rs` 668 (869);
    - the new files `pump.rs` 527, `server/win.rs` 118, `test_support.rs` 179, `pre_push/linux.rs` 688, `mutants/base.rs` 394,
      `mutants/leg.rs` 70, `mutants/scratch.rs` 244;
    - 75 files, 0 over 800.
    No master states these sizes (0 hits for 1335/1074/1029/869 in the seven masters).
- **Dev-tool versions:** none — re-read unchanged on the dev host: cargo-machete 0.9.2, ripgrep 15.2.0
  (`target/tools/ripgrep/bin/rg.exe`, rev e89fff89ac), cargo-modules 0.27.0, cargo-mutants 27.1.0, tokei 14.0.0, jscpd 5.0.16.
- **Harness / gate surface:**
  - **`run --mutants` on a Windows host (`HOST_SCRATCH`):**
    - It resolves `<repo parent>/viola-mutants-scratch` from the repo's absolute path and refuses unless the final component is
      exactly `viola-mutants-scratch` and the repo is neither it nor under it (`reason:"scratch-refused"`, exit 1, no fallback to
      `%TEMP%` or the repo).
    - It measures the scratch's bytes, removes and recreates it; a failed removal is `reason:"scratch-wipe-failed"`.
    - It runs `cargo mutants` with `TMP`/`TEMP` = scratch and `--output <scratch>`, and reads `outcomes.json` / `missed.txt` /
      `timeout.txt` from `<scratch>/mutants.out`.
    - The `mutants` document part gains `scratch_bytes`. Off Windows nothing changes.
  - **`--file` (scoped inner loop):** `verdict:"scoped"` with `files:[…]`; no `mutants-verdict-<leg>.json` is ever written;
    survivors stay red; `--file` + `--leg` → exit 2 `{"reason":"usage","detail":"scoped-leg"}`.
  - **Run document:** gains `archived` (`"target/run-archive/<n>"`, after `mutants` in key order) whenever the run had a source
    to archive; an absent source is named on stderr `run-archive: skipped {name} (absent)`.
  - **pre-push `cache`:** gains `windows_scratch_bytes` (at the cache stage) and `windows_scratch_bytes_after` (after the
    windows leg, when it ran); both are byte counts, no path.
  - **harness_lifecycle tests:**
    - `Booted` guard: `cleanup(.., keep_homes = true)` on drop, then removes the home unless `AGENT_RUN_KEEP_FAILED=1` and the
      thread is panicking.
    - The booted tests' own cleanup calls now assert `home_removed:"kept"`.
  - **`scripts/wsl-exec.sh`:** an operator aid.
    - `[--cd DIR] CMD …` runs `MSYS2_ARG_CONV_EXCL='*' wsl.exe -d Ubuntu [--cd DIR] --exec /usr/bin/env -i HOME=<distro home>
      PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin CMD …`.
    - `--probe` prints `wsl-exec probe: HOME and PATH only, no CLAUDE* name, argv unconverted`.
    - No arguments → usage, exit 2.
  - **Plan gate entry G1:** now `PATH="$PWD/target/tools/ripgrep/bin:$PATH" rg …` with `env = []` (the gate shell has no `rg`,
    `.claude/docs/commands.md:67`).
  - **The five harness commands:** unchanged in name and shape.
- **Cross-project / external claims:**
  - **CI:** ci#36333711860 on **`f0e6dbc857cb`** (the pre-CI commit): verdict green, checks 15/15, wall 2698 s
    (`ci.py conclusion`, evidence/operator-pass.md; also read by the overseer: both legs base `69abc0d`, 200 tested, union breaches
    `[]`).
  - **Older CI verdicts behind the survivors disposition** (evidence/audit-survivors-union.md): 36318398739 (`3efed41`),
    36298052174 (`ed359cd`), 36165685381 (`17ea8c7`).
  - **Platform record:** rstudio/rstudio#18884 (a key lost within ~50 ms of a ConPTY resize; research.md).
  - **cargo-mutants 27.1.0 on Windows** reads `TMP`/`TEMP` through `std::env::temp_dir()`: measured — its temp copy
    `cargo-mutants-viola-IE1lTy.tmp` appeared in the scratch mid-run, none new in `%TEMP%` (evidence/pre-push-sizing.md).
- **Reverted / negative API facts:**
  - Temporary control tests `zz_control_fails` / `zz_control_passes` (harness_lifecycle) and `zz_watch_control` (viola-pty) were
    written, run and removed (`grep -c` = 0 each).
  - The scratch guard was neutralised to `true` for its remove-the-guard pair and restored.
  - `CHILD_WITHIN` went to 10 s and then 12 s for the recorder's pair and was restored to 7 s.
- **Insufficient fixes (written, kept, not the remedy):**
  - **The viola-pty recorder** (`CHILD_WITHIN` 7 s + the report file) captures a recurrence; it does not fix the lost-key/timeout
    watch, whose cause stays unknown.
    - The watch recurred once, the first local red: 2026-09-27T16:02:58Z, the windows leg's cargo-mutants baseline in the
      operator pre-push, `TIMEOUT [10.107s]`, capture absent. It stays **OPEN**, with the expiry counter reset and read **1 of 3**
      after ci#36333711860.
    - Owner: the watch (route CARRY 4 lineage); the product question H2 is owed to the operator.
  - **The host scratch** bounds C: residue (wiped each run) but does not fix the leak behind it: a `viola-fake-agent.exe` that
    outlives a mutant test in cargo-mutants' temp copy. Two such, from `%TEMP%/cargo-mutants-viola-Bk11Ik.tmp` (started 13:16Z),
    were stopped by the overseer by exact path. Owner: none assigned — the operator's.
- **Spec claims disproved by measurement:**
  - (a) **Plan step 1** predicted the clones control at 3; it measured **4**. The fourth is a 6-line fragment
    `server.rs:961 ↔ tests/channel_endpoint.rs:325` outside research fact 10's top-10 basis (evidence/metrics-before.md). A
    plan-only claim, 0 hits in the masters.
  - (b) **The plan's sizing** predicted ≈ 165 mutants per leg; each leg measured **200** (both pre-pushes, evidence/pre-push-sizing.md,
    operator-pass.md). The wall-clock prediction held (≈ 25 min predicted; 1470 s and 1404 s measured). A plan-only claim.
  - (c) **Plan step 6's move list** cannot reach its own ≤ 700 target: 733 tokei lines after it, 820 after steps 7/9/11. It was
    resolved by the two extra moves in Deviations. A plan-only claim.
  - (d) **`test-plan.md:1450` / `:1484` / `commands.md:73`** state the release-check probe line `3/3 refused` — false since step
    14 (`5/5`). A master claim → Expected amendments / Counts.
- **Expected amendments (from plan):**
  - **architecture §Occupied Resources** — the host scratch, `target/run-archive/`, `scripts/wsl-exec.sh`, the `test-support`
    feature (root dev-dependency only), `AGENT_RUN_KEEP_FAILED` now also read by the viola-e2e lifecycle tests, and the new
    submodule paths. **Carried:** Changes → Paths/resources, Schema/config, Env vars, Crates/modules. Search: `sites.py` —
    architecture `Occupied Resources` 3 hits (:76, :106, :321), `AGENT_RUN_KEEP_FAILED` 2 (:368, :546), `viola-pre-push-scratch`
    1 (:546), `rust-analyzer` 2 (:423, :546), `TMPDIR` 2 (:337, :546), `release-check` 7.
  - **test-plan §3** — `run --mutants` on a Windows host (scratch, `--output`, `scratch_bytes`, `scratch-refused` /
    `scratch-wipe-failed`), `--file` + `verdict:"scoped"` + `detail:"scoped-leg"`, `archived`, pre-push
    `cache.windows_scratch_bytes` / `_after`; each new closed value in the Decisions Log. **Carried:** Changes → Harness / gate
    surface. Search: test-plan `mutants.out` 12 hits (:441, :556, :557, :565, :649, :1469, :1508, :1794…), `no-rust-delta` 8,
    `scratch_bytes` 1 (:664), `viola-pre-push-scratch` 2 (:662, :663), `pre-push` 17, `AGENT_RUN_KEEP_FAILED` 2 (:555, :747),
    `mutants-exit` 1 (:557), `Decisions Log` (:453, :518, :672 "A new value needs a Decisions Log entry").
  - **security-plan** — the release-check feature refusal (Decisions Log 2026-09-27 Conditions), and the scratch wipe outside the
    repo and its guard. **Carried:** Changes → Schema/config (release-check), Harness / gate surface (the guard). Search:
    security-plan `release-check` 2 hits (:161, :689), `fake-agent` 5 (:232, :580, :685, :688, :689), `viola-pre-push-scratch` 1
    (:426), `pre-push` 6, `mutants.out` 2 (:388, :651).
  - **obs-plan §8 item 6** — the new pre-push fields are path-free; `target/run-archive/` is never uploaded. **Carried:** Changes →
    Harness / gate surface (pre-push `cache`), Paths/resources (the archive is gitignored and outside `target/agent-run/`). Search:
    obs-plan `pre-push` 2 hits (:1209 the §8 pre-push document line, :1255), `mutants.out` 1 (:1211).
- **Coverage of new surfaces:**
  - `run --mutants --file` → validation clap `requires = "mutants"` + `scoped-leg` refusal ✓ · instrumentation n/a (harness
    document) · PII n/a · tests unit (`run_mutants_scoped_*`, bin `run_file_is_repeatable_and_needs_mutants`) · a11y n/a · tokens n/a
  - host mutation scratch (wipe outside the repo) → validation `scratch_allowed` guard ✓ (remove-the-guard pair,
    evidence/scratch-guard.md) · instrumentation n/a · PII n/a (path never printed; `scratch_bytes` a count) · tests unit (13
    `scratch` tests) + the scoped entry + the pre-push · a11y n/a · tokens n/a
  - `target/run-archive/` → validation n/a · instrumentation n/a · PII n/a (outside every upload path; gitignored) · tests unit
    (`archive_*`, `run_mutants_archives_the_outcomes_it_read`) · a11y n/a · tokens n/a
  - pre-push `cache.windows_scratch_bytes[_after]` → validation n/a · PII path-free✓ (`pre_push_document_carries_no_absolute_path`
    extended) · tests unit (`pre_push_cache_reports_the_host_scratch_before_and_after_the_windows_leg`) · a11y n/a · tokens n/a
  - `scripts/wsl-exec.sh` → validation fail-closed (absolute distro HOME required; usage exit 2) ✓ · PII env scrubbed (`env -i`,
    probe proves no `CLAUDE*`) ✓ · tests the `--probe` gate entry · a11y n/a · tokens n/a
  - `viola-channel` `test-support` feature → validation release-check refusal ✓ (probe 5/5; release build `viola only`) · tests
    `json_fields_records_each_type_as_its_json_value`, `canonical_sddl_renders_well_known_sids_as_their_aliases`,
    `user_sid_is_a_user_sid` · the rest n/a

## Deviations from intent
1. **The `run/mutants.rs` split moved more than step 6 listed.** The chunk-diff classifiers went to `mutants/base.rs`, and the per-leg
   verdict (4 fns + its test) went to a new `mutants/leg.rs`, a file the plan does not list. Justification: step 6's list left 733
   tokei lines against its own ≤ 700 target, and 820 once steps 7/9/11's tests landed; now 697. It costs more moved mutants per leg
   (200 measured vs ≈ 165 predicted).
2. **A labelled case table for the scratch guard, not rstest.** viola-e2e has no `[dev-dependencies]`; a table avoids a new
   manifest edge, and each case names itself in its assertion.
3. **No separate "not a drive or filesystem root" check.** A root has no final path component, so the exact-name check refuses it;
   a second check would be an unkillable redundant mutant (testing.md 2026-09-24). Cases for `/` and `C:\` witness the refusal.
4. **The archive names skipped sources on stderr.** `archive()` returns them so the condition is testable; the document gains only
   `archived`, as planned.
5. **`keep_home` omits `AGENT_RUN_KEEP_HOMES`.** It is `keep_decision` without it, so passing tests remove their homes as before
   ("as today" in step 10).
6. **`mini()` and `pinned()` place their repo one level down**, so each test's sibling scratch is its own. Without it, parallel
   tests would share `%TEMP%\viola-mutants-scratch` and wipe it under each other.
7. **metrics.py `SPLIT_PARENTS`.** The allowed-list check reads a new submodule end at its parent file, so a clone that moved with
   its concern is not read as new. The in-scope check reads real paths.
8. **Plan entry 22 (G1) edited on the overseer's direction.** The pinned ripgrep PATH is inline plus `env = []`, after a 127 red
   (no `rg` in the gate shell). The entry text differs from the phase's plan.
9. **The viola-pty recorder, on the overseer's direction after the operator pre-push red.** `CHILD_WITHIN` 10 → 7 s (a lowering;
   plan step 2 forbids raising it), the known-path report file and the `Child` drop. Recorder only, no fix; see Insufficient fixes.
10. **rust-analyzer left running for both pre-pushes and the scoped run.** The plan's Test Commands say the stop is needed only until
    step 7 lands; no gate failed on it.

## Decisions & corrections
- **Overseer (founder-delegated) decisions this chunk:**
  - accepted implement's green and all deviations;
  - pinned PATH into G1 and `chmod +x` for wsl-exec.sh before the pre-CI commit;
  - "fold the RECORDER, not a fix";
  - the red stays OPEN, a green re-run never closes it, and the expiry counter resets;
  - run entry 35 ONCE after the fold;
  - the 8 root-package tests with 10 s waits equal to the `mutants` kill (`tests/support/fake.rs` `WAIT_WITHIN`,
    `tests/support/home.rs` `READY_WITHIN`, `tests/support/outer_pty.rs` `EXIT_WITHIN`, `tests/run_cli.rs` `READY_WITHIN` and
    `:262`, `tests/cli_instance_state.rs:220`, `tests/contract_diag_schema.rs:244` and `:264`) fold into the NEXT chunk at its
    phase P1, **not as a CARRY**.
- **Founder:** removed `mutants.out/` (7 560 311 B) and `mutants.out.old/` (7 156 549 B) at 17:19 local, after the session's `rm
  -rf` was denied by the permission layer.
- **Measured rule (from the recorder pair):** a test's own wait bound EQUAL to the nextest kill line is a race, not a guaranteed
  loss: at 10 s the dump got out once (control) and was lost once (the 16:02Z red); past the kill (12 s) it is always lost; at 7 s
  the test always wins. The bound must sit strictly below the kill.
- **Measured:** cargo-mutants' temp copy on Windows follows `TMP`/`TEMP`, so a harness-set `TMP` moves a 12+ GB copy off C:.
- **Sweep hazard:** in the interactive Git Bash, `command -v rg` answers `rg` because `rg` is a shell FUNCTION there, while the gate
  tool's non-login bash has no `rg` at all (entry 22 exit 127). The pinned binary is `target/tools/ripgrep/bin/rg.exe`, put on PATH
  per `.claude/docs/commands.md:67`. Probe a tool with `type`, not `command -v`.
- **Sweep hazard:** jscpd's audit "top 10" is by size; a 6-line fragment of an in-scope clone class sat outside it (the clones
  control read 4, not 3).
- **Sweep hazard:** a unit test that builds a throwaway repo at the tempdir ROOT gets a scratch sibling SHARED with every other test
  (`%TEMP%/viola-mutants-scratch`); nest the repo one level down.
- **Host friction:** bash-guard refused a heredoc with a file target and doubled backslashes; Git Bash `rm -rf` of a repo dir was
  permission-denied.
- **Carried deferred learnings from the handoff (not re-decided here):** the `"777"` digit-substring sweep hazard; "a red found now
  folds into this chunk even outside its diff; a green re-run never closes a red" (now also stated by the overseer this chunk).

## Outcome
**Acceptance criteria (re-asserted against the diff):**
- (arch) No file over 800 tokei code lines; the four ≤ 700: **met** — `metrics.py size` last line `0`; 654/590/697/668.
- (arch) orphans-check passes, every new submodule reached, no out-of-line `#[cfg(test)]` file: **met** —
  `orphans-check: … clean` (gate); every moved test module is inline.
- (arch) No new workspace member, no product crate on viola-e2e, viola-pty deps unchanged, no tracing-subscriber in a product
  crate's normal deps: **met** — `Cargo.lock` unchanged; viola-channel's manifest adds only a feature.
- (tests) `metrics.py clones` last line `0`, both DACL witnesses remain: **met** — 0; the in-crate one
  (`server/win.rs bound_pipe_dacl_reads_back_protected_user_and_system`) and the root one
  (`channel_endpoint_pipe_dacl_is_protected_user_and_system`).
- (tests) Every external name resolves at its former path; unit/integration green with unchanged test names/assertions: **met**,
  with one noted change — the harness_lifecycle main test now asserts `home_removed:"kept"` plus home removal at guard drop (step
  10's own design).
- (tests) The scratch guard refuses each bad path and accepts the sibling; its pair recorded: **met** (evidence/scratch-guard.md).
- (tests) `run --mutants` takes the scratch; `scratch_bytes` in the scoped document; `test ! -e mutants.out` after it: **met**; the
  pre-push windows leg ran from `viola-mutants-scratch/cargo-mutants-viola-*.tmp`.
- (tests) `--file` → `verdict:"scoped"`, no leg verdict; `--file` + `--leg` → exit 2 `scoped-leg`: **met**.
- (tests) A failing lifecycle test under `AGENT_RUN_KEEP_FAILED=1` keeps its home, a passing one removes it: **met**
  (evidence/keep-failed-home.md).
- (tests) The forced-window test 20/20 green, or stop at step 2: **met** — 20/20 three times (before the split, the gate, and after
  the recorder fold).
- (tests) pre-push `ok:true` at `stage:"union"`, cache carries the two fields, no absolute path: **met** — the implement gate
  pre-push and the operator second run.
- (security) release-check probe `5/5 refused, control clean`; release-check `viola only`: **met**.
- (security) `target/run-archive/` gitignored; scratch outside the repo; wsl-exec probe passes: **met**.
- (security) One `security_descriptor(`, zero `try_overwrite` / `ListenerOptionsExt::mode`; the channel security tests pass
  unchanged: **met**.
- (obs) G1, the raw `event!` probe, G4 pass; the `obs_event!` census identical before and after: **met** (evidence/metrics-before.md).
- (a11y/design) `viola run` writes nothing of its own; tui passthrough tests pass after the pty split: **met** (integration entry
  green).
- (tests) `cargo machete crates` reports nothing; bare `cargo machete` lists only `viola-fuzz arbitrary`: **met**.

**Gates** (implement run `.andromeda/runs/2026-09-27T14-42-16-implement`; by `run` text):
- `cargo fmt --all --check` green · `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` green ·
  `bash scripts/lint-probes.sh` green · `bash scripts/orphans-check.sh` green · the sync-crates `cargo check` green · `cargo deny
  --config deny-sync.toml … viola-channel … check bans` green · `cargo deny check` green · `bash scripts/deny-probes.sh` green.
- `bash scripts/release-check.sh --probe` green (`5/5 refused, control clean`) · `bash scripts/release-check.sh` green (`viola
  only`).
- `metrics.py size` green (`0`) · `metrics.py clones` green (`0`) · `cargo machete crates` green · `cargo machete` green (exit 1,
  lacks proc-macro2, contains viola-fuzz).
- `run --unit` green · `run --unit --filter 'package(viola-channel) | package(viola-pty) | package(viola-e2e)'` green (re-run
  green after the recorder fold) · the 20× forced-window loop green (re-run green after the fold).
- `AGENT_RUN_KEEP_HOMES=1 … run --integration` green · the integration filter entry green · `schema-check` green · `secret-scan`
  green.
- The G1 `rg` entry: red · exit 127 at the block run (no `rg` on the gate shell PATH) → re-run with the pinned PATH, green → entry
  edited (operator-directed) to carry the PATH inline, green unaided (exit 1, no output).
- The `try_overwrite|ListenerOptionsExt::mode` grep green · the `security_descriptor(` count green (`1`) · `git check-ignore -q
  target/run-archive/1` green · `bash scripts/wsl-exec.sh --probe` green · `run --mutants --file …/scratch.rs` green (21/21 caught,
  `verdict:"scoped"`, `scratch_bytes` present).
- `test ! -e mutants.out`: red at the block run (17:18:14, before the founder's 17:19 removal) → re-run green.
- smoke ×4 (cleanup, boot, status, cleanup `p-cleanup-smoke`) green · the MSRV `cargo check` (1.96) green.
- `bash scripts/agent-run.sh pre-push` green (1470 s; legs 200/200; union 0 breaches).
- `leg = 'operator'` entries, treatment recorded in evidence/operator-pass.md:
  - `bash scripts/agent-run.sh pre-push` — first run **red** (`stage:"windows-leg"`, `mutants-exit-4`: the unmutated baseline's
    `TIMEOUT [10.107s]` of the watched viola-pty test), the pass stopped; after the recorder fold, run once more: **green**
    (16:09:39Z → 16:33:03Z, `stage:"union"`, 0 breaches);
  - `git diff --quiet && git diff --cached --quiet && git push origin build/viola-0.1.0` → fast-forward `a0e6506..f0e6dbc`;
  - `ci.py conclusion --sha HEAD --wait 5400` → `f0e6dbc857cb verdict: green · checks 15/15 · wall 2698 s · runs ci#36333711860`.
- Smoke (boot path changed: `viola-harness`): ✓ — boot ok, status `ready`, cleanup `processes_gone:true`, `endpoint_gone:true`.

**Outcome basis:** the operator pass ran (`f0e6dbc` is its pre-CI commit, no fix commits after it). The verdict rests on its final
state: the green operator pre-push (op-35b) on the uncommitted tree, and CI ci#36333711860 on `f0e6dbc`, recorded in
evidence/operator-pass.md. The implement conversation is present and is the basis for the gate block, the controls and the
deviations. The two post-commit files (evidence/operator-pass.md's CI section, `op-37.out`) carry records, no source.

**Process hygiene** (re-measured on the host process list at 17:20Z): no `viola-harness`, `viola-fake-agent`, `cargo-mutants` or
`cargo-nextest` process remains; the WSL distro is stopped. The two leaked `viola-fake-agent.exe` from `Bk11Ik` (13:16Z, before this
chunk) were stopped by the overseer. `viola.exe` from `additional/viola-lab/prototype` is another project's: not attributable,
left. rust-analyzer ×2 is the session's own.
