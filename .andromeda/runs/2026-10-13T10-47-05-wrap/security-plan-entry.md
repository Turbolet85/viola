
## 2026-10-13-epoch-4-cleanup — the wiped mutation scratch re-created owner-only
**Section:** §Security Anti-Patterns › Code Patterns (the `run --mutants` row)
**Change:** the wiped scratch is re-created owner-only through `viola_state::fs::create_private_dir` (the protected user + SYSTEM DACL on Windows), so an instance tree a test lays inside it passes the strict-modes check as one under `%TEMP%` does; NEVER re-create it with a plain `create_dir_all`, and NEVER loosen the check for it. The guard, no-fallback and no-path rules are unchanged.
**Why:** run 38215104120's root-package baseline failed the instance check inside a plainly re-created scratch; the fix is in the harness's lay-down, a narrowing, and `check_instance` is unchanged.
**Ref:** .andromeda/runs/2026-10-13T10-47-05-wrap/
