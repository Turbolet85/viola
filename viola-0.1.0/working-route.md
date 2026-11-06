# Working Route — viola-0.1.0

### Epoch 1 — Foundation

[2026-09-24-three-os-ci-headless-harness-skeleton] Three-OS CI and headless harness skeleton — SHA-pinned actions, pinned 1.96-floor toolchain, nextest, five agent-run commands, minimal fake agent, per-role JSON line, mutation gate
[2026-09-24-fake-agent-and-test-data-fixtures] Fake agent and test-data fixtures — scripted modes and control file, receipts, temp-home fixture chain under target/e2e-home, fixture scrub-and-schema walk, committed property seeds
[2026-09-24-supply-chain-and-workflow-gates] Supply-chain and workflow gates — cargo-deny families with telemetry-crate and redaction-toggle bans, weekly advisory run, zizmor, tokio-free sync-crate check, least-privilege permissions
[2026-09-24-diagnostics-plane] Diagnostics plane — per-role JSON sinks with process-start service identity, closed event vocabulary and line schemas, owner-only per-instance detail files, diagnostics_level, torn-line-aware logs merge
[2026-09-24-log-redaction-and-never-log-floor] Log redaction and never-log floor — skip-all span fields, redacted payload types, fixed error displays, content-bearing records only in instance detail files
[2026-09-24-observability-gates] Observability gates — panic hook first, zero-panic, schema, bare-instrument and abort-panic gates, canary secret scan before any upload, print and raw-log lint bans
[2026-09-24-quality-gates] Quality gates — fatal fmt/clippy lint, MSRV 1.96 job, per-OS coverage floors, seeded property and fuzz corpus replay, zero retries, per-job gate verdict
[2026-09-24-workspace-tree-and-code-graph-planes] Workspace tree and code-graph planes — arch tree lists test/obs/a11y artifacts, release build free of test binaries, Rust plane ok, TypeScript plane decided

### Epoch 2 — Windows slice I: wrapper, events, ledger

[2026-09-24-epoch-1-cleanup] Epoch 1 cleanup — MAX_FRAME value witnessed by a test, harness and fake-agent functions within the cognitive ceiling, run.rs under 800 lines, test clone pairs gone
[2026-09-25-security-prerequisites] Security prerequisites — SQOS pipe-handle spike and pure-Rust SHA-256 pick, both recorded ahead of the channel client and bin/ writes; no unprotected-connect fallback
[2026-09-25-pty-wrapper-on-windows] PTY wrapper on Windows — viola run passthrough with zero own bytes, npm-shim resolution, .cmd refusal, CLAUDE* strip, process-handle exit, portable-pty line recorded
[2026-09-26-ci-chunk-base-and-union-verdict] CI chunk base and union verdict — whole-chunk CI base surviving the operator pass, per-leg union mutation verdict, owned secret-scan residue scope
[2026-09-26-local-linux-pre-push-gate] Local Linux pre-push gate — WSL2 Ubuntu at CI's pins, synced Linux-filesystem clone of the working tree, Unix tests and ubuntu mutation leg before every push
[2026-09-27-instance-state-and-start-order] Instance state and start order — append-only event log, atomic snapshot, heartbeat, pinned bin/ copy, plugin folder, documented start order, live/stale name refusal
[2026-09-27-wrapper-channel] Wrapper channel — JSON-RPC 2.0 ndjson over the per-instance named pipe (Unix: per-user socket directory), SQOS-opened client, v/sender/conn in every frame, MAX_FRAME, newer-peer refusal

### Epoch 2b — Windows slice I b: events and ledger

