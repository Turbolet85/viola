@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=46
last: - 2026-11-21: A promise that a value reaches no fixture cove
@leaf_edit n=1 leaf=.claude/rules/security.md line=14 class=omission
leaf-text: and `verify` through `check_home_tree` before their log opens, exit 1;
source: dec-oeec26zu «run, revive's start, open and verify hold the home tree to the strict-modes check before their first read or open under the home and before any log opens; a refusal exits 1.»
replace-with: and `verify` through `check_home_tree` before their log opens, and `open` on its home tree before its first read or open under the home and before any log opens, exit 1;
@leaf_edit n=2 leaf=.claude/rules/security.md line=19 class=omission
leaf-text: and authenticates nothing.
source: dec-dontskbc «The recorded window origin, cli or agent, is self-reported by the caller and authenticates nothing.»
replace-with: and authenticates nothing; the recorded window origin, `cli` or `agent`, is self-reported by the caller and likewise authenticates nothing.
@leaf_edit n=3 leaf=.claude/rules/security.md line=22 class=omission
leaf-text: window Address validator lives in the seam crate,
source: dec-kir2odyr «viola list --json and the MCP list item carry a recorded window only when its address re-parses through the one window address validator, and an origin only when it is a closed origin word.»
replace-with: window Address validator lives in the seam crate, and `viola list --json` and the MCP list item carry a recorded window only when its address re-parses through it and an origin only when it is a closed origin word,
@leaf_edit n=4 leaf=.claude/rules/security.md line=16 class=omission
leaf-text: hands portable-pty an absolute path.
source: dec-re6kerzk «viola run's hidden --origin takes only the closed words cli or agent and --place only an integer, and --place without --origin is a usage error.»
replace-with: hands portable-pty an absolute path. Its hidden `--origin` takes only `cli` or `agent` and `--place` only an integer (`--place` without `--origin` a usage error); its window step moves only the window of its own terminal, found by its parent pid within a 5 s wait, at most once, never a window it did not open; an instance's window record (address, workspace, placement, origin) is kept only in the snapshot and the event log's window line and leaves only through `list --json` and the MCP list item, no address, pid or workspace on a process-log line.
@leaf_edit n=5 leaf=.claude/rules/security.md line=16 class=omission
leaf-text: unlike verify's, the record persists.
source: dec-z6izmek6 «viola open takes only the name and the flags --parent, --role overseer|builder|worker, --cwd, --workspace, --model, --effort and --json, and the one word claude after the dashes; any other word, or a parent»
replace-with: unlike verify's, the record persists. `viola open` takes only the name, `--parent`, `--role overseer|builder|worker`, `--cwd`, `--workspace`, `--model`, `--effort`, `--json` and the one word `claude` after the dashes (any other word, or a parent equal to the name, exit 2 before any home read, log, pin or spawn); it spawns the terminal resolved on the calling process's `PATH` directly, never through a shell, with a closed argv of one element per value, starting in it only the pinned `viola run` with `claude`, and its parent-window lookup walks at most 8 ancestor pids, held in memory only and never recorded.
