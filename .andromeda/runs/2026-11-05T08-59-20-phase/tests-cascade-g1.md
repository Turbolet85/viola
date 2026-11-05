@carried n=1 label=test-plan/test-scope-summary fact=fct-jnkzde5v
why: The Windows listener admission tests and the macOS server-verification carve-out must leave with their gates, while the same-user channel tests stay on Linux and each client seam's euid wiring stays held by no committed test, with no seam added.
quote: The Windows listener's admission is asserted on CI `test (windows-2025)` (`tests/security_negatives_channel.rs`): rejecting remote pipe clients, by the pipe-type read-back beside an accept-remote and a reject-remote control, and denying access through the Windows pipe DACL
@carried n=1 run=2 label=test-plan/test-scope-summary fact=fct-jnkzde5v
quote: These negatives compile out on macOS, where the client reads no peer pid, a named carve-out owned by no route entry
@carried n=1 run=3 label=test-plan/test-scope-summary fact=fct-jnkzde5v
quote: each client seam's euid wiring (`src/cmd/client.rs` `connect`, `src/cmd/hook.rs` `connect`, `crates/viola-mcp/src/tools.rs` `frame`) has no committed witness, since one account can make no foreign peer and no seam may be added
@carried n=1 run=4 label=test-plan fact=fct-qg6j3l2y
quote: the Windows listener rejecting remote pipe clients and its pipe DACL denying an anonymous open are asserted on `test (windows-2025)`; only the cross-user part (a peer of another euid) cannot be driven on hosted runners and is asserted at the peer-credential check boundary.
@carried n=1 run=5 label=test-plan fact=fct-syqfxw5d
quote: Cross-OS-user attacks (a second local user squatting or connecting to the endpoint, reading the home). Hosted runners provide one account (security excerpt, Vectors 1 and 7), and no seam may be added
@carried n=1 run=6 label=test-plan fact=fct-5akis2u7
quote: full workspace and fake-agent E2E on all three runners. OS-branch tests cover named pipe vs Unix socket, ConPTY vs openpty, Windows DACL / strict-modes vs Unix modes, and macOS euid + dir-only server verification.
@carried n=2 label=test-plan/test-scope-summary fact=fct-zgqp2syf
why: Live sessions are 0 and every reworked CI job must keep running only the fake agent, never the real claude CLI; the claude.exe npm-shim and the R8 strip's Windows names are OS text the plan must list for P4.
quote: The real `claude` CLI's live behaviour cannot be reached in CI (arch excerpt, CI/CD: "The real `claude` CLI runs only locally, never in CI"; `viola verify` runs in CI only against the fake agent).
@carried n=2 run=2 label=test-plan/test-scope-summary fact=fct-zgqp2syf
quote: npm-shim → `claude.exe` resolution
@carried n=2 run=3 label=test-plan fact=fct-u3dkdc2w
quote: except persistent-environment names — Windows registry `Environment` value names, Unix `config.json` `claude_env_keep` — and always the 11-name identity floor measured on the Windows host
@carried n=2 run=4 label=test-plan fact=fct-lbxoplwk
quote: NEVER run the real `claude` CLI or `viola verify` against the real CLI in CI.
@carried n=3 label=test-plan/test-scope-summary fact=fct-xttglbra
why: The typed exit set and revive's closed refusal codes must stay as they are on Linux, and `batch-script-child` names the .cmd/.bat refusal whose fate P3 lists and P4 decides.
quote: exits 0/1/2/10–14/20/21; `revive` takes `--json` on both arms (an exit-1 refusal of its strict-modes check, its preflight or the start it shares with `run` prints `{"v":1,"error":"<code>","detail":null}` on stdout, `<code>` one of `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`, `stale-heartbeat`, `batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, `socket-dir-refused`, `internal-error`;
@carried n=4 label=test-plan/test-scope-summary fact=fct-fnukifof
why: The cli signal's exit codes and one JSON document must stay unchanged on Linux, and the stale-row witness's Unix gate goes only where its sibling leaves, the witness kept running on Linux.
quote: exit code (0, 1, 2, 10–14, 20, 21) plus one JSON document on stdout
@carried n=4 run=2 label=test-plan/test-scope-summary fact=fct-fnukifof
quote: The dim `stale` row is witnessed under the outer PTY on Unix by `tests/cross_list_rows.rs` `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal`
@carried n=5 label=test-plan/test-scope-summary fact=fct-b2j3wr36
why: The forced vt100 panic case must keep its Linux byte search, and its Windows byte reading owed to the retired route entry, like the plan's three-OS wording, is an amendment the wrap writes and the plan names as expected.
quote: the child's own bytes (`e4 b8 ad`) arrive on Unix and none of `panic`, `parse-rejected`, `vt100`, `unable`, `hint:` appears on it on every OS (`tests/chaos_feed_panic.rs` `chaos_feed_panic_refuses_send_and_keeps_passthrough`; the Windows byte reading owed to the route entry "Windows-only live measurements")
@carried n=5 run=2 label=test-plan fact=fct-phy2lvsc
quote: forced vt100 panic → `not-delivered`/`input-not-ready` with passthrough continuing, witnessed on the outer terminal: the child's own bytes reach it on Unix and no viola literal appears on it on every OS (the Windows byte reading owed to the route entry "Windows-only live measurements")
@carried n=5 run=3 label=test-plan fact=fct-cyzmyvps
quote: across 3 required OSes
@carried n=5 run=4 label=test-plan fact=fct-yjkq3rgm
quote: the creator brief requires multi-platform CI (Founder Direction 4, D3)
@carried n=5 run=5 label=test-plan fact=fct-oftlgaxh
quote: These tests run on all three CI OSes.
@carried n=6 label=test-plan/test-scope-summary fact=fct-5qkdel63
why: The named-pipe transport leaves with its Windows arm while the Linux wire contract over the Unix socket, its result, refusal and fault shapes, must stay unchanged.
quote: wrapper channel: JSON-RPC 2.0 over ndjson on a named pipe / Unix socket
@carried n=6 run=2 label=test-plan/test-scope-summary fact=fct-5qkdel63
quote: Protocol faults are `{"error":{code,message,data}}` with codes -32700 / -32600 / -32601 / -32602 (`data:{supported, wrapper}`) / -32603 (`"internal error"`, `data:null`).
@carried n=7 label=test-plan/test-scope-summary fact=fct-67zmkx76
why: The harness cleanup's Windows pipe check must leave with its gate while the Unix socket check keeps holding, the .cmd/.bat start refusal must be listed for P4, and `viola verify` against the real CLI must stay out of CI.
quote: a CLI verb against the name returns exit 21, and on Unix a connect to the socket path is refused or the path is absent, on Windows the pipe no longer exists
@carried n=7 run=2 label=test-plan/test-scope-summary fact=fct-67zmkx76
quote: A `run` refusal to start (exit 1: a `live` or `stale` name, a tampered pinned copy, or a `.cmd`/`.bat` child) is surfaced verbatim.
@carried n=7 run=3 label=test-plan/test-scope-summary fact=fct-67zmkx76
quote: Local-only mode adds `viola verify` against the real CLI. It must never run in CI (arch excerpt, CI/CD).
@carried n=7 run=4 label=test-plan fact=fct-tfd2itkl
quote: `viola verify` against the real CLI is local-only, never in CI.
@carried n=8 label=test-plan/test-scope-summary fact=fct-6upjfbpd
why: The Windows DACL clauses name strict-modes code and stubs the chunk removes, while the Unix read-rule refusal of a plain home must keep holding on Linux.
quote: the instance and home-tree checks refuse a `create_dir_all` home (Unix: its 0755 mode fails the read rule; Windows: an inherited write grant to a SID strict-modes does not trust, whichever SID holds it)
@carried n=8 run=2 label=test-plan fact=fct-i65r3xxr
quote: Windows strict-modes cases that need a foreign-owner SID or a volume without persistent ACLs (FAT/exFAT) cannot be provisioned on windows-2025 hosted runners. They get stubs at the descriptor-check layer
@carried n=8 run=3 label=test-plan fact=fct-ykziik74
quote: same Windows DACL limitation as `viola-state`.
@carried n=8 run=4 label=test-plan fact=fct-lgbstcvc
quote: a home, instance directory or `diagnostics/` another user may read → refused (Unix a group or other bit; Windows a read grant to another SID)
@carried n=9 label=test-plan/test-scope-summary fact=fct-n2arkmru
why: The ConPTY sideload step and its companion carve-out must leave the run start sequence, and the Unix settings.json write loses its gate while keeping its Linux behaviour and the remaining start order.
quote: the ConPTY sideload (Windows x64, fail-open: any failure leaves the child on the inbox ConPTY)
@carried n=9 run=2 label=test-plan/test-scope-summary fact=fct-n2arkmru
quote: `plugin/` is rewritten, and on Unix the instance's `settings.json` (the statusline override: 0600, the absolute pinned path, only for a pinned path of plain characters); on Windows no `settings.json` is written.
@carried n=9 run=3 label=test-plan fact=fct-2ls6kelo
quote: the fail-open ConPTY sideload
@carried n=9 run=4 label=test-plan fact=fct-lgbstcvc
quote: a ConPTY companion another user may write → the child on the inbox ConPTY with `sideload_fallback` `strict-modes-failed` (Windows x64)
@carried n=10 label=test-plan/test-scope-summary fact=fct-qobe5v26
why: The statusline pass-through is a cfg(unix) arm whose Windows sibling leaves, so it must lose its gate and keep passing the user's stdout through byte for byte on Linux.
quote: passes the user's statusline stdout through unchanged (on Unix only: on Windows `run` writes no `settings.json` override and the arm runs no command)
@carried n=10 run=2 label=test-plan/test-scope-summary fact=fct-qobe5v26
quote: The equality is Unix only; on Windows the arm runs no command and prints nothing.
@carried n=10 run=3 label=test-plan fact=fct-paewrdc5
quote: (the pass-through on Unix only; on Windows the arm runs no command)
@carried n=10 run=4 label=test-plan fact=fct-tcg4lsy4
quote: passes the user's statusline through unchanged on Unix
@carried n=11 label=test-plan/test-scope-summary fact=fct-yxyv2kzp
why: The no-EOF child chaos case must keep detecting exit on the handle on Linux while its ConPTY reason and viola-pty's Windows TerminateProcess fallback leave.
quote: child exits without EOF (ConPTY): exit detected on the handle
@carried n=11 run=2 label=test-plan fact=fct-njspb5nv
quote: exit detected on the process handle and never on EOF, plus the windows-sys `TerminateProcess` fallback
@carried n=11 run=3 label=test-plan fact=fct-ukay3vyw
quote: NEVER detect child exit by reading master EOF; use `child.wait()` on the handle (ConPTY).
@carried n=12 label=test-plan/test-scope-summary fact=fct-3wc25tvq
why: Verify's status-line probe is Unix-gated, so where its sibling leaves it loses its gate and the local fixture refresh must keep recording both status line variants.
quote: through Run B's probe status line (Unix) the two status line variants (`StatusLine.default-1.json`, `StatusLine.rate-limits-1.json`)
@carried n=13 label=test-plan fact=fct-yqn2bvnu
why: Whether the PowerShell shim leaves with the Windows CI steps is P4's decision, and the POSIX shim must keep forwarding the harness's exit code and stdout unchanged.
quote: `scripts/agent-run.sh` (POSIX) and `scripts/agent-run.ps1` (PowerShell) are identical thin shims. They run `cargo run -q -p viola-e2e --bin viola-harness -- <command> [flags]` and forward its exit code and stdout unchanged. All logic lives once, in Rust, so both shells have byte-identical semantics on all 3 OSes.
@carried n=14 label=test-plan fact=fct-lktzdsoq
why: With the Windows and macOS runners gone, no Windows or macOS arm may stay in the tree as unverified code: each one leaves rather than being taken as proven by a Linux run.
quote: NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
@carried n=15 label=test-plan fact=fct-b4avhaux
why: The Linux coverage gate must stay green after the removal without widening the coverage ignore regex, whose Windows separator class is no widening.
quote: NEVER widen the coverage `--ignore-filename-regex` (`viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\]`) to pass a gate. The separator class that makes it match Windows paths is not a widening, because the four trees are unchanged.
@carried n=16 label=test-plan fact=fct-l5lhrwtf
why: The test-plan's OS-target amendments are the wrap's to write, so the phase must leave the test-plan unedited and its plan must name those amendments as expected.
quote: Do not edit during phase loop runs (`/andromeda-phase` / `/andromeda-implement`).
@binding n=1
**test-scope-summary ↔ ci-integration**: removing the `windows-2025` and `macos-latest` legs (test-plan fct-shyrfs6v) must retire the Windows listener and macOS carve-out tests (test-plan/test-scope-summary fct-jnkzde5v) and the multi-os-compat OS-branch tests (test-plan fct-5akis2u7) with their gates, leaving `ubuntu-latest` running the same Linux tests unchanged.
@binding n=2
**test-scope-summary ↔ quality-gates-coverage-targets**: the unchanged Linux exit set (test-plan/test-scope-summary fct-xttglbra) must be shown by every gate green on Linux, with the per-OS coverage thresholds (test-plan fct-ryyndnld) met without widening the ignore regex (test-plan fct-b4avhaux).
@binding n=3
**test-scope-summary ↔ the plan's body**: the §1 lines that name Windows or macOS (test-plan/test-scope-summary fct-b2j3wr36, test-plan/test-scope-summary fct-n2arkmru, test-plan/test-scope-summary fct-qobe5v26) change only through the wrap's amendments, never a phase edit (test-plan fct-l5lhrwtf), and the plan must name them as expected amendments.
@binding n=4
**test-harness-contract ↔ ci-integration**: the `.ps1` shim (test-plan fct-yqn2bvnu) is run only by Windows steps, among them the dispatch-only `windows-mutants.yml` (test-plan fct-fo2372wn), so P4 must decide whether it leaves with them, and the `.sh` shim's contract must keep holding on Linux.
@binding n=5
**test-scope-summary ↔ test-anti-patterns**: the harness e2e-home free-space cases that fail the `msrv` job (test-plan fct-bpswkwdk) are the chunk's own flake fix, so they must decide on a free-space figure the test controls and be fixed at the root cause, never ignored (test-plan fct-2cc4sa5w).
