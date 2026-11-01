# Classified — architecture · occupied-resources.b2

## Answers
occupied-resources#26.1 · yes · add-to · the socket-dir refusal joins each entry point's refusal list after its instance check (run and revive starts exit 1, frame verbs and link/unlink exit 21, MCP isError, hook's WARN line); the instance-check refusals stand as written
occupied-resources#27.1 · no · the strict-modes rules stand (only board's use of them moved), and the new socket check is Unix-only, so the Windows NotFound reading never reaches it
occupied-resources#28.1 · yes · contradict · F10: board::scan's home check moved to the read rule on every OS, so the write-rule disagreement with check_instance and its CARRY reconciliation no longer hold as written
occupied-resources#29.1 · no · the budget read order is unchanged and read_budget_stamped still holds the home to the write rule (F10); budget.rs is not reached
occupied-resources#30.1 · no · viola_state::stamps is not reached, and the strict checks it calls (check_stamps, read_stamps_strict) stand; only board's use of the rules moved
occupied-resources#31.1 · no · verify's probe dirs and captures are written by verify, which the session does not reach
occupied-resources#32.1 · no · verify's interactive probe dirs and their cleanup are not reached
occupied-resources#33.1 · yes · add-to · F13: a force-killed wrapper's socket file stays in the per-user directory until a next bind takes it over, a recorded choice; F14: harness cleanup's endpoint_gone on Unix reads a refused connect or an absent path, beside holder_gone
occupied-resources#34.1 · no · the error-body check in tests/support/events.rs is not reached; the reached error-body tests only gain the new word
occupied-resources#34.2 · no · the file is still the exit-cause table with its four checks; the new cause row is not a count this entity states
occupied-resources#35.1 · yes · add-to · F15: test-only socket roots under TMPDIR or XDG_RUNTIME_DIR (tests/support/home.rs SocketRoot) join this record of test-only /tmp resources; viola never names them
occupied-resources#36.1 · no · no fixture or fixture set moved; the reached files add no hook payload, screen or statusline fixture
occupied-resources#37.1 · no · the harness session and artifact files and the five perf exports are unchanged; the cleanup change reads endpoints, not these files
occupied-resources#38.1 · no · prepare_home_base, the owner record and remove_owned are not among what changed in tests/support/home.rs, which gained only the socket-root helpers
occupied-resources#39.1 · no · the CI workflow ci.yml is not reached and no CI step changed
occupied-resources#40.1 · no · seed_conpty still makes the home and seed dir through create_private_dir, which gains test callers only
occupied-resources#41.1 · no · scripts/profraw-census.sh and its inputs are not reached
occupied-resources#42.1 · no · the harness mutation scratch guard and its create_private_dir use are not reached or changed
occupied-resources#43.1 · no · the Windows DACL of a home made by create_private_dir is unchanged; the new socket checks are Unix-only
occupied-resources#43.2 · no · the Windows temp dir's inherited ACL is the host's, and the session changes no Windows check (the socket checks return Ok off Unix)
