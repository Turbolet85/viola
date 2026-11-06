@carried n=1 label=test-plan/integration-test-strategy fact=fct-ouc4a2b7
why: item 7 asks the founder's word on the socket-directory pair's wording, so the plan's statement of the socket-directory checks and their witnesses must read as his ruling leaves it.
quote: The directory is also checked before use: `check_socket_dir`, through `check_endpoint_dir`, refuses a directory with a foreign bit or a foreign owner, a symlink or a non-directory, passes an absent path and never chmods, a refused directory staying as found;
@carried n=2 label=test-plan/integration-test-strategy fact=fct-vmzc7an2
why: moving portable-pty off =0.8.1 moves the dependency under viola-pty, so the real-openpty boundary tests (exit on the handle, a resize propagated, a key after the reported size) must stay green on the new release.
quote: Real openpty spawn of `viola-fake-agent` through `viola-pty`. The no-EOF mode checks exit on handle, a resize is propagated
@carried n=2 run=2 label=test-plan fact=fct-nvtf2boy
quote: real openpty spawns of `viola-fake-agent` through `viola-pty`: exit detected on the handle, a resize propagated, and a key written after a resize only once the child reports the new size.
@carried n=2 run=3 label=test-plan fact=fct-ukay3vyw
quote: NEVER detect child exit by reading master EOF; use `child.wait()` on the handle.
@carried n=3 label=test-plan/integration-test-strategy fact=fct-vmzc7an2
why: the boundary's Tools line names the pin the chunk moves, so it must name the new portable-pty release once item 12 lands.
quote: **Tools:** portable-pty `=0.8.1` via viola-pty seam
@carried n=3 run=2 label=test-plan fact=fct-5vfqglut
quote: **PTY:** real portable-pty through `viola-pty`.
@carried n=4 label=test-plan/integration-test-strategy fact=fct-aajqyzhy
why: item 1 re-measures that no manifest holds trycmd or assert_cmd, so every plan line naming assert_cmd as a driver must agree with the masters-agree verdict, tool or text.
quote: **Tools:** assert_cmd 2.2.2
@carried n=4 run=2 label=test-plan fact=fct-s42l2rwd
quote: **CLI and hooks:** assert_cmd 2.2.2 + predicates 3.1.4 against `CARGO_BIN_EXE_viola`.
@carried n=4 run=3 label=test-plan/integration-test-strategy fact=fct-lzdm6e7w
quote: `tests/cli_verify.rs` pins `viola verify`'s human lines by literal assert_cmd asserts
@carried n=5 label=test-plan/integration-test-strategy fact=fct-ksdwqwkf
why: item 8 holds that an always-admit peer_admitted read at the three client seams, MCP's frame among them, survives every committed test, so this line naming checks that hold each cause must agree with that untestable zone.
quote: MCP's mapping (`crates/viola-mcp/src/tools.rs` `frame`, `crates/viola-mcp/src/result.rs` `of_reply`): a refused `check_instance` is `strict-modes-failed`, no `live_endpoint` is `without_endpoint`'s reading, a refused `check_endpoint_dir` after `live_endpoint` is `socket-dir-refused`, a peer euid `peer_admitted` refuses or a failed `server_verified` is `server-verify-failed`, `Connect` is `instance-dead`, and `Closed` or `Io` is `endpoint-lost`, with no frame and no line; `handlers_frame_tools_name_the_cause_of_an_unreachable_instance` (`crates/viola-mcp/tests/handlers.rs`) and `mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost` (`crates/viola-e2e/tests/mcp_tools.rs`) hold each cause its word.
@carried n=6 label=test-plan/integration-test-strategy fact=fct-bt3npe7x
why: item 7 asks the founder's word on the board's read rule, so the plan's statement of that rule and the check that holds it must read as his ruling leaves it.
quote: a home made before viola that another user may read (0750) refuses `viola run`: exit 1 with the home pair, no `diagnostics/`, the grant as found (`tests/security_negatives_strict_modes.rs` `a_home_another_user_may_read_refuses_run`).
    - The same home refuses `viola list`, held to the read rule by `list_refuses_a_home_another_user_may_read_with_and_without_an_instance` (`tests/cli_list.rs`), with and without an instance, the refusal the whole board's, `ScanRefused::Home`, and opening nothing.
