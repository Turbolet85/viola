@account n=1 entity=test-strategy#1.1 v=carried
why: the unit level's macOS carve-out names the `cfg(not(target_os = "macos"))` gates the chunk removes
@account n=2 entity=test-strategy#1.4 v=not_in_scope
why: its time-boxed ubuntu cargo-fuzz job is another nightly.yml job that only keeps holding, a re-check every change owes
@account n=3 entity=test-strategy#1.5 v=carried
why: it names the per-OS `perf` CI job whose Windows and macOS legs the chunk removes from ci.yml
@account n=4 entity=test-strategy#1.7 v=carried
why: the mutation level's Windows audit leg (windows-mutants.yml, the Windows mutation-scope test, the Windows-host scratch) leaves with the chunk
@account n=5 entity=test-strategy#1.8 v=carried
why: it names a Unix-gated contract case whose gate the census counts and the chunk re-points or retires
@account n=6 entity=test-strategy#2.1 v=carried
why: the shared root helpers' Windows and macOS arms and the sh/ps1 shim pair are removed or decided by the chunk
@account n=7 entity=test-strategy#2.2 v=carried
why: it homes the census binary `tests/contract_os_gated_tests.rs` the chunk re-points or retires
@account n=8 entity=test-strategy#3.1 v=carried
why: the IPC row names the Windows channel negatives and the macOS- and Unix-gated negatives the chunk removes or ungates
@account n=9 entity=test-strategy#3.6 v=carried
why: the filesystem row names the Windows protected-DACL negative the chunk removes
@account n=10 entity=test-strategy#3.8 v=carried
why: the supply-chain gates run over the Cargo.toml and deny.toml entries the chunk trims
@account n=11 entity=test-strategy#3.9 v=not_in_scope
why: the property-test row names no OS leg and the property suite is untouched
@account n=12 entity=test-strategy#3.13 v=carried
why: the multi-os-compat row is the trigger and census the chunk retires or re-points
@account n=13 entity=test-strategy#3.17 v=not_in_scope
why: the mutation trigger row names no OS leg and no mutation run belongs to this chunk
@account n=14 entity=test-strategy#3.18 v=carried
why: the no-flakes discipline rules over the chunk's own fix of the two host-disk-dependent msrv cases
@account n=15 entity=unit-test-strategy#1.2 v=carried
why: viola-channel's Windows and macOS golden vectors, handle non-inheritance and server pid arms leave with the chunk
@account n=16 entity=unit-test-strategy#1.3 v=carried
why: viola-state's Windows strict-modes cases and macOS server-verdict row leave or lose their gates with the chunk
@account n=17 entity=unit-test-strategy#1.4 v=carried
why: viola-agent-claude's Windows strip arm, npm shim, batch-child refusal and per-OS relied set are removed or decided by the chunk
@account n=18 entity=unit-test-strategy#2.1 v=carried
why: the root bin's macOS-gated `refused_inside` helper and its captured-span cases lose their gate with the chunk
