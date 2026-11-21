@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=47
last: - `doc:architecture` [PTY] · `.claude/docs/gotchas.md` (ConPTY,
@leaf_read leaf=.claude/docs/services/viola-state.md lines=49
last: - `doc:architecture` §Standard Contracts (snapshot envelope, i
@leaf_read leaf=.claude/docs/services/viola.md lines=58
last: - `doc:architecture` · `.claude/rules/api.md` · `.claude/rules
@leaf_edit n=1 leaf=.claude/docs/services/viola.md line=36 class=omission
leaf-text: is `verify`'s capture arm: no `VIOLA_*`, no obs init, no channel, raw 0600 write, exit 0 always.
source: ctr-v725f56b «On SessionStart alone the capture arm also writes Env.<k>.json owner-only beside the payload, holding the CLAUDE names of its own process with every other name only counted; no value is ever read or written.»
replace-with: is `verify`'s capture arm: no `VIOLA_*` and no environment value read, no obs init, no channel, raw 0600 write (on SessionStart alone also `Env.<k>.json` owner-only beside the payload: the CLAUDE names of its own process, every other name only counted, no value ever read or written), exit 0 always.
@leaf_edit n=2 leaf=.claude/docs/services/viola.md line=6 class=omission
leaf-text: the relied `local-command-echo` among them)
source: dec-fcus6tgb «The identity-floor relied ledger row passes when every CLAUDE name the CLI adds to a SessionStart hook is an identity-floor name or a known non-identity name, and fails on an unknown name or an unread or off-shape env-names file; kept names are dropped before the check.»
replace-with: the relied `local-command-echo` among them; the relied identity-floor row passing when every CLAUDE name the CLI adds to a SessionStart hook, kept names dropped first, is an identity-floor or known non-identity name, and failing on an unknown name or an unread or off-shape env-names file)
