@apply_row n=1 ref_file=fanout-test-plan--cascade-g4.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-test-plan--cascade-g4.md ref_n=2 v=applied edit=E2
@apply_row n=3 ref_file=fanout-test-plan--cascade-g4.md ref_n=4 v=applied edit=E3
@apply_row n=4 ref_file=fanout-test-plan--cascade-g4.md ref_n=6 v=applied edit=E4
@apply_row n=5 ref_file=fanout-test-plan--cascade-g4.md ref_n=7 v=applied edit=E5
@apply_row n=6 ref_file=fanout-test-plan--cascade-g4.md ref_n=8 v=applied edit=E6
@apply_row n=7 ref_file=fanout-test-plan--cascade-g4.md ref_n=9 v=applied edit=E7
@apply_row n=8 ref_file=fanout-test-plan--cascade-g4.md ref_n=10 v=applied edit=E8
@apply_row n=9 ref_file=fanout-test-plan--cascade-g4.md ref_n=11 v=applied edit=E9
@apply_row n=10 ref_file=fanout-test-plan--cascade-g4.md ref_n=12 v=applied edit=E10
@apply_row n=11 ref_file=fanout-test-plan--cascade-g4.md ref_n=14 v=applied edit=E11
@apply_row n=12 ref_file=fanout-test-plan--cascade-g4.md ref_n=15 v=applied edit=E12
@apply_row n=13 ref_file=fanout-test-plan--cascade-g4.md ref_n=16 v=applied edit=E13
@apply_row n=14 ref_file=fanout-test-plan--cascade-g5.md ref_n=1 v=applied edit=E14
@apply_row n=15 ref_file=fanout-test-plan--cascade-g5.md ref_n=2 v=applied edit=E15
@apply_row n=16 ref_file=fanout-test-plan--cascade-g5.md ref_n=3 v=refused
why: a hand removal · fct-7jgwgzx2
@apply_row n=17 ref_file=fanout-test-plan--cascade-g5.md ref_n=4 v=folded_into to_file=fanout-test-plan--cascade-g5.md to_n=2
@apply_row n=18 ref_file=fanout-test-plan--cascade-g5.md ref_n=5 v=applied edit=E16
@apply_row n=19 ref_file=fanout-test-plan--cascade-g5.md ref_n=6 v=applied edit=E17
@apply_row n=20 ref_file=fanout-test-plan--cascade-g5.md ref_n=7 v=folded_into to_file=fanout-test-plan--cascade-g5.md to_n=6
@apply_row n=21 ref_file=fanout-test-plan--cascade-g5.md ref_n=8 v=folded_into to_file=fanout-test-plan--cascade-g5.md to_n=6
@apply_row n=22 ref_file=fanout-test-plan--cascade-g5.md ref_n=9 v=refused
why: a hand removal · fct-eoindiy7
@apply_row n=23 ref_file=fanout-test-plan--cascade-g5.md ref_n=10 v=folded_into to_file=fanout-test-plan--cascade-g5.md to_n=6
@apply_row n=24 ref_file=fanout-test-plan--cascade-g5.md ref_n=11 v=applied edit=E18
@apply_row n=25 ref_file=fanout-test-plan--cascade-g5.md ref_n=12 v=refused
why: a hand removal · fct-7lzg7pgb
@apply_row n=26 ref_file=fanout-test-plan--cascade-g5.md ref_n=13 v=folded_into to_file=fanout-test-plan--cascade-g5.md to_n=11
@apply_row n=27 ref_file=fanout-test-plan--cascade-g5.md ref_n=14 v=folded_into to_file=fanout-test-plan--cascade-g5.md to_n=11
@apply_row n=28 ref_file=fanout-test-plan--cascade-g5.md ref_n=15 v=refused
why: a hand removal · fct-7lzg7pgb
@apply_row n=29 ref_file=fanout-test-plan--cascade-g5.md ref_n=16 v=folded_into to_file=fanout-test-plan--cascade-g5.md to_n=11
@apply_row n=30 ref_file=fanout-test-plan--cascade-g5.md ref_n=17 v=refused
why: a hand removal · fct-7lzg7pgb
@apply_row n=31 ref_file=fanout-test-plan--cascade-g5.md ref_n=18 v=applied edit=E19
@apply_row n=32 ref_file=fanout-test-plan--cascade-g5.md ref_n=19 v=applied edit=E20
@apply_row n=33 ref_file=fanout-test-plan--cascade-g5.md ref_n=20 v=applied edit=E21
@apply_row n=34 ref_file=fanout-test-plan--cascade-g5.md ref_n=21 v=refused
why: a hand removal · fct-6whu4pj5
@apply_row n=35 ref_file=fanout-test-plan--cascade-g5.md ref_n=22 v=refused
why: a hand removal · fct-6whu4pj5
@apply_row n=36 ref_file=fanout-test-plan--cascade-g5.md ref_n=23 v=applied edit=E22
@apply_row n=37 ref_file=fanout-test-plan--cascade-g5.md ref_n=24 v=applied edit=E23
@apply_row n=38 ref_file=fanout-test-plan--cascade-g5.md ref_n=25 v=applied edit=E24
@apply_row n=39 ref_file=fanout-test-plan--cascade-g5.md ref_n=26 v=folded_into to_file=fanout-test-plan--cascade-g5.md to_n=25
@apply_row n=40 ref_file=fanout-test-plan--cascade-g5.md ref_n=27 v=applied edit=E25
@fact id=fct-4m5t4osf expect_rev=2
old: (`cross_list_rows_style_each_span_beside_its_word_on_a_terminal`; on windows-2025 the parsed attribute set)
new: (`cross_list_rows_style_each_span_beside_its_word_on_a_terminal`)
old: under the outer PTY on `ubuntu-latest` and `macos-latest` (`cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal`, `tests/cross_list_rows.rs`, `cfg(unix)`; `list` row integrity)
new: under the outer PTY on `ubuntu-latest` (`cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal`, `tests/cross_list_rows.rs`, ungated; `list` row integrity)

