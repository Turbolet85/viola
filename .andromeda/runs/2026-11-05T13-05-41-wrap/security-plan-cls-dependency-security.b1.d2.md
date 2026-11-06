@answer n=1 entity=dependency-security#1.1 v=yes mode=contradict
why: the session deletes vendor/conpty/ and scripts/conpty-vendor.sh, so the vendored ConPTY audit no longer exists.
@answer n=2 entity=dependency-security#2.1 v=no
why: the deny.toml change moves only targets and drops the recvmsg exception, and CI run 42030025050 is red only on RUSTSEC-2026-0441, so the serde_json unbounded_depth ban still fires.
@answer n=3 entity=dependency-security#3.1 v=yes mode=stale
why: deny-sync.toml targets move from three triples to x86_64-unknown-linux-gnu alone, so the three-triple count is stale.
@answer n=4 entity=dependency-security#4.1 v=no
why: the session does not touch fuzz/ or its lockfile, and the ci.yml supply-chain fuzz audit step stays green in CI run 42030025050.
@answer n=5 entity=dependency-security#5.1 v=yes mode=contradict
why: src/conpty.rs and the .gitattributes vendor/conpty/** binary rule are removed, so the four ConPTY pins no longer exist.
@answer n=6 entity=dependency-security#6.1 v=yes mode=contradict
why: windows-mutants.yml is deleted, so the dispatch-only workflow and its permissions and upload facts no longer hold.
@answer n=7 entity=dependency-security#7.1 v=no
why: the G2 single exemption is untouched per the session, and neither scripts/g2-zero-panics.sh nor src/cmd/hook/seam.rs is reached.
@answer n=8 entity=dependency-security#8.1 v=no
why: ci.yml keeps the same four SHA-pinned actions on the ubuntu-latest matrix, and the session adds or bumps no action.
