# Scope — Readiness gate and timing constants

**Marker:** `2026-10-04-readiness-gate-and-timing-constants` · **Version:** viola-0.1.0 · **Epoch:** Epoch 3 — Windows slice II: driving verbs and live proof
**Working entry:** `viola-0.1.0/working-route.md:72` — "Readiness gate and timing constants — screen-model quiet period, input-box and modal signatures as ledger rows, named confirmation window and deadlines, injected clock, parser-panic degrade", with four CARRY blocks (all four folded below; `route.py pins` indexed 4 freight blocks on :72, 363 · 212 · 300 · 993 chars).
**Host:** Linux (Omarchy, btrfs). Windows / ConPTY behaviour is witnessed only by the `windows-2025` CI runner (operator context at take-up).
**CLI:** installed `claude` 2.1.287; recorded fixtures exist only for 2.1.283 (operator context at take-up), so 2.1.287 is an unverified build until a `viola verify` stamps it.
**Mode:** autonomous (operator directive at take-up): any boundary widening is shown at P4 and HELD until the founder's morning, never ratified here; no epoch split (founder ruling 2026-09-29).
**P3 closure:** every `[inferred]` bullet below is closed against `research.md` (verified, tag dropped and evidence named; or `[premise-corrected: …]`).

## What this chunk builds

The pre-send readiness gate on `run`'s pump thread (architecture §Established Decisions, [Screen Model]): a vt100 screen
model fed the child's PTY output under `catch_unwind`, which after `turn-ended` waits for the screen to go quiet and
then requires the version-stamped input-box signature and the absence of every known modal signature, else types
nothing and yields `not-delivered` / `input-not-ready`. With it, the timing constants the send path will need, each
named in one home, and an injected clock so every one of them is testable without wall time.

### 1. Screen model and readiness gate (the working entry's head)
- vt100 0.16.2 (architecture Stack row "Screen model") is added and fed every PTY output byte on `run`'s pump thread;
  byte passthrough to the human terminal is never delayed or altered by the feed. VERIFIED (research.md M1): vt100 and
  vte are absent from `Cargo.lock`; the graph is vt100 (MIT) · vte 0.15 (Apache-2.0 OR MIT) · itoa · unicode-width 0.2 ·
  arrayvec · memchr (MIT OR Apache-2.0), no build script, no env read, every MSRV under 1.96; the `cargo deny` verdict
  itself is implement's to read. 0.16.x is required: 0.15.x logged unhandled escapes to STDERR, `run`'s terminal. The
  pump's one production caller hands it `io::stdout()` (`src/cmd/run.rs:419`), so a tee there feeds the model without a
  viola-pty change.
- The gate's verdict is a closed outcome `ready | input-not-ready` (obs-plan `run.readiness_gate` span: `outcome`,
  `vt100_panicked`). Screen content is read ONLY as that verdict — never logged, never an event field, never asserted in
  a test (test-plan R7, obs-plan §11 "never derive telemetry from screen content").
- Quiet period: the screen must show no change for the quiet period before the signature checks; the gate gives up at
  its maximum wait. Both are open items architecture holds "as ledger rows beside the signatures".
- On an unverified CLI build the gate falls back to delivery confirmation only (architecture [Screen Model], last
  sentence). VERIFIED as an open design point: that sentence is the only source (no code reads a verdict yet), so what
  "falls back" means for the verdict is P4's to pin.
- The gate's first in-repo consumer. VERIFIED: no `send` verb, channel `send` method or MCP `send` exists at
  take-up (`src/cmd/` holds `hook`, `run`, `verify` only; the channel `Methods::dispatch` serves only `hook.event`,
  `src/cmd/run.rs:100-110`); no `RefusalReason` or refusal-detail type exists anywhere yet; the next entry (`:74 Confirmed send`) owns `send`. The
  consumer this chunk can wire is CARRY 1's typed-input probe in `viola verify` and the run pump itself; whether the
  channel exposes the gate here or at :74 is a P4 question under the consumer-first rule (testing.md: no code without a
  live consumer — the 2026-09-28-cli-output-tokens P4 ruling).

### 2. Ledger rows: input-box and modal signatures, quiet period, maximum wait (working entry + CARRY 1)
- CARRY 1, verbatim: "chunk 2026-09-28-capability-ledger-and-viola-verify landed six print-mode ledger rows and kept
  typed probes out (plan P4 operator ruling: typing without this readiness gate risks the H2 key-loss class): the
  typed-input probe drive (a PTY behind this gate) and the modal / input-box screen-signature ledger rows, each with its
  `viola verify` step, land here". Re-verified at P1: that chunk's `plan.md:499-500` (the P4 rejection) and `:525`
  ("the typed-input probe drive (a PTY behind the readiness gate), and the screen-signature rows") — `:58` there is the
  entry's line at that time, now `:72`.
