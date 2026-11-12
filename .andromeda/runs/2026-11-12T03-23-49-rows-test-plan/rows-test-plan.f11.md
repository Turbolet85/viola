@row_add n=1 row=decision/local-command-unconfirmable area=unit-test-strategy rec=1
name: Local command without a post-condition is unconfirmable
rule: A local command with no measured post-condition, or sent to an unverified CLI, gives unconfirmable with no confirmation window.
guard: …_without_a_measured_post_condition_is_unconfirmable

@row_add n=2 row=decision/clear-confirmed-by-new-session area=unit-test-strategy rec=1
name: /clear confirmed by a new session
rule: /clear on a verified CLI is confirmed only by a session-start with cause clear and a new session id, never by a prompt-submitted or another session-start; its window expiry gives no-prompt-submitted with the cursor.
guard: …_clear_is_confirmed_by_a_new_session

@row_add n=3 row=decision/compact-confirmed-by-same-session area=unit-test-strategy rec=1
name: /compact confirmed by its same session
rule: /compact on a verified CLI is confirmed only by a session-start with cause compact and the remembered same session id; another cause, another id, no id, nothing remembered or a prompt-submitted claims nothing.
guard: …_compact_is_confirmed_by_its_same_session

@row_add n=4 row=decision/e2e-home-headroom-floor area=test-data-bootstrap rec=4,2,3
name: Headroom floor of the e2e home base
rule: A linked e2e home base whose filesystem reads less than the 9 GiB headroom floor free is refused; the floor is one compiled constant shared by the harness keeper and the test home, set by no env, variable, flag, feature or config key; a plain base reads no headroom.
why: The tmpfs's EDQUOT reading of about 8 GiB free plus 1 GiB.
guard: harness::tests::e2e_home_*
