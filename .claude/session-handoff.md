# Session Handoff

**Last Updated:** 2026-09-28T08:04Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit is pushed at P7)
**Status:** clean
**Last Commit:** 2026-09-28-hook-perf-gate — the chunk's wrap commit (P7)

## Position
- Done: 2026-09-28-hook-perf-gate — `run --perf` + `gate --require perf` (four hyperfine rows) in a per-OS `perf` CI job;
  the founder-ratified `FAKE_AGENT_HOOK_PANIC` seam with the forced-panic and over-4 KiB concurrent fail-open tests; G2 as
  `scripts/g2-zero-panics.sh` with its exact-path seam exemption; the interim controls table.
- Next: **CLI output tokens** (working-route :51, Epoch 2b) → `/andromeda-phase`. It carries the moved WSL root-install
  CARRY (fires only if that chunk re-provisions the distro).

## Work done
- Phase, implement, the operator pass (29–32, driven on the operator's word) and this wrap in one window. CI on `5a693d6`:
  ci#36390764600, 18/18 green (9 jobs now).

## Drift resolved
- 45 amendments: architecture 13, security-plan 5, test-plan 18, obs-plan 9 (6 raised by Validate from the plan's list).
  10 leaves re-derived; 8 sidecar entries.
- **Ratified live by the founder** (09:52:07, relay the Viola overseer): the seam (first shown at 06:21) together with G2's
  exemption for panic lines at exactly `src/cmd/hook/seam.rs:<digits>` (shown to him as new). Recorded as the security-plan
  Decisions Log `2026-09-28` test-seam entry.
- **Resolved by the operator:** the `perf` job's `diag-perf-<os>` / `perf-<os>` uploads are the existing scan-gated,
  synthetic-input class (no new crossing), recorded in the obs sidecar.

## Notes
- **Owed to the founder:** H2, the product question (a key lost within ~50 ms of a ConPTY resize, rstudio/rstudio#18884),
  still OPEN.
- **For the operator:**
  - the kept control home `target/e2e-home/viola-session-NETvJg`, the kept perf home `target/e2e-home/viola-session-5gy5sh`,
    and `target/harness-check/`;
  - hyperfine 1.20.0 is now on the host (`cargo install --locked`);
  - the `.wslconfig` memory cap; `CARGO_BUILD_JOBS=16` kept; `target/mutants/` is the mutation target dir;
  - 3 `viola.exe` from `additional/viola-lab/prototype` are running (not this chunk's).
- **Curation:** two Tier 2 entries (testing.md: a feature-gated test module stacks `#[cfg(test)]` separately for
  cargo-mutants; retarget a `usage` test's unbuilt selector when it lands).
- **Deferred learnings:** `recurrence-despite-learning`: host-win32.md (documents through the Write tool). A `cat >>`
  heredoc document write was still tried at the operator pass, and the Bash guard refused it.
- **Last failed command:** none.

## Session End Status
Completed normally at 2026-09-28 11:04:18
