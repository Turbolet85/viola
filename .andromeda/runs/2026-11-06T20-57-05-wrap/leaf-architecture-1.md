@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/gotchas.md lines=147
last: - For architectural decisions and their rationale, see `doc:
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=15 class=stale
leaf-text: Five open divergences, not exceptions, the rule standing, their owner a CARRY on the working-route entry "Version done-check":
source: fct-xqkfyxga «The five, ratified as named exceptions: `viola-mcp` holds `ServeError` (`Runtime`, `Initialize`, `Session`; fixed messages, returned by `serve`) beside `McpError`»
replace-with: Five ratified exceptions, named in [Error Handling]:
@leaf_edit n=2 leaf=.claude/docs/gotchas.md line=23 class=stale
leaf-text: the pin stands on 0.8.1 being spike-proven.
source: fct-omwrdnlc «portable-pty `=0.9.0` pinned behind viola's own `pty` seam»
replace-with: the pin is `=0.9.0`, and `viola-pty` builds and passes its tests on it unchanged.
@leaf_edit n=3 leaf=.claude/docs/gotchas.md line=24 class=stale
leaf-text: keep `=0.8.1` behind the `pty` seam; moving it off `=0.8.1` (RUSTSEC-2026-0441) is owed to the working-route entry "Version done-check"; the named swap is portable-pty-psmux 0.9.7 if the O1 stray-spaces artifact reproduces only under `viola run`.
source: fct-omwrdnlc «0.9.0's graph holds neither `pin-utils` (RUSTSEC-2026-0441) nor the `serial` crate family, and `viola-pty` builds and passes its tests on it unchanged.»
replace-with: keep `=0.9.0` (pinned) behind the `pty` seam; 0.9.0's graph holds neither `pin-utils` (RUSTSEC-2026-0441) nor the `serial` crate family.
@leaf_edit n=4 leaf=.claude/docs/services/viola-agent-claude.md line=25 class=retired
leaf-text: stamped on live 2.1.287 on the Linux dev host only; its Windows reading owed to "Windows-only live measurements")
source: fct-va43dnf2 «and stamped on live 2.1.287 on the Linux dev host there (`evidence/live-round.md`: one prompt equal to the 113-byte text byte for byte)»
replace-with: stamped on live 2.1.287 on the Linux dev host)
@leaf_edit n=5 leaf=.claude/docs/services/viola-agent-claude.md line=23 class=stale
leaf-text: both `pid` and `sessionId` exposed, relied on every OS)
source: fct-yj2flshm «and the join is ledger row `agents-join`, relied on every build.»
replace-with: both `pid` and `sessionId` exposed, relied on every build)
