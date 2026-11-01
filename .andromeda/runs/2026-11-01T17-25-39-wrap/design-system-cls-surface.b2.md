# Classified — design-system · surface.b2

## Answers
surface#14.2 · yes · stale · revive --json's closed codes move from ten to eleven with socket-dir-refused, the F4 document a socket-directory refusal prints.
surface#14.3 · yes · contradict · F4 reads a failed create of the socket directory as socket-dir-refused, so an anyhow error of revive's start no longer always prints internal-error.
surface#14.4 · no · pump_child runs after the child starts, and the new refusal comes before any snapshot, child or bind, so the no-document rule holds.
surface#15.1 · no · the exit 0 success row is unchanged and the exit set is unchanged.
surface#15.2 · yes · add-to · F1 adds the socket-directory `unable:` refusal with its hint to run and revive's start, and F4 adds socket-dir-refused to revive's --json column.
surface#15.3 · no · exit 2 lines are clap usage and send/answer input-size checks the change does not reach; the exit set is unchanged.
surface#15.4 · no · exit 10's human-typing refusal is untouched and the exit set is unchanged.
surface#15.5 · no · exit 11's budget-paused refusal is untouched and the exit set is unchanged.
surface#15.6 · no · exit 12's unverified-cli refusal is untouched and the exit set is unchanged.
surface#15.7 · no · exit 13's not-delivered causes are untouched and the exit set is unchanged.
surface#15.8 · no · exit 14's unknown refusal is untouched and the exit set is unchanged.
surface#15.9 · no · a refused peer is dropped silently, so the wrapper fault exit 20 gains no row or code.
surface#15.10 · yes · add-to · F3 adds the socket-dir cause: the eight verbs print instance-unreachable with the socket-dir hint, and socket-dir-refused joins the --json cause list.
surface#16.1 · no · exit 0's link and unlink documents are unchanged; no success path moves.
surface#16.2 · no · exit 20 on link/unlink is the wrapper's pair refusal, which the change does not touch.
surface#16.3 · no · the refused peer is a silent drop, so sanitised_fault's five known codes hold with no new code.
surface#16.4 · no · codes outside the five keep the same line and document; the change adds no wrapper code.
surface#16.5 · yes · add-to · F3 and F4 add --json failure documents (one document, stderr empty, naming only the ViolaName) to the group this rule describes.
surface#16.6 · no · the catch site's internal-error document is unchanged; the new refusals print their own documents and do not pass through it.
surface#16.7 · no · the catch site's counting of --json before -- is unchanged.
surface#16.8 · no · run and revive's start stay silent at the catch site; F4's document is printed by the start itself, and run's silence holds through the new refusal.
surface#17.1 · yes · stale · server-verify-failed now also covers a refused euid (on macOS an euid mismatch alone), so the pid-or-start-time reading no longer states its whole reach.
surface#17.2 · yes · stale · the new socket-dir check is a pre-frame step in the same verbs, so the no-endpoint instance-directory readings and their ordering after strict-modes can move; the report does not clear them.
surface#17.3 · yes · stale · F5's refused-peer drop gives the client end-of-file with the call open, and the endpoint-lost reading for a lost wrapper now has to place it; the report does not clear it.
surface#17.4 · yes · stale · the pre-frame socket-dir check now sits ahead of the stale-instance instance-dead reading; the report does not clear it.
