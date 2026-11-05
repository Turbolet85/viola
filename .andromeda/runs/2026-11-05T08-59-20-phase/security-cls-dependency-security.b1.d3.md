@answer n=1 entity=dependency-security#1.1 v=yes
why: vendor/conpty/ and the ConPTY gate scripts/conpty-vendor.sh (--verify and --probe) lose their subject, both named as removed by the scope (task file 5, item 8)
@answer n=2 entity=dependency-security#2.1 v=yes
why: the deny.toml serde_json unbounded_depth bans.features entry must stay live (task item 9)
@answer n=3 entity=dependency-security#3.1 v=yes
why: the C-crate bans in deny.toml, deny-sync.toml, channel-tokio-free.sh and deny-probes.sh are named to stay live (task item 9)
@answer n=4 entity=dependency-security#4.1 v=yes
why: the fuzz/ test-only exemption and its lockfile audits in the ci.yml supply-chain step and nightly.yml advisories job are named to stay unchanged (task item 15)
@answer n=5 entity=dependency-security#5.1 v=yes
why: the four ConPTY pins live in src/conpty.rs, which the chunk removes along with vendor/conpty/ (task file 1, item 8)
@answer n=6 entity=dependency-security#6.1 v=yes
why: windows-mutants.yml is deleted by the chunk (task file 10, item 18), so the rules over that workflow bind its removal
@answer n=7 entity=dependency-security#7.1 v=yes
why: the G2 zero-panics exemption and its --probe are named by item 23, and G2 runs in the ci.yml test and perf jobs the chunk edits
@answer n=8 entity=dependency-security#8.1 v=no standing=S1
why: pinning actions by full SHA with a version comment and checkout with persist-credentials: false is the convention S1
