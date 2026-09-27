# Codebase Research — 2026-09-27-epoch-2-cleanup

## Scope
- **Depth:** deep · **Reads:** 16 · **Globs/Greps:** 22 · **Graph queries:** 2 (trace `.andromeda/runs/2026-09-27T14-05-04-phase/tree-query-2026-09-27-epoch-2-cleanup.json`)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` (in full, 5 Session Additions applied: `--leg windows-2025` for
  `#[cfg(unix)]` diffs, judge a leg by its verdict never its counts, `test(=tests::name)` filters) · `.claude/rules/testing.md` (in full, 15
  Session Additions applied: inline test modules only (2026-09-25), forced-window holds (2026-09-27), in-crate security tests
  (2026-09-27), const-initializer mutants (2026-09-27), the 20 s mutant bound (2026-09-24)) · `.claude/rules/observability.md` (auto-loaded)
- **Platform issues consulted:** filled at P5 (check 9: scope item 7 carries the watch's runner-only red, run 36296402785, job
  108555954166). Query `ConPTY ResizePseudoConsole input lost keystroke after resize microsoft/terminal issue` (web, 2026-09-27) →
  the fetched rstudio/rstudio PR #18884 states: "A keystroke typed within ~50ms of a process_set_size RPC was lost on Windows CI
  (ConPTY + MSYS bash)"; "I found no loss point in RStudio's input path … so the drop appears to happen below RStudio (conhost / the
  MSYS runtime)"; its fix detects the loss and retypes once in its e2e tests; it cites no microsoft/terminal issue. Also listed,
  not fetched as a match: microsoft/terminal #10400 (a resize ignored near a client attach). Reading: the red's signature (a key
  written right after `resize` never reaching the child, windows-2025) matches a publicly reported ConPTY/conhost loss window.

## Files inspected
- `crates/viola-e2e/src/harness/pre_push.rs` (full: 1–605 production, 606–1373 inline tests) — the 12 stages; the Linux mutation
  scratch (`SCRATCH_DIR` :33, `cache` :438–459 wipes + recreates 0700 and reports `scratch_bytes`, `stages` :250–255 adds `bytes_after` /
  `scratch_bytes_after`); the host `windows-leg` (:270–285) calls `run_with` with no temp redirection; the one per-stage env wrap is the
  `capped` closure (:261–265, `CARGO_BUILD_JOBS=16`).
- `crates/viola-e2e/src/harness/run/mutants.rs` (300–404; outline of the rest) — `mutants()` deletes `mutants.out/outcomes.json` (:343),
  builds the root package, runs `cargo mutants --workspace --features fake-agent --in-diff <chunk.diff> --test-tool=nextest
  --copy-target=true --caught --unviable --build-timeout-multiplier=5` with `AGENT_RUN_KEEP_HOMES=0` / `AGENT_RUN_KEEP_FAILED=0`
  (:357–378); reads `ws.root/mutants.out` (:342, :382). `chunk_diff` (:129–157) is `git diff <merge-base>` with no rename flag plus
  untracked files as `/dev/null` diffs.
- `crates/viola-pty/src/lib.rs` (395–489, 969–1165; outline of the rest) — `HostTerminal::enter` has a `#[cfg(windows)]` body (:403–425)
  and a `#[cfg(unix)]` body (:427–440); `host_size` has one block per OS (:461–489). The watch test
  `spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code` (:1123–1163) drives `tests::pty_child_entry` (:982–1020)
  through `spawn_child_entry` (:1051–1090); `lines()` waits `CHILD_WITHIN` = 10 s (:1092–1109).
- `crates/viola-channel/src/server.rs` (outline; 896–1000) — `mod win` (:456–554, the DACL/SID code), the Windows DACL read-back test
  (:896–950) and `canonical_sddl` (:953–990).
