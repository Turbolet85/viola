@answer n=1 entity=dependency-security#1.1 v=yes
why: deny.toml is reached whole, and the portable-pty graph change must keep its serde_json unbounded_depth ban and deny probe passing.
@answer n=2 entity=dependency-security#1.2 v=yes
why: the moved graph may leave a licence exception naming an absent crate, which deny.toml's licence list must then drop.
@answer n=3 entity=dependency-security#2.1 v=yes
why: task item 3 requires the C-build bans, sync-crate tokio bans, channel-tokio-free check and deny probes to pass with the new transitive crates.
@answer n=4 entity=dependency-security#2.2 v=yes
why: this is the task's supply-chain item 1: cargo deny check advisories is red on RUSTSEC-2026-0441, cleared by moving the portable-pty pin with no ignore widened.
@answer n=5 entity=dependency-security#2.3 v=yes
why: it rules over the portable-pty =0.8.1 pin that the chunk edits in Cargo.toml, and the Windows reason behind that pin.
@answer n=6 entity=dependency-security#3.1 v=yes
why: task item 7 names the fuzz/Cargo.lock advisories and sources step of CI's cargo deny gate as a gate the chunk must bring green.
@answer n=7 entity=dependency-security#4.1 v=yes
why: task item 7 names the G2 zero-panics gate and its one FAKE_AGENT_HOOK_PANIC exemption as a gate the chunk must keep green.
@answer n=8 entity=dependency-security#5.1 v=no standing=S5
why: the full-SHA action pinning of CI workflows is the convention standing S5 states, and the chunk changes no workflow.
