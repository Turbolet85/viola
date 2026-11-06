@carried n=1 label=test-plan/test-scope-summary fact=fct-jnkzde5v
why: freight item 8's survivor must stay recorded as this untestable zone with no seam added, and item 15 must measure the channel no-leak property this plan requires in the child.
quote: each client seam's euid wiring (`src/cmd/client.rs` `connect`, `src/cmd/hook.rs` `connect`, `crates/viola-mcp/src/tools.rs` `frame`) has no committed witness, since one account can make no foreign peer and no seam may be added
@carried n=1 run=2 label=test-plan fact=fct-upolcsa3
quote: a mutant replacing a seam's `peer_admitted` read with an always-admit survives every committed test; the committed tests hold the decision over injected euids, the real euid read on Linux and the wrapper's drop over an injected decision.
@carried n=1 run=3 label=test-plan fact=fct-3znwz3ga
quote: no inherited channel handles in the child
@carried n=2 label=test-plan/test-scope-summary fact=fct-zgqp2syf
why: the chunk's live sessions and item 11's real `claude` read must stay local route-entry measurements, never a test layer or CI step, and no fake-agent reading may stand as proof of real-CLI behaviour.
quote: The real `claude` CLI's live behaviour cannot be reached in CI (arch excerpt, CI/CD: "The real `claude` CLI runs only locally, never in CI"; `viola verify` runs in CI only against the fake agent).
@carried n=2 run=2 label=test-plan fact=fct-upolcsa3
quote: Local only: through `viola verify` for the suites, and through a route entry's own live measurement for a real session's dialogs and readings (as at chunk 2026-10-08-first-live-test-and-self-drive).
@carried n=2 run=3 label=test-plan fact=fct-lbxoplwk
quote: a route entry's own live measurement of a real session is local too and never a CI step.
@carried n=2 run=4 label=test-plan fact=fct-egysilpo
quote: NEVER treat a green fake-agent run as proof of real-CLI behaviour.
@carried n=2 run=5 label=test-plan fact=fct-u3dkdc2w
quote: always the 11-name identity floor measured on the Windows host; a differing Linux dev-host reading on `claude` 2.1.287 stands beside it and moves nothing
@carried n=3 label=test-plan/test-scope-summary fact=fct-ze6ljnvw
why: item 17's remnant removal must leave the tree at this Linux-only scope, the census still reading any windows or macos gate as a fault and its counts consistent for v1-53's claim.
quote: **OS scope:** Linux only: the core on any current distribution, CI on `ubuntu-latest`, and the window features of 0.2.0 on Hyprland; the Windows and macOS code, tests, fixtures and CI jobs are not in the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`).
@carried n=3 run=2 label=test-plan fact=fct-5akis2u7
quote: the product is Linux-only: the full workspace and fake-agent E2E run on `ubuntu-latest`, and the census `tests/contract_os_gated_tests.rs` reads a gate whose predicate names `windows` or `target_os = "macos"` as a fault (`another OS: …`), even when a row names it.
@carried n=4 label=test-plan/test-scope-summary fact=fct-fnukifof
why: item 1's masters-agree check must decide tool or text for the cli surface's named driver against the re-measured manifests, and this wording must follow that decision.
quote: **Driver:** subprocess + `assert_cmd` / `Command::cargo_bin("viola")`, with `--home <per-test temp>` and `--json`
@carried n=4 run=2 label=test-plan fact=fct-a6gfb3bv
quote: driven as a subprocess by `assert_cmd` with a per-test `--home` and `--json`
@carried n=5 label=test-plan/test-scope-summary fact=fct-b2j3wr36
why: the portable-pty release replacing `=0.8.1` must keep the tui passthrough and the `viola-pty` seam green and clear RUSTSEC-2026-0441 with no ignore beyond RUSTSEC-2017-0008, and the plan's pin and Windows ConPTY wording must follow the new pin and item 17.
quote: **Notes:** traced to arch excerpt, Stack (portable-pty, vt100) and creator excerpt, Must-Work (R2, R3/S1, Windows ConPTY).
@carried n=5 run=2 label=test-plan fact=fct-njspb5nv
quote: **Entity:** `viola-pty`: the seam over portable-pty `=0.8.1` (spawn, read, write, resize, wait, kill), exit detected on the process handle and never on EOF
    - **Source:** arch excerpt, Stack and Workspace / Modules; creator excerpt, Must-Work (Windows ConPTY EOF)
@carried n=5 run=3 label=test-plan fact=fct-hlx2zkbt
quote: CI gates on exit code: `cargo deny check` (only ignore RUSTSEC-2017-0008), `cargo check` of sync crates without tokio, zizmor on workflows, SHA-pinned Actions assertion.
@carried n=6 label=test-plan/test-scope-summary fact=fct-elurmfjv
why: item 4's dispositions of v1-13's MCP `link` and budget-release clauses and v1-36's board link set must hold the MCP tool list at exactly five tools, never adding `release` or `link`.
quote: There must be no `release` / `pause` / `link` / `unlink` tools in `tools/list`.
@carried n=6 run=2 label=test-plan fact=fct-apiugjwf
quote: `tools/list` is exactly the five tools, never `release` / `pause` / `link` / `unlink`
@carried n=7 label=test-plan/test-scope-summary fact=fct-67zmkx76
why: removing the EXE_SUFFIX joins and the `session.rs` remnant must leave `boot` building `viola` and starting its wrappers in a fresh home exactly as required.
quote: Builds `viola` and creates a fresh temp home, passed as `--home` (arch excerpt, Per-test isolation).
      - Starts one or more `viola run <name> -- <fake agent> <args>` wrappers.
@carried n=7 run=2 label=test-plan fact=fct-tfd2itkl
quote: `boot` builds `viola` and starts the wrappers in a fresh home to readiness
@carried n=8 label=test-plan/test-scope-summary fact=fct-n2arkmru
why: the founder's word on the socket-directory pair's wording (item 7) must leave the start's refusal at exit 1, no bind, no child and its `socket-dir-refused` line.
quote: A refused socket directory (at 0755, or a symlink to 0700) exits 1 on `viola run` and on `revive`'s start, with no bind and no child, left as found, and one `process-exit` line of `detail` `socket-dir-refused`
@carried n=9 label=test-plan/test-scope-summary fact=fct-3oyxjfyz
why: item 10's hypothesis that a terminal query reply loses the wheel at start, and the live driver session, read exactly this human-input flip and refusal, and a shape joins the closed list only when measured on the founder's word.
quote: A human editing key in the `run` terminal since the last turn boundary moves the wheel to the human. A driver `send` is refused with `human-typing`.
@carried n=9 run=2 label=test-plan fact=fct-nzaflfcl
quote: a human editing key since the last turn boundary moves the wheel to the human, a driver `send` is refused `human-typing`
@carried n=10 label=test-plan/test-scope-summary fact=fct-yxyv2kzp
why: the dependency move under `viola-pty` must keep a child exiting without EOF detected on the process handle, never on reader EOF.
quote: child exits without EOF: exit detected on the handle
@carried n=10 run=2 label=test-plan fact=fct-pw5tvowr
quote: a child exit without EOF detected on the handle
@carried n=10 run=3 label=test-plan fact=fct-ukay3vyw
quote: NEVER detect child exit by reading master EOF; use `child.wait()` on the handle.
@carried n=11 label=test-plan fact=fct-sqrvsbbz
why: every harness command touched by the EXE_SUFFIX and `session.rs` edits must still print one JSON document and exit 0, 1 or 2.
quote: every harness command exits 0 on success, 1 on failure and 2 on usage error. Each prints exactly one JSON document on stdout, starting `{"v":1,"cmd":"<command>","ok":<bool>,…}` (key order held by serde_json `preserve_order`).
@carried n=11 run=2 label=test-plan fact=fct-jyezwnnp
quote: NEVER let a harness command print a human-only message without the JSON document and exit code.
@carried n=12 label=test-plan fact=fct-bilmaai6
why: the Windows remnant at `crates/viola-e2e/src/harness/session.rs:52` sits in the session record, whose path and field set must hold after its removal.
quote: each harness session writes `target/agent-run/<session>/session.json` with `{v, session, home, instances:[{name, wrapper_pid, started_at, fake_args}], supervisor_pid, supervisor_started_at, agents_mode}`
@carried n=13 label=test-plan fact=fct-b4avhaux
why: rewording or removing the `COVERAGE_IGNORE` Windows-path comment and the `COVERAGE_FLOORS` per-OS comment must leave the regex over its four trees unwidened and the floors unlowered.
quote: NEVER widen the coverage `--ignore-filename-regex` (`viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\]`) to pass a gate. The separator class that makes it match Windows paths is not a widening, because the four trees are unchanged.
@carried n=13 run=2 label=test-plan fact=fct-r52fahdc
quote: NEVER lower coverage threshold to pass build (raise quality, not bar)
@binding n=1
**scope-entities OS scope ↔ triggers multi-os-compat census**: item 17's remnant removal must reach the Linux-only tree (test-plan/test-scope-summary fct-ze6ljnvw) while the census still faults any windows or macos gate and its count stays consistent with v1-53's claim (test-plan fct-5akis2u7); the plan's Windows wording (identity floor, ConPTY trace) follows (test-plan fct-u3dkdc2w) (test-plan/test-scope-summary fct-b2j3wr36).
@binding n=2
**scope-surfaces tui ↔ triggers chaos-test ↔ scope-entities viola-pty**: the portable-pty release replacing `=0.8.1` (test-plan fct-njspb5nv) must keep the outer-PTY passthrough (test-plan/test-scope-summary fct-b2j3wr36) and the no-EOF exit on the handle (test-plan/test-scope-summary fct-yxyv2kzp), and clear RUSTSEC-2026-0441 under deny's single ignore (test-plan fct-hlx2zkbt).
@binding n=3
**scope-entities viola-agent-claude ↔ body untestable zones and anti-patterns**: both live sessions and item 11's real-CLI read are local route-entry measurements (test-plan/test-scope-summary fct-zgqp2syf) (test-plan fct-upolcsa3), never a CI step (test-plan fct-lbxoplwk), and a fake-agent reading never stands as real-CLI proof (test-plan fct-egysilpo).
@binding n=4
**scope-surfaces MCP ↔ the 17 caps' dispositions**: v1-13's and v1-36's dispositions must keep `tools/list` at five tools with no `release` or `link` (test-plan/test-scope-summary fct-elurmfjv) (test-plan fct-apiugjwf); frontend-naming caps defer only on the founder's word (test-plan fct-t57awnjr).
@binding n=5
**strategy 5-command boot ↔ body harness contract**: the harness EXE_SUFFIX and `session.rs` edits must keep `boot` (test-plan/test-scope-summary fct-67zmkx76), the session record (test-plan fct-bilmaai6) and each command's one JSON document and exit code (test-plan fct-sqrvsbbz).
@binding n=6
**scope-surfaces cli driver ↔ item 1's masters-agree check**: the cli surface's `assert_cmd` driver (test-plan/test-scope-summary fct-fnukifof) (test-plan fct-a6gfb3bv) must agree with the re-measured manifests, tool or text decided where the check runs.
@binding n=7
**scope-entities viola-channel ↔ freight items 8 and 15**: the euid-wiring zone stays untestable with no seam added (test-plan/test-scope-summary fct-jnkzde5v) (test-plan fct-upolcsa3), and the Linux `/proc/<child pid>/fd` measurement witnesses no inherited channel handles in the child (test-plan fct-3znwz3ga).
