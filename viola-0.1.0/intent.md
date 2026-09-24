# Viola 0.1.0 — version intent

Source key: **B** = `refs/viola-brief.md` · **A** = `.andromeda/architecture.md` · **S** = `security-plan.md` ·
**DS** = `design-system.md` · **LT** = `layout-templates.md` · **TP** = `test-plan.md` · **OB** = `obs-plan.md` ·
**AY** = `a11y-plan.md` · **XF** = overseer `cross-plan-findings.md` (X*, T*, O*, I*, F*, B*, Y*) · **H** = overseer
`.claude/session-handoff.md`. `A:46` = line 46. B, A, S and XF anchors are line numbers. TP, DS, LT, OB and AY changed
in the overseer fix passes `693e083`, `85d6354` and `90c705d`, so those are anchored by section name or decision id.

## Context

The founder runs two Claude Code sessions per managed project: an **overseer** that verifies the build and writes
relays, and a **builder** that runs the Andromeda pipeline skills. Today the founder is the transport between them:
relays are pasted by hand, reviews and forks are answered by hand, and skills and `/clear` are typed by hand. A third
project (viola itself) beside Pulse and Conductor makes this the bottleneck (B §1). The platform declined to supply
a way to deliver input into a running session (issue #85289, closed NOT_PLANNED; B §7). A throwaway prototype in
`viola-lab` proved the mechanisms on Windows and already drove this project's own `/andromeda-arch` run (B §4.1,
§4.2, §6). The viola repo holds no product code yet: only `refs/`, the seven `.andromeda/` masters and a stray
`prototype/` folder (H:91-94).

## Core problem

User authority in a Claude Code session comes only from that session's own terminal input (B §3.1). So one session
can drive another only if a bridge owns the driven session's terminal input, types at turn boundaries, answers its
dialogs through hooks, reports turn-synchronised events, and hands the wheel to the human the moment they type —
on the user's own subscription through the unmodified `claude` binary, never the API (B D1–D5, R1–R8; A:468-486).

## Definition of done

0.1.0 is done when all of the following hold:

1. On the founder's Windows host, the overseer drives a builder **through viola itself** (not the prototype): it
   sends `/andromeda-new-session`, waits for the turn to end, reads the dashboard, answers a dialog, clears between
   skills, and the founder typing into the builder window takes the wheel and pauses automation (B §6; H:73-77).
2. Every capability below is delivered and verified headlessly by an agent: the harness, the fake agent and the
   recorded fixtures are the contract, and the real CLI is exercised only by local `viola verify` (TP header
   "Agent-driven invariant"; H:51-54).
3. CI is green on Windows, macOS and Linux on every commit, with the mutation, coverage, perf, observability,
   accessibility and supply-chain gates holding (A:431-442; TP §9-§10; OB §9-§10; AY §9-§10).
4. A local view-only page shows the active sessions, the links between them and a live event feed, and meets its
   accessibility label (B D5; A:91; AY §3 WCAG criteria mapping).
5. The masters agree with each other: every cross-plan amendment below is reflected in `architecture.md` by the
   wrap reconcile of the chunk that implements it (XF header; H:126-140).

## Findings

### (a) Infrastructure first — before any feature code (founder direction, H:60-66)

**F-01 · Three-OS CI pipeline from the first commit**
- OBSERVED: no workspace, no `Cargo.toml`, no `.github/workflows/`; nothing builds or runs in CI (H:91-94). Two
  masters disagree on CI mechanics: the perf gates' job (tests: own job; obs: inside E2E), `jq`/`rg` availability,
  where test homes live and whether they survive until the gates, and which artifacts upload after the secret scan
  (XF F1, F2, B1, B4, B7, Z8).
- EXPECT: every push and PR runs build, format, lint (warnings fatal), the tokio-free check of the sync crates, the
  dependency policy (licences, C-crate, tokio and banned-crate rules, advisories, sources), workflow linting and
  tests, natively on Windows, macOS and Linux; all actions are SHA-pinned under least-privilege permissions; a weekly
  scheduled advisory check runs without code changes; the perf gates run in their own job; tools the gates need are
  checked present first and fail loudly if missing; test homes survive until every gate has read them; and nothing
  from a test home is uploaded unless the secret scan passed.
- Anchors: A:34, A:99, A:431-442; S:303-333; TP §9 CI Integration; OB §9 CI Integration; XF:73-74 (F1, F2), XF:81 (B1), XF:84 (B4), XF:87 (B7); B §3.4 "CI on all three OSes".

**F-02 · The headless harness: five agent-run commands**
- OBSERVED: no harness exists; the prototype's three tests were run by hand and not mutation-checked (H:59).
- EXPECT: an agent can `boot`, `run`, `status`, `cleanup` and `logs` a product session on any of the three OSes
  through identical shell shims, each command printing exactly one JSON document and a typed exit code; `boot`
  reaches readiness or names the unmet check, `cleanup` is idempotent and proves endpoints, ports and token files are
  gone, and `status` shows the CLI board and the GUI sessions agree.
- Anchors: TP §3 Test Harness Contract (5-command implementation, Status endpoint shape); TP §1 Test harness requirements; H:51-54.

**F-03 · The fake agent and recorded fixtures as the test contract**
- OBSERVED: CI cannot run the real `claude`; the only record of real hook payloads is the prototype's ad-hoc logs.
- EXPECT: a fake agent behaves like the CLI at every boundary viola uses (version answer, bracketed paste as one
  prompt, hook calls through the plugin files `run` wrote, scripted dialogs and modes, receipts of what it received),
  replays per-CLI-version hook fixtures recorded by `viola verify`, and a contract suite fails when the fake agent
  drifts from the recorded fixtures (including forwarded `annotations`); fixtures carry no real prompts, home paths
  or usernames.
- Anchors: B §3.4 (fake agent), B §4.1 S8 (instrument defect); A:99, A:366-367; TP §7 (Fake agent, Fixture hygiene), TP §6 Contract suite; S:271, S:505.

**F-04 · Structured JSON logs from every process, and a `logs` view**
- OBSERVED: no logging exists. The contract is now fixed across two masters: tests own the harness-grepped fields;
  obs adds the `cli` role, six events, absent-key-means-null, the codes-only home-level files and the per-instance
  detail files (OB D-21; TP §12 fix-pass entry).
- EXPECT: `run`, `hook`, `mcp`, `ui` and client-side `cli` each write one-line JSON records with the grepped fields
  (timestamp, level, target, message, event, process, instance, corr) under the closed event vocabulary (including
  `release-from-driver`, `liveness-changed`, `state-recovered`, `sse-opened`/`sse-closed`, `parse-rejected`); a null
  correlation is a missing key; `hook` never writes stderr and a failed log write never changes its exit; `logs`
  merges events, process logs and detail files, filterable by instance, kind and process, emitting torn lines as
  marked records rather than dropping them.
