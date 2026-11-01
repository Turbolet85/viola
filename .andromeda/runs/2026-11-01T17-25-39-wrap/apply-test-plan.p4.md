# Apply — test-plan · part 4 of 5

## Verdicts
- fanout-test-plan--cascade-g3.md 1 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:37+#1
- fanout-test-plan--cascade-g3.md 2 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:250#1
- fanout-test-plan--cascade-g3.md 4 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:252+#1
- fanout-test-plan--cascade-g3.md 5 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:264+#1
- fanout-test-plan--cascade-g3.md 6 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:278+#1
- fanout-test-plan--cascade-g3.md 7 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:280#1
- fanout-test-plan--cascade-g3.md 8 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:284#1
- fanout-test-plan--cascade-g3.md 9 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:289+#1
- fanout-test-plan--cascade-g3.md 10 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:289#1
- fanout-test-plan--cascade-g3.md 11 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:297+#1
- fanout-test-plan--cascade-g3.md 12 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:300#1
- fanout-test-plan--cascade-g3.md 14 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:317#1
- fanout-test-plan--cascade-g3.md 15 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:318#1
- fanout-test-plan--cascade-g3.md 16 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:318+#1
- fanout-test-plan--cascade-g3.md 17 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:323+#1
- fanout-test-plan--cascade-g3.md 19 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:14+#1
- fanout-test-plan--cascade-g3.md 20 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:30#1
- cascade-compare-test-plan.md 4 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:15+#1
- cascade-compare-test-plan.md 5 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:19#1
- cascade-compare-test-plan.md 6 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:32+#1

