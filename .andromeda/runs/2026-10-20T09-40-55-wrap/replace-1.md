- line: .andromeda/architecture.md:388
old: «Anything else refuses the start with a fixed message that names no path:»
new: «A linked base whose filesystem reads less than `HEADROOM_FLOOR_MIB` (9 GiB) free by `statvfs` (`f_bavail × f_frsize`) is refused too, with the fixed message `e2e-home backing: {free} MiB free, below the {floor} MiB floor` (the two counts, no path); the floor is one compiled constant per keeper, equal in both and set by no variable, flag or config key, its basis the tmpfs's EDQUOT reading of about 8 GiB free plus 1 GiB (as measured at `viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup/evidence/headroom.md`). Anything else refuses the start with a fixed message that names no path:»

- line: .andromeda/test-plan.md:1092
old: «The witness is `scripts/profraw-census.sh <runs> <workers> [<name>]`, a dev-host script»
new: «A second mechanism is measured: a test that kills the wrapper closes its PTY, and the hangup reaches the hooks the fake agent ran in that terminal's foreground process group, so a `viola hook` cut off in its exit-time profile write leaves a page-cut profile that `llvm-profdata` refuses, or none; with every command the fake agent spawns in a process group of its own (the fake agent's exit bullet, §7) the census over the kill-shaped `chaos_wait_parked_wrapper_killed_exits_21` reads 0 refused profiles of 800 in 200 runs (as measured at `viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup/evidence/profraw-cause.md`). The witness is `scripts/profraw-census.sh <runs> <workers> [<name>]`, with a second form `--case <binary stem> <test name> <runs> <workers> [<name>]` over one integration case that offers each profile alone to `llvm-profdata merge` and counts the refused ones, a dev-host script»

- line: .andromeda/registries/contracts/test-plan/5-command-implementation.md:35
old: «the same tree in a plain-created sibling reads `Writable`»
new: «the same tree in a plain-created sibling granted the Users group's write explicitly (`icacls <plain> /grant *S-1-5-32-545:(OI)(CI)(W)`, its exit asserted) reads `Writable` whatever the sibling's parent grants»

- line: .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10
old: «against the class's 60 s kill and the `mutants` profile's 45 s (about 2 s of margin locally), and no kill line moved;»
new: «against the class's 60 s kill (the `mutants` profile's too), and no kill line moved;»

- line: .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:11
old: «plus three `[[profile.mutants.overrides]]`, the first matching winning: `filter = 'test(/verify_window_/)'` at `slow-timeout = { period = "15s", terminate-after = 3 }` (a 45 s kill) stands first, ahead of `filter = 'package(viola-e2e)'` and `filter = 'test(/send_window_/)'` at `{ period = "15s", terminate-after = 2 }`»
new: «plus four `[[profile.mutants.overrides]]`, the first matching winning: `filter = 'test(/verify_window_/)'` at `slow-timeout = { period = "20s", terminate-after = 3 }` (a 60 s kill, the class's `ci` kill) stands first, ahead of `filter = 'package(viola-e2e)'`, `filter = 'test(/send_window_/)'` and `filter = 'test(/send_under_the_paste_hint_/)'` at `{ period = "15s", terminate-after = 2 }`»

- line: .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:11
old: «finds the `package(viola-e2e)` override wherever it stands).»
new: «finds the `package(viola-e2e)` override wherever it stands). The paste-hint override holds the two `send_under_the_paste_hint_` cases, which wait out the product's gate (its 8.5 s maximum) under a held paste hint by design, 8.7–9.9 s on three OSes under `ci` (as measured at ci#39383206773), within a second of the 10 s default kill; `mutants_paste_hint_override_holds_the_class_above_the_default_kill` pins its kill above the default.»

- line: .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:11
old: «(planted-hang control at that chunk: TIMEOUT at 45.005 s under `mutants`, PASS at 84.513 s under the default profile;»
new: «(planted-hang control of the class at its 60 s kill: TIMEOUT at 60.003 s under `mutants`, PASS at 112.961 s under the default profile, as measured at `viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup/evidence/kill-lines.md`, and of the paste-hint class at its 30 s kill: TIMEOUT at 30.003 s, PASS at 48.413 s;»

- line: .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:11
old: «34.3 s as measured on the dev host at chunk 2026-10-07-send-waits-out-the-paste-hint, below the 45 s kill)»
new: «with Run E's settle 42.952 s alone on the dev host and 42.874 / 43.078 / 43.714 s on the CI legs under `ci` (as measured at ci#39383206773), below the 60 s kill)»
