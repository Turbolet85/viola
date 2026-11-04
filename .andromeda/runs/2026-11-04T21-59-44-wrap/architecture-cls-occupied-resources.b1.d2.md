@answer n=1 entity=occupied-resources#1.1 v=no
why: the socket-directory rule lives in product src and the session moved no product file or endpoint, so the path claim holds
@answer n=2 entity=occupied-resources#2.1 v=no
why: the force-killed socket-file claim is product behaviour the session leaves alone, and its cleanup test change only moves which OS runs it
@answer n=3 entity=occupied-resources#3.1 v=no
why: the macOS no-peer-pid gap is unchanged, and the 43-case carve-out count is not stated in this entity
@answer n=4 entity=occupied-resources#4.1 v=no
why: the fake agent's argv options, env and print mode are unchanged in src/bin, and the SIGHUP reader move is test-side only and not named here
@answer n=5 entity=occupied-resources#5.1 v=no
why: the subcommands and their exit codes are unchanged, no src file moved and the CLI surface is untouched
@answer n=6 entity=occupied-resources#6.1 v=no
why: the link and unlink exit codes and result shapes are unchanged, no product symbol moved
@answer n=7 entity=occupied-resources#7.1 v=no
why: the send, wait, last, answer, pause and release refusal exits are unchanged, and the session touched only test gates
@answer n=8 entity=occupied-resources#8.1 v=no
why: the hidden hook and statusline capture flags are unchanged, and no hook contract file moved
@answer n=9 entity=occupied-resources#9.1 v=no
why: the child flags and the SETTINGS_FLAG constant live in viola_agent_claude src, which the session does not touch
@answer n=10 entity=occupied-resources#10.1 v=no
why: the verify probe plugin and its five interactive runs are unchanged, and the session moved no verify product code
@answer n=11 entity=occupied-resources#11.1 v=no
why: the per-session settings override is written by the run path in src, which the session does not change
@answer n=12 entity=occupied-resources#12.1 v=no
why: VIOLA_DIR's read and derivation are product src behaviour the session leaves unchanged
@answer n=13 entity=occupied-resources#13.1 v=no
why: the hook's VIOLA_DIR instance check is product src, unchanged by the session
@answer n=14 entity=occupied-resources#14.1 v=no
why: the CLAUDE* strip and identity floor live in the agent crate and config handling, both unchanged, and the session adds no env name
@answer n=15 entity=occupied-resources#15.1 v=no
why: the harness-only env names are read by harness code the session leaves unchanged, and tests/support/home.rs, the reader of AGENT_RUN_KEEP_HOMES, is not among the reached files
@answer n=16 entity=occupied-resources#16.1 v=no
why: the two fake-agent test seams live in product src behind the fake-agent feature, unchanged, and the test that uses one keeps its seam
@answer n=17 entity=occupied-resources#17.1 v=no
why: the config.json keys and read rules are product src behaviour the session does not change
@answer n=18 entity=occupied-resources#18.1 v=no
why: the pinned bin copy placement is product src, and the session moved no pin code
@answer n=19 entity=occupied-resources#19.1 v=no
why: the diagnostics role files and their producers are product src, and the session adds no diagnostics line or path
@answer n=20 entity=occupied-resources#20.1 v=no
why: the channel-response detail enum is product schema and wire behaviour the session leaves unchanged
@answer n=21 entity=occupied-resources#21.1 v=no
why: the verify refusal codes are product src the session does not touch
@answer n=22 entity=occupied-resources#22.1 v=no
why: the state-recovered lines come from product replay readers the session does not change
@answer n=23 entity=occupied-resources#23.1 v=no
why: open_role_file's strict check is src/obs.rs, which the session does not touch
@answer n=24 entity=occupied-resources#24.1 v=no
why: the instance file set and its entry-point checks are product src the session leaves unchanged, and the new test-only Watch report is outside the home
@answer n=25 entity=occupied-resources#25.1 v=yes mode=stale
why: the session's windows-2025 CI reading now stands on ci#41918694243 (cbdd3874, JUnit 3386), so the cited run id ci#40439188053 has moved
