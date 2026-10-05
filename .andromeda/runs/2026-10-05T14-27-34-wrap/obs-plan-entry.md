
## 2026-10-05-dialog-rows-and-re-probe — six verify spawn pairs, the capture arm's answer body
**Section:** §4 Edge flows → `verify` / `plugin install` · §6 Child / shell spawns
**Change:**
- verify logs six spawn pairs (was four): `version-probe`, `verify-probe`, then `verify-pty-probe` for Run A, B, C and D; the diag-line subject enum is unchanged.
- The capture arm stays uninstrumented, but was "its only output is the raw stdin payload … empty stdout"; now it claims its capture exclusively and, with `--answers`, a dialog event prints one product-built decision body to stdout; nothing on any failure; stderr empty, exit 0.
- CI drives the fake agent's four interactive runs, adding `--dialogs`.
**Why:** the chunk added Runs C and D and the `--answers` arm (the founder's live ruling of 2026-10-05, M7 = A, relayed by the overseer, ratified by the operator at this wrap as his). §10 exemption 5 (no role file) still holds.
**Ref:** .andromeda/runs/2026-10-05T14-27-34-wrap/