[2026-09-27-epoch-2-cleanup] Epoch 2 cleanup — four files under 800 lines, clone pairs gone, channel/state/pty survivors killed, mutation scratch off C:, harness loop aids, machete annotation
[2026-09-27-browser-verdict-reachability] Browser verdict reachability — stub page, Playwright on three CI OSes and the WSL pre-push, pipe to a recorded green verdict; capability reachable, not real assertions
[2026-09-27-hooks-to-normalised-events] Hooks to normalised events — exec-form plugin hooks by absolute path, hook-to-kind map, harness/paste/tag normalisation, silent unwrapped no-op, fail-open exit 0, perf-job-gated hook deadlines
[2026-09-28-hook-perf-gate] Hook perf gate — hyperfine-gated hook deadlines per OS in CI, forced-panic fail-open case on the real binary, over-4 KiB concurrent detail line
[2026-09-28-cli-output-tokens] CLI output tokens — SGR attention/stale/callsign tokens with depth fallback, colour decision order, stdout/stderr split, plain under non-TTY/NO_COLOR/TERM=dumb/--json, escaped controls
[2026-09-28-capability-ledger-and-viola-verify] Capability ledger and viola verify — versioned rows with probes and post-conditions, stamps, scrubbed fixture recording, largest-payload row, transport-only degrade, fake-agent verify in CI
[2026-09-28-mutation-testing-to-the-epoch-boundary] Mutation testing to the epoch boundary — per-chunk mutation gate retired: pre-push mutation leg, CI mutation jobs and mutants-verdict removed; cargo-mutants kept for code-audit
[2026-09-29-h2-conpty-resize-probe] H2 ConPTY resize probe — the key lost within ~50 ms of a ConPTY resize reproduced on Windows a set number of times, localised, then fixed or documented
[2026-09-29-verify-stamped-test-homes-and-harness] Verify-stamped test homes and harness — test homes and harness boot stamped through viola verify, recorded-fixture readiness, contract ledger probes, local-live run, fake agent at the recorded version
[2026-09-29-sideloaded-conpty] Sideloaded ConPTY — Microsoft NuGet conpty.dll and OpenConsole.exe beside the pinned viola.exe, version and hash pinned, signature checked, loaded only from that path
[2026-09-29-fake-agent-drift-contract] Fake-agent drift contract — fake agent's hook sequences and payloads equal the recorded verify fixtures per CLI version, annotations forwarded, fixture schema check

### Epoch 3 — Windows slice II: driving verbs and live proof

