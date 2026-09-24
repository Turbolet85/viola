CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings): none
  Tier 2 (.claude/rules/*):
    + testing.md: "A test wait that a mutant can reach must detect the watched process's exit and be bounded below cargo-mutants' 20 s auto-timeout floor, and so must the nextest mutants kill …" (confidence 0.8: operator correction +0.4, verified by measurement +0.4)
      Proof: CI run 35995290314 (sha 2834e4d) mutants artifact. Two `Wrapper::wait_ready` consumers were SIGTERM'd at 19.82 s under the 20 s auto timeout, and only 2 of 125 tests started. The fix (exit-aware 10 s wait, 5 s×2 kill) made the local mutation baseline green; `wrapper_boot_exiting_before_ready_fails_as_exited` passes. Operator at phase P4: "fix by cause and by value, not through nextest alone".
    + testing.md: "Keep a #[cfg(unix)]-only function to a minimal OS reader …" (confidence 0.8: verified by measurement +0.4, specific technical detail +0.2, load-bearing +0.2)
      Proof: implement gate runs. Missed mutants went 13 → 1 → 3 after the mode check was split into pure `owner_only` / `mode_hit` plus a `#[cfg(unix)] file_mode`. The final 3 are all `file_mode` mutants, uncompiled on the Windows host (`implement-2026-09-24T12-21-11/30.log`).
    + host-win32.md: "Git Bash pwd prints /d/..., which a native Windows tool reads as D:\d\... …" (confidence 0.8: verified by measurement +0.4, specific technical detail +0.2, no-other-home +0.2)
      Proof: `lint-probes.sh` failed 6/6 with `failed to read D:\d\dev\projects\viola\crates\viola-core\Cargo.toml` until the manifest path used `pwd -W`; it then printed `4 bans fired, 2 controls clean`.
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters:
    - 1 dup: the clippy disallowed-macros allow limit, now in the re-derived `observability.md` body;
    - 1 below threshold at 0.6 exactly: the Write-hook block on `/target/` paths;
    - 1 deferred to the handoff by the max-3 cap: the lock edge-line probe lesson.
  Load-bearing: "Keep a #[cfg(unix)]-only function to a minimal OS reader …" → Quality gates
  No-other-home: "Git Bash pwd prints /d/… write any path a native tool will read as pwd -W"
  CLAUDE.md size: 121/200 · T1 1.3 KB, 0 over 600 B
