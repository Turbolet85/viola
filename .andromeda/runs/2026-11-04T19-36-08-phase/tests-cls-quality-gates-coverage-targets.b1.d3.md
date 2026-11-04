@answer n=1 entity=quality-gates-coverage-targets#1.1 v=yes
why: the per-OS coverage gate counts Unix child processes, and the measured kill-hangup mechanism (the fake agent's hooks in their own process groups) binds the Unix killed-wrapper reading and the ubuntu test job's profile merge.
@answer n=2 entity=quality-gates-coverage-targets#2.0 v=yes
why: the two cargo-mutants harness tests compiled out on macOS by cfg(not(target_os = "macos")) with their named runner-side reason are a per-OS gate the chunk's OS gating must name.
@answer n=3 entity=quality-gates-coverage-targets#3.1 v=yes
why: the perf gate's five hook rows timed against SPINE_DEADLINE on max, with the Unix socket-directory and peer-euid checks in the timed path, bind the per-OS perf job.
