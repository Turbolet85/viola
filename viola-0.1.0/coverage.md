# Coverage — viola-0.1.0

verified 29/53 · deferred 6 · planned 18 · implemented 0 · unclaimed 18

## deferred (6)
- v1-22 · `viola ui` on an OS-assigned port
- v1-48 · `viola ui` serves a loopback, view-only, authenticated feed
- v1-49 · The strip-bay page
- v1-50 · Automated accessibility verdict on every page state
- v1-51 · Token-pair contrast, forced colours and reduced motion hold
- v1-52 · The 401 access strip after a UI restart

## planned, unclaimed (18)
- v1-01 · Three-OS CI pipeline from the first commit
- v1-03 · The fake agent and recorded fixtures as the test contract
- v1-04 · Structured JSON logs from every process, and a `logs` view
- v1-05 · Observability gates: no unlogged panic, schema-true lines, no leaked secret
- v1-07 · Quality gates: coverage, performance budgets, property/fuzz replay, zero flakes
- v1-09 · Security's eight amendments folded into the architecture
- v1-10 · CL-1: send issue and outcome are events in the log
- v1-11 · The live page sees a dialog clear and state refresh
- v1-13 · Result payloads carry what the CLI prints
- v1-21 · Timing constants named and located
- v1-24 · The statusline source `run` records
- v1-26 · Cross-plan fix-pass items honoured by the implementing chunks
- v1-27 · `viola run` wraps the unmodified CLI on Windows
- v1-34 · Capability ledger, `viola verify`, and transport-only degrade
- v1-35 · Budget governor from the statusline
- v1-36 · Links between sessions
- v1-44 · The IPC endpoint admits only the same OS user and verifies its server
- v1-53 · Linux and macOS reach parity with Windows

## verified (29)
- v1-02 · The headless harness: five agent-run commands · 2026-10-17-end-to-end-path-binary-and-harness-legs
- v1-06 · Mutation gate per chunk from chunk 1 · 2026-09-24-three-os-ci-headless-harness-skeleton
- v1-08 · Code-graph seeded at setup and built on the first real code · 2026-09-27-browser-verdict-reachability
- v1-12 · At most one send in flight per instance · 2026-10-04-confirmed-send-with-cl-1-records
- v1-14 · `viola verify` against the fake agent runs in CI · 2026-09-29-verify-stamped-test-homes-and-harness
- v1-15 · Plan revise is answered as a plan, not a permission · 2026-10-05-dialog-rows-and-re-probe
- v1-16 · Permission suggestions have a place in the answer · 2026-10-05-permission-end-to-end
- v1-17 · Diagnostics roots, sinks and initialisation order · 2026-10-26-home-and-code-bearing-file-integrity
- v1-18 · Every exit-21 and exit-1 cause has a machine-readable code · 2026-10-31-exit-cause-code-catalogue
- v1-19 · Toolchain floor 1.96 · 2026-09-24-quality-gates
- v1-20 · Obs additions to the channel and config contracts · 2026-09-27-wrapper-channel
- v1-23 · The architecture's tree includes the test and obs artifacts · 2026-09-24-workspace-tree-and-code-graph-planes
- v1-25 · The PTY dependency pin is decided on current evidence · 2026-09-25-pty-wrapper-on-windows
- v1-28 · Hooks report normalised events and fail open · 2026-09-27-hooks-to-normalised-events
- v1-29 · `send` types at a turn boundary and is confirmed after the fact · 2026-10-06-local-command-send-outcomes
- v1-30 · `wait` and `last` wake on turn boundaries without polling · 2026-10-04-wait-and-last
- v1-31 · Dialogs answered through hooks by `dialog_id` · 2026-10-08-first-live-test-and-self-drive
- v1-32 · The wheel: the human always wins, automation is refused with a reason · 2026-10-04-the-wheel
- v1-33 · The slice passes the first live test on the Linux dev host, then viola drives its own build · 2026-10-19-overseer-switch-to-viola
- v1-37 · The board: `viola list` and session liveness · 2026-10-16-unwrapped-sessions-on-the-board
- v1-38 · The MCP server for drivers · 2026-10-17-mcp-server-for-drivers
- v1-39 · CLI contract: machine output, typed exits, cause-named hints · 2026-10-31-exit-cause-code-catalogue
- v1-40 · CLI and terminal output discipline · 2026-10-17-cli-output-discipline
- v1-41 · Crash-safe, self-healing, version-tolerant state · 2026-10-26-home-and-code-bearing-file-integrity
- v1-42 · Optional plugin install for unwrapped drivers · 2026-10-18-plugin-precedence-live-readings
- v1-43 · Chunk-blocking security prerequisites · 2026-09-25-security-prerequisites
- v1-45 · The viola home and its code-bearing files keep their integrity · 2026-10-26-home-and-code-bearing-file-integrity
- v1-46 · Bounded, validated inputs at every boundary · 2026-10-27-bounded-inputs-at-every-boundary
- v1-47 · Sanitised errors and the never-log floor · 2026-10-31-sanitised-error-surfaces
