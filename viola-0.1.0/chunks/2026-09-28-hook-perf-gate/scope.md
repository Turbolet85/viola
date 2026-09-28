# Scope — 2026-09-28-hook-perf-gate

**Working entry:** `viola-0.1.0/working-route.md:49` — "Hook perf gate — hyperfine-gated hook deadlines per OS in CI,
forced-panic fail-open case on the real binary, over-4 KiB concurrent detail line" (Epoch 2b — Windows slice I b: events and
ledger). It carries three CARRYs, folded below.

**Intent (the val-1 anchor):** `viola hook`'s deadlines stop being provisional prose and become a gated CI verdict. A hyperfine
row times the real `viola hook` binary per hook on every CI OS, the harness writes `perf-*.json`, and `gate --require perf`
fails the run when a hook's `max` sample reaches the deadline. The fail-open contract is proven where it is weakest, on a panic:
a feature-gated seam forces `viola hook` to panic, and the real binary still exits 0, writes no stderr and prints no body. The
same panic's detail line (payload + backtrace, over 4 KiB) closes the concurrent-append half of obs D-28.

**CI verdict read at Setup 5a:** `85ae5aa` (the last wrap's flip, the only sha through HEAD) — green, 15/15 checks, wall 242 s,
ci#36380661854 push completed/success. Nothing to fold; the overseer's relay said the same.

## What this chunk builds

1. **The perf arm.** `scripts/agent-run.* run --perf` — the harness arm that runs hyperfine over `viola hook` and writes one
   `perf-<hook>.json` per timed hook into the run's artifacts, where `gate.rs` already reads `perf-*.json` and gates hyperfine's
   `max` against `SPINE_DEADLINE_S = 1.0` (crates/viola-e2e/src/harness/gate.rs:37, :192–216, read at P1). Today the reader
   exists and no writer does (no `hyperfine` in `.github/`, `scripts/` or `crates/viola-e2e/`, grepped at P1).
   - [premise-corrected: test-plan §10's table lists session-start · user-prompt-submit · stop · session-end ·
     pre-tool-use and no async-tier row; `pre-tool-use` is an unknown event at HEAD (tests/hook_fail_open.rs:140), and
     no recorded fixture exists (`fixtures/` holds only `fake-scripts/`)] The timed rows are the four §10 rows that
     exist at HEAD: `session-start`, `user-prompt-submit`, `stop`, `session-end`, each against one live perf session
     (the wrapper only appends a `hook.event`, src/cmd/run.rs:100–105) with a synthetic payload the harness writes.
     The `pre-tool-use` row lands with the dialog-tier chunk (working-route :62); the async tier is untimed. The gate
     requires each named row, not only a non-empty set (gate.rs:202 passes a missing row today).
2. **The hyperfine 1.20.0 pin.** Installed at that exact version on the host before any gate command is written against it
   (the Tier-1 learning: probe, baseline and gate a CI tool at the version CI pins).
3. **The CI perf job and `gate --require perf`.** The **job-shape fork is this chunk's phase call** (entry CARRY 1): its own
   per-OS job (test-plan §9) vs a step inside `test` (obs-plan §10). Either shape runs on all three CI OSes.
4. **Deadline values stay provisional:** the gate `max < 1.0 s`, the hook's own `SPINE_DEADLINE` 750 ms (src/cmd/hook.rs:33,
   read at P1). This chunk measures and gates them; it does not retune them. A measured `max` that breaches on any OS is
   a surfaced finding for the operator, not a silent retune (verified: test-plan §10 Verdict, architecture [Hook
   Transport]).
