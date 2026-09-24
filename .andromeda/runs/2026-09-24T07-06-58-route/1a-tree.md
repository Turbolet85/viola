# 1A tree — viola-0.1.0

_Hierarchical decomposition (debug artifact; sub-block tier dropped at 1C)._

## Epoch 1 — Foundation
- (1a) CI + harness spine
  - Three-OS CI and headless harness (source: intent F-01, F-02, F-04, F-06, F-19; founder rule "chunk 1"; TP §3 5-command implementation, §9; S:303-333 CI integration; OB §3 Logging stack)
  - Fake-agent contract suite (source: intent F-03; TP §6 Contract suite, §7 Fake agent, Fixture hygiene; A:99, A:366-367)
  - Supply-chain and workflow gates (source: intent F-01; S §Dependency Security, Bootstrap dep-audit-tooling-install + dep-security-ci-gate; A:377, A:437-438)
- (1b) Observability plane
  - Diagnostics plane (source: intent F-04, F-17 sinks half, F-20 config half; OB §3 Logging stack, Log format JSON schema, Log file location, Bootstrap logger-stack-install → log-format-schema-emit; TP §3 Log format)
  - Observability gates (source: intent F-05; OB §7, §8, §9 G1–G4, Bootstrap pii-scrubbing-wire + obs-ci-gate-wire; S logging-redaction-wire)
- (1c) Quality gates + tree
  - Quality gates (source: intent F-07, F-01 perf-job/tool-presence/home-lifetime; TP §10, §3 `gate`, Bootstrap coverage-tooling-install → quality-gate-config-emit)
  - Workspace tree and code-graph planes (source: intent F-23, F-08; TP §2 directory conventions; OB §3 Bootstrap; AY §3 Bootstrap)

## Epoch 2 — Windows slice I: wrapper, events, ledger
- Security prerequisites (source: intent F-43; S Decisions Log open questions (SQOS spike, SHA-256 crate); S:358-364 per-chunk ordering)
- PTY wrapper on Windows (source: intent F-27, F-25; A [PTY], [Session Liveness]; B §3.2.1, R5, R8, S1/S6; TP §6 Path 1, E2)
- Instance state and start order (source: intent F-27, F-41 first half, F-09 (8); A [Database / State Store], [Deployment / Distribution], [Session Liveness]; S amendment 8)
- Wrapper channel (source: intent F-20, F-09 (2)(6), F-43 SQOS client; A [Message Broker / IPC], [API Style]; S amendments 2, 6; OB D-10)
- Hooks to normalised events (source: intent F-28; A [Hook Transport], [Hook Contract], Hook → kind map; TP §6 E1; OB D-16)
- Capability ledger and viola verify (source: intent F-34, F-14, F-09 (7); A [CLI Version Compatibility]; S amendment 7, S:554; TP §3 `boot` step 4, §6 Path 7)

## Epoch 3 — Windows slice II: driving verbs and live proof
- Readiness gate and timing constants (source: intent F-21; A [Screen Model], [Delivery Confirmation], [Hook Transport] open items; TP §12 arch requests 1 and 3)
- Confirmed send with CL-1 records (source: intent F-29, F-10, F-12, F-09 (3); A [Delivery Confirmation], [CLI Conventions]; XF X1, X3, T2; TP §6 Path 2; DS Readback)
- wait and last (source: intent F-30; A [Message Broker / IPC] wait/last; TP §6 Path 3)
- Dialog answers by dialog_id (source: intent F-31, F-15, F-16; A [Hook Contract], channel `answer`; B S3/S7/S8; TP §6 Path 4)
- The wheel (source: intent F-32, F-09 (4); A [Human Takeover / Wheel]; S amendment 4; TP §6 Path 5)
- First live test and self-drive (source: intent F-33; B §6; H:73-77)

## Epoch 4 — Session state & governance
- Self-healing state (source: intent F-41; A Snapshot envelope, [Validation], Mixed-version tolerance; TP §6 E5, Chaos suite; OB D-03)
- Statusline pass-through (source: intent F-24; A Hook contract `hook statusline`; TP §6 Path 6, `boot --statusline-echo`)
- Budget governor (source: intent F-35, F-13 release half; A [Budget Governor]; TP §6 Path 6; DS web-spa component 5)
- Session links (source: intent F-36, F-13 link half; A [Links]; TP §6 E3)
- The board: viola list (source: intent F-37; A GUI list envelope, Session liveness; DS cli pattern 1; LT cli `viola list`; TP §6 Cross-surface parity)

## Epoch 5 — Driver surface
- CLI machine contract (source: intent F-39; A [CLI Conventions], CLI exit codes, Config management; DS/LT Surface: cli)
- CLI output discipline (source: intent F-40; AY P4, P6, §10 CLI invariants; DS cli colour decision order)
- MCP server for drivers (source: intent F-38, F-21 MCP half; A [MCP]; S:228, S:293; TP §1 MCP surface)
- Optional plugin install (source: intent F-42; A [Plugin Scope]; S:553)

## Epoch 6 — Security hardening
- Windows endpoint admission (source: intent F-44 Windows half; S §Authentication & Authorization, Bootstrap auth-scaffolding-baseline `viola-channel` listener)
- Server verification before any frame (source: intent F-44 Windows half; S Bootstrap `viola-channel` client; S:472-478)
- Home and code-bearing file integrity (source: intent F-45, F-17 Windows half; S Bootstrap `viola-state`; S:207, S:262-266)
- Bounded inputs at every boundary (source: intent F-46; S §Input Validation, input-validation-library-install)
- Sanitised errors and never-log floor (source: intent F-47; S error-sanitization-wire, logging-redaction-wire; OB §8)
- Exit-cause code catalogue (source: intent F-18; XF X7, T4, F4, B6; DS cli pattern 2; OB §6 detail code catalog; TP §6 Exit-cause matrix)

## Epoch 7 — Web UI
- (7a) Server + feed
  - viola ui loopback server (source: intent F-48, F-22, F-09 (1)(5); A [GUI Control Scope], GUI routes; S API Security, `viola-ui` bootstrap; TP §3 boot steps 6-7)
  - Resumable SSE feed (source: intent F-48, F-11, F-46 Last-Event-ID; A SSE feed; XF X2; OB §3 Heartbeat ticks; TP §6 E4)
- (7b) Page
  - Web test toolchain and a11y harness (source: TP §3 test-runner-install (Node side); AY §3 Bootstrap a11y-tooling-install → violation-json-emission-wire)
  - Design token bundle and bay layout (source: DS Color Palette, Typography, Spacing, Surface: web-spa tokens; LT Surface: web-spa; AY D-A11Y-02/03)
  - Strip-bay live page (source: intent F-49, F-10 page half, F-26 X8/X10/X11; DS web-spa components 1–5; LT web-spa components)
  - 401 access strip (source: intent F-52, F-26 X13; XF X9; AY P5, D-A11Y-06; DS web-spa component 6)
- (7c) A11y verification
  - A11y verdict across page states (source: intent F-50; AY §3, §9, §10)
  - Contrast, forced colours, reduced motion (source: intent F-51; AY §6; DS Color Palette, Motion)

## Epoch 8 — Polish & ship (cross-OS completion)
- Linux live confirmation (source: intent F-53; H:180-181)
- Unix endpoint and home hardening (source: intent F-44, F-45, F-17, F-18 Unix halves; S amendment 2, Bootstrap `viola-channel` Unix + `viola-state` strict modes; S Known gaps (macOS))
- Linux and macOS parity (source: intent F-53; TP §9 Matrix builds)
- Version done-check (source: intent §Definition of done, F-26; XF fix passes)
