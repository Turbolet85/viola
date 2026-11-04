@answer n=1 entity=cognitive-accessibility#26.1 v=no
why: its test tests/cli_error_bodies.rs is not in the session's file set, and no --json failure document or exit changed
@answer n=2 entity=cognitive-accessibility#27.1 v=no
why: its census file tests/contract_error_bodies.rs is not in the session's file set, and no error writer or witness changed
@answer n=3 entity=cognitive-accessibility#28.1 v=no
why: tests/cross_exit_causes.rs is reached only by a reason line on two refusal tests, and no hint changed, so one hint per code and no hint naming release still hold
@answer n=4 entity=cognitive-accessibility#29.1 v=no
why: tests/cross_exit_causes.rs is reached only by a reason line on two refusal tests, and the table's exits, documents and hints are unchanged
@answer n=5 entity=cognitive-accessibility#30.1 v=no
why: tests/support/events.rs is not in the session's file set, since only the Frozen guard moved into tests/support/frozen.rs
@answer n=6 entity=cognitive-accessibility#31.1 v=no
why: the pinned literals in the reached files cli_list.rs, cli_plugin_install.rs, cli_verify.rs and run_cli.rs are unchanged, as no CLI string changed and the files gained only OS-gate reason lines
@answer n=7 entity=cognitive-accessibility#32.1 v=no
why: tests/cli_links.rs and src/human/verbs.rs are not in the session's file set, and the link lines are unchanged
@answer n=8 entity=cognitive-accessibility#33.1 v=no
why: src/human/verbs.rs is not in the session's file set, and the link time and name rules are unchanged
@answer n=9 entity=cognitive-accessibility#34.1 v=no
why: the CAPTION literal in tests/cli_list.rs is in a reached file, but only a reason line was added there and the caption order NAME, LIVE, STATUS, WHEEL, DIALOG, CLI is unchanged
