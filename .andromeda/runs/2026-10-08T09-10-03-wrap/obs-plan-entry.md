## 2026-10-08-first-live-test-and-self-drive — the G2 probe's other-file case carries no line citation
**Section:** §9 CI Integration (the sentence on `scripts/g2-zero-panics.sh --probe`)
**Change:** the probe's other-file case reads "a panic located in another file (`src/cmd/hook.rs`, at the line `scripts/g2-zero-panics.sh` plants)" (was: the literal `src/cmd/hook.rs:9`). The probe, its cases and their verdicts did not change.
**Why:** form only, on the operator's word at this wrap: the planted literal lives in the script, and the first citation sweep read it as a citation of the tree and would have re-pointed it. Standing rule: a master names a planted literal by its home, never as `path:N`.
**Ref:** .andromeda/runs/2026-10-08T09-10-03-wrap/
