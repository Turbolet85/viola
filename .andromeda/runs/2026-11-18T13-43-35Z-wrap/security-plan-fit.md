@fit n=1 row=decision/start-trust-skips-trusted-folders sha=4aad6aecabc2 v=fits type=1
@fit n=2 row=decision/trust-record-writer-guarded sha=2cf1fd5d7241 v=fits type=1
@fit n=3 row=decision/revive-recorded-cwd-spawn-dir sha=f656a07fe7f2 v=fits type=1
@fit n=4 row=decision/revive-cwd-spawn-only sha=51ae96b00f6b v=fits type=1
@fit n=5 row=decision/cwd-only-in-snapshot sha=2b5d6aa86d03 v=fits type=1
@fit n=6 row=decision/spawn-cwd-snapshot-only sha=26433ed322bc v=fits type=1
@fit n=7 row=decision/nothing-written-under-claude-home sha=7e014516318d v=fits type=2
@fit n=8 row=decision/no-writes-under-claude-home sha=aa62b3b378b9 v=fits type=2
@fit n=9 row=decision/viola-writes-nothing-in-claude-dir sha=e84753dca9f2 v=fits type=2
@fit n=10 row=decision/credential-strip-by-name-only sha=c6dd15aaf5e0 v=fits type=2
@fit n=11 row=decision/env-not-config-channel sha=b0045f835300 v=fits type=2
@fit n=12 row=decision/no-stored-secrets sha=0a44db9c83f4 v=fits type=2
@fit n=13 row=decision/r8-claude-env-strip sha=c71ded43c8a0 v=fits type=2
@fit n=14 row=decision/start-writes-trust-record-flag-only sha=36318a46f0f3 v=fits type=1
@fit n=15 row=decision/trust-record-closed-words sha=8fa287428956 v=fits type=1
@fit n=16 row=decision/live-rig-seed-from-bundle-logic sha=7b5f9c907ae9 v=not_master type=4
why: The added sentinel project entry describes how the test rig checks the start, so it lengthens the rule with one more seeded item, and the rig's test code holds that.
