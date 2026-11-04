@answer n=1 entity=dependency-security#1.1 v=no
why: the vendored ConPTY blobs and the conpty-vendor.sh gate are not touched by the Unix openpty path, and the chunk re-measures no part of that gate
@answer n=2 entity=dependency-security#2.1 v=no
why: the serde_json unbounded_depth ban in deny.toml is outside the chunk; it adds no dependency that enables that feature and does not edit deny.toml
@answer n=3 entity=dependency-security#3.1 v=no
why: the C-build and tokio bans in deny.toml and deny-sync.toml stay as they are; the chunk adds no tokio or C-building crate
@answer n=4 entity=dependency-security#4.1 v=no
why: the fuzz/ separate workspace and its lockfile audit are outside the chunk, which does not touch fuzz (task not_needed item 6)
@answer n=5 entity=dependency-security#5.1 v=yes
why: the scope names the ConPTY companion sideload as a Windows control a shared pty-seam change must leave as it is, and this entity pins that sideload's four values in src/conpty.rs
@answer n=6 entity=dependency-security#6.1 v=no
why: windows-mutants.yml is neither reached nor named by the chunk, which edits ci.yml only, and its dispatch-only ruling is its own
@answer n=7 entity=dependency-security#7.1 v=yes
why: task items 14 and 15 change the G2 zero-panics gate on ubuntu-latest and macos-latest, and this entity states its one exemption for the FAKE_AGENT_HOOK_PANIC seam's panic_location
@answer n=8 entity=dependency-security#8.1 v=no standing=S3
why: the full-SHA action pin set is the CI convention every new CI step owes, and the chunk names no action outside the set this entity lists