- Anchors: OB §3 Logging stack, Log format JSON schema, Log file location, Snapshot / paste-to-AI integration; OB §6 Log Coverage; OB D-06, D-12, D-21; TP §3 Log format and `logs`; H:51-54 (founder direction); XF O1–O3 (59-61).

**F-05 · Observability gates: no unlogged panic, schema-true lines, no leaked secret**
- OBSERVED: obs defines gates with no implementation: a panic hook installed before anything else, schema
  conformance of every real log line, a secret scan with a canary, and lint bans on raw logging, printing and
  unskipped span fields; the printing ban as written also hits the harness and fake agent, which must print (XF B5).
- EXPECT: every panic after log init leaves exactly one error line and `hook` still exits 0; every home-level and
  detail line in CI validates against the committed schemas; a canary fed through synthetic inputs never reaches
  home-level logs, and no token, cookie or stripped `CLAUDE*` value appears in any artifact; product crates cannot
  print to stdout/stderr or log outside the one sanctioned macro, while test-only binaries may print; redaction cannot
  be switched off by any setting.
- Anchors: OB §3 OTel SDK init (init order), Bootstrap phases; OB §7 Error Capture, §8 PII Scrubbing, §9 gates G1–G4, §10 SLO Invariants; OB D-19; XF:85 (B5); S:383-389.

**F-06 · Mutation gate per chunk from chunk 1**
- OBSERVED: `cargo-mutants` is installed on the host; no project runs it (H:55-59).
- EXPECT: every chunk's diff is mutation-tested as part of the harness `run` and in CI; any missed or timed-out
  mutant makes the run red, a missing diff base fails rather than passes vacuously, and each guard test in a chunk
  plan is paired with its own remove-the-guard mutation; obs code is mutated like product code.
- Anchors: H:55-59 (founder: «тесты сразу с мутациями»); TP §2 pyramid (Mutation row), TP §3 `run` step 4, TP §10 Mutation gate; OB §9 Pipeline integration (Mutation row).

**F-07 · Quality gates: coverage, performance budgets, property/fuzz replay, zero flakes**
- OBSERVED: no gates exist.
- EXPECT: per-OS coverage meets lines 85 / functions 95 / regions 80; hook deadlines hold on the worst sample; every
  parser surface named by security has property tests with committed seeds and a corpus replay that fails on an empty
  corpus; no retries anywhere, and a flaky test keeps its chunk red until fixed; one `gate` verdict per CI job reads
  all artifacts and treats a missing artifact as a breach.
- Anchors: TP §10 Quality Gates; TP §6 Property suite; TP §3 `gate`; S:240 (parser surfaces); OB §10 Performance budgets.

**F-08 · Code-graph seeded at setup and built on the first real code**
- OBSERVED: no master section covers the code-graph (a pipeline-level direction). The viola tree has no root
  `Cargo.toml`, so setup's refresh would skip the Rust plane; the page is Lit as plain vendored ESM with no JS build
  (DS web-spa Toolkit), while the Playwright and a11y specs are TypeScript (TP §2 directory conventions; AY §3
  Bootstrap phases).
- EXPECT: all five code-graph files land in `scripts/` at setup with its health check green; after the first chunk's
  wrap the Rust plane reports ok and its database exists; whether a TypeScript plane is active for `e2e-web/` is
  stated explicitly, not left to chance.
- Anchors: H:87-96 (founder: «проследи чтоб граф сразу засеялся»). No master anchor exists.

### (b) Architecture amendments the other masters depend on

**F-09 · Security's eight amendments folded into the architecture**
- OBSERVED: `architecture.md` still says no GUI token, a Unix socket at `$TMPDIR/viola-<h12>.sock`, no
  `control-character` detail, `release`-from-driver undecided, no `ui/<port>.url`, no frame cap, no largest-payload
  ledger row, and an unspecified `<hash>` (A:91, A:332, A:126-133, A:51, A:357-364, A:70-86, A:96); tests now follow
  the security side (TP §12 fix pass T6).
- EXPECT: the architecture carries all eight — (1) per-launch cookie on `/api/*` and SSE in v1, (2) the Unix endpoint
  in a per-user 0700 directory, (3) `not-delivered`/`control-character` checked before the refusal order plus the
  `unauthorized` and `cross-origin-forbidden` problem URNs, (4) `release` with `from` refused as
  `release-from-driver`, (5) the `ui/<port>.url` file, (6) `MAX_FRAME` 16 MiB, (7) a "largest hook payload seen"
  ledger row, (8) `<hash>` = truncated SHA-256 — each landing no later than the chunk ordering security names.
- Anchors: S:572-580 (amendments), S:358-364 (per-chunk ordering); H:33-38, H:127-129.

**F-10 · CL-1: send issue and outcome are events in the log**
- OBSERVED: design's readback box and the tests' critical path need `send-issued` (with `cursor`, `from`),
  `send-refused` (`refusal` + `detail`) and a logged outcome for `ok`/`confirmed:false`; none is in arch's closed
  event-kind list (A:158), client-side refusals never reach a wrapper, and `from` is optional so a plain-shell send
  has no instance to show (XF X1, T2). Obs already logs `send-*` process-log lines with `conn`/`rpc_id` (OB D-27, D-30).
- EXPECT: every send accepted by a wrapper leaves an issue record in the instance's event log, and every outcome
  (read back, refused with reason, unconfirmable) is recoverable from that log alone; these records never wake
  `wait`; a refusal decided client-side is visible in the `cli` process log; the page and the tests read the same
  records.
- Anchors: XF:11 (X1), XF:34 (T2); DS domain status rows (Readback, send-issued/send-refused) and web-spa component 2; LT signature placement; A:158, A:288; TP §6 Path 2; OB §3 Trace context propagation.

**F-11 · The live page sees a dialog clear and state refresh**
- OBSERVED: no event marks a dialog answered, expired or answered `null`, and polling is banned, so a cocked strip
  never returns into line and LIVE/STATUS/budget never refresh on the page (XF X2). Obs's `liveness-changed` is a
  `ui` process-log line, not a feed event (OB §3 Heartbeat ticks).
- EXPECT: the view learns, without polling, when a pending dialog clears and when liveness, status or the budget
  reading change, so the strip bay reflects current state through the same feed that carries events.
