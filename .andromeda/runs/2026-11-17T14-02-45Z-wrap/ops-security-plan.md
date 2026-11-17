@row_retire n=1 row=decision/rmcp-advisory-ignores seen=1 heir=decision/advisory-ignore-list detector=D-security-deps
why: The advisory ignore list no longer holds exactly the three disabled-rmcp-feature ids: it holds four, the fourth on the unmaintained-upstream ground, so a row keyed and named for the rmcp ignores alone no longer states the list.
basis: fct-r6cpzaxa
because: The ignore list moved from three ids to four with RUSTSEC-2026-0543, so the row naming exactly three is false and its heir states the whole list.
@row_add n=1 row=decision/advisory-ignore-grounds area=dependency-security fact=F1 detector=D-security-deps
name: Advisory ignores admitted on two grounds only
rule: An advisory ignore is admitted only as the Critical CVE response for a disabled rmcp feature or for an unmaintained upstream with no safe upgrade; each entry carries its reason, the comment over the list states both grounds, and each unmaintained-upstream ignore is re-read at every epoch boundary.
basis: fct-s6xve3jr
because: The advisory rule was widened from the disabled-feature response alone to a second, unmaintained-upstream ground, and no standing row states the rule with both grounds.
@row_add n=2 row=decision/advisory-ignore-list area=dependency-security fact=F2 detector=D-security-deps
name: Closed advisory ignore list
rule: cargo-deny treats unmaintained and unsound advisories as failures, denies yanked crates and ignores exactly RUSTSEC-2026-0457, RUSTSEC-2026-0461 and RUSTSEC-2026-0462, for rmcp features banned or off in the release graph, and RUSTSEC-2026-0543, chrono unmaintained with no safe upgrade.
why: The three rmcp ids stand under the Critical CVE response for a disabled feature, the chrono id under the unmaintained-upstream ground.
basis: fct-r6cpzaxa
because: The ignore list now holds four ids, the fourth on the unmaintained-upstream ground, and replaces the row that named exactly the three rmcp ids.
@row_none n=1 fact=F3 type=4
relied: A stopped viola verify's own guard ends its children, removes its own run dirs and writes no stamp, which the probe-directory cleanup rows already state; the test harness's parent-death tie that makes a killed test stop verify by SIGTERM is how the e2e harness is built, held by the code and test-plan's harness row.
