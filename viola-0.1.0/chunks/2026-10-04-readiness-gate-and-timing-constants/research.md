# Codebase Research — 2026-10-04-readiness-gate-and-timing-constants

## Scope
- **Depth:** deep · **Reads:** 19 · **Globs/Greps:** 24 · **Scratch probes:** 2 (vt100 panic probe, git-maintenance race probe)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` (read in full, 60 lines; 8 Session Additions applied — the
  `TMPDIR` ≤ ~60 B path rule, the never-pipe-`boot` rule, judge-a-local-red-by-pairs) · `.claude/rules/testing.md` (read in
  full, 74 lines; 23 Session Additions applied — the Clock/mock_instant rule, one global per test, proptest format hygiene,
  a guard test's remove-the-guard pair, keep test deadlines below the kill line)
- **Platform issues consulted:** none — no runner-only bullet: CI on `5bcb0a0` re-read green (below); the vt100 panic
  class is reproduced locally, not a runner observation.
- **Code graph:** plane rust, `db_state: fresh` (venv `~/.local/viola-venv`; system python has no duckdb), 1 query, 52 rows
  — trace `.andromeda/runs/2026-10-04T04-28-47-phase/tree-query-2026-10-04-readiness-gate-and-timing-constants.json`.

## Files inspected
- `crates/viola-pty/src/pump.rs` (1-175) — `pump(pty, input, output: Box<dyn Write + Send>, spawned, host_size)`: the
  output thread `copy`s 8 KiB reads straight into `output` (`write_all` + `flush`, :41-53); both workers run under
  `catch_unwind` and a worker panic kills the child (`PumpEnd::WorkerPanicked`, :96). It knows no screen.
- `src/cmd/run.rs` (60-149, 405-439) — `pump_child` passes `Box::new(io::stdout())` as `output` (:419-425); the channel
  `Methods::dispatch` serves only `hook.event` (:100-110), so no `send` method exists.
- `src/run/mod.rs` (full) — role lines, `child_launch`, `resolve_program`; no gate, no screen code.
- `src/main.rs` (40-254) — `viola_panic_hook` writes the codes-only `event:"panic"` role line and the detail line
  (payload + raw frames) for EVERY panic, before any catch site (`:138-181`); nothing distinguishes a caught one.
- `crates/viola-core/src/lib.rs` (full) — `MAX_FRAME`, `ViolaName`, `EventKind`; deps nutype only. No `Clock`, no
  `RefusalReason`, no refusal-detail type anywhere in the workspace (`grep -rn 'RefusalReason|input-not-ready|NotDelivered'
  src crates tests`: 1 hit, a schema-case string in `tests/contract_diag_schema.rs:174`).
- `crates/viola-agent-claude/src/ledger.rs` (1-60, 160-291) — closed `LedgerRow` (6, `ALL`), `check(row, &ProbeRun)` over
  print-mode captures, `merge_stamp` writing `rows` + `measured.largest_hook_payload`, `verified` = every row of `ALL`
  `"pass"` (:281-291).
- `crates/viola-agent-claude/Cargo.toml` — deps viola-core, serde, serde_json, serde_path_to_error, thiserror (no tracing).
- `src/cmd/verify.rs` (grep map) — one print-mode probe through `run_bounded` (std `Command`, pipes, `:78`), no PTY;
  `check_rows` takes `total = LedgerRow::ALL.len()` (:265), so `[NN/MM]` follows the row set; `step_line` :281-290.
- `src/cmd/hook.rs:40` — `const SPINE_DEADLINE: Duration = Duration::from_millis(750)` (crate-private).
- `crates/viola-e2e/src/harness/gate.rs:36` — `pub const SPINE_DEADLINE_S: f64 = 1.0` (the perf gate's literal, :155-159).
- `crates/viola-e2e/Cargo.toml` — deps viola-core, viola-pty, viola-channel (not viola-agent-claude).
- `crates/viola-state/src/heartbeat.rs:13` (`BEAT_EVERY = 1 s`), `liveness.rs:22` (`classify(beat_age, same_process)`
  — fed ages, no clock).
- `src/bin/viola-fake-agent.rs` (1-80, grep) — modes `--print`, scripted turns, `--suppress-prompt-submit`,
  `--local-command-mode`, `--inject-harness-turn`, `--exit-no-eof`, `--report-version`; interactive mode reads stdin and
  writes NO screen bytes (stdout carries only `--version` and print mode's reply, :1-4).
- `fuzz/Cargo.toml` — `viola-fuzz`, path deps on viola-core / viola-channel / viola-agent-claude, 3 targets; corpus
  `fuzz/corpus/hook_stdin` holds 10 seeds.
- `schemas/diag-line.v1.json:102-103,129` — `parse-rejected.parser` already lists `vt100-feed`, `detail` already lists
  `panicked`; `subject` enum `self · claude-child · version-probe · verify-probe · agents-probe · statusline-shell`.
- `scripts/g2-zero-panics.sh` (1-40) — counts every non-exempt `event:"panic"` in role files under `target/e2e-home`;
  the one exemption is `panic_location` exactly `src/cmd/hook/seam.rs:<digits>`.
- `tests/support/home.rs:104-117` — root test homes live under `target/e2e-home/viola-test-*` (inside G2's scope).
- `crates/viola-e2e/src/harness/run.rs:509-531` (`git_repo` / `init_repo`) and `…/mutants/base.rs:228-273` (`Pass`,
  `commit` :248-268) — the two throwaway-repo git writers; 6 `Command::new("git")` sites across the two files
  (`grep -rn 'Command::new("git")' crates/viola-e2e/src`).
- `viola-0.1.0/chunks/2026-10-04-windows-boundary-mutation-workflow/evidence/leak.md:74-101` — the 21 leftovers' content.
- vt100 0.16.2 and its graph in `~/.cargo/registry` (manifests, CHANGELOG, `src/`).

## Measured facts
- **M1 — vt100 0.16.2's graph** (manifest read): `vt100` MIT, rust-version 1.70, deps `itoa` 1.0.15 · `unicode-width`
  0.2 · `vte` 0.15.0 (default features = `std` only; `vte` Apache-2.0 OR MIT, 1.62.1, deps `arrayvec` 0.7 (no default
  features) · `memchr` 2.7). `itoa`/`unicode-width`/`arrayvec`: MIT OR Apache-2.0, `build = false`. No `build.rs` in the
  graph; `grep -rnE 'env::var|std::env' vt100-0.16.2/src` → 0. MSRVs all below the 1.96 floor. None of vt100 · vte ·
  unicode-width · arrayvec is in `Cargo.lock` today (`grep -nE 'vte|vt100' Cargo.lock` → 0); itoa and memchr already are.
  0.16.0 stopped logging unhandled escapes to STDERR (CHANGELOG) — 0.15.x would write to `run`'s terminal. The
  `cargo deny` verdict itself is implement's to read.
- **M2 — vt100 0.16.2 panics on real bytes, at small sizes** (scratch probe `scratchpad/vtfuzz`, dev profile with
  overflow checks and release, identical results):
  - random inputs at random sizes 1–200 cols with resizes: 21 937 panics in 64 546 runs (dev), 23 372 / 68 773 (release);
  - fixed 24x80, no resize, inputs shrunk greedily: 0 panics in 54 025 (release) + 37 897 (dev) runs;
  - deterministic cases: `Parser::new(24, 1, 0)` + `e4 b8 ad` (U+4E2D, a wide char) → panic; `(1, 1)` + `?u` → panic;
    `(1, 2)` + `abc` → panic; `(24, 2)` + U+4E2D, `(2, 2)` + `abc`, `(24, 80)` + a wide char at col 80, `(24, 80)` +
    `CSI 999;999H` + two wide chars, `(1, 80)` + newlines → no panic.
  So a forced panic needs no new env seam: bytes the child writes, at a PTY size the test chooses (24x1 + a wide char),
  panic the feed. It also means a human with a 1-column or 1-row terminal panics the real feed.
- **M3 — a caught panic still writes a G2-counted line**: `viola_panic_hook` runs before `catch_unwind` returns and
  writes `event:"panic"` + a detail line with `panic_payload` and raw frames (`src/main.rs:138-181`, 23.7 ms per capture
  per obs history). A feed panic under a root E2E home therefore reads red on G2 (location `vt100-0.16.2/src/…`), and a
  feed that panicked on every chunk would write one panic + detail line per 8 KiB read.
- **M4 — signature rows cannot pass against today's fake agent**: `verified` needs every `LedgerRow::ALL` row `"pass"`
  (ledger.rs:290), CI and every root/harness home stamp against the fake agent at 2.1.283, and the fake agent renders no
  screen. A new screen-signature row in `ALL` reads `fail` on every stamped home until the fake agent replays a recorded
  input-box screen — and the only recording source is a live `claude` (no 2.1.283 or 2.1.287 screen recording exists:
  `fixtures/claude/2.1.283/` holds hook payloads only).
- **M5 — CARRY 4 leftovers point at a second git process**: the half-removed repos hold `objects/pack/tmp_idx_*`,
  `objects/pack/tmp_rev_*`, `rr-cache/`, `MERGE_RR` and no `HEAD` (leak.md:79-81). `add`/`commit` write none of those;
  pack-objects (repack/gc) and `rerere gc` do. So a gc-class git process wrote into the tree after the fixture's own
  calls; the detached `git maintenance run --auto` is the only such process measured to spawn (GIT_TRACE, git 2.55.0,
  `git --version` here: 2.55.0). The race with the `TempDir` drop is NOT reproduced: scratch `gitrace.py`, 150
  init+4-commit repos removed at once per arm, on tmpfs and on the NOCOW btrfs scratch, default and
  `-c maintenance.auto=false`: 0 / 0 / 0 / 0 left (an idle host; the measured 21 came from a loaded 78-min run).
- **M6 — CI since the last wrap**: `5bcb0a0cc584` re-read at P3 (`ci.py conclusion`): **`verdict: green` · checks 15/15 ·
  wall 280 s** · ci#37177029795 completed/success.

## Graph impact
- **`pump`** (viola-pty) — 1 production caller: `cmd/run/pump_child()` @ `src/cmd/run.rs:419`; 11 in-crate tests. A tee
  around the `output` writer in the root bin leaves `pump`'s signature and viola-pty untouched.
- **`check` / `merge_stamp`** (ledger) — production callers only `cmd/verify/check_step()` @ `src/cmd/verify.rs:275` and
  `cmd/verify/measure()` @ `src/cmd/verify.rs:182`; the rest are ledger tests (:669-881).
- **`verified`** — production caller `run/version_gate/stamps_verdict()` @ `src/run/version_gate.rs:169`; tests :179-183.
  Growing `LedgerRow::ALL` changes this verdict for every stamped home (M4).

## Patterns detected
- **Pure verdict in agent-claude, I/O in the root bin** (`ledger::verified` ← `run::version_gate::stamps_verdict`,
  version_gate.rs:169): the screen model + signature matching go pure in `viola-agent-claude`, the feed thread, clock loop
  and `catch_unwind` in `src/run/`.
- **Worker bodies under `catch_unwind`** (pump.rs:29-39): the precedent shape for the feed's own catch.
- **Named timing constants** (`pump.rs:11-15` `TICK`/`RESIZE_EVERY`/`DRAIN_WITHIN`/`KILL_WAIT`; heartbeat.rs:13
  `BEAT_EVERY`; hook.rs:40 `SPINE_DEADLINE`).
- **Step counter from the row set** (verify.rs:265, :281): `[NN/MM]` grows with `LedgerRow::ALL` without a literal.
- **Fixed-size proptest config** (viola-core lib.rs:80-88: `cases: 512`, `FileFailurePersistence::SourceParallel`).

## Conventions to follow
- **Logs only through `obs_event!`, spans `#[instrument(skip_all, name = …)]`** (`src/run/mod.rs:18-86`).
- **Throwaway git calls carry their config with `-c`** (`run.rs:521`, `base.rs:259`: `-c user.name=t -c user.email=…`) —
  `-c maintenance.auto=false` joins the same arg list; no `GIT_*` env assignment (pre-push's `env -i` rule).
- **viola-e2e has no dev-dependencies**: labelled case tables, not rstest (test-plan history).

## New files to create
- `crates/viola-agent-claude/src/screen.rs` — the screen model over vt100 (feed, quiet tracking from fed instants,
  signature verdict), pure
- `src/run/gate.rs` — the output tee, the feed thread under `catch_unwind`, the gate verdict and its span
- `fuzz/fuzz_targets/vt100_feed.rs` — the vt100 feed fuzz target
- `fuzz/corpus/vt100_feed/` — its synthetic seed corpus

## Files to modify
- `Cargo.toml` — `vt100 = "=0.16.2"` in `[workspace.dependencies]`
- `Cargo.lock` — vt100, vte, unicode-width 0.2, arrayvec
- `crates/viola-agent-claude/Cargo.toml` — the vt100 dependency
- `crates/viola-agent-claude/src/lib.rs` — `mod screen`, its exports, the gate constants
- `crates/viola-core/src/lib.rs` — the `Clock` seam and the shared spine-deadline constant
- `src/cmd/run.rs` — `pump_child` hands the pump the tee instead of bare stdout
- `src/run/mod.rs` — `mod gate`
- `src/cmd/hook.rs` — `SPINE_DEADLINE` from the shared constant
- `crates/viola-e2e/src/harness/gate.rs` — `SPINE_DEADLINE_S` from the shared constant
- `crates/viola-e2e/src/harness/run.rs` — `-c maintenance.auto=false` in `init_repo`
- `crates/viola-e2e/src/harness/run/mutants/base.rs` — `-c maintenance.auto=false` in `Pass::commit`
- `fuzz/Cargo.toml` — the `vt100_feed` target
- `fuzz/Cargo.lock` — vt100's graph in the fuzz workspace
<!-- P4 forks decided (overseer, 2026-10-04): "Mechanism now, rows held" and "In-process witness" — so ledger.rs,
verify.rs, the fake agent, tests/cli_verify.rs, tests/support/verify.rs, the --local-live row set, src/main.rs and
scripts/g2-zero-panics.sh are NOT touched; a proptest-regressions file appears only when a property fails. -->


## Open questions
- The signature rows (CARRY 1) need a recorded real-CLI screen before any fake-agent home can stamp them `pass` (M4):
  record live on this host against 2.1.287 now, or land the gate mechanism with the rows' live measurement deferred to
  `:82` → blocks: plan-decision
- A forced feed panic under a root E2E home writes a G2-counted panic line (M3): keep the forced-panic witness
  in-process only (no G2 exposure; the run-level chaos case waits for `:74`), or change the panic hook so a feed-contained
  panic writes `parse-rejected` instead of `panic` (an obs §7 contract change), or add a second G2 exemption → blocks:
  plan-decision
- After a feed panic the model is unusable; whether the feed stops until the next resize (bounding panic lines to one per
  size) or re-creates the model per turn → blocks: plan-decision
