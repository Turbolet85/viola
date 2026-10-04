
## 2026-10-04-dialog-answers-by-dialog-id — relayed dialog fixtures, the 2.1.287 default, Paths 3 and 4 as landed
**Section:** §2 Test Strategy (Contract row; the contract required-test line) · §6 Path 3 · Path 4 (surfaces, step 1) · §6 Security sweep → Windows `--home` outside `%USERPROFILE%` · §7 Fixture library · Recorded hook payloads · Fake agent (hook commands, modes) · Fixture hygiene · §3 → 5-command implementation (the root fixture chain)
**Change:**
- Fixture source was `viola verify` recordings only; now the spine is recorded and the dialog tier (`fixtures/claude/2.1.287/` `PreToolUse.*` / `PermissionRequest.*` for `ask-user-question` / `exit-plan-mode`, `RELAYED.md`) is relayed from the viola-lab prototype's live captures, `tool_input.plan` redacted, reviewed before commit, until `:82`'s re-probe; the contract suite checks both, the hygiene walk covers sets `2.1.283` and `2.1.287` and scripts `gated-turn` / `path3` / `path4`.
- The default CLI version was 2.1.283; now 2.1.287 (`DEFAULT_CLI_VERSION`, `RECORDED_CLI_VERSION`, `stamped_home`, `Wrapper::boot`).
- The fake agent's matchers were "not evaluated yet"; now `hook_commands(…, tool)` evaluates a group's `matcher` against `tool_name`.
- Path 3: the `question` / `plan` end-to-end wake witness landed (was owed to `:78`); the `permission` one is owed to `:82`. Path 4 as landed: `tests/cli_answer.rs` over `path4.json` on three OSes (question with annotations, plan approve, plan revise via the PermissionRequest repeat, the question's repeat silent, a second concurrent dialog empty, `unknown-dialog` exit 13); `permission` unit / insta only, its e2e owed to `:82`; step 1 fixture names in the §2 form.
- Security sweep: the Windows creation half landed (`create_private_dir`, unit both OSes + `tests/cli_version_gate.rs`), the `RUNNER_TEMP` negative owed to `:111`; a Windows DACL test homes under `target/e2e-home`, never `%TEMP%`.
**Why:** print mode raises no dialog hook, so the founder ruled live for relayed fixtures (R1) and pulled the creation half forward (R3); the paths record what the chunk's tests cover.
**Ref:** .andromeda/runs/2026-10-04T16-53-44-wrap/