@carried n=7 label=test-plan/integration-test-strategy fact=fct-lzdm6e7w
why: item 1 re-measures that no `tests/cmd/` exists and no manifest holds trycmd, so the plan's trycmd lines must follow the masters-agree verdict, the tool built or the text re-worded to the literals that pin the layouts.
quote: trycmd `tests/cmd/*.toml` covers the `viola-send` mirror `[RB]`/`[  ]`/`[/ ]`.
@carried n=7 run=2 label=test-plan fact=fct-klrvzf5j
quote: **Human-mode layouts:** trycmd 1.2.1 case files, except the `viola list` board, pinned by literal asserts in `tests/cli_list.rs` (chunk 2026-10-11-the-board-viola-list), and `viola --help`, pinned by the `HELP` literal in `tests/cli_output_plain.rs` (chunk 2026-10-17-cli-output-discipline).
@carried n=7 run=3 label=test-plan fact=fct-w5xcngvx
quote: the human layouts by trycmd or literal with no `\x1b[` under non-TTY / `NO_COLOR` / `TERM=dumb`
@carried n=8 label=test-plan/integration-test-strategy fact=fct-lzdm6e7w
why: the trycmd verdict of item 1 reaches the plan's promised trycmd case and its snapbox redactions too, which stand or are re-worded with it.
quote: A trycmd case for the verify step counter arrives with a redaction-heavy consumer
@carried n=8 run=2 label=test-plan/integration-test-strategy fact=fct-lzdm6e7w
quote: snapbox redactions `[..]` mask timestamps, cursors and temp paths
@carried n=9 label=test-plan/integration-test-strategy fact=fct-lzdm6e7w
why: item 17 edits the Windows remnants at tests/cli_output_plain.rs:104,116, so this witness must keep the HELP literal and the served-verb check holding, and its plan wording follows the remnant's removal or kept reason.
quote: `viola --help` under the outer PTY carries no SGR whose parameters hold bold `1` or underline `4` (the `ESC [ params m` parameters are parsed, never the screen text), and on Unix no CR that is not directly followed by LF; one case self-checks that parser against literals. Since chunk 2026-10-17-cli-output-discipline it also pins the grouped help: `help_is_the_grouped_verb_table` (the whole stdout as the `HELP` literal, exit 0, stderr empty) and `help_lists_only_served_verbs` (each served verb path answers `--help` exit 0, `plugin install --help` included, its usage program read as `viola` or `viola.exe` followed by the verb's words;
@carried n=10 label=test-plan/integration-test-strategy fact=fct-lzdm6e7w
why: item 17 removes or keeps `plain_drive` in `src/cmd/mod.rs` with its reason, so the units backing the home resolution order must follow, the `plain_drive_*` name leaving the plan with the code or kept with that reason.
quote: pins the home resolution order `--home` → the `VIOLA_NAME` + `VIOLA_DIR` instance home → `<user home>/.viola`, canonicalised once it exists (`home_resolution_takes_the_flag_then_the_session_then_the_default`, `two_homes_are_isolated`, `a_home_reached_through_a_link_is_canonicalised`), backed by the `src/cmd/mod.rs` units `resolve_home_*`, `canonical_home_*` and `plain_drive_*`;
@binding n=1
**Module-to-PTY boundary ↔ supply-chain trigger and the §1 viola-pty entity**: the portable-pty release that replaces `=0.8.1` must drop `pin-utils` so `cargo deny` reads clean with RUSTSEC-2017-0008 still the only ignore (test-plan fct-hlx2zkbt), while the real-openpty boundary keeps exit on the handle, the propagated resize and the key after the reported size green (test-plan/integration-test-strategy fct-vmzc7an2); the §5 Tools line and the §1 viola-pty entity's pin text (test-plan fct-njspb5nv) both name the new release.
@binding n=2
**§5 assert_cmd and trycmd drivers ↔ §2 invariants and §7 human-mode expected output**: one masters-agree verdict on item 1 (tool or text) governs every line naming the tools: the Hook exec and CLI Tools and driver lines (test-plan/integration-test-strategy fct-aajqyzhy, test-plan fct-s42l2rwd), the trycmd cases and redactions (test-plan/integration-test-strategy fct-lzdm6e7w, test-plan fct-klrvzf5j), «trycmd only in verify mode» (test-plan fct-gcer5i4i) and «trycmd cases with snapbox redactions» (test-plan fct-puo2hazj); none may keep naming a tool or a `tests/cmd/` tree the manifests do not hold.
@binding n=3
**cli_output_plain.rs remnant removal ↔ multi-os-compat census**: removing or keeping the Windows remnants at lines 104 and 116 must keep `help_is_the_grouped_verb_table` and `help_lists_only_served_verbs` green (test-plan/integration-test-strategy fct-lzdm6e7w) and leave no gate naming `windows` or `target_os = "macos"`, the census in `tests/contract_os_gated_tests.rs` reading its count and gated files consistent (test-plan fct-5akis2u7).
@binding n=4
**MCP peer_admitted mapping ↔ §1 and §6 untestable zone**: the line holding each unreachable cause its word (test-plan/integration-test-strategy fct-ksdwqwkf) must not claim a committed test reads MCP `frame`'s euid wiring, since the untestable zone records an always-admit `peer_admitted` mutant at the three client seams surviving every committed test with no seam to be added (test-plan fct-upolcsa3, test-plan fct-lkgqv65p).
@binding n=5
**Board read rule and socket-directory checks ↔ §4 viola-state units, on the founder's word (item 7)**: the plan's board read rule (test-plan/integration-test-strategy fct-bt3npe7x) and socket-directory checks (test-plan/integration-test-strategy fct-ouc4a2b7) stand, or are re-worded, exactly as the founder rules, together with their unit witnesses `board_scan_holds_the_home_to_the_read_rule_with_and_without_an_instance` and the socket directory verdict cases (test-plan fct-7vfmbe3m).
