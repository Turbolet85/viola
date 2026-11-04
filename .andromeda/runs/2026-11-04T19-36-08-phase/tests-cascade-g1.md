@carried n=1 label=test-plan/test-scope-summary fact=fct-jnkzde5v
why: The server-identity negatives must stay compiled out on macOS as the named carve-out, with no Unix test reading the macOS check as a pid or start-time comparison.
quote: the pid/start-time server verification is asserted on Linux and Windows: the eight CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) by `tests/security_negatives_server_identity.rs` (an impostor server gets no frame and the verb exits 21, `detail` `server-verify-failed` under `--json`, beside a control that reaches the recorded wrapper; §6 Security control negatives), the hook by `tests/hook_fail_open.rs`'s impostor rows (exit 0, no body, no frame), the MCP frame tools by `crates/viola-mcp/tests/handlers.rs` (`instance-unreachable` with `detail` `server-verify-failed`, no frame), and the decision table by the `viola-state` unit `server_verdict_rows`. These negatives compile out on macOS, where the client reads no peer pid, a named carve-out owned by no route entry; there `server-verify-failed` means an euid mismatch alone, its decision and its real read held by `peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`, `client_server_euid_is_this_user` and `tokio_client_server_euid_is_this_user`.
@carried n=1 run=2 label=test-plan/test-scope-summary fact=fct-jnkzde5v
quote: each client seam's euid wiring (`src/cmd/client.rs` `connect`, `src/cmd/hook.rs` `connect`, `crates/viola-mcp/src/tools.rs` `frame`) has no committed witness, since one account can make no foreign peer and no seam may be added; it is read only through forced refusing decisions, as measured at chunk 2026-11-01-unix-endpoint-and-home-hardening (`evidence/remove-the-guard.md` rows 7, 8 and 9). The decisions over injected euids and the wrapper's drop over an injected decision are committed tests (`peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`, `serve_drops_a_refused_peer_before_any_frame`).
@carried n=1 run=3 label=test-plan fact=fct-5akis2u7
quote: full workspace and fake-agent E2E on all three runners. OS-branch tests cover named pipe vs Unix socket, ConPTY vs openpty, Windows DACL / strict-modes vs Unix modes, and macOS euid + dir-only server verification.
@carried n=1 run=4 label=test-plan fact=fct-syqfxw5d
quote: macOS local runs. Security excerpt, Context: "macOS is CI-only in v1".
@carried n=2 label=test-plan/test-scope-summary fact=fct-zgqp2syf
why: Every Unix reading must run on the fake agent and recorded fixtures with no real claude in CI, and the R8 strip's Unix claude_env_keep survival is a one-OS behaviour a Unix-only test may name.
quote: parsers, mapping, normalisation, the env strip (using a stub child that dumps its env) and the version gate are testable against the fake agent and against `fixtures/claude/<cli-version>/`. The real `claude` CLI's live behaviour cannot be reached in CI (arch excerpt, CI/CD: "The real `claude` CLI runs only locally, never in CI"; `viola verify` runs in CI only against the fake agent).
@carried n=2 run=2 label=test-plan fact=fct-u3dkdc2w
quote: every other inherited `CLAUDE*` name under the R8 prefix rule, except persistent-environment names — Windows registry `Environment` value names, Unix `config.json` `claude_env_keep` — and always the 11-name identity floor measured on the Windows host; a differing Linux dev-host reading on `claude` 2.1.287 stands beside it and moves nothing, §4 viola-agent-claude)
@carried n=2 run=3 label=test-plan fact=fct-egysilpo
quote: NEVER treat a green fake-agent run as proof of real-CLI behaviour.
@carried n=3 label=test-plan/test-scope-summary fact=fct-xttglbra
why: The typed exit set must gain no code and every --json document, revive's closed error codes included, must hold the same keys and closed words on ubuntu-latest and macos-latest as on windows-2025.
quote: global `--home`, exits 0/1/2/10–14/20/21; `revive` takes `--json` on both arms (an exit-1 refusal of its strict-modes check, its preflight or the start it shares with `run` prints `{"v":1,"error":"<code>","detail":null}` on stdout, `<code>` one of `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`, `stale-heartbeat`, `batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, `socket-dir-refused`, `internal-error`;
@carried n=4 label=test-plan/test-scope-summary fact=fct-fnukifof
why: The v1-40 integration-tier witness must read the dim stale row on a pseudo-TTY stdout through the outer-PTY driver, never by inspecting the child's screen, and staleness must rest on the 1 s beat and 5 s threshold.
quote: The human TTY rendering (bold NAME, amber `DIALOG`, dim `stale`) needs a pseudo-TTY on stdout. It is driven through the same outer-PTY driver as the tui surface, not by screen inspection of the child agent.
@carried n=4 run=2 label=test-plan fact=fct-w7y4wg56
quote: (touched every 1 s): when pid + start time no longer name a running process the instance is `gone` whatever the beat's age; otherwise a beat at most 5 s old is `live`, and an older or absent one `stale`.
@carried n=5 label=test-plan/test-scope-summary fact=fct-b2j3wr36
why: The outer-PTY driver on Unix must read the forced vt100 panic case by byte search, the child's own bytes reaching the outer terminal on Unix and no viola literal on any OS.
quote: for the forced vt100 panic chaos case, a byte search of the outer terminal's drained stream: the child's own bytes (`e4 b8 ad`) arrive on Unix and none of `panic`, `parse-rejected`, `vt100`, `unable`, `hint:` appears on it on every OS (`tests/chaos_feed_panic.rs` `chaos_feed_panic_refuses_send_and_keeps_passthrough`; the Windows byte reading owed to the route entry "Windows-only live measurements"), a byte search, never a parse of the rendered child screen for content
@carried n=5 run=2 label=test-plan/test-scope-summary fact=fct-b2j3wr36
quote: PEXPECT-class outer-PTY driver. The test spawns `viola run` inside a PTY it owns, writes human keystrokes and bracketed pastes, and resizes.
@carried n=6 label=test-plan/test-scope-summary fact=fct-5qkdel63
why: The channel-response WARN process-log line must carry the same closed detail words on every OS, as the one diagnostics schema requires.
quote: `channel-response`'s `detail`, on the server's and the client's line at WARN, is closed to `unsupported-version` · `invalid-params` · `release-from-driver` for the three `-32602` faults and absent on every other line, with no name or param on it
@carried n=7 label=test-plan/test-scope-summary fact=fct-elurmfjv
why: The MCP instance-unreachable body must stay the CLI exit-21 document less v with the same closed causes on every OS, no OS-only cause word added.
quote: `isError:true` + `{"error":"instance-unreachable","detail":"<cause>"}` (`<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`, the CLI exit-21 document less `v`)
@carried n=8 label=test-plan/test-scope-summary fact=fct-67zmkx76
why: The harness run behind the re-read per-OS CI test job must confirm the endpoint gone by its OS's own check, a Unix socket connect refused or absent, and must keep real-CLI verify out of CI.
quote: Deletes the temp home and verifies the endpoint is gone: a CLI verb against the name returns exit 21, and on Unix a connect to the socket path is refused or the path is absent, on Windows the pipe no longer exists; `cleanup` removes no leftover socket file, so a force-killed wrapper's file stays until a next bind of that name takes it over.
@carried n=8 run=2 label=test-plan/test-scope-summary fact=fct-67zmkx76
quote: Local-only mode adds `viola verify` against the real CLI. It must never run in CI (arch excerpt, CI/CD).
@carried n=9 label=test-plan/test-scope-summary fact=fct-n2arkmru
why: The instance settings.json written on Unix only and the Unix socket directory refusal are one-OS behaviours whose tests must name that reason where they stand, Windows writing no settings.json.
quote: `snapshot.json` has `pinned_bin`, `pid`, `started_at`, `child_pid` (`endpoint` from "Wrapper channel" on); `plugin/` is rewritten, and on Unix the instance's `settings.json` (the statusline override: 0600, the absolute pinned path, only for a pinned path of plain characters); on Windows no `settings.json` is written.
@carried n=9 run=2 label=test-plan/test-scope-summary fact=fct-n2arkmru
quote: A refused socket directory (at 0755, or a symlink to 0700) exits 1 on `viola run` and on `revive`'s start, with no bind and no child, left as found, and one `process-exit` line of `detail` `socket-dir-refused` (`tests/security_negatives_socket_dir.rs` `security_negative_a_start_refuses_the_socket_directory_before_any_bind`; `tests/cross_exit_causes.rs` `a_loosened_socket_directory_on_a_start_is_socket_dir_refused`); a `Server::bind` failure is no socket-directory refusal.
@carried n=10 label=test-plan/test-scope-summary fact=fct-3oyxjfyz
why: The CARRY's closed non-editing list rules which bytes count as a human editing key, so a terminal reply joins it only when measured, and the human is never blocked.
quote: A human editing key in the `run` terminal since the last turn boundary moves the wheel to the human.
@carried n=10 run=2 label=test-plan/test-scope-summary fact=fct-3oyxjfyz
quote: The human is never blocked: typing still reaches the fake agent.
@carried n=11 label=test-plan/test-scope-summary fact=fct-qobe5v26
why: The statusline pass-through must be asserted on Unix only, the Windows arm running no command, as a named one-OS behaviour.
quote: passes the user's statusline stdout through unchanged (on Unix only: on Windows `run` writes no `settings.json` override and the arm runs no command)
@carried n=11 run=2 label=test-plan fact=fct-paewrdc5
quote: (the pass-through on Unix only; on Windows the arm runs no command)
@carried n=12 label=test-plan/test-scope-summary fact=fct-yxyv2kzp
why: Under openpty the wrapper must read child exit on the process handle and never on reader EOF, and the stale-heartbeat distinction the v1-40 witness rests on must hold on Unix.
quote: child exits without EOF (ConPTY): exit detected on the handle
@carried n=12 run=2 label=test-plan/test-scope-summary fact=fct-yxyv2kzp
quote: stale-heartbeat / live-pid distinction
@carried n=12 run=3 label=test-plan fact=fct-njspb5nv
quote: the seam over portable-pty `=0.8.1` (spawn, read, write, resize, wait, kill), exit detected on the process handle and never on EOF, plus the windows-sys `TerminateProcess` fallback
@carried n=13 label=test-plan fact=fct-lktzdsoq
why: Every macOS and Linux parity reading must come from its own CI runner, never from a Linux-only local run.
quote: NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
@carried n=14 label=test-plan fact=fct-45jodiii
why: The v1-40 witness blocks on the product's heartbeat staleness, so its verdict must be the product's own status field with the same deadline clock-driven in a unit case.
quote: E2E may block on a deadline inside the product that it cannot inject: the send confirmation window behind Path 2's `no-prompt-submitted`, `wait --timeout-ms` in Path 3, and heartbeat staleness in the chaos SIGSTOP case. Two conditions apply: the verdict is the product's own exit code or status field, and the same deadline is clock-driven in a §4 unit case.
@carried n=15 label=test-plan fact=fct-r52fahdc
why: Tests moved onto Unix runners change what each Unix coverage job counts, and the per-OS gate must be met without lowering a threshold or widening the ignore regex.
quote: NEVER lower coverage threshold to pass build (raise quality, not bar)
@carried n=15 run=2 label=test-plan fact=fct-b4avhaux
quote: NEVER widen the coverage `--ignore-filename-regex` (`viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\]`) to pass a gate.
@binding n=1
**scope-entities (viola-channel, macOS carve-out) ↔ triggers (multi-os-compat) ↔ the body's Linux-only anti-pattern**: the macOS dir-only, euid-only server check must be verified on macos-latest alone and never read as a pid or start-time comparison, while the server-identity negatives keep to Linux and Windows (test-plan/test-scope-summary fct-jnkzde5v) (test-plan fct-5akis2u7) (test-plan fct-lktzdsoq)
@binding n=2
**scope-surfaces (cli TTY rendering) ↔ scope-surfaces (tui outer-PTY driver) ↔ the body's heartbeat and real-time rules**: the v1-40 witness must drive viola list on a pseudo-TTY through the outer-PTY driver, assert only verb-output SGR, and wait out the 5 s threshold over the 1 s beat with the product's status field as verdict (test-plan/test-scope-summary fct-fnukifof) (test-plan/test-scope-summary fct-b2j3wr36) (test-plan fct-w7y4wg56) (test-plan fct-45jodiii)
@binding n=3
**scope-entities (CLI exit-code contract) ↔ scope-surfaces (MCP, wrapper channel)**: one diagnostics schema on every OS means the CLI --json documents, the MCP isError bodies that mirror them less v, and the channel-response WARN line carry the same closed words on ubuntu-latest and macos-latest as on windows-2025, with no new exit code (test-plan/test-scope-summary fct-xttglbra) (test-plan/test-scope-summary fct-elurmfjv) (test-plan/test-scope-summary fct-5qkdel63)
@binding n=4
**paths (Path 1, Path 6) ↔ scope-entities (hook statusline, R8 strip)**: the Unix-only settings.json, statusline pass-through and claude_env_keep survival are the named reasons a test may stay one-OS, and the Windows arm that writes no settings.json and runs no command must not move (test-plan/test-scope-summary fct-n2arkmru) (test-plan/test-scope-summary fct-qobe5v26) (test-plan fct-paewrdc5) (test-plan fct-u3dkdc2w)
@binding n=5
**triggers (chaos, child exit without EOF) ↔ scope-entities (viola-pty seam) ↔ scope-surfaces (tui driver)**: under openpty the six-method seam must detect exit on the process handle, never reader EOF, and the outer-PTY driver must drain its master on Unix as on Windows (test-plan/test-scope-summary fct-yxyv2kzp) (test-plan fct-njspb5nv) (test-plan/test-scope-summary fct-b2j3wr36)
@binding n=6
**scope-entities (viola-agent-claude) ↔ strategy (harness run) ↔ the body's fake-agent anti-pattern**: every Unix reading must run on the fake agent with real-CLI verify kept local-only, and no green fake-agent run on Unix may be claimed as proof of real-CLI behaviour (test-plan/test-scope-summary fct-zgqp2syf) (test-plan/test-scope-summary fct-67zmkx76) (test-plan fct-egysilpo)
