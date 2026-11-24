@leaf_group group=design-system+layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=73
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=59 class=stale
leaf-text: the exit-1 refusals of `run`, `revive`, `open` and `verify` and every exit-21 strict-modes refusal use the fixed `unable: <text>` / `hint:` pair instead
source: ctr-pwvbecet «contract/cli-refusal-line/fixed-unable-form · CLI refusal line · fixed unable form — The exit-1 refusals of `run`, `revive`, `open`, `focus` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `focus`,»
replace-with: the exit-1 refusals of `run`, `revive`, `open`, `focus` and `verify` and every exit-21 strict-modes refusal use the fixed `unable: <text>` / `hint:` pair instead
@leaf_edit n=2 leaf=.claude/docs/design-summary.md line=50 class=omission
leaf-text: `window-unseen` · `unreachable` · `oversize` · `malformed`), the `viola run`
source: ctr-x63m6t5f «contract/cli-verb-output/focus · CLI verb output · viola focus — `viola focus <name>` prints `<name>  focused` on stdout, or under `--json` `{"v":1,"ok":{"name":"<name>"}}`; no window address, pid or workspace»
replace-with: `window-unseen` · `unreachable` · `oversize` · `malformed`), `viola focus <name>` (brings the named session's window forward and shows its workspace on Hyprland; one name and `--json` only, a CLI-only human control; prints `<name>  focused` on stdout, or under `--json` `{"v":1,"ok":{"name":"<name>"}}`, no window address, pid or workspace ever printed or logged; a name not live or a failed strict-modes check exits 21 before any compositor read, and its exit-1 refusals are six closed codes `refused` · `unsupported` · `window-not-found` · `unreachable` · `oversize` · `malformed`), the `viola run`
