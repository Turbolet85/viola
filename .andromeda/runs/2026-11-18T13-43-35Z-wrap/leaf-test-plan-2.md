@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/verification-harness.md lines=74
last: - 2026-11-17: A change to what the fake agent draws under `--
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=41 class=stale
leaf-text: `--trust-config <file>` reads a trust record from that config file by its own key rule (the nearest ancestor holding a git entry, else the canonical cwd) through a bounded read; any failure reads untrusted.
source: ctr-kxqcdc74 «The fake agent reads trust from the named config file by the CLI's walk, the canonical cwd's exact key then each parent up to the git root, so inside a repository the cwd's own record counts, through a bounded read»
replace-with: `--trust-config <file>` reads trust from that config file by the CLI's walk (the canonical cwd's exact key, then each parent up to the git root, so inside a repository the cwd's own record counts) through a bounded read; any failure reads untrusted.
@leaf_edit n=2 leaf=.claude/rules/verification-harness.md line=28 class=omission
leaf-text: it hands the fake agent `--screens --trusted-root <cwd>`)
source: dec-vky7scrr «Harness wrappers under a session user home — The harness's supervised wrappers of a stamped session run under a session user home seeded with an empty config.»
replace-with: it hands the fake agent `--screens --trusted-root <cwd>`; the supervised wrappers of a stamped session run under a session user home seeded with an empty config)
@leaf_edit n=3 leaf=.claude/rules/verification-harness.md line=42 class=omission
leaf-text: `every_test_verify_runs_under_a_test_home` holds every test-started `viola verify` start tied to a test-owned home
source: dec-yexam4hk «Test runs and revives under a test-owned user home — Every test-started viola run and viola revive runs under a test-owned user home, never the runner's real HOME.»
replace-with: `every_test_verify_runs_under_a_test_home` holds every test-started `viola verify` start tied to a test-owned home, and every test-started `viola run` and `viola revive` runs under a test-owned user home, never the runner's real HOME
@leaf_edit n=4 leaf=.claude/rules/verification-harness.md line=42 class=omission
leaf-text: Homes live under `target/e2e-home/viola-test-*`
source: dec-bbcsxrjr «Trust-start test folders outside any repository — Trust-start test folders sit under /tmp, outside any repository, an exception to test homes under target/e2e-home.»
replace-with: Trust-start test folders sit under `/tmp`, outside any repository; every other home lives under `target/e2e-home/viola-test-*`