- Anchors: XF:12 (X2); LT web-spa primary screens; DS cocked strip, Motion; A:242, A:248, A:315, A:89; OB §3 Heartbeat ticks, D-02.

**F-12 · At most one send in flight per instance**
- OBSERVED: design pairs a send with "the next driver `prompt-submitted`" assuming at most one send in flight, but
  arch never serialises concurrent sends to one instance (XF X3).
- EXPECT: two concurrent sends to the same instance can never both be typed; the second is refused or ordered, and
  every read-back pairs with exactly the send that caused it.
- Anchors: XF:13 (X3); DS web-spa component 2 (Sources); A:67.

**F-13 · Result payloads carry what the CLI prints**
- OBSERVED: after the fix pass, `release --budget` still prints `until …Z` and `link` prints `since` /
  `already linked`, but their `ok` payloads are `{wheel, budget_paused}` and `{}` (XF X4, X5, F3).
- EXPECT: every human output line of `release --budget` and `link` is derivable from its machine result, and the
  `--json`/MCP result carries the same facts.
- Anchors: XF:14-15 (X4, X5), XF:75 (F3); DS cli pattern 4; LT cli wheel/handoff verbs; A:68, A:270-271.

**F-14 · `viola verify` against the fake agent runs in CI**
- OBSERVED: arch says `viola verify` runs "only locally, never in CI" (A:441); stamps may be written only by
  `viola verify` (S:554); tests ratified running verify against the fake agent in CI (TP §12 User review 1; XF T8).
- EXPECT: CI test homes are stamped only by `viola verify` run against the fake agent; the real `claude` never runs
  in CI; a local-live mode refuses to start under CI.
- Anchors: XF:40 (T8); A:99, A:441; S:554; TP §3 `boot` step 4 and `run --local-live`; H:122-123.

**F-15 · Plan revise is answered as a plan, not a permission**
- OBSERVED: plan revise arrives through PermissionRequest, which arch maps to kind `permission` with
  `allow`/`deny`; tests and the prototype answer `revise` (XF T9).
- EXPECT: a plan dialog raised by either hook is presented and answered as a `plan` (approve / revise with a
  message); approve never relies on the ignored PermissionRequest path, and the mapping is one ledger-backed rule.
- Anchors: XF:41 (T9); B §4.1 S7 (194-208); A:163-164, A:265-268; TP §4 viola-agent-claude, TP §6 Path 4.

**F-16 · Permission suggestions have a place in the answer**
- OBSERVED: the brief measured a permission dialog's options as allow · allow plus suggestion n · deny, and R7 says
  a permission answer includes a suggestion (B:203-205, B:109-114); arch's permission answer is
  `{behavior, message?}` with no suggestion slot (A:267) (XF I1).
- EXPECT: the architecture decides whether a driver can pick one of the offered permission suggestions; if yes, the
  answer shape carries it and the chosen suggestion takes effect in the driven session; if no, the omission is
  recorded as a deliberate v1 limit rather than lost.
- Anchors: XF:66 (I1); B:109-114 (R7), B:202-205 (S7); A:267, A:272.

**F-17 · Diagnostics roots, sinks and initialisation order**
- OBSERVED: arch names only `instances/<name>/diagnostics/` and says `mcp` logs to stderr unless `VIOLA_DIR` is set
  (A:362, A:458); obs decided codes-only files at `<home>/diagnostics/` plus per-instance detail files, with `mcp`
  always writing `mcp.ndjson` (OB D-08, D-09, D-22; XF O4, O6, B10); obs originally created the diagnostics directory before
  the strict-modes check, which on Windows makes a fresh home fail it, with no ownership/mode/symlink check on existing
  diagnostics files (XF B2); obs fix pass 2 moves the strict-modes check first, but arch states no initialisation order.
- EXPECT: the architecture names both diagnostics roots and every process's sink; the home is created and passes the
  strict-modes check before any diagnostics file is created or opened; existing diagnostics files get the same
  owner, mode and symlink checks as other state files; content-bearing detail exists only per instance.
- Anchors: XF:62 (O4), XF:70 (O6), XF:82 (B2), XF:90 (B10); OB §3 OTel SDK init (init step 4), Logging stack (Sink), Log file location; OB D-08, D-09, D-22; A:362, A:458; S:207, S:498, S:534.

**F-18 · Every exit-21 and exit-1 cause has a machine-readable code**
- OBSERVED: exit 21 and exit 1 each have several causes; design now gives one hint per cause in human mode, but arch
  fixes `{"error":"instance-unreachable","detail":null}` for exit 21 and gives exit 1 no `--json` document (A:292-294;
  DS cli pattern 2 "pending an arch amendment"); obs's codes lack stale heartbeat and unwrapped name (OB §6 detail code catalog,
  D-20), and strict-modes or socket-directory refusals at `run`/`ui`/`mcp` start have no code (XF X7, T4, F4, B6).
- EXPECT: every cause of exit 21 (not running, unwrapped name, strict-modes, server verification, endpoint lost
  mid-call) and of exit 1 (already live, stale heartbeat, squatted name, pinned-hash mismatch, script child,
  strict-modes or socket-directory refusal, internal error) reaches `--json`, MCP and the process log as a distinct
  code with no path or pid, matching the human hint for the same cause.
- Anchors: XF:17 (X7), XF:36 (T4), XF:76 (F4), XF:86 (B6); A:134-145, A:292-294; DS cli pattern 2 and exit-code table; OB §6 detail code catalog, D-20; S:203, S:207; TP §6 Exit-cause matrix.

**F-19 · Toolchain floor 1.96**
- OBSERVED: arch declares `rust-version = "1.89"` (A:13, A:374, A:490); sysinfo 0.39.6 needs 1.95; security moves the
  host to ≥1.96 (S:320); tests and obs now use 1.96 (TP §9 MSRV row; OB D-22); the host still has 1.95 (XF T14, O5, I2,
  I3).
- EXPECT: one declared floor (1.96) is what the workspace states, what the MSRV CI job checks and what the pinned
  toolchain provides; the primary toolchain is one pinned current stable; external CLI tools are stated as minimum
  floors, not exact pins.
- Anchors: XF:46-47 (T14, T15), XF:63 (O5), XF:67-68 (I2, I3); S:320-321, S:339-342; OB D-18, D-22; H:116.

**F-20 · Obs additions to the channel and config contracts**
- OBSERVED: obs adds a `conn` field (`<process>-<pid>-<t0>-<n>`) to every channel request so calls can be joined
  across processes, and a `diagnostics_level` key (`info`|`debug`) to `config.json`; arch lists neither (XF O6).