- New closed-set rows in `viola_agent_claude::ledger` (six today: `shim-resolution` · `spine-hooks` ·
  `session-start-fields` · `prompt-verbatim` · `stop-message` · `largest-hook-payload`): the input-box signature, the
  modal signature set, and the quiet-period / maximum-wait values. Each row gets its `viola verify` step and its stamp
  through the one stamps writer. Signatures come only from compiled ledger rows, never from runtime or upstream text
  (security-plan :588).
- The typed-input probe drive: `viola verify` gains a PTY-driven interactive probe behind this gate (the existing probe
  is print-mode only). VERIFIED as open, with its facts: `verify` spawns only through `run_bounded` (std `Command`,
  pipes, `src/cmd/verify.rs:78`) — this probe would be its first PTY spawn; `[NN/MM]` already follows
  `LedgerRow::ALL.len()` (verify.rs:265). Its shape is P4's.
- Measuring the real signatures needs a live `claude` under a PTY. VERIFIED, and sharper (research.md M4): `verified`
  needs every row of `LedgerRow::ALL` `pass`, every CI and root/harness home stamps against the fake agent at 2.1.283,
  and the fake agent renders no screen — so a signature row added to `ALL` turns every stamped home unverified until the
  fake agent replays a RECORDED input-box screen, and no screen recording exists for any version. Whether this chunk runs a live
  `viola verify` on the Linux host against 2.1.287 (subscription use; the "First live test" entry `:82` is the planned
  live-proof chunk) or lands the rows with fixture-recorded screens and leaves the live stamp to `:82` is a P4 fork;
  Windows-specific screen behaviour is witnessed on the `windows-2025` runner only with the fake agent (no Claude
  credential on any runner).

### 3. Named confirmation window and deadlines; injected clock (working entry)
- Every timing constant the send path uses gets ONE named home: the confirmation window (architecture [Delivery
  Confirmation]: a per-CLI-version ledger row with a built-in fallback compiled into `viola-agent-claude`), the
  gate's quiet period and maximum wait, and the deadlines `send`'s total is built from (gate max wait + atomic paste +
  window, which must stay under the MCP client's tool-call timeout). VERIFIED, and wider: architecture §[Hook
  Transport] names THIS chunk to turn the hook's crate-private `SPINE_DEADLINE` (750 ms, `src/cmd/hook.rs:40`) into "the
  product constant the perf gate reads" (test-plan :1220, arch request (3)); the gate's literal is
  `SPINE_DEADLINE_S = 1.0` (`crates/viola-e2e/src/harness/gate.rs:36`). viola-e2e depends on viola-core, not
  viola-agent-claude, so a shared constant lives in viola-core.
- The confirmation window's USE (matching `prompt-submitted`, `no-prompt-submitted`) is `:74`'s; this chunk names the
  constant and its ledger row.
- An injected `viola_core::Clock` (test-plan :1106: sync crates use mock_instant 0.6.1 or the injected
  `viola_core::Clock` for heartbeat 1 s / 5 s, the send confirmation window and `budget_override_until`). VERIFIED: no
  `Clock` exists (viola-core deps: nutype only); the heartbeat is `BEAT_EVERY = 1 s` (`viola-state/src/heartbeat.rs:13`)
  and liveness is already clock-free (`classify(beat_age, …)` takes fed ages, `liveness.rs:22`); whether the heartbeat
  loop migrates onto `Clock` here is P4's.

### 4. Parser-panic degrade (working entry + CARRY 2 + CARRY 3)
- The vt100 feed runs under `std::panic::catch_unwind`; on a panic the gate degrades to `not-delivered` /
  `input-not-ready`, passthrough to the human terminal continues, and `parse-rejected{parser:"vt100-feed"}` is logged
  (security-plan :238 / :591; obs-plan :915). The human always wins: a gate failure never blocks a human keystroke.
- CARRY 2, verbatim: "chunk 2026-09-24-fake-agent-and-test-data-fixtures deferred the fake agent's
  `--vt100-panic-bytes` mode to this entry (consumer-first: its byte sequence is set by the vt100 parser-panic degrade
  built here)". Re-verified at P1: that chunk's `report.md:115` ("`--vt100-panic-bytes` … deferred").
  [premise-corrected: vt100 0.16.2 panics on real bytes at small sizes — 24x1 + U+4E2D, 1x1 + `?u`, 1x2 + `abc`,
  measured in both profiles; 0 panics in ~92k inputs at 24x80 (research.md M2)] — no new env seam is needed: the mode
  writes the bytes and the test chooses the PTY size. NEW, from the same research (M3): the viola panic hook writes a
  G2-counted `event:"panic"` line (+ a detail line with raw frames) for every panic, caught ones included
  (`src/main.rs:138-181`), so a forced feed panic under a root E2E home reads red on G2, and an unguarded feed would
  write one such pair per 8 KiB read at a 1-column terminal.
