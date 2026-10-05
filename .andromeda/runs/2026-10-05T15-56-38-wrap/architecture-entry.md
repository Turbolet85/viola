
## 2026-10-05-permission-end-to-end — the permission kind end to end; the question first raised by PermissionRequest stays null
**Section:** §Established Decisions → [CLI Version Compatibility]; §Standard Contracts (the `hook.dialog` receipt paragraph)
**Change:**
- [CLI Version Compatibility]: was "the `permission` kind's end to end and a `question` first raised by PermissionRequest are owed to the 'Permission end to end' route entry"; now the `permission` kind runs end to end (`tests/cli_answer.rs` over the gated `fixtures/fake-scripts/path4-permission.json`), and a `question` first raised by PermissionRequest stays `null` to the human: on 2.1.288 and 2.1.287 a PermissionRequest `allow` WITH `updatedInput` would satisfy it (read statically from both installed bundles, never run), but that body has no ledger row; adding one needs a 15th row, its `viola verify` probe and a re-stamp of both versions (at least 8 live sessions).
- §Standard Contracts: the list of dialogs logged once and answered `null` at once gains a `question` first raised by PermissionRequest with no matching armed dialog; a late `answer` to it is refused `unknown-dialog`.
**Why:** the chunk landed the permission end-to-end case and measured the question's body statically with zero live sessions; the `null` stays because the capability ledger is the single gate for a CLI behaviour and the 15th row's cost exceeds the live-session cap left (the overseer's word at the P5 review, 2026-10-05).
**Kept:** viola's bare PermissionRequest `allow` on ExitPlanMode stays "ignored": true for the body viola sends, although an `allow` WITH `updatedInput` would satisfy ExitPlanMode on both CLIs (static read).
**Ref:** .andromeda/runs/2026-10-05T15-56-38-wrap/
