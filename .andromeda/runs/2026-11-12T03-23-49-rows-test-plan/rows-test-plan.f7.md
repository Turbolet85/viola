@row_add n=1 row=decision/env-clear-keeps-profile-file area=11-test-anti-patterns-never-do-these rec=1,2
name: Cleared environment keeps LLVM_PROFILE_FILE
rule: A test that calls `.env_clear()` on a child process always re-adds `LLVM_PROFILE_FILE`.
why: Without it the run silently drops out of coverage.

@row_add n=2 row=decision/compact-confirm-window area=e2e-test-strategy rec=3,4,5
name: Compact confirmation window
rule: `/compact` on a verified CLI awaits its post-condition inside its own window, `COMPACT_CONFIRM_WINDOW`, of 40 s, clock-driven; a missed post-condition is `not-delivered`/`no-prompt-submitted` with the cursor.
guard: send_compact_confirm_window_is_its_own_bound
except: `CONFIRM_WINDOW_FALLBACK` stays 10 s for every other confirmed send.
state: provisional

@row_add n=3 row=contract/events/human-input-wheel area=e2e-test-strategy rec=6,7
boundary: events.ndjson
item: human-input
promise: Human editing input logs a `wheel{holder:"human",cause:"human-input"}` record.

@row_add n=4 row=contract/events/harness-prompt area=e2e-test-strategy rec=6,7
boundary: events.ndjson
item: harness-injected turn
promise: A harness-injected turn logs `prompt-submitted{origin:"harness"}` and no `wheel` record.

@row_add n=5 row=decision/five-hour-pause-threshold area=e2e-test-strategy rec=9,11
name: Five-hour pause threshold at 90
rule: A `five_hour` reading of exactly 90 pauses `send` (exit 11, `detail:"five-hour"`), and 89 does not.
guard: path6_the_boundaries_and_the_window_word
