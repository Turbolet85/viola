@answer n=1 entity=quality-gates-coverage-targets#1.1 v=no standing=S4
why: its binding line, that tests calling env_clear must re-add LLVM_PROFILE_FILE, is the convention of standing S4, and the coverage-profile mechanism it records is not touched by the removal
@answer n=2 entity=quality-gates-coverage-targets#2.0 v=yes
why: the two real-cargo-mutants tests compiled out by the macOS cfg leave with that arm, and the Windows mutation leg (windows-mutants.yml, host mutation scratch, cfg(windows) host exclusion) is removed by the chunk
@answer n=3 entity=quality-gates-coverage-targets#3.1 v=yes
why: the scope names the peer euid wiring (item 20) and the socket-directory check's tests (item 11), so the two checks the perf gate times on the Linux hook path must stay real after their Windows arms leave