@fact id=fct-tr6c4sml expect_rev=1
old: | direct (ConPTY / openpty) |
new: | direct (openpty) |
old: (the child's own bytes on Unix, viola's literals absent on every OS)
new: (the child's own bytes, and none of viola's literals)

@fact id=fct-4khqx2uf expect_rev=1
old: The `settings.json` half of step 5 is Unix's. On Unix the restart rewrites
new: The restart rewrites
old: `_ - . / :`. On Windows no `settings.json` is written. Landed
new: `_ - . / :`. Landed
old: (`tests/cli_instance_state.rs`, with a `cfg!(windows)` branch)
new: (`tests/cli_instance_state.rs`)
old: On Unix the start checks the socket directory
new: The start checks the socket directory

@fact id=fct-n6muav3b expect_rev=1
old: and tui (the receipt) on all three CI OSes;
new: and tui (the receipt) in CI on `ubuntu-latest`;

@fact id=fct-qbytqn26 expect_rev=1
old: two-CR tails; on all three CI OSes)
new: two-CR tails; on `ubuntu-latest`)
old: 0 fail`; its Windows reading is owed to "Windows-only live measurements")
new: 0 fail`)
old: multi-line text, an LF typed inside a paste on Windows, and what
new: multi-line text, and what

@fact id=fct-mdqnbyad expect_rev=1
old: covers cli and ipc-internal (the wrapper channel) on all three CI OSes,
new: covers cli and ipc-internal (the wrapper channel) in CI on `ubuntu-latest`,

@fact id=fct-lhsyucqg expect_rev=1
old: `run_child_ends_when_its_wrapper_is_terminated`, on every OS, terminates the wrapper (Windows `OpenProcess` + `TerminateProcess`; Unix `kill -KILL <wrapper pid>`, its exit asserted)
new: `run_child_ends_when_its_wrapper_is_terminated` terminates the wrapper (`kill -KILL <wrapper pid>`, its exit asserted)

@fact id=fct-p2i2onck expect_rev=1
old: covers cli and ipc-internal (the wrapper channel) on all three CI OSes:
new: covers cli and ipc-internal (the wrapper channel) in CI on `ubuntu-latest`:

@fact id=fct-3r2a46vd expect_rev=1
old: (the wrapper channel) on all three CI OSes; on `windows-2025` the focus case asserts the platform fact of the founder's live ruling F-W3 (an injected mouse report takes the wheel, focus reports are swallowed), Unix the full assertion.
new: (the wrapper channel) in CI on `ubuntu-latest`, the focus case holding the full assertion.