- CARRY 3, verbatim: "chunk 2026-09-24-quality-gates seeded the fuzz pipeline with a `viola_name` target. This entry's
  vt100 feed under `catch_unwind` over arbitrary bytes takes a proptest property (`cases: 512`, committed seeds), plus a
  `fuzz/fuzz_targets/` target and a seeded corpus (test-plan §6 Property suite)". Re-verified at P1:
  `fuzz/fuzz_targets/` holds `viola_name.rs`, `channel_frame.rs`, `hook_stdin.rs`.

### 5. The half-removed fixture repos (CARRY 4 — [inferred], founder-delegated overseer disposition)
- CARRY 4, verbatim: "chunk 2026-10-04-windows-boundary-mutation-workflow ([inferred]; the overseer's disposition,
  founder-delegated, at its implement report): one full Linux `run --mutants --package viola-e2e` under
  `terminate = "wait"` left 21 half-removed throwaway git repos of the `Pass` fixture
  (`crates/viola-e2e/src/harness/run/mutants/base.rs`) in `TMPDIR` with no nextest kill — a `terminate`-independent
  class (26 of 461 under `immediate`), measured at that chunk's `evidence/leak.md`. Each fixture `git commit` spawns a
  detached `git maintenance run --auto --quiet --detach` (measured: `GIT_TRACE`, git 2.55.0, dev host). hypothesis:
  that process writes into `.git/objects/pack/` while the `TempDir` drop walks the tree, so the removal stops
  part-way. Fix candidate: `-c maintenance.auto=false` on the fixture's git calls. Two-sided acceptance: a full
  viola-e2e mutation run in a fresh short NOCOW `TMPDIR` leaves 0 half-removed repos with the fix, beside a control run
  without it that leaves ≥ 1". Re-verified at P1: `base.rs:234` `struct Pass(tempfile::TempDir)`, its git calls at
  `:255` (`add -A`, `commit -q -m`); `evidence/leak.md:84-101` holds the measurement and the hand-off.
- [inferred] causal mechanism, kept as the CARRY's own "hypothesis:" — the detached maintenance process racing the
  `TempDir` drop. Measured: the spawn (GIT_TRACE). Not measured: that it causes the half-removal. P3 (research.md M5):
  partly supported — the leftovers hold `objects/pack/tmp_idx_*` / `tmp_rev_*`, `rr-cache/`, `MERGE_RR` and no `HEAD`,
  which only a gc-class git process writes, never the fixture's `add`/`commit`; the race itself did not reproduce on an
  idle host (0 of 300 scratch repos, tmpfs and NOCOW btrfs, both arms). Stays `[inferred]`; the two-sided acceptance is
  the witness. The fix sites are `init_repo` (`crates/viola-e2e/src/harness/run.rs:516-531`) and `Pass::commit`
  (`…/mutants/base.rs:248-268`).
- This is a test-harness fix outside the readiness gate's surface; it rides this chunk because the wrap named it the
  owner (route-resolve fold), not because it touches the gate.

## P4 forks (overseer, autonomous mode, 2026-10-04) — val-1 intent-incomplete, amended at P5
- CARRY 1 → "Mechanism now, rows held": the gate mechanism lands; no new `LedgerRow`, no PTY typed-input probe, no live
  recording in this chunk. They are shown in plan.md §Held widening (FOR DISCUSSION) as a Boundary-widening item for
  the founder's morning ruling; owner `:82` First live test, or the ruling. Basis: research.md M4.
- CARRY 2 → "In-process witness": the degrade is proven in-process with the measured bytes; the fake agent's
  `--vt100-panic-bytes` mode and the run-level forced-panic E2E go to `:74`'s send chaos case, which owns the G2 question
  (overseer: "the consumer exists and is named"). Basis: research.md M2, M3.
- No capability claimed: v1-21 needs the dialog deadline (`:78`) and the MCP `wait` default (`:76`) too.

## Boundaries
- No `send` verb, channel `send` method, MCP `send` tool or delivery matching (`:74`); no wheel (`:80`); no dialog
  answers (`:78`).
- Screen content is never logged, evented, snapshotted or asserted; only the gate verdict leaves the screen model.
- No new env var outside `VIOLA_*` and the two existing `fake-agent` test seams without a Decisions Log entry; no
  `config.json` key that can widen or disable the gate.
- No Claude credential on any CI runner; a live `claude` run happens only on the operator's host, on the operator's
  word.
- No mutation entry in the `[[gate]]` block and no `pre-push` mutation leg (the 2026-09-28 ruling); CARRY 4's witness
  runs are operator-pass evidence.
- No epoch split (founder ruling 2026-09-29).

## CI verdict since the last wrap (Setup 5a)
- `5bcb0a0cc584` (the 2026-10-04-windows-boundary-mutation-workflow wrap commit; the only sha from the last flip
  through HEAD): read 2026-10-04T04:29Z `verdict: in progress` · checks 15/15 · ci#37177029795 in_progress, oldest
  running `test (windows-2025)` 106 s — CI 5bcb0a0: verdict not yet available; re-read at P3: **`verdict: green` ·
  checks 15/15 · wall 280 s** · ci#37177029795 completed/success. Nothing to disposition.