## Edits
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:37
  anchor: «  - The start runs the home-tree strict-modes check (`check_home_tree`: the home, `check_instance`'s set, `ledger/`, `bin/` and `plugin/`, then `bin/<key>/`, the pinned copy, `plugin/<key>/`, its `hooks/` and `.claude-plugin/`, each that exists) before the wrapper log opens. A refused tree exits 1 with the home-state `unable:` / `hint:` pair, or the instance-state pair when the refusal is at instance level, and opens no log, writes no line and starts no child (`tests/run_cli.rs` `run_refuses_a_widened_home_tree_before_its_log`; `tests/security_negatives_strict_modes.rs` `a_home_another_user_may_read_refuses_run`).»
  add: «  - On Unix the start checks the socket directory (`socket_dir_ready`) before any snapshot, child or `Server::bind`, `revive`'s start sharing the check: a refused directory exits 1, logs `process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"}` (a step with no span, one line), binds nothing and spawns no child; a `Server::bind` failure is not this refusal (`tests/security_negatives_socket_dir.rs` `security_negative_a_start_refuses_the_socket_directory_before_any_bind`).»

- line: .andromeda/test-plan-detail/e2e-test-strategy.md:250
  old: «a server that is not the recorded wrapper (`server-verify-failed`). The sweep»
  new: «a server that is not the recorded wrapper (`server-verify-failed`); on Unix, beyond this matrix, a socket directory the check refuses (`socket-dir-refused`, `security_negative_a_hook_fails_open_on_the_socket_directory` in `tests/security_negatives_socket_dir.rs`) and a server of another euid (`server-verify-failed`, its wiring with no committed witness) each exit 0 with empty streams and no frame. The sweep»

- after: .andromeda/test-plan-detail/e2e-test-strategy.md:252
  anchor: «  The impostor rows, each under `cfg_attr(not(target_os = "macos"), …)`, are `case::impostor` in `hook_fails_open_silently_within_the_spine_bound` and `question_impostor` and `permission_impostor` in `hook_dialog_tier_fails_open_silently`, driving `stop`, `pre-tool-use` and `permission-request` against a same-user server on the recorded endpoint while the snapshot records another live process (the fake agent's `--hold-stdout-internal` holder): each exits 0 with empty streams under the 1.0 s `Instant` bound, no frame reaches the server, and one `hook-decision` line carries `detail:"server-verify-failed"` (on the dialog path after `hook-invoked{corr:null}`, with `decision_emitted:false`).»
  add: «»
  add: «  A refused socket directory's record is one `hook-decision` line at WARN carrying `detail:"socket-dir-refused"` (on the dialog path after `hook-invoked{corr:null}`, with `decision_emitted:false`).»

- after: .andromeda/test-plan-detail/e2e-test-strategy.md:264
  anchor: «  - `channel_error_bodies` and `cli_error_bodies` are `[[test]]` tables with `required-features = ["fake-agent"]`, each on the `ci` profile's 20 s kill line.»
  add: «  - `cli_error_documents_of_a_loosened_socket_directory_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`) holds the socket directory's error documents, `revive --json`'s among them; `assert_names_nothing` and `digit_runs` (`tests/security_negatives_socket_dir.rs`) hold its log lines free of path, pid and uid.»

- after: .andromeda/test-plan-detail/e2e-test-strategy.md:278
  anchor: « `tests/cross_exit_causes.rs` asserts each cause in both modes where a mode exists: the document under `--json`, the human form ending in its hint, and each logged cause's one `process-exit` line with its own `detail`.»
  add: « `tests/cross_exit_causes.rs` holds `a_loosened_socket_directory_on_a_frame_verb_is_socket_dir_refused` and `a_loosened_socket_directory_on_a_start_is_socket_dir_refused`, each in human and `--json` form, and pins the socket directory's hint as the literal `SOCKET_DIR`; the table's self-checks read `socket-dir-refused`.»

- line: .andromeda/test-plan-detail/e2e-test-strategy.md:280
  old: «exit 21's five closed `detail` causes are `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost` and `server-verify-failed`, on the eight frame verbs, `list` and `plugin install`, with»
  new: «exit 21's six closed `detail` causes are `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed` and `socket-dir-refused`, on the eight frame verbs, `list` and `plugin install` (`socket-dir-refused` on the eight frame verbs alone), with»

- line: .andromeda/test-plan-detail/e2e-test-strategy.md:284
  old: «On macOS the client reads no peer pid and compares no server (the recorded gap): the client euid check and the `#[cfg(target_os = "macos")]` case that loosens the verified per-user socket dir to 0755 and asserts exit 21 (security excerpt, Vector 1) are not built, one CARRY on the route entry "Unix endpoint and home hardening".»
  new: «On Linux and macOS a server of another euid reads `server-verify-failed` too, through the client euid check, its decision held by injected values and each seam's wiring by no committed test; on macOS that is the word's whole meaning, the client reading no peer pid and comparing no server (the named carve-out, owned by no route entry). A loosened socket directory on macOS is the socket directory cause below, not this one.»

- after: .andromeda/test-plan-detail/e2e-test-strategy.md:289
  anchor: «    - a wrapper lost with the call open: `detail` `endpoint-lost`, `during:"call"`, its own hint, the table's `a_wrapper_killed_under_a_parked_wait_is_endpoint_lost`; `instance-dead`, `unwrapped-name` and `server-verify-failed` are `during:"connect"`»
  add: «    - the socket directory is refused (Unix: a directory at 0755 or a symlink to 0700): `detail` `socket-dir-refused`, its own hint, cause `socket-dir` in `src/human/hints.rs`; its log line is `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN, with no `channel-request`»

- line: .andromeda/test-plan-detail/e2e-test-strategy.md:289
  old: «`instance-dead`, `unwrapped-name` and `server-verify-failed` are `during:"connect"`»
  new: «`instance-dead`, `unwrapped-name`, `server-verify-failed` and `socket-dir-refused` are `during:"connect"`»

- after: .andromeda/test-plan-detail/e2e-test-strategy.md:297
  anchor: «    - the `--home` fails strict-modes at `run`: the home-tree check before the wrapper log (Path 1), exit 1 with the home-state pair or, at instance level, the instance-state pair, no log, no line, no child, a read-granted pre-existing home included on every OS (`a_home_another_user_may_read_refuses_run`); beside it, `viola verify` over a refused home tree (the home, `ledger/`, `bin/`, `plugin/`) exits 1 with the home-state pair, no step line and no summary (`tests/cli_verify.rs` `verify_refuses_a_widened_ledger_before_any_step`)»
  add: «    - the socket directory is refused at the start (Unix), before any snapshot, child or `Server::bind`, `revive`'s start sharing it: exit 1, one `process-exit` line with `detail` `socket-dir-refused`, the table's `a_loosened_socket_directory_on_a_start_is_socket_dir_refused`; a failed create and a refused check read one code, and a `Server::bind` failure is not this cause»

- line: .andromeda/test-plan-detail/e2e-test-strategy.md:300
  old: «`squatted-name` and `internal-error`.»
  new: «`squatted-name`, `socket-dir-refused` and `internal-error`.»

- line: .andromeda/test-plan-detail/e2e-test-strategy.md:317
  old: «(`#[cfg(target_os = "linux")]`, IPC access control). With a test-owned `XDG_RUNTIME_DIR`, pre-create `viola/` as (a) a symlink to another 0700 dir and (b) a 0755 dir. In each case `viola run` exits 1, a CLI client verb against an existing instance exits 21, and `viola hook <event>` exits 0 with no body and writes no frame. The foreign-owner case needs another uid, which a non-root runner cannot create, so it runs at unit level: the directory-check function gets an injected `lstat` result with `owner != geteuid()` and must refuse.»
  new: «(`#![cfg(unix)]`, IPC access control). With a test-owned socket root (`SOCKET_ROOT_VAR`), pre-create the per-user socket directory as (a) a symlink to another 0700 dir and (b) a 0755 dir. In each case `viola run` exits 1, a CLI client verb against an existing instance exits 21, and `viola hook <event>` exits 0 with no body and writes no frame, the directory left as found. The foreign-owner case needs another uid, which a non-root runner cannot create, so it runs at unit level: `socket_dir_verdict` gets an injected uid other than the euid and must refuse; a non-directory reads `Refused::NotADirectory` there.»

- line: .andromeda/test-plan-detail/e2e-test-strategy.md:318
  old: «The socket-directory refusal has no code: `bind_endpoint`'s `create_private_dir` failure is an anyhow error at the catch site, owned by the route entry "Unix endpoint and home hardening" with macOS's server check.»
  new: «The socket-directory refusal's code is the closed word `socket-dir-refused` (`check_socket_dir`, which never chmods): exit 1 at `run` and `revive`'s start, exit 21 at the eight CLI frame verbs, exit 0 with empty streams at the hook, and `instance-unreachable` with no line at MCP's `frame`; off Unix the check returns `Ok(())`, the DACL and the pid comparison staying the controls.»

- after: .andromeda/test-plan-detail/e2e-test-strategy.md:318
  anchor: « The socket-directory refusal has no code: `bind_endpoint`'s `create_private_dir` failure is an anyhow error at the catch site, owned by the route entry "Unix endpoint and home hardening" with macOS's server check.»
  add: «    - As landed: `tests/security_negatives_socket_dir.rs`, a Unix-only binary, drives a directory at 0755 and a symlink to 0700 against a start, the eight frame verbs and the hook, each left as found, through `security_negative_a_start_refuses_the_socket_directory_before_any_bind`, `security_negative_a_frame_verb_refuses_the_socket_directory_before_any_frame` and `security_negative_a_hook_fails_open_on_the_socket_directory`.»

- after: .andromeda/test-plan-detail/e2e-test-strategy.md:323
  anchor: «  - **Umask-independent modes** (`#[cfg(unix)]`). Start `viola run` under `umask 000` (via `sh -c 'umask 000; exec viola run …'`). `events.ndjson`, `snapshot.json` and the other home files are still 0600, directories 0700, and the pinned `bin/<version>-<hash>/viola` is 0700 and never group- or world-writable.»
  add: «    - `security_negative_a_clean_start_under_an_open_umask_keeps_the_socket_private` (`tests/security_negatives_socket_dir.rs`) holds a clean start's socket private under an open umask.»

- after: .andromeda/test-plan-detail/test-data-fixtures.md:14
  anchor: «| The statusline source (`<home>/statusline-source.json`, or the user's `.claude/settings.json` for a default home) | Written by the test, never by viola, which only reads it: `tests/support/home.rs` `plant_statusline_source(home, command)`, with the command from `statusline_echo_command(marker, extra)` and the marker at `statusline_marker(home)`. A default-home case takes `TestHome::default_of_user` and `Wrapper::boot_as_user`, which sets `HOME` (on Windows `USERPROFILE`) on that one child. | per-test |»
  add: «| A test-owned socket root | `tests/support/home.rs`, under `/tmp`: `SOCKET_ROOT_VAR` (`TMPDIR` on macOS, else `XDG_RUNTIME_DIR`), `SocketRoot`, `Wrapper::boot_on_socket_root` and `first_life_on`, the last there and not in `tests/support/start.rs` | per-test |»

- line: .andromeda/test-plan-detail/test-data-fixtures.md:30
  old: «beside an endpoint the test binds, so that»
  new: «beside an endpoint the test binds in a private directory (`viola_state::fs::create_private_dir`, below), so that»

- after: .andromeda/test-plan-detail/test-data-fixtures.md:15
  anchor: «| Widened test homes (the strict-modes negatives) | Set by the test on the real file, never by viola: `tests/support/home.rs` `widen` · `grant_read` · `unwiden` · `still_widened` (Unix modes; on Windows `icacls`, its exit asserted). | per-test |»
  add: «| A loosened socket directory (the socket-directory negatives) | Set by the test on the real directory, never by viola, whose check never chmods it: `tests/security_negatives_socket_dir.rs` `Form` (`apply` · `assert_as_found` · `restore`), a directory at 0755 or a symlink to 0700, the refused directory asserted as found. A foreign owner is not driven: `socket_dir_verdict` takes it by an injected uid. | per-test |»

- line: .andromeda/test-plan-detail/test-data-fixtures.md:19
  old: «(`tests/cross_exit_causes.rs` `CAUSES`), are literals in their test files»
  new: «(`tests/cross_exit_causes.rs` `CAUSES`), and the socket-directory refusal's last `hint:` line (`tests/cross_exit_causes.rs` `SOCKET_DIR`, `tests/security_negatives_socket_dir.rs` `HINT`), are literals in their test files»

- after: .andromeda/test-plan-detail/test-data-fixtures.md:32
  anchor: «- A home a test makes before viola, where the case needs it to pass the strict-modes read rule, is made through `viola_state::fs::create_private_dir`, never `create_dir_all` (a `create_dir_all` home is refused, On-disk product state above), as `tests/contract_diag_schema.rs`, `tests/cli_machine_contract.rs` and `tests/tui_env_strip.rs` do. A catch-site case holds a viola-made home with a regular file where the checked entry goes (`tests/cli_wait_last.rs`, `diagnostics/`; `tests/cli_verify.rs`, `ledger/`). A home planted under a regular file takes a per-OS oracle: on Unix it is a strict-modes refusal, while on Windows such a path stats `NotFound`, the check skips it as absent and the catch-site line stands (`verify_under_a_regular_file_refuses_the_home`).»
  add: «- A directory holding a test's endpoint, where the case needs it to pass the socket-directory check, is made through `viola_state::fs::create_private_dir`, never left a bare `tempfile::tempdir()`: `tempfile` 3.27 leaves the mode to the umask (0755 as measured at chunk 2026-11-01-unix-endpoint-and-home-hardening), and such a directory meets `check_endpoint_dir` as a home meets the read rule. `crates/viola-mcp/tests/handlers.rs`, `tests/hook_fail_open.rs`, `tests/security_negatives_server_identity.rs` and `refused_inside` in `src/cmd/client.rs` make theirs so; §5 states the bare temp dir's 0755 reading for a home.»

## Patterns
[[pattern]]
id = "tp4-01"
fixed = 'five closed `detail` causes'

[[pattern]]
id = "tp4-02"
fixed = 'the client euid check and the `#[cfg(target_os = "macos")]` case'

[[pattern]]
id = "tp4-03"
fixed = '`unwrapped-name` and `server-verify-failed` are `during`'

[[pattern]]
id = "tp4-04"
fixed = '`squatted-name` and `internal-error`'

[[pattern]]
id = "tp4-05"
fixed = 'the directory-check function gets an injected'

[[pattern]]
id = "tp4-06"
fixed = 'socket-directory refusal has no code'
