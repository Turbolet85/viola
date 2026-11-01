# tests · Test Scope Summary — category read

## Carried
- (.andromeda/test-plan.md:24) The chunk's tests must be held to the plan's rigor tier, which sets the expected breadth for its negatives.
  > **Test tier:** `Comprehensive (2)`
- (.andromeda/test-plan-detail/test-scope-summary.md:11) The peer euid refusal, on both sides of the socket, must be asserted at the peer-credential check boundary, because no hosted runner can drive a second account.
  > The cross-user part cannot be driven on hosted GitHub runners, which have no second OS account: rejecting a peer with a different euid. It has to be asserted at the peer-credential check boundary.
- (.andromeda/test-plan-detail/test-scope-summary.md:12) The macOS cases are an owed item this entry must settle, and they must settle as the scope's named carve-out: no macOS test may read the absent peer pid as a comparison.
  > These negatives compile out on macOS, where the client reads no peer pid; the macOS cases are owed to the route entry "Unix endpoint and home hardening".
- (.andromeda/test-plan-detail/test-scope-summary.md:12) Every Unix client's peer euid check before its first frame must be witnessed on the same surfaces as the server check, each with its no-frame reading: CLI frame verbs, hook and MCP frame tools.
  > the eight CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) by `tests/security_negatives_server_identity.rs` (an impostor server gets no frame and the verb exits 21, `detail` `server-verify-failed` under `--json`, beside a control that reaches the recorded wrapper; §6 Security control negatives), the hook by `tests/hook_fail_open.rs`'s impostor rows (exit 0, no body, no frame), the MCP frame tools by `crates/viola-mcp/tests/handlers.rs` (`instance-unreachable` with `detail` `server-verify-failed`, no frame), and the decision table by the `viola-state` unit `server_verdict_rows`.
