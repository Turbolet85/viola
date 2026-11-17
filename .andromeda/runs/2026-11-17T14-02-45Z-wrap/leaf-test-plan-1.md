@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=119
last: - 2026-11-16: A CI red read at the operator pass that is not
@leaf_edit n=1 leaf=.claude/rules/testing.md line=23 class=omission
leaf-text: so its table-driven cases are literal tables in one `#[test]`, e.g. `dialog_hold_takes_whole_seconds_from_sixty_to_thirty_six_hundred`).
source: dec-dyqgj6cs «viola-e2e and viola-core carry no rstest dev-dependency; their table-driven cases, and the own cases of a shared helper taken by path, which run in every taker, are labelled tables in one test.»
replace-with: so its table-driven cases are literal tables in one `#[test]`, e.g. `dialog_hold_takes_whole_seconds_from_sixty_to_thirty_six_hundred`; and a shared helper taken by path keeps its own cases, which run in every taker, as a labelled table in one test).
@leaf_edit n=2 leaf=.claude/rules/testing.md line=19 class=omission
leaf-text: (rmcp client; a numbered Path binary is named `path<N>_<topic>.rs`)
source: dec-w5ivfz5r «No viola-e2e test boots a harness session untied: a census over viola-e2e's test sources finds every in-test boot tied, and is read red on a planted untied boot first.»
replace-with: (rmcp client; a numbered Path binary is named `path<N>_<topic>.rs`; no test boots a harness session untied: an in-test session ties its supervisor to the test thread by parent-death SIGKILL and its stamping `viola verify` by SIGTERM, and the census `viola_e2e_census_finds_no_untied_boot`, read red on a planted untied boot first, finds every in-test boot tied)
@leaf_edit n=3 leaf=.claude/rules/testing.md line=40 class=omission
leaf-text: a lower-layer case (parser, refusal order, threshold, clock) belongs in unit tests, not E2E.
source: dec-aqwevj4k «A kill witness for a tied stamping verify waits until verify holds a child inside a viola-verify run dir, asserts an owned dir stood at the kill, and reads the stamp's absence besides processes and dirs.»
replace-with: a lower-layer case (parser, refusal order, threshold, clock) belongs in unit tests, not E2E. A kill witness for a tied stamping verify (`killed_test_leaves_no_stamping_verify`) waits until verify holds a child inside a viola-verify run dir, asserts an owned dir stood at the kill, and reads the stamp's absence besides processes and dirs: an untied verify orphaned by the kill finishes inside the wait bound and removes its own dirs.
