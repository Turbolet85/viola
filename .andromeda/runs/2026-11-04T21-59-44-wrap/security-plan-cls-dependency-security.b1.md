@answer n=1 entity=dependency-security#1.1 v=no
why: the ConPTY vendor files, src/conpty.rs and scripts/conpty-vendor.sh are not reached and no src/ or vendor file moved
@answer n=2 entity=dependency-security#2.1 v=no
why: deny.toml and scripts/deny-probes.sh are not reached and the session names no serde_json or deny-probes change, so the ban and its probe still hold
@answer n=3 entity=dependency-security#3.1 v=no
why: Cargo.toml and Cargo.lock are unchanged and deny-sync.toml, scripts/sync-crates.txt, scripts/channel-tokio-free.sh and scripts/deny-probes.sh are not reached, so the tokio bans still hold
@answer n=4 entity=dependency-security#4.1 v=no
why: fuzz/ and its lockfile are not reached and no crate was added or bumped, so the fuzz exemption still holds
@answer n=5 entity=dependency-security#5.1 v=no
why: the four ConPTY pins in src/conpty.rs are not reached and no src/ file moved
@answer n=6 entity=dependency-security#6.1 v=no
why: no workflow changed, so windows-mutants.yml and its permissions and pins still hold
@answer n=7 entity=dependency-security#7.1 v=no
why: scripts/g2-zero-panics.sh and src/cmd/hook/seam.rs are not reached and no seam or env read was added, so the single G2 exemption still holds
@answer n=8 entity=dependency-security#8.1 v=no
why: no action pin, workflow or ci.yml step changed, and the only .config/ change is the nextest kill line
