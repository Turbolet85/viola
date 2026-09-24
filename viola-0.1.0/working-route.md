# Working Route — viola-0.1.0

_Ordered WHAT-not-HOW chunk list for this version. Reorder = move up/down._
_No numbers, no per-chunk IDs/metadata. At promotion /andromeda-phase prefixes the chunk's line with_
_`[{marker}]` to freeze it (wrap's route-resolve edits only the markerless tail; once the chunk's master_
_record is complete, wrap P7 flip-compacts its line to `[{marker}] {title} — {scope hint}`, archiving the_
_verbatim line to route-archive.md); markerless lines stay mutable._
_Chunks separated by `   ↓` within an epoch; only `### Epoch K — {name}` headers are structural._

### Epoch 1 — Foundation
[2026-09-24-three-os-ci-headless-harness-skeleton] Three-OS CI and headless harness skeleton — SHA-pinned actions, pinned 1.96-floor toolchain, nextest, five agent-run commands, minimal fake agent, per-role JSON line, mutation gate
   ↓
[2026-09-24-fake-agent-and-test-data-fixtures] Fake agent and test-data fixtures — scripted modes and control file, receipts, temp-home fixture chain under target/e2e-home, fixture scrub-and-schema walk, committed property seeds
   ↓
[2026-09-24-supply-chain-and-workflow-gates] Supply-chain and workflow gates — cargo-deny families with telemetry-crate and redaction-toggle bans, weekly advisory run, zizmor, tokio-free sync-crate check, least-privilege permissions
   ↓
[2026-09-24-diagnostics-plane] Diagnostics plane — per-role JSON sinks with process-start service identity, closed event vocabulary and line schemas, owner-only per-instance detail files, diagnostics_level, torn-line-aware logs merge
   ↓
[2026-09-24-log-redaction-and-never-log-floor] Log redaction and never-log floor — skip-all span fields, redacted payload types, fixed error displays, content-bearing records only in instance detail files
   ↓
[2026-09-24-observability-gates] Observability gates — panic hook first, zero-panic, schema, bare-instrument and abort-panic gates, canary secret scan before any upload, print and raw-log lint bans
   ↓
