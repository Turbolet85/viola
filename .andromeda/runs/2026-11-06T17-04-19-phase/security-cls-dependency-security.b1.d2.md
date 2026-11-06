@answer n=1 entity=dependency-security#1.1 v=yes
why: the serde_json unbounded_depth ban in deny.toml is witnessed by its deny-probes row and test, and the gates the chunk must bring green include the deny probes on the new graph.
@answer n=2 entity=dependency-security#1.2 v=yes
why: the portable-pty bump can drop crates from the graph, so any licence exception naming a crate that goes absent must be dropped in deny.toml, not kept.
@answer n=3 entity=dependency-security#2.1 v=yes
why: the C-building crate bans and the sync-crate tokio bans (deny-sync, channel-tokio-free, deny probes) must still pass with the new transitive crates.
@answer n=4 entity=dependency-security#2.2 v=yes
why: the RUSTSEC-2026-0441 advisories red that the portable-pty move off =0.8.1 must clear is the chunk's core fix, with no ignore widened.
@answer n=5 entity=dependency-security#2.3 v=yes
why: the rule that the portable-pty =0.8.1 pin carries no Windows reason governs the Cargo.toml pin edit and the new version the chunk records.
@answer n=6 entity=dependency-security#3.1 v=yes
why: CI's fuzz/Cargo.lock advisories and sources step is a gate the chunk must bring green, and this entity states how that separate audit and its exemption work.
@answer n=7 entity=dependency-security#4.1 v=yes
why: the G2 zero-panics gate and its one FAKE_AGENT_HOOK_PANIC exemption are a named gate the chunk must bring green.
@answer n=8 entity=dependency-security#5.1 v=no standing=S5
why: the rule that actions are pinned by full commit SHA with a version comment is the standing S5 convention for CI workflow changes.
