@carried n=1 label=test-plan/e2e-test-strategy fact=fct-blbpbqaj
why: E2's Unix-only reading of the child's live fd table must keep its Linux behaviour whether its cfg(unix) gate stays or is made unconditional with its sibling gone.
quote: On Unix the test reads the running child's live fd table (`/proc/<pid>/fd`), never the fake agent's self-dump, whose own receipt and `/dev/fd` handle show as extra fds.
@carried n=1 run=2 label=test-plan/e2e-test-strategy fact=fct-ja62ftpj
quote: On Unix a `claude_env_keep` name survives.
@carried n=1 run=3 label=test-plan/e2e-test-strategy fact=fct-ja62ftpj
quote: On Windows, handle non-inheritance is asserted at the product unit boundary.
@carried n=1 run=4 label=test-plan fact=fct-tenptmlu
quote: on Unix a `claude_env_keep` name survives and the child holds no viola socket or home path
@carried n=2 label=test-plan/e2e-test-strategy fact=fct-7jgwgzx2
why: the hook fail-open matrix must keep every Linux row while its macOS-gated impostor rows lose their cfg_attr gate and its Windows server leg leaves.
quote: and, on Linux and Windows, a server that is not the recorded wrapper (`server-verify-failed`); on Unix, beyond this matrix, a socket directory the check refuses (`socket-dir-refused`, `security_negative_a_hook_fails_open_on_the_socket_directory` in `tests/security_negatives_socket_dir.rs`) and a server of another euid (`server-verify-failed`, its wiring with no committed witness) each exit 0 with empty streams and no frame.
@carried n=2 run=2 label=test-plan/e2e-test-strategy fact=fct-7jgwgzx2
quote: The impostor rows, each under `cfg_attr(not(target_os = "macos"), …)`, are `case::impostor` in `hook_fails_open_silently_within_the_spine_bound` and `question_impostor` and `permission_impostor` in `hook_dialog_tier_fails_open_silently`
@carried n=2 run=3 label=test-plan/e2e-test-strategy fact=fct-7jgwgzx2
quote: Not a refusal case: a hook reaching an exiting wrapper's own Windows pipe (ci#36532038635) passes the server check by design, the exiting wrapper being the recorded process; whether sysinfo still lists a terminated Windows process whose pipe handle is open, and SessionEnd's outcome in that window, are one CARRY on the route entry "Windows-only live measurements".
@carried n=2 run=4 label=test-plan fact=fct-4aawmv2h
quote: and, on Linux and Windows, a server that is not the recorded wrapper and, on Unix, a socket directory the check refuses or a server of another euid (its decision held by injected euids, §1 Untestable zones), each exit 0 with no body
@carried n=3 label=test-plan/e2e-test-strategy fact=fct-iplgfuqy
why: the diagnostics 0600 clause is a Unix arm the chunk makes unconditional with its Linux mode unchanged, and the per-OS harness upload name shrinks with the matrix.
quote: `diagnostics/` files are 0600 on Unix
@carried n=3 run=2 label=test-plan/e2e-test-strategy fact=fct-iplgfuqy
quote: excluded from the scan and from the `harness-<os>` upload)
@carried n=3 run=3 label=test-plan fact=fct-gjahavf7
quote: `diagnostics/` files 0600 on Unix
@carried n=4 label=test-plan/e2e-test-strategy fact=fct-eoindiy7
why: the exit-cause matrix must keep its six closed exit-21 causes on Linux while its Windows icacls cause, its macOS-gated server case, the macOS carve-out and the cfg(windows) .cmd / .bat cause leave or are decided by P4.
quote: exit 21's six closed `detail` causes are `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed` and `socket-dir-refused`
@carried n=4 run=2 label=test-plan/e2e-test-strategy fact=fct-eoindiy7
quote: the home fails strict-modes (Unix a widened mode; Windows a Users grant through `icacls`, on windows-2025)
@carried n=4 run=3 label=test-plan/e2e-test-strategy fact=fct-eoindiy7
quote: server verification fails. On Linux and Windows: `snapshot.json` pid or start time does not match the listening process (a same-user impostor listener on the recorded endpoint), as a `#[cfg(not(target_os = "macos"))]` case. On Linux and macOS a server of another euid reads `server-verify-failed` too, through the client euid check, its decision held by injected values and each seam's wiring by no committed test; on macOS that is the word's whole meaning, the client reading no peer pid and comparing no server (the named carve-out, owned by no route entry).
@carried n=4 run=4 label=test-plan/e2e-test-strategy fact=fct-eoindiy7
quote: the PTY child is `.cmd` / `.bat` (`#[cfg(windows)]`)
@carried n=4 run=5 label=test-plan/e2e-test-strategy fact=fct-eoindiy7
quote: `a_revive_whose_claude_is_a_script_is_the_batch_script_child_document` is `verification-matrix.json#v1-39`'s Windows witness
@carried n=4 run=6 label=test-plan/e2e-test-strategy fact=fct-eoindiy7
quote: `link_verbs_refuse_a_widened_instance_before_any_log` (`cfg(unix)`)
@carried n=4 run=7 label=test-plan fact=fct-ndx7lm2p
quote: beside the table, the server-verification cause is a case of `tests/security_negatives_server_identity.rs` on Linux and Windows (macOS compares no peer pid, the named carve-out `server_verdict_rows` holds, so these negatives compile out there; a loosened socket directory on Linux and macOS is a case of `tests/security_negatives_socket_dir.rs`)
@carried n=4 run=8 label=test-plan fact=fct-x7oaiapn
quote: macOS local runs, which are CI-only.
@carried n=5 label=test-plan/e2e-test-strategy fact=fct-7lzg7pgb
why: the Windows-only security negatives leave with their cfg(windows) gates and tests/conpty_sideload.rs, while the Unix socket directory, umask and server identity negatives must keep their Linux assertions as their gates are retired or kept.
quote: **Unix socket directory refusal** (`#![cfg(unix)]`, IPC access control).
@carried n=5 run=2 label=test-plan/e2e-test-strategy fact=fct-7lzg7pgb
quote: off Unix the check returns `Ok(())`, the DACL and the pid comparison staying the controls.
@carried n=5 run=3 label=test-plan/e2e-test-strategy fact=fct-7lzg7pgb
quote: **Windows client SQOS** (`#[cfg(windows)]`, client-side server verification).
@carried n=5 run=4 label=test-plan/e2e-test-strategy fact=fct-7lzg7pgb
quote: **Windows `--home` outside `%USERPROFILE%`** (`#[cfg(windows)]`, `~/.viola/` access control).
@carried n=5 run=5 label=test-plan/e2e-test-strategy fact=fct-7lzg7pgb
quote: (`a_home_another_user_may_read_refuses_run`, on every OS: a Unix 0750 home, a Windows Users read grant).
@carried n=5 run=6 label=test-plan/e2e-test-strategy fact=fct-7lzg7pgb
quote: **Umask-independent modes** (`#[cfg(unix)]`).
@carried n=5 run=7 label=test-plan/e2e-test-strategy fact=fct-7lzg7pgb
quote: The case is `tests/security_negatives_strict_modes.rs` `only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps`, on the three CI OSes
@carried n=5 run=8 label=test-plan/e2e-test-strategy fact=fct-7lzg7pgb
quote: **Windows listener remote-client rejection** (`#[cfg(windows)]`, IPC access control (Windows)).
@carried n=5 run=9 label=test-plan/e2e-test-strategy fact=fct-7lzg7pgb
quote: **Windows listener DACL denial** (`#[cfg(windows)]`, IPC access control (Windows)).
@carried n=5 run=10 label=test-plan/e2e-test-strategy fact=fct-7lzg7pgb
quote: **Server identity** (`#![cfg(not(target_os = "macos"))]`, IPC client-side server verification).
@carried n=5 run=11 label=test-plan/e2e-test-strategy fact=fct-7lzg7pgb
quote: **ConPTY companion owner/DACL** (`#[cfg(windows)]` x64, windows-2025). `tests/conpty_sideload.rs` `run_on_a_companion_another_user_may_write_falls_back_to_conhost`
@carried n=5 run=12 label=test-plan fact=fct-4uc5ehox
quote: Windows client SQOS at the identification level, the Windows listener rejecting remote clients and denying an anonymous open (each beside its controls)
@carried n=5 run=13 label=test-plan fact=fct-dcxdz6xw
quote: Non-persistent-ACL volumes and foreign-owner SIDs. Covered by synthetic SDDL descriptor stubs (§4).
@carried n=5 run=14 label=test-plan fact=fct-lkgqv65p
quote: Cross-OS-user peers. Covered by the peer-credential boundary unit test (§4) and same-user SDDL read-back (§5); the euid wiring at the three client seams is held by no committed test (§1 Untestable zones).
@carried n=6 label=test-plan/e2e-test-strategy fact=fct-hdfefjry
why: the list row integrity case's Windows terminal arm leaves and its Unix-only no-ESC assertion must keep holding on Linux once its gate goes.
quote: on Windows the terminal case reads each escaped name as text under a 200-column terminal (ConPTY re-renders the stream with escapes of its own) and the no-ESC assertion is Unix-only
@carried n=7 label=test-plan/e2e-test-strategy fact=fct-3fk5wzsy
why: the stale-row dim witness is a cfg(unix) census row whose macos-latest leg leaves, its Linux escape-sequence assertion unchanged.
quote: `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` (`cfg(unix)`, on `booted_wrapper`), `v1-40`'s integration-tier witness, freezes the wrapper through `Frozen`
@carried n=7 run=2 label=test-plan/e2e-test-strategy fact=fct-3fk5wzsy
quote: green on `ubuntu-latest` and `macos-latest` (as measured at ci#41918694243).
@carried n=8 label=test-plan/e2e-test-strategy fact=fct-6whu4pj5
why: the chaos suite's Windows and macOS legs leave with their arms while the Linux held-output, SIGSTOP and byte-oracle legs stay, and its Windows byte reading is owed to the retired route entry the masters re-point.
quote: Whether the output outlives the session leader is OS-specific, as measured in CI runs 36165685381 / 36166907442: natively on ConPTY; on Linux only while the `--exit-no-eof` holder runs in its own process group (a holder in the leader's foreground group dies by SIGHUP when the leader exits); on macOS the output ends at the leader's exit even with the holder alive in its own group. The held-output half is therefore asserted on Windows + Linux only, and on macOS exit on the handle is shown by the handle wait alone (`tests/tui_pty_seam.rs`) and a `session-end` record is logged
@carried n=8 run=2 label=test-plan/e2e-test-strategy fact=fct-6whu4pj5
quote: SIGSTOP the wrapper (Unix) for over 5 s: `liveness:"stale"`, and a new `run` exits 1 with `stale-heartbeat` (`run_refuses_a_stale_name`, on the `Frozen` guard)
@carried n=8 run=3 label=test-plan/e2e-test-strategy fact=fct-6whu4pj5
quote: the outer terminal shows the child's own panicking bytes (`e4 b8 ad`, Unix) and none of `panic`, `parse-rejected`, `vt100`, `unable`, `hint:` (every OS)
@carried n=8 run=4 label=test-plan/e2e-test-strategy fact=fct-6whu4pj5
quote: On Windows the case asserts the literals' absence only: ConPTY re-renders the child's output, and a wide character on a 1-column screen has no guaranteed rendering, so the byte oracle for the panicking wide character is owed to the route entry "Windows-only live measurements".
@carried n=8 run=5 label=test-plan/e2e-test-strategy fact=fct-6whu4pj5
quote: `.cmd` child (`#[cfg(windows)]`): `run` exits 1 — landed in `tests/cli_program_resolution.rs`
@carried n=8 run=6 label=test-plan fact=fct-47fi4ugg
quote: and a `.cmd` child refused exit 1.
@carried n=8 run=7 label=test-plan fact=fct-ktlsbc3h
quote: (the child's own bytes on Unix, no viola literal on every OS)
@carried n=9 label=test-plan/e2e-test-strategy fact=fct-nwgyqyjs
why: the hangup case's Linux gate exists only to keep it off macos-latest and is a census row, and the SIGHUP-ignoring exit it witnesses is a Unix arm the chunk makes unconditional with its Linux behaviour unchanged.
quote: (`tests/cli_fake_agent.rs` `fake_agent_exit_outlives_a_hangup_that_lands_while_it_exits`, `cfg(target_os = "linux")`)
@carried n=9 run=2 label=test-plan/e2e-test-strategy fact=fct-nwgyqyjs
quote: it does not run on `macos-latest`, whose `ps` has no keyword that prints the ignored-signal mask (as measured at ci#41915621814), a gate set by the overseer's technical answer (founder-delegated) to the implement dialog, so the fake agent's SIGHUP-ignoring exit, shared Unix code, is witnessed on Linux only (§7 Fake agent).
@carried n=10 label=test-plan/e2e-test-strategy fact=fct-sn6e6lxw
why: the contract suite reads the deny.toml the chunk trims and its bounded-reader and error-body tables hold rows to files that must still stand once the Windows-only product files leave.
quote: `boundaries_each_name_a_bounded_read_that_stands` holds each row's read to its file and its witness to a defined test, and each exception to a read that stands; its other tests are `product_readers_are_each_bounded_or_named`, `census_finds_an_unbounded_read_and_passes_a_bounded_one` and `serde_json_unbounded_depth_is_banned_in_deny_toml` (the `deny.toml` feature ban read).
@carried n=10 run=2 label=test-plan/e2e-test-strategy fact=fct-sn6e6lxw
quote: `tests/contract_error_bodies.rs` (a `contract_` binary, `run --e2e`) is the error-body enumeration: the tables `CLI_DOCUMENTS`, `MCP_RESULTS`, `MCP_FIELD_WORDS`, `CHANNEL_FAULTS`, `CHANNEL_METHODS`, `NOT_ERROR_BODIES` and `PLUMBING` over `PRODUCT_ROOTS`
@carried n=10 run=3 label=test-plan/e2e-test-strategy fact=fct-sn6e6lxw
quote: `sha256_published_vector_matches_digest` over the three FIPS 180-2 vectors and `sha256_truncated_to_eight_bytes_yields_sixteen_hex` over the 8-byte / 16-hex truncation, literal oracles, on all three OSes.
@carried n=10 run=4 label=test-plan fact=fct-q6ltykaz
quote: beside `serde_json_unbounded_depth_is_banned_in_deny_toml`.
@carried n=11 label=test-plan/e2e-test-strategy fact=fct-liulfbun
why: the census is the reached file the chunk re-points or retires with its gates, and its faults read both an unnamed gate and a row that no longer stands.
quote: its `GATES` table holds a `file`, `test`, `gate` and `reason` per row, and the census walks every `.rs` directly under `tests/` and in each `crates/*/tests/`, counting a leading `#![cfg(…)]` and a test function whose contiguous attribute run holds `#[test]`, `#[rstest]` or `#[tokio::test]` and a `#[cfg(…)]` or `#[cfg_attr(…)]` whose predicate names `windows`, `unix` or `target_os` outside its string values; `faults` reads both ways, an unnamed gate and a row that no longer stands. Its tests are `os_gated_tests_are_each_named_with_their_reason`, `gates_table_rows_are_unique` and the self-check `census_finds_an_unnamed_gate_and_passes_a_named_one` over eight planted cases.
@carried n=11 run=2 label=test-plan/e2e-test-strategy fact=fct-vgds3skp
quote: the census reads 52 OS-gated test functions and 7 gated files, 59 `GATES` rows (rule: the rows of `GATES` in `tests/contract_os_gated_tests.rs`), its rule taking the `cfg_attr` case rows (`tests/hook_fail_open.rs`'s two functions).
@carried n=11 run=3 label=test-plan fact=fct-lktzdsoq
quote: NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
@binding n=1
**census of OS-gated tests ↔ every gated suite**: each gate the chunk removes or keeps — the hook fail-open cfg_attr impostor rows, the server identity binary's cfg(not macos) gate, the socket directory and umask cfg(unix) gates, the Windows-only security negatives and tests/conpty_sideload.rs, the .cmd child, the list row arms, the stale-row dim witness and the hangup case's Linux gate — must move the census's GATES rows and count with it, or the census is retired with them; no row may name a gate or file that no longer stands (test-plan/e2e-test-strategy fct-liulfbun) (test-plan/e2e-test-strategy fct-vgds3skp) (test-plan/e2e-test-strategy fct-7jgwgzx2) (test-plan/e2e-test-strategy fct-7lzg7pgb) (test-plan/e2e-test-strategy fct-nwgyqyjs)
@binding n=2
**contract suite ↔ supply chain (deny.toml)**: the trimmed deny.toml must still hold the serde_json depth ban the contract test reads, and the bounded-reader and error-body tables must drop or re-point any row naming a read or writer in the removed Windows-only files so their stands checks stay green on Linux (test-plan/e2e-test-strategy fct-sn6e6lxw) (test-plan fct-q6ltykaz) (test-plan fct-ys3szg34)
@binding n=3
**made-unconditional Unix arms ↔ Linux behaviour unchanged**: E2's claude_env_keep survival and fd-table read, diagnostics 0600, Path 1's Unix settings.json override, Path 6's Unix statusline pass-through and the fake agent's SIGHUP-ignoring exit must each keep their Linux assertions byte for byte when their gates go (test-plan/e2e-test-strategy fct-ja62ftpj) (test-plan/e2e-test-strategy fct-iplgfuqy) (test-plan fct-mwv2sqc7) (test-plan fct-jtonrpse) (test-plan/e2e-test-strategy fct-nwgyqyjs)
@binding n=4
**route entry «Windows-only live measurements» ↔ expected test-plan amendments**: the security sweep's exiting-Windows-pipe CARRY and the chaos vt100 Windows byte oracle are owned by the retired route entry, and the plan must name their re-pointing as amendments the wrap writes (test-plan/e2e-test-strategy fct-7jgwgzx2) (test-plan/e2e-test-strategy fct-6whu4pj5) (test-plan fct-phy2lvsc)
@binding n=5
**exit-cause matrix ↔ boundaries (no Linux behaviour moves)**: the six closed exit-21 causes and every Linux row stay; the cfg(windows) .cmd / .bat cause, its batch-script-child word and v1-39's Windows witness are a fork P3 lists and P4 decides against the masters' OS-target change (test-plan/e2e-test-strategy fct-eoindiy7) (test-plan/e2e-test-strategy fct-6whu4pj5)
@binding n=6
**per-OS legs ↔ CI matrix shrink**: the suites' «three CI OSes», macos-latest and windows-2025 legs shrink to ubuntu-latest with the matrix, worded by the wrap's test-plan amendment that this plan names as expected (test-plan/e2e-test-strategy fct-7lzg7pgb) (test-plan/e2e-test-strategy fct-sn6e6lxw) (test-plan/e2e-test-strategy fct-3fk5wzsy) (test-plan fct-shyrfs6v)
