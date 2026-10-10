
## 2026-10-10-viola-revive — the `revive` refusals and hints, its `--list` rows, the verb in the `setup` help group
**Section:** §Brand Identity (the expression table, the cli 0.0 row) · §Surface: cli → Component Patterns (the stderr rule; pattern 2, hints; pattern 5, a `revive` bullet; Exit-code phraseology, the exit 1 row) · §Surface: cli → Navigation Pattern · §Per-Surface Bans (the no-colour ban)
**Change:**
- Pattern 2: the exit-1 refusals of `viola revive`, one fixed-message line and one hint per cause, read in a fixed order that stops at the first refusal: `<name>'s state files can be written by another user` → `viola will not read them; make the viola home and its files owner-only`; `run`'s `already live` pair and its stale sibling, unchanged; `<name> has no logged session to resume` → `start one: viola run <name> -- claude`; `<name> has no logged session with that id` → `viola revive <name> --list`; `<name>'s recorded directory is missing` → `resume it by hand from a directory you choose: viola run <name> -- claude --resume <id>, ids from viola revive <name> --list`. None shows the recorded directory, a pid or a logged id; no hint names `viola release`.
- The stderr rule: the fixed message form `unable: <text>` is used by the exit-1 refusals of `viola run`, `viola revive` and `viola verify` (was "only the exit-1 start refusal"). The exit 1 phraseology row lists revive's four lines.
- Pattern 5, `revive`: prints nothing once the child starts; `--list` prints one static stdout line per logged session, `<time>  <cause>  <session id>`, two spaces between fields, no header, no colour, plain ASCII; no `--json`.
- The detail-code sentence names `no-session` and `cwd-missing`; revive's strict-modes refusal reuses `strict-modes-failed` at exit 1.
- Navigation Pattern: the flat verb list names `revive`; `viola --help` groups it as `setup: run, revive, verify, plugin install`.
- The 0.0 expression row and the no-colour ban name a passed `viola revive` as the same passthrough as `viola run`.
**Why:** the chunk landed the verb. The four pairs stand as the plan worded them on the founder's answer, live, 2026-10-10, relayed by the operator. The help group is the operator's answer at this wrap's Phase 2 halt: the binary's own help lists `revive` right after `run`.
**Ref:** .andromeda/runs/2026-10-10T15-07-22-wrap/