[2026-10-02-epoch-2b-cleanup] Epoch 2b cleanup — in-repo suite red diagnosed and fixed, coverage re-taken, eight mutation survivors killed, viola-e2e and cfg(unix) mutants scored, test tempdir leaks closed
[2026-10-03-mutation-scoring-completion] Mutation scoring completion — viola-e2e harness and twelve cfg(unix) mutants scored natively at the boundary tier; pre-push and gate tools moved to the Linux host
[2026-10-04-windows-boundary-mutation-workflow] Windows boundary mutation workflow — cfg(windows) mutants scored on the windows runner by a manually dispatched, non-blocking workflow at each epoch boundary
[2026-10-04-readiness-gate-and-timing-constants] Readiness gate and timing constants — screen-model quiet period, input-box and modal signatures as ledger rows, named confirmation window and deadlines, injected clock, parser-panic degrade
[2026-10-04-confirmed-send-with-cl-1-records] Confirmed send with CL-1 records — RB readback mirror with per-reason hints, typed not-delivered details, unconfirmable local commands, /clear post-condition, control-character refusal, one in flight
[2026-10-04-wait-and-last] wait and last — cursor-based wake on driver-relevant events, already-logged events returned at once, typed timeout, unreachable on vanished wrapper, newest turn's text
[2026-10-04-dialog-answers-by-dialog-id] Dialog answers by dialog_id — question, permission and plan through hooks, plan answered as plan, permission-suggestion decision, one pending dialog, deadline expiry, unknown-dialog
[2026-10-04-the-wheel] The wheel — human editing key takes it, focus/mouse/resize never do, held bytes during paste, harness turns ignored, pause and release, human-typing/manual-pause refusals, release-from-driver
[2026-10-04-running-turn-refusal] Running-turn refusal — prompt-submitted marks a turn running, turn-ended/session-start/session-end end it, automated send refused turn-running meanwhile, release clears it
[2026-10-05-real-cli-verify-probes] Real-CLI verify probes — typed-input PTY probe against the installed claude, screen signatures and timing, local-command, dialog-body and paste-framing ledger rows, measured and stamped
[2026-10-05-dialog-rows-and-re-probe] Dialog rows and re-probe — S3/S7/S8 and dialog-concurrency ledger rows with their own interactive verify re-probe, the decision taking effect, the permission end-to-end case
[2026-10-05-permission-end-to-end] Permission end to end — the permission kind's end-to-end case and wake witness, and the PermissionRequest body for a question first raised there
[2026-10-06-local-command-and-paste-framing-rows] Local-command and paste-framing rows — local-command post-conditions and unconfirmable, long-paste wrapper, tag-escaping, harness-prefix and R8 identity-floor ledger rows, each measured by typed probe
[2026-10-06-local-command-send-outcomes] Local-command send outcomes — send's unconfirmable outcome with its mirror line, and /clear confirmed by its new-session post-condition
[2026-10-07-test-homes-off-the-contended-volume] Test homes off the contended volume — on the dev host a test start no longer stalls behind other builders' writes, the homes' path and CI's own fsync unchanged
[2026-10-07-live-rows-and-paste-shapes-on-the-dev-host] Live rows and paste shapes on the dev host — the harness-prefix and identity-floor rows, three paste shapes and the paste-hint window, measured on the real CLI
[2026-10-07-send-waits-out-the-paste-hint] Send waits out the paste hint — on a verified CLI a send issued while the paste hint stands is delivered once the input box returns, within 8.5 s
[2026-10-07-a-send-ending-in-a-newline-is-confirmed] A send ending in a newline is confirmed — a driver's text whose last byte is a newline is reported delivered when it is, and the driver keeps the wheel
[2026-10-08-first-live-test-and-self-drive] First live test and self-drive — overseer drives a builder through viola, founder takes the wheel, overseer drops the prototype

### Epoch 4 — Session state & governance

[2026-10-09-epoch-3-cleanup] Epoch 3 cleanup — test scaffolding shared once, three functions within the cognitive ceiling, viola-e2e scored, eighteen Linux survivors disposed, trailing CR confirmed, newline-only send refused at once
[2026-10-09-inner-cr-and-crlf-in-a-sent-text] Inner CR and CRLF in a sent text — typed as the LF the CLI submits, so the send is confirmed and the driver keeps the wheel
[2026-10-09-epoch-3-cleanup-ii] Epoch 3 cleanup II — test scaffolding shared once, three functions within the cognitive ceiling, viola-e2e whole-unit score, eighteen Linux survivors disposed
[2026-10-10-windows-mutation-grade] Windows mutation grade — the strict-modes and DACL checks pinned by Windows-side refusal tests, host-excluded twins left out of the count, every job inside its ceiling, workflow green
[2026-10-10-self-healing-state] Self-healing state — torn-line healing, snapshot rebuild by log replay, unknown kinds and fields counted and surfaced, newer-snapshot and newer-peer tolerance
[2026-10-10-viola-revive] viola revive — resume a dead instance in place by its newest logged session id in the recorded cwd, no launch replay, four preflight refusals
[2026-10-10-statusline-pass-through] Statusline pass-through — named read-only source, per-home redirect for tests, settings.json rewritten each start with absolute pinned path, user output unchanged, readings to budget.json
[2026-10-10-budget-governor] Budget governor — five-hour/seven-day thresholds, budget-paused on automated send only, lift at reset or per-instance override, unknown with age, release --budget payload
[2026-10-10-session-links] Session links — driver-to-driven link on first cross-instance send or answer, explicit link/unlink, log-derived link set with since, link result payload
[2026-10-11-the-board-viola-list] The board: viola list — BAY header over six-column board, liveness, status, wheel/budget pause, amber DIALOG, dim stale rows, CLI verdict, read-only unwrapped rows

