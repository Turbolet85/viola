@answer n=1 entity=occupied-resources#26.1 v=no
why: the check_home_tree rule lives in the product strict module under src, which the session did not change
@answer n=2 entity=occupied-resources#27.1 v=no
why: the instance-check order of the CLI frame verbs and hook paths is product code the session did not reach
@answer n=3 entity=occupied-resources#28.1 v=no
why: the Windows NotFound stat and its per-OS test oracle are an OS fact the test-only session leaves unchanged
@answer n=4 entity=occupied-resources#29.1 v=no
why: the list read rule lives in product board::scan under src, and the reached cli_list.rs test does not move that claim
@answer n=5 entity=occupied-resources#30.1 v=no
why: the home write rule for read_budget_stamped, mcp start and plugin install is product code under src that the session did not change
@answer n=6 entity=occupied-resources#31.1 v=no
why: the budget.json read and write path is product code under src, and no reached file moves its claim
@answer n=7 entity=occupied-resources#32.1 v=no
why: the ledger stamps writer and reader are product code under src, and the session changed test files only
@answer n=8 entity=occupied-resources#33.1 v=no
why: the verify probe directories are written by product verify code, which the session did not change
@answer n=9 entity=occupied-resources#34.1 v=no
why: verify's interactive probe dirs and their drop guards are product code the session did not change
@answer n=10 entity=occupied-resources#35.1 v=yes mode=stale
why: Frozen now polls under the root Watch bounded by WITHIN as a new user of its report file, and the root count of Instant::now() + WITHIN waits reads 32, not the recorded 28
@answer n=11 entity=occupied-resources#36.1 v=no
why: harness cleanup's endpoint_gone lives in crates/viola-e2e/src, which the session did not reach
@answer n=12 entity=occupied-resources#37.1 v=yes mode=add-to
why: F3 adds tests/support/frozen.rs, the shared Frozen guard, as a new test-only support module beside the events.rs error-body check
@answer n=13 entity=occupied-resources#37.2 v=yes mode=stale
why: the session reaches tests/cross_exit_causes.rs whole, and the report does not show its four named checks still hold on the changed file
@answer n=14 entity=occupied-resources#38.1 v=no
why: the tmpfs e2e-home backing on the dev host is outside the session's changes and no reached file touches it
@answer n=15 entity=occupied-resources#39.1 v=no
why: the test-owned socket roots in tests/support/home.rs are not in the reached files and the session changes no socket root
@answer n=16 entity=occupied-resources#40.1 v=no
why: the fixtures/claude recordings and their hygiene walk are untouched, and fixtures/ is not reached
@answer n=17 entity=occupied-resources#41.1 v=no
why: target/agent-run is the harness's own directory, and the operator script reads gh downloads into a temp dir outside the tree
@answer n=18 entity=occupied-resources#42.1 v=no
why: the target/e2e-home test homes and their stamping are unchanged, and the session adds no home or owner record
@answer n=19 entity=occupied-resources#43.1 v=no
why: the after-every-test-run env and if always in ci.yml are unchanged, since .github/ did not move
@answer n=20 entity=occupied-resources#44.1 v=no
why: seed_conpty and its icacls grant live in tests/support/home.rs, which the session did not reach
@answer n=21 entity=occupied-resources#45.1 v=yes mode=add-to
why: F1 adds scripts/junit-os-sets.py beside profraw-census.sh as a repository script that runs only in the operator pass and writes nothing in the tree
@answer n=22 entity=occupied-resources#46.1 v=no
why: the viola-mutants-scratch sibling directory is outside the session's changes
@answer n=23 entity=occupied-resources#47.1 v=no
why: the protected owner-only DACL reading on windows-2025 is a measured CI fact the session does not move
@answer n=24 entity=occupied-resources#47.2 v=no
why: the runner-inherited temp dir ACL facts on windows-2025 are not moved by a test-only change
