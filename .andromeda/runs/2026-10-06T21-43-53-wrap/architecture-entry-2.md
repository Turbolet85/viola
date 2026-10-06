## 2026-10-06-local-command-and-paste-framing-rows — registry rows for the framing variants, the fake agent's two options and the committed sets
**Section:** §Occupied Resources (Test-only binaries, `viola-fake-agent`; Repository, `fixtures/claude/<cli-version>/`) · [CLI Version Compatibility] verify paragraph (`--record`, the CI modes) · §Infrastructure Patterns → Crate dependency direction · CI/CD approach · Project directory structure
**Change:**
- The fake agent's argv options serving verify's interactive runs are seven (was five): `--framing` (a compiled paste text fires its recorded `paste-1` / `paste-2` UserPromptSubmit variant unchanged; the probed local command fires the recorded `clear-1` SessionEnd then SessionStart and no UserPromptSubmit) and the test-only hold `--paste-hint-ms <ms>`, capped at 8 000 ms. Still no env.
- `--record` also writes Run B's framing captures as `UserPromptSubmit.paste-<n>.json`, `SessionEnd.clear-<n>.json` and `SessionStart.clear-<n>.json` (stems `paste`, `clear`), through the same scrub, refusal and hygiene walk.
- Committed sets: was `2.1.287` and `2.1.288` stamped at fourteen rows, `2.1.283` drift-only; now `2.1.287` (with 4 framing variants) stamped at seventeen rows, `2.1.288` (no framing variant) and `2.1.283` drift-only.
- CI runs verify against the fake agent's `--screens --turn-stop --trusted-root --dialogs --framing` modes, seventeen rows (the verify paragraph and the CI/CD key).
- Key files: the `ledger` module at seventeen rows with its compiled paste texts and local-command list; `fixtures/claude/` holds dialog and framing variants; `verify/typed.rs` holds Run B's wait.
**Why:** the registry rows follow what the chunk landed: two argv options of the test-only binary, four recorded variants under 2.1.287 only, and a stamped list of one. 2.1.288 was not re-recorded (the live cap named 2.1.287 alone), so it can no longer stamp every row.
**Ref:** .andromeda/runs/2026-10-06T21-43-53-wrap/
