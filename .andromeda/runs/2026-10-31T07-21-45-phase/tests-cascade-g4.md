# tests · cascade read · group 4 of 7

## Carried
- (.andromeda/test-plan-detail/integration-test-strategy.md:78) The CLI witness and the enumeration must hold every verb's error document within this per-exit-code coverage, each a closed code with a code-shaped or null `detail`.
  > root `tests/cli_*.rs` covers the `--json` document per exit code
  (.andromeda/test-plan.md:452)
  > root `tests/cli_*.rs` cover the `--json` document per exit code
  (.andromeda/test-plan.md:413)
  > assert_cmd 2.2.2 + predicates 3.1.4 against `CARGO_BIN_EXE_viola`.
- (.andromeda/test-plan-detail/integration-test-strategy.md:97) The machine-contract suite reads each driver-facing verb's document and literal exit, so every sanitised CLI error body must keep its literal exit in these worlds.
  > `tests/cli_machine_contract.rs` reads each driver-facing verb's `--json` document and literal exit over one live wrapper's life (`every_verb_prints_one_document_and_its_literal_exit_over_a_wrapper_s_life`, a labelled case table per world), over a home with no instance (`verbs_over_a_home_with_no_instance_print_their_documents`) and over a home another user could write (`verbs_over_a_home_another_user_could_write_print_their_documents`)
- (.andromeda/test-plan-detail/integration-test-strategy.md:91) A `--json` usage error is an error document the CLI surface owes, and it must carry a code and no free text even when the rejected argv value holds a path-shaped word.
  > Each case exits 2 with a `--json` usage error.
  (.andromeda/test-plan-detail/integration-test-strategy.md:90)
  > `tests/cli_slash_args.rs` (Vector 6, §6 rewritten-path warning): an rstest `#[case]` table passes leading-slash argv values (`/clear`, `/model x`, `//server/share`) in the target and text positions of `send` and `answer`, on all 3 OSes.
- (.andromeda/test-plan-detail/integration-test-strategy.md:88) The refusals of `plugin install` and the link verbs are verb error documents the enumeration must name, each keeping its exit 21.
  > `plugin_install_refuses_a_widened_home` (exit 21 in both modes, the home left as found, no `diagnostics/`, Unix)
  (.andromeda/test-plan-detail/integration-test-strategy.md:89)
  > `link_verbs_against_a_stopped_instance_exit_21` (exit 21, the not-running hint last)
- (.andromeda/test-plan-detail/integration-test-strategy.md:67) The strict-modes refusals' bodies sit on two of the chunk's surfaces, so the witnesses and the enumeration must hold them as codes with a null `detail`.
  > MCP frame tools `instance-unreachable` with `detail: null` and no frame or line, CLI 21 for every other verb
  (.andromeda/test-plan-detail/integration-test-strategy.md:68)
  > a refusal is exit 21 with the fixed instance-state `unable:` / `hint:` pair (under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout, stderr empty), no line in any log and the directory's mode as found
  (.andromeda/test-plan.md:443)
  > strict-modes refusals give exit 1 for `run` / `mcp` / `viola revive` / `viola verify`, hook 0 with no output, `instance-unreachable` from the MCP frame tools, and CLI 21 for every other verb (the six frame verbs printing the instance pair)