- EXPECT: the channel contract names the optional connection discriminator, older peers without it still work, and
  it is never treated as identity; `config.json` names the diagnostics level, which changes volume only and can never
  disable redaction.
- Anchors: XF:70 (O6); OB §3 Trace context propagation (conn), D-10, D-15, D-32; A:111, A:446-450; S:556.

**F-21 · Timing constants named and located**
- OBSERVED: the send-confirmation window, the spine-hook deadline, the dialog deadline, the readiness gate's quiet
  period and maximum wait, and the MCP `wait` default timeout are open items (A:45-46, A:58, A:61, A:66); the perf
  gate runs on a provisional 1.0 s (TP §10 Performance budgets; OB §10).
- EXPECT: each of these values has one named home (a built-in value or a per-CLI-version ledger row measured by
  `viola verify`), tests drive them through an injected clock, the perf gate reads the named value, and every MCP
  call completes below the client's tool-call timeout.
- Anchors: TP §12 User review 1 (arch requests 1 and 3) and open questions; A:45-46, A:58, A:61, A:66; H:43-45.

**F-22 · `viola ui` on an OS-assigned port**
- OBSERVED: harness `boot` must pick a free port and release it, a race it reports as `ui-port-taken`.
- EXPECT: `viola ui` can start on an OS-assigned port and report the bound port through its launch file, so a
  headless boot never races for a port.
- Anchors: TP §12 (arch request 2); TP §3 `boot` step 6; A:320; S:197.

**F-23 · The architecture's tree includes the test and obs artifacts**
- OBSERVED: tests add `crates/viola-e2e` (tokio-based clients, `viola-harness`), a feature-gated `viola-fake-agent`,
  `e2e-web/`, `scripts/agent-run.*` and `fuzz/`; obs adds `schemas/`, a workspace `clippy.toml` and a
  `target/secret-scan/` report dir; a11y adds `e2e-web/*`, `a11y/sr-pass/` and `viola-ui/assets/index.html`; arch
  lists none of them and says no test crate is declared (XF T16, B12, Z15).
- EXPECT: the architecture's workspace, directory tree and dependency policy include every one of these, the tokio
  ban still holds for every sync crate, and the release build contains no test-only binary.
- Anchors: XF:48 (T16), XF:92 (B12); TP §2 directory conventions, TP §12 (test crate deviation); OB §3 Bootstrap phases (log-format-schema-emit, obs-ci-gate-wire), §8 secret-scan hit report; A:95, A:347, A:377, A:397-429.

**F-24 · The statusline source `run` records**
- OBSERVED: arch says `run` resolves the user's statusline command "from the user's effective Claude Code settings"
  without naming the source; tests cannot redirect it per home, so the budget pass-through path is blocked (TP §3
  `boot --statusline-echo`; TP §12 open questions).
- EXPECT: the source of the user's statusline command is named, is read-only to viola, and can be redirected per
  test home; the wrapped statusline prints the user's output unchanged.
- Anchors: TP §6 Path 6; A:69, A:305, A:344; B §5 O8; H:43-45.

**F-25 · The PTY dependency pin is decided on current evidence**
- OBSERVED: arch pins portable-pty `=0.8.1` (2023) and excludes 0.9.0 for a Windows garbage-read bug (A:44); a11y
  notes 0.9.0 is the maintained line and inherits the risk (AY D-A11Y-13; XF Y6).
- EXPECT: the chunk that writes the PTY seam records which portable-pty line it uses and why (the Windows defect
  re-checked), keeping the seam's contract, exit-on-handle detection and non-inheritable handles either way.
- Anchors: XF:101 (Y6); A:44, A:17; AY D-A11Y-13 (1328); S:254, S:502.

**F-26 · Cross-plan fix-pass items honoured by the implementing chunks**
- OBSERVED: three overseer fix passes (`693e083`, `85d6354`, `90c705d`) aligned the tests, design, layouts, obs
  and a11y masters with each other (XF T*, B*, Y*, Z1–Z14); each change is logged in its master's Decisions Log.
  Left to the implementing chunk: X8 budget-override gate display, X10 refusal class of `instance-unreachable`,
  X11 `release-from-driver` display, X13 401/launch copy.
- EXPECT: each chunk that implements one of these surfaces follows the ratified side, and its acceptance includes the
  corresponding test, so no shipped behaviour contradicts a master.
- Anchors: XF:18-23 (X8–X13), XF B*, Y*, Z* sections; the "overseer fix pass" entries in each master's Decisions Log.

### (c) The thin Windows slice: run / send / wait / answer with delivery confirmation

**F-27 · `viola run` wraps the unmodified CLI on Windows**
- OBSERVED: only the prototype hosts `claude` in a ConPTY; the product has no wrapper.
- EXPECT: `viola run <name> -- claude …` shows the child's screen and passes the human's keys exactly as without the
  wrapper, writing no bytes of its own to the terminal; it resolves the npm shim to the real executable and refuses a
  `.cmd`/`.bat` child, strips the parent session's identity from the child, pins a copy of itself plus a freshly
  written plugin that hooks call by absolute path, starts in the documented order (collision check … events … child
  spawn), refuses a name that is already live or stale, and detects child exit on the process, never on end of output.
- Anchors: B §3.2.1, R5, R8, §4.1 S1/S6, Windows details (216-220), §4.2 M6; A:44, A:90, A:96, A:354; S:235; TP §6 Path 1; AY §1 tui entity (94-100).

**F-28 · Hooks report normalised events and fail open**
- OBSERVED: the prototype's hooks fire with the wrapper's identity (B §4.1 S2); the product has none.
- EXPECT: every registered hook event becomes one normalised event line in the instance's log (never a raw Claude
  payload); harness-injected prompts are classified as harness, long-paste wrappers and tag escaping are normalised
  away; a hook outside a wrapped session is a silent no-op that writes nothing anywhere; any hook failure exits 0 with
  no output so the human sees the dialog; spine hooks never delay a turn past their deadline.
- Anchors: B R5, §4.1 S2, §4.2 M2–M4, M7; A:59-66, A:158-168, A:275-290, A:302-305; TP §6 E1; OB D-16.

**F-29 · `send` types at a turn boundary and is confirmed after the fact**
- OBSERVED: the prototype sends without a readiness gate; a CLI-native modal swallowed a paste (M1), local commands
  fire no prompt event (M5), and Git Bash rewrote a leading-slash argument (B §6).
