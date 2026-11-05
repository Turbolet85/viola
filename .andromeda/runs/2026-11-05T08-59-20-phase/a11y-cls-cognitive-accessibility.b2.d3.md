@answer n=1 entity=cognitive-accessibility#26.1 v=no
why: the --json failure body check sits in tests/cli_error_bodies.rs, a file the chunk does not reach, and its Linux documents stay as they are
@answer n=2 entity=cognitive-accessibility#27.1 v=no
why: the error writer and witness census sits in tests/contract_error_bodies.rs, a file the chunk does not reach, and the scope names no writer or witness in it
@answer n=3 entity=cognitive-accessibility#28.1 v=no
why: the one-hint-per-code check in tests/cross_exit_causes.rs is not reached, and removing Windows-only causes keeps each remaining code on its own hint
@answer n=4 entity=cognitive-accessibility#29.1 v=no
why: the exit-cause table in tests/cross_exit_causes.rs is not reached, and its Linux cause rows keep their exit, document and hint
@answer n=5 entity=cognitive-accessibility#30.1 v=no
why: the body-check helpers in tests/support/events.rs are not reached, and their absolute-path, caused-by, serde-path and planted classes do not move on Linux
@answer n=6 entity=cognitive-accessibility#31.1 v=yes
why: the help string, the refusal pairs and the server-verify hint are pinned literally, so a Windows or macOS word the scope drops from any of them must update its pin on purpose
@answer n=7 entity=cognitive-accessibility#32.1 v=no
why: the link and unlink result lines carry no Windows or macOS word, and their one-write, no-SGR pins sit in src/human/verbs.rs and tests/cli_links.rs, neither reached
@answer n=8 entity=cognitive-accessibility#33.1 v=no
why: the link time as UTC whole seconds and the verb's parsed names carry no OS word, and the chunk changes neither
@answer n=9 entity=cognitive-accessibility#34.1 v=no
why: the viola list caption order and its CAPTION literal do not move, which is the task's not-needed item 5