Quality gates — fatal fmt/clippy lint, MSRV 1.96 job, per-OS coverage floors, seeded property and fuzz corpus replay, zero retries, per-job gate verdict  CARRY: chunk 2026-09-24-supply-chain-and-workflow-gates left zizmor's pedantic-persona `concurrency-limits` (help, low) open on `ci.yml` and `nightly.yml` — the default persona suppresses it; decide whether the workflows take a `concurrency:` block (it changes run cancellation)  PREREQ: first action at take-up (overseer direction at the 2026-09-24-observability-gates wrap): read the CI ubuntu `mutants` job on that chunk's pushed sha. Its 3 `crates/viola-e2e/src/harness/secret_scan.rs` `file_mode` mutants (`#[cfg(unix)]`, unkillable on the Windows host, measured at that chunk's local gate) must read caught there; any survivor is folded into this chunk's scope and fixed first  CARRY: chunk 2026-09-24-observability-gates left the `mutants` job's `mutants.out/` upload unscanned (it carries test output; obs-plan §9's scan roots do not cover it): scan it or stop uploading it (overseer direction at that chunk's phase review)  CARRY: chunk 2026-09-24-observability-gates already wired fatal `cargo fmt --all --check` and `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` on all three OSes in the `ci.yml` `lint` job; this entry's lint half is done, and what remains is the MSRV job, coverage floors, replay, zero retries and the per-job gate verdict
   ↓
Workspace tree and code-graph planes — arch tree lists test/obs/a11y artifacts, release build free of test binaries, Rust plane ok, TypeScript plane decided

### Epoch 2 — Windows slice I: wrapper, events, ledger
Security prerequisites — SQOS pipe-handle spike and pure-Rust SHA-256 pick, both recorded ahead of the channel client and bin/ writes; no unprotected-connect fallback
   ↓
PTY wrapper on Windows — viola run passthrough with zero own bytes, npm-shim resolution, .cmd refusal, CLAUDE* strip, process-handle exit, portable-pty line recorded  CARRY: chunk 2026-09-24-three-os-ci-headless-harness-skeleton's `viola run` is the first slice — `src/run/mod.rs::spawn_child` is the one spawn function whose body becomes the `viola-pty` seam (it spawns with the full inherited env today: the R8 strip is this entry's); it sets no `VIOLA_NAME`/`VIOLA_DIR`/`VIOLA_BIN` yet and emits no `run.start` span; the harness supervisor stops wrappers through a stdin pipe (`\x03` then close) until outer PTYs exist — switch `supervise` and the `run_cli` tests to the PTY form here  CARRY: chunk 2026-09-24-fake-agent-and-test-data-fixtures deferred the `viola_e2e::fixtures` copy of the fixture chain to its first E2E consumer (test-plan §3 `run` step 2: Paths 1/5/E1/E2/E5; E2, the CLAUDE* strip, is this entry's) — build it here if this entry lands the E2 scenario, else re-pin to the next entry that lands one; the root chain's `tests/support/home.rs` `Wrapper` also drives `viola run` through a stdin pipe — switch it to the PTY form with `supervise`
   ↓
Instance state and start order — append-only event log, atomic snapshot, heartbeat, pinned bin/ copy, plugin folder, documented start order, live/stale name refusal
   ↓
Wrapper channel — JSON-RPC 2.0 ndjson over the per-instance named pipe (Unix: per-user socket directory), SQOS-opened client, v/sender/conn in every frame, MAX_FRAME, newer-peer refusal  CARRY: chunk 2026-09-24-supply-chain-and-workflow-gates wired the tokio ban as a sole-root `deny-sync.toml` run per crate in `scripts/sync-crates.txt` — `viola-channel` joins that list (and CI job 3's `cargo check`) with its crate, without its `tokio` feature  CARRY: chunk 2026-09-24-diagnostics-plane made `corr` a caller-supplied typed field of `obs_event!`. It has no macro arm; obs-plan §3 was amended to match, per the operator's escalation ruling at that wrap. This first corr producer makes `corr` `required` in `schemas/diag-line.v1.json` for every corr-bearing event (channel-*, dialog-*, hook-*, send-*, release-from-driver), with a negative test: a corr-bearing line without `corr` is rejected.  CARRY: chunk 2026-09-24-log-redaction-and-never-log-floor found no redaction subject at HEAD (0 `#[instrument`, 0 payload types, 0 product `<Crate>Error` enums, no `veil` dependency), so this first-consumer entry lands them: `veil` 0.3.0 in `[workspace.dependencies]` without `toggle` and `#[derive(Redact)]` on the first payload types (obs-plan §8 Scrubbing libraries), `#[instrument(skip_all, name = "<area>.<operation>", fields(..))]` on the first spans (obs-plan §4), and a fixed-`Display` `ChannelError`.
   ↓
Hooks to normalised events — exec-form plugin hooks by absolute path, hook-to-kind map, harness/paste/tag normalisation, silent unwrapped no-op, fail-open exit 0, perf-job-gated hook deadlines  CARRY: chunk 2026-09-24-diagnostics-plane deferred two obs items to the first shared-writer, exit-0 role, and this is it. (1) The three-OS concurrent-append check (obs D-28), with a detail line over 4 KiB; concurrent hook processes are the first to share `hook-<name>.ndjson` / `detail-hook.ndjson`. (2) The pre-clap argv role classification at the main-thread catch site (obs §7); today every caught panic exits 1, because `run` is the only verb, and `hook` must exit 0. `viola::obs::role_file_name` already names `hook-<name>.ndjson`.  CARRY: chunk 2026-09-24-log-redaction-and-never-log-floor routed dispatch errors through `cmd::Failure` + `viola::obs::report_internal_error` (the `chain` detail line; the role line is `run`-only via `internal_error_exit_line`), but no drift-report producer existed. This first serde_path_to_error consumer routes its drift reports as `drift_report` into `detail-hook.ndjson` (obs-plan §8 Detail-file scope; the field is already in `schemas/diag-detail.v1.json`), and its `Failure` sink must keep `hook` at exit 0 with empty stderr.
   ↓
CLI output tokens — SGR attention/stale/callsign tokens with depth fallback, colour decision order, stdout/stderr split, plain under non-TTY/NO_COLOR/TERM=dumb/--json, escaped controls  CARRY: chunk 2026-09-24-log-redaction-and-never-log-floor left the catch site silent because `run` was the only verb (obs-plan §7 per-role: `run` prints nothing). When a `cli`-process verb exists, its catch-site error prints exactly `error: internal error` on stderr, uncoloured and with no hint (obs-plan §7; design-system §Surface: cli → Exit-code phraseology). The chain stays only in the detail file.
   ↓
Capability ledger and viola verify — versioned rows with probes and post-conditions, stamps, scrubbed fixture recording, largest-payload row, transport-only degrade, fake-agent verify in CI  CARRY: chunk 2026-09-24-fake-agent-and-test-data-fixtures deferred the `fixtures/claude/*/*.json` `#[files]` hygiene walk and the claude-fixture schema to the first recorded fixture (test-plan §7 Fixture hygiene; checker `tests/support/hygiene.rs`); record fixtures as `fixtures/claude/<cli-version>/<Event>.<variant>.json` (PascalCase hook event name, test-plan §2); the root `stamped_home` is an interim no-stamp seam — make it stamp through `viola verify` here
   ↓
Fake-agent drift contract — fake agent's hook sequences and payloads equal the recorded verify fixtures per CLI version, annotations forwarded, fixture schema check  CARRY: chunk 2026-09-24-fake-agent-and-test-data-fixtures's fake agent runs every `type:"command"` hook registered for an event — hook `matcher` evaluation is deferred to this entry, where recorded `tool_name`s exist

### Epoch 3 — Windows slice II: driving verbs and live proof
Readiness gate and timing constants — screen-model quiet period, input-box and modal signatures as ledger rows, named confirmation window and deadlines, injected clock, parser-panic degrade  CARRY: chunk 2026-09-24-fake-agent-and-test-data-fixtures deferred the fake agent's `--vt100-panic-bytes` mode to this entry (consumer-first: its byte sequence is set by the vt100 parser-panic degrade built here)
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
Statusline pass-through — named read-only source, per-home redirect for tests, settings.json rewritten each start with absolute pinned path, user output unchanged, readings to budget.json  CARRY: chunk 2026-09-24-fake-agent-and-test-data-fixtures deferred the fake agent's `statusline-echo` mode and its `settings.json` read to this entry (test-plan §7; no shape before a recorded fixture)
   ↓
Budget governor — five-hour/seven-day thresholds, budget-paused on automated send only, lift at reset or per-instance override, unknown with age, release --budget payload
   ↓
Session links — driver-to-driven link on first cross-instance send or answer, explicit link/unlink, log-derived link set with since, link result payload
   ↓
The board: viola list — BAY header over six-column board, liveness, status, wheel/budget pause, amber DIALOG, dim stale rows, CLI verdict, read-only unwrapped rows  CARRY: chunk 2026-09-24-fake-agent-and-test-data-fixtures deferred the fake agent's `agents --json` stub (`FAKE_CLAUDE_AGENTS_MODE=recorded|oversize|malformed`, test-plan §7 seed table) to this entry — pin its shape against a recorded `claude agents --json` output

### Epoch 5 — Driver surface
CLI machine contract — --json per verb mirroring channel results, typed exits 0/1/2/10–14/20/21, cause-named hints, global --home, config.json thresholds and port  CARRY: chunk 2026-09-24-three-os-ci-headless-harness-skeleton resolves `--home` as given, else `<user home>/.viola`, without `std::fs::canonicalize` and without the `VIOLA_DIR`-grandparent step (security-plan §Input Validation CLI row; arch §Cross-cutting Config management)
   ↓
CLI output discipline — linear static output on three OSes, grouped --help, colour only beside its word, zero wrapper bytes, unblocked human keys during sends
   ↓
MCP server for drivers — stdio tools send/wait/last/answer/list, CLI-parity results and code-only refusals, no wheel or link controls, calls under the tool-call timeout  CARRY: chunk 2026-09-24-supply-chain-and-workflow-gates measured (research.md §Measured facts, scratch workspace) that once a member enables `viola-channel[tokio]`, `viola-channel` as its own `deny-sync.toml` sole root false-fails (`cargo metadata` reports its unified features) — keep `viola-channel`'s own normal graph tokio-ban-checked by another sync root or form, never drop it from coverage
   ↓
Optional plugin install — MCP-only plugin for unwrapped drivers, absolute-path command, never two hook processes per event, MCP-server precedence measured as ledger row

### Epoch 6 — Security hardening
Windows endpoint admission — pipe access for the owning user and SYSTEM only, remote pipe clients rejected, squatted name blocks start (per security-plan §Authentication)
   ↓
Server verification before any frame — clients match the recorded wrapper pid and start time; CLI/MCP report unreachable, hooks fail open on mismatch
   ↓
Home and code-bearing file integrity — owner-only home, DACL check before any diagnostics file, pinned-binary re-hash, per-start plugin rewrite, single stamp/snapshot writers  CARRY: chunk 2026-09-24-three-os-ci-headless-harness-skeleton's `viola run` creates the home and `diagnostics/` with Unix 0700/0600 only — no strict-modes check before opening the role file and no Windows protected user+SYSTEM DACL for a `--home` outside `%USERPROFILE%` (test-plan §3 boot step 2 and security-plan §Anti-Patterns Data Protection state the target)
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
