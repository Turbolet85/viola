@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=120
last: - 2026-11-18: A test-support helper in a crate's `src/` (the
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=21 class=stale
leaf-text: reads a trust record from that config file by its own key rule (the nearest ancestor holding a git entry else the canonical cwd, through a bounded read; any failure reads untrusted)
source: ctr-kxqcdc74 «The fake agent reads trust from the named config file by the CLI's walk, the canonical cwd's exact key then each parent up to the git root, so inside a repository the cwd's own record counts, through a bounded read»
replace-with: reads trust from that config file by the CLI's walk (the canonical cwd's exact key then each parent up to the git root, so inside a repository the cwd's own record counts, through a bounded read; any failure reads untrusted)
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: every test-started `viola verify` and every harness boot stamping runs under a test-owned user home whose config file is seeded empty and owner-only,
source: dec-yexam4hk «decision/run-under-test-owned-user-home · Test runs and revives under a test-owned user home — Every test-started viola run and viola revive runs under a test-owned user home, never the runner's real HOME.»
replace-with: every test-started `viola verify`, `viola run` and `viola revive` and every harness boot stamping runs under a test-owned user home whose config file is seeded empty and owner-only, never the runner's real `HOME` (the harness's supervised wrappers of a stamped session too, under a session user home seeded with an empty config),
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=20 class=omission
leaf-text: viola creates it, except `tests/chaos_feed_panic.rs`'s
source: dec-bbcsxrjr «decision/trust-start-folders-outside-repository · Trust-start test folders outside any repository — Trust-start test folders sit under /tmp, outside any repository, an exception to test homes under target/e2e-home.»
replace-with: viola creates it (trust-start test folders sit under `/tmp`, outside any repository, since a folder inside the workspace keys at its git root: an exception to test homes under `target/e2e-home`), except `tests/chaos_feed_panic.rs`'s
@leaf_edit n=4 leaf=.claude/rules/testing.md line=34 class=omission
leaf-text: Every test-started `viola verify` and every harness boot stamping runs under a test-owned user home whose config file is seeded empty and owner-only,
source: dec-yexam4hk «decision/run-under-test-owned-user-home · Test runs and revives under a test-owned user home — Every test-started viola run and viola revive runs under a test-owned user home, never the runner's real HOME.»
replace-with: Every test-started `viola verify`, `viola run` and `viola revive` and every harness boot stamping runs under a test-owned user home whose config file is seeded empty and owner-only, never the runner's real `HOME` (the harness's supervised wrappers of a stamped session too, under a session user home seeded with an empty config),
@leaf_edit n=5 leaf=.claude/rules/testing.md line=33 class=omission
leaf-text: The one carve-out (the founder's ruling, 2026-10-04):
source: dec-bbcsxrjr «decision/trust-start-folders-outside-repository · Trust-start test folders outside any repository — Trust-start test folders sit under /tmp, outside any repository, an exception to test homes under target/e2e-home.»
replace-with: Trust-start test folders sit under `/tmp`, outside any repository (a folder inside the workspace keys at its git root), an exception to test homes under `target/e2e-home`. The one home carve-out (the founder's ruling, 2026-10-04):
