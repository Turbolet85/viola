
## 2026-10-13-epoch-4-cleanup — the mutation scratch re-created owner-only
**Section:** §Occupied Resources (the `<repo parent>/viola-mutants-scratch/` row)
**Change:** the wiped scratch is re-created through `viola_state::fs::create_private_dir`: on Windows it is the topmost folder created outside the profile and takes the protected user + SYSTEM DACL, so every temp copy and test home laid inside inherits owner-only, as one under `%TEMP%` does (a plain-created scratch on the runner's workspace drive inherits BUILTIN\Users write, refused `Writable` by the instance check). Witness: `wipe_scratch_takes_an_instance_tree_the_check_accepts_where_a_plain_dir_does_not` on CI's `test (windows-2025)` leg, as measured at ci#38375200217. The root package's Windows mutation baseline reading green is not yet measured (the next `windows-mutants.yml` dispatch). Was: "removed and recreated" with no lay-down mode.
**Why:** run 38215104120's red baseline on the root package — its cause was the scratch's DACL, not the instance check, which stands unchanged.
**Ref:** .andromeda/runs/2026-10-13T10-47-05-wrap/
