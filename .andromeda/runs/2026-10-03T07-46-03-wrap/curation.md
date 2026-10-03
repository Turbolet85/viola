# Curation — 2026-10-02-epoch-2b-cleanup

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):
    + testing.md: "Import an item used only inside `#[cfg(windows)]` (or `#[cfg(unix)]`) test code inside that cfg'd item…" (confidence 0.8)
      Proof: CI ci#37106821284 on `e848944` failed `lint (ubuntu-latest)` and `lint (macos-latest)` with `error: unused import: remove_owned --> tests/cli_instance_state.rs:19:5`; Windows clippy (gate entry 2) was green. WSL clippy in `~/viola-pre-push` reproduced it (exit 101), and after the fix it read exit 0. Fix commit `9e3b850`; ci#37107107417 green. (`evidence/operator-pass.md`)
    + testing.md: "On Windows, std's `remove_dir_all` (and so a `TempDir` drop) deletes read-only files…" (confidence 0.8)
      Proof: `throwaway_repo_leaves_no_dir_after_drop` was green with the planned guard removed; a 225-test viola-e2e run left 0 `%TEMP%` dirs. `remove_owned_keeps_the_owner_record_while_a_file_is_held` read red under a plain `remove_dir_all` ("the record went first") with the held file sorted after `owner.json`, and green with the guard. A first form with the held file under `home/` read green both ways. (`evidence/leaks.md`; it falsified plan step 4)
    + host-win32.md: "cargo-mutants copies the tree under `TMP`, and a deep `TMP` pushes the copy's build paths past the Windows path limit…" (confidence 0.8)
      Proof: five direct-file runs with `TMP` under the session scratchpad's `target/agent-run/m1-*/tmp` each failed in 5–8 s with `error: linking with link.exe failed: exit code: 1104` and "cargo build failed in an unmutated tree"; the same commands with `--in-place` ran (`evidence/m1.md`).
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters:
    - 0 dup. Probe hits read: `testing.md:58` covers cfg bodies unkillable by mutation, a different facet; `host-win32.md:24` covers a PowerShell redirect writing UTF-16, a different facet.
    - 2 task-specific: the M2 open-red ruling (route-pinned) and the D: disk guard (a session directive).
    - 0 conflict.
    - 2 deferred (→ handoff), the max-3 cap at a four-way 0.8 tie:
      - C4: Windows PowerShell 5 parses a BOM-less UTF-8 `.ps1` as ANSI, so a non-ASCII character breaks parsing; keep scratch `.ps1` files ASCII (0.8).
      - C5: let a red CI run finish before folding its fix, so one fix covers every red it shows (0.7).
    - 1 existing entry applied as designed (testing.md 2026-09-25 remove-the-guard runs): the positive is recorded in evolve, not curated.
  No-other-home: the three applied entries (each 0.6 + the no-other-home signal; none of the three is carried on the route, in a master, or in a playbook rule)
  CLAUDE.md size: see P7's health check 1
