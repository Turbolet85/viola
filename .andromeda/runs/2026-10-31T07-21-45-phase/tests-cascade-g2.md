# tests · cascade read · group 2 of 7

## Carried
- (.andromeda/test-plan-detail/test-strategy.md:73) The chunk's proof must answer this trigger on both layers it names, its witnesses at integration and E2E and the scan after them, since making these bodies clean is the whole obligation.
  > | error sanitization / secret logging | integration + E2E | §6 Error sanitization and secret scan; §3 logs |
- (.andromeda/test-plan-detail/test-strategy.md:14) The witness per surface must drive the CLI, MCP and channel contracts over a real endpoint, a real home and the real binary, never a double of the surface whose body it reads.
  > Required. Every arch Standard Contract (Wrapper channel, On-disk, Hooks, CLI, MCP) has a crate-level `tests/` suite, using a real `interprocess` endpoint, a real tempfile home, and assert_cmd against `CARGO_BIN_EXE_viola`.
  (.andromeda/test-plan.md:291)
  > **Integration:** required — every arch Standard Contract has a crate-level `tests/` suite over a real `interprocess` endpoint, a real tempfile home and `assert_cmd`.
- (.andromeda/test-plan-detail/test-strategy.md:17) The MCP stdio witness must go through the rmcp client and the planted-value leg through the fake-agent wrapper of scenario E2, and the Path 1 watch this chunk tallies is one of these required paths.
  > Required for all 7 critical paths plus 4 expansion scenarios (Section 6). They run against harness-booted `viola run … -- viola-fake-agent`, using the outer PTY (portable-pty) and the rmcp client.
  (.andromeda/test-plan.md:293)
  > **E2E:** required for all seven critical paths and the expansion scenarios, against harness-booted wrappers over the fake agent, through the outer PTY and the rmcp client.
- (.andromeda/test-plan-detail/test-strategy.md:10) A CLI `--json` document that changes must keep the exit code this table pins for it, the unknown refusal still at exit 14, so the sanitised documents must be read against these exit-code cases.
  > rstest 0.27.0 `#[case]` tables cover the refusal-order matrices, the exit-code table (0/1/2/10–14/20/21) and the per-`<cli-version>` fixture replay.
  (.andromeda/test-plan.md:289)
  > with rstest `#[case]` tables for the refusal-order, exit-code and fixture-replay matrices
