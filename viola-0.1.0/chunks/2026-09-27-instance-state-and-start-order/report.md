# Report — 2026-09-27-instance-state-and-start-order

**Chunk:** Instance state and start order: append-only event log, atomic snapshot, heartbeat, pinned bin/ copy with
re-hash refusal, plugin folder by absolute path, documented start order, live/stale name refusal, VIOLA_* at spawn
**Date:** 2026-09-27
**Commits:** `39b8297` chore(2026-09-27-instance-state-and-start-order): operator pre-CI commit, for the run this
chunk's verdict reads · `ed359cd` fix(2026-09-27-instance-state-and-start-order): run_cli's canary scan skips the
byte-identical pinned copy, measured in run 36296402785 (base = the pre-CI commit's parent `a892917`)

## Changes (structured — detectors read this)
- **Files:** (basis `git diff --name-status a892917`, run dirs and the chunk folder excluded) new:
  `crates/viola-state/{Cargo.toml, src/lib.rs, src/fs.rs, src/events.rs, src/snapshot.rs, src/heartbeat.rs,
  src/liveness.rs, src/pin.rs}`, `plugin/.claude-plugin/plugin.json`, `plugin/hooks/hooks.json`, `plugin/.mcp.json`,
  `tests/cli_instance_state.rs`; modified: `Cargo.toml`, `Cargo.lock`, `scripts/sync-crates.txt`,
  `crates/viola-core/src/lib.rs`, `crates/viola-agent-claude/src/lib.rs`, `src/obs.rs`, `src/cmd/run.rs`,
  `src/run/mod.rs`, `src/bin/viola-fake-agent.rs`, `crates/viola-e2e/src/bin/viola-harness.rs`,
  `crates/viola-e2e/src/harness/{boot.rs, logs.rs, pre_push.rs, run/mutants.rs}`, `tests/support/home.rs`,
  `tests/tui_env_strip.rs`, `tests/run_cli.rs`, `tests/cli_fake_agent.rs`; plus the phase's promotion
  (`.andromeda/master-route.md`, `viola-0.1.0/working-route.md`, `viola-0.1.0/verification-matrix.json` notes).
- **Symbols / APIs:**
  - `viola-core`: `pub enum EventKind { Wheel, BudgetGate }` + `EventKind::as_str()` → `"wheel"` / `"budget-gate"`
    (no serde derive; viola-core has no serde dependency — `viola-state` serialises it through `as_str`).
  - `viola-state` (new, sync): `StateError { Io, Encode }` (fixed messages) · `timestamp(DateTime<Utc>)` (RFC 3339
    ms `Z`) · `fs::{create_private_dir, open_private_append, open_private_lock, replace_private,
    replace_private_shared, DIR_MODE, FILE_MODE}` · `events::{EventLine {v, ts, instance, kind, source, data},
    Source::Wrapper, append_event, EVENTS}` · `snapshot::{InstanceSnapshot, Wheel {Driver, Human}, write_snapshot,
    read_snapshot, SNAPSHOT}` · `heartbeat::{touch_heartbeat, beat_age, Heartbeat::start, HEARTBEAT, BEAT_EVERY}` ·
    `liveness::{Liveness {Live, Stale, Gone}, classify, process_start_time, own_start, same_process, STALE_AFTER}` ·
    `pin::{content_key, pin_exe, Pinned {key, path, path_fwd}, PinError {HashMismatch, State}}`.
    - `create_private_dir` creates each missing component with mode 0700 and then sets 0700 (the umask can only
      narrow), tolerating `AlreadyExists` from a concurrent creator; an existing target is narrowed.
    - `open_private_lock` opens `.lock` siblings for WRITE (not append): Windows denies `File::lock` on an
      append-only handle (measured: `Access is denied`, os error 5, on `events.ndjson.lock` and
      `snapshot.json.lock`).
    - `replace_private` = `tempfile::NamedTempFile::new_in(parent)` → mode set on the temp file BEFORE the write →
      `write_all` → `sync_all` → `persist`. `replace_private_shared` = the same, and a failed replace counts as done
      when the target already holds exactly those bytes (used for the pinned copy and the plugin files, which every
      concurrent start of one version writes identically).
    - `Heartbeat` guard holds only the channel `Sender`; dropping it ends the thread at its next wake-up (no join).
    - `classify(beat_age, same_process)`: not the same process → `Gone` whatever the beat; else age ≤ 5 s →
      `Live`, older or absent → `Stale`. `same_process` compares the snapshot's `started_at` (the wrapper's own
      sysinfo `start_time()` seconds, rendered RFC 3339) with sysinfo's value for the snapshot `pid`.
    - `pin_exe(home, exe)`: key `<VERSION>-<hex(sha256(exe))[..16]>`, copy at `bin/<key>/viola(.exe)` mode 0700;
      an existing copy is re-hashed in 64 KiB chunks and any mismatch → `PinError::HashMismatch` (file left as found).
  - `viola-agent-claude`: `pub const PLUGIN_DIR_FLAG = "--plugin-dir"`; `pub fn plugin_files(pinned_bin_fwd) ->
    [(&str, String); 3]` for `.claude-plugin/plugin.json`, `hooks/hooks.json`, `.mcp.json` (templates embedded by
    `include_str!` from repo-root `plugin/`; `@@VIOLA_VERSION@@` and `@@VIOLA_BIN@@` tokens rendered). Contents:
    `plugin.json` `{name:"viola", version, description}`, `hooks.json` `{"hooks": {}}`, `.mcp.json`
    `{"mcpServers": {}}` (operator ruling at P4: empty until the `hook`/`mcp` verbs exist).
  - root bin: `src/cmd/run.rs` `run()` split into `collision_check`, `pin_and_plugin`, `start_state`,
    `spawn_and_pump`, with refusal fns `refuse_live`, `refuse_stale`, `refuse_tampered_pin` (and the existing
    `refuse_batch_script`) over one `refuse(unable, hint)`; sole product caller `cmd::dispatch` unchanged.
    `src/run/mod.rs` `ChildLaunch` + `child_launch(name, instance_dir, pinned, plugin_dir, inherited_path, user_args)`.
    `src/obs.rs` `ensure_private_dir` / `open_private_append` now delegate to `viola_state::fs` (callers unchanged).
  - Start order in `viola run`: program resolution (the `.cmd`/`.bat` refusal stays first and writes nothing) →
    collision check (read snapshot → `classify`) → `pin_exe` + plugin folder rewrite (`plugin/<key>/`, every start)
    → first snapshot (`pid`, `started_at`, `pinned_bin`, `cli_verified:false`, `wheel:"driver"`,
    `budget_paused:false`, `links:[]`; no `endpoint`, no `child_pid`) → `touch_heartbeat` + `Heartbeat::start` →
    `wheel{holder:"driver",cause:"start"}` then `budget-gate{paused:false}` (`source:"wrapper"`) → spawn → snapshot
    rewritten with `child_pid` → `process-start` claude-child line. The version-gate and endpoint-bind slots have no
    step yet.
  - Refusals (stderr, exit 1): live → `unable: <name> is already live` / `hint: viola list`; stale → `unable: <name>
    is still running but not answering` / `hint: viola list shows it as stale; stop that process before starting
    <name> again`; both log `process-exit{exit_code:1, detail:"already-live"}` (ERROR). Tampered copy → `unable:
    the pinned viola copy failed its integrity check` / `hint: the pinned copy was changed after it was written, so
    viola will not run it`, `detail:"pinned-hash-mismatch"`. A `gone` name is taken over at once (no 5 s wait) and its
    `events.ndjson` appended to.
  - Env vars set on the child (via `SpawnSpec.env_set`): `VIOLA_NAME`, `VIOLA_DIR` (absolute
    `<home>/instances/<name>`), `VIOLA_BIN` (forward-slash pinned path), `PATH` = pinned dir + inherited PATH (an
    absent or empty inherited PATH → the pinned dir alone); args prefixed `--plugin-dir <abs plugin/<key>>`. R8 strip
    unchanged. No `VIOLA_*` is read by any viola build.
  - Fake agent (`src/bin/viola-fake-agent.rs`): the `start` receipt gains `started_at` (RFC 3339 ms `Z`) and
    `plugin_dir` (the parsed `--plugin-dir`, or `null`).
  - Harness (`viola-e2e`): `logs` gains the events source — each `instances/<name>/events.ndjson` line as
    `{"src":"events","instance","offset","record"}` (offset = the line's first byte), an unparseable or non-object
    line as `{…,"torn":true}` — and `--kind <k>` (matches an event record's `kind`; a diagnostics line never matches
    it). The public `Filter {instance, process}` is kept (a test builds it literally) and a new `Query {instance,
    process, kind}` + `query()` carry `--kind`; `logs()` = `query(…, kind: None)`. `boot` readiness is staged: the
    two `process-start` lines with both processes alive, THEN `snapshot.json` with `data.pid`, `data.started_at`,
    `data.child_pid` and a heartbeat under 5 s (`snapshot_ready`, `beat_fresh`, `fresh_age`, `instance_dir`; new
    missing names `<name>:snapshot`, `<name>:heartbeat`). `run --mutants` sets `AGENT_RUN_KEEP_HOMES=0` and
    `AGENT_RUN_KEEP_FAILED=0` on the `cargo mutants` command (the one runner `ci.yml`'s mutants legs and `pre-push`
    both reach: `ci.yml:173` → `run.rs:182` → `mutants.rs`), overriding `ci.yml`'s workflow-wide
    `AGENT_RUN_KEEP_HOMES=1` there. `pre-push` runs the Linux mutation leg with `TMPDIR=$HOME/viola-pre-push-scratch`
    (beside the clone, on its ext4 disk, outside the copied tree), wiped and recreated 0700 at the cache stage; the
    document's `cache` gains `scratch_bytes` (before the wipe) and `scratch_bytes_after`.
  - Root fixture chain (`tests/support/home.rs`): readiness counts `process-start` self/child and `start` receipts
    newer than before the boot, then the snapshot + fresh heartbeat; `Wrapper::stop_keep`, `Wrapper::instance_dir`,
    `snapshot_data`, `beat_age`; every `TestHome` records `owner.json` `{pid, started_at}` beside its home, and
    `TestHome::new` first runs `sweep_gone_owners` over `target/e2e-home` (only when neither `AGENT_RUN_KEEP_*` is
    set): a dir is removed ONLY when its owner record names a process that is gone (dead pid, or the pid alive with
    another start time); a dir without a readable record is never touched; a kept home drops its record.
- **Crates / modules:** added `viola-state` (sync; listed in `scripts/sync-crates.txt`, now 4 lines: viola-core,
  viola-pty, viola-agent-claude, viola-state; root `[dependencies]` path dep). Workspace crates now: `viola`,
  `viola-core`, `viola-pty`, `viola-agent-claude`, `viola-state`, `viola-e2e` (basis `ls crates`).
- **Dependencies:** no new third-party crate (basis `git diff a892917 -- Cargo.lock`: the only added package is
  `viola-state 0.1.0`). `viola-state` normal deps: viola-core (path), chrono, serde, serde_json, sha2, sysinfo,
  tempfile, thiserror, tracing (all `.workspace`; `tracing` declared per the plan, unused until spans land). sha2
  becomes a product dependency (it was a root dev-dependency only; that dev-dep stays for
  `tests/contract_content_hash.rs`); tempfile becomes a product dependency. Root: `[dev-dependencies] sysinfo`
  (the fixture owner check). Profile: `[profile.dev.package.sha2] opt-level = 3` (dev/test builds hash the 38 MB
  Linux debug exe at release speed; every profile keeps `panic = "unwind"`). atomic-write-file 0.3.1 NOT added
  (BSD-3-Clause, outside `deny.toml`'s allow list — operator ruling at P4).
- **Schema / config:** none changed — `detail` values `already-live` and `pinned-hash-mismatch` already exist in
  `schemas/diag-line.v1.json:80`; no new config key; the plugin JSON bodies above; `events.ndjson` line = the arch
  event line with kinds `wheel` / `budget-gate`, `source:"wrapper"`.
- **Spec-master edits:** none during implement (the seven masters untouched; basis `git diff a892917 --
  .andromeda/*.md` shows only `master-route.md`, the phase's promotion).
- **Counts / qualifiers moved:** workspace crates landed: 5 → 6 (+`viola-state`; basis `ls crates`, root package);
  `scripts/sync-crates.txt` 3 → 4 lines; mutants per leg on this chunk's diff 160 (CI run 36298052174 and local
  pre-push); CI `test (windows-2025)` suite 405 → 464 tests (job logs 108516959970 → 108560461939).
- **Dev-tool versions:** none — cargo-mutants re-read at 27.1.0 in WSL `Ubuntu` (`cargo mutants --version`),
  cargo-nextest 0.9.146, rustc 1.98.1 unchanged.
- **Harness / gate surface:** `viola-harness logs --kind`; the `logs` events source; staged `boot` readiness with
  `<name>:snapshot` / `<name>:heartbeat`; `run --mutants` no-keep override; `pre-push` Linux-leg `TMPDIR` scratch and
  `cache.scratch_bytes` / `cache.scratch_bytes_after`; fixture owner record + sweep. No CI workflow change
  (`ci.yml` untouched).
- **Cross-project / external claims:**
  - CI run 36296402785 on `39b8297` (the pre-CI commit): RED — `mutants (ubuntu-latest)` job 108555954043
    `FAILED Unmutated baseline` (TIMEOUT 10.005 s `run_cli run_never_writes_a_claude_canary_anywhere`, mutants-exit-4)
    and `test (windows-2025)` job 108555954166 FAIL 10.031 s viola-pty
    `tests::spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code` (`lib.rs:1099`); called red by
    the overseer before the windows mutation leg finished.
  - CI run 36298052174 on `ed359cd` (final HEAD): success, 15/15 check-runs (overseer read); both mutation legs base
    `a892917`, 160 tested; each leg missed 3 cfg-sided mutants (a different 3 per leg), each killed on the other leg;
    the union `mutants-verdict` reads `breaches: []`.
  - cargo-mutants 27.1.0 source (WSL cargo registry): `copy_tree.rs:81-84` builds each scratch copy with
    `tempfile::Builder::new().prefix(..).tempdir()` (so `std::env::temp_dir()`, honouring `TMPDIR`); `--help` has
    no scratch-location option; it has `--copy-target` and `--in-place` (reported, not taken).
  - atomic-write-file 0.3.1 licence `BSD-3-Clause` (crates.io API, research).
  - WSL `Ubuntu` `/tmp` is a tmpfs of 16 801 394 688 B (16.8 GB, RAM).
- **Reverted / negative API facts:**
  - A `kind` field on the harness's public `logs::Filter` was written, then replaced by the separate `Query` type
    (`crates/viola-e2e/tests/harness_lifecycle.rs` builds a `Filter` literal).
  - `Heartbeat` shipped at first with a `JoinHandle` and a `Drop` that joined; removed (the join was unobservable
    and survived mutation as `replace <impl Drop for Heartbeat>::drop with ()`).
  - `create_private_dir` at first used a recursive `DirBuilder` plus an explicit set on each created ancestor
    (that set was unobservable: `delete !` survived); replaced by per-component creation.
  - `replace_private` at first filtered an empty parent to `.` (redundant; `delete !` survived); removed.
  - Not adopted: atomic-write-file; registered hooks / MCP servers in the plugin; a `viola hook` no-op verb.
- **Insufficient fixes (written, kept, not the remedy):**
  - `replace_private_shared` (pin copy + plugin files) — **a fix BY REASONING, not a reproduced cause.** Defect:
    `viola-e2e::harness_lifecycle harness_session_boots_reports_logs_and_tears_down` went red ONCE in a gate run
    (the concurrent `overseer`+`builder` boot into one home: `builder` `run-exited`, exit 1); 8/8 reproductions
    green; the red run's `detail-run.ndjson` chain was removed with its home. Reasoning: the only files two
    concurrent starts share are `bin/<key>/viola` and `plugin/<key>/*`, and a Windows `MoveFileExW` replace fails
    while another process holds the target open. What it resolves, measured: a failed replace over a target already
    holding the same bytes now succeeds (Windows read-only stand-in test). Owner of the remainder: any recurrence
    of that test name is matched to this entry (a new red with a captured chain re-opens it).
- **Spec claims disproved by measurement:**
  1. arch §Established Decisions [Snapshot writer] (`architecture.md:50`) and the Stack "State-file primitives" row
     (`:22`), `:435`, `:594`: "atomic-write-file 0.3.1 over tempfile `persist`" — its licence (BSD-3-Clause) is
     outside `deny.toml:16`'s allow list; shipped: tempfile 3.27.0 `persist` through ONE shared helper
     (`replace_private`, mode set on the temp file before the write); security-plan `:197` `:207` `:265` `:266`
     `:380` and obs-plan `:72` `:185` name atomic-write-file the same way (grep `atomic-write-file`: arch 4 ·
     security-plan 5 · obs-plan 2 hits).
  2. arch `:92` / test-plan `:229` / §3 Readiness target: the snapshot "has `endpoint`" — no endpoint is written
     until "Wrapper channel" (the field is optional; the harness readiness checks `pid`/`started_at`/`child_pid`).
  3. The plan's step 3/4 wording "`.lock` created with `open_private_append`" — measured false on Windows
     (`File::lock` → os error 5 on an append-only handle); `.claude/rules/events.md` already states "append +
     exclusive lock fails on Windows", so the rule holds and the plan text was wrong.
  4. The plan's step 5 "the guard drop … joins the thread" — shipped without a join (mutation-unobservable).
  5. test-plan §6 Path 1 (`:1027`): "The first three `events.ndjson` records are `wheel`, `budget-gate`,
     `session-start{source:"hook"}`" — this chunk writes the first two; line 3 waits for "Hooks to normalised events".
- **Expected amendments (from plan):**
  - arch [Snapshot writer] + Stack "State-file primitives" row → tempfile `persist` through one shared helper —
    carried (Spec claims 1; grep `atomic-write-file` arch: 4 hits `:22 :50 :435 :594`).
  - arch Crate dependency direction → `viola-state` → viola-core, chrono, serde, serde_json, sha2, sysinfo,
    tempfile, thiserror, tracing (notify with tailing) — carried (Dependencies; grep `viola-state →` arch `:435`,
    1 hit).
  - arch §Occupied Resources Workspace crates "Landed so far", the rust code-graph plane member list and the licence
    inheritance list → `viola-state` — carried (Crates / modules; grep `Landed so far` arch 1 hit `:353`; plane and
    licence lists located by the doc-agent).
  - arch §Standard Contracts Instance snapshot → `endpoint` absent until "Wrapper channel" — carried (Spec claims 2;
    grep arch `:92`, test-plan `:229`).
  - arch §Standard Contracts → Session liveness → a dead pid + start time is `gone` whatever the beat's age —
    carried (Symbols: `classify`; grep `older than 5 s` arch 1 hit `:310`, test-plan `:210`).
  - security-plan §Data Protection / §Authentication & Authorization / §Bootstrap phases → "atomic-write-file" →
    the tempfile `persist` helper (mode set on the temp file before `persist`) — carried (Spec claims 1;
    security-plan 5 hits).
  - obs-plan §6 `detail` catalog → the `run` stale refusal logs `already-live` — carried (Refusals; grep
    `already-live` obs-plan 6 hits, `:308` names the exit-1 detail list).
  - test-plan §6 Path 1 → line 3 `session-start{source:"hook"}` and the M6 absolute-command witness wait for "Hooks
    to normalised events" — carried (Spec claims 5; grep test-plan `:1027` 1 hit).
- **Coverage of new surfaces:**
  - `viola run` start sequence (collision / pin / plugin / snapshot / heartbeat / start events / spawn env) →
    validation ✓ (`ViolaName` before the path join, `Read::take(MAX_FRAME)` in `read_snapshot`, closed enums) ·
    instrumentation: exit-1 `process-exit` detail codes ✓, spans n/a this chunk (the first spans land with "Wrapper
    channel") · PII n/a (names/codes only; no path or pid in a refusal or `detail`) · tests unit + integ
    (`cli_instance_state`, `tui_env_strip`) · a11y n/a (no viola byte on the terminal; `tui_passthrough` +
    `run_cli` literal-absence checks green) · tokens n/a.
  - `instances/<name>/{events.ndjson, snapshot.json, heartbeat, *.lock}`, `bin/<key>/`, `plugin/<key>/` → validation
    ✓ (owner-only modes on Unix, re-hash) · instrumentation n/a · PII n/a · tests unit + integ · a11y n/a ·
    tokens n/a.
  - `viola-harness logs --kind` / events source → validation ✓ (instance dirs via `ViolaName::try_new`, symlinks
    skipped, 64 MiB read cap) · instrumentation n/a (harness) · PII n/a · tests unit · a11y n/a · tokens n/a.

## Deviations from intent
- Lock siblings opened write-mode via `open_private_lock`, not `open_private_append`: Windows refuses `File::lock` on
  an append-only handle (gate red, measured).
- `viola-state` split into sibling module files (the plan allowed it past 800 lines).
- `EventKind` serialised through `as_str` + `serialize_with` (viola-core has no serde; its manifest was outside the
  lists).
- Plugin tests assert literal strings (viola-agent-claude has no `serde_json` dev-dependency; manifest outside the
  lists).
- `--kind` travels in a new `Query` type; the public `Filter` is unchanged (a `Filter` literal in
  `harness_lifecycle.rs`).
- Harness readiness is staged (snapshot/heartbeat checked after both process starts) so `harness_lifecycle.rs`'s
  two-entry `missing` oracle holds unedited.
- `tests/cli_fake_agent.rs` edited (outside the original lists; it pinned the `start` receipt step 13 grows).
- Empty/absent inherited PATH → the pinned dir alone (was a trailing separator).
- `sha2` `opt-level = 3` in the dev profile, and both canary scans (`tui_env_strip`, `run_cli`) skip a file
  byte-identical to the built `viola`: hashing and scanning the 38 MB Linux debug copy timed tests out (10 s).
- `tracing` declared per the plan, unused.
- Fixtures: `stop_keep`, start counts newer than the boot, owner record + sweep.
- `Heartbeat` without a join; `create_private_dir` per component with an `AlreadyExists` helper;
  `replace_private_shared` (by reasoning, above).
- Research/plan file lists widened by direct edit three times on the overseer's word (option 1: `mutants.rs`,
  `home.rs` sweep, `sysinfo` dev-dep, `cli_fake_agent.rs`; second fold: `pre_push.rs`; CI-red fix: `run_cli.rs`).

## Decisions & corrections
- Operator rulings (P4): tempfile `persist` through ONE helper instead of atomic-write-file; the plugin ships empty
  `hooks` / `mcpServers` until their verbs exist.
- Overseer ruling (option 1): a mutation run keeps no test homes — the kept-home gates (secret scan, artifact upload)
  belong to the test job, not the mutants legs; option 2 (hard-link pinning) would change security's copy design and
  is not decided under disk pressure; option 3 (smaller debug info) only shrinks the symptom.
- Overseer condition: the leftover-home sweep removes a home ONLY when its owning test process is verifiably gone
  (pid + start time), never by age or name; it carries its own remove-the-guard pair.
- Overseer second fold: the pre-push Linux mutation scratch off the RAM tmpfs onto the clone's disk; verify a
  tool's mechanism at the pinned version (its `--help` and source) before relying on it.
- Overseer: a CI-runner timeout fix removes the slow work (the scan of a verbatim binary), never raises the timeout;
  a runner-only red the host cannot reproduce is recorded with history and CARRIED with an owner, not silenced.
- Measured lessons: Windows `File::lock` on an append-only handle → os error 5; nextest's `fail-fast … terminate =
  "immediate"` kills tests without running `Drop`, so their temp homes leak; a per-start copy of the running exe in
  every test home multiplies across mutation runs (≈1.2 GB leaked per mutant before the fix; the 16.8 GB WSL tmpfs
  filled after ~10 of 137); the Linux debug `viola` is 38 MB vs 4 MB on Windows; cargo-mutants 27.1.0 honours
  `TMPDIR` for its scratch copies; pre-push (32-core WSL, no llvm-cov Windows test stage) passed twice on trees CI
  read red; a Windows `MoveFileExW` replace fails on a read-only or held target.
- Host recipes re-hit this session (existing rules): a `boot` stdout captured through a subprocess pipe hung
  (verification-harness.md 2026-09-25); a stop by command-line substring killed the stopping shell
  (host-win32.md 2026-09-25); MSYS rewrote `/usr/bin/…` in a `wsl.exe --exec` call (host-win32.md §Paths).
- Sweep hazard: `grep 'Filter {'` finds harness `Query {` literals too after the split — the public-struct
  consumer sweep needs `Filter {` with a word boundary and the tests dir (`crates/viola-e2e/tests/`).

## Outcome
Acceptance criteria, against the diff:
- (arch) `viola-state` exists, in `sync-crates.txt`, sole-root tokio ban green, `cargo deny check` green with no new
  licence exception — MET.
- (tests) `events_append_keeps_prior_bytes_and_writes_one_line` · `snapshot_write_replaces_atomically_with_the_envelope`
  (+ the Unix 0644→0600 case as `snapshot_write_narrows_an_open_target_to_owner_only`) · `liveness_classify_boundaries`
  (7 cases) + `liveness_same_process_rejects_a_wrong_start_time` · `heartbeat_thread_refreshes_the_beat` — MET (the
  heartbeat test tolerates ONE in-flight touch after the guard drops; a still-running thread fails it).
- (security) `pin_exe_copies_then_refuses_a_tampered_copy`; Unix `fs_private_modes_are_explicit` — MET.
- (arch) `plugin_files_are_the_three_layout_paths`, `plugin_files_render_substitutes_the_pinned_path`,
  `spawn_env_sets_viola_names_and_prefixes_path` — MET.
- (tests/layouts/design/arch/security) `path1_start_writes_state_before_the_spawn`, `run_refuses_a_live_name`, Unix
  `run_refuses_a_stale_name`, `run_takes_over_a_gone_name_and_appends`, `run_refuses_a_tampered_pinned_copy`,
  `run_rewrites_the_plugin_folder_each_start`, `tui_env_viola_names_reach_the_child` — MET.
- (a11y) `tui_passthrough` zero-viola-bytes and `run_cli` literal-absence — MET (green in every suite run).
- (obs) G4 schema-check over the kept homes — MET (875+ files, 0 failures); no `process-*` line carries a path, pid
  in detail or env value.
- (tests) `run --unit`, `run --integration`, `pre-push` (both legs + union: 0 missed, 0 timeout, unviable ≤ caught)
  — MET (union `breaches: []` locally and in CI 36298052174).
- Remove-the-guard pairs, 8, each red then green (`evidence/remove-the-guard.md`): re-hash compare, live refusal,
  stale refusal (Linux, WSL clone), plugin rewrite, 5 s boundary, fixture-sweep owner liveness, mutation no-keep
  override, mutation scratch `TMPDIR`.

Gates (the final /implement runs on the final tree; entries by `run`):
- `cargo fmt --all --check` green · `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`
  green · `test -s scripts/sync-crates.txt && cargo check $(sed …)` green · `cargo deny --config deny-sync.toml
  --manifest-path crates/viola-state/Cargo.toml check bans` green · `cargo deny check` green · `bash
  scripts/deny-probes.sh` green · `bash scripts/orphans-check.sh` green · `run --unit` green · `run --unit --filter
  'package(viola-state)'` green · `run --unit --filter 'test(/plugin_files_|spawn_env_|event_kind_/)'` green ·
  `AGENT_RUN_KEEP_HOMES=1 … run --integration` green · `… --filter 'binary(cli_instance_state) |
  test(/tui_env_viola_names_reach_the_child/)'` green · `schema-check` green · `secret-scan` green · smoke
  `cleanup` / `boot --session p-state-smoke --instance builder` / `status` / `logs … --kind wheel` (contains
  `"cause":"start"`, `"holder":"driver"`) / `cleanup` (`processes_gone:true`) green · MSRV `RUSTUP_TOOLCHAIN=1.96 …
  cargo check` green · `bash scripts/agent-run.sh pre-push` (non-leg) green (union 0 breaches, 160 tested per leg).
- `leg = 'operator'` entries (`evidence/operator-pass.md`): `pre-push` on the uncommitted tree green (1187 s);
  pre-CI commit `39b8297` + guarded push; CI 36296402785 red (both reds dispositioned in
  `evidence/ci-red-36296402785.md`); fix `ed359cd` + `pre-push` green (1163 s) + guarded push; check-runs read on
  `ed359cd` → `success` (run 36298052174, 15/15).
- Smoke: boot path changed; the plan's smoke entries ran as gates, green.
- The viola-pty Windows red on 39b8297 — `red — not this chunk's: viola-pty
  tests::spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code passed on CI for a892917 (job
  108526305228, 0.033 s) and 054ebe4 (job 108516959970, 0.034 s), failed at 10.031 s on 39b8297 (job 108555954166,
  lib.rs:1099), passed 0.022 s on ed359cd (job 108560461939); host 60/60; git diff a892917 HEAD --
  crates/viola-pty/ = 0 lines → CARRY on "Wrapper channel" (owner: its /implement)`; not a red on the final HEAD.

Outcome basis: implement's P4 report as given in this conversation, then three overseer directives (option 1 with
three conditions; the second fold; the operator pass and its CI-red fix) and their artifacts
(`evidence/remove-the-guard.md`, `evidence/pre-push-tmp-peak.md`, `evidence/operator-pass.md`,
`evidence/ci-red-36296402785.md`).

Process hygiene: implement's census — gate test processes and the `p-state-smoke` session terminated; WSL
cargo-mutants / nextest / samplers terminated; rust-analyzer (IDE) stopped before each pre-push and restarted by the
IDE; other `viola.exe` / `claude.exe` (a separate `viola-lab` prototype, Claude Code sessions) not this run's.
Re-measured at this wrap's P1: 0 processes from `D:\dev\projects\viola\target\` on the host; `pgrep` in WSL finds no
viola / cargo-mutants / nextest.
