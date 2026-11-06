@answer n=1 entity=integration-test-strategy#1.1 v=no
why: the session's Cargo, Windows and dependency edits do not reach the module-to-module boundary or the rstest 0.27.0 pin, which stand as written
@answer n=2 entity=integration-test-strategy#1.2 v=no
why: the dependency move names only portable-pty, nix, the serial family, pin-utils, memoffset and termios, so tempfile 3.27.0 and the concurrent-append tests stand
@answer n=3 entity=integration-test-strategy#1.3 v=no
why: src/bin/viola-fake-agent.rs only drops the EXE_SUFFIX join and its binary is still copied to the test PATH as claude, so the external-API claim holds
@answer n=4 entity=integration-test-strategy#1.4 v=no
why: the socket-directory claims are the Unix 0700/0600 modes and refusals, which the Windows-remnant edits to the reached test files leave unchanged
@answer n=5 entity=integration-test-strategy#1.5 v=yes mode=stale
why: the Tools line names portable-pty =0.8.1, now =0.9.0 under the viola-pty seam, while the exit-on-handle no-EOF claim stands
@answer n=6 entity=integration-test-strategy#1.6 v=no
why: the hook instance check and VIOLA_DIR canonicalisation on Linux are unchanged, since the plain_drive and verbatim-prefix removals only touch Windows paths
@answer n=7 entity=integration-test-strategy#2.1 v=no
why: the placed-copy ETXTBSY witness lives in crates/viola-state/src/fs.rs, which the session does not reach
@answer n=8 entity=integration-test-strategy#2.2 v=no
why: the harness cleanup endpoint-gone witness is #[cfg(unix)] in a reached file, and the report leaves its Unix claim as it was
@answer n=9 entity=integration-test-strategy#3.0 v=no
why: the home.rs edit only drops the verbatim-prefix strip, which never applied on Linux, so prepare_home_base, wait_ready and the outside_scan carve-out stand
@answer n=10 entity=integration-test-strategy#4.1 v=no
why: the wrapper channel, send and answer refusals and the reached channel_endpoint and cli_send cases name no rewritten-path warning, drive or Windows reading the session moved
@answer n=11 entity=integration-test-strategy#4.2 v=no
why: the on-disk strict-modes, EventKind and refusal claims hold on the reached run_cli, cli_verify and cli_list files because the session moves no permission or count
@answer n=12 entity=integration-test-strategy#5.1 v=yes mode=stale
why: it names viola.exe as the help usage reading, the plain_drive_* units and the cli_slash_args rewritten-path warning, all of which the session moved
