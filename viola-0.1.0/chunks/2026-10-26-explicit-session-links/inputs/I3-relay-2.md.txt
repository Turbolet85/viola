# P4 fork — the widening's form for `viola link` / `viola unlink` (asked by dialog, answered live)

## The question, as shown

Boundary widening (playbook "Boundary widening — what ratifies it"): serving `viola link` / `viola unlink` means the wrapper serves two more channel methods, `link` and `unlink` (crossing a). Since your 2026-10-13 «Не делать до Эпохи 6», "Server verification before any frame" has landed. Every CLI frame verb now checks the serving process's pid and start time before its first frame (Linux and Windows; macOS is the recorded gap), so crossing (b) is closed for these verbs too. Only (c) remains: the snapshot read without the strict-modes check. Which form do you rule for `viola link` and `viola unlink`?

## The options, in order (the recommendation marked)

1. **Instance check first (Recommended)** — Your original second option, re-stated against HEAD. The wrapper serves `link` / `unlink`. Before opening any log or reading the snapshot, the two verbs run `check_instance` on the home and the driven instance, as `viola revive` does. Then they take the liveness pre-check and the server check like `send`. A refused check exits 21 (`instance-unreachable`; human mode prints the instance-state pair), opens no log and writes no line. It takes no dated gap. Why recommended: it closes (b) and (c) for these two verbs, and it takes none of "Home and code-bearing file integrity"'s work for the other eight clients. Residual: the check is proven on Unix only (its Windows DACL case is owed to "Home and code-bearing file integrity"), and the macOS server check stays the recorded gap.
2. **Ninth dated gap** — Your original first option, re-stated against HEAD. The wrapper serves `link` / `unlink`. The verbs take the liveness pre-check and the server check like `send`, but read `snapshot.json` without the strict-modes check, as a ninth dated gap of their own (borrowing none of the eight) until "Home and code-bearing file integrity". Trade-off: it mirrors the other six frame verbs exactly, and that entry later closes all eight gaps together. Until then, a snapshot in a home another user can write is read and used.
3. **Not yet** — Your original third option, re-stated: its literal «before Epoch 6» is spent, since this entry now sits in Epoch 6. No explicit verbs until after "Home and code-bearing file integrity" lands. This chunk builds nothing, the methods keep answering `-32601`, and the entry moves behind that one. Trade-off: no new crossing at all, but the `unlink` product path, test-plan E3 steps 1 and 3, and v1-13's `link` half stay owed.

## The answer

Selected: "Instance check first (Recommended)"

Notes, as given: The founder's own word, live by dialog in the overseer session at 2026-10-26T14:41Z: «Сначала instance check (Рекомендую)», shown the three forms re-stated against HEAD (instance check first · ninth dated gap · not yet). This is the boundary widening's ratification for `viola link` / `viola unlink`.
