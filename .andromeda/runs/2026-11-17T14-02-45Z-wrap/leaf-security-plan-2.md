@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=45
last: - 2026-11-16: Write a bounded read as `take(reader, MAX_FRAME
@leaf_edit n=1 leaf=.claude/rules/security.md line=31 class=stale
leaf-text: the advisory ignore list holds exactly RUSTSEC-2026-0457, RUSTSEC-2026-0461 and RUSTSEC-2026-0462, each in an rmcp feature banned or off in the release graph, its reason naming it, standing until rmcp leaves its below-3.5 range;
source: dec-rmzsvauh «ignores exactly RUSTSEC-2026-0457, RUSTSEC-2026-0461 and RUSTSEC-2026-0462, each in an rmcp feature banned or off in the release graph, and RUSTSEC-2026-0543, chrono unmaintained with no safe upgrade.»
replace-with: the advisory ignore list holds exactly RUSTSEC-2026-0457, RUSTSEC-2026-0461 and RUSTSEC-2026-0462, each in an rmcp feature banned or off in the release graph, its reason naming it, standing until rmcp leaves its below-3.5 range, and RUSTSEC-2026-0543, chrono unmaintained with no safe upgrade, on the unmaintained-upstream ground, re-read at each epoch boundary;
@leaf_edit n=2 leaf=.claude/rules/security.md line=31 class=stale
leaf-text: the one admitted ignore is the Critical CVE response: within 7 days of an advisory confined to a disabled feature, an ignore entry whose reason names that feature
source: dec-sq2ejgbt «or an unmaintained advisory with no safe upgrade whose crate an upstream dependency brings into the graph; each carries its reason beside it.»
replace-with: an ignore is admitted on two grounds only, each with its reason beside it: the Critical CVE response (within 7 days of an advisory confined to a disabled feature, an ignore entry whose reason names that feature), or an unmaintained advisory with no safe upgrade whose crate an upstream dependency brings into the graph, re-read at every epoch boundary
