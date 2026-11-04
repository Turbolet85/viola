@answer n=1 entity=span-trace-coverage#5.1 v=no
why: the pin_copy placement is src code (pin_bytes and place_absent), no src file moved and the session changes no span or log field.
@answer n=2 entity=span-trace-coverage#6.1 v=no
why: the send path's spans and server-check wording live in src, no src file moved, and the macOS carve-out it names stands unchanged in behaviour with no count in it.
@answer n=3 entity=span-trace-coverage#6.2 v=no
why: the send span attributes are emitted by src code the session did not touch, so no required attribute moved.
@answer n=4 entity=span-trace-coverage#6.3 v=no
why: the send log fields are unchanged, no product field or line moved, and the carve-out wording stands while the 43-case count belongs to test-inventory text.
@answer n=5 entity=span-trace-coverage#6.4 v=no
why: the send cleanup and process-exit details are unchanged in src, and its carve-out wording still holds in behaviour.
@answer n=6 entity=span-trace-coverage#7.1 v=no
why: the wait and last must-trace spans are src behaviour the session did not change, and the macOS carve-out wording still holds.
@answer n=7 entity=span-trace-coverage#7.2 v=no
why: the wait span attributes are emitted by unchanged src code, so no attribute moved.
@answer n=8 entity=span-trace-coverage#7.3 v=no
why: the wait and last log fields are unchanged, no product line moved, and the macOS euid carve-out still holds in behaviour.
@answer n=9 entity=span-trace-coverage#7.4 v=no
why: the wait cleanup behaviour is unchanged src code, and the session moves no refusal path.
@answer n=10 entity=span-trace-coverage#8.1 v=no
why: the dialog spans and the server-check wording are src behaviour the session did not change, and the macOS carve-out still holds in behaviour.
@answer n=11 entity=span-trace-coverage#8.2 v=no
why: the dialog span attributes are emitted by unchanged src code, so no attribute moved.
@answer n=12 entity=span-trace-coverage#8.3 v=no
why: the dialog and answer log fields are unchanged, and no hook or driver line moved.
@answer n=13 entity=span-trace-coverage#8.4 v=no
why: the dialog expiry and cleanup behaviour is unchanged src code, and the session writes no dialog line.
@answer n=14 entity=span-trace-coverage#9.1 v=no
why: the wheel and pause or release spans are unchanged src behaviour, and the macOS carve-out wording still holds in behaviour.
@answer n=15 entity=span-trace-coverage#9.2 v=no
why: the run.wheel_transition span attributes are emitted by unchanged src code, so no attribute moved.
@answer n=16 entity=span-trace-coverage#9.3 v=no
why: the wheel and release log fields are unchanged, and no release or budget line moved.
@answer n=17 entity=span-trace-coverage#9.4 v=no
why: the wheel transition is a point-in-time src span the session did not change, and the wheel event audit is unchanged.
@answer n=18 entity=span-trace-coverage#10.1 v=no
why: the budget governor spans are src code the session did not touch, so no span moved.
@answer n=19 entity=span-trace-coverage#10.2 v=no
why: the budget span attributes are emitted by unchanged src code, so no attribute moved.
@answer n=20 entity=span-trace-coverage#10.3 v=no
why: the budget and statusline log fields are unchanged, and no hook or statusline line moved.
@answer n=21 entity=span-trace-coverage#10.4 v=no
why: the statusline shell-out cleanup and its deadline are unchanged src behaviour, so the claim stands.
@answer n=22 entity=span-trace-coverage#11.1 v=no
why: the E2 env strip is src behaviour with no env var or config key changed, so the claim stands.
@answer n=23 entity=span-trace-coverage#12.1 v=no
why: the E3 implicit link record is unchanged src behaviour, and no link or send line moved.
@answer n=24 entity=span-trace-coverage#13.1 v=no
why: the E3 explicit link and unlink methods are unchanged src behaviour, and no IPC method moved.
@answer n=25 entity=span-trace-coverage#14.1 v=no
why: the E5 state-recovered line is written by unchanged src replay code, and the session adds no list writer or replay line.
