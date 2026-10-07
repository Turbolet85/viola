## 2026-10-06-local-command-send-outcomes — the read-back line's second trigger
**Section:** §Surface: cli → Signature placement (item 2, the outcome line)
**Change:** was "`[RB] read back …` on stdout with exit 0 when the matching `prompt-submitted` confirms the send"; now it prints when the send is confirmed, by the matching `prompt-submitted` or, for `/clear` on a verified CLI version, by its new-session post-condition (a `session-start` with cause `clear` and a new session id). The `unable` and `unconfirmable` clauses are unchanged.
**Why:** the chunk made `send` confirm `/clear` by its post-condition; the web statements of the same trigger already carried it.
**Kept:** the `viola send` wireframe's `clear.txt` example ending `not-delivered  input-not-ready` stays: the refusal rungs apply to a listed command as to any text.
**Ref:** .andromeda/runs/2026-10-06T23-49-33-wrap/
