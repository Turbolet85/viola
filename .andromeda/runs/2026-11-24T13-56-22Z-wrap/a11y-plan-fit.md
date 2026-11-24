@fit n=1 row=contract/frame-verb-refusal/strict-modes-failed sha=20c6fbb32d0d v=fits type=3
@fit n=2 row=contract/frame-verb-refusal/server-verify-failed sha=4516626c9a26 v=fits type=3
@fit n=3 row=contract/cli-exit-codes/10 sha=b22eedef0216 v=fits type=3
@fit n=4 row=decision/send-refused-while-human-typing sha=5e1813355b70 v=fits type=1
@fit n=5 row=decision/respawn-reap-bound sha=13c5a3dc0aee v=fits type=1
@fit n=6 row=contract/viola-switch-output/result-line sha=f3e3714c36a1 v=fits type=3
@fit n=7 row=decision/switch-respawn-same-terminal sha=23b1f42cc293 v=not_master type=4
why: The basis only measures how the respawn is built (a new PTY pair swapped under the same host terminal) and states no reason for it, so the code holds it.
