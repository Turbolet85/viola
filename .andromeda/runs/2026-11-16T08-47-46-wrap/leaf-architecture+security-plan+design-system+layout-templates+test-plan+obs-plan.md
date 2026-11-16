@leaf_group group=architecture+security-plan+design-system+layout-templates+test-plan+obs-plan
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:a
@leaf_read leaf=CLAUDE.md lines=133
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=.claude/docs/gotchas.md line=84 class=stale
leaf-text: and `send` consumes them by exact equality of the typed text
source: dec-xqby3ost «A send is a local command only when its typed text is exactly a name on the ledger's local-command list, or that name, one space and a non-empty rest not starting with a space, never by a leading slash»
replace-with: and `send` classifies a typed text as one of them only when it is exactly a listed name, or that name, one space and a non-empty rest not starting with a space; the typed text
@leaf_edit n=2 leaf=.claude/docs/gotchas.md line=84 class=omission
leaf-text: A listed command with no post-condition, or any listed command on an unverified CLI version, is typed and answered `unconfirmable` at once, with no window.
source: ctr-wp2mvqrq «A listed local command whose post-condition is none, a stdout-echo command with no readable session transcript, or any listed command on an unverified CLI, is typed and answered ok with {"confirmed":false,"detail":"unconfirmable"} at once»
replace-with: A listed command with no post-condition, a stdout-echo command with no readable session transcript, or any listed command on an unverified CLI version, is typed and answered `unconfirmable` at once, with no window.
@leaf_edit n=3 leaf=.claude/docs/gotchas.md line=84 class=omission
leaf-text: Skills are not local commands.
source: dec-mqcmxqng «A local command whose post-condition is a stdout echo is confirmed by counting `<local-command-stdout>` tags in the session transcript past its end recorded before the paste; only a count crosses, never content»
replace-with: `/mcp`, alone or with arguments, is confirmed on a verified version by a new `<local-command-stdout>` tag in the session transcript past its end recorded before the paste (only a count crosses, the read bounded by `MAX_FRAME`; row `local-command-echo`, probed by Run B's seventh paste `/mcp disable viola-probe-absent`), with no hook awaited; a missed one is `not-delivered` / `no-prompt-submitted`. Skills are not local commands.
@leaf_edit n=4 leaf=CLAUDE.md line=85 class=omission
leaf-text: the one exception a ledger-listed local command whose post-condition is none or not yet measured, or whose CLI is unverified
source: dec-zztbfb2d «A listed local command whose post-condition is none or unmeasured, a stdout-echo one with no readable transcript, or any on an unverified CLI returns `ok`, `confirmed: false`, `unconfirmable`.»
replace-with: the one exception a ledger-listed local command whose post-condition is none or not yet measured, a stdout-echo one with no readable session transcript, or one whose CLI is unverified
@leaf_edit n=5 leaf=CLAUDE.md line=36 class=omission
leaf-text: content-bearing detail goes only to the instance's `detail-*.ndjson`;
source: dec-bl6fgjzy «The Claude transcript path is a redacted value, carried only as the optional transcript param beside hook.event, and appears in no event data, events log, log line, span, reply or verify record.»
replace-with: the Claude transcript path rides only as the optional `transcript` param beside `hook.event` and reaches no event data, events log, log line, span, reply or verify record; content-bearing detail goes only to the instance's `detail-*.ndjson`;
