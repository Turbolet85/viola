@answer n=1 entity=e2e-test-strategy#19.2 v=yes mode=contradict
why: the census gained a fault class (F1) so a gate naming windows or target_os = "macos" is a fault even when a GATES row names it, and the clause that says faults reads both ways no longer holds
@answer n=2 entity=e2e-test-strategy#19.3 v=yes mode=stale
why: the session removed the windows and macOS gated tests and un-gated hook_fail_open's cfg_attr pair, so the 52 functions, 7 gated files and 59 GATES rows count moved