@fact id=fct-kn7ryhs7 expect_rev=1
old: empty stderr. The marker equality is Unix's: there `run` writes
new: empty stderr: `run` writes
old: `settings.json` override and the arm runs the recorded command as `/bin/sh -c <command>` under
new: `settings.json` override, and the arm runs the recorded command as `/bin/sh -c <command>` (`shell_argv` returns `[&str; 3]`) under
old: (PROVISIONAL). On Windows `run` writes no override and the arm runs no command (`override_document` and `shell_argv` return `None` for every input). Exit 0 with empty stderr holds on every OS. `budget.json`
new: (PROVISIONAL). `budget.json`

@fact id=fct-kylcwwrw expect_rev=1
old: `path6_a_restart_pauses_again`, green on the three CI runners.
new: `path6_a_restart_pauses_again`, green in CI.
old: (`tests/cli_budget.rs`), green on all three CI runners, `test (windows-2025)` included.
new: (`tests/cli_budget.rs`), green in CI.
old: with no ESC byte; on Unix the statusline arm prints the marker output and files the marker, and on Windows (`cfg(not(unix))`, run on `windows-2025` in CI) it prints nothing and files no marker.
new: with no ESC byte; the statusline arm prints the marker output and files the marker.
old: (green on the three CI runners, run 38098238731)
new: (green as measured at run 38098238731, a run of the retired three-OS matrix)

