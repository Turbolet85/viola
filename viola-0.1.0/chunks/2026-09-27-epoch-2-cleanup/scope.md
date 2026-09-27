# Scope — 2026-09-27-epoch-2-cleanup

**Working entry (verbatim title + hint):** Epoch 2 cleanup — four files under 800 lines, clone pairs gone, channel/state/pty survivors
killed, mutation scratch off C:, harness loop aids, machete annotation

**Epoch:** Epoch 2b — Windows slice I b: events and ledger (its first entry). This is the Epoch 2 boundary cleanup chunk, inserted by the
operator-requested adaptation at the 0-pending wrap `.andromeda/runs/2026-09-27T14-05-00-wrap/adaptation-record.md` (relay
`D:/dev/projects/additional/viola-overseer/e2-route-adaptation.md` item A; triage `epoch2-triage.md` beside it). Its subject is Epoch 2's
code and harness, as measured by the Epoch 2 code audit (`.andromeda/runs/2026-09-27T13-39-34-code-audit/proposals.md`, trend
a28f696 → 69abc0d, 0 proposals, informational rows) and the Epoch 2 evolve diagnosis
(`.andromeda/runs/2026-09-27T13-31-25-evolve-diagnose/proposals.md`, instance bucket).

_Premise closure ran at P3 (research.md). Every `[inferred]` tag is now either dropped (verified) or replaced by
`[premise-corrected: …]`._

## What this chunk builds

A behaviour-preserving cleanup of the Epoch 2 problem spots, plus harness tooling that removes recurring friction. It adds no product
capability.

1. **Four files under 800 lines.** Split by concern, no behaviour change, every name reached from outside kept at its current path. The
   metric is the code audit's: tokei 14.0.0 `code` lines (`record.py:49`), physical `wc -l` reported beside it. No file the split
   creates or leaves exceeds 800.
   - `crates/viola-e2e/src/harness/pre_push.rs` 1335 (physical 1373)
   - `crates/viola-pty/src/lib.rs` 1074 (physical 1218)
   - `crates/viola-e2e/src/harness/run/mutants.rs` 1029 (physical 1155)
   - `crates/viola-channel/src/server.rs` 869 (physical 1025)
   - A split moves production code with its tests: an out-of-line `#[cfg(test)] mod x;` file is an orphan to the orphans gate, and inline
     tests do not shrink a file (testing.md 2026-09-25; research.md Patterns). The parent file stays the module root (the Epoch 1
     `harness/run.rs` + `harness/run/*.rs` shape), so its unmoved lines stay out of the chunk diff.
2. **Clone pairs gone.** Shared helpers replace (coordinates re-measured, research.md fact 10):
   - the `Fields` tracing visitor, `crates/viola-channel/src/lib.rs:103` (`test_capture`) ↔ `src/cmd/run.rs:372` (13 L);
   - the Windows DACL read-back, `crates/viola-channel/src/server.rs:919` ↔ `tests/channel_endpoint.rs:385` (21 L), and `canonical_sddl`,
     `server.rs:967` ↔ `tests/channel_endpoint.rs:330` (17 L). Both copies stay as tests: the in-crate one is the only one cargo-mutants
     runs for `viola-channel` mutants (testing.md 2026-09-27), and the root one reads the user SID independently of the product's lookup.
   - No new clone pair may appear. The other top-10 pairs lie outside the entry's named classes and stay (research.md fact 10).
3. **Survivors: the audit's 11 are already judged; no killing test is owed.**
   [premise-corrected: the audit's per-crate cargo-mutants ran on this Windows host alone, with no union. On the compiling leg, recorded
   CI verdicts judged all 11 caught or unviable: `viola-channel` 4 are `#[cfg(unix)]` code (run 36318398739, ubuntu: 2 caught, 2
   unviable); `viola-state` 2 (run 36298052174, ubuntu: both caught); `viola-pty` 5 are the `#[cfg(unix)]` `HostTerminal::enter` body
   and the Unix `host_size` block, not Windows code (run 36165685381, ubuntu: 4 caught, 1 unviable). research.md fact 5.]
   - What stays: the evidence of those readings lands in this chunk's `evidence/`, and this chunk's own union verdict (pre-push and CI)
     covers every mutant its diff regenerates, the moved ones included. A mutant the union reports missed is this chunk's to kill.
4. **Windows mutation scratch off C:.** pre-push's Windows mutation leg stops copying the workspace into `%TEMP%` on C:; it uses a
   dedicated directory on D:, wiped at the start of every run and counted in the cache report.
   - measured (per the triage): ~30 GB per run left behind by killed runs and by one pre-push that finished green (15:26, 2026-09-27).
   - [premise-corrected: one instance of the triage's hypothesis is measured — a stray `viola-fake-agent.exe` from a windows leg's
     cargo-mutants temp copy, parent gone, stopped by pid (`wrapper-channel/evidence/operator-pass-continued.md:34–35`). The 15:26
     residue's own cause stays a hypothesis: "a leaked fake-agent holds files". The wipe-at-start bounds the residue whatever the cause.]
   - The Linux leg's `TMPDIR=<distro home>/viola-pre-push-scratch` is a named, distro-derived constant (`pre_push.rs:33`, `:581`;
     security.md); this item touches the Windows leg only.
