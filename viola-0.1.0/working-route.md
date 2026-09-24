# Working Route — viola-0.1.0

_Ordered WHAT-not-HOW chunk list for this version. Reorder = move up/down._
_No numbers, no per-chunk IDs/metadata. At promotion /andromeda-phase prefixes the chunk's line with_
_`[{marker}]` to freeze it (wrap's route-resolve edits only the markerless tail; once the chunk's master_
_record is complete, wrap P7 flip-compacts its line to `[{marker}] {title} — {scope hint}`, archiving the_
_verbatim line to route-archive.md); markerless lines stay mutable._
_Chunks separated by `   ↓` within an epoch; only `### Epoch K — {name}` headers are structural._

### Epoch 1 — Foundation
Three-OS CI and headless harness skeleton — SHA-pinned actions, pinned 1.96-floor toolchain, nextest, five agent-run commands, minimal fake agent, per-role JSON line, mutation gate
   ↓
Fake agent and test-data fixtures — scripted modes and control file, receipts, temp-home fixture chain under target/e2e-home, fixture scrub-and-schema walk, committed property seeds
   ↓
Supply-chain and workflow gates — cargo-deny families with telemetry-crate and redaction-toggle bans, weekly advisory run, zizmor, tokio-free sync-crate check, least-privilege permissions
   ↓
Diagnostics plane — per-role JSON sinks with process-start service identity, closed event vocabulary and line schemas, owner-only per-instance detail files, diagnostics_level, torn-line-aware logs merge
   ↓
Log redaction and never-log floor — skip-all span fields, redacted payload types, fixed error displays, content-bearing records only in instance detail files
   ↓
Observability gates — panic hook first, zero-panic, schema, bare-instrument and abort-panic gates, canary secret scan before any upload, print and raw-log lint bans
   ↓
Quality gates — fatal fmt/clippy lint, MSRV 1.96 job, per-OS coverage floors, seeded property and fuzz corpus replay, zero retries, per-job gate verdict
   ↓
Workspace tree and code-graph planes — arch tree lists test/obs/a11y artifacts, release build free of test binaries, Rust plane ok, TypeScript plane decided

### Epoch 2 — Windows slice I: wrapper, events, ledger
Security prerequisites — SQOS pipe-handle spike and pure-Rust SHA-256 pick, both recorded ahead of the channel client and bin/ writes; no unprotected-connect fallback
   ↓
PTY wrapper on Windows — viola run passthrough with zero own bytes, npm-shim resolution, .cmd refusal, CLAUDE* strip, process-handle exit, portable-pty line recorded
   ↓
Instance state and start order — append-only event log, atomic snapshot, heartbeat, pinned bin/ copy, plugin folder, documented start order, live/stale name refusal
   ↓
Wrapper channel — JSON-RPC 2.0 ndjson over the per-instance named pipe (Unix: per-user socket directory), SQOS-opened client, v/sender/conn in every frame, MAX_FRAME, newer-peer refusal
   ↓
Hooks to normalised events — exec-form plugin hooks by absolute path, hook-to-kind map, harness/paste/tag normalisation, silent unwrapped no-op, fail-open exit 0, perf-job-gated hook deadlines
   ↓
CLI output tokens — SGR attention/stale/callsign tokens with depth fallback, colour decision order, stdout/stderr split, plain under non-TTY/NO_COLOR/TERM=dumb/--json, escaped controls
   ↓
Capability ledger and viola verify — versioned rows with probes and post-conditions, stamps, scrubbed fixture recording, largest-payload row, transport-only degrade, fake-agent verify in CI
   ↓
Fake-agent drift contract — fake agent's hook sequences and payloads equal the recorded verify fixtures per CLI version, annotations forwarded, fixture schema check

### Epoch 3 — Windows slice II: driving verbs and live proof
Readiness gate and timing constants — screen-model quiet period, input-box and modal signatures as ledger rows, named confirmation window and deadlines, injected clock, parser-panic degrade
   ↓
Confirmed send with CL-1 records — RB readback mirror with per-reason hints, typed not-delivered details, unconfirmable local commands, /clear post-condition, control-character refusal, one in flight
   ↓
wait and last — cursor-based wake on driver-relevant events, already-logged events returned at once, typed timeout, unreachable on vanished wrapper, newest turn's text
   ↓
Dialog answers by dialog_id — question, permission and plan through hooks, plan answered as plan, permission-suggestion decision, one pending dialog, deadline expiry, unknown-dialog
   ↓
The wheel — human editing key takes it, focus/mouse/resize never do, held bytes during paste, harness turns ignored, pause and release, human-typing/manual-pause refusals, release-from-driver
   ↓
