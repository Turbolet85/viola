# Classified — test-plan · test-data-fixtures.b1

## Answers
test-data-fixtures#1.1 · yes · stale · the Windows plain-home refused entry is no longer always the Users S-1-5-32-545 write ACE (runner-dependent, measured), and the plain-home oracle now reads any inherited untrusted write grant by its right
test-data-fixtures#1.2 · no · its widened-home claim names no SID, and the strict.rs change is a test-only win::tests helper, so the widened-tree refusal and seed_conpty hold unchanged
test-data-fixtures#1.3 · no · no test or harness code starts writing ledger/stamps.json, the Path 7 MCP leg stays unstamped, and stamped_home is unchanged
test-data-fixtures#1.4 · no · budget.json, its statusline-arm writer and its budget.rs reader cases are not reached by the placement or the oracle change
test-data-fixtures#1.5 · no · the statusline source is planted by the test and read by viola, and the session touches neither plant_statusline_source nor the statusline command
test-data-fixtures#1.6 · no · the widen, grant_read, unwiden and still_widened setters in tests/support/home.rs are unchanged, the session's DACL change being the plain-home oracle's own test-only helper
test-data-fixtures#1.7 · no · the fake agent's agents --json answer and its test-scoped PATH rule are unchanged, its only change being the hook receipt's spawn_os_error
test-data-fixtures#1.8 · no · no fixture or fixture set was added or changed, so the fake agent's fixture reading and the 34-file count of the 2.1.287 set hold
test-data-fixtures#1.9 · no · the Mcp fixture set and its replay in mcp_recorded_exchange.rs are unchanged, the session's MCP change being in mcp_tools.rs, a different test
test-data-fixtures#1.10 · no · the trycmd cases and the human-mode literals in src/human/verbs.rs and tests/cli_list.rs are not reached and no output changed
test-data-fixtures#1.11 · no · the insta decision bodies are not reached and no decision output changed
test-data-fixtures#1.12 · no · the byte comparison through the hook receipt's stdin_hex still holds, since the receipt gains only spawn_os_error beside it
test-data-fixtures#1.13 · no · the proptest strategies and proptest-regressions are not reached by the change
test-data-fixtures#1.14 · no · the forced-window case takes its window through the placed hook rather than a clock, and no manifest or dependency changed, so the mock_instant and start_paused claims hold
test-data-fixtures#2.1 · no · the impostor cases write their own snapshot and hold the fake agent's holder, and the session changes neither
test-data-fixtures#2.2 · no · the plain-home claim names no SID so the report keeps it unmoved, and create_private_dir is unchanged (fs.rs only gains place_private_new)
test-data-fixtures#3.1 · no · the fake-agent copies' ETXTBSY is another writer's busy text file, a claim the report holds unmoved by the pinned-copy fix
test-data-fixtures#4.1 · no · an absolute command is still spawned directly with its args and the non-absolute receipt is unchanged, the failed-spawn field belonging to the receipt list in test-data-fixtures#5.1
test-data-fixtures#5.1 · yes · add-to · F3 adds spawn_os_error to the hook receipt (the raw OS error as an integer, or null) on a failed spawn of an absolute command, a field this list does not hold yet, and the statusline receipt stays unmoved and takes no such field
test-data-fixtures#6.1 · no · the fake agent's argv options and seams are unchanged, the placed hook being a private function parameter rather than an option or seam, and the Path 7 sentence names no instance count
test-data-fixtures#7.1 · no · the fixture hygiene walk and its scrub rule are unchanged, since no fixture, fixture set or scrub rule was added or changed
