# Classified — design · surface.b2

## Answers
surface#15.1 · no · exit 0 success envelope and result line; the chunk changes no success output and no exit-0 cause
surface#15.2 · yes · exit 1 row: the fixed `error: internal error` form, the run and revive start refusals each with its own hint, revive's --json causes, and the no-document rule for the rest, all cases of this chunk's exit-cause table
surface#15.3 · no · exit 2 usage and fixed UNUSABLE lines, not a cause of exit 1 or 21; the chunk adds no exit-2 output
surface#15.4 · no · exit 10 send refusal, not a cause of exit 1 or 21; the task says send refusals are not re-opened
surface#15.5 · no · exit 11 budget-paused refusal, not a cause of exit 1 or 21; send refusals not re-opened
surface#15.6 · no · exit 12 unverified-cli refusal, not a cause of exit 1 or 21; send refusals not re-opened
surface#15.7 · no · exit 13 not-delivered refusals, not a cause of exit 1 or 21; send refusals not re-opened
surface#15.8 · no · exit 14 unknown refusal, not a cause of exit 1 or 21; send refusals not re-opened
surface#15.9 · no · exit 20 wrapper fault, a fault outside exit 1 and 21; link and unlink keep their exit-20 path unchanged
surface#15.10 · yes · exit 21 row: the instance-unreachable pair with the cause's hint, and the --json document whose per-cause detail code this chunk settles
surface#16.1 · no · link and unlink success documents at exit 0; the chunk changes no success output
surface#16.2 · no · exit-20 wrapper refusal of link and unlink, a fault the chunk does not reach
surface#16.3 · no · exit-20 detail closed set in viola_channel::sanitised_fault, which the chunk does not edit; its client.rs edit is the vanished-server read as exit 21
surface#16.4 · no · exit-20 fallback document for unknown wrapper codes, a fault the chunk does not reach
surface#16.5 · yes · the --json failure rule the chunk's new and moved documents must meet: one document, literal exit code, stderr empty, no ESC, path or upstream text, as tests/cli_error_bodies.rs asserts
surface#17.1 · yes · exit 21 server-verification cause for send, wait, last, answer, pause, release, link and unlink: no frame, the server-verification hint last on stderr, or the document with stderr empty
surface#18.1 · no · viola --help verb groups; the chunk adds and renames no verb
