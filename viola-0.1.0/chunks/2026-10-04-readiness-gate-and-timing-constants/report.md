# Report — 2026-10-04-readiness-gate-and-timing-constants

**Chunk:** Readiness gate and timing constants — vt100 screen model under catch_unwind on run's pump, the gate's timing
constants named, injected clock, parser-panic degrade; signature rows and the typed-input probe HELD
**Date:** 2026-10-04T05:30Z
**Commits:** `afef92f chore(2026-10-04-readiness-gate-and-timing-constants): operator pre-CI commit, for the run this chunk's verdict reads` (the only commit since `last_wrap` 2026-10-04T04:24:24Z; base `5bcb0a0`)

## Changes (structured — detectors read this)
- **Files:** (basis: `git diff --name-only 5bcb0a0` + untracked, `gate.py scope` 22 changed · 22 listed)
  - new: `crates/viola-agent-claude/src/screen.rs` · `src/run/gate.rs` · `fuzz/fuzz_targets/vt100_feed.rs` ·
    `fuzz/corpus/vt100_feed/` (6 files: `wide-one-col`, `csi-u-one-by-one`, `abc-one-by-two`, `sgr-cup-24x80`,
    `osc-title-bel-24x80`, `wide-line-24x80`)
  - modified: `Cargo.toml` · `Cargo.lock` · `crates/viola-agent-claude/Cargo.toml` · `crates/viola-agent-claude/src/lib.rs` ·
    `crates/viola-core/src/lib.rs` · `src/cmd/run.rs` · `src/run/mod.rs` · `src/cmd/hook.rs` ·
    `crates/viola-e2e/src/harness/gate.rs` · `crates/viola-e2e/src/harness/run.rs` ·
    `crates/viola-e2e/src/harness/run/mutants/base.rs` · `fuzz/Cargo.toml` · `fuzz/Cargo.lock`
  - chunk evidence: `evidence/{fuzz-override,guards,carry4,operator-pass}.md`
