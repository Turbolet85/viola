@row_set n=1 row=decision/identity-floor-always-stripped cell=rule seen=1 fact=F1 detector=none
value: Every identity-floor name is always removed whatever the persistent set says; CLAUDE_PLUGIN_ROOT, CLAUDE_PROJECT_DIR, CLAUDE_ENV_FILE and CLAUDE_PLUGIN_DATA are known non-identity hook names that the floor never strips, read only by the identity-floor ledger row.
basis: fct-yw2wvymn
because: The floor grew by two names and a compile-time list of four known non-identity hook names now stands beside it, so the row's count and its «no compile-time list of known names beyond the floor» are false.
@row_set n=2 row=decision/env-names-only cell=rule seen=1 fact=F2 detector=none
value: The strip, every log, the hook capture arm's env-names file and the verify stamp and record carry environment variable names only, never values, and no env map is passed to a span or event.
basis: fct-yw2wvymn
because: The capture arm now files an env-names document on SessionStart and verify stamps env_probe from it, both names only, so the names-only rule must cover these two new records.
@row_add n=1 row=decision/kept-env-names-never-stamped area=4-span-trace-coverage fact=F3 detector=none
name: Kept env names never stamped or recorded
rule: The identity-floor env reading drops every kept CLAUDE name before it reaches the verify stamp or a recorded fixture, so no kept name reaches either.
guard: kept_name_reaches_no_stamp_or_record
basis: fct-pu7enr4k
because: Verify's identity-floor row now stamps env_probe from the capture arm's names, and the chunk rules that kept names are removed before the stamp and the record.
@row_add n=2 row=decision/env-fixtures-names-only area=8-pii-scrubbing-compliance fact=F4 detector=none
name: Recorded env fixtures hold names only
rule: A recorded env fixture holds variable names only, never a value, passes the record scrub and an env-name fault check, and an env name carrying the user word is refused with nothing recorded.
guard: env_fixtures_pass_hygiene; record_refuses_an_env_name_carrying_the_user_word
basis: fct-y2ah5a4y
because: The chunk adds a names-only env fixture class with its own scrub and user-word refusal, a PII control no standing row states.