- `crates/viola-channel/src/lib.rs` (85–135) — `#[cfg(test)] mod test_capture`: the `Fields` visitor (5 `record_*`) + the `Captured` layer.
- `src/cmd/run.rs` (355–400) — its test module's own `Fields` visitor (3 `record_*`) + a `Spans` layer with parents and `on_record`.
- `tests/channel_endpoint.rs` (320–410; outline) — `mod win` (:294–410): `canonical_sddl`, an independent `user_sid` ("apart from the
  product's own lookup"), `dacl_of`; used by `channel_endpoint_pipe_dacl_is_protected_user_and_system` (:145–152).
- `crates/viola-e2e/src/bin/viola-harness.rs` (150–170) — `cleanup_cmd` keeps a home only for `AGENT_RUN_KEEP_HOMES=1` (:162).
- `crates/viola-e2e/tests/harness_lifecycle.rs` (outline) — the watch test calls `cleanup(&ws, Target::Session(..), false)` directly (:76).
- `tests/support/home.rs` (grep) — the root chain's `keep_decision(keep_homes, keep_failed, panicking)` (:21–23) is the existing
  keep-failed rule.
- `.github/workflows/ci.yml` (upload steps) — `harness-<os>` uploads `target/agent-run/` minus `chunk.diff` (:114–118); `junit-<os>`
  uploads `target/nextest/ci/junit.xml`; `mutants-verdict-<os>` one JSON; nothing else under `target/`.
- `crates/viola-core/Cargo.toml` — normal deps: `nutype` only (no `tracing`, no `serde_json`).
- `viola-0.1.0/chunks/2026-09-27-instance-state-and-start-order/evidence/ci-red-36296402785.md` — the viola-pty watch's only red.
- `viola-0.1.0/chunks/2026-09-27-wrapper-channel/evidence/operator-pass-continued.md` (20–94) — measured pre-push timings; a leaked
  `viola-fake-agent.exe` from a cargo-mutants temp copy (:34–35).
- `.andromeda/runs/2026-09-27T07-34-11-implement/fix-pp5.err` / `fix-pp3.err` (the per-leg cargo-mutants summary lines).
- Code-audit inputs: `c-mutation-viola-{channel,state,pty}.json`, `c-duplication.json`, `c-sizes.json`, `record.py` (instruments).

## Graph impact (code-graph, plane rust, fresh at `a0e6506`)
- **viola-pty external surface** (canonical query 5) — 35 rows, all defined in `crates/viola-pty/src/lib.rs`: `Size` 20 sites / 6
  files, `PortablePty` 9/3, `SpawnSpec` 6/3, `HostTerminal` 5/2, `Pty` 4/3, `PtyError` 4/1, `host_size` 3/2, `spawn` 3/3, `pump` 1/1,
  `PumpEnd` 3/1, `PTY_BACKEND` 2/2, `enter` 2/2 … (trace row 1). Every one stays reachable at `viola_pty::<name>`: a split re-exports.
- **viola-channel external surface** — 16 rows; from `server.rs`: `Server` 13/5, `bind` 9/5, `Dispatch` 6/3, `Serving` 4/2, `serve`
  4/3 (trace row 2). `mod win` and `test_capture` have NO external reference (both crate-internal), so moving `win` into a submodule
  changes no caller outside `viola-channel`.
- `viola-e2e` harness modules are consumed only inside the crate and its `viola-harness` bin (`pre_push`, `pre_push_with`, `wsl_path`,
  `ci_pins`, `last_document`, `PRE_PUSH_HOST_SUPPORTED`, `CACHE_CAP_BYTES` via `harness::pre_push::`); `derived-without-graph` for
  the bin's `use` paths, read directly (viola-harness.rs).

## Measured facts (each with its derivation)
1. **Sizes** (tokei 14.0.0 `code`, the audit's instrument per `record.py:49`; re-derived: `tokei -f -o json <file>`): `pre_push.rs`
   1335, `viola-pty/src/lib.rs` 1074, `run/mutants.rs` 1029, `server.rs` 869 (`wc -l` 1373 / 1218 / 1155 / 1025). Production/test line
   split (outline grep of `#[cfg(test)]`): pre_push 1–605 / 606–1373; mutants 1–404 / 405–1155; pty 1–489 / 490–1218; server 1–554 /
   555–1025.
2. **Mutants in the four files** (cargo-mutants 27.1.0 `--list --json --workspace --features fake-agent --file <f>`, this host):
   pre_push 88 · pty 85 · run/mutants 84 · server 68 = **325**. Per function (same listing): pre_push Linux side (`Linux::*` 3,
   `distro_home` 2, `tools` 2, `sync` 7, `dir_bytes` 2, `target_bytes` 2, `cache` 4, `linux_harness` 1, `linux_tests` 3, `linux_leg` 4)
   = 30; run/mutants base resolution (`git` 3, `commit_sha` 3, `resolve_base` 4, `chunk_flip` 6, `uncommitted_promotion` 3,
   `pre_ci_parent` 3, `pending_marker` 6, `chunk_diff` 4) = 32; pty pump (`spawn_worker` 1, `copy` 1, `pump` 8, `drain` 3,
   `kill_and_reap` 4) = 17; pty host terminal (`raw_input_mode` 7, `vt_output_mode` 4, `size_from_window` 10, `nonzero_size` 9,
   `HostTerminal::enter` 10, its `drop` 1, `host_size` 4) = 45; server `win::*` = 12.
3. **Moved production lines are added lines under `--in-diff`** (re-derived from `chunk_diff`, mutants.rs:129–157: plain `git diff
   <merge-base>`, a new file is all `+` lines; a parent file that keeps a line keeps it out of the diff). Epoch 1's `run.rs` split
   (parent kept, `harness/run/*.rs` added — `ls crates/viola-e2e/src/harness/`) re-tested 143–148 mutants per leg
   (`epoch-1-cleanup/evidence/operator-pass-3.md:20`, `-4.md:21`), against its scope's 154 (`epoch-1-cleanup/scope.md:63`); the
   relayed "~154" is the scope's count, the measured legs 143/148.
4. **Leg speed at HEAD's configuration** (`fix-pp5.err`, the green `3efed41` pre-push, 11:50:24Z → 12:14:53Z = 1469 s): `ubuntu-latest`
   `190 mutants tested in 6m`, `windows-2025` `190 mutants tested in 17m` (≈ 1.9 s and 5.4 s per mutant); the non-leg stages ≈ 1.5 min.
   `fix-pp3.err` (`90eaf46`): 185 in 5 m and 15 m. Epoch 1's viola-e2e-heavy diff: host windows 143 in 12–13 m, CI ubuntu 148 in 10 m.
5. **The 11 audit survivors against the project's own verdict** (CI `mutants-verdict-<leg>.json` artifacts, downloaded with `gh run
   download`): the audit ran cargo-mutants per crate on this Windows host only (`c-mutation-*.json`), with no union.
   - `viola-channel` 4 — all in `#[cfg(unix)]` code (`client.rs:193` `open`, `endpoint.rs:75` `host_socket_dir`, `server.rs:66–79` the Unix
     `Guard` drop, `server.rs:106` Unix `listen`). Run 36318398739 (`3efed41`) ubuntu leg: `endpoint.rs:76` caught, `server.rs:75`
     caught, `client.rs:195` unviable, `server.rs:108` unviable.
   - `viola-state` 2 — `fs.rs:16` `restrict` (its Windows body is a no-op) and `pin.rs:74` (the `NotFound` guard). Run 36298052174
     (`ed359cd`) ubuntu leg: both caught (windows leg: both missed).
   - `viola-pty` 5 — all in the `#[cfg(unix)]` `HostTerminal::enter` body and the Unix block of `host_size`, NOT Windows code. Run
     36165685381 (`17ea8c7`, lines then 10 lower): ubuntu `420:9` None caught, `420:9` Some(Default) unviable, `422:64` caught, `427:71`
     caught, `472:76` caught.
   - So all 11 were caught or unviable on the leg that compiles them; the compiling-leg union (`harness::cfg_legs`) never counted them.
6. **The viola-pty watch's red** (`ci-red-36296402785.md:20–33`): `test (windows-2025)` under `cargo llvm-cov nextest`, `FAIL [10.031s]`,
   report stopped at `["start pid=4284 raw=true size=100x30", "byte 78"]` — the `y` written right after `resize(120x40)` (lib.rs:1141–1149)
   was never reported, so `lines(&child, 4)` hit its 10 s bound. Not a hang: a bounded wait. Host 60/60 green.
7. **The leak behind item 4** — measured once: a stray `viola-fake-agent.exe` from a windows mutation leg's cargo-mutants temp copy
   (parent gone) stopped by pid (`operator-pass-continued.md:34–35`). cargo-mutants copies the tree into `std::env::temp_dir()`
   (pre_push.rs:30–32's own comment); on Windows that is `%TMP%`/`%TEMP%` (C:).
8. **Upload scope** (`ci.yml:114–118`): anything under `target/agent-run/` except `chunk.diff` is uploaded (after `secret-scan`); an
   archive of `outcomes.json` there would upload absolute argv paths — the class the `mutants.out/` upload removal refused
   (obs/security sidecars, quality-gates). `target/pre-push/` and other `target/` paths are never uploaded.
9. **No rust-analyzer running now** (`Get-CimInstance Win32_Process -Filter "Name='rust-analyzer.exe'"`: 0 rows); cargo-machete 0.9.2
   on the host (`cargo machete --version`), the audit's pin.
10. **Clone pairs** (`c-duplication.json` `top`, jscpd 5.0.16): `server.rs:919` ↔ `tests/channel_endpoint.rs:385` (21 L, the DACL read-back),
    `server.rs:967` ↔ `tests/channel_endpoint.rs:330` (17 L, `canonical_sddl`), `viola-channel/src/lib.rs:103` ↔ `src/cmd/run.rs:372`
    (13 L, the `Fields` visitor). Also in the top 10, not named by the entry: `viola-pty/src/lib.rs:532` ↔ `:713` (10 L, two pump tests,
    same file), `tests/channel_sqos_open.rs:90` ↔ `tests/security_negatives_channel.rs:72` (19 L), `crates/viola-channel/tests/
    channel_frames.rs:40` ↔ `tests/channel_endpoint.rs:51` (11 L), and three more outside this chunk's files.

## Patterns detected
- **Parent file kept, submodules beside it** (`harness/run.rs` + `harness/run/*.rs`, Epoch 1): the parent stays the module root, no
  `mod.rs`; a split keeps the parent's unmoved lines out of the diff.
- **Inline tests only** (testing.md 2026-09-25): each moved submodule carries its own `#[cfg(test)] mod tests { … }`; shared test
  scaffolding stays inline in the parent's test module as `pub(super)` items (private items of a module are visible to its
  descendants), never an out-of-line `#[cfg(test)] mod x;`.
- **Per-stage env wrap** (pre_push.rs:261–265): a closure over the runner adds env to every command of a stage.
- **Scratch beside the tree, wiped at `cache`, reported before/after** (pre_push.rs:438–459, 250–255): the Linux model for item 4.
- **keep_decision** (tests/support/home.rs:21–23): keep for `AGENT_RUN_KEEP_HOMES=1`, or for a panicking test under
  `AGENT_RUN_KEEP_FAILED=1`.
- **Forced-window hold as a test-only mode**: the pre-push fix used the `fake-agent`-only `FAKE_AGENT_PUMP_DELAY_MS`; the viola-pty test
  child already switches on a test-binary env var (`CHILD_MODE`, lib.rs:998) — a new mode needs no product seam.

## Conventions to follow
- A split re-exports every externally reached name at its current path (`pub use` in the parent): graph rows above.
- `PtyError` keeps its hand-written `Display` (lib.rs:90–101); `viola-pty` normal deps stay portable-pty, tracing, windows-sys, libc.
- Harness documents: one JSON document, closed values; a new field/value is a test-plan §12 Decisions Log entry (wrap).
- No new `AGENT_RUN_*`-less env var; harness-only overrides carry `AGENT_RUN_`.

## New files to create
- `crates/viola-e2e/src/harness/pre_push/linux.rs` (or the concern the plan picks) — a moved concern of `pre_push.rs` + its inline tests.
- `crates/viola-e2e/src/harness/run/mutants/base.rs` — the base-resolution concern of `run/mutants.rs` + its tests.
- `crates/viola-pty/src/pump.rs` — the pump machinery (+ tests), re-exported from `lib.rs`.
- `crates/viola-channel/src/server/win.rs` — `mod win` (+ its DACL tests).
- `scripts/wsl-exec.sh` — the WSL crossing wrapper.
- The shared test-support home for the two clone classes (placement is the P4 fork below).

## Files to modify
- `crates/viola-e2e/src/harness/pre_push.rs` — scratch for the windows leg (item 4), the split.
- `crates/viola-e2e/src/harness/run/mutants.rs` — the split; the scoped inner loop and the `--output` / archive aids (item 5).
- `crates/viola-e2e/src/harness/run.rs` — `Selection` / flags for a scoped mutants run (if the plan takes a flag).
- `crates/viola-e2e/src/bin/viola-harness.rs` — the flag parse; `cleanup_cmd`'s keep rule (item 5, P3).
- `crates/viola-e2e/tests/harness_lifecycle.rs` — the watch test keeps its home when it fails (item 5 P3 + item 7).
- `crates/viola-pty/src/lib.rs` — the split; a new `CHILD_MODE` hold in `pty_child_entry` for the forced window (item 7).
- `crates/viola-channel/src/server.rs`, `crates/viola-channel/src/lib.rs`, `src/cmd/run.rs`, `tests/channel_endpoint.rs` — the dedupes.
- `crates/viola-e2e/Cargo.toml` — `[package.metadata.cargo-machete] ignored = ["proc-macro2"]`.
- Manifests the test-support placement touches (`crates/viola-channel/Cargo.toml` and the root `Cargo.toml` `[dev-dependencies]`, or
  `crates/viola-core/Cargo.toml`), per the fork.
- Companion sweep (name grep over the whole tree, `grep -rn 'pre_push::\|run::mutants::\|viola_pty::\|test_capture'`): the in-crate
  `use super::…` sites move with their code; external users are the graph rows above, all kept by re-export — no other file changes.

## Open questions
- Where do the shared test helpers live (the `Fields` visitor and the DACL read-back/`canonical_sddl`)? → blocks: plan-decision (P4 fork).
  Options measured: `viola-core` has no `tracing`/`serde_json` (Cargo.toml); `viola-channel` has both as normal deps and is a normal
  dep of the root.
- rust-analyzer: the harness stops it by exact path, or `--output` moves `mutants.out` off the watched tree? → blocks: plan-decision
  (P4 fork). Whether rust-analyzer watches `target/` is unmeasured (hypothesis either way); a dir outside the repo is outside every
  workspace root by construction.
- Which concerns each split moves (drives the diff mutant count, fact 2) → blocks: plan-decision (P4, the sizing).
