@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=17 class=omission
leaf-text: single writers for snapshot and stamps;
source: dec-qaro53sc «The start of viola mcp and viola plugin install hold the home to the write rule.»
replace-with: single writers for snapshot and stamps; the start of `viola mcp` holding the home to the write rule (a refusal exit 1, both streams empty, no role file);
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=19 class=omission
leaf-text: (for `claude agents --json`)
source: dec-otmvj2vn «list, revive's preflight, adopt's live-elsewhere check and the MCP list tool resolve claude for claude agents --json from the calling process's PATH and current directory; its output is untrusted»
replace-with: (for `claude agents --json`, resolved from the caller's PATH and current directory by `list`, `revive`'s preflight, `adopt`'s live-elsewhere check and the MCP list tool; its output untrusted and bounded by `MAX_FRAME`)
@leaf_edit n=3 leaf=.claude/docs/security-summary.md line=29 class=omission
leaf-text: Typed fields in diagnostics; paths never in external errors;
source: dec-okfbhtnd «The start's spawn directory lives only in the 0600 snapshot, as the key of its trust record in the Claude config, and as the working directory on viola up's terminal argv, on no log line»
replace-with: Typed fields in diagnostics; paths never in external errors; the start's spawn directory kept only in the 0600 snapshot, as the key of its trust record in the Claude config and as the working directory on `viola up`'s terminal argv, never on a log line, error body or stdout;
@leaf_edit n=4 leaf=.claude/docs/security-summary.md line=29 class=omission
leaf-text: `viola focus`'s window lookup walks at most 8 ancestor pids of the session's wrapper, held in memory only and never recorded
source: dec-2zama6hr «viola open's parent-window lookup walks at most 8 ancestor pids, held in memory only and never recorded.»
replace-with: `viola focus`'s window lookup walks at most 8 ancestor pids of the session's wrapper, and `viola open`'s parent-window lookup at most 8 ancestor pids, each held in memory only and never recorded
@leaf_edit n=5 leaf=.claude/docs/security-summary.md line=65 class=omission
leaf-text: `viola adopt` resumes only a session no live claude holds
source: dec-osxmbafc «viola adopt places an operator-typed id there too, not a logged one, in the same closed shape.»
replace-with: `viola adopt` resumes only a session no live claude holds (its operator-typed id placed on the child's argv only in the closed session-id shape, one argv element of a direct spawn)
@leaf_edit n=6 leaf=.claude/docs/security-summary.md line=65 class=omission
leaf-text: then resumes the session in place on the new pair.
source: dec-uyp3bfs7 «viola open and viola up spawn the terminal program resolved on the calling process's PATH directly, never through a shell, with a closed argv of one element per value»
replace-with: then resumes the session in place on the new pair. `viola open` and `viola up` spawn the terminal program resolved on the caller's PATH directly, never through a shell, with a closed argv of one element per value: open starts in it only the pinned `viola run` with claude, up only the pinned `viola revive <name> --origin` in the recorded cwd.
@leaf_edit n=7 leaf=.claude/docs/security-summary.md line=72 class=omission
leaf-text: Dev-host prerequisite: run it from a trusted folder whose external CLAUDE.md import is answered (the CLI keys that answer on the git root).
source: dec-pv6qgtem «a run that first meets the trust or import dialog is refused untrusted-folder, exit 1, with no stamp written.»
replace-with: Dev-host prerequisite: run it from a trusted folder whose external CLAUDE.md import is answered (the CLI keys that answer on the git root); verify never answers that dialog, and a run that first meets the trust or import dialog is refused `untrusted-folder`, exit 1, with no stamp written.
@leaf_edit n=8 leaf=.claude/docs/security-summary.md line=72 class=omission
leaf-text: and the shared scrub refuses an unclean set whole.
source: dec-fnlaf3dn «The test-only R5 adopt live-reading script ends only the wrapper it recorded, while that process's exe is the given viola, and its own script host»
replace-with: and the shared scrub refuses an unclean set whole; the test-only R5 adopt live-reading script ends only the wrapper it recorded, while that process's exe is the given viola, and its own script host (TERM, then 10 s, then KILL only if the start time is unchanged), its verdicts keeping words only.
@leaf_edit n=9 leaf=.claude/docs/security-summary.md line=72 class=omission
leaf-text: the test-only `viola-hypr-tap` moves only
source: dec-j2qfvfi4 «The test-only R7 placing script opens one terminal window of its own through viola open, starts no claude but an owner-only stand-in answering its version, unsets every inherited VIOLA_ name»
replace-with: the test-only R7 placing script opens one terminal window of its own through `viola open`, starts no claude but an owner-only stand-in answering its version, unsets every inherited `VIOLA_` name, refuses on a locked desktop and closes only its own terminal; the test-only `viola-hypr-tap` moves only
