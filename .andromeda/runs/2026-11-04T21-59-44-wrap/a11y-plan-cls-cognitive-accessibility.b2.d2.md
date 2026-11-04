@answer n=1 entity=cognitive-accessibility#26.1 v=no
why: its pinned case tests/cli_error_bodies.rs is not in the session's changed files and cites none of the moved refusal cases
@answer n=2 entity=cognitive-accessibility#27.1 v=no
why: its error-writer census in tests/contract_error_bodies.rs is not in the session's changed files
@answer n=3 entity=cognitive-accessibility#28.1 v=no
why: tests/cross_exit_causes.rs gained only a reason line on two strict-mode cases, and no hint, exit code or code word changed, so the no-shared-hint and no-release claims still hold
@answer n=4 entity=cognitive-accessibility#29.1 v=no
why: the exit-cause table rows and their exit, document and hint checks are unchanged, the session only added a reason line to two strict-mode cases in the same file
@answer n=5 entity=cognitive-accessibility#30.1 v=no
why: tests/support/events.rs, which holds the body-check helpers, is not in the session's changed files and tests/support/mod.rs gained no change to those helpers
@answer n=6 entity=cognitive-accessibility#31.1 v=no
why: every literal it cites still pins the same string, since no CLI string changed and the session only added reason lines to tests/cli_list.rs, tests/cli_plugin_install.rs, tests/cli_verify.rs and tests/run_cli.rs
@answer n=7 entity=cognitive-accessibility#32.1 v=no
why: the viola link and unlink result lines and tests/cli_links.rs are not in the session's changed files
@answer n=8 entity=cognitive-accessibility#33.1 v=no
why: the link time unit in src/human/verbs.rs is untouched and no src/ file moved
@answer n=9 entity=cognitive-accessibility#34.1 v=no
why: the CAPTION literal in tests/cli_list.rs still pins the same caption order, since no caption changed and the session added a reason line only to a different cli_list case
