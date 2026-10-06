## 2026-10-06-local-command-and-paste-framing-rows — the fake-agent verify flag list gains --framing
**Section:** §4 Span / Trace Coverage → Edge flows → `verify` / `plugin install` (the CI bullet)
**Change:** CI's fake-agent verify runs pass `--screens --turn-stop --trusted-root <workspace root> --dialogs --framing` (was the list ending at `--dialogs`). Nothing else moves: `[NN/MM]` carries no number here, verify still logs six spawn pairs, and no row text, prompt or path reaches any line.
**Why:** boot step 4 and the stamped-home fixture stamp seventeen rows through the framing replay; the list is bound to test-plan §3's boot step 4, which moved in the same pass.
**Ref:** .andromeda/runs/2026-10-06T21-43-53-wrap/
