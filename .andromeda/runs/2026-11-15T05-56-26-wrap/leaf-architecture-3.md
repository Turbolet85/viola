@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/services/viola.md lines=58
last: - `doc:architecture` · `.claude/rules/api.md` · `.claude/rul
@leaf_read leaf=.claude/docs/stack.md lines=66
last: 5. Commit as `chore: bump {tool} to {version}`
@leaf_read leaf=.claude/docs/workflow.md lines=67
last: This project uses a **reference-based** CLAUDE.md architectu
@leaf_edit n=1 leaf=.claude/docs/services/viola.md line=6 class=retired
leaf-text: Run B trusted in `<cwd>/.viola-verify-<pid>/` (for shell-plain paths only, also one `--settings <probe dir>/typed/statusline.json` after its `--plugin-dir` pair, whose status line command `<pinned viola> hook statusline --capture <probe dir>/typed/statusline --shell "$0"` files each status line payload with its shell verdict) with six
source: abs-lpwwhjjj «absence/status-line-interception — Status-line interception: no settings layer, no --settings at start, no statusLine source read, no status-line command»
replace-with: Run B trusted in `<cwd>/.viola-verify-<pid>/` with six
@leaf_edit n=2 leaf=.claude/docs/services/viola.md line=6 class=stale
leaf-text: all twenty-three of `LedgerRow::ALL` on every build
source: dec-ucvfrmxd «adding one needs a 22nd row, its viola verify probe and a re-stamp of every stamped version.»
replace-with: all twenty-one of `LedgerRow::ALL` on every build
@leaf_edit n=3 leaf=.claude/docs/services/viola.md line=20 class=retired
leaf-text: + heartbeat (the per-session settings override is rewritten after the first snapshot and before the spawn)
source: abs-lpwwhjjj «absence/status-line-interception — Status-line interception: no settings layer, no --settings at start, no statusLine source read, no status-line command»
replace-with: + heartbeat
@leaf_edit n=4 leaf=.claude/docs/services/viola.md line=21 class=retired
leaf-text: `--plugin-dir <dir>`, `--settings <instance dir>/settings.json` when the start wrote the override, `--resume <id>`
source: abs-lpwwhjjj «absence/status-line-interception — Status-line interception: no settings layer, no --settings at start, no statusLine source read, no status-line command»
replace-with: `--plugin-dir <dir>`, `--resume <id>`
@leaf_edit n=5 leaf=.claude/docs/services/viola.md line=23 class=retired
leaf-text: - The statusline pass-through (`src/cmd/run.rs`, `src/cmd/hook/statusline.rs`): at start `run` reads the user's command from `<home>/statusline-source.json`, else for the default home only (`canonical_home(default_home(user_home))`) the user's `.claude/settings.json`, so a home named by `--home` or reached through `VIOLA_DIR` never reads it, records it as the snapshot's `statusline_command`, and rewrites `instances/<name>/settings.json` (0600, only for a pinned path of ASCII letters, digits and `_ - . / :`) and passes `--settings`. `viola hook statusline` (with `--capture <DIR>` it takes `viola verify`'s probe capture instead, before the pass-through: no instance check, no command run, nothing printed) canonicalises its instance directory, runs `check_instance` before anything under the home is read or opened (a refused instance: exit 0 at once, no log, no line), reads stdin through `take(MAX_FRAME + 1)`, reads the snapshot through `replay::read_snapshot_or_replay` (a replayed one holds no `statusline_command`, so no command runs), then runs the command as one argument of `/bin/sh -c` under `STATUSLINE_DEADLINE` (5 s, PROVISIONAL) and prints its stdout unchanged; always exit 0, empty stderr.
source: abs-lpwwhjjj «absence/status-line-interception — Status-line interception: no settings layer, no --settings at start, no statusLine source read, no status-line command»
replace-with: - No status-line interception: no settings layer, no `--settings` at start, no `statusLine` source read, no status-line command; the retired hook word `statusline` maps to no hook event, so `viola hook statusline` takes the unknown-event exit 0 and reads and writes nothing.
@leaf_edit n=6 leaf=.claude/docs/services/viola.md line=36 class=retired
leaf-text: exit 0 always. Its statusline form `hook statusline --capture <DIR> [--shell <WORD>]` (`cmd::hook::capture_statusline`, sole caller Run B's probe settings) files the payload raw at the first free `StatusLine.<k>.json` beside `StatusLine.<k>.shell` holding exactly `bin-sh` or `other` (the word written nowhere), prints nothing and reads no snapshot.
source: ctr-zgw4cnv3 «contract/hook-to-kind-map/hook-statusline · Hook to kind map · hook statusline — The retired hook word `statusline` maps to no hook event: `viola hook statusline` takes the unknown-event exit 0 and reads»
replace-with: exit 0 always.
@leaf_edit n=7 leaf=.claude/docs/workflow.md line=49 class=stale
leaf-text: all twenty-three on every build
source: dec-ucvfrmxd «adding one needs a 22nd row, its viola verify probe and a re-stamp of every stamped version.»
replace-with: all twenty-one on every build