- (.andromeda/test-plan-detail/test-scope-summary.md:34) The typed exit set must not grow, and a socket-directory refusal at the start `revive` shares with `run` must take its place in `revive`'s closed code list, moving that list and its tests with it.
  > exits 0/1/2/10–14/20/21; `revive` takes `--json` on both arms (an exit-1 refusal of its strict-modes check, its preflight or the start it shares with `run` prints `{"v":1,"error":"<code>","detail":null}` on stdout, `<code>` one of `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`, `stale-heartbeat`, `batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, `internal-error`;
- (.andromeda/test-plan-detail/test-scope-summary.md:37) A refusal at a client's connect must keep the frame verbs' exit-21 form and their order: the instance check ahead of any role file or log, and no line written.
  > all eight CLI frame verbs run `viola_state::strict::check_instance` on the named instance (for `link` / `unlink` the driven one) through the shared `src/cmd/client.rs` `refuse_untrusted`, before any role file or log opens: a refusal exits 21 with the instance pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, and writes no line (§5 On-disk)
- (.andromeda/test-plan-detail/test-scope-summary.md:44) Each new CLI refusal must be driven as a subprocess against a per-test home with `--json`.
  > **Driver:** subprocess + `assert_cmd` / `Command::cargo_bin("viola")`, with `--home <per-test temp>` and `--json`
- (.andromeda/test-plan-detail/test-scope-summary.md:45) Each new refusal's verdict must be its exit code and its one JSON document on stdout.
  > exit code (0, 1, 2, 10–14, 20, 21) plus one JSON document on stdout (`{"v":1,"ok":…}` / `{"v":1,"refusal":…,"detail":…}` / `{"v":1,"error":…}`).
- (.andromeda/test-plan-detail/test-scope-summary.md:69) The MCP cause list is closed, so a new refused-directory or euid cause reaching the MCP frame tools must join it by the same rule, the CLI exit-21 document less `v`.
  > `isError:true` + `{"error":"instance-unreachable","detail":"<cause>"}` (`<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`, the CLI exit-21 document less `v`)
- (.andromeda/test-plan-detail/test-scope-summary.md:72) The MCP frame tools must refuse a failed strict-modes check with no frame and no line, and the new connect checks must keep that shape.
  > the frame tools `send`, `wait`, `last` and `answer` run `check_instance` before the liveness pre-check; a refused instance returns `isError:true` `{"error":"instance-unreachable","detail":"strict-modes-failed"}` with no frame and no line (`crates/viola-mcp/tests/handlers.rs` `handlers_frame_tools_refuse_a_widened_instance_before_any_frame`).
- (.andromeda/test-plan-detail/test-scope-summary.md:86) Harness `boot` must surface a refused socket directory at a wrapper's start verbatim, as it does every other `run` refusal, if P3 reads that refusal as exit 1.
  > A `run` refusal to start (exit 1: a `live` or `stale` name, a tampered pinned copy, or a `.cmd`/`.bat` child) is surfaced verbatim.
- (.andromeda/test-plan-detail/test-scope-summary.md:101) The harness `cleanup` contract is the rule behind the scope's hypothesis on a force-killed wrapper's leftover socket file; P3 must measure it against this line.
  > Deletes the temp home and verifies the endpoint is gone: a CLI verb against the name returns exit 21, and the Unix socket file or pipe no longer exists.
- (.andromeda/test-plan-detail/test-scope-summary.md:122) A test that makes a home or directory ahead of viola must use the private maker, and only a deliberate widening may make a directory the checks refuse.
  > A test that makes a home before viola runs makes it through `viola_state::fs::create_private_dir`, never `create_dir_all`: the instance and home-tree checks refuse a `create_dir_all` home (Unix: its 0755 mode fails the read rule;
- (.andromeda/test-plan-detail/test-scope-summary.md:128) The `run` start order must hold, with the socket directory verified before the exclusive endpoint bind and everything still ahead of the child spawn; the squatted-endpoint exit must survive beside the takeover of a leftover file.
  > The home-tree strict-modes check before the wrapper log opens (`check_home_tree`: the home under the read rule, the instance check's set, then `ledger/`, `bin/`, `plugin/`, `bin/<key>/` and the pinned copy, `plugin/<key>/` and its `hooks/` and `.claude-plugin/`, each that exists), the collision check (over a replayed snapshot, liveness is the beat alone), pinned copy and plugin folder, the ConPTY sideload (Windows x64, fail-open: any failure leaves the child on the inbox ConPTY), version gate, exclusive endpoint bind, first snapshot and heartbeat, start `wheel` and `budget-gate` events, and only then the child spawn. A second `run` of the same name, or a squatted endpoint, exits 1.
- (.andromeda/test-plan-detail/test-scope-summary.md:134) The home-tree refusal at `run` must stay exit 1 before any log with no line and no child, and it is the nearest form for a refused socket directory at a wrapper's start.
  > A refused home tree (a 0770 home, a widened `instances/<name>`, `bin/` or pinned copy) exits 1 before any log opens, with the home pair (the instance pair for an instance-level refusal), no line and no child (`tests/run_cli.rs` `run_refuses_a_widened_home_tree_before_its_log`).
- (.andromeda/test-plan-detail/test-scope-summary.md:183) A killed wrapper's leftover socket file must still read `endpoint-lost` under a parked `wait`, beside the new connect checks.
  > endpoint vanishes during `wait`: `instance-unreachable`, `detail` `endpoint-lost` (`tests/cross_exit_causes.rs` `a_wrapper_killed_under_a_parked_wait_is_endpoint_lost`)
- (.andromeda/test-plan.md:88) A second-user squat or connect stays an untestable zone, so no test of this chunk may claim to drive one.
  > Cross-OS-user attacks (a second local user squatting or connecting to the endpoint, reading the home). Hosted runners provide one account (security excerpt, Vectors 1 and 7).
- (.andromeda/test-plan.md:90) macOS legs of this chunk can be proven only on the CI runner, so its macOS verdicts must come from `macos-latest`.
  > macOS local runs. Security excerpt, Context: "macOS is CI-only in v1".
- (.andromeda/test-plan.md:184) The IPC security triggers this chunk touches must all stand as negatives: the server-identity reading on three surfaces, the squatted endpoint, and the directory and socket modes asserted.
  >     - server pid / start-time mismatch vs `snapshot.json` → CLI exit 21 / MCP `instance-unreachable` / hook exit 0 with no body
  >     - squatted endpoint → `run` exit 1
  >     - Unix socket dir 0700 and file 0600 asserted
- (.andromeda/test-plan.md:193) A refusal of this chunk must reach `hook` only as a fail-open, and the statusline command must not run behind a failed home check.
  > Each must give exit 0 with empty stderr and no body. `statusline_command` must not run when the home fails strict-modes.
- (.andromeda/test-plan.md:201) The strict-modes exit split across surfaces must hold on every path the Unix strict modes reach.
  > `--home` / `VIOLA_DIR` failing strict-modes → exit 1 / 21 / hook 0
- (.andromeda/test-plan.md:209) These are the filesystem negatives the board's one home rule must satisfy: a home another user may read is refused, with or without an instance in it.
  >     - group/world-writable home or instance dir → refused
  >     - a home, instance directory or `diagnostics/` another user may read → refused (Unix a group or other bit; Windows a read grant to another SID)
- (.andromeda/test-plan.md:254) The chunk's Unix branches must run on both Unix runners, and this trigger's macOS case must be read as the scope's carve-out for the server check.
  > full workspace and fake-agent E2E on all three runners. OS-branch tests cover named pipe vs Unix socket, ConPTY vs openpty, Windows DACL / strict-modes vs Unix modes, and macOS euid + dir-only server verification.
- (.andromeda/test-plan.md:269) Every new refusal's code, detail and hint must pass the error-sanitization rule, holding no path of the refused directory.
  > assertions that CLI `--json`, MCP `isError` and channel `error.data` contain no absolute paths, serde paths or values, anyhow chains or tool `input`.
- (.andromeda/test-plan.md:380) The peer euid decision must have its unit layer with injected euids, the only layer that can hold a mismatch.
  > the peer-credential decision with injected euids, the connected stream's server pid on each OS
- (.andromeda/test-plan.md:382) The directory verification and the board's home rule must carry unit cases beside the existing strict-modes rows.
  > the pure server verdict's decision table, Unix strict-modes and Windows strict-modes (the write and the read rule) over synthetic SDDL descriptors and, on `windows-2025`, real grants, the instance check, the home-tree and diagnostics checks,
- (.andromeda/test-plan.md:393) Each new closed unreachable cause must be asserted logged inside the frame verb's and the hook's span.
  > the server check refused inside each frame verb's and the hook's span with each closed unreachable cause logged,
- (.andromeda/test-plan.md:428) Integration cases must read the 0600 socket back on a real endpoint and build any squatter through the server API.
  > the squatter created first through the server API (never `try_overwrite`), and the Unix socket modes and the Windows pipe DACL read back
- (.andromeda/test-plan.md:444) The board's home-check change and the new Unix strict modes must keep this per-surface exit map and its no-line clause.
  > strict-modes refusals give exit 1 for `run` / `mcp` / `viola revive` / `viola verify`, hook 0 with no output, `instance-unreachable` from the MCP frame tools, and CLI 21 for every other verb (the six frame verbs printing the instance pair); a refused check at `run`, `viola revive`, `viola verify`, the frame verbs, the hook and the MCP frame tools writes no line.
- (.andromeda/test-plan.md:484) A foreign-euid peer must be covered by the boundary unit test, never skipped as untested.
  > Cross-OS-user peers. Covered by the peer-credential boundary unit test (§4) and same-user SDDL read-back (§5).
- (.andromeda/test-plan.md:604) The hook's fail-open matrix must take a row for each client refusal this chunk adds, at exit 0 with no body.
  > an instance the instance check refuses or a `VIOLA_DIR` naming no instance directory and, on Linux and Windows, a server that is not the recorded wrapper, each exit 0 with no body
- (.andromeda/test-plan.md:610) The socket-directory and peer-euid refusals must each be a matrix row with a hint naming its own cause, since the scope gives each a row of `tests/cross_exit_causes.rs`.
  > one negative test per cause of exit 21 and of exit 1 it holds, each asserting the exit code, the `--json` document and a last `hint:` line that names its cause — a hint shared across causes fails the table (`no_two_codes_share_a_hint`, its control `shared_hints_reads_a_planted_duplicate`)
- (.andromeda/test-plan.md:610) The owed macOS server-verification case must be discharged here, and this line must move with that discharge as the named carve-out.
  > the server-verification cause is a case of `tests/security_negatives_server_identity.rs` on Linux and Windows (its macOS case owed to the route entry "Unix endpoint and home hardening")
- (.andromeda/test-plan.md:610) No hint this chunk adds may name `release`.
  > driver-facing hints never contain `release` (`no_hint_names_release`)
- (.andromeda/test-plan.md:612) The Unix socket directory refusal must have its security-control negative.
  > one negative test per security-plan control that had none — the Unix socket directory refusal,
- (.andromeda/test-plan.md:612) The board's one home rule must keep its whole-board refusal at exit 21 opening no log.
  > `viola list`'s whole-board refusal exit 21 opening no log and `viola plugin install`'s exit 21 creating nothing (`tests/cross_exit_causes.rs` `strict_modes_on_the_board_is_exit_21_and_opens_no_log`, `strict_modes_on_plugin_install_is_exit_21_and_creates_nothing`)
- (.andromeda/test-plan.md:618) Any new cause of exit 21 must hold CLI and MCP parity.
  > for each cause of exit 21, the MCP `isError` body `{error, detail}` equals the CLI `--json` document less `v`, its text `refused: instance-unreachable`
- (.andromeda/test-plan.md:821) New `#[cfg(unix)]` code must clear the coverage floor on each Unix job, so each OS needs its own tests of it.
  > Gates are enforced per OS job with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`. Per-OS gating is deliberate: `#[cfg(windows)]` / `#[cfg(unix)]` code compiles only on its own runner.
- (.andromeda/test-plan.md:881) The dev host's live Linux run must not stand as proof of a macOS leg; the macOS job must read green before that leg is claimed.
  > NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.

## Bindings
- **Owed macOS cases ↔ the body's exit-cause suite and the scope's carve-out**: the detail's owed macOS cases and the suite's owed macOS case name this entry, and both must close as the recorded gap the scope requires, no test reading it as a comparison, (.andromeda/test-plan-detail/test-scope-summary.md:12) and (.andromeda/test-plan.md:610).
- **Macos trigger ↔ §9 CI matrix**: the multi-os-compat trigger's macOS euid and dir-only server verification is the same reading the §9 `macos-latest` row names, and both read with the macOS gap, (.andromeda/test-plan.md:254).
- **Socket modes trigger ↔ security plan Vector 1**: the required 0700 directory and 0600 socket negatives are sourced from the security excerpt's Vector 1, so the code and hint of the refusal bind to both plans, (.andromeda/test-plan.md:186).
- **Filesystem negatives ↔ security plan Vector 7**: the board's one home rule answers the filesystem trigger sourced from the security excerpt's Vector 7, (.andromeda/test-plan.md:209).
- **`cleanup` requirement ↔ §3 harness contract**: the socket-file clause of the 5-command `cleanup` is the contract `crates/viola-e2e/src/harness/cleanup.rs` is measured against in the scope's hypothesis, and any change moves both §1 and the §3 key, (.andromeda/test-plan-detail/test-scope-summary.md:101).
- **CLI exit document ↔ MCP cause list**: a new closed cause must land on both surfaces together, the MCP list being the CLI's exit-21 document less `v`, (.andromeda/test-plan-detail/test-scope-summary.md:69) and (.andromeda/test-plan.md:618).
- **Exit-cause matrix ↔ the exit-cause catalogue freight**: each refusal's closed code and hint land as a matrix row, the socket-directory leg standing under `v1-18`, (.andromeda/test-plan.md:610).
