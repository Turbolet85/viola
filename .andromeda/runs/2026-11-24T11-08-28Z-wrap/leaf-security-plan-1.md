@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=14 class=omission
leaf-text: with no bind on `run` and no frame on a client
source: dec-ntxnz2nn «A start, run, revive or adopt alike, checks the socket directory before any bind, first snapshot or child, makes it only when absent, and checks again so the loser of a race to make it reads the same verdict.»
replace-with: with no bind on a `run`, `revive` or `adopt` start and no frame on a client
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=17 class=omission
leaf-text: `run`, `revive`'s start, `open` and `verify` through `check_home_tree`
source: dec-oeec26zu «run, revive's start, adopt's start, open and verify hold the home tree to the strict-modes check before their first read or open under the home and before any log opens; a refusal exits 1.»
replace-with: `run`, `revive`'s start, `adopt`'s start, `open` and `verify` through `check_home_tree`
@leaf_edit n=3 leaf=.claude/docs/security-summary.md line=38 class=stale
leaf-text: `viola verify` and the start of `run` and `revive` read `CLAUDE_CONFIG_DIR`'s value
source: dec-rlsdenig «viola verify and the start of run, revive and adopt read CLAUDE_CONFIG_DIR's value, only when claude_env_keep keeps that name, to choose the Claude config holding the trust record.»
replace-with: `viola verify` and the start of `run`, `revive` and `adopt` read `CLAUDE_CONFIG_DIR`'s value
@leaf_edit n=4 leaf=.claude/docs/security-summary.md line=72 class=stale
leaf-text: the trust flag `run` and `revive`'s start set for an untrusted spawn folder
source: dec-q2hyz4q4 «verify writes a trust record for its own temp folder and removes only it after, a SIGKILL leaving it; the start of run, revive and adopt sets the trust flag of an untrusted spawn folder, which stays.»
replace-with: the trust flag the start of `run`, `revive` and `adopt` sets for an untrusted spawn folder
@leaf_edit n=5 leaf=.claude/docs/security-summary.md line=73 class=stale
leaf-text: on a verified CLI only, `run` and `revive`'s start set `hasTrustDialogAccepted` alone
source: dec-3hpe5epg «On a verified CLI only, the start of run, revive and adopt writes the claude CLI's trust record for an untrusted spawn folder into the config its child reads, setting hasTrustDialogAccepted alone»
replace-with: on a verified CLI only, the start of `run`, `revive` and `adopt` sets `hasTrustDialogAccepted` alone
@leaf_edit n=6 leaf=.claude/docs/security-summary.md line=65 class=omission
leaf-text: the founder's own answer to P4 fork 3 of that chunk.
source: dec-7vcwrkel «viola adopt resumes only a session no live claude holds, spawns its own child in its own terminal, and never ends or signals a process viola did not start.»
replace-with: the founder's own answer to P4 fork 3 of that chunk. `viola adopt` (provisional) resumes only a session no live claude holds, spawns its own child in its own terminal, and never ends or signals a process viola did not start: a plain session still running elsewhere is refused, never taken over.
