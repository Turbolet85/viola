@fit n=1 row=decision/paste-bound sha=b3e1fc0d5a49 v=fits type=1
@fit n=2 row=decision/paste-cut-mark sha=5c1f3c5e79ba v=fits type=1
@fit n=3 row=decision/one-pty-writer-thread sha=6647670fcd76 v=not_master type=4
why: How the PTY writer is built (one worker thread, a queue of 8, catch_unwind), with no reason stated in the basis, is a mechanism the code holds.
