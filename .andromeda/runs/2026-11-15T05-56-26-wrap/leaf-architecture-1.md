@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=44 class=retired
leaf-text: `viola hook`'s event, dialog and statusline paths take the `instance_of` session
source: ctr-zgw4cnv3 «contract/hook-to-kind-map/hook-statusline · Hook to kind map · hook statusline — The retired hook word `statusline` maps to no hook event: `viola hook statusline` takes the unknown-event exit 0»
replace-with: `viola hook`'s event and dialog paths take the `instance_of` session
@leaf_edit n=2 leaf=.claude/docs/services/viola-agent-claude.md line=22 class=retired
leaf-text: The pure `statusline` module (as landed; chrono for it, no file, clock or environment read): `user_command` (a `statusLine` of type `command`, non-empty, no NUL), `override_document` and `shell_argv` (`/bin/sh -c`, `shell_argv` the fixed three-element argv `[&str; 3]`), `SETTINGS_FLAG`, `USER_SETTINGS`, `BIN_SH`, `SHELL_FLAG` and `probe_document` (the document behind Run B's probe `--settings`, `typed/statusline.json`). Its two CLI shapes are the ledger rows `statusline-override` · `statusline-shell` (since chunk 2026-10-13-owed-ledger-rows), relied on every build (`LedgerRow::relied()` holds all twenty-three rows; architecture, Capability ledger).
source: abs-lpwwhjjj «absence/status-line-interception — Status-line interception: no settings layer, no --settings at start, no statusLine source read, no status-line command»
replace-with: No status-line interception: no settings layer, no `--settings` at start, no `statusLine` source read and no status-line command run; the crate still holds the `--settings` flag (`SETTINGS_FLAG`) that `viola verify` reads, and the stamps no longer carry the two status-line row ids.
@leaf_edit n=3 leaf=.claude/docs/services/viola-agent-claude.md line=23 class=retired
leaf-text: shim resolution, the settings-override mechanism and the shell (rows `statusline-override` · `statusline-shell` since chunk 2026-10-13-owed-ledger-rows, relied on every build), harness prefixes
source: dec-zejotuie «except: Removing the budget gate's fields and event kind, the snapshot's statusline_command and the stamps' two status-line row ids kept every own-format `v` at 1; readers skip what was removed.»
replace-with: shim resolution, harness prefixes
@leaf_edit n=4 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=stale
leaf-text: `LedgerRow`, twenty-three closed rows
source: dec-ucvfrmxd «why: A PermissionRequest allow with updatedInput would satisfy it, but that body has no ledger row; adding one needs a 22nd row, its viola verify probe and a re-stamp of every stamped version.»
replace-with: `LedgerRow`, twenty-one closed rows
@leaf_edit n=5 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=retired
leaf-text: `paste-newline` · `statusline-override` · `statusline-shell` · `resume-session`
source: dec-zejotuie «except: Removing the budget gate's fields and event kind, the snapshot's statusline_command and the stamps' two status-line row ids kept every own-format `v` at 1; readers skip what was removed.»
replace-with: `paste-newline` · `resume-session`
@leaf_edit n=6 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=stale
leaf-text: (every row of `ALL`, in its order, all twenty-three)
source: dec-ucvfrmxd «why: A PermissionRequest allow with updatedInput would satisfy it, but that body has no ledger row; adding one needs a 22nd row, its viola verify probe and a re-stamp of every stamped version.»
replace-with: (every row of `ALL`, in its order, all twenty-one)
@leaf_edit n=7 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=retired
leaf-text: `ShellVerdict`, `StatusLineCapture`, `statusline_variants` (`StatusLine.default-1.json`, `StatusLine.rate-limits-1.json`) and the `StatusLine.<k>` file-name helpers; 
source: abs-lpwwhjjj «absence/status-line-interception — Status-line interception: no settings layer, no --settings at start, no statusLine source read, no status-line command»
replace-with: (no status-line capture, variant or shell verdict: viola intercepts no status line);
@leaf_edit n=8 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=retired
leaf-text: `Probes { print, typed, trusted, dialogs, statusline, resumed, agents }` (`trusted`: Run B's captures in claim order; `statusline`: Run B's status line captures in `k` order;
source: abs-lpwwhjjj «absence/status-line-interception — Status-line interception: no settings layer, no --settings at start, no statusLine source read, no status-line command»
replace-with: `Probes { print, typed, trusted, dialogs, resumed, agents }` (`trusted`: Run B's captures in claim order;
@leaf_edit n=9 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=stale
leaf-text: (every row of `LedgerRow::relied()` `pass`, all twenty-three on every build,
source: dec-ucvfrmxd «why: A PermissionRequest allow with updatedInput would satisfy it, but that body has no ledger row; adding one needs a 22nd row, its viola verify probe and a re-stamp of every stamped version.»
replace-with: (every row of `LedgerRow::relied()` `pass`, all twenty-one on every build,
@leaf_edit n=10 leaf=.claude/docs/services/viola-agent-claude.md line=38 class=retired
leaf-text: **Ledger rows + probes, screen signatures, statusline, agents parser:** same crate
source: abs-lpwwhjjj «absence/status-line-interception — Status-line interception: no settings layer, no --settings at start, no statusLine source read, no status-line command»
replace-with: **Ledger rows + probes, screen signatures, agents parser:** same crate
@leaf_edit n=11 leaf=.claude/docs/services/viola-agent-claude.md line=44 class=retired
leaf-text: **Property / fuzz:** hook stdin, statusline, `claude agents --json`
source: dec-u7smqsbl «decision/tolerant-external-payloads · External payloads parsed tolerantly — Hook JSON and `claude agents --json` are parsed with no `deny_unknown_fields`, with defaults and options;»
replace-with: **Property / fuzz:** hook stdin, `claude agents --json`
