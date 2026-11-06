@answer n=1 entity=occupied-resources#26.1 v=yes mode=contradict
why: the pinned binary is named viola.exe on Windows and the Windows clauses of the strict-modes check are removed
@answer n=2 entity=occupied-resources#27.1 v=yes mode=stale
why: the refusal leaves the mode or ACL as found, and the ACL reading is the Windows DACL arm the session removed from the strict-modes check
@answer n=3 entity=occupied-resources#28.1 v=yes mode=contradict
why: the Windows NotFound behaviour under a regular file and its per-OS oracle measured on windows-2025 are Windows-only and the Windows target is removed
@answer n=4 entity=occupied-resources#29.1 v=yes mode=contradict
why: the Windows clause that one granting read to a SID trusted_sid rejects is gone with the Windows owner and DACL strict-modes arm
@answer n=5 entity=occupied-resources#30.1 v=no
why: the write rule on the home is the Unix check body, which the session left unchanged, so the claim still holds on the changed strict.rs
@answer n=6 entity=occupied-resources#31.1 v=no
why: the budget read still runs strict::check_path, takes the lock and replaces through replace_private, all unchanged on Unix
@answer n=7 entity=occupied-resources#32.1 v=yes mode=contradict
why: read_stamps_strict's owner plus DACL on Windows arm is removed by the session's strict-modes change
@answer n=8 entity=occupied-resources#33.1 v=yes mode=stale
why: the Windows close, logoff and shutdown reading names a platform the session retired from the verify stop arm
@answer n=9 entity=occupied-resources#34.1 v=yes mode=contradict
why: the Windows CTRL_C_EVENT and CTRL_BREAK_EVENT stop arm of cmd::verify::stop is removed, leaving SIGINT and SIGTERM only
@answer n=10 entity=occupied-resources#35.1 v=yes mode=stale
why: the Instant::now() + WITHIN site count moved from 32 in 24 files to 31 in 23, and the Windows pipe clause is retired
@answer n=11 entity=occupied-resources#36.1 v=no
why: crates/viola-e2e cleanup.rs is not among the files the session reached and its UnixStream connect reading still holds on Unix
@answer n=12 entity=occupied-resources#37.1 v=no
why: the error-body check in tests/support/events.rs has no Windows or macOS clause and the session did not reach that file
@answer n=13 entity=occupied-resources#38.1 v=yes mode=contradict
why: the Windows debugger twin of the Frozen guard under its own gate is Windows code the session removed, so one twin per OS no longer holds
@answer n=14 entity=occupied-resources#39.1 v=no
why: the exit-cause table checks in tests/cross_exit_causes.rs are unchanged and the CLI exit code set stands
@answer n=15 entity=occupied-resources#40.1 v=no
why: the Linux dev-host tmpfs home under /tmp is not a Windows or macOS claim and the /tmp/viola-<uid> socket form is unchanged
@answer n=16 entity=occupied-resources#41.1 v=yes mode=contradict
why: SOCKET_ROOT_VAR is no longer TMPDIR on macOS because the macOS TMPDIR socket dir is retired, so it reads XDG_RUNTIME_DIR alone
@answer n=17 entity=occupied-resources#42.1 v=yes mode=contradict
why: the relied set is now all twenty-four rows on every build, so the 21-on-Windows reading of the 2.1.287 stamp no longer holds
@answer n=18 entity=occupied-resources#43.1 v=yes mode=stale
why: the bin/claude[.exe] session copy names a Windows binary form the session retired with the npm-shim and agent-run.ps1 removal
@answer n=19 entity=occupied-resources#44.1 v=yes mode=contradict
why: the statement that on Windows the keepers are the plain create describes a Windows arm the session removed from prepare_home_base and ensure_e2e_home
@answer n=20 entity=occupied-resources#45.1 v=yes mode=stale
why: ci.yml is reached whole, its matrix legs and pwsh steps are cut, and the job env this entity cites is not shown to hold
@answer n=21 entity=occupied-resources#46.1 v=yes mode=contradict
why: the seed_conpty hard links into target/conpty-seed are gone with the ConPTY sideload, so the icacls inheritance hazard no longer exists
@answer n=22 entity=occupied-resources#47.1 v=no
why: the profraw census script and its target/profraw-census tallies are not among the session's reached files and name no Windows claim
@answer n=23 entity=occupied-resources#48.1 v=yes mode=contradict
why: scripts/junit-os-sets.py is deleted and its three per-OS junit artifacts are gone
@answer n=24 entity=occupied-resources#49.1 v=yes mode=contradict
why: viola-mutants-scratch, HOST_SCRATCH and the scratch_bytes key are retired with windows-mutants.yml
@answer n=25 entity=occupied-resources#50.1 v=yes mode=contradict
why: the protected owner-only DACL reading on windows-2025 is a Windows-only measurement and that runner is removed
