
## 2026-10-05-dialog-rows-and-re-probe — verify output at fourteen rows
**Section:** §Output structure — `viola verify`
**Change:** the wireframe counters `/10` → `/14`, rows `[11/14] question-answer …` and `[14/14] dialog-concurrency …` shown, the summary `14 pass  0 fail`, the failing example `12 pass  2 fail` (was `8 pass  2 fail`); the ledger-order list gains the four dialog rows and `MM` is 14 (was 10).
**Why:** the chunk landed the dialog rows.
**Kept:** the `--help` sentence "verify never answers it" is about the CLI-native external-import dialog, which verify still never answers; the probe's own dialogs are answered by its hook, not by verify typing.
**Ref:** .andromeda/runs/2026-10-05T14-27-34-wrap/
