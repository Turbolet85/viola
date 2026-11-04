@answer n=1 entity=data-protection#1.1 v=no
why: the session adds only gate-reason lines to owner-only and strict-modes tests and moves no src/ code, so the ~/.viola 0700/0600 and strict-modes rules still hold
@answer n=2 entity=data-protection#2.1 v=no
why: the session changes no pin_bytes or place_private_new code, and the no-clobber placement of the pinned copy is unchanged
@answer n=3 entity=data-protection#3.1 v=no
why: the session changes no place_private_new code, so a present pinned-copy name is still left untouched
@answer n=4 entity=data-protection#4.1 v=no
why: the session changes no pin_bytes re-hash on a lost placement, so the reuse and HashMismatch behaviour stands
@answer n=5 entity=data-protection#5.1 v=no
why: the session touches no build, pin-key or folder-keying code, so the stated reading of one tree's release builds is unaffected
@answer n=6 entity=data-protection#6.1 v=no
why: no link or unlink method, LinkSet call or events.ndjson line changed, and the session reaches no file that writes those lines
@answer n=7 entity=data-protection#7.1 v=no
why: the wrapper's link and unlink process-log and output behaviour is unchanged, since no link verb or wrapper method moved
@answer n=8 entity=data-protection#8.1 v=no
why: check_diagnostics and the diagnostics open paths are unchanged, and the session edits no diagnostics code
@answer n=9 entity=data-protection#9.1 v=no
why: fixtures/ is unchanged and the session records no live session, so the repository fixture and scrub rules hold
@answer n=10 entity=data-protection#10.1 v=no
why: verify's refusal codes are unchanged, and tests/cli_verify.rs gains only a reason line on verify_a_batch_script_refuses, naming no code
@answer n=11 entity=data-protection#11.0 v=no
why: the accepted residuals concern live verify and plugin-install rounds, and this session ran no live session, only the fake agent