5. **Harness loop aids** (evolve proposals, instance bucket):
   - P3 — keep a failed test's home; a scoped `run --mutants` inner loop (the proposal: file-scoped cargo-mutants over the chunk's changed
     sources, host leg only, the full pre-push union kept as the verdict).
   - P7 — archive each run's `outcomes.json` and junit into the run's own record before the next run overwrites them.
   - P10 with L4 — `run --mutants` stops the session's rust-analyzer by exact ExecutablePath itself, or directs cargo-mutants `--output`
     outside the LSP-watched workspace.
   - P11 — a `scripts/wsl-exec.sh` wrapper for the WSL crossing (the proposal: `MSYS2_ARG_CONV_EXCL='*' wsl.exe -d <distro> --exec "$@"`).
     No such script exists in `scripts/` today (measured at the adaptation wrap).
6. **cargo-machete annotation.** The audit's `viola-e2e` → `proc-macro2` unused-dep hit is a false positive: a feature-enabling pin for
   syn's `span-locations` (`c-dead.json`). A `[package.metadata.cargo-machete] ignored` entry states it. machete runs only inside the code
   audit, not CI (measured at the adaptation wrap); cargo-machete 0.9.2 is on the host.
7. **Recurrence watches (moved here from Hooks to normalised events).**
   - `viola-pty tests::spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code` (one CI red on run 36296402785, job
     108555954166, windows-2025 under llvm-cov: the key written right after the resize to 120x40 was never reported within the 10 s
     `lines()` bound — a bounded wait, not a hang; research.md fact 6) — this chunk tries a forced-window reproduction first (a
     test-only hold, testing.md 2026-09-27).
   - `viola-e2e::harness_lifecycle harness_session_boots_reports_logs_and_tears_down` (one concurrent-boot red; `replace_private_shared` is a
     fix by reasoning, and the exclusive-bind arbiter does not reach it).
   - Match any recurrence to these names; a red with a captured chain re-opens its cause. Expiry: retire both watches after 3 consecutive
     green CI runs with no recurrence of either name, recorded when it happens.

## Folded freight (working-route.md:43, 4 CARRY blocks — `route.py pins`, run dir `pins.txt`)

- **CARRY 1 (adaptation, item A):** folded into items 1–3 and 6. Coordinates re-verified by P3 at HEAD.
- **CARRY 2 (harness items):** folded into items 4–5. The cargo-mutants 27.1.0 facts (mutates const initializers; runs only the mutated
  package's tests; unviable without the fake-agent feature — evolve P3) go to `testing.md` through THIS chunk's wrap curation, not its code.
- **CARRY 3 (sizing, overseer direction for this chunk's P5):** moving code in the four splits has cargo-mutants re-test the moved lines on
  both legs.
  - [premise-corrected: the mechanism is re-derived — `chunk_diff` is a plain `git diff <merge-base>`, so moved lines in a new file are
    added lines and lines a kept parent file keeps are not in the diff (`run/mutants.rs:129–157`). The relayed "~154" is the Epoch 1
    scope's count; that chunk's measured legs tested 143 and 148 (research.md fact 3).]
  - The plan states the diff mutant count and the estimated pre-push minutes; above ~60 min it names which splits to defer, as a decision
    for the operator.
- **CARRY 4 (recurrence watches):** folded into item 7.

## Operator directions at P4 (overseer, founder-delegated, 2026-09-27)

- The shared test helpers live in a `viola-channel` `test-support` feature, enabled only by the root dev-dependency, and
  `scripts/release-check.sh` proves the feature (and `fake-agent`) absent from the release build.
- One host scratch dir serves item 4 and P10. On a Windows host, `run --mutants` puts cargo-mutants' temp copies and `--output`
  beside the repo in `viola-mutants-scratch`. Because it wipes a dir outside the repo, a guard refuses unless the resolved path ends
  in exactly `viola-mutants-scratch`, is not the repo root or any ancestor, and is not a drive root. A refusal is a named error,
  never a fallback, and the guard gets its own remove-the-guard pair.
- Added at P5 validation: the platform record behind the viola-pty watch (rstudio/rstudio#18884, a key lost within ~50 ms of a
  ConPTY resize) is carried to the operator as an owed product question. This chunk builds no fix for it (plan step 2).

## CI read at Setup

- `69abc0d` (the Epoch 2 wrap): green · 15/15 · wall 242 s · run ci#36322480903.
- `a0e6506` (the route adaptation, route/docs only): verdict not yet available at Setup (in progress, 14/14 checks, run ci#36324598324).
  Re-read at P5: green · 15/15 · wall 232 s · run ci#36324598324. No red to disposition.

## Boundaries (out of scope)

- No product behaviour change and no spec-source amendment is expected; the harness JSON documents and typed exits stay identical except
  where an aid adds a field or file (items 4–5), which the plan names.
- Evolve P10's second half (`scripts/wsl-provision.sh` running `apt-get update` before its installs) is not in the route entry's freight;
  out unless the operator adds it.
- The pipeline-class evolve items (V22–V28) and the already-known ones are the overseer's, not this chunk's. So is the code audit's
  Windows-only per-crate mutation method that produced item 3's false survivors (a pipeline observation for the overseer).
- The other files over 600 lines (`src/bin/viola-fake-agent.rs` 619, `src/obs.rs` 615, `tests/cli_fake_agent.rs` 609) are under 800 and stay.

## Surfaces and contracts touched

- `viola-e2e` harness modules (`harness/pre_push.rs`, `harness/run/mutants.rs` → submodules) and the harness's cache report.
- `viola-pty` (`lib.rs` split; the watch test's child), `viola-channel` (`server.rs` split, test capture, DACL helpers).
- `src/cmd/run.rs` (tracing capture dedupe), `tests/channel_endpoint.rs`.
- `crates/viola-e2e/Cargo.toml` (machete metadata), `scripts/wsl-exec.sh` (new).
- The test-plan §3 harness contract (5 commands, one JSON document plus a typed exit per command) holds.
