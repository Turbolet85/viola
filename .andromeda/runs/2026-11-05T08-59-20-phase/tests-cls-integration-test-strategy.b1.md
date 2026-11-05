@answer n=1 entity=integration-test-strategy#1.1 v=no
why: generic module-to-module approach (direct crate API calls, rstest fixtures) with no OS-gated rule; the removal leaves it as it is
@answer n=2 entity=integration-test-strategy#1.2 v=yes
why: names the all-3-OS concurrent-append runs (hook_processes_append_whole_lines_side_by_side, hook_panics_append_whole_lines_over_4_kib_side_by_side) that task item 13 and the CI-matrix amendment remove, while the Linux whole-line rule stays
@answer n=3 entity=integration-test-strategy#1.3 v=yes
why: the Windows npm shim .cmd fixture beside a stub claude.exe is a Windows-only fixture the chunk removes with its arm (task item 10)
@answer n=4 entity=integration-test-strategy#1.4 v=yes
why: the Windows pipe DACL SDDL read-back is a cfg(windows) test removed under task item 8, and the Unix socket-dir 0700/0600 and check_socket_dir rules it also states are Linux behaviour the chunk keeps
@answer n=5 entity=integration-test-strategy#1.5 v=yes
why: names the windows-2025 sideloaded-ConPTY runs, the DA1 answer under the sideload and the cfg(windows) .cmd child refusal, all removed under task items 7 and 10; the Linux openpty resize rules stay
@answer n=6 entity=integration-test-strategy#1.6 v=no
why: hook exec over fixture stdin states Linux instance-check and fail-open behaviour with no OS-gated rule, and the removal reaches no hook test or hook path it names
@answer n=7 entity=integration-test-strategy#2.1 v=yes
why: names the Windows form place_private_new_placed_copy_runs_at_once and its windows-2025 green leg, plus the macOS green leg, all removed with their arms; the Linux ETXTBSY witness stays
@answer n=8 entity=integration-test-strategy#2.2 v=yes
why: the #[cfg(unix)] gate of cleanup_after_a_force_killed_wrapper_reads_the_endpoint_gone is tied to the macOS runner the chunk removes, and the gate may lose its need under task item 12
@answer n=9 entity=integration-test-strategy#3.0 v=yes
why: names seed_conpty for Windows x64 homes, the windows-2025 runner-inherited grants and the macOS sample arm of note_stall, all in tests/support/home.rs and removed under task items 7, 8 and 11; the Linux /proc stall path stays
@answer n=10 entity=integration-test-strategy#4.1 v=yes
why: names the macOS carve-out (no peer pid, EPIPE or ENOTCONN on close) and the Windows server-identity negatives, both arms the chunk removes; the Linux wire contract and reset behaviour stay
@answer n=11 entity=integration-test-strategy#4.2 v=yes
why: names the cfg(windows) case a_home_viola_makes_outside_the_profile_takes_the_protected_dacl (task item 8) and the three-OS strict-modes runs the chunk narrows; the Linux strict-modes refusals stay
@answer n=12 entity=integration-test-strategy#5.1 v=yes
why: names the per-OS [24/24] Unix and [21/21] Windows literals and cfg(unix) gates in tests/cli_verify.rs, and the all-3-OS plain-output and slash-argument tests the chunk re-points under task items 5 and 12