@fact id=fct-5eczp6gf expect_rev=1
old: (green on all three CI OSes, as measured at ci#40439188053)
new: (green as measured at ci#40439188053, a run of the retired three-OS matrix)

@fact id=fct-ja62ftpj expect_rev=1
old: On Unix a `claude_env_keep` name survives.
new: A `claude_env_keep` name survives.
old: On Unix, the child holds nothing of viola's
new: The child holds nothing of viola's
old: cannot go red either way. On Windows, handle non-inheritance is asserted at the product unit boundary.
new: cannot go red either way.

@fact id=fct-53es7vlo expect_rev=1
old: green on the three CI runners (run 38098238731)
new: green as measured at run 38098238731 (a run of the retired three-OS matrix)

@fact id=fct-7jgwgzx2 expect_rev=1
old: The impostor rows, each under `cfg_attr(not(target_os = "macos"), …)`, are
new: The impostor rows, each ungated, are
old: and, on Linux and Windows, a server that is not the recorded wrapper
new: and a server that is not the recorded wrapper
old: 0.414 s on the Windows debug build, as measured at chunk 2026-09-28-hook-perf-gate
new: 0.414 s on the debug build of the retired Windows host, as measured at chunk 2026-09-28-hook-perf-gate
old: (green on the three CI OSes, as measured at ci#40458923095)
new: (green as measured at ci#40458923095, a run of the retired three-OS matrix)

@fact id=fct-iplgfuqy expect_rev=1
old: files are 0600 on Unix
new: files are 0600

@fact id=fct-eoindiy7 expect_rev=1
old: stale heartbeat, the `.cmd` human pair, `verify`'s version
new: stale heartbeat, `verify`'s version
old: the home fails strict-modes (Unix a widened mode; Windows a Users grant through `icacls`, on windows-2025)
new: the home fails strict-modes (a widened mode)
old: (`tests/security_negatives_strict_modes.rs`, the three CI OSes)
new: (`tests/security_negatives_strict_modes.rs`)
old: a read-granted pre-existing home included on every OS
new: a read-granted pre-existing home included
old: server verification fails. On Linux and Windows: `snapshot.json`
new: server verification fails: `snapshot.json`
old: on the recorded endpoint), as a `#[cfg(not(target_os = "macos"))]` case. On Linux and macOS a server of another euid
new: on the recorded endpoint). A server of another euid
old: by no committed test; on macOS that is the word's whole meaning, the client reading no peer pid and comparing no server (the named carve-out, owned by no route entry). A loosened socket directory on macOS is the socket directory cause below, not this one. A same-user impostor inside a valid 0700 dir on macOS is an accepted upstream limitation, not asserted as a refusal.
new: by no committed test.
old: its Linux and Windows case is the eight-verb
new: its case is the eight-verb
old: after a passed preflight (`batch-script-child`, `pinned-hash-mismatch`
new: after a passed preflight (`pinned-hash-mismatch`
old: (`a_revive_whose_claude_is_a_script_is_the_batch_script_child_document`, `a_tampered_pinned_copy_is_pinned_hash_mismatch`
new: (`a_tampered_pinned_copy_is_pinned_hash_mismatch`
old: `stale-heartbeat`, `batch-script-child`, `pinned-hash-mismatch`
new: `stale-heartbeat`, `pinned-hash-mismatch`
old: `a_revive_whose_claude_is_a_script_is_the_batch_script_child_document` is `verification-matrix.json#v1-39`'s Windows witness, and `run` passes
new: `run` passes

@fact id=fct-7lzg7pgb expect_rev=1
old: with no line at MCP's `frame`; off Unix the check returns `Ok(())`, the DACL and the pid comparison staying the controls.
new: with no line at MCP's `frame`.
old: **Windows `--home` outside `%USERPROFILE%`** (`#[cfg(windows)]`, `~/.viola/` access control). `viola run` with a not-yet-existing `--home` under `RUNNER_TEMP` (outside `%USERPROFILE%`) creates it with a protected DACL whose ACEs are exactly the user and SYSTEM (read back through `icacls <home> /save`). As landed (chunk 2026-10-04-dialog-answers-by-dialog-id, the creation half pulled forward from the route entry "Home and code-bearing file integrity" by the founder's live ruling): `viola_state::fs::create_private_dir` gives the topmost folder it creates outside the profile the protected `D:P(A;OICI;FA;;;<user>)(A;OICI;FA;;;SY)` DACL and fails the creation when it cannot be set, covered by unit tests on both OSes (Windows on CI) and the integration `tests/cli_version_gate.rs`; the negative case is `tests/security_negatives_strict_modes.rs` `a_home_viola_makes_outside_the_profile_takes_the_protected_dacl` (windows-2025), whose read-back reads `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)`, canonical SDDL spelling the runner's user (local Administrator, RID 500) `LA`, as measured at ci#40400887151. A Windows DACL test creates its home where the CI homes live (`target/e2e-home`), never under `%TEMP%`, which lies inside the profile. A pre-existing directory there that inherits a broader ACL (for example `BUILTIN\Users` read) is refused
new: A pre-existing home another user may read is refused
old: (`a_home_another_user_may_read_refuses_run`, on every OS: a Unix 0750 home, a Windows Users read grant)
new: (`a_home_another_user_may_read_refuses_run`, a 0750 home)
old: `only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps`, on the three CI OSes: both files
new: `only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps`: both files
old: (`tests/security_negatives_strict_modes.rs`, the three CI OSes). `frame_verb_refuses
new: (`tests/security_negatives_strict_modes.rs`). `frame_verb_refuses
old: the mode or ACL read back as found
new: the mode read back as found
old: the umask-independent modes, single-writer and Windows `--home` outside the profile cases above
new: the umask-independent modes and single-writer cases above
old: **Server identity** (`#![cfg(not(target_os = "macos"))]`, IPC client-side server verification)
new: **Server identity** (IPC client-side server verification)
@fact id=fct-hdfefjry expect_rev=1
old: `cross_list_rows_print_escaped_in_the_captions_columns_in_every_mode`; on Windows the terminal case reads each escaped name as text under a 200-column terminal (ConPTY re-renders the stream with escapes of its own) and the no-ESC assertion is Unix-only; the binary
new: `cross_list_rows_print_escaped_in_the_captions_columns_in_every_mode`; the binary

@fact id=fct-3fk5wzsy expect_rev=1
old: (`cfg(unix)`, on `booted_wrapper`)
new: (on `booted_wrapper`, ungated)
old: green on `ubuntu-latest` and `macos-latest` (as measured at ci#41918694243)
new: green on `ubuntu-latest` (as measured at ci#41918694243)

@fact id=fct-6whu4pj5 expect_rev=1
old: Whether the output outlives the session leader is OS-specific, as measured in CI runs 36165685381 / 36166907442: natively on ConPTY; on Linux only while
new: The output outlives the session leader only while
old: when the leader exits); on macOS the output ends at the leader's exit even with the holder alive in its own group. The held-output half is therefore asserted on Windows + Linux only, and on macOS exit on the handle is shown by the handle wait alone (`tests/tui_pty_seam.rs`) and a `session-end` record is logged
new: when the leader exits), as measured in CI runs 36165685381 / 36166907442, and the held-output half is asserted
old: (`e4 b8 ad`, Unix)
new: (`e4 b8 ad`)
old: `unable`, `hint:` (every OS)
new: `unable`, `hint:`
@fact id=fct-nwgyqyjs expect_rev=1
old: both streams; it does not run on `macos-latest`, whose `ps` has no keyword that prints the ignored-signal mask (as measured at ci#41915621814), a gate set by the overseer's technical answer (founder-delegated) to the implement dialog, so the fake agent's SIGHUP-ignoring exit, shared Unix code, is witnessed on Linux only (§7 Fake agent).
new: both streams; it runs on CI's one leg, `ubuntu-latest` (§7 Fake agent).

@fact id=fct-sn6e6lxw expect_rev=1
old: literal oracles, on all three OSes.
new: literal oracles.

@fact id=fct-liulfbun expect_rev=1
old: `faults` reads both ways, an unnamed gate and a row that no longer stands.
new: `faults` reads three ways: an unnamed gate, a row that no longer stands, and a gate whose predicate names `windows` or `target_os = "macos"` (`another OS: …`) even when a row names it, its parse catching a multi-line `cfg_attr` OS predicate that a line grep cannot see.
old: `gates_table_rows_are_unique` and the self-check `census_finds_an_unnamed_gate_and_passes_a_named_one` over eight planted cases.
new: `gates_table_rows_are_unique`, the self-check `census_finds_an_unnamed_gate_and_passes_a_named_one` over eight planted cases (its Windows cases building the word from a `WINDOWS` const) and the self-check `census_reads_a_gate_naming_another_os_as_a_fault_even_when_named`.

@fact id=fct-vgds3skp expect_rev=1
old: 52 OS-gated test functions and 7 gated files, 59 `GATES` rows (rule: the rows of `GATES` in `tests/contract_os_gated_tests.rs`), its rule taking the `cfg_attr` case rows (`tests/hook_fail_open.rs`'s two functions).
new: 39 OS-gated test functions and 2 gated files, 41 `GATES` rows (rule: the rows of `GATES` in `tests/contract_os_gated_tests.rs`; as measured at chunk 2026-11-05-linux-only-code-and-ci).
@pattern id=tp4-01
fixed: on windows-2025 the parsed attribute set
@pattern id=tp4-02
fixed: direct (ConPTY / openpty)
@pattern id=tp4-03
fixed: On Windows no `settings.json` is written
@pattern id=tp4-04 ignore_case=true
fixed: on all three CI OSes
@pattern id=tp4-05
fixed: its Windows reading is owed to
@pattern id=tp4-06
fixed: an LF typed inside a paste on Windows
@pattern id=tp4-07
fixed: Windows `OpenProcess` + `TerminateProcess`
@pattern id=tp4-08
fixed: the focus case asserts the platform fact
@pattern id=tp4-09
fixed: return `None` for every input
@pattern id=tp4-10 ignore_case=true
regex: green on (all )?(the )?three CI (runners|OSes)
@pattern id=tp4-11
fixed: handle non-inheritance is asserted at the product unit boundary
@pattern id=tp4-12
fixed: on Linux and Windows, a server that is not the recorded wrapper
@pattern id=tp4-13
fixed: 0.414 s on the Windows debug build
@pattern id=tp4-14
fixed: files are 0600 on Unix
@pattern id=tp4-15
fixed: the `.cmd` human pair
@pattern id=tp4-16
fixed: a Users grant through `icacls`
@pattern id=tp4-17
fixed: the client reading no peer pid and comparing no server
@pattern id=tp4-18
fixed: `batch-script-child`, `pinned-hash-mismatch`
@pattern id=tp4-19
fixed: off Unix the check returns `Ok(())`
@pattern id=tp4-20
fixed: creates it with a protected DACL
@pattern id=tp4-21
fixed: the mode or ACL read back as found
@pattern id=tp4-22
fixed: ConPTY re-renders the stream with escapes of its own
@pattern id=tp4-23
fixed: green on `ubuntu-latest` and `macos-latest`
@pattern id=tp4-24
fixed: natively on ConPTY
@pattern id=tp4-25
fixed: it does not run on `macos-latest`
@pattern id=tp4-26
fixed: literal oracles, on all three OSes
@pattern id=tp4-27
fixed: `faults` reads both ways
@pattern id=tp4-28
fixed: 52 OS-gated test functions and 7 gated files