- EXPECT: a driver's text (from stdin or a file, never a leading-slash argument, with a warning on a rewritten path)
  is typed as one paste plus Enter only when the input box is ready; it returns read back only when the matching
  prompt is seen, `not-delivered` with a typed detail otherwise (`input-not-ready`, `no-prompt-submitted`,
  `turn-running`, `control-character`), and `unconfirmable` only for a ledger-listed local command without a measured
  post-condition; `/clear` is confirmed by its new-session post-condition; text with a disallowed control character
  is refused, never stripped.
- Anchors: B R3, §4.2 M1, M3–M5, §6 (Git Bash); A:45-46, A:98, A:126-133, A:262; S:221, S:488-489; TP §6 Path 2.

**F-30 · `wait` and `last` wake on turn boundaries without polling**
- OBSERVED: the prototype's `wait` exits on the next Stop (B §4.1 S5); there is no cursor-based contract.
- EXPECT: `wait` returns the first driver-relevant event (turn ended, question, permission, plan, session end) at or
  after a cursor — at once if it is already logged, never woken by activity, wheel, budget or send-record lines —
  times out with a typed result, reports an unreachable instance when the wrapper vanishes, and `last` returns the
  newest turn's final text; every send and wait result carries the cursor for the next call.
- Anchors: B §4.1 S5, §6; A:49, A:263-264; TP §6 Path 3.

**F-31 · Dialogs answered through hooks by `dialog_id`**
- OBSERVED: the prototype answers questions (including free text and annotations), permissions and plans through
  hooks (B §4.1 S3, S7, S8).
- EXPECT: exactly one dialog is pending per instance and appears once in the log with its id; a driver answers a
  question (option or free text, with notes), a permission (allow, or deny with a message, plus a suggestion if F-16
  admits one) or a plan (approve, or revise with a message) and the dialog never renders; a second concurrent dialog,
  a human wheel, an unverified CLI or an expired deadline all leave the dialog to the human; an unknown id is refused
  `unknown-dialog`.
- Anchors: B R7, §4.1 S3/S7/S8; A:49, A:66, A:265-272, A:290; TP §6 Path 4.

**F-32 · The wheel: the human always wins, automation is refused with a reason**
- OBSERVED: the prototype took harness turns for human takes twice before a classifier was added (B §4.2 M2); the
  Pulse live test showed take / refuse / release working (B §6).
- EXPECT: any human editing key since the last turn boundary moves the wheel to the human (focus, mouse and resize do
  not), human keys are never blocked beyond the current paste, a pending dialog is handed back at once, harness turns
  never flip the wheel, `pause` takes it without a keystroke, and only a human `release` returns it; driver sends and
  answers are refused `human-typing` (with `manual-pause` after `pause`) and no driver-facing hint suggests `release`.
- Anchors: B R2, §4.2 M2, §6; A:5, A:67; S:206; XF X6, T3; TP §6 Path 5; AY P4 (785-789).

**F-33 · The slice passes the first live test on Windows, then viola drives its own build**
- OBSERVED: the first live test passed only on the prototype (B §6); the overseer still drives the builder through
  the prototype binary (H:23-26).
- EXPECT: with the product binary, an overseer session sends a pipeline skill to a builder session, waits for the
  turn, reads the final text, answers a review, clears between skills, and the founder's typing takes the wheel;
  after that the overseer switches its own driving from the prototype to viola.
- Anchors: B §6 (283-301); H:73-77; A:41.

### (d) Remaining v1 capabilities

**F-34 · Capability ledger, `viola verify`, and transport-only degrade**
- OBSERVED: every load-bearing CLI behaviour was measured once on 2.1.280 and the docs lag the build (B §4); the
  product has no ledger.
- EXPECT: each relied-on CLI behaviour is a versioned ledger row with a live probe and a post-condition; `viola
  verify` probes the local CLI, stamps the version, records scrubbed fixtures and prints a step counter ending in a
  pass/fail summary; on an unstamped version viola still types, holds the wheel and logs events but withholds every
  dialog answer and refuses drivers `unverified-cli`.
- Anchors: B §4, §4.1-4.2; A:70-88, A:455; S:554; TP §6 Path 7; LT cli `viola verify`.

**F-35 · Budget governor from the statusline**
- OBSERVED: the plan-limit source was found (statusline `rate_limits`, B §5 O8); the policy page makes a governor a
  first-version item (B §7).
- EXPECT: wrapped sessions record usage readings without touching the user's own statusline configuration or output;
  crossing a threshold (defaults five-hour 90 %, seven-day 85 %) refuses automated sends `budget-paused` while running
  turns and human keys continue; the pause lifts at the crossed window's reset or by a per-instance override;
  missing readings show as `unknown` with their age and never block.
- Anchors: B §5 O8, §7 (volume); A:69, A:305, A:363; TP §6 Path 6; DS web-spa component 5 (ATIS).

**F-36 · Links between sessions**
- OBSERVED: D5 asks to see which sessions are linked; nothing records links.
- EXPECT: a send or answer from inside a wrapped session to another instance records a `driver → driven` link once,
  `link`/`unlink` add or remove one explicitly, and the link set (with its first-linked time) is derived from the log
  and shown by the board and the page.
- Anchors: B D5, §1 (third quote); A:68, A:212, A:271; TP §6 E3.

**F-37 · The board: `viola list` and session liveness**
- OBSERVED: `claude agents --json` lists every session but knows nothing of wheels, dialogs or links (B §4).
- EXPECT: `list` works with no live wrapper, shows each wrapped session's liveness (live / stale; gone omitted),
  idle/busy status, wheel holder with any budget pause, pending dialog kind and CLI version verdict, plus unwrapped
  sessions as read-only rows that are never valid targets; human output is the fixed six-column board with the BAY
  line, every row single-line with controls (including newline and tab) escaped, and `--json` equals the GUI sessions
  envelope.
- Anchors: B R6, §4; A:90, A:204-216, A:273, A:307-309; DS cli pattern 1; LT cli `viola list`; TP §6 `list` row integrity, Cross-surface parity; XF T5.

**F-38 · The MCP server for drivers**
- OBSERVED: the prototype driver uses the CLI through Bash; there is no MCP surface.
- EXPECT: a wrapped driver session gets `send`, `wait`, `last`, `answer` and `list` as MCP tools with the same results
  and refusals as the CLI (refusals as error results carrying codes only), no wheel or link controls on the tool list,
  inputs re-validated server-side regardless of advertised schemas, and every call returning below the client's
  tool-call timeout.
- Anchors: B §3.2.3; A:51-58, A:146, A:296-300; S:228, S:293; TP §1 MCP surface, TP §6 Cross-surface parity.

