# Classified — architecture · standard-contracts.b1

## Answers
standard-contracts#1.1 · no · the list payload gained no field, and the task's Untouched list says the list and board show no link
standard-contracts#2.0 · no · the snapshot envelope gained no field, and replay still recovers links from the link and unlink lines the explicit verbs now write
standard-contracts#3.0 · no · the stamps envelope, its merge and the relied row set are unchanged, and no ledger row was added or relied on
standard-contracts#4.1 · yes · add-to · F3 and F4: explicit link and unlink now change the set too, each answering only after its line and the snapshot write, and a failed record leaves the set unchanged
standard-contracts#5.1 · yes · stale · link and unlink, now served, also run the server check before their first frame, and this enumerated list of frame-writing methods names neither
standard-contracts#6.1 · no · send is not changed, and the held-pair no-op it relies on still holds on the shared LinkSet, as F1 shows for the explicit link
standard-contracts#7.1 · no · link and unlink are not WAIT_WAKE kinds, so their records never wake a wait, whatever path appends them
standard-contracts#8.1 · no · hook.event's validation, append and server check are unchanged, and the task says hook transport and the hook contract were not reached
standard-contracts#9.1 · no · the list is not a channel method, its ok payload and its scan, read and join order are unchanged, and the task says the list and board show no link
standard-contracts#10.1 · no · prompt-submitted's text and origin rules are unchanged, and the session adds no event kind or field
standard-contracts#11.1 · no · the statusline wrapper was not reached, as the task's Untouched list states
standard-contracts#12.1 · no · the server identity read is stated by class for every frame-writing client, and the link verbs take the liveness pre-check and then the server check
standard-contracts#13.1 · no · link and unlink run no claude agents read, as the task's verb sequence shows, and no ledger row or claude behaviour is involved
