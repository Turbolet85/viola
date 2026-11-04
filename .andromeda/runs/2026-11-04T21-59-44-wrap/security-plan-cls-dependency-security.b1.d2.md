@answer n=1 entity=dependency-security#1.1 v=no
why: the ConPTY vendor files, script and src/conpty.rs pins sit in no file the session reached, and the session moved no vendor pin
@answer n=2 entity=dependency-security#2.1 v=no
why: deny.toml's serde_json ban and its probe are in no file the session reached, and no lockfile or deny.toml entry changed
@answer n=3 entity=dependency-security#3.1 v=no
why: the sync-crate bans in deny.toml, deny-sync.toml and the channel tree script are unreached and the session made no ban change
@answer n=4 entity=dependency-security#4.1 v=no
why: the fuzz/ workspace and its lockfile are unreached and the session moved no fuzz or lockfile
@answer n=5 entity=dependency-security#5.1 v=no
why: the four ConPTY pins in src/conpty.rs are unreached and the session moved no vendored package
@answer n=6 entity=dependency-security#6.1 v=no
why: windows-mutants.yml is unreached and the session changed no workflow, permission, pin or upload
@answer n=7 entity=dependency-security#7.1 v=no
why: the G2 exemption lives in scripts/g2-zero-panics.sh and seam.rs, both unreached, and the session added no seam or exemption
@answer n=8 entity=dependency-security#8.1 v=no
why: the SHA-pinned action set lives in ci.yml, which the session did not reach, and no action pin changed