**F-39 · CLI contract: machine output, typed exits, cause-named hints**
- OBSERVED: the prototype prints free text; drivers need parseable refusals (A:98).
- EXPECT: every verb has a `--json` form mirroring the channel result and a typed exit code (0, 1, 2, 10–14, 20,
  21); human output follows the design phraseology (readback mirror for `send`, one `hint:` naming the next step per
  cause, results on stdout and context on stderr); a global `--home` isolates all state; `config.json` sets
  thresholds, port and diagnostics level, and no setting can switch off a security control.
- Anchors: A:98, A:134-145, A:171, A:292-294, A:446-450; DS Surface: cli; LT Surface: cli; S:556; TP §5 CLI.

**F-40 · CLI and terminal output discipline**
- OBSERVED: a11y makes no conformance claim for terminal surfaces but binds output discipline for them: no spinner or
  redraw, colour only beside its word, controls escaped, and the wrapped TUI untouched (AY P4, P6).
- EXPECT: on all three OSes, CLI output is linear and static (one `waiting:` line on a TTY only, no carriage-return
  redraw or cursor movement), no colour or glyphs under non-TTY, `NO_COLOR`, `TERM=dumb` or `--json`, every colour
  span sits beside its word (amber only on `DIALOG`, dim only on stale rows), injected control characters in
  `wait`/`last`/`list` output appear only escaped, `verify` prints appended step lines, `ui` prints its launch line
  once; under `viola run` viola adds zero bytes to the terminal, human keys are delivered unblocked during a driver
  send, and focus, mouse and resize sequences never move the wheel.
- Anchors: AY §1 cli/tui entities (88-100, 163-175), §4 P4 (785-789) and P6 (802-814), §5 (823-866), §9 Unit/integration row, §10 (CLI invariants); DS cli colour decision order, Anti-Patterns (cli); S:388, S:532.

**F-41 · Crash-safe, self-healing, version-tolerant state**
- OBSERVED: prior-art tools broke on shared state files (B prior-art §4); viola's state must survive a crash on
  either side (B R4).
- EXPECT: logs are append-only one-line records never truncated, snapshots are replaced atomically, a torn last line
  or an unreadable/newer snapshot is healed by replay (recovering only log-derived fields), unknown kinds and fields
  are counted and shown rather than failing, and a newer peer's request is refused with the supported version.
- Anchors: B R4; A:47-48, A:93, A:111-112, A:238-249, A:454, A:457; TP §6 E5, Chaos suite; OB D-03 (`state-recovered`).

**F-42 · Optional plugin install for unwrapped drivers**
- OBSERVED: arch defines `viola plugin install` for the MCP verbs in an unwrapped driver; its interplay with the
  `--plugin-dir` plugin is an unmeasured ledger row (A:97).
- EXPECT: an unwrapped driver can install viola's MCP tools without hooks, calling viola by absolute path; a
  session that has both never runs two hook processes for one event, and which MCP server wins is measured.
- Anchors: A:84, A:97, A:361; S:553.

### (e) Security hardening after the slice (two items block specific chunks)

**F-43 · Chunk-blocking security prerequisites**
- OBSERVED: two open questions have no answer: whether a SQOS-opened Windows pipe handle can be adopted by the IPC
  library, and which pure-Rust SHA-256 implementation passes the C-build ban (S:589-592; H:36-37, H:130-131).
- EXPECT: the SQOS spike is resolved (or its replacement recorded) before the chunk that writes the channel client,
  and the SHA-256 choice is recorded before the chunk that writes `bin/`; a failed spike never falls back to an
  unprotected connect.
- Anchors: S:205, S:265, S:366-368, S:590-592; H:36-37, H:130-131.

**F-44 · The IPC endpoint admits only the same OS user and verifies its server**
- OBSERVED: the endpoint is a code-execution interface with predictable names in shared namespaces (S:21, S:55-71).
- EXPECT: on each OS only the owning user (and SYSTEM on Windows) can connect; remote pipe clients and other users'
  peers are rejected; every client verifies it is talking to the recorded wrapper process before writing any frame,
  and on mismatch CLI/MCP report unreachable while hooks fail open; a squatted name blocks start rather than being
  taken over; macOS's missing pid check is recorded as a known gap.
- Anchors: S:202-205, S:472-478, S:593-594; A:466; TP §1 Coverage triggers (IPC), TP §6 security control negatives, Exit-cause matrix.

**F-45 · The viola home and its code-bearing files keep their integrity**
- OBSERVED: snapshot, stamps, plugin files, settings and the pinned binary decide what code runs (S:21, S:30-40).
- EXPECT: the home is created owner-only, every process that trusts a state file first refuses a home or file
  writable (or readable, where content lives) by others, the pinned binary is re-hashed before reuse, plugin and
  settings files are rewritten at every start, only `viola verify` writes stamps and only the owning wrapper writes a
  snapshot, and the statusline command never runs from a home that fails these checks.
- Anchors: S:207, S:262-266, S:498-500, S:553-556; TP §1 Coverage triggers (filesystem), TP §6 Exit-cause matrix and security control negatives.

**F-46 · Bounded, validated inputs at every boundary**
- OBSERVED: arch validation stops at JSON-RPC parsing, the version check and the name newtype (S:70).
- EXPECT: every external reader is size-capped, depth-limited and closed-typed; names are validated before any path
  join; a bad `Last-Event-ID` pair drops the whole header; the paste rule is enforced client-side and authoritatively
  in the wrapper; a terminal-parser panic degrades to `input-not-ready` without harming passthrough.
- Anchors: S:219-240, S:483-493; TP §1 Coverage triggers (paste injection, property-test); TP §6 Property suite.

**F-47 · Sanitised errors and the never-log floor**
- OBSERVED: nothing prevents paths, upstream text or secrets reaching error output.
- EXPECT: CLI `--json`, MCP errors, Problem Details and channel errors carry only codes and fixed messages; no log,
  diagnostic, fixture, event or a11y artifact ever holds the GUI token, cookie, launch URL or stripped `CLAUDE*`
  values; content and drift reports go only to per-instance detail files readable by the owner; a secret scan after
  every E2E test proves it.
- Anchors: S:378-389, S:438-461, S:519-535; TP §6 Error sanitization and secret scan; OB §8 PII Scrubbing; AY §3 Structured violation JSON schema (scrubbing).

### (f) Web UI per design, layouts and a11y

**F-48 · `viola ui` serves a loopback, view-only, authenticated feed**
- OBSERVED: no GUI exists; the brief wants sessions, links and recent events (B §3.2.4).
- EXPECT: `viola ui` binds loopback only, rejects any Host but its own, serves only GETs (others 405), answers
  health/readiness/info, the sessions and links envelopes and a resumable SSE feed of verbatim event lines, gates all
  content behind a per-launch token exchanged once for a strict same-site cookie, sets the security headers and CSP on
  every response, never compresses the feed, serves embedded assets only, logs request paths without query strings,
  and prints its launch line once.
