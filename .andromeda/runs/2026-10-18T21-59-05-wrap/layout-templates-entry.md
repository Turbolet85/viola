
## 2026-10-18-compact-post-condition-ledger-row — verify's twenty-four step lines; /compact in the readback outcome line
**Section:** §Surface: cli → Signature placement (item 2); → Output structure — `viola verify`
**Change:** item 2: on a verified CLI `/clear` is read back by its new-session post-condition and `/compact` by its compacted-session one (a `session-start` of cause `compact` with the session's own id, inside its own 40 s PROVISIONAL window, else `not-delivered` / `no-prompt-submitted`); `[  ] unconfirmable …` for `/remote-control` and for `/compact` or `/clear` on an unverified CLI. The verify wireframe shows 24 relied rows (`[NN/24]`, the new last line `[24/24] local-command-compact …`), `stamped 2.1.287  24 pass  0 fail`, the failing example `22 pass  2 fail`; the Usage list ends `local-command-compact`, `MM` 24 / 21. Was: `/clear` alone confirmed by a post-condition; 23 rows, `21 pass  2 fail`, `MM` 23 / 20.
**Why:** the chunk built the compact post-condition and the 24th row (report Symbols / APIs, Retired, Outcome).
**Ref:** .andromeda/runs/2026-10-18T21-59-05-wrap/