- (.andromeda/test-plan-detail/test-strategy.md:26) Every failure a witness forces, the `-32603` handler fault first, must come from a process or file fault or an existing mockall seam, and the chaos suite's row must keep holding the `.cmd` refusal the `Refusal` carry leaves unchanged.
  > Faults come from sysinfo 0.39.6 / `std` process and file operations (`File::set_len` truncation of a stopped wrapper's log, the torn append as landed in place of a kill mid-write; snapshot corruption, SIGSTOP heartbeat freeze, a no-EOF child), plus mockall 0.15.0 faults on the `viola-pty` seam.
  (.andromeda/test-plan-detail/test-strategy.md:67)
  > | chaos-test | chaos | §6 Chaos suite |
  (.andromeda/test-plan.md:299)
  > **Chaos / Fault:** required — faults come from process and file operations and from mockall faults on the `viola-pty` seam.
- (.andromeda/test-plan-detail/test-strategy.md:29) A witness that starts a product process to hold a cap, the pre-`initialize` `viola mcp` line above all, must be bounded so a neutralised guard fails it rather than hangs it, and the chunk owes the remove-the-guard rule and no mutation gate.
  > Beside it stands the per-chunk remove-the-guard rule (`.claude/rules/testing.md`): a control whose neutralised guard lets a product process start needs a bounded witness; the G4 / G10 witnesses spawn through `tests/support/cli.rs` `Running`, bounded by `WITHIN`, since nextest's default profile has no kill line (an unbounded witness hung 27 minutes, as measured at `viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md`).
  (.andromeda/test-plan-detail/test-strategy.md:29)
  > No per-chunk, pre-push or CI mutation gate.
  (.andromeda/test-plan.md:301)
  > there is no per-chunk, pre-push or CI mutation gate
- (.andromeda/test-plan-detail/test-strategy.md:33) A changed error body that meets a recorded MCP exchange must leave `Mcp.*.json` valid against its schema, the recording changing only through its own recorder.
  > every `fixtures/claude/*/Mcp.*.json` MCP exchange to `schemas/claude-mcp-fixture.v1.json`
  (.andromeda/test-plan.md:303)
  > every fixture is checked against its schema.
- (.andromeda/test-plan-detail/test-strategy.md:72) The five owed cap-driving cases must land in the §6 Contract suite that names this chunk their owner, and the recorded MCP exchange stays under the §7 fixtures.
  > | contract-test-against-sandbox | contract | §6 Contract suite; §7 |
- (.andromeda/test-plan-detail/test-strategy.md:58) The channel witness must sit at these layers and read the raw-frame `-32602` cases named here among the causes the `channel-response` line tells apart, each staying `-32602` with no line written.
  > | IPC vector (V1) | integration + E2E | §5 Module ↔ IPC + Wrapper channel (the MCP impostor case, `crates/viola-mcp/tests/handlers.rs`); §6 Scenario 1 + Security control negatives (root `security_negatives_channel.rs`: the Windows listener's remote-client rejection and DACL denial; root `security_negatives_server_identity.rs`, off macOS: a CLI frame verb against a server that is not the recorded wrapper; root `channel_links.rs`: raw `link` / `unlink` frames with a bad name or pair, each `-32602` with no line written)
- (.andromeda/test-plan-detail/test-strategy.md:60) The status-line row's cap-driving witness must drive `viola hook statusline` as an assert_cmd child at this layer.
  > | Hook stdin (V4) | integration (assert_cmd hook) | §5 hook boundary; §6 Security sweep |
- (.andromeda/test-plan-detail/test-strategy.md:61) The MCP surface's witness must run through the rmcp client for the stdio path, and the §6 places named here must keep their `isError` assertions true over the sanitised bodies.
  > | MCP (V5) | E2E (rmcp client) | §6 Scenarios 2, 5, 7 + Security sweep |
- (.andromeda/test-plan-detail/test-strategy.md:62) The CLI `--json` witness and the oversize `config.json` cases must sit at these two layers, the exit mapping in the root bin's unit cases and the documents in the §5 CLI suites.
  > | CLI and env (V6) | unit + integration | §4 viola-core/root bin; §5 |
- (.andromeda/test-plan-detail/test-strategy.md:63) The enumeration of CLI error bodies must name the `instance-unreachable` document this check witnesses, with the control itself left to its own route entry, and the `config.json` rows' oversize files sit at this disk boundary.
  > | Filesystem (V7) | integration + chaos | §5 Module ↔ disk; §6 Scenario 1, E5 (root `chaos_snapshot_replay.rs`: steps 1–3) + Security control negatives (root `security_negatives_strict_modes.rs`: each of the six frame verbs `send` · `wait` · `last` · `answer` · `pause` · `release` refusing a widened instance before any log, exit 21 with the instance pair or the `instance-unreachable` document, no frame and no line
- (.andromeda/test-plan-detail/test-strategy.md:64) The planted stripped value must travel through scenario E2 at this layer, and the child pipe drain row's witness and the `.cmd` / `.bat` and npm-shim behaviour the `Refusal` carry keeps sit at the §5 process boundary.
  > | Child spawn (V8) | integration + E2E | §5 process boundary; §6 Scenario E2 |
- (.andromeda/test-plan-detail/test-strategy.md:65) A test-only seam forcing the `-32603` fault must stay behind the `fake-agent` feature and fail this release stage if it leaks, and no failpoint crate may enter the graphs these gates judge.
  > | Supply chain (V9) | CI gates | §9 Lint stage (sync-crate `cargo check`, `scripts/orphans-check.sh`) + Supply-chain stage (cargo deny incl. the `fuzz/Cargo.lock` audit, sole-root tokio ban, ban probes, zizmor) + Release build stage (`scripts/release-check.sh`)
- (.andromeda/test-plan-detail/test-strategy.md:69) The chunk must keep the MCP `structured_content` `{refusal, detail}` equal to the CLI `--json` `{refusal, detail}` for the same refusal, at this layer, through the sanitised bodies.
  > | cross-surface-coordination | E2E | §6 Scenarios 2, E3 + Cross-surface parity |
- (.andromeda/test-plan-detail/test-strategy.md:70) The chunk's OS-only witnesses, the Unix status-line pass-through and `claude_env_keep` cases and the Windows `.cmd` refusal, must each be verified on their own runner of this matrix, never by the Linux dev host's run.
  > | multi-os-compat | CI matrix | §9 Matrix builds |
- (.andromeda/test-plan-detail/test-strategy.md:71) The higher-`v` `-32602` rejection and the `RefusalReason` `unknown` fallback are this trigger's cases, so telling the `-32602` causes apart and closing the CLI documents must leave them holding, and `viola-agent-claude`'s fold stays inside its §4 cases.
  > | multi-version-compat | unit + contract | §4 viola-agent-claude; §7 fixtures |

## Bindings
- **The error-sanitization row ↔ the E2E suite and its trigger**: the row's layers bind to the suite's after-every-test scan and the trigger's assertion list, (.andromeda/test-plan-detail/test-strategy.md:73) with (.andromeda/test-plan.md:605) and (.andromeda/test-plan.md:269).
- **The contract row ↔ the §6 Contract suite's owed clause**: the five cap-driving cases sit under the row and discharge the owed clause, whose rows and census must keep standing, (.andromeda/test-plan-detail/test-strategy.md:72) with (.andromeda/test-plan.md:622) and (.andromeda/test-plan.md:623).
- **The remove-the-guard rule ↔ the §4 viola-mcp clause**: the pre-`initialize` stdin case the unit clause owes this chunk must be a bounded witness under the rule, (.andromeda/test-plan-detail/test-strategy.md:29) with (.andromeda/test-plan.md:390).
- **The fault sources ↔ the §11 failpoint and replay bans**: the fault-source row and the bans state the same limit on forcing `-32603`, (.andromeda/test-plan-detail/test-strategy.md:26) with (.andromeda/test-plan.md:920) and (.andromeda/test-plan.md:955).
- **The child-spawn row ↔ scenario E2**: the planted value must be one the strip removes, the row's scenario reading no stripped name at the child and no canary in the scan, (.andromeda/test-plan-detail/test-strategy.md:64) with (.andromeda/test-plan.md:561).
- **The cross-surface row ↔ the §6 parity suite**: the row's layer holds the suite's equality of `{refusal, detail}`, (.andromeda/test-plan-detail/test-strategy.md:69) with (.andromeda/test-plan.md:616).
- **The multi-version row ↔ the `-32602` line's schema check**: a changed `channel-response` line telling the causes apart must still validate against obs-plan's schema while the higher-`v` case stays `-32602`, (.andromeda/test-plan-detail/test-strategy.md:71) with (.andromeda/test-plan.md:262) and (.andromeda/test-plan.md:607).
- **The multi-os row ↔ the §11 Linux-only ban**: the OS-only witnesses read green only on their runners, (.andromeda/test-plan-detail/test-strategy.md:70) with (.andromeda/test-plan.md:879).
