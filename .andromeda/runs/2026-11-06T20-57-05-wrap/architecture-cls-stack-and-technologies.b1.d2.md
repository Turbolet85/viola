@answer n=1 entity=stack-and-technologies#1.1 v=no
why: the session names no toolchain, rust-version or msrv change, so the 1.98.1 pin and the 1.96 floor stand
@answer n=2 entity=stack-and-technologies#1.2 v=no
why: tokio 1.53.3 is not among the session's dependency moves and only viola-mcp enables it
@answer n=3 entity=stack-and-technologies#1.3 v=no
why: clap is not among the session's dependency moves
@answer n=4 entity=stack-and-technologies#1.4 v=yes mode=stale
why: portable-pty moves from =0.8.1 to =0.9.0 in Cargo.toml, while the openpty backend stays
@answer n=5 entity=stack-and-technologies#1.5 v=yes mode=stale
why: Cargo.lock adds serial2 and cfg_aliases and drops the serial-windows family, so the no-windows-sys reading of the reached lock must be re-read
@answer n=6 entity=stack-and-technologies#1.6 v=no
why: vt100 0.16.2 is not among the session's dependency moves and no screen-model edit is named
@answer n=7 entity=stack-and-technologies#1.7 v=no
why: interprocess 2.4.4 is unchanged and the live round re-witnessed the 0700 directory and 0600 socket
@answer n=8 entity=stack-and-technologies#1.8 v=no
why: the session adds no store, and the ndjson and snapshot persistence is untouched
@answer n=9 entity=stack-and-technologies#1.9 v=no
why: the session moves no record version and the diag schema files are not reached
@answer n=10 entity=stack-and-technologies#1.10 v=no
why: tempfile 3.27.0 and the write primitives are unchanged, and the reached fs-adjacent files change only EXE_SUFFIX joins and the .exe trim
@answer n=11 entity=stack-and-technologies#1.11 v=no
why: sha2 =0.11.0 is unchanged and the hash keys do not depend on the file-name edits
@answer n=12 entity=stack-and-technologies#1.12 v=no
why: serde, serde_json and serde_path_to_error are not among the session's dependency moves
@answer n=13 entity=stack-and-technologies#1.13 v=no
why: tracing, tracing-subscriber and veil are not among the session's dependency moves
@answer n=14 entity=stack-and-technologies#1.14 v=no
why: nutype 0.8.0 is not among the session's dependency moves
@answer n=15 entity=stack-and-technologies#1.15 v=no
why: chrono 0.4.45 is not among the session's dependency moves
@answer n=16 entity=stack-and-technologies#1.16 v=no
why: thiserror 2.0.20 and the hand-written PtyError stand, and the T1 ratification lives in the Error Handling convention rather than this row
@answer n=17 entity=stack-and-technologies#1.17 v=no
why: the message broker row names no crate or edit the session touches
@answer n=18 entity=stack-and-technologies#1.18 v=no
why: sysinfo 0.39.6 is unchanged and the session liveness contract is untouched
@answer n=19 entity=stack-and-technologies#1.19 v=no
why: rmcp 3.4.1 and its tool set are unchanged
@answer n=20 entity=stack-and-technologies#1.20 v=no
why: viola still drives the unmodified claude CLI, which the live round re-witnessed
@answer n=21 entity=stack-and-technologies#1.21 v=no
why: the mobile deferral is not reached by the session
@answer n=22 entity=stack-and-technologies#1.22 v=no
why: cargo install and the plugin written at start were re-witnessed unchanged on the live round
@answer n=23 entity=stack-and-technologies#1.23 v=no
why: the CI workflow is not among the reached files and CI was green on all seven jobs on 849aad18
@answer n=24 entity=stack-and-technologies#1.24 v=no
why: the lint tool list is unchanged, and the deny.toml ignore-list edit does not alter the cargo-deny row
@answer n=25 entity=stack-and-technologies#1.25 v=no
why: dist and self_update are not reached by the session
