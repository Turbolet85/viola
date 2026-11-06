@account n=1 entity=occupied-resources#27.1 v=proposed
why: its «mode or ACL left as found» names the Windows owner/DACL arm the report retires from strict.rs
@account n=2 entity=occupied-resources#28.1 v=proposed
why: the Windows NotFound skip and the per-OS oracle measured on windows-2025 are retired with the Windows strict-modes arm
@account n=3 entity=occupied-resources#29.1 v=proposed
why: its «on every OS» and Windows SID read clause are stale now that board.rs lost its OS arms and the owner/DACL arm is gone
@account n=4 entity=occupied-resources#32.1 v=proposed
why: read_stamps_strict's «owner + DACL on Windows» clause names the removed Windows strict.rs arm
@account n=5 entity=occupied-resources#33.1 v=proposed
why: the «Windows close, logoff or shutdown» clause belongs to the console-control stop arm the report removes
@account n=6 entity=occupied-resources#34.1 v=proposed
why: it names the Windows CTRL_C_EVENT / CTRL_BREAK_EVENT stop arm that stop.rs no longer has
@account n=7 entity=occupied-resources#35.1 v=proposed
why: the WITHIN-wait count moved from 32 sites in 24 files to 31 in 23 and its Windows pipe-not-found arm is retired
@account n=8 entity=occupied-resources#38.1 v=proposed
why: frozen.rs keeps only the Unix kill -STOP twin ungated, so the per-OS twins and the Windows debugger twin are stale
@account n=9 entity=occupied-resources#41.1 v=proposed
why: SOCKET_ROOT_VAR now reads XDG_RUNTIME_DIR alone, the macOS TMPDIR arm gone
@account n=10 entity=occupied-resources#42.1 v=proposed
why: the relied set's «21 on Windows» is gone and LedgerRow::relied() is all twenty-four rows on every build
@account n=11 entity=occupied-resources#43.1 v=proposed
why: the bin/claude[.exe] naming is a Windows form on a product that is now Linux only
@account n=12 entity=occupied-resources#44.1 v=proposed
why: its «On Windows the keepers are the plain create» clause and its Unix qualifier are stale under the Linux-only target
@account n=13 entity=occupied-resources#45.1 v=not_touched
why: the report keeps AGENT_RUN_KEEP_HOMES and every other ci.yml step, so the claim still holds on the changed file
@account n=14 entity=occupied-resources#46.1 v=proposed
why: the icacls grant through seed_conpty into target/conpty-seed is retired whole with the ConPTY sideload
@account n=15 entity=occupied-resources#48.1 v=proposed
why: scripts/junit-os-sets.py is deleted and the three junit-<os> artifacts retired
@account n=16 entity=occupied-resources#49.1 v=proposed
why: the Windows scratch arm (HOST_SCRATCH, scratch_bytes, windows-mutants.yml) is retired, only the dev-host TMPDIR fact stays
@account n=17 entity=occupied-resources#50.1 v=proposed
why: the windows-2025 owner-only DACL SDDL measurement belongs to the retired Windows owner/DACL arm
@account n=18 entity=occupied-resources#50.2 v=proposed
why: the windows-2025 temp-dir ACL oracle and strict::win tests are retired with the win module