- (.andromeda/test-plan-detail/integration-test-strategy.md:56) The MCP witness and enumeration must keep these exact `isError` bodies and refused texts, driving the transport-free handle against the test wrapper.
  > MCP `send`, `wait`, `last` and `answer` against a server that is not the recorded wrapper each get `isError` with `{"error":"instance-unreachable","detail":null}` and text `refused: instance-unreachable`; no frame reaches the server and no log line is written.
  (.andromeda/test-plan-detail/integration-test-strategy.md:57)
  > MCP `send`, `wait`, `last` and `answer` each get `isError` with `{"error":"instance-unreachable","detail":null}` before any snapshot read or connection; no frame reaches the wrapper, no line is written and the widening is left as found.
  (.andromeda/test-plan.md:456)
  > adapter-level handler tests in `crates/viola-mcp/tests/handlers.rs` drive the transport-free `viola_mcp::Tools` handle (the same `Server::call` `serve` answers with) against a `viola_channel::Server` test wrapper (`TestWrapper`, its snapshot written through `viola_state::snapshot::write_snapshot`); the handlers under test use the Tokio client (`viola_channel::tokio_client::TokioClient`, `viola-channel`'s `tokio` feature), never the sync client (chunk 2026-10-17-mcp-server-for-drivers). The full stdio path is E2E (§6).
- (.andromeda/test-plan-detail/integration-test-strategy.md:51) Every channel error the chunk fixes, `-32603` among them, must be schema-validated in these suites, and a forced handler fault must come only from the seams the plan allows.
  > result, refusal and error shapes schema-validated
  (.andromeda/test-plan-detail/integration-test-strategy.md:50)
  > `send`/`wait`/`last`/`answer`/`pause`/`release`/`link`/`unlink`/`hook.dialog`/`hook.event` via `crates/viola-channel/tests/` and root `tests/channel_*.rs`
  (.andromeda/test-plan.md:441)
  > every method through the crate and root channel suites, with schema-validated result, refusal and error shapes, the `cursor` equal to the log length at acceptance, `-32602` on a higher `v`, `-32600` and a close on an oversize frame
  (.andromeda/test-plan.md:920)
  > NEVER introduce a failpoint crate that pulls tokio into the sync crates (for example fail-parallel). Fault seams go through mockall doubles of existing traits plus process and file faults.
- (.andromeda/test-plan-detail/integration-test-strategy.md:18) The channel surface's witness must drive a real endpoint and validate each error frame by jsonschema, its endpoint taken only from `snapshot.json`.
  > Wrapper channel: real endpoint bound by `viola-channel` server in a temp home, with frames validated by jsonschema.
  (.andromeda/test-plan.md:412)
  > the `viola-channel` sync client, a raw interprocess 2.4.4 `local_socket::Stream` for malformed frames, and jsonschema 0.57.0 validators built from the product's schemars 1.2.2 schemas.
  (.andromeda/test-plan.md:427)
  > a real endpoint bound in a temp home with frames validated by jsonschema
  (.andromeda/test-plan.md:895)
  > NEVER use interprocess `try_overwrite`, or reuse an endpoint name across tests. Take the endpoint only from `snapshot.json`.
- (.andromeda/test-plan-detail/integration-test-strategy.md:53) The `-32602` carry tells this cause apart from `InvalidParams` and `release-from-driver`, and a higher `v` must still give `-32602` with `viola.compat.v_rejected` counting it.
  > higher `params.v` gives -32602
- (.andromeda/test-plan-detail/integration-test-strategy.md:54) The channel witness's `-32600` body must keep this reply-and-close oracle across the three OSes.
  > a frame over `MAX_FRAME` gives -32600 and the socket closes. After the close, Windows reads 0 bytes, Linux may see ECONNRESET, and on macOS the sender's tail write may get EPIPE (as measured in CI run 36313377307) or ENOTCONN (os error 57, as measured in CI run 36482322449), whichever state the close reached first; the test accepts EPIPE, ENOTCONN or a reset on the write and still requires the -32600 reply and the close
- (.andromeda/test-plan-detail/integration-test-strategy.md:59) This case crosses two of the chunk's surfaces, so its CLI error document and its `-32600` must both hold the sanitised form.
  > oversize `answer` (Vector 6): `viola answer` stdin over `MAX_FRAME` exits non-zero with one `--json` error document before connecting. Through the raw client it gets -32600 and the connection closes. In both cases the dialog stays pending (`dialog_pending:true`) and the hook stdout stays empty.
- (.andromeda/test-plan-detail/integration-test-strategy.md:58) The wrapper's refusal bodies must stay in the enumeration with their `detail` a code, so that the MCP and CLI forms of the same refusal stay equal.
  > Each gets `{"refusal":"not-delivered","detail":"control-character"}` from the wrapper, no `send-issued` record is written, and the receipt gains no prompt.
  (.andromeda/test-plan-detail/integration-test-strategy.md:58)
  > the raw client gets `{"refusal":"not-delivered","detail":"empty-text"}` from the wrapper
- (.andromeda/test-plan-detail/integration-test-strategy.md:62) The `-32602` carry's change to the `channel-response` line must keep this closed-list rule, and any added cause must be a closed value, never a client-supplied name.
  > an unserved method's conn-less response line carries no `method` key, because the channel's `method_label` names only the closed method list: `channel_wrapper_logs_each_call_with_corr_and_conn` (`tests/channel_endpoint.rs`) sends the unserved `anything` and asserts the key absent; a test asserting a `method` value on such a line for a name outside that list is wrong by construction
- (.andromeda/test-plan-detail/integration-test-strategy.md:60) The thresholds, dialog hold and `claude_env_keep` rows each owe an oversize-file case, and each case must read its fallback and its rejection line against these landed behaviours rather than a refusal to start.
  > out-of-range `Percent` in `config.json` (Vector 6, a case in `tests/cli_controls_not_disableable.rs`): `five_hour` / `seven_day` thresholds of `101`, `-1` and `"90"`. As landed (chunk 2026-10-10-budget-governor) the product falls back to that key's default and never refuses to start: `setting_does_not_disable_the_budget_gate`, its cases the three `five_hour` values and `seven_day_over_one_hundred` and `v_two` (chunk 2026-10-17-cli-machine-contract), each asserting exit 11 on the `send` after `five_hour` 91 is fed through `viola hook statusline` and exactly one `parse-rejected{parser:"config-json", detail:"malformed"}` line. The wider value table (`101`, `-1`, `"90"`, `90.5`, `256`, `null`, `true`) is the unit case `thresholds_a_value_that_is_no_whole_percentage_takes_its_default`; two bad keys give two lines.
  (.andromeda/test-plan-detail/integration-test-strategy.md:94)
  > chunk 2026-10-19-dialog-hold-for-the-driver added the `config.json` key `dialog_hold_s` as the row `setting_does_not_unbound_the_dialog_hold` (booted on a stamped home, five cases — `0`, `3601`, `-1`, `"3600"`, `false` — each giving exactly one `parse-rejected{parser:"config-json", detail:"malformed"}` line in `run-builder.ndjson`; the value each falls back to is the `src/run/dialog.rs` unit `read_hold_falls_back_to_sixty_with_one_rejection`)
  (.andromeda/test-plan-detail/integration-test-strategy.md:94)
  > `config_key_does_not_disable_the_identity_strip` (`diagnostics_level` / `claude_env_keep`)
- (.andromeda/test-plan-detail/integration-test-strategy.md:15) The child pipe drain row's witness must run oversize through `viola-fake-agent` as `claude` on a test-scoped `PATH`, and the `Refusal` carry must keep the shim and `.cmd` behaviour whichever way its question is answered.
  > The only external boundary is child processes. `claude agents --json` is served by `viola-fake-agent` copied to a test-scoped `PATH` as `claude`, in recorded, oversize and malformed modes. The Windows npm shim is a fixture `.cmd` shim next to a stub `claude.exe`.
  (.andromeda/test-plan.md:425)
  > no outbound HTTP exists; the only external boundary is child processes, served by `viola-fake-agent` as `claude` on a test-scoped `PATH` and by a fixture npm shim.
  (.andromeda/test-plan.md:429)
  > real ConPTY / openpty spawns of `viola-fake-agent` through `viola-pty`: exit detected on the handle, a resize propagated, a key written after a resize only once the child reports the new size, a `.cmd` child refused
- (.andromeda/test-plan-detail/integration-test-strategy.md:25) The status-line row's cap-driving witness must be an exec case of `viola hook statusline` fed its stdin, with its verdict read from this boundary's outputs.
  > `viola hook <event>` fed fixture stdin with `VIOLA_NAME`/`VIOLA_DIR` against a live test wrapper. The event line, the `diagnostics/hook-<name>.ndjson` trace and `budget.json` are asserted.
  (.andromeda/test-plan.md:431)
  > `viola hook <event>` fed fixture stdin against a live test wrapper, its event line, hook trace and `budget.json` asserted.

## Bindings
- **Integration error documents ↔ E2E error sanitization**: the CLI, MCP and channel bodies pinned here (.andromeda/test-plan-detail/integration-test-strategy.md:78) (.andromeda/test-plan-detail/integration-test-strategy.md:51) must also pass the after-test sweep for paths, serde paths and `Caused by` (.andromeda/test-plan.md:605).
- **MCP handler bodies ↔ cross-surface parity**: the MCP refusal bodies (.andromeda/test-plan-detail/integration-test-strategy.md:56) and the wrapper's refusals (.andromeda/test-plan-detail/integration-test-strategy.md:58) must equal the CLI `--json` form of the same refusal (.andromeda/test-plan.md:616).
- **Channel error mapping ↔ unit layer**: the `-32602` and `-32600` integration cases (.andromeda/test-plan-detail/integration-test-strategy.md:53) (.andromeda/test-plan-detail/integration-test-strategy.md:54) stand beside the JSON-RPC error mapping unit cases (.andromeda/test-plan.md:380), and each case sits at the lowest layer that can hold it (.andromeda/test-plan.md:874).
- **`channel-response` line ↔ obs-plan**: a line changed by the `-32602` carry (.andromeda/test-plan-detail/integration-test-strategy.md:62) must be read against `viola.compat.v_rejected` (.andromeda/obs-plan-detail/metric-coverage.md:28) and validate under schema conformance (.andromeda/test-plan.md:607).
- **`config.json` and child-drain witnesses ↔ the owed Contract rows**: the fallbacks (.andromeda/test-plan-detail/integration-test-strategy.md:60) (.andromeda/test-plan-detail/integration-test-strategy.md:94) and the oversize fake agent (.andromeda/test-plan-detail/integration-test-strategy.md:15) are what the five rows' cap-driving witnesses read (.andromeda/test-plan.md:623).
- **Fake agent and status line ↔ test data**: the `claude` seed (.andromeda/test-plan.md:645) and the test-written status line source (.andromeda/test-plan.md:643) feed the witnesses (.andromeda/test-plan-detail/integration-test-strategy.md:15) (.andromeda/test-plan-detail/integration-test-strategy.md:25).
