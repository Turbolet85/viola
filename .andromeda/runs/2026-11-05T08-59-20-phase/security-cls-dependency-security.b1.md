@answer n=1 entity=dependency-security#1.1 v=yes
why: the vendor/conpty/ tree and the conpty-vendor.sh gate are units the scope names (task_file n=5, task_item n=8), so the rule over that gate binds the removal
@answer n=2 entity=dependency-security#2.1 v=yes
why: deny.toml is edited, and the serde_json bans.features entry (unbounded_depth) must stay live under task_item n=9
@answer n=3 entity=dependency-security#3.1 v=yes
why: the C-crate bans in deny.toml, deny-sync.toml, channel-tokio-free.sh and deny-probes.sh must stay live (task_item n=9), and deny.toml is edited
@answer n=4 entity=dependency-security#4.1 v=yes
why: the nightly.yml advisories job over fuzz/Cargo.lock and the fuzz/ test-only exemption stay on Linux (task_item n=15), and nightly.yml is edited
@answer n=5 entity=dependency-security#5.1 v=yes
why: src/conpty.rs (task_file n=1) holds the four pins and is removed, and conpty-vendor.sh parses them, so it loses its subject (task_item n=8)
@answer n=6 entity=dependency-security#6.1 v=yes
why: windows-mutants.yml is deleted whole (task_file n=10, task_item n=18), and its dispatch-only, never-a-gate and never-a-ci.yml-dependency rules bind that removal; its hardening lines go with the deleted job
@answer n=7 entity=dependency-security#7.1 v=yes
why: the G2 zero-panics single exemption and its --probe run in the test and perf jobs of ci.yml, which the chunk edits (task_item n=23)
@answer n=8 entity=dependency-security#8.1 v=yes
why: the chunk's own workflow hardening (task_item n=11) rules over the SHA pins of the actions kept in ci.yml and nightly.yml, the scope's own obligation, so not dropped as standing S1
