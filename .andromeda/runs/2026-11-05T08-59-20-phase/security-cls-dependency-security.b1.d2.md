@answer n=1 entity=dependency-security#1.1 v=yes
why: the ConPTY vendor gate and its binaries (vendor/conpty/, src/conpty.rs) are removed, and item 8 retires the gate's subject
@answer n=2 entity=dependency-security#2.1 v=yes
why: item 9 names the serde_json bans.features entries as staying live in deny.toml
@answer n=3 entity=dependency-security#3.1 v=yes
why: item 9 names the C-crate bans, deny-sync.toml, channel-tokio-free.sh and deny-probes.sh as staying live
@answer n=4 entity=dependency-security#4.1 v=yes
why: item 15 keeps the nightly fuzz/Cargo.lock advisories check and the fuzz/ test-only exemption unchanged
@answer n=5 entity=dependency-security#5.1 v=yes
why: the four ConPTY pins live in src/conpty.rs, which goes, and item 8 retires the script that parses them
@answer n=6 entity=dependency-security#6.1 v=yes
why: the scope names windows-mutants.yml and its dispatch-only workflow for removal (item 18, file n=10)
@answer n=7 entity=dependency-security#7.1 v=yes
why: item 23 keeps the G2 zero-panics exemption and its --probe, which the test and perf jobs run
@answer n=8 entity=dependency-security#8.1 v=yes
why: item 11 names SHA-pinned actions with version comments and persist-credentials false on every edited workflow, and ci.yml and nightly.yml keep these pins