- Anchors: B §3.2.4, §3.4; A:91, A:113-119, A:184-230, A:320-328; S:197-201, S:281-293; TP §6 E4, Security sweep; OB §1 web-spa surface.

**F-49 · The strip-bay page**
- OBSERVED: design and layouts fully specify the page; a11y replaces design's ARIA-on-custom-element rows with native
  tables, adds `<h1>`/`<h2>` headings and banner/main landmarks, and silences the tape's implicit live region (AY
  D-A11Y-02, D-A11Y-03; XF Y1, Y2); nothing is built.
- EXPECT: one page shows the ATIS header (budget figures with age, gate, tape and skipped state), a WRAPPED rack and
  an UNWRAPPED · READ-ONLY rack of fixed-field strips in stable name order as native tables, a cocked strip for a
  pending dialog, transfer markers for links, and the tape (a log that never announces arrivals) with a readback box
  on every send that opens, fills or strikes only on the logged outcome; all upstream text renders as text; no
  horizontal scroll at the supported widths on the Linux font fallback; no spinner or loading text; out-of-view
  attention through the title and polite announcements; focus never moves or is lost on live updates or trimming; the
  CLI board and the page agree cell for cell.
- Anchors: DS Brand Identity, Surface: web-spa, Anti-Patterns (web-spa); LT Surface: web-spa; AY §4 (706-821), §5 (823-866), §7 (965-1025), D-A11Y-02/03/07/17; TP §6 Bay layout states, Cross-surface parity.

**F-50 · Automated accessibility verdict on every page state**
- OBSERVED: a11y binds axe inside the tests' Playwright driver with a fixed rule configuration, plus keyboard,
  aria-tree, screen-reader-proxy, html-validation and lint checks, all reduced to violation rows (AY §3); none exists.
- EXPECT: every web-spa state (steady, narrow, empty, expanded line, scrolled up, trimmed, cocked, refused,
  unconfirmable, human wheel, degraded, access) passes the a11y configuration with zero violations and zero
  contrast-incomplete items; the keyboard sequence equals the oracle with no trap and focus never obscured by the
  sticky header; only the defined messages are announced, each once; every applicable success criterion has at least
  one passing tagged test; lint errors in the page templates fail the build; violation rows land only in test
  artifacts, scrubbed, with no CSP or Trusted Types violation during any check.
- Anchors: AY §3 A11y testing tool pick (484-507), WCAG criteria mapping (510-566), Structured violation JSON schema (568-599), Focus/Keyboard/Screen reader harness (601-638), CI integration (658-670), Bootstrap phases (672-704); AY §9 (1101-1134), §10 (1136-1168), D-A11Y-01, -09, -14; XF Y3, Y5; TP §6 Selector strategy (axe).

**F-51 · Token-pair contrast, forced colours and reduced motion hold**
- OBSERVED: design fixes eight colours and computed contrast by hand, with three inks failing AA on the holder
  surface (DS Color Palette contrast rules); a11y binds a token-pair checker and forced-colours/reduced-motion checks
  (AY §6).
- EXPECT: every design token pair used for text meets 4.5:1 and every non-text state pair (rules, strike, cock band,
  focus ring) meets 3:1 on the rendered Linux fallback; no failing ink is ever placed on the holder surface; under
  forced colours every state border, strike and band stays distinguishable, and under reduced motion both motions
  drop to zero while the cocked position and band remain.
- Anchors: AY §3 Contrast verification harness (640-656), §6 Visual Design Verification (868-963), D-A11Y-10, -18, -19; DS Color Palette, Motion, Surface: web-spa (forced colors); XF Y4.

**F-52 · The 401 access strip after a UI restart**
- OBSERVED: after `viola ui` restarts, the open page's SSE gets 401 but design shows `TAPE stopped · not answering`
  instead of the access strip (XF X9); a11y requires the access strip, announced, with a plain recovery line (AY P5;
  a11y overseer direction 4).
- EXPECT: a page whose cookie no longer matches the running `viola ui` shows the access strip in place of the stopped
  tape, announces it once, tells the user in plain words how to recover (the launch line, the launch file, or a
  restart), and never shows or echoes the token, URL, `?t=` or file path.
- Anchors: XF:19 (X9); AY §4 P5 (790-801), §8 Error recovery (1045-1058), D-A11Y-06, resolved question "401 access strip copy"; DS web-spa component 6; S:197-200, S:455.

### (g) Cross-OS completion

**F-53 · Linux and macOS reach parity with Windows**
- OBSERVED: every live measurement is Windows-only (B §5 O5); the Unix endpoint location, modes and peer checks
  differ per OS; a Linux live run is suggested before the first Unix chunk (H:180-181).
- EXPECT: on Linux and macOS the same end-to-end suite passes against the fake agent (openpty, per-user socket
  directory, file modes, peer and server checks, macOS gap recorded), Windows remains the live-supported target, and
  one live `viola run` on Linux confirms the fake agent's Unix behaviour before the Unix-specific chunk lands.
- Anchors: B D3, §5 O5; A:7, A:40, A:459-464; S:203-205, S:593-594; TP §1 Coverage triggers (multi-os-compat), TP §9 Matrix builds; H:180-181.

## Coverage check

