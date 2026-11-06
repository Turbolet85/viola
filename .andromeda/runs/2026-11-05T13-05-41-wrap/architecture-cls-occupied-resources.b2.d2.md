@answer n=1 entity=occupied-resources#26.1 v=yes mode=stale
why: the pinned viola.exe-on-Windows path is retired and check_home_tree runs through strict.rs, whose Windows clauses are gone
@answer n=2 entity=occupied-resources#27.1 v=yes mode=stale
why: strict.rs and client.rs are reached, and the refusal wording "mode or ACL" names the Windows DACL arm the session removed
@answer n=3 entity=occupied-resources#28.1 v=yes mode=stale
why: the Windows NotFound skip and the per-OS oracle need rest on the windows-2025 leg and its strict-modes clauses, now removed
@answer n=4 entity=occupied-resources#29.1 v=yes mode=contradict
why: the Windows read rule that rejects a grant to a SID is removed with the strict-modes Windows clauses, so "every OS" is Linux alone
@answer n=5 entity=occupied-resources#30.1 v=no
why: the write rule on the home and its callers stay on the Unix check, and the removed Windows clauses cover diagnostics files and events.ndjson, not the home
@answer n=6 entity=occupied-resources#31.1 v=no
why: replace_private keeps its name and its 0600 whole replace, and only its retry loop was removed, which this entity does not describe
@answer n=7 entity=occupied-resources#32.1 v=yes mode=contradict
why: read_stamps_strict's "owner + DACL on Windows" arm is removed with the Windows owner/DACL strict-modes arm
@answer n=8 entity=occupied-resources#33.1 v=yes mode=stale
why: the Windows console-control stop arm is removed, so the Windows close, logoff and shutdown clause and the Ctrl-C reading rest on a removed arm
@answer n=9 entity=occupied-resources#34.1 v=yes mode=contradict
why: the CTRL_C_EVENT and CTRL_BREAK_EVENT arm of cmd::verify::stop is removed, leaving SIGINT and SIGTERM only
@answer n=10 entity=occupied-resources#35.1 v=yes mode=stale
why: the root Instant::now() + WITHIN count moved from 32 sites in 24 files to 31 in 23, and the Windows pipe-not-found clause is retired with the named pipe
@answer n=11 entity=occupied-resources#36.1 v=no
why: the entity states only the Unix unconnectable reading, which holds, and cleanup.rs is not among the reached files
@answer n=12 entity=occupied-resources#37.1 v=no
why: the error-body check in tests/support/events.rs carries no Windows clause and the session did not reach that file
@answer n=13 entity=occupied-resources#38.1 v=yes mode=contradict
why: the Windows debugger twin of the Frozen attach is removed with the Windows test code, so "one twin per OS" no longer holds
@answer n=14 entity=occupied-resources#39.1 v=no
why: the four exit-cause table checks name no code the session removed and tests/cross_exit_causes.rs is not reached
@answer n=15 entity=occupied-resources#40.1 v=no
why: the Linux dev-host home link and the /tmp/viola-<uid>/ fallback socket directory both stand, and no CI runner is named
@answer n=16 entity=occupied-resources#41.1 v=yes mode=contradict
why: the macOS TMPDIR arm of SOCKET_ROOT_VAR is gone, so the variable is XDG_RUNTIME_DIR on Linux alone
@answer n=17 entity=occupied-resources#42.1 v=yes mode=contradict
why: the relied set is all twenty-four rows on every build, so the "24 rows on Unix, 21 on Windows" reading is retired
@answer n=18 entity=occupied-resources#43.1 v=no
why: the harness session copy has no extension on Linux, the harness-<os> artifact keeps its matrix form, and no removed Windows file is named
@answer n=19 entity=occupied-resources#44.1 v=yes mode=contradict
why: the "On Windows the keepers are the plain create" clause is retired with the Windows branch of the home-base keepers
@answer n=20 entity=occupied-resources#45.1 v=no
why: the ubuntu end-to-end job keeps AGENT_RUN_KEEP_HOMES on the job and if: always() in ci.yml
@answer n=21 entity=occupied-resources#46.1 v=yes mode=contradict
why: seed_conpty, target/conpty-seed/ and its hard-linked seed dir are removed with the ConPTY sideload
@answer n=22 entity=occupied-resources#47.1 v=no
why: profraw-census names no Windows path and no CI job it names is changed by the session
@answer n=23 entity=occupied-resources#48.1 v=yes mode=contradict
why: scripts/junit-os-sets.py is deleted, and only one junit leg remains
@answer n=24 entity=occupied-resources#49.1 v=yes mode=contradict
why: viola-mutants-scratch, HOST_SCRATCH and the scratch_bytes key are retired with windows-mutants.yml
@answer n=25 entity=occupied-resources#50.1 v=yes mode=contradict
why: the windows-2025 DACL reading and the oracle that matched its SID string are retired with the Windows leg and dacl_of
