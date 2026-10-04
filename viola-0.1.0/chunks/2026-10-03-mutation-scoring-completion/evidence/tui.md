# The in-repo TUI boundary set on the Linux host (step 9)

Command: `bash scripts/agent-run.sh run --integration --filter 'binary(/^tui_/)'` (gate entry 8, this run)
→ `ok:true`, nextest-integration **12 passed / 0 failed**. It read the same on the first gate pass (archive 29) and on the
final pass after the scoring kill tests (archive 50). P3 read 12 / 0 as well.

| binary | test |
|---|---|
| `tui_pty_seam` | `pty_exit_is_read_on_the_handle_while_the_output_is_held` |
| `tui_pty_seam` | `pty_resize_reaches_the_child` |
| `tui_pty_seam` | `pty_resize_reaches_a_child_that_reads_no_key` |
| `tui_passthrough` | `tui_keys_reach_the_child_as_typed_and_ctrl_c_ends_it` |
| `tui_passthrough` | `tui_child_output_passes_through_without_viola_bytes` |
| `tui_passthrough` | `tui_hooks_firing_add_no_viola_bytes` |
| `tui_passthrough` | `tui_host_resize_reaches_the_child` |
| `tui_passthrough` | `tui_host_resize_in_the_pump_start_window_reaches_the_child` |
| `tui_env_strip` | `tui_env_strip_keeps_a_pass_listed_name_on_unix` |
| `tui_env_strip` | `tui_env_viola_names_reach_the_child` |
| `tui_env_strip` | `tui_env_strip_removes_identity_and_unknown_claude_names` |
| `tui_channel_fds` | `tui_channel_fds_the_child_holds_nothing_of_violas` |

## What this host witnesses, and what it does not
- **openpty only.** On Linux, portable-pty's Unix PTY system backs every one of these cases. None of them exercises
  ConPTY, inbox or sideloaded.
- **The ConPTY cases are not credited here.** They stay witnessed on the `windows-2025` CI runner (a11y-plan §3
  Tooling).
- **a11y-plan §4 P4 clause (3) has no case at HEAD.** No `tests/tui_*.rs` asserts focus, mouse or wheel
  (`grep -lnE 'focus|mouse|1004|wheel' tests/tui_*.rs` → no file). That case belongs to the wheel / driving-verbs route
  entries, not to this chunk. It is recorded here, not built.
