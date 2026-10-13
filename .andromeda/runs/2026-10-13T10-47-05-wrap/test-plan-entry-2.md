
## 2026-10-13-epoch-4-cleanup — the Windows scratch lay-down and the Windows-target clippy
**Section:** §3 → 5-command implementation (`run` step 4, Command, the Windows host scratch arm); §9 Gate commands (the Lint row)
**Change:**
- `run` step 4: the scratch is removed and re-created through `viola_state::fs::create_private_dir`, so it takes the protected user + SYSTEM DACL and every temp copy and test home inside inherits owner-only, as under `%TEMP%`. Pinned by the `#[cfg(windows)]` unit `wipe_scratch_takes_an_instance_tree_the_check_accepts_where_a_plain_dir_does_not` on CI's `test (windows-2025)` leg with no dispatch (a tree inside a wiped scratch passes `check_instance`; the same tree in a plain-created sibling reads `Writable`), as measured at ci#38375200217. The root package's Windows baseline reading green is not yet measured (the next `windows-mutants.yml` dispatch). Was: "removes and recreates it" with no mode.
- The Lint row: beside CI, a chunk plan may list a check-only Windows-target clippy (`CARGO_TARGET_DIR=target/wincheck … --target x86_64-pc-windows-msvc -- -D warnings`) as a chunk gate entry on the Linux dev host, nothing linked; neither a CI step nor a `pre-push` stage.
**Why:** run 38215104120's root-package baseline failed inside a plainly re-created scratch; CI lints `cfg(windows)` code the dev host does not build.
**Ref:** .andromeda/runs/2026-10-13T10-47-05-wrap/
