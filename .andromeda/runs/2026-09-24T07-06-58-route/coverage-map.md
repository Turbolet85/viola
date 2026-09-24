# Capability coverage map — viola-0.1.0

_Route-run artifact for the overseer's P4 coverage check (founder rule 3/4). Every capability `v1-NN` (= intent
`F-NN`) maps to the chunk expected to CLAIM it (the chunk whose acceptance fully proves it) plus any chunks that
only ADVANCE it. Advisory only: the binding `chunk` link is written at promotion by /andromeda-phase
(verification-matrix contract §chunk ownership). Arch-amendment findings (F-09…F-26) are claimed by the chunk that
implements them; its wrap reconcile amends `architecture.md`._

| Cap | Title (short) | Claiming chunk (epoch) | Advanced by |
|---|---|---|---|
| v1-01 | Three-OS CI pipeline | Hooks to normalised events (E2) — the perf job, in its own job, is born with the hooks | Three-OS CI and headless harness; Supply-chain and workflow gates; Observability gates; Quality gates |
| v1-02 | Headless harness, five commands | Three-OS CI and headless harness (E1) | — |
| v1-03 | Fake agent + fixtures contract | Fake-agent drift contract (E2) | Three-OS CI and headless harness; Fake agent and test-data fixtures |
| v1-04 | JSON logs + `logs` view | Diagnostics plane (E1) | Three-OS CI and headless harness |
| v1-05 | Observability gates | Observability gates (E1) | — |
| v1-06 | Mutation gate from chunk 1 | Three-OS CI and headless harness (E1) | — |
| v1-07 | Quality gates | Hooks to normalised events (E2) — hook deadlines on the worst sample | Quality gates (E1) |
| v1-08 | Code-graph seeded + planes | Workspace tree and code-graph planes (E1) | (setup-project seeds the files) |
| v1-09 | Security's eight arch amendments | viola ui loopback server (E8) — lands (1)(5), the last two | Instance state (8); Wrapper channel (2)(6); Capability ledger (7); Confirmed send (3); The wheel (4) |
| v1-10 | CL-1 send issue/outcome in the log | Confirmed send with CL-1 records (E3) | Strip-bay live page (page reads the same records) |
| v1-11 | Page sees dialog clear + state refresh | Resumable SSE feed (E8) | Strip-bay live page |
| v1-12 | One send in flight | Confirmed send with CL-1 records (E3) | — |
| v1-13 | Result payloads carry what CLI prints | Session links (E4) | Budget governor (`release --budget` half) |
| v1-14 | `viola verify` vs fake agent in CI | Capability ledger and viola verify (E2) | — |
| v1-15 | Plan revise answered as plan | Dialog answers by dialog_id (E3) | — |
| v1-16 | Permission suggestion decided | Dialog answers by dialog_id (E3) | — |
| v1-17 | Diagnostics roots, sinks, init order | Unix endpoint and home hardening (E7) | Diagnostics plane (roots + sinks); Home and code-bearing file integrity (Windows init order) |
| v1-18 | Exit-21/exit-1 cause codes | Unix endpoint and home hardening (E7) — adds the socket-directory cause | Exit-cause code catalogue (E6) |
| v1-19 | Toolchain floor 1.96 | Three-OS CI and headless harness (E1) | — |
| v1-20 | `conn` + `diagnostics_level` | Wrapper channel (E2) | Diagnostics plane (config half) |
| v1-21 | Timing constants named and located | MCP server for drivers (E5) — "every MCP call below the tool-call timeout" | Readiness gate and timing constants |
| v1-22 | `viola ui` on an OS-assigned port | viola ui loopback server (E8) | — |
| v1-23 | Arch tree includes test/obs artifacts | Workspace tree and code-graph planes (E1) | — |
| v1-24 | Statusline source named | Statusline pass-through (E4) | — |
| v1-25 | PTY pin decided on current evidence | PTY wrapper on Windows (E2) | — |
| v1-26 | Fix-pass items honoured | Version done-check (E8) | every chunk touching X8/X10/X11/X13 surfaces (Strip-bay live page, Access and error strips, The wheel, Exit-cause code catalogue) |
| v1-27 | `viola run` wraps the CLI on Windows | Instance state and start order (E2) | PTY wrapper on Windows |
| v1-28 | Hooks → normalised events, fail open | Hooks to normalised events (E2) | — |
| v1-29 | `send` at a turn boundary, confirmed | Confirmed send with CL-1 records (E3) | Readiness gate and timing constants |
| v1-30 | `wait` / `last` | wait and last (E3) | — |
| v1-31 | Dialogs answered by `dialog_id` | Dialog answers by dialog_id (E3) | — |
| v1-32 | The wheel | The wheel (E3) | — |
| v1-33 | First live test, then self-drive | First live test and self-drive (E3) | — |
| v1-34 | Ledger, `verify`, transport-only degrade | Capability ledger and viola verify (E2) | — |
| v1-35 | Budget governor | Budget governor (E4) | Statusline pass-through |
| v1-36 | Links between sessions | Session links (E4) | — |
| v1-37 | The board: `viola list` | The board: viola list (E4) | — |
| v1-38 | MCP server for drivers | MCP server for drivers (E5) | — |
| v1-39 | CLI contract | CLI machine contract (E5) | — |
| v1-40 | CLI + terminal output discipline | CLI output discipline (E5) | CLI output tokens (E2); PTY wrapper on Windows (zero own bytes); The wheel (focus/mouse/resize) |
| v1-41 | Crash-safe self-healing state | Self-healing state (E4) | Instance state and start order |
| v1-42 | Optional plugin install | Optional plugin install (E5) | — |
| v1-43 | Chunk-blocking security prerequisites | Security prerequisites (E2) | — |
| v1-44 | Endpoint same-user only + server verified | Unix endpoint and home hardening (E7) — "on each OS" | Windows endpoint admission; Server verification before any frame |
| v1-45 | Home + code-bearing file integrity | Unix endpoint and home hardening (E7) — Unix strict modes | Home and code-bearing file integrity (Windows) |
| v1-46 | Bounded, validated inputs | Resumable SSE feed (E8) — `Last-Event-ID` is the last boundary | Bounded inputs at every boundary (E6) |
| v1-47 | Sanitised errors + never-log floor | viola ui loopback server (E8) — Problem Details + token never logged | Log redaction and never-log floor (E1); Observability gates (secret scan); Sanitised error surfaces (E6) |
| v1-48 | `viola ui` loopback authenticated feed | Resumable SSE feed (E8) | viola ui loopback server |
| v1-49 | The strip-bay page | Strip-bay live page (E8) | Design token bundle and bay layout; Strip and readback primitives |
| v1-50 | A11y verdict on every page state | A11y verdict across page states (E8) | Web test toolchain and a11y harness |
| v1-51 | Contrast, forced colours, reduced motion | Contrast, forced colours, reduced motion (E8) | Design token bundle and bay layout |
| v1-52 | 401 access strip | Access and error strips (E8) | — |
| v1-53 | Linux + macOS parity | Linux and macOS parity (E7) | Linux live confirmation |

**Tally (post-merge):** 53 / 53 capabilities mapped to a claiming chunk; 0 unmapped. 50 chunks. Chunks that claim
nothing on their own (bootstrap units, or they advance a capability claimed later): Fake agent and test-data fixtures,
Supply-chain and workflow gates, Log redaction and never-log floor, Quality gates, CLI output tokens, Readiness gate and
timing constants, Windows endpoint
admission, Server verification before any frame, Home and code-bearing file integrity, Bounded inputs at every boundary,
Sanitised error surfaces, Exit-cause code catalogue, Web test toolchain and a11y harness, Design token bundle and bay
layout, Strip and readback primitives, Linux live confirmation.

**P4 resolved:** the three Unix chunks moved ahead of the Web UI (E7 Cross-OS completion; E8 Web UI, polish & ship).
No claim changed chunk; only epoch labels moved. Overseer checked coverage at P4: 53/53 claimed.
