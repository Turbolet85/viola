@answer n=1 entity=cognitive-accessibility#26.1 v=no
why: the --json failure documents sit in tests/cli_error_bodies.rs, which the session did not reach, and the claim names no OS, count or retired refusal
@answer n=2 entity=cognitive-accessibility#27.1 v=no
why: the error-writer census sits in tests/contract_error_bodies.rs, which the session did not reach, and the claim names no OS, count or retired writer
@answer n=3 entity=cognitive-accessibility#28.1 v=yes mode=stale
why: tests/cross_exit_causes.rs is reached and its catalogue moved (batch-script-child retired, revive codes 11 to 10, process-exit detail 20 to 19), so the one-hint-per-code checks can have moved and the report does not show them holding
@answer n=4 entity=cognitive-accessibility#29.1 v=yes mode=stale
why: tests/cross_exit_causes.rs is reached and the exit-cause table lost the retired batch-script-child cause while the detail enum moved 20 to 19, so its rows and the comment naming causes held outside the table can have moved
@answer n=5 entity=cognitive-accessibility#30.1 v=no
why: the body-check classes sit in tests/support/events.rs, which the session did not reach, and the four named classes are unchanged
@answer n=6 entity=cognitive-accessibility#31.1 v=yes mode=stale
why: its pins sit in reached files, the HINT literal in tests/security_negatives_server_identity.rs (gates made unconditional), the verify home-state pair in tests/cli_verify.rs (verify codes 6 to 5, shim row changed) and SOCKET_DIR in tests/cross_exit_causes.rs (socket-dir forms moved)
@answer n=7 entity=cognitive-accessibility#32.1 v=no
why: the link lines are pinned in src/human/verbs.rs and tests/cli_links.rs, neither of which the session reached, and the claim names no OS, count or retired item
@answer n=8 entity=cognitive-accessibility#33.1 v=no
why: the link time unit sits in src/human/verbs.rs, which the session did not reach, and the UTC rule names no OS, count or retired item
@answer n=9 entity=cognitive-accessibility#34.1 v=no
why: the CAPTION literal is byte-identical on Linux and its file tests/cli_list.rs was not reached
