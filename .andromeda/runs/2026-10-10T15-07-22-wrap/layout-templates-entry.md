
## 2026-10-10-viola-revive — an output structure for `viola revive`; the verb in the command list and the `setup` help group
**Section:** §Surface: cli (Expression level; Primary screens (commands); Output structure — `viola --help`; a new Output structure — `viola revive`; Component — Primary navigation; Component — Primary content block 2, Line form)
**Change:**
- New Output structure — `viola revive`: a start that passes its preflight prints nothing of its own while the child holds the terminal; `viola revive <name> --list` prints one stdout row per logged session in log order, `<ts>  <cause>  <id>`, two spaces between fields, no header, no colour, plain ASCII, empty stderr, the cause one of `startup`, `clear`, `resume`, `compact`, else `unknown`; each refusal is exit 1 with one `unable:` line and one `hint:` line last on stderr and an empty stdout, the five pairs listed (the state-files pair, `run`'s collision pair, the two no-session pairs, the recorded-directory pair); `--list` has two of them; a malformed `--id` is clap's usage error, exit 2; the verb takes no `--json`.
- Primary screens: a `viola revive` bullet after `viola run`. `viola --help`: `setup:    run, revive, verify, plugin install`.
- Primary navigation: `--json` is the machine view of every data verb except `viola revive`, owed to the route entry "CLI machine contract" (was "every data verb").
- Line form: the fixed-message exceptions name the exit-1 refusals of `viola revive` beside `run`'s and `verify`'s. Expression level: the 0.0 list names the `viola run` / `viola revive` passthrough.
**Why:** the chunk landed a new user-facing verb with a list arm and four refusals of its own. The help group is the operator's answer at this wrap's Phase 2 halt.
**Ref:** .andromeda/runs/2026-10-10T15-07-22-wrap/