| Master · section | Findings |
|---|---|
| B §1 problem, §2 D1–D5 | framing, F-27, F-33, F-36, F-48, F-53 |
| B §3.1 constraint, §3.2 parts 1–4 | F-27, F-28, F-29, F-38, F-48 |
| B §3.3 R1–R8 | F-27 (R5, R8), F-28 (R5), F-29 (R3), F-31 + F-16 (R7), F-32 (R2), F-41 (R4), F-37 (R6), F-47 (R1) |
| B §3.4 stack | F-01, F-03, F-27, F-44, F-48 |
| B §4 measured, §4.1 S1–S8, §4.2 M1–M7 | F-27 (S1, S6, M6), F-28 (S2, M2–M4, M7), F-29 (M1, M3–M5), F-30 (S5), F-31 + F-15 + F-16 (S3, S7, S8), F-32 (M2), F-34 |
| B §5 O5, O6, O8 | F-53, F-46 + F-09 (7), F-35 |
| B §6 first live test + Git Bash | F-29, F-33 |
| B §7 policy, security, a11y | F-35, F-43–F-47, F-50 |
| A Design Philosophy, Stack | F-01, F-19, F-25, F-27, F-41 |
| A Established Decisions (PTY … CI) | F-27–F-42, F-14, F-23, F-25 |
| A Conventions (versioning, refusals, exits, naming) | F-39, F-41, F-29, F-18, F-20 |
| A Standard Contracts (GUI, SSE, events, snapshot, channel, hook, liveness) | F-48, F-41, F-28, F-30, F-31, F-37, F-10, F-11, F-13, F-16, F-20 |
| A Occupied Resources / Infrastructure / Cross-cutting | F-01, F-09, F-17, F-22, F-23, F-27, F-45 |
| S Auth & Authz, Input Validation, Data Protection, API Security | F-09, F-43–F-46, F-48, F-29 |
| S Dependency Security, Bootstrap phases, Error Handling, Anti-Patterns | F-01, F-19, F-47, F-43 |
| S Decisions Log (8 amendments, 2 open items, accepted risks, macOS gap) | F-09, F-43, F-44 |
| DS web-spa (tokens, components 1–7, navigation) | F-49, F-51, F-52, F-10, F-11, F-12 |
| DS cli (list, send mirror, wait/last, verbs, verify/ui/run, exit phraseology) | F-37, F-39, F-40, F-29, F-34, F-18 |
| LT web-spa screens, components, IA | F-49, F-11 |
| LT cli output structures | F-37, F-39, F-13 |
| TP §1 scope, critical paths 1–7, triggers | F-27, F-29–F-32, F-35, F-34, F-07 |
| TP §3 harness, log format, bootstrap phases | F-02, F-04, F-03 |
| TP §6 E1–E5, security sweep, negatives, exit-cause matrix, parity, chaos, contract | F-28, F-27, F-36, F-48, F-41, F-18, F-37, F-03, F-44, F-45 |
| TP §9–§10 CI, gates, mutation, perf | F-01, F-06, F-07, F-19 |
| TP §12 arch requests and open questions | F-14, F-21, F-22, F-23, F-24 |
| OB §1 scope, §2 strategy | F-04, F-05, F-48 |
| OB §3 harness (init order, logging stack, log format, file location, trace context, heartbeat, bootstrap) | F-04, F-05, F-17, F-20, F-11 |
| OB §4–§6 spans, metrics, log coverage (detail codes) | F-04, F-18 |
| OB §7–§8 error capture, PII scrubbing | F-05, F-47 |
| OB §9–§10 CI gates G1–G4, SLOs, perf | F-01, F-05, F-07 |
| OB §12 D-08/09/10/15/18/20/22/32 (arch requests) | F-17, F-18, F-19, F-20 |
| AY §1 scope (web-spa, cli, tui entities) | F-49, F-50, F-40 |
| AY §3 harness (axe, WCAG map, violation rows, focus/keyboard/SR, contrast, CI, bootstrap) | F-50, F-51 |
| AY §4–§5 ARIA patterns, paths P1–P6, keyboard | F-49, F-50, F-40, F-52 |
| AY §6 visual verification | F-51 |
| AY §7–§8 screen reader, cognitive (error recovery) | F-49, F-50, F-52 |
| AY §9–§10 CI, SLOs | F-50, F-40 |
| AY §12 D-A11Y-02/03/06/09/13/14/17/19 | F-49, F-50, F-52, F-25, F-51 |
| XF design audit X1–X13 | F-10 (X1), F-11 (X2), F-12 (X3), F-13 (X4, X5), F-32 (X6), F-18 (X7), F-26 (X8, X10, X11, X13), F-52 (X9), F-37 (X12) |
| XF tests audit T1–T17 + negatives | F-10 (T2), F-14 (T8), F-15 (T9), F-19 (T14, T15), F-23 (T16), F-18 (T4), F-26 (closed items), F-44/F-45 (negatives) |
| XF obs O1–O6 | F-04 (O1–O3), F-17 (O4, O6), F-19 (O5, O6), F-20 (O6) |
| XF intent draft I1–I3 | F-16 (I1), F-19 (I2, I3) |
| XF after fix pass F1–F4 | F-01 (F1, F2), F-13 (F3), F-18 (F4) |
| XF obs vs upstreams B1–B12 | F-01 (B1, B4, B7), F-17 (B2, B10), F-26 (B3, B5, B8, B9, B11), F-05 (B5), F-18 (B6), F-23 (B12) |
| XF a11y Y1–Y6, Z1–Z15 | F-49 (Y1, Y2), F-50 (Y3, Y5), F-51 (Y4), F-25 (Y6), F-26 (Y1–Y5 fix pass), F-01 (Z8), F-23 (Z15), F-26 (Z1–Z14 fix pass) |
| H founder directions | F-01, F-02, F-04, F-06, F-08, F-33, ordering (a)–(g) |

**Deliberately left out (not 0.1.0 capabilities):**
- API-key / Agent SDK hosting and any credential handling — excluded by D2 and the policy page (B §2, §7).
- GUI brake routes (`POST …/pause`, `…/unlink`), their controls, cross-origin check and Button/dialog a11y contracts — reserved for v1.x (A:91, A:328; DS web-spa component 7; AY D-log "Deferred"); v1 asserts only 405.
- Phone / remote view and any layout below 760 px, including SC 1.4.10 below 760 — later version behind authentication (A:101, A:486; AY D-A11Y-04).
- Public distribution: dist releases, signing, winget/Homebrew/Scoop, marketplace entry, self-update — v1.x (A:36, A:105, A:485; S:601-606).
- A second driven agent and the adapter trait — until a second agent exists (A:42, A:481).
- OTel SDK, OTLP exporter, browser telemetry and web-vitals — deferred by obs (OB D-12, D-17).
- Real screen-reader runs (NVDA/VoiceOver/Orca, Guidepup) — founder-local supplemental records, never a gate (AY D-A11Y-12).
- `events.ndjson` retention/bounding, `diagnostics/` rotation and pinned-copy cleanup — open items with no v1 scheme; re-carry as residuals (A:359, A:362; S:591; OB §3 Log file location).
- Driver-side decision policy (Appendix A) — the consumer's policy, not viola's (B App. A; A:41).
- Unmeasured dialog paths: "Chat about this" (O7), plan "approve with feedback" and `multiSelect` answers — no arch decision; candidates for ledger probes in a later version.
- Cross-session messaging on native Windows (O3) and `initialUserMessage` seeding (O4) — viola builds neither (B R6).
- Workspace-trust prompt in a fresh folder (O2) and the O1 stray-spaces artifact — watch items; the PTY swap trigger stays in arch (A:44).
- The Opus 5.5 cost saving — context, not a requirement (B §1).
