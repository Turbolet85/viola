@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=29 class=stale
leaf-text: the launch pair kept only in the snapshot's launch field and the events log's launch line, with no span
source: dec-62b4vb7f «An instance's launch pair is kept only in the snapshot's launch field and the events log's launch lines, one at start and one per switch, never in the user's Claude settings»
replace-with: the launch pair kept only in the snapshot's launch field and the events log's launch lines (one at start and one per switch), never in the user's Claude settings, with no span
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=36 class=omission
leaf-text: `cli_verified:false`, `answer` exit 12
source: ctr-qrfoyeec «answer or switch refused unverified-cli: no verify stamp for the child's CLI version on which every relied ledger row counts (pass, or absent on the browser-tools-prompt row alone).»
replace-with: `cli_verified:false`, `answer` or `switch` exit 12
@leaf_edit n=3 leaf=.claude/docs/security-summary.md line=65 class=omission
leaf-text: a plain session still running elsewhere is refused, never taken over.
source: dec-vxcpgd5u «A switch ends only the wrapper's own claude child, the one its own PTY handle holds, with no pid lookup, then resumes the session in place on the new pair.»
replace-with: a plain session still running elsewhere is refused, never taken over. A switch (provisional) ends only the wrapper's own claude child, the one its own PTY handle holds, with no pid lookup, then resumes the session in place on the new pair.
