@row_retire n=1 row=decision/rmcp-advisory-ignores seen=1 heir=decision/advisory-ignore-list detector=D-security-deps
why: The advisory ignore list no longer holds exactly the three disabled-rmcp-feature ids: it holds four, the fourth on the unmaintained-upstream ground, so a row keyed and named for the rmcp ignores alone no longer states the list.
basis: fct-r6cpzaxa
because: The ignore list moved from three ids to four with RUSTSEC-2026-0543, so the row naming exactly three is false and its heir states the whole list.
@row_add n=1 row=decision/advisory-ignore-grounds area=dependency-security detector=D-security-deps
name: Advisory ignores admitted on two grounds only
rule: An advisory ignore stands on one of two grounds: the Critical CVE response to an advisory confined to a disabled feature, or an unmaintained advisory with no safe upgrade whose crate an upstream dependency brings into the graph; each carries its reason beside it.
why: The second ground widens the Critical CVE response; each unmaintained-upstream ignore is re-read at every epoch boundary, its only control.
basis: fct-s6xve3jr
because: The advisory rule was widened from the disabled-feature response alone to a second, unmaintained-upstream ground, and no standing row states the rule with both grounds.
@row_add n=2 row=decision/advisory-ignore-list area=dependency-security detector=D-security-deps
name: Closed advisory ignore list
rule: cargo-deny treats unmaintained and unsound advisories as failures, denies yanked crates and ignores exactly RUSTSEC-2026-0457, RUSTSEC-2026-0461 and RUSTSEC-2026-0462, each in an rmcp feature banned or off in the release graph, and RUSTSEC-2026-0543, chrono unmaintained with no safe upgrade.
why: The rmcp ids stand under the Critical CVE response until rmcp leaves its below-3.5 range; the chrono id stands on the unmaintained-upstream ground, re-read at each epoch boundary.
basis: fct-r6cpzaxa
because: The ignore list now holds four ids, the fourth on the unmaintained-upstream ground, and replaces the row that named exactly the three rmcp ids.
