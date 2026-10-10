
## 2026-10-10-viola-revive — the `revive` verb registered: subcommand, exit-1 refusals, child flags, log producer
**Section:** §Stack and Technologies (CLI parser row) · §Conventions (CLI exit codes, the `1` bullet; CLI) · §Occupied Resources (Binary, subcommands and exit codes; Claude Code integration names; Filesystem, `diagnostics/`) · §Established Decisions [CLI Conventions] · §Infrastructure Patterns → Project directory structure
**Change:**
- The subcommand lists name `revive` after `run` (the Stack row and the Subcommands bullet). `revive` takes `<name> [--id <ID>] [--fork] [-- <child args>]` or `<name> --list` (`--list` conflicts with the other three); no `--json`; exits 0, 1 for a preflight refusal, 2 for a clap usage error, a malformed `--id` included. It sends no channel frame.
- Exit 1 gains revive's preflight: four readings in a fixed order that stop at the first refusal, `strict-modes-failed`, `already-live` (`run`'s collision check and its two pairs, unchanged), `no-session` (two pairs) and `cwd-missing`; each one `unable:` / `hint:` pair holding no recorded directory, pid or logged id, and one `process-exit{subject:"self", exit_code:1, detail}` in `run-<name>.ndjson`. `--list` has two of them as the stderr pair and exit 1 with no log line.
- `role_of` files every first word but `run`, `revive`, `hook` and a leading `-` flag under `cli` (was `run`, `hook` and a flag).
- "Flag passed to the child" became "Flags": `--plugin-dir` first on every start; a revived start launches `claude` by name on its own `PATH` and adds `--resume <id>`, `--fork-session` with `--fork`, then the words after `--`; the id is one argv element, the newest logged id of the session-id shape or an `--id` the log holds.
- `diagnostics/`: `run-<name>.ndjson` has a second producer, revive's start arm, which logs as process `run`; `--list` opens no process log.
- [CLI Conventions]: `revive` has no machine form yet; its `--json` is owed to the working-route entry "CLI machine contract".
- The directory tree lists `revive` among `src/cmd/`'s modules, with a line for `revive.rs`, and among `human.rs`'s callers.
**Why:** the chunk landed the verb. The logged id on the child's command line is a crossing the founder was not shown as one: the body states what is built and that his word is owed, and no ratification is recorded, on the operator's answer at this wrap's Phase 2 halt. A later wrap records his word as his.
**Kept:** the wording "one endpoint per `viola run`" in the IPC rows and the `VIOLA_*` lines set "by `viola run`" stand: the report states no env or settings fact for revive, and a revived wrapper goes through the same start.
**Ref:** .andromeda/runs/2026-10-10T15-07-22-wrap/
