@answer n=1 entity=cognitive-accessibility#26.1 v=no
why: its check lives in tests/cli_error_bodies.rs, which the session does not reach, and the measured 3383 passed 0 failed holds its claim
@answer n=2 entity=cognitive-accessibility#27.1 v=no
why: the census lives in tests/contract_error_bodies.rs, which the session does not reach, and the measured 3383 passed 0 failed shows its writers and witnesses still stand
@answer n=3 entity=cognitive-accessibility#28.1 v=no
why: tests/cross_exit_causes.rs is reached, but its one-hint-per-code and no-release claims hold on the changed file as the measured 3383 passed 0 failed shows
@answer n=4 entity=cognitive-accessibility#29.1 v=no
why: the exit-cause table in tests/cross_exit_causes.rs still checks each remaining cause in human and --json modes, as the measured 3383 passed 0 failed shows
@answer n=5 entity=cognitive-accessibility#30.1 v=no
why: tests/support/events.rs is not in the reached set and its body classes name no OS, count or removed code
@answer n=6 entity=cognitive-accessibility#31.1 v=no
why: the session changed none of the pinned copy literals it names, and the measured 3383 passed 0 failed shows each literal still matches
@answer n=7 entity=cognitive-accessibility#32.1 v=no
why: the link and unlink result lines in src/human/verbs.rs and tests/cli_links.rs are outside the reached set and the session changed no link copy
@answer n=8 entity=cognitive-accessibility#33.1 v=no
why: the link time and name rule in src/human/verbs.rs is outside the reached set and the session moved neither the time form nor the name source
@answer n=9 entity=cognitive-accessibility#34.1 v=no
why: the CAPTION literal and caption order stay byte-identical on Linux per the session report, and tests/cli_list.rs is not reached