### Epoch 5 — Driver surface

[2026-10-13-epoch-4-cleanup] Epoch 4 cleanup — Epoch 4's mutation survivors disposed, the Windows mutation baseline green on the root package, send and human output split by phase, Windows-target lint gated
[2026-10-13-epoch-4-survivors] Epoch 4 survivors — the Epoch 4 audit's 47 mutation survivors each disposed: an owning-package test, a behaviour-preserving restatement, or a not-measured-here record by coordinate
[2026-10-13-epoch-4-survivors-ii] Epoch 4 survivors II — the Epoch 4 audit's 16 `typed.rs` survivors in `viola verify`'s Run B driver disposed: an owning-package test after a behaviour-preserving restatement, or a not-measured-here record
[2026-10-13-interrupted-verify-cleanup] Interrupted verify cleanup — `viola verify` stopped by Ctrl-C or TERM removes only what that run created; older, foreign and kill-or-crash leftovers stay by hand
[2026-10-13-paste-newline-ledger-row] Paste newline ledger row — an LF typed inside a paste, which every multi-line send relies on, is a probed and stamped capability-ledger row (v1-34)
[2026-10-13-owed-ledger-rows] Owed ledger rows — the relied-on CLI shapes still without a ledger row become probed, stamped capability-ledger rows (v1-34)
[2026-10-14-resume-and-compact-ledger-rows] Resume and compact ledger rows — `claude --resume <id>` reopening a session and `/compact` become probed, stamped capability-ledger rows (v1-34)
[2026-10-16-remove-the-browser-leg] Remove the browser leg — the browser test leg leaves 0.1.0: every gate green with no Node on the host, no browser step in any workflow, pre-push or harness
[2026-10-16-unwrapped-sessions-on-the-board] Unwrapped sessions on the board — `claude agents --json` read once, unwrapped read-only rows, idle/busy STATUS, revive refuses a session id live elsewhere (v1-37)
[2026-10-17-end-to-end-path-binary-and-harness-legs] End-to-end path binary and harness legs — first `path_` E2E binary, `run --e2e` selector, harness boot with the statusline echo, two-instance budget leg, status `list` member
[2026-10-17-cli-machine-contract] CLI machine contract — --json per verb mirroring channel results, typed exits 0/1/2/10–14/20/21, cause-named hints, global --home, config.json thresholds
[2026-10-17-cli-output-discipline] CLI output discipline — linear static output on three OSes, grouped --help, colour only beside its word, zero wrapper bytes, unblocked human keys during sends
[2026-10-17-mcp-server-for-drivers] MCP server for drivers — stdio tools send/wait/last/answer/list, CLI-parity results and code-only refusals, no wheel or link controls, calls under the tool-call timeout
[2026-10-17-optional-plugin-install] Optional plugin install — MCP-only plugin for unwrapped drivers, absolute-path command, never two hook processes per event, MCP-server precedence measured as ledger row
[2026-10-18-compact-post-condition-ledger-row] Compact post-condition ledger row — `/compact` confirmed by its SessionStart of cause `compact`, a probed, stamped ledger row with its own confirm window (v1-34)
[2026-10-18-plugin-precedence-live-readings] Plugin precedence live readings — the two plugin readings on claude 2.1.287 (which `viola` MCP server or servers a wrapped session holding both the `--plugin-dir` plugin and the installed `viola@viola` starts and which answers its `list` call; whether a registered `viola mcp` that exits 1 blocks an unwrapped start), the method past the folder-trust dialog, and `v1-42`
[2026-10-19-dialog-hold-for-the-driver] Dialog hold for the driver — a longer or configurable dialog hold and a record of an expired hold (gap-list group 2)
[2026-10-19-overseer-switch-to-viola] Overseer switch to viola — the overseer drives through viola instead of the prototype; claims v1-33, its text re-worded to the Linux dev host and a session-terminal key by the founder's ruling 2026-10-19 (inputs#I1 of the 2026-10-19T07-31-13-wrap run)