First live test and self-drive — real-CLI verify, overseer drives a builder through viola, founder takes the wheel, overseer drops the prototype

### Epoch 4 — Session state & governance
Self-healing state — torn-line healing, snapshot rebuild by log replay, unknown kinds and fields counted and surfaced, newer-snapshot and newer-peer tolerance
   ↓
Statusline pass-through — named read-only source, per-home redirect for tests, settings.json rewritten each start with absolute pinned path, user output unchanged, readings to budget.json
   ↓
Budget governor — five-hour/seven-day thresholds, budget-paused on automated send only, lift at reset or per-instance override, unknown with age, release --budget payload
   ↓
Session links — driver-to-driven link on first cross-instance send or answer, explicit link/unlink, log-derived link set with since, link result payload
   ↓
The board: viola list — BAY header over six-column board, liveness, status, wheel/budget pause, amber DIALOG, dim stale rows, CLI verdict, read-only unwrapped rows

### Epoch 5 — Driver surface
CLI machine contract — --json per verb mirroring channel results, typed exits 0/1/2/10–14/20/21, cause-named hints, global --home, config.json thresholds and port
   ↓
CLI output discipline — linear static output on three OSes, grouped --help, colour only beside its word, zero wrapper bytes, unblocked human keys during sends
   ↓
MCP server for drivers — stdio tools send/wait/last/answer/list, CLI-parity results and code-only refusals, no wheel or link controls, calls under the tool-call timeout
   ↓
Optional plugin install — MCP-only plugin for unwrapped drivers, absolute-path command, never two hook processes per event, MCP-server precedence measured as ledger row

### Epoch 6 — Security hardening
Windows endpoint admission — pipe access for the owning user and SYSTEM only, remote pipe clients rejected, squatted name blocks start (per security-plan §Authentication)
   ↓
Server verification before any frame — clients match the recorded wrapper pid and start time; CLI/MCP report unreachable, hooks fail open on mismatch
   ↓
Home and code-bearing file integrity — owner-only home, DACL check before any diagnostics file, pinned-binary re-hash, per-start plugin rewrite, single stamp/snapshot writers
   ↓
Bounded inputs at every boundary — size caps, depth limits, closed types, names validated before path joins, paste rule client and wrapper side
   ↓
Sanitised error surfaces — codes and fixed messages only in CLI --json, MCP errors and channel error data, no paths or upstream text
   ↓
Exit-cause code catalogue — every exit-21 and exit-1 cause a distinct code across --json, MCP and process log, matching its human hint

### Epoch 7 — Cross-OS completion
Linux live confirmation — founder-attended on the Linux laptop, one live viola run checks the fake agent's Unix fidelity; reorderable, never blocks CI-run Unix chunks
   ↓
Unix endpoint and home hardening — per-user 0700 socket directory, 0600 socket, peer euid both sides, Unix strict modes, macOS pid gap recorded
   ↓
Linux and macOS parity — openpty wrapper, full fake-agent suite and obs gates green on both, identical diagnostics schema, Windows stays live-supported target

### Epoch 8 — Web UI, polish & ship
viola ui loopback server — OS-assigned port and launch file, Host allowlist, GET-only, health/ready/info and envelopes, cookie-gated /api and SSE, CSP, path-only request logs
   ↓
Resumable SSE feed — verbatim event lines, composite cursor with Last-Event-ID validation, dialog-clear and state-refresh events, logged stream lifecycle and liveness transitions, uncompressed
   ↓
Web test toolchain and a11y harness — Playwright session fixture, axe config, keyboard oracle, screen-reader proxy, scrubbed violation rows, template lint step, SC coverage report
   ↓
Design token bundle and bay layout — app.css token layers and token test, Linux-render font stacks, native-table racks, headings and landmarks, narrow and empty states
   ↓
Strip and readback primitives — light-DOM session-row strip in lit, stale, cocked and unwrapped states; readback box in four states, one drawing
   ↓
Strip-bay live page — ATIS header, transfer markers, silent expandable tape and readback box, polite announcer, text-only fields, viola list parity, Playwright path specs
   ↓
Access and error strips — 401 access strip after a ui restart, 503 state-unreadable strip, TAPE stopped strip, announced once, never token, URL or path
   ↓
Contrast, forced colours, reduced motion — token-pair checks on the Linux font fallback, holder-surface ink rule, forced-colours state distinctness, zero-motion mode
   ↓
A11y verdict across page states — zero violations per web-spa state, keyboard walk equals oracle, announcements once, no focus theft, every applicable criterion tagged
   ↓
Version done-check — every gate green on three OSes, masters agree, fix-pass items honoured, founder screen-reader pass recorded, overseer drives a builder through viola
