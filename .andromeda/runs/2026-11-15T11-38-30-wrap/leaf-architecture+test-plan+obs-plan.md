@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=89
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'sele
@leaf_edit n=1 leaf=.claude/docs/commands.md line=24 class=omission
leaf-text: on every build) `pass`: a seventeen-row stamp
source: dec-yhgeek4o «a stamp holding a retired ledger row verifies on the relied rows alone»
replace-with: on every build) `pass`, a retired row the stamp also holds changing nothing: a seventeen-row stamp
@leaf_edit n=2 leaf=.claude/docs/commands.md line=56 class=omission
leaf-text: `cargo deny check` — advisories, licences, sources, bans over `deny.toml`, the root `Cargo.lock`
source: dec-ibxlffrd «An advisory that sits only in an rmcp feature banned or off in the release graph is ignored with a reason naming that feature, standing until rmcp leaves <3.5.»
replace-with: `cargo deny check` — advisories, licences, sources, bans over `deny.toml`, the root `Cargo.lock`; no advisory is ignored but one sitting only in an rmcp feature banned or off in the release graph, ignored with a reason naming that feature until rmcp leaves <3.5
