# Session Handoff

**Last Updated:** 2026-09-29T06:31Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit is pushed at P7)
**Status:** clean
**Last Commit:** 2026-09-29-h2-conpty-resize-probe — the chunk's wrap commit (P7)

## Position
- Done: 2026-09-29-h2-conpty-resize-probe — H2 reproduced 13/200 on the windows-2025 runner, every loss class K (the
  resize reached the Rust test child; the key, written and flushed into ConPTY, never reached its read; `dsr-cpr 0`);
  cause not established, product impact for `claude` unmeasured (overseer correction at the wrap); document branch:
  recorded in arch [PTY], test-plan §5, gotchas.md; the red test now keys after the child sees the new size (200/200).
  Final HEAD `8a98b9d` ci#36529984077 green 15/15.
- Next: **Verify-stamped test homes and harness** (working-route :59) → `/andromeda-phase`. Its coverage-channel CARRY
  is retired (closed by this chunk).

## Work done
- Implement, the operator pass (4 pushes: pre-CI, reproduction, verification, loop removal + fold; 2 of the 3
  measurement pushes used) and this wrap in one session. ci.yml byte-identical to `90aba7c`. One CI red folded: the
  ubuntu corrupt coverage profile (the harness self-tests' throwaway crate no longer inherits cargo-llvm-cov's names).

## Drift resolved
- 3 detector proposals (arch 2 · test-plan 1; the other five 0) + 1 orchestrator raise (test-plan §10 :1506): 4
  applied, 0 rejected, 0 escalations; 2 narrowed at apply (collateral claims the report does not carry); 4 sidecar
  entries; cascade 71 rows, no leaf changed.

## Notes
- **The founder's product question on H2 stays open** — whether the real `claude` loses a key typed right after a
  resize in `viola run` is unmeasured; the measurement is owned by working-route "First live test and self-drive"
  (CARRY pinned at this wrap, per the overseer).
- **Route:** `run --e2e` (the plan's entry 6, a plan defect) pinned on :85 "The board: viola list" with its owner; a
  CARRY on :61 "Fake-agent drift contract" for test-plan §7's `size` receipt wording vs the fake agent; the H2 CARRY on
  :74; the coverage-channel CARRY on :59 retired (closed by this chunk); the WSL `--install-deps` hardening CARRY moved
  unchanged from the H2 line to :59 (this chunk did not re-provision).
- **Epoch 2b has grown to 10 entries (8 promoted):** a boundary here would restore the diagnose/audit cadence — the
  split is yours to name at a wrap.
- **Curation:** T2 1 (verification-harness.md), T3 2; 3 rejected at 0.6 (homed in masters this wrap).
- **Deferred learnings:** `recurrence-despite-learning: host-win32.md Transports — "Documents: the Write tool … a script to a scratchpad file run by path"` (a `cat > file` heredoc was blocked by the Bash guard).
- **For the operator:** 5 `viola.exe` of `additional/viola-lab/prototype` are running (not this chunk's).
- **Last failed command:** none.