5. **The `FAKE_AGENT_HOOK_PANIC` seam** (entry CARRY 2; RATIFIED by the founder, live, 2026-09-28 06:21, relay: the Viola
   overseer — re-stated in the operator's invocation). Compiled only under feature `fake-agent`, never in a release build,
   read nowhere else. It lands with:
   - its security-plan Decisions Log entry (security.md: "Another seam needs a Decisions Log entry"). Phase and implement
     never amend spec sources, so the entry is authored by this chunk's wrap cascade from implement's report, the way
     E1's `2026-09-28` entry was (verified: security-plan sidecar `2026-09-27-hooks-to-normalised-events`); the "one
     exception" statements (security-plan §Input Validation row, §Anti-Patterns Universal, §Secret Management Storage;
     architecture's four; the security.md rule line) widen in the same cascade.
   - a `cli_controls_not_disableable.rs` row. [premise-corrected: no `tests/cli_controls_not_disableable.rs` exists, and
     test-plan §5's Vector 6 table re-runs four negatives (`send` ESC → 13, `answer` unstamped → 12, human wheel → 10, a
     0770 `--home` → 21) plus a completeness case, none of whose verbs or controls exists at HEAD (`src/cmd/` = hook.rs,
     run.rs, mod.rs)] What the row asserts now, or whether it waits for the verbs, is a P4 fork.
   - the trigger is a closed value: the seam fires only on `FAKE_AGENT_HOOK_PANIC=1` (a Vector 6 row sets `0`, `false`,
     `off` and empty, which must not fire), and only after `viola_obs_init` (src/cmd/hook.rs:81), where the panic sink
     carries the instance, so one role line and one detail line are written (src/main.rs:137–157).
   - `scripts/release-check.sh` already refuses a `fake-agent` artifact in the release build, probe `5/5` (verified:
     architecture sidecar `2026-09-27-epoch-2-cleanup`); the seam adds no release path.
6. **The forced-panic fail-open case on the real binary** (test-plan §6 Security sweep): a built `viola hook` with the seam
   set panics, and the process exits 0, writes nothing to stderr and prints no body (CLAUDE.md invariant: exit 2 is
   forbidden). It joins root `tests/hook_fail_open.rs` (verified: 11 cases, and its module doc, :5, names this chunk).
   [inferred — new] The forced panic leaves an `event:"panic"` role line in a kept home under `target/e2e-home`, and CI's
   G2 (ci.yml:124–129, `AGENT_RUN_KEEP_HOMES: "1"` at :28) fails on any such line; obs-plan carries no exemption. How
   the deliberate panic and G2 fit is a P4 fork.
7. **The obs D-28 concurrent-append half** (obs-plan.md:621): the concurrent-append check gains a `detail-hook.ndjson` line
   over 4 KiB — the panic payload + backtrace is the only hook detail line that large — so concurrent `viola hook` processes
   prove whole lines under a single `write_all` with torn-line tolerance, on all 3 CI OSes. The head chunk's 8-process check
   (16 whole lines in `hook-<name>.ndjson`, 8 in `detail-hook.ndjson`) carries none.

## P4 forks (operator, with the overseer's notes)

1. The perf gate runs in its own per-OS `perf` job (test-plan §9); `test`, pre-push and WSL stay untouched, so
   CARRY 3 does not fire and moves on.
2. G2 exempts exactly the seam's file path (`src/cmd/hook/seam.rs`), never a pattern, with a known-positive control
   that a panic line in any other file still fails; G2 becomes one script both jobs call.
3. `tests/cli_controls_not_disableable.rs` lands now with the seam row over the hook-path controls that exist at
   HEAD; test-plan §5 records the interim shape at wrap.

## Boundaries

- Not here: retuning either deadline; the dialog-tier hooks (owned by working-route :62's chunk); server verification and
  strict-modes for `hook.event` (Epoch 6, the ratified E1 gap stands); any other panic seam.
- No new env var beyond `FAKE_AGENT_HOOK_PANIC`, and it only under `fake-agent` (security.md seam rule).
- The fake agent and the release build stay separate: no test-only feature reaches `viola`'s release artifact.

## Folded annotation — CARRY 3 (conditional, travelling)

- The WSL root-install fix (chunk 2026-09-27-browser-verdict-reachability, overseer live ratification, operator-only):
  before the WSL distro is next re-provisioned, `scripts/wsl-provision.sh --install-deps` must stop running user-writable code
  as root — root runs only `apt-get install` over an unprivileged `install-deps --dry-run` list checked against a committed
  allowlist. **The chunk that first re-provisions takes it; until then it moves on with the first markerless entry.**
  This chunk triggers it only if it re-provisions the distro (verified mechanism: `scripts/wsl-provision.sh:44–47`
  replays exactly ci.yml's taiki-e `tool:` line holding `cargo-nextest@…cargo-llvm-cov@`; a separate
  `cargo install --locked hyperfine@1.20.0` step is not replayed). It fires if hyperfine joins that line or the WSL
  pre-push gains a perf stage — which depends on the P4 job-shape fork. If it does not fire, the CARRY moves on to the
  next markerless entry at this chunk's wrap.
