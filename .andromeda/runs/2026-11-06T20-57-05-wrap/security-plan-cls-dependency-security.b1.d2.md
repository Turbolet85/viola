@answer n=1 entity=dependency-security#1.1 v=no
why: deny-probes 14 of 14 banned with a clean control, re-read unmoved, and cargo deny bans ok on the changed deny.toml, so the serde_json unbounded_depth ban still holds
@answer n=2 entity=dependency-security#1.2 v=no
why: cargo deny licenses reads ok on the changed deny.toml, and the session changed only the advisories ignore list, not how licenses exceptions are read
@answer n=3 entity=dependency-security#2.1 v=no
why: deny-probes 14 of 14 banned with a clean control, re-read unmoved, and cargo deny bans ok, so the C-build and tokio bans hold on the changed deny.toml
@answer n=4 entity=dependency-security#2.2 v=yes mode=contradict
why: portable-pty moved to =0.9.0, which drops pin-utils, so RUSTSEC-2026-0441 is closed and advisories no longer reads red; the advisories ignore list is emptied, RUSTSEC-2017-0008 gone
@answer n=5 entity=dependency-security#2.3 v=yes mode=stale
why: the portable-pty pin cited as =0.8.1 moved to =0.9.0 in workspace.dependencies
@answer n=6 entity=dependency-security#3.1 v=no
why: the fuzz/ lockfile and its audit steps are not in the change, and the root deny.toml walk-up resolution and its C-build scope are unchanged
@answer n=7 entity=dependency-security#4.1 v=no
why: the seam file src/cmd/hook/seam.rs and scripts/g2-zero-panics.sh are not in the change, and no G2 exemption was added
@answer n=8 entity=dependency-security#5.1 v=no
why: no workflow, script or toolchain file changed, so the pinned action SHAs of ci.yml are untouched
