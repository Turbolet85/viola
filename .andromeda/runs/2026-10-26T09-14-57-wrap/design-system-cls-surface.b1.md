# Classified — design-system · surface.b1

## Answers
surface#1.1 · no · no SGR or depth rule changed, and the new hint is plain ASCII with no SGR, so the DIALOG-only amber rule holds on the changed files
surface#1.2 · no · no SGR or depth rule changed, and list's stale rows stay as they were since list writes no frame
surface#1.3 · no · no SGR rule changed, and NAME's bold belongs to list's board, which the report leaves unchanged
surface#1.4 · no · no SGR rule changed, and the reset still follows every styled span, the new hint being plain ASCII
surface#1.5 · no · the session reaches no link output and no colour rule, so the handoff blue stays unused
surface#1.6 · no · the change sets no background, and the new hint is plain ASCII with no SGR
surface#2.1 · no · the stream rule holds on the changed files: the new refusal prints its hint last on stderr and none under --json, asserted for both modes
surface#3.1 · no · the width read and the unwrapped-name cut belong to list, which the report leaves unchanged
surface#4.1 · no · list writes no frame and the report leaves its board columns unchanged, so the as-built board holds on the changed files
surface#4.2 · no · list's strict-modes refusal is unchanged and list writes no frame, so the server-verify refusal is not its
surface#5.1 · yes · add-to · the server-verify cause is now built for send, wait, last, answer, pause and release, its shared hint plain ASCII with no pid, start time, path or endpoint (F1, F2), and it cannot print on macOS (F3)
surface#6.1 · no · the release --budget success line is unchanged and the new server-verify refusal precedes it, so the phraseology holds on the changed release.rs
surface#7.1 · no · verify's step lines, summary and stop line are unchanged, the report leaving cli-verify-run as it was
surface#8.1 · no · revive's refusal lines and --list output are unchanged, and run's collision check does not use the new verification
surface#9.1 · no · exit 0 keeps its meaning and its streams, and only exit 21 gains a cause
surface#9.2 · no · exit 1's codes and phraseology are unchanged, and only exit 21 gains a cause
surface#9.3 · no · the clap usage text and exit 2 are unchanged, and no verb is added
surface#9.4 · no · the human-typing refusal under exit 10 is unchanged, the new refusal coming before any frame and taking exit 21
surface#9.5 · no · the budget-paused refusal under exit 11 is unchanged, and only exit 21 gains a cause
surface#9.6 · no · the unverified-cli refusal under exit 12 is unchanged, and only exit 21 gains a cause
surface#9.7 · no · the not-delivered refusals under exit 13 are unchanged, and only exit 21 gains a cause
surface#9.8 · no · the unknown refusal under exit 14 is unchanged, and only exit 21 gains a cause
surface#9.9 · no · exit 20 and its wrapper-fault document are unchanged, and the new refusal is exit 21 rather than a fault
surface#9.10 · yes · add-to · the server-verify cause beside not-running is now built for send, wait, last, answer, pause and release, with an unchanged --json document (F1, F2), and no hint prints on macOS (F3)
surface#10.1 · no · no verb is added or removed and mcp stays hidden, so the help groups are unchanged
