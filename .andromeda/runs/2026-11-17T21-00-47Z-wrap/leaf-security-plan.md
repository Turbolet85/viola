@leaf_group group=security-plan
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_read leaf=.claude/rules/security.md lines=46
last: - 2026-11-16: Write a bounded read as `take(reader, MAX_FRAM
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=38 class=omission
leaf-text: `XDG_RUNTIME_DIR` is read by the channel endpoint only to place the socket directory;
source: dec-rlsdenig «viola verify reads CLAUDE_CONFIG_DIR's value, only when claude_env_keep keeps that name, to choose the Claude config holding its trust record.»
replace-with: `XDG_RUNTIME_DIR` is read by the channel endpoint only to place the socket directory; `viola verify` reads `CLAUDE_CONFIG_DIR`'s value, only when `claude_env_keep` keeps that name, to choose the Claude config holding its trust record;
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=72 class=omission
leaf-text: SIGKILL, a crash and a failed handler install leave it to be removed by hand)
source: dec-pzgvqgc3 «SIGKILL, a crash or a failed handler install leave the directories, and verify's own trust record in the Claude config, to be removed by hand.»
replace-with: SIGKILL, a crash and a failed handler install leave it, and verify's own trust record in the Claude config, to be removed by hand)
@leaf_edit n=3 leaf=.claude/docs/security-summary.md line=72 class=stale
leaf-text: nothing written by viola under `~/.claude` (the M7 = A and STOP 7 rulings, 2026-10-05)
source: dec-obqex4gy «viola verify writes a trust record for its own fresh temp folder into the Claude config its child reads and removes only that record after; a SIGKILL leaves it.»
replace-with: nothing written by viola under `~/.claude` save verify's own trust record for its fresh temp folder in the Claude config its child reads, only that record removed after and a SIGKILL leaving it (the M7 = A and STOP 7 rulings, 2026-10-05)
@leaf_edit n=4 leaf=.claude/docs/security-summary.md line=73 class=stale
leaf-text: viola itself writes nothing under `~/.claude` or into `~/.claude.json`.
source: dec-q2hyz4q4 «viola verify writes a trust record for its own fresh temp folder into the Claude config its child reads and removes only that record after; a SIGKILL leaves it. Not a rule for run or revive.»
replace-with: viola itself writes nothing under `~/.claude` or into `~/.claude.json`, save `viola verify`'s own trust record for its fresh temp folder in the Claude config its child reads, only that record removed after (a SIGKILL leaves it); `run` and `revive` write none.
@leaf_edit n=5 leaf=.claude/rules/security.md line=14 class=stale
leaf-text: every replaced file goes through the one helper `viola_state::fs::replace_private` (tempfile `persist`), which sets the temp file's mode before any byte is written;
source: dec-qfrs67xc «The trust record writer replaces Claude's config file keeping that file's own mode.»
replace-with: every replaced file goes through the one helper `viola_state::fs::replace_private` (tempfile `persist`), which sets the temp file's mode before any byte is written, save `viola verify`'s trust record writer, which replaces Claude's config file keeping that file's own mode;
@leaf_edit n=6 leaf=.claude/rules/security.md line=14 class=omission
leaf-text: `marketplace/` is no trusted file and its installed `command` is not re-hashed (`marketplace/` is off the integrity set).
source: dec-tolctq2d «The trust record writer never creates an absent config, refuses a link, a non-regular file, a file of another owner, an oversize or malformed document and a present key, takes a lock beside the file»
replace-with: `marketplace/` is no trusted file and its installed `command` is not re-hashed (`marketplace/` is off the integrity set). The trust record writer never creates an absent config, refuses a link, a non-regular file, a file of another owner, an oversize or malformed document and a present key, takes a lock beside the file only when free, tries three times, keeps the file's mode and touches no other key.
@leaf_edit n=7 leaf=.claude/rules/security.md line=17 class=omission
leaf-text: SIGKILL, a crash and a failed handler install leave them to be removed by hand;
source: dec-pzgvqgc3 «SIGKILL, a crash or a failed handler install leave the directories, and verify's own trust record in the Claude config, to be removed by hand.»
replace-with: SIGKILL, a crash and a failed handler install leave them, and verify's own trust record in the Claude config, to be removed by hand;
@leaf_edit n=8 leaf=.claude/rules/security.md line=17 class=stale
leaf-text: viola writes nothing under `~/.claude` (the CLI's own Run B
source: dec-obqex4gy «viola verify writes a trust record for its own fresh temp folder into the Claude config its child reads and removes only that record after; a SIGKILL leaves it.»
replace-with: viola writes nothing under `~/.claude` save verify's own trust record for its fresh temp folder in the Claude config its child reads, only that record removed after (the CLI's own Run B
@leaf_edit n=9 leaf=.claude/rules/security.md line=37 class=stale
leaf-text: The only env vars outside `VIOLA_*` any `viola` build reads are `XDG_RUNTIME_DIR` (read by the channel endpoint only to place the socket directory),
source: dec-opfbpjfx «viola verify reads the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to choose the Claude config holding its trust record; else it uses the user HOME's .claude.json.»
replace-with: The only env vars outside `VIOLA_*` any `viola` build reads are `XDG_RUNTIME_DIR` (read by the channel endpoint only to place the socket directory), `CLAUDE_CONFIG_DIR` (its value read by `viola verify` only when `claude_env_keep` keeps that name, to choose the Claude config holding its trust record; else the user HOME's `.claude.json`),
@leaf_edit n=10 leaf=.claude/rules/security.md line=41 class=omission
leaf-text: - Full detail goes only to `instances/<name>/diagnostics/detail-*.ndjson` (0600) when an instance resolves.
source: dec-cqckpo7v «Trust record outcomes are closed code words — A trust record write or removal ends in one of 14 closed code words, and when nothing was written that word alone is stamped as the trust-record row's value»
replace-with: - Full detail goes only to `instances/<name>/diagnostics/detail-*.ndjson` (0600) when an instance resolves. A trust record write or removal ends in one of 14 closed code words, and when nothing was written that word alone is stamped as the trust-record row's value, never a path, key or value.
