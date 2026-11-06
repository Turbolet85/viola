@apply_row n=1 ref_file=fanout-test-plan--cascade-g6.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-test-plan--cascade-g6.md ref_n=2 v=refused
why: a hand removal · fct-oanlcr47
@apply_row n=3 ref_file=fanout-test-plan--cascade-g6.md ref_n=3 v=applied edit=E2
@apply_row n=4 ref_file=fanout-test-plan--cascade-g6.md ref_n=4 v=applied edit=E3
@apply_row n=5 ref_file=fanout-test-plan--cascade-g6.md ref_n=5 v=applied edit=E4
@apply_row n=6 ref_file=fanout-test-plan--cascade-g6.md ref_n=6 v=applied edit=E5
@apply_row n=7 ref_file=fanout-test-plan--cascade-g6.md ref_n=7 v=applied edit=E6
@apply_row n=8 ref_file=fanout-test-plan--cascade-g6.md ref_n=8 v=applied edit=E7
@apply_row n=9 ref_file=fanout-test-plan--cascade-g6.md ref_n=9 v=applied edit=E8
@apply_row n=10 ref_file=fanout-test-plan--cascade-g6.md ref_n=10 v=applied edit=E9
@apply_row n=11 ref_file=fanout-test-plan--cascade-g6.md ref_n=11 v=applied edit=E10
@apply_row n=12 ref_file=fanout-test-plan--cascade-g6.md ref_n=12 v=applied edit=E11
@fact id=fct-bxlupfu3 expect_rev=1
old: Four exceptions: the chaos tests, which corrupt files the product already wrote; the server-identity impostor cases (below); the MCP
new: Three exceptions: the chaos tests, which corrupt files the product already wrote; the server-identity impostor cases (below); and the MCP
old: `record_live` below); and on Windows x64 the root tests' `seed_conpty`, which hard-links (or copies) the ConPTY companions into `<home>/bin/<key>/conpty/`, byte-identical to what viola would write (§5 carve-out; `conpty_sideload` never seeds, so it covers the product's write). `seed_conpty` makes the home and the seed dir `target/conpty-seed/<key>/` through `viola_state::fs::create_private_dir` before `bin/<key>/conpty/`, so a seeded home passes `check_instance` and, on windows-2025, the three `hook_statusline` cases pass. A home made with `create_dir_all` before viola is refused: on Unix (0755) by the read rule; on windows-2025 `Writable`, the refused entry an inherited write grant to a SID strict-modes does not trust, read by its right whichever SID holds it (`strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry`, as measured at ci#40439188053); that home's canonical SDDL is not recorded. A case
new: `record_live` below). A home made with `create_dir_all` before viola is refused (0755) by the read rule. A case
old: viola created on every OS starts from `stamped_home`, as the every-OS `hook_statusline` cases
new: viola created starts from `stamped_home`, as the `hook_statusline` cases
@fact id=fct-652w7iir expect_rev=1
old: sets `HOME` (on Windows `USERPROFILE`) on that one child
new: sets `HOME` on that one child
@fact id=fct-jfjmhkwu expect_rev=1
old: `SOCKET_ROOT_VAR` (`TMPDIR` on macOS, else `XDG_RUNTIME_DIR`)
new: `SOCKET_ROOT_VAR` (`XDG_RUNTIME_DIR`)
@fact id=fct-t5v77dkt expect_rev=1
old: (Unix modes; on Windows `icacls`, its exit asserted).
new: (Unix modes).
@fact id=fct-myaghpwm expect_rev=1
old: and on Unix Run B's probe status line
new: and Run B's probe status line
@fact id=fct-rgx56qfp expect_rev=1
old: impostor cases (Linux and Windows: `served` in
new: impostor cases (`served` in
@fact id=fct-wmieczsn expect_rev=1
old: `handlers_frame_tools_name_the_cause_of_an_unreachable_instance`: a Windows `Serving` has no `Drop` (its pipe lives in the accept thread until the process exits), so the row binds nothing.
new: `handlers_frame_tools_name_the_cause_of_an_unreachable_instance`.
@fact id=fct-ubb4ig4c expect_rev=1
old: A home planted under a regular file takes a per-OS oracle: on Unix it is a strict-modes refusal, while on Windows such a path stats `NotFound`, the check skips it as absent and the catch-site line stands (`verify_under_a_regular_file_refuses_the_home`).
new: A home planted under a regular file is a strict-modes refusal (`verify_under_a_regular_file_refuses_the_home`).
@fact id=fct-pwscsozw expect_rev=1
old: `viola-fake-agent` as the fake CLI; windows-sys 0.61.2 `CreateNamedPipeW` (Windows only: test-owned control pipes) |
new: `viola-fake-agent` as the fake CLI |
old: before `run`; in the listener-admission negatives the test-owned control pipes (accept-remote, reject-remote, the default descriptor) stand beside the real `viola_channel::Server::bind` listener, and `ImpersonateAnonymousToken` stands in for a foreign principal; in the server-verification negatives (Linux and Windows) an impostor
new: before `run`; in the server-verification negatives an impostor
@fact id=fct-zsbxg2jw expect_rev=1
old: none (a real tempfile home, real modes and ACLs)
new: none (a real tempfile home, real modes)
old: never a stubbed one: on windows-2025 an instance tree granted Users write, or a `diagnostics/` granted Users read, is refused beside a `create_private_dir` tree that passes; the pure verdicts (`windows_read_verdict` among them) take synthetic descriptors at the unit tier; a test that widens a home tree above `bin/<key>/conpty/` starts its home unseeded (§7, a widened seeded home); a loosened
new: never a stubbed one; a loosened
@fact id=fct-6xaoh2kl expect_rev=1
old: `peer_euid_admitted(observed, own, reported)` takes its euids as arguments
new: `peer_euid_admitted(observed, own)` takes its euids as arguments (admitting `observed == Some(own)`; an unreadable euid refused)
@pattern id=tp5-01
fixed: hard-links (or copies) the ConPTY companions
@pattern id=tp5-02
fixed: (on Windows `USERPROFILE`)
@pattern id=tp5-03
fixed: `TMPDIR` on macOS, else
@pattern id=tp5-04
fixed: on Windows `icacls`, its exit asserted
@pattern id=tp5-05
fixed: on Unix Run B's probe status line
@pattern id=tp5-06
fixed: impostor cases (Linux and Windows
@pattern id=tp5-07
fixed: a Windows `Serving` has no `Drop`
@pattern id=tp5-08
fixed: on Windows such a path stats `NotFound`
@pattern id=tp5-09
fixed: `ImpersonateAnonymousToken` stands in for a foreign principal
@pattern id=tp5-10
fixed: an instance tree granted Users write
@pattern id=tp5-11
fixed: peer_euid_admitted(observed, own, reported)
