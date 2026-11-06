@answer n=1 entity=dependency-security#1.1 v=yes
why: deny.toml's serde_json unbounded_depth ban is read by a test and proven by a deny probe, both of which the chunk's deny gates must keep passing across the new portable-pty graph.
@answer n=2 entity=dependency-security#1.2 v=yes
why: the new portable-pty graph may leave a deny.toml licence exception naming a crate absent from it, and such an exception must be dropped, not kept.
@answer n=3 entity=dependency-security#2.1 v=yes
why: the C-building crate bans, the sync-crate tokio bans, the channel-tokio-free tree form and the deny probes must still pass with the new transitive crates (task items 3 and 7).
@answer n=4 entity=dependency-security#2.2 v=yes
why: clearing RUSTSEC-2026-0441 in cargo deny check advisories and CI's supply-chain job by moving portable-pty off =0.8.1 is the chunk's core supply-chain need, with no ignore entry widened.
@answer n=5 entity=dependency-security#2.3 v=yes
why: the chunk edits the workspace portable-pty =0.8.1 pin in Cargo.toml, and this entity's no-Windows-reason and move-off rule rules over that pin.
@answer n=6 entity=dependency-security#3.1 v=yes
why: the scope names the fuzz/Cargo.lock advisories and sources step as a gate the chunk must bring green, and this entity states how that audit's separate workspace is governed.
@answer n=7 entity=dependency-security#4.1 v=yes
why: the scope names the G2 zero-panics gate with its one FAKE_AGENT_HOOK_PANIC exemption as a gate the chunk must bring green, and its --probe must stay red for look-alike paths.
@answer n=8 entity=dependency-security#5.1 v=no standing=S5
why: the rule that actions are pinned by full SHA with a version comment is the standing convention S5 for every CI workflow change, and the chunk adds no action instance that the ci.yml SHA list must record.
