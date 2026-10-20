
## 2026-10-20-epoch-5-cleanup — the mutants profile's paste-hint class and the verify_window_ kill at 60 s
**Section:** §3 → `bootstrap-phases-derive-for-route-setup-project` (test-runner-install: `[profile.ci]`'s Run E sentence, `[profile.mutants]`, its fail-fast kill list, the viola-e2e override; Dev-deps in `viola-e2e`)
**Change:**
- `[profile.mutants]` holds four overrides, first match winning: `verify_window_` at `{ period = "20s", terminate-after = 3 }` (a 60 s kill, the class's `ci` kill) stands first, then `package(viola-e2e)`, `send_window_` and `send_under_the_paste_hint_`, each at `{ period = "15s", terminate-after = 2 }` (30 s). Was three overrides, `verify_window_` at 45 s.
- The paste-hint class holds the two `send_under_the_paste_hint_` cases, which wait out the product's gate under a held hint by design, 8.7–9.9 s on three OSes under `ci`. `mutants_paste_hint_override_holds_the_class_above_the_default_kill` pins its kill above the default.
- The no-screen `verify_window_` case reads 42.952 s alone on the dev host and 42.874 / 43.078 / 43.714 s on the CI legs, below the 60 s kill (was «34.3 s … below the 45 s kill»). The planted-hang controls: TIMEOUT 60.003 s / PASS 112.961 s for the class, TIMEOUT 30.003 s / PASS 48.413 s for the paste-hint class.
- The fail-fast kill list reads 10 s; viola-e2e 30 s; the `send_window_` and paste-hint classes 30 s; the `verify_window_` class 60 s. The viola-e2e harness case takes that class's 60 s kill. Run E's sentence reads «the class's 60 s kill (the `mutants` profile's too)».
- Dev-deps in `viola-e2e` add libc `=0.2.189` under `cfg(unix)`, for the keeper's `statvfs` headroom read.
**Why:** both floors are by design and measured across runs, so each kill moved with its planted-hang pair, never by raising the default; the replay contracts got faster instead (their sets run at once).
**Ref:** .andromeda/runs/2026-10-20T09-40-55-wrap/
