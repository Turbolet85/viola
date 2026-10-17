P4 forks of chunk 2026-10-17-cli-machine-contract, asked in one dialog round; the answers as returned.

Q1 (Revive doc): What machine document should `viola revive --json` print for its exit-1 refusals? There is no channel result for them; architecture's `error` documents cover only exits 20 and 21.
  1. error = the code (Recommended) — `{"v":1,"error":"<code>","detail":null}`, code ∈ strict-modes-failed · already-live · no-session · session-live · cwd-missing
  2. error = refused + detail — `{"v":1,"error":"refused","detail":"<code>"}`
  3. Only --list gets a document — refusals stay human-only until "Exit-cause code catalogue"
Answer: "error = the code (Recommended)" — notes: Overseer, technical (founder-delegated): reuse architecture's existing error form so --json, the process log and the hint name one code; Exit-cause code catalogue extends the same form.

Q2 (Canonicalise): Should this chunk canonicalise `--home` (and the `VIOLA_DIR`-derived home)? security-plan's CLI row requires it, and its amendments say it is owed here; CARRY 1 records the landed form as resolved without canonicalisation.
  1. Canonicalise now (Recommended) — every verb canonicalises its home once it exists (`run` after creating it); on Windows the plain drive form, not the `\\?\` verbatim prefix
  2. VIOLA_DIR step only; defer — canonicalisation to Epoch 6's "Home and code-bearing file integrity"
Answer: "Canonicalise now (Recommended)" — notes: Overseer, technical: security-plan's CLI row requires it and its amendments owe it here; tests that compare paths compare the resolved home. Also carry the operator's folded CI red (the note sent at 02:33Z) into scope before P5.
