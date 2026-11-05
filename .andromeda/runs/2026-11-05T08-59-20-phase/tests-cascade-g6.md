@carried n=1 label=test-plan/test-data-fixtures fact=fct-bxlupfu3
why: The Windows x64 `seed_conpty` exception leaves with the ConPTY sideload, so on-disk product state must have only the chaos, impostor and MCP-handler exceptions, and the Unix read-rule refusal of a `create_dir_all` home stays.
quote: and on Windows x64 the root tests' `seed_conpty`, which hard-links (or copies) the ConPTY companions into `<home>/bin/<key>/conpty/`, byte-identical to what viola would write (§5 carve-out; `conpty_sideload` never seeds, so it covers the product's write).
@carried n=1 run=2 label=test-plan fact=fct-54gkcbby
quote: except the chaos tests' corruption of files the product already wrote, the Windows x64 `seed_conpty` companions and the server-identity impostors' test-built `snapshot.json`
@carried n=2 label=test-plan/test-data-fixtures fact=fct-oanlcr47
why: The widened-seeded-home carve-out exists only for Windows x64 seeding and must leave with it, while the Linux case `run_refuses_a_widened_home_tree_before_its_log` keeps its refusal.
quote: so a test that widens a home tree above `bin/<key>/conpty/` starts that home unseeded (`run_refuses_a_widened_home_tree_before_its_log`).
@carried n=3 label=test-plan/test-data-fixtures fact=fct-652w7iir
why: The `USERPROFILE` arm of `Wrapper::boot_as_user` is a removed Windows arm, and the default-home cases must keep setting `HOME` on that one child.
quote: A default-home case takes `TestHome::default_of_user` and `Wrapper::boot_as_user`, which sets `HOME` (on Windows `USERPROFILE`) on that one child.
@carried n=4 label=test-plan/test-data-fixtures fact=fct-jfjmhkwu
why: The macOS `TMPDIR` arm of `SOCKET_ROOT_VAR` is a `target_os = "macos"` arm the chunk removes, and the test-owned socket root must read `XDG_RUNTIME_DIR` alone.
quote: `SOCKET_ROOT_VAR` (`TMPDIR` on macOS, else `XDG_RUNTIME_DIR`), `SocketRoot`, `Wrapper::boot_on_socket_root` and `first_life_on`
@carried n=5 label=test-plan/test-data-fixtures fact=fct-t5v77dkt
why: The `icacls` arm of the widened-home helpers is the Windows set and read-back the chunk removes, and the helpers must keep widening by real Unix modes on the real file.
quote: Set by the test on the real file, never by viola: `tests/support/home.rs` `widen` · `grant_read` · `unwiden` · `still_widened` (Unix modes; on Windows `icacls`, its exit asserted).
@carried n=6 label=test-plan/test-data-fixtures fact=fct-2mxqmdgt
why: When `tests/security_negatives_socket_dir.rs` loses its macOS leg, its `Form` fixture must still loosen the real directory and read it back as found, and the foreign owner must stay an injected uid.
quote: Set by the test on the real directory, never by viola, whose check never chmods it: `tests/security_negatives_socket_dir.rs` `Form` (`apply` · `assert_as_found` · `restore`), a directory at 0755 or a symlink to 0700, the refused directory asserted as found. A foreign owner is not driven: `socket_dir_verdict` takes it by an injected uid.
@carried n=7 label=test-plan/test-data-fixtures fact=fct-6wjlyq3w
why: Help strings and hints that name removed Windows code are a fork the scope leaves to P4, and any one that changes must move its literal in its test file in the same commit.
quote: `viola --help` (the `HELP` literal, `tests/cli_output_plain.rs`), the `viola list` board (`tests/cli_list.rs`) and the three `viola link` / `viola unlink` result lines (`src/human/verbs.rs` `link_verb_lines_are_the_design_lines_in_one_write` and `link_since_is_the_utc_time_alone`; `tests/cli_links.rs` `link_verbs_print_the_design_lines`, the printed time checked by shape), and the exit-cause table's codes, exits and last `hint:` lines (`tests/cross_exit_causes.rs` `CAUSES`), and the socket-directory refusal's last `hint:` line (`tests/cross_exit_causes.rs` `SOCKET_DIR`, `tests/security_negatives_socket_dir.rs` `HINT`), are literals in their test files
@carried n=8 label=test-plan/test-data-fixtures fact=fct-rgx56qfp
why: The impostor cases lose their Windows arm, and on Linux they must keep writing their own `InstanceSnapshot` that records a held live pid beside an endpoint bound in a private directory.
quote: The server-identity impostor cases (Linux and Windows: `served` in `tests/security_negatives_server_identity.rs`, `Impostor::serve` in `tests/hook_fail_open.rs`) write an `InstanceSnapshot` themselves, recording a held live process's pid (the fake agent's holder, below) beside an endpoint the test binds in a private directory
@carried n=9 label=test-plan/test-data-fixtures fact=fct-wmieczsn
why: The stated reason for `record_live` names a Windows `Serving` that the removal deletes, so P3 must list that text and P4 must decide it, and the unserved row must keep its Linux behaviour.
quote: `crates/viola-mcp/tests/handlers.rs` `record_live(instance, endpoint, pid)` writes an `InstanceSnapshot` recording an endpoint no one ever bound, for the unserved row of `handlers_frame_tools_name_the_cause_of_an_unreachable_instance`: a Windows `Serving` has no `Drop` (its pipe lives in the accept thread until the process exits), so the row binds nothing.
@carried n=10 label=test-plan/test-data-fixtures fact=fct-ubb4ig4c
why: The Windows `NotFound` arm of this per-OS oracle leaves, and `verify_under_a_regular_file_refuses_the_home` must keep its Unix strict-modes refusal unconditionally.
quote: A home planted under a regular file takes a per-OS oracle: on Unix it is a strict-modes refusal, while on Windows such a path stats `NotFound`, the check skips it as absent and the catch-site line stands (`verify_under_a_regular_file_refuses_the_home`).
@carried n=11 label=test-plan/test-data-fixtures fact=fct-fvd26vjh
why: Once their Unix arm loses its gate, the endpoint directories of the edited impostor, fail-open and handler tests must stay `create_private_dir`, never a bare temp dir.
quote: A directory holding a test's endpoint, where the case needs it to pass the socket-directory check, is made through `viola_state::fs::create_private_dir`, never left a bare `tempfile::tempdir()`
@carried n=12 label=test-plan/test-data-fixtures fact=fct-vrevms4b
why: The holder's Unix process group and the fake agent's per-command process group are cfg(unix) arms the task lists to be made unconditional, and each must keep its Linux behaviour.
quote: `--exit-no-eof` (a bounded re-exec holder keeps the child's console / PTY slave open for 10 s — not viola's stdout — writes a `hold` receipt, and on Unix spawns in its own process group)
@carried n=12 run=2 label=test-plan fact=fct-7o4nbra2
quote: on Unix every command it spawns (each hook, and the status line command) runs in a process group of its own (`process_group(0)`, as the `--exit-no-eof` holder does)
@carried n=13 label=test-plan/mocking-stubbing-discipline fact=fct-sik7tvk4
why: The held live process behind the impostor cases the chunk edits must stay a real process, killed and reaped by its `Drop` guard, never a double.
quote: a held live process whose pid an impostor's snapshot records is a real process, never a double: in root tests the fake agent's `--hold-stdout-internal` holder, in `crates/viola-mcp/tests/handlers.rs` its own `Held` child, entered at `handlers_held_child_entry` only when its `HELD` variable is set, each killed and reaped by a `Drop` guard
@carried n=14 label=test-plan/mocking-stubbing-discipline fact=fct-63urz52n
why: The `Frozen` guard's Unix `SIGSTOP` arm loses its Windows sibling, and `run_refuses_a_stale_name` must keep standing on a real stopped wrapper.
quote: A stale holder is a real wrapper under the `Frozen` guard (`tests/support/frozen.rs`, its sole home, shared through `tests/support/mod.rs`; on Unix a `SIGSTOP`), which `run_refuses_a_stale_name` stands on
@carried n=15 label=test-plan/mocking-stubbing-discipline fact=fct-pwscsozw
why: The Windows control pipes and the anonymous-token stand-in must leave with the listener-admission negatives, and the Linux impostor of the server-verification negatives must stay.
quote: in the listener-admission negatives the test-owned control pipes (accept-remote, reject-remote, the default descriptor) stand beside the real `viola_channel::Server::bind` listener, and `ImpersonateAnonymousToken` stands in for a foreign principal; in the server-verification negatives (Linux and Windows) an impostor, a server on the recorded endpoint while the snapshot records another held live process (Process mocking), stands in for a server that is not the recorded wrapper
@carried n=16 label=test-plan/mocking-stubbing-discipline fact=fct-zsbxg2jw
why: The windows-2025 grant cases, the synthetic Windows descriptors and the unseeded carve-out must leave, and the remaining filesystem faults must stay real Unix modes on a real tempfile home.
quote: on windows-2025 an instance tree granted Users write, or a `diagnostics/` granted Users read, is refused beside a `create_private_dir` tree that passes; the pure verdicts (`windows_read_verdict` among them) take synthetic descriptors at the unit tier; a test that widens a home tree above `bin/<key>/conpty/` starts its home unseeded (§7, a widened seeded home)
@carried n=16 run=2 label=test-plan fact=fct-hkmaq6ye
quote: Filesystem: not mocked. A real tempfile home is used everywhere, because modes, `File::lock`, torn lines and symlink handling are the behaviour under test. Only the Windows foreign-SID / non-persistent-ACL descriptors are stubbed, as synthetic SDDL descriptors fed to the check function.
@carried n=17 label=test-plan/mocking-stubbing-discipline fact=fct-6xaoh2kl
why: The scope bars any env, flag or feature seam that forces a refusing peer-euid decision, so the peer euid decision must stay injected only as an argument.
quote: `peer_euid_admitted(observed, own, reported)` takes its euids as arguments and the private `serve_admitting` takes its decision as an `Admission` argument (`crates/viola-channel/src/server.rs`): dependency injection, with no env, flag or feature seam
@carried n=17 run=2 label=test-plan fact=fct-3a3m4jll
quote: **Anti-monkey-patching:** prefer dependency injection over runtime patching of production code.
@binding n=1
**test-data-fixtures ↔ mocking-stubbing-discipline (the ConPTY seed)**: the `seed_conpty` exception and the widened-seeded-home row leave together with the Filesystem-faults row's unseeded carve-out and its windows-2025 grant cases. Afterwards no test seeds a home, and every remaining widened-home case widens real Unix modes on a real tempfile home through the `widen` helpers (test-plan/test-data-fixtures fct-bxlupfu3) (test-plan/test-data-fixtures fct-oanlcr47) (test-plan/test-data-fixtures fct-t5v77dkt) (test-plan/mocking-stubbing-discipline fct-zsbxg2jw).
@binding n=2
**test-data-fixtures ↔ mocking-stubbing-discipline (the impostor cases)**: when `tests/security_negatives_server_identity.rs` and `tests/hook_fail_open.rs` lose their Windows arm, the Linux impostor keeps writing its own snapshot and recording the pid of the fake agent's real `--hold-stdout-internal` holder, never a double. Its endpoint directory stays `create_private_dir`, and the Windows-only `CreateNamedPipeW` control pipes and `ImpersonateAnonymousToken` leave (test-plan/test-data-fixtures fct-rgx56qfp) (test-plan/test-data-fixtures fct-fvd26vjh) (test-plan/mocking-stubbing-discipline fct-sik7tvk4) (test-plan/mocking-stubbing-discipline fct-pwscsozw) (test-plan/test-data-fixtures fct-vrevms4b).
@binding n=3
**test-data-fixtures / mocking-stubbing-discipline ↔ the scope's cfg(unix) rule**: each cfg(unix) arm whose only sibling leaves (the `Frozen` guard's `SIGSTOP`, the holder's and the fake agent's process groups, `boot_as_user`'s `HOME`, the per-OS oracle's Unix refusal, `SOCKET_ROOT_VAR`'s `XDG_RUNTIME_DIR`) loses its gate and keeps its Linux behaviour unchanged (test-plan/mocking-stubbing-discipline fct-63urz52n) (test-plan fct-7o4nbra2) (test-plan/test-data-fixtures fct-652w7iir) (test-plan/test-data-fixtures fct-ubb4ig4c) (test-plan/test-data-fixtures fct-jfjmhkwu).
@binding n=4
**mocking-stubbing-discipline ↔ the scope's boundary and the e2e-home free-space fix**: no env, flag or feature seam may force a refusing peer-euid decision; the decision stays injected as an argument. The free-space fix follows the same discipline: the two harness cases take their free-space figure as an argument or constructor value the test controls, never an env seam, and never a stubbed filesystem (test-plan/mocking-stubbing-discipline fct-6xaoh2kl) (test-plan fct-3a3m4jll) (test-plan fct-hkmaq6ye).
@binding n=5
**test-data-fixtures ↔ the scope's OS-text fork**: help strings, hints and doc comments that name removed Windows or macOS code are listed by P3 and decided by P4. Any that changes moves its pinned literal (`HELP`, `CAUSES`, `SOCKET_DIR`, `HINT`) in the same commit, and `record_live`'s Windows reason is one such text (test-plan/test-data-fixtures fct-6wjlyq3w) (test-plan/test-data-fixtures fct-wmieczsn).
@binding n=6
**test-data-fixtures / mocking-stubbing-discipline ↔ the test-plan masters**: §7's and §8's rows that name Windows or macOS (seed_conpty, the widened seeded home, `icacls`, `USERPROFILE`, `TMPDIR` on macOS, the Windows control pipes, the windows-2025 grants) are master text that the wrap's amendments rewrite. The chunk's plan names them as expected amendments, never edits them itself (test-plan/test-data-fixtures fct-bxlupfu3) (test-plan/mocking-stubbing-discipline fct-pwscsozw).
