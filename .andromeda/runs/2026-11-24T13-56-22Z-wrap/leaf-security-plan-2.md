@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=47
last: - 2026-11-24: A new verb that starts through `run::start` in
@leaf_edit n=1 leaf=.claude/rules/security.md line=12 class=omission
leaf-text: `release`, `link` and `unlink` through `client::refuse_untrusted`
source: dec-pmdeyzjd «CLI send, wait, last, answer, pause, release, link, unlink, focus and switch run the instance strict-modes check before any frame or compositor read»
replace-with: `release`, `link`, `unlink` and `switch` through `client::refuse_untrusted`
@leaf_edit n=2 leaf=.claude/rules/security.md line=16 class=omission
leaf-text: and never ends or signals a process viola did not start.
source: dec-vxcpgd5u «A switch ends only the wrapper's own claude child, the one its own PTY handle holds, with no pid lookup, then resumes the session in place on the new pair.»
replace-with: and never ends or signals a process viola did not start; a switch ends only the wrapper's own claude child, the one its own PTY handle holds, with no pid lookup, then resumes the session in place on the new pair.
@leaf_edit n=3 leaf=.claude/rules/security.md line=16 class=omission
leaf-text: no address, pid or workspace on a process-log line.
source: dec-62b4vb7f «An instance's launch pair is kept only in the snapshot's launch field and the events log's launch lines, one at start and one per switch, never in the user's Claude settings»
replace-with: no address, pid or workspace on a process-log line; an instance's launch pair is kept only in the snapshot's launch field and the events log's launch lines, one at start and one per switch, never in the user's Claude settings, and leaves only through `list --json` and the MCP list item.
@leaf_edit n=4 leaf=.claude/rules/security.md line=19 class=omission
leaf-text: A non-`null` dialog decision requires a `viola verify` stamp
source: ctr-qrfoyeec «answer or switch refused unverified-cli: no verify stamp for the child's CLI version on which every relied ledger row counts (pass, or absent on the browser-tools-prompt row alone).»
replace-with: A non-`null` dialog decision, and a switch, requires a `viola verify` stamp
