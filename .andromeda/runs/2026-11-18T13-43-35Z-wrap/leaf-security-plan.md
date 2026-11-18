@leaf_group group=security-plan
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_read leaf=.claude/rules/security.md lines=45
last: - 2026-11-16: Write a bounded read as `take(reader, MAX_FRAM
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=38 class=stale
leaf-text: `viola verify` reads `CLAUDE_CONFIG_DIR`'s value, only when `claude_env_keep` keeps that name, to choose the Claude config holding its trust record
source: dec-rlsdenig «viola verify and the start of run and revive read CLAUDE_CONFIG_DIR's value, only when claude_env_keep keeps that name, to choose the Claude config holding the trust record.»
replace-with: `viola verify` and the start of `run` and `revive` read `CLAUDE_CONFIG_DIR`'s value, only when `claude_env_keep` keeps that name, to choose the Claude config holding the trust record
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=72 class=stale
leaf-text: nothing written by viola under `~/.claude` save verify's own trust record for its fresh temp folder in the Claude config its child reads, only that record removed after and a SIGKILL leaving it
source: dec-obqex4gy «verify writes a trust record for its own temp folder and removes only it after, a SIGKILL leaving it; run and revive's start set the trust flag of an untrusted spawn folder, which stays.»
replace-with: nothing written by viola under `~/.claude` save verify's own trust record for its fresh temp folder in the Claude config its child reads, only that record removed after and a SIGKILL leaving it, and the trust flag `run` and `revive`'s start set for an untrusted spawn folder, which stays
@leaf_edit n=3 leaf=.claude/docs/security-summary.md line=73 class=stale
leaf-text: `run` and `revive` write none.
source: dec-3hpe5epg «On a verified CLI only, run and revive's start write the claude CLI's trust record for an untrusted spawn folder into the config its child reads, setting hasTrustDialogAccepted alone»
replace-with: on a verified CLI only, `run` and `revive`'s start set `hasTrustDialogAccepted` alone (every other key kept) in the trust record of an untrusted spawn folder in the config its child reads, never for the user home folder or a folder the CLI's own walk reads as trusted, and that record stays.
@leaf_edit n=4 leaf=.claude/rules/security.md line=14 class=stale
leaf-text: save `viola verify`'s trust record writer, which replaces Claude's config file keeping that file's own mode
source: dec-tolctq2d «The one trust record writer, verify's and the start's, never creates an absent config, refuses a link, a non-regular file, another owner's file, an oversize or malformed document and, for verify»
replace-with: save the one trust record writer, verify's and the start's, which replaces Claude's config file keeping that file's own mode
@leaf_edit n=5 leaf=.claude/rules/security.md line=14 class=stale
leaf-text: The trust record writer never creates an absent config, refuses a link, a non-regular file, a file of another owner, an oversize or malformed document and a present key,
source: dec-tolctq2d «The one trust record writer, verify's and the start's, never creates an absent config, refuses a link, a non-regular file, another owner's file, an oversize or malformed document and, for verify»
replace-with: The one trust record writer, verify's and the start's, never creates an absent config, refuses a link, a non-regular file, a file of another owner, an oversize or malformed document and, for verify, a present key,
@leaf_edit n=6 leaf=.claude/rules/security.md line=16 class=omission
leaf-text: and hands portable-pty an absolute path.
source: dec-3hpe5epg «On a verified CLI only, run and revive's start write the claude CLI's trust record for an untrusted spawn folder into the config its child reads, setting hasTrustDialogAccepted alone»
replace-with: and hands portable-pty an absolute path. On a verified CLI only, `run` and `revive`'s start write the CLI's trust record for an untrusted spawn folder into the config its child reads (never for the user home folder, nor a folder the CLI's own walk reads as trusted by its own key or a parent up to the git root), setting `hasTrustDialogAccepted` alone with every other key kept, or a one-key entry where none stands; unlike verify's, the record persists.
@leaf_edit n=7 leaf=.claude/rules/security.md line=17 class=stale
leaf-text: viola writes nothing under `~/.claude` save verify's own trust record for its fresh temp folder in the Claude config its child reads, only that record removed after (
source: dec-obqex4gy «verify writes a trust record for its own temp folder and removes only it after, a SIGKILL leaving it; run and revive's start set the trust flag of an untrusted spawn folder, which stays.»
replace-with: viola writes nothing under `~/.claude` save verify's own trust record for its fresh temp folder in the Claude config its child reads, only that record removed after, and the trust flag `run` and `revive`'s start set for an untrusted spawn folder, which stays (
@leaf_edit n=8 leaf=.claude/rules/security.md line=37 class=stale
leaf-text: `CLAUDE_CONFIG_DIR` (its value read by `viola verify` only when `claude_env_keep` keeps that name, to choose the Claude config holding its trust record;
source: dec-opfbpjfx «verify and run and revive's start read CLAUDE_CONFIG_DIR's value, only when claude_env_keep keeps the name, to choose the Claude config holding the trust record; else the user HOME's .claude.json.»
replace-with: `CLAUDE_CONFIG_DIR` (its value read by `viola verify` and the start of `run` and `revive` only when `claude_env_keep` keeps that name, to choose the Claude config holding the trust record;
@leaf_edit n=9 leaf=.claude/rules/security.md line=41 class=stale
leaf-text: A trust record write or removal ends in one of 14 closed code words, and when nothing was written that word alone is stamped as the trust-record row's value, never a path, key or value.
source: dec-cqckpo7v «A trust record write or removal ends in one of 14 closed code words, a start's trust step in one of 16 start words; verify's word alone is stamped as the trust-record row's value when nothing was»
replace-with: A trust record write or removal ends in one of 14 closed code words, a start's trust step in one of 16 start words; verify's word alone is stamped as the trust-record row's value when nothing was written, a start's is its trust-record line's detail, never a path, key or value.