- **Symbols / APIs:**
  - `viola_core::SPINE_DEADLINE: Duration = 1 s` (new pub const) — the bound every spine hook process meets; read by
    the perf gate `crates/viola-e2e/src/harness/gate.rs` `perf()` (`let bound = viola_core::SPINE_DEADLINE.as_secs_f64()`),
    which replaces the removed `pub const SPINE_DEADLINE_S: f64 = 1.0` (its only readers were gate.rs :155/:159 —
    `grep -rn SPINE_DEADLINE_S crates src tests` → 0 after). Failure text keeps `{file} max {max} >= {bound}` (`max 1 >= 1`).
  - `viola_core::Clock` (new pub trait, `Send + Sync`, `fn now(&self) -> Instant`) + `viola_core::SystemClock` (unit
    struct, `Instant::now()`); std only, no dependency. Production consumer: `src/run/gate.rs::start` (via
    `src/cmd/run.rs pump_child`, `SystemClock`). No other caller; the heartbeat was NOT migrated.
  - `src/cmd/hook.rs`: crate-private `SPINE_DEADLINE` (750 ms) RENAMED `CONNECT_DEADLINE` (same value, same single use
    in `spine_deadline()`); test `connect_deadline_is_below_the_spine_deadline` asserts `CONNECT_DEADLINE < viola_core::SPINE_DEADLINE`.
  - `viola_agent_claude::screen` (new pub module): `QUIET_PERIOD = 300 ms` · `GATE_MAX_WAIT = 5 s` ·
    `CONFIRM_WINDOW_FALLBACK = 10 s` (pub consts, each one home, doc'd PROVISIONAL — built-ins, not measured) ·
    `Signatures { input_box: &'static [&'static str], modals: &'static [&'static str] }` (compiled data; NO production
    instance — no per-version table, the rows are held) · `Readiness { Ready, InputNotReady }` with `as_str` →
    `ready` / `input-not-ready` · `GateStep { Wait, Done(Readiness) }` · `Screen { new, feed, resize, poison,
    is_poisoned, size, verdict }`. `verdict(sigs, waiting_since, now)` order: poisoned → `Done(InputNotReady)`; not quiet
    (`now - last_fed < QUIET_PERIOD`) and waited `>= GATE_MAX_WAIT` → `Done(InputNotReady)`; not quiet → `Wait`; quiet →
    `Ready` iff a row holds an `input_box` literal and no row holds a `modals` literal. Pure: no I/O, no clock read, no
    row text returned/stored/logged. `verdict` has NO production caller yet (`:74` Confirmed send is its consumer).
  - `src/run/gate.rs` (crate-private): `Feed { Bytes(Vec<u8>), Size(Size) }` · `Tee<W>` (`Write`: writes to the output
    first, then sends a copy of exactly the bytes written; a send failure is ignored; `flush` flushes the output) ·
    `start(clock, size) -> (Sender<Feed>, JoinHandle<()>)` (the feed thread owns a `Screen`; bytes → `feed`, a size
    different from the current one → `resize`, each under `catch_unwind`; a caught panic → `poison()` + one
    `obs_event!(WARN, ParseRejected, parser="vt100-feed", detail="panicked", count=1)`, no `corr`; ends when every sender
    drops). The channel is `std::sync::mpsc::channel()` — UNBOUNDED (as the plan specified; see Decisions).
  - `src/cmd/run.rs pump_child` (the `viola_pty::pump` sole production caller, unchanged): starts the gate at the
    spawned size with `SystemClock`, passes `Box::new(Tee::new(io::stdout(), feed))` as `output` and a `host_size`
    closure that reads `viola_pty::host_size()` and sends each reading to the feed. The feed thread's handle is dropped
    (detached). `viola_pty::pump`'s signature and viola-pty are unchanged.
  - `crates/viola-e2e/src/harness/run.rs` `test_support`: new `const GIT_CONFIG: [&str; 6]` (`-c user.name=t -c
    user.email=t@example.com -c maintenance.auto=false`) used by all three throwaway-repo git call sites (`init_repo`,
    and `mini()`'s `add -A` + `commit`); `…/run/mutants/base.rs` `Pass::commit` gains `-c maintenance.auto=false`. Test
    code only; no `GIT_*` env assignment.
  - Env vars: none added (`! (git diff 5bcb0a0 -- '*.rs' | grep '^\+.*std::env::var')` green).
- **Crates / modules:** new module `viola-agent-claude::screen` (pub), new root-bin module `run::gate` (`pub(crate) mod gate`
  in `src/run/mod.rs`); new fuzz `[[bin]]` `vt100_feed` in `fuzz/Cargo.toml`. No crate added or removed.
- **Dependencies:** added `vt100 = "=0.16.2"` to `[workspace.dependencies]` (default features) and to
  `crates/viola-agent-claude/Cargo.toml` ONLY (`grep -rl --include=Cargo.toml vt100 Cargo.toml crates src | wc -l` → 2).
  Lockfile additions (root and fuzz, +34 lines each, 0 deletions): `vt100 0.16.2` (MIT) · `vte 0.15.0` (Apache-2.0 OR MIT)
  · `unicode-width 0.2.2` · `arrayvec 0.7.8` (`itoa` 1.0.18 and `memchr` 2.8.3 already present). `cargo deny check`,
  the per-sync-crate tokio ban (viola-agent-claude is on `scripts/sync-crates.txt`), `scripts/deny-probes.sh` and
  `cargo deny --manifest-path fuzz/Cargo.toml check advisories sources` all green; no allow / skip / ignore / exception
  added (`git diff 5bcb0a0 -- Cargo.toml deny.toml deny-sync.toml | grep -cE '^\+.*(skip|ignore|exceptions|allow *=)'` → 0).
- **Schema / config:** none — `schemas/diag-line.v1.json` already lists `parse-rejected.parser` `vt100-feed` and `detail`
  `panicked` (unchanged: the `git diff --quiet 5bcb0a0 -- … schemas/diag-line.v1.json` gate green). No `config.json` key.
- **Spec-master edits:** none (implement wrote no master).
- **Counts / qualifiers moved:** `run --fuzz-replay` targets 3 → 4 (`viola_name`, `channel_frame`, `hook_stdin`,
  `vt100_feed`; gate atom `"passed":4` green). `fuzz/fuzz_targets/` 3 → 4 files. Proptest properties in
  viola-agent-claude: +1 (`screen::tests::feed_prop_a_caught_panic_poisons_until_resize`, `cases: 512`). Unit tests
  (nextest-unit 759 total at implement; screen 21, run::gate 3, viola-core +2, hook +1). Masters stating these: test-plan
  §6 cargo-fuzz paragraph (the target list), architecture `fuzz/` line :413 (basis: `grep -n fuzz_targets` /
  `grep -c vt100` per master above).
- **Dev-tool versions:** none — cargo-fuzz 0.13.2 and the `nightly-2026-09-20` fuzz toolchain re-read unchanged; git
  2.55.0 re-read.
- **Harness / gate surface:** `run --fuzz-replay` replays a 4th target (no harness code change — targets are read from
  `fuzz/Cargo.toml`); the perf gate's bound now comes from `viola_core::SPINE_DEADLINE` (same 1.0 s, same breach text).
  Throwaway fixture repos run with `maintenance.auto=false`.
- **Cross-project / external claims:**
  - CI ci#37179459192 on `afef92f8885f` (the pre-CI commit): `verdict: green · checks 15/15 · wall 364 s`; every job
    `success`; `test (windows-2025)` / `(macos-latest)` / `(ubuntu-latest)` each ran 5 `tui_passthrough` PASS / 0 FAIL,
    3 `run::gate::tests`, 21 `screen::tests` (job logs read through `gh run view --job`; evidence/operator-pass.md).
  - libfuzzer-sys 0.4.13 (cargo registry source): `initialize` installs a panic hook that aborts on every panic
    (`src/lib.rs:83-94`); the target body runs under the crate's own `catch_unwind` and aborts on `Err` (`:60-70`).
  - vt100 0.16.2 (registry source): `Grid::new` computes `size.rows - 1` (`src/grid.rs:26`), so a 0-row parser
    overflows; `viola_pty::host_size` already filters zero sizes (`nonzero_size`), so no zero reaches the gate.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - plan.md step 10 / research M3 premise "libfuzzer-sys … source is not on this host" — now measured: the source IS in
    the registry and CONFIRMS the premise's substance (abort hook + catch-and-abort). Nothing in a master states the
    absence; disposition: report-only (evidence/fuzz-override.md).
  - CARRY 4's `[inferred]` hypothesis (scope.md §5: the detached `git maintenance run --auto` races the `TempDir`
    drop) — research M5 had "the race itself did not reproduce on an idle host (0 of 300)"; the implement witness
    REPRODUCED it on an idle host through the `Pass` tests: control 4 and 5 half-removed per 200 rounds, fix 0 and 0
    (evidence/carry4.md). The hypothesis moves from `[inferred]` to supported two-sided. Body site (basis: `grep -n
    "half-removed\|maintenance" test-plan.md registries/contracts/test-plan/*.md` → 1 body hit):
    `registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:15` ("21 are half-removed
    throwaway git repos, a `terminate`-independent class") — still true as a count, now with a measured cause and fix
    (fixture repos run with `maintenance.auto=false`). `test-plan-amendments.md:429` is history (not edited).
- **Expected amendments (from plan):** (search basis per line; `grep -c`/`grep -n` over `.andromeda/*.md` and
  `registries/contracts/**`)
  - architecture [Screen Model] (`grep -n '\[Screen Model\]' architecture.md` → :48, 1 hit) — CARRIED: the gate
    mechanism as landed (pure model in viola-agent-claude, tee + feed thread in `src/run/gate.rs`, poisoned until a size
    change), `QUIET_PERIOD` 300 ms / `GATE_MAX_WAIT` 5 s PROVISIONAL built-ins, the signature format (two compiled
    `&'static [&'static str]` literal lists, row-contains matching) PROVISIONAL, signature/timing ledger rows HELD → owed
    to `:82` or the founder's ruling (Symbols bullet).
  - architecture [Delivery Confirmation] (:49, 1 hit) — CARRIED: `CONFIRM_WINDOW_FALLBACK` = 10 s compiled in
    viola-agent-claude, PROVISIONAL.
  - architecture [Hook Transport] (:62-64; `SPINE_DEADLINE` 1 hit in architecture.md) — CARRIED: `viola_core::SPINE_DEADLINE`
    (1.0 s) named; the hook's 750 ms is `CONNECT_DEADLINE`, asserted below it.
  - architecture Project directory structure — the keyed contract `registries/contracts/architecture/project-directory-structure.md`
    (`grep -ln "src/cmd/\|fuzz_targets" registries/contracts/*/*.md` → this file) — CARRIED: `src/run/gate.rs`,
    `crates/viola-agent-claude/src/screen.rs`, `fuzz/fuzz_targets/vt100_feed.rs`, `fuzz/corpus/vt100_feed/` (Files bullet).
    architecture.md:413's `fuzz/` line names the targets too (check its list).
  - test-plan §10 Performance budgets — Spine deadline (`grep -n 'Spine deadline' test-plan.md` → :1220, 1 hit) —
    CARRIED: the perf gate reads `viola_core::SPINE_DEADLINE`; the "once arch request (3) lands" / provisional-literal
    sentence retires.
  - test-plan §7 Fake agent (`grep -n vt100-panic-bytes test-plan.md` → 1 hit; :1076 Modes line) — CARRIED as a
    not-built fact: `--vt100-panic-bytes` still not built, lands with `:74`'s chaos case; its bytes are now measured
    (24×1 + a wide char, e.g. `e4 b8 ad`).
  - test-plan §6 Property suite (:1019) and cargo-fuzz paragraph — CARRIED: the vt100 feed property (512 cases) and the
    `vt100_feed` target with its silent-panic-hook override + remove-the-guard pair (Counts bullet).
  - obs-plan §7 Error classes captured (`grep -n 'Error classes captured' obs-plan.md` → :912) — CARRIED: a contained
    feed panic writes the `parse-rejected` line AND (in a process with `viola_panic_hook` installed, `src/main.rs:138`)
    the hook's `event:"panic"` + detail line pair, once per poisoning (research M3; unchanged code); the G2 question is
    `:74`'s.
  - security-plan §Input Validation — PTY output row (`grep -n 'PTY output' security-plan.md` → :238) — CARRIED: a vt100
    panic is reachable at real small sizes (24×1 + a wide char; 1×1 `?u`; 1×2 `abc` — research M2, re-witnessed by
    `screen::tests::feed_wide_char_at_one_column_panics`), and the degrade is poisoned-until-the-size-changes with one
    `parse-rejected` line per poisoning.
  - v1-21 not claimed — not carried: nothing for wrap to write (its P5 note exists).
- **Coverage of new surfaces:**
  - `Tee` on `run`'s pump output (PTY output → human terminal) → validation n/a (bytes passed verbatim, never altered) ·
    instrumentation n/a (no per-chunk span/event by design, obs §11) · PII n/a (no screen content leaves) · tests
    unit (`run::gate::tests` ×2) + integ (`tui_passthrough` ×5, three CI OSes) · a11y n/a (passthrough byte-identical,
    a11y-plan §1) · tokens n/a
  - vt100 feed thread (PTY output bytes → screen model) → validation catch_unwind + poison✓ · instrumentation
    `parse-rejected` WARN on poisoning✓ · PII n/a (no row text logged) · tests unit + property (512) + fuzz target ·
    a11y n/a · tokens n/a
  - `Screen::verdict` → validation closed enums✓ · instrumentation n/a here (the `run.readiness_gate` span is `:74`'s)
    · PII n/a · tests unit (boundary tables) · a11y n/a · tokens n/a
  - `viola_core::Clock` / `SPINE_DEADLINE` → n/a · tests unit ✓

## Deviations from intent
- Step 11 named `init_repo` and `Pass::commit`; `run.rs` `test_support::mini()` holds two more throwaway-repo git
  calls (`add -A`, `commit`) in the same intent ("no `git commit` in a throwaway repo spawns `git maintenance`"). All
  three `run.rs` sites now share one `GIT_CONFIG` literal, so the plan's probe (`run.rs:1`) still reads 1. Listed file.
- Step 6: `gate::start` returns `(Sender<Feed>, JoinHandle<()>)`, not the sender alone — the witness tests join the
  thread to synchronise without sleeps; production drops the handle.
- Step 5: `Screen::size()` added (the feed thread compares a size message with the current size).
- Step 12 (CARRY 4) ran at /implement rather than the operator pass, on the plan's economy clause (the overseer
  later confirmed: "the cheaper CARRY 4 witness was the right call"): the `Pass` fixture's 14 tests × 200 rounds per
  side in fresh short NOCOW `TMPDIR`s, two copies of the tree (control = step 11 reverted) — not the two 78-min mutation
  runs. Pass: control 4 and 5, fix 0 and 0, reverse-order pairs.
- scope record: none — `gate.py scope` clean (changed 22 · listed 22 · recorded 0), at /implement P4 and at this wrap's P1.

## Decisions & corrections
- Overseer pre-directions for this wrap: (1) route the unbounded tee→feed `mpsc` (`src/run/gate.rs` `start`) to `:74`
  as a named "bound every input" item; (2) the held widening (the PTY typed-input probe, the live 2.1.287 recording, the
  signature/timing ledger rows) stays HELD — owed to `:82` or the founder's morning ruling; nothing written toward it;
  (3) 300 ms / 5 s / 10 s and the signature format are recorded PROVISIONAL. Any other widening is held.
- Overseer at the implement report: the economy CARRY 4 witness was the right call.
- Measured: `$CLAUDE_SCRATCH` is not set in the Bash tool's shell — a redirect to it wrote to `/chk.log` and was denied;
  use the literal scratchpad path.
- Measured: the Bash permission guard refuses a read after `cd` into the cargo registry when the relative file name
  matches a deny glob (`./secrets/**` matched `parser.rs`'s relative read); absolute paths pass.
- Measured: the fixture half-removal reproduces on an IDLE host at ~2–2.5 % of `Pass`-test rounds (4/200, 5/200) —
  research M5's idle 0/300 scratch probe (`gitrace.py`, init + 4 commits) did not reach it; the fixture's own tests do.
- Sweep hazard: a plan's named call sites for a config arg were not every site — `grep -n 'user.email=t@example.com'`
  over the two files found 4 sites, not 2.

## Outcome
- Acceptance (re-asserted against the diff):
  - (arch) vt100 only in the workspace table + viola-agent-claude (probe → 2); viola-agent-claude gains no I/O
    (screen.rs: no `std::fs`/`io`/`thread`/clock read — every instant a parameter); deny suite green, no new
    exception — MET.
  - (arch/security) the three timing constants each one home in viola-agent-claude, no config key; signature lists
    compiled, no path from screen/upstream text — MET.
  - (arch/tests) `SPINE_DEADLINE` is the one spine bound; perf gate has no `1.0` literal (probe exit 1, last line 0);
    `CONNECT_DEADLINE` asserted below it — MET.
  - (tests) verdict asserted only as ready / input-not-ready / wait; boundaries at exactly 300 ms / 5 000 ms and 1 ms
    before, from injected instants; 512-case property — MET.
  - (security/obs) caught panic poisons; exactly one `parse-rejected{vt100-feed, panicked, count 1}` WARN, no `corr`;
    no further line for 8 KiB more; one more after a size change; process continues — MET (`run::gate::tests`, guard
    pair evidence/guards.md).
  - (design/layouts/a11y) tee byte-identical (incl. panicking bytes, short writes); `tui_passthrough` green with the
    feed active on all three CI OSes (Windows on windows-2025) — MET (ci#37179459192).
  - (tests) `run --fuzz-replay` replays 4 targets, `vt100_feed` corpus non-empty synthetic; remove-the-guard pair
    recorded — MET (evidence/fuzz-override.md).
  - (tests) CARRY 4: fix 0, control ≥ 1, fresh short NOCOW `TMPDIR`s — MET on the economy witness (evidence/carry4.md).
  - (obs) no line/span/detail carries screen bytes, row text or signature text; no per-chunk span/event — MET.
  - (tests) operator CI read on the final HEAD `verdict: green`, run id named — MET (ci#37179459192 on `afef92f`).
- Gates (/implement, run dir `.andromeda/runs/2026-10-04T04-59-18-implement/`, two full runs, both 22 green · 0 red ·
  3 operator legs):
  - `cargo fmt --all --check` green · `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`
    green · `run --unit --filter 'test(/screen::tests::/)'` green (21) · `run --unit --filter 'test(/run::gate::tests::/)'`
    green (3) · `run --unit` green · `run` green (unit 759 · integration 230) · `cleanup --session p-rg-smoke` green ·
    `boot --session p-rg-smoke --instance builder` green · `status --session p-rg-smoke` green (`state:ready`) ·
    `cleanup --session p-rg-smoke` green (`processes_gone`, `endpoint_gone`) · `run --fuzz-replay` green (`passed:4`) ·
    `cargo deny check` green · the per-sync-crate tokio ban loop green · `scripts/deny-probes.sh` green · the fuzz
    lockfile audit green · `grep -rl … vt100 … | wc -l` green (2) · `git diff 5bcb0a0cc584 -- Cargo.toml deny.toml
    deny-sync.toml | grep -cE …` green (exit 1, 0) · `grep -c 'pub const SPINE_DEADLINE_S: f64 = 1.0' …` green (exit 1,
    0) · `grep -c 'maintenance.auto=false' …` green (run.rs:1, base.rs:1) · `git diff --quiet 5bcb0a0cc584 -- ledger.rs
    verify.rs viola-fake-agent.rs main.rs g2-zero-panics.sh diag-line.v1.json` green · `! (git diff … | grep -E
    '^\+.*(#\[ignore|retries *=|test\.skip|std::env::var)')` green · `bash scripts/agent-run.sh pre-push` green
    (`ok:true`, stage `linux-tests`: coverage 989 · playwright 1 · gate breaches []).
  - `gate.py hygiene` — leg operator: `hygiene: clean`, exit 0 (evidence/operator-pass.md).
  - `git diff --quiet && git diff --cached --quiet && git push origin HEAD` — leg operator: pushed `5bcb0a0..afef92f`.
  - `ci.py conclusion --sha HEAD --wait 1800` — leg operator: `verdict: green · checks 15/15` ci#37179459192 on `afef92f8885f`.
  - Smoke (boot path changed — `pump_child`): the plan's role=smoke entries, green in both runs; boot ok, status ready,
    cleanup exact.
- Watches: none folded.
- Outcome basis: the operator pass ran (pre-CI commit `afef92f`, no fix commit above it); the final HEAD's CI run
  ci#37179459192 is recorded in `evidence/operator-pass.md`; implement's P4 report (this conversation) holds the gate
  runs, the guard pairs and the CARRY 4 witness; the overseer's directive between implement and this wrap ran the
  operator pass and confirmed the economy witness.
- Process hygiene: implement's census — none left running (host process list checked for harness, wrapper,
  fake-agent, nextest, fuzz, `git maintenance`); re-measured at this wrap below at P7. Disk residue left for the
  operator in `../viola-mutants-scratch/`: `c4ctl/`, `c4fix/` (873 MB each), `wctl`, `wctl2`, `wfix`, `wfix2`.