### Epoch 6 — Security hardening

[2026-10-19-newer-claude-cli-rows] Newer claude CLI rows — the four ledger rows claude 2.1.302 fails, re-measured and brought back to a full stamp
[2026-10-20-epoch-5-cleanup] Epoch 5 cleanup — the debts the Epoch 5 boundary's audit and diagnosis named, each closed so the next boundary's audit reads it closed
[2026-10-20-epoch-5-cleanup-ii] Epoch 5 cleanup II — the three Epoch 5 boundary debts the founder's split moved out of "Epoch 5 cleanup", each closed so the next boundary's audit reads it closed
[2026-10-25-windows-endpoint-admission] Windows endpoint admission — pipe access for the owning user and SYSTEM only, remote pipe clients rejected, squatted name blocks start (per security-plan §Authentication)
[2026-10-26-tokio-rustsec-2026-0386-lock-bump] tokio RUSTSEC-2026-0386 lock bump — a lock bump of `tokio` to `>=1.53.3`: the `unsound` advisory RUSTSEC-2026-0386 against the locked tokio 1.53.1 fails CI's `supply-chain` job, with no ignore-list widening (per security-plan §Dependency Security)
[2026-10-26-server-verification-before-any-frame] Server verification before any frame — clients match the recorded wrapper pid and start time; CLI/MCP report unreachable, hooks fail open on mismatch
[2026-10-26-explicit-session-links] Explicit session links — `viola link` / `unlink` verbs and their channel methods, held for the founder's word on the boundary widening
[2026-10-26-home-and-code-bearing-file-integrity] Home and code-bearing file integrity — owner-only home, DACL check before any diagnostics file, pinned-binary re-hash, per-start plugin rewrite, single stamp/snapshot writers
[2026-10-27-pinned-copy-race-and-reproducible-builds] Pinned copy race and reproducible builds — the pinned copy of the binary never replaced under a concurrent first start (the `pin_exe` race on a fresh home), and whether a rebuild of one tree can read as another binary (two clean release builds of one tree, compared by file and by section); carved out of «Home and code-bearing file integrity» with its two CARRYs verbatim by the founder's own word at its P4 (`chunks/2026-10-26-home-and-code-bearing-file-integrity/inputs/I3-relay-1.md.txt`, Q4) and his P5 «Да» on the card (`chunks/2026-10-26-home-and-code-bearing-file-integrity/inputs/I4-relay-2.md.txt`)
[2026-10-27-bounded-inputs-at-every-boundary] Bounded inputs at every boundary — size caps, depth limits, closed types, names validated before path joins, paste rule client and wrapper side
[2026-10-31-sanitised-error-surfaces] Sanitised error surfaces — codes and fixed messages only in CLI --json, MCP errors and channel error data, no paths or upstream text
[2026-10-31-exit-cause-code-catalogue] Exit-cause code catalogue — every exit-21 and exit-1 cause a distinct code across --json, MCP and process log, matching its human hint

### Epoch 7 — Cross-OS completion

[2026-11-01-unix-endpoint-and-home-hardening] Unix endpoint and home hardening — per-user 0700 socket directory, 0600 socket, peer euid both sides, Unix strict modes, macOS pid gap recorded
[2026-11-04-linux-and-macos-parity] Linux and macOS parity — openpty wrapper, full fake-agent suite and obs gates green on both, identical diagnostics schema, Windows stays live-supported target
[2026-11-05-linux-only-code-and-ci] Linux-only code and CI — no Windows or macOS code, tests, fixtures or CI jobs left, recoverable at a pushed tag; every gate green on Linux

### Epoch 8 — Polish & ship

[2026-11-06-version-done-check] Version done-check — every gate green on Linux, masters agree, fix-pass items honoured, overseer drives a builder through viola
