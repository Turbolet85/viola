@answer n=1 entity=quality-gates-coverage-targets#1.1 v=no standing=S4
why: its rule that tests calling env_clear re-add LLVM_PROFILE_FILE is the convention S4 names, which every new test owes
@answer n=2 entity=quality-gates-coverage-targets#2.0 v=yes
why: it names the dispatch-only windows-mutants.yml, the Windows-host mutation scratch and the cfg(windows) mutants, all of which the chunk removes
@answer n=3 entity=quality-gates-coverage-targets#3.1 v=no
why: the perf session and its timed hook rows are a Linux harness step the chunk keeps unchanged, and the entity names no removed arm or OS job
