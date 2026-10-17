**Project directory structure**
```
viola/
├── Cargo.toml                  # [package] viola (bin) + [workspace] members = ["crates/*"], exclude = ["fuzz"]
├── Cargo.lock
├── rust-toolchain.toml         # channel = "1.98.1" (exact pin), components = ["rustfmt", "clippy"]
├── deny.toml                   # cargo-deny: advisories, licences, sources, bans (C, telemetry, features)
├── deny-sync.toml              # the tokio ban, run per scripts/sync-crates.txt crate as sole root
├── clippy.toml                 # disallowed-macros: tracing::{info,warn,error,debug,trace}
├── .gitignore
├── LICENSE-MIT                 # MIT text (holder Turbolet85); the project is MIT OR Apache-2.0
├── LICENSE-APACHE              # the standard Apache License 2.0 text
├── README.md                   # description + `## License` naming both licence files
├── plugin/                     # embedded via include_str!, written out by `viola run`
│   ├── .claude-plugin/plugin.json
│   ├── hooks/hooks.json        # exec-form commands, placeholder for the pinned bin copy
│   └── .mcp.json
├── src/                        # the `viola` bin: anyhow edge only
│   ├── main.rs                 # clap 4.6.7 dispatch (Windows: the System32 DLL-search restriction is its second statement)
│   ├── conpty.rs               # Windows x64: the embedded ConPTY companions and their four pins (the vendor script parses this text)
│   ├── human.rs                # the `human` module root (human-facing text): the refusal and internal-error stderr writers and the stdout result writer (`write_refusal` / `refuse`, `write_internal_error` / `internal_error`, `write_result` / `result`), and `pub(crate) use` re-exports of its children's names, so every caller keeps its path; called by `run`, `revive`, `verify`, `send`, `wait`, `last`, `list`, `answer`, `pause`, `release` and the `main` catch site
│   ├── human/                  # the human module's phases: mirror.rs: `send`'s readback mirror (`[  ] open` / `[RB] read back` / `[  ] unconfirmable` with its one fixed note / `[/ ] unable`), `write_wrapper_fault`, `write_unable`;
│   │                           #   escape.rs: the message-mode escaper (`escape_message`) and the row-mode escaper (`escape_row`); board.rs: the two state refusal pairs (`instance_state_pair`, `home_state_pair`) and the board writers (`BayBudget`, `bay_line`, `board_rows`, `write_board`: with a known terminal width the unwrapped NAME cut, at a colour depth the spans); style.rs: the colour layer (`Depth`, `Tokens`, `Facts`, `decide`, `read`, the Windows VT enable and its restoring guard);
│   │                           #   wait.rs: the `wait` / `last` lines; verbs.rs: the `answer` line, the `pause` / `release` lines and the budget lift / open lines; hints.rs: the hints (`HUMAN_TYPING_HINT`, `answer_hint`, `send_hint`)
│   ├── cmd/                    # one module per subcommand: run, revive, send, wait, last, list,
│   │                           #   answer, hook, mcp, verify, pause, release, link, unlink, plugin;
│   │                           #   revive.rs: the strict-modes check ahead of the wrapper log, the four-reading preflight (its third the `session-live` agents read) and the `--list` rows; its child shapes come from `viola-agent-claude`;
│   │                           #   list.rs: the caller-side board (the strict-modes scan before any log, then one `claude agents --json` read joined into it — STATUS idle/busy, the unwrapped rows — then the `--json` document or the human board as one stdout write; exits 0 / 21 / 1), and `board_document`, the `mcp` `list` tool's provider;
│   │                           #   mcp.rs: the hidden `viola mcp` (the home and `diagnostics/` through `check_path` before anything opens, exit 1 on a refusal; then `viola_mcp::serve`);
│   │                           #   hook/statusline.rs: the `hook statusline` arm (the instance check, the `budget.json` write, the bounded shell-out);
│   │                           #   client.rs (a helper, no subcommand): the channel client send / wait / last share;
│   │                           #   verify/typed.rs: verify's five interactive PTY runs (Run A untrusted, Run B trusted, Run C dialogs, Run D plan, Run E resume),
│   │                           #   the settle rule, Run B's wait for the input box after an added turn, and the signature-only screen recording;
│   │                           #   verify/stop.rs: verify's Ctrl-C / TERM stop (Unix SIGINT / SIGTERM, Windows Ctrl-C / Ctrl-Break)
│   ├── run/                    # PTY pump, wheel, budget governor; gate.rs: the pump-output tee + bounded vt100 feed + Gate;
│   │                           #   agents.rs: the bounded `claude agents --json` read (`read_logged`, `read_here`) shared by `list`, `revive`, verify's Run B and the `mcp` `list` tool (through `list::board_document`);
│   │                           #   send.rs: the `send` module root: the wrapper's `send` method on `Ctx` with its refusal order, re-exporting `SendSlot`, `append_hook_event` and `parse_from` so every caller keeps its path; its phases under send/:
│   │                           #     slot.rs: the one-in-flight slot (`SendSlot`) with the remembered session id, `append_hook_event` with the driver relabel, and the `session-start` post-condition claim;
│   │                           #     classify.rs: the local-command decision over `LOCAL_COMMANDS` (`classify`); params.rs: `parse` / `parse_from`; budget.rs: `Ctx::budget_paused`;
│   │                           #     relabel.rs: only the `#[cfg(test)]` prompt-match and relabel cases; test_support.rs (`#[cfg(test)]`): the shared send test fixtures;
│   │                           #   wait.rs: the WaitFeed (Mutex + Condvar wake, newest turn, start rebuild), `wait` / `last`;
│   │                           #   dialog.rs: the DialogSlot (one pending dialog, Condvar await, armed continuation);
│   │                           #   wheel.rs: the WheelSlot (holder + cause, one lock), its record worker, the stdin observer + classifier;
│   │                           #   snapshot.rs: Snapshots, the wrapper's one in-process snapshot holder;
│   │                           #   budget.rs: the thresholds read (`read_thresholds`), the pure decision (`decide`) and the BudgetGate (one per wrapper: `start` / `evaluate` / `lift` / `paused`);
│   │                           #   links.rs: the LinkSet (one per wrapper, the instance's link set under one lock: `restore` from the log at start, `current`, `record`, and `remove`, which has no product caller yet)
│   └── bin/viola-fake-agent.rs # test-only stand-in `claude` (feature `fake-agent`)
├── tests/                      # root integration tests (sync)
│   ├── cmd/*.toml              # trycmd cases: human-mode expected output (snapbox redactions)
│   ├── snapshots/              # insta snapshots (check mode only)
│   └── support/                # the sync root fixture chain + fixture-hygiene checker; sgr.rs: the SGR reader of a terminal stream (`sgr_attributes`, `redraws`, `sets_a_foreground`)
├── schemas/                    # JSON schemas: fake-script.v1.json, claude-fixture.v1.json, claude-statusline-fixture.v1.json, claude-agents-fixture.v1.json, claude-screen.v1.json and config.v1.json (test-side), diag-line/diag-detail.v1.json (obs line contracts)
├── crates/
│   ├── viola-core/             # normalised events, RefusalReason + NotDelivered, HumanTyping, WheelCause, validate_paste_text, ViolaName, Percent, `v` constants,
│   │                           #   SPINE_DEADLINE, Clock / SystemClock, the budget reading (`Reading`, `BudgetWindow`, `BudgetReading`), `BudgetWindowKind` (`five-hour` · `seven-day`)
│   │                           #   (+ proptest-regressions/, committed seeds)
│   ├── viola-pty/              # pty seam over portable-pty =0.8.1 (+ windows-sys kill fallback; HostTerminal raw mode: windows-sys Console / libc termios;
│   │                           #   PasteHandle: the child's input writer shared by the human copy and the one-write bracketed paste;
│   │                           #   host_stdin(): the host stdin (Windows console: viola's own ReadConsoleW reader, every 0x1A kept);
│   │                           #   `sideload` (Windows): the System32 DLL-search restriction + the absolute-path conpty.dll pre-load)
│   ├── viola-channel/          # JSON-RPC 2.0 ndjson over interprocess local sockets; tokio_client.rs: the Tokio client behind the `tokio` feature
│   ├── viola-state/            # ndjson logs (the append heals a torn last line), atomic snapshots, File::lock, the events
│   │                           #   reader (`events::read_from`: counts unknown kinds, unknown fields and torn lines), the
│   │                           #   liveness-only endpoint read (`liveness::live_endpoint`, shared by the CLI frame verbs and viola-mcp's frame tools), the
│   │                           #   classified snapshot read, the log replay and the session chain (`replay.rs`; read by `viola revive` and `board::read`),
│   │                           #   the link rule (`replay::apply_link_line`, public: applied by the replay's one pass and by the root bin's `LinkSet`),
│   │                           #   the instance strict-modes check (`strict::check_instance`), the budget reading's write
│   │                           #   and its checked read (`budget.rs`: `write_budget`, `read_budget_stamped`, and `read_budget`, which wraps it),
│   │                           #   the board read model (`board.rs`, read by `viola list`: `scan` / `read`, `Board`, `Item`,
│   │                           #   `SkippedTotal`, `RowLiveness`, `SessionStatus`, `ScanRefused`, and the agents join: `JoinKeys`, `Observation`, `join`,
│   │                           #   `Item::unwrapped`; sync, std only, no Claude shape; liveness's `start_times`, one process-table read; no type
│   │                           #   holds `endpoint`, `pinned_bin`, `cwd`, `statusline_command`, `pid`, `started_at` or `links`)
│   │                           #   (+ tests/, the crate-level suite: `state_events.rs`, `state_replay.rs`)
│   ├── viola-agent-claude/     # hook parsing, dialog mapping, R8 strip, shim resolution,
│   │                           #   capability ledger, the vt100 screen model (`screen`), the pure `agents` module (the `claude agents --json` parse), the pure `statusline` module
│   │                           #   (the payload reading, the settings reader, the override document, the shell argv)
│   │                           #   (+ proptest-regressions/, committed seeds; src/snapshots/, the insta
│   │                           #   snapshots of the dialog decision bodies)
│   ├── viola-mcp/              # rmcp 3.4.1 stdio server, thin adapter over viola-channel
│   └── viola-e2e/              # test-only: viola-harness (agent-run boot/run/status/cleanup/logs, plus the
│                               #   internal subcommands incl. `gate` and `pre-push`: harness::pre_push; the `#[cfg]`
│                               #   reader behind `run --mutants`' host exclusion: harness::run::mutants::host)
├── scripts/
│   ├── agent-run.{sh,ps1}      # identical shims over viola-harness
│   ├── conpty-vendor.sh        # re-vendor vendor/conpty/ from the pinned nupkg (+ --verify: sha256 + byte compare + signer; --probe)
│   ├── sync-crates.txt         # the single sync-crate list (CI job 3 + the sole-root tokio ban)
│   ├── deny-probes.sh          # negative probe per cargo-deny ban + a clean control, and the channel tree form's probe + control
│   ├── channel-tokio-free.sh   # viola-channel's normal graph without its feature reads no `tokio v` line, its `--features tokio` control one (CI supply-chain)
│   ├── rmcp-features.sh        # viola's release graph requests rmcp features exactly `server transport-io`, rmcp present (CI lint job)
│   ├── lint-probes.sh          # each clippy ban fires, controls pass, fail-closed raw-event! grep
│   ├── release-check.sh        # target job 6: the release build carries `viola` only, no test-only feature (+ --probe)
│   ├── orphans-check.sh        # cargo modules orphans --deny per lib/bin target (+ --probe)
│   ├── g2-zero-panics.sh       # obs G2: 0 `event:"panic"` role lines under target/e2e-home, exempting only a
│   │                           #   `panic_location` of exactly `src/cmd/hook/seam.rs:<digits>` (fail-closed; + --probe)
│   ├── install-ripgrep.sh      # pinned, sha256-verified ripgrep 15.2.0 → target/tools/ripgrep
│   └── profraw-census.sh       # <runs> <workers> [<name>]: dev-host witness, the start test N times on the
│                               #   instrumented root-bin test binary, each run's profiles counted
│                               #   → target/profraw-census/ (no CI job, pre-push stage or harness command)
├── vendor/conpty/<version>/x64/ # the committed Microsoft conpty.dll + OpenConsole.exe (binary per .gitattributes)
├── .config/nextest.toml        # nextest profiles `ci` and `mutants`
├── fuzz/                       # separate cargo-fuzz workspace (own Cargo.lock; excluded from the root)
│   ├── rust-toolchain.toml     # channel = "nightly-2026-09-20" (fuzz only)
│   ├── fuzz_targets/{viola_name,channel_frame,hook_stdin,vt100_feed,paste_text,claude_agents_json}.rs
│   └── corpus/<target>/        # committed synthetic seeds
├── fixtures/
│   ├── claude/<cli-version>/   # hook-payload fixtures and signature-only Screen.<phase>.json screens and <Event>.<stem>-<n>.json dialog and framing variants and StatusLine.<stem>-<n>.json status line variants and Agents.recorded-<n>.json agents variants recorded by `viola verify`, plus superseded relayed dialog fixtures (RELAYED.md)
│   └── fake-scripts/           # committed fake-agent turn scripts (synthetic)
├── .github/
│   └── workflows/
│       ├── ci.yml              # push + PR: 3-OS test/perf (hyperfine rows + gate --require
│       │                       #   perf)/lint (+ module orphans), msrv, fuzz-replay, 3-OS release
│       │                       #   (release-check), supply-chain (+ fuzz
│       │                       #   lockfile audit)
│       ├── nightly.yml         # weekly schedule + workflow_dispatch: cargo deny check advisories (root + fuzz/Cargo.lock)
│       │                       #   + fuzz time-box
│       └── windows-mutants.yml # workflow_dispatch only (no inputs), dispatched at the boundary audit, never a gate:
│                               #   windows-2025 run --mutants --package … --file … per labelled matrix item, over that
│                               #   item's cfg(windows) files (the root package split by file)
├── refs/                       # brief and prior-art survey (arch input)
└── .andromeda/                 # pipeline runs and cache
```
