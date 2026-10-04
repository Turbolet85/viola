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
│   ├── human.rs                # human-facing text: the refusal and internal-error stderr writers, the stdout result writer, `send`'s readback mirror (`[  ] open` / `[RB] read back` / `[/ ] unable` + its hints), the message-mode escaper, the `wait` / `last` lines, the `answer` line and the `pause` / `release` lines, called by `run`, `verify`, `send`, `wait`, `last`, `answer`, `pause`, `release` and the `main` catch site
│   ├── cmd/                    # one module per subcommand: run, send, wait, last, list,
│   │                           #   answer, hook, mcp, ui, verify, pause, release, link, unlink, plugin;
│   │                           #   client.rs (a helper, no subcommand): the channel client send / wait / last share
│   ├── run/                    # PTY pump, wheel, budget governor; gate.rs: the pump-output tee + bounded vt100 feed + Gate;
│   │                           #   send.rs: the wrapper's `send` method, the one-in-flight slot, the driver relabel;
│   │                           #   wait.rs: the WaitFeed (Mutex + Condvar wake, newest turn, start rebuild), `wait` / `last`;
│   │                           #   dialog.rs: the DialogSlot (one pending dialog, Condvar await, armed continuation);
│   │                           #   wheel.rs: the WheelSlot (holder + cause, one lock), its record worker, the stdin observer + classifier;
│   │                           #   snapshot.rs: Snapshots, the wrapper's one in-process snapshot holder
│   └── bin/viola-fake-agent.rs # test-only stand-in `claude` (feature `fake-agent`)
├── tests/                      # root integration tests (sync)
│   ├── cmd/*.toml              # trycmd cases: human-mode expected output (snapbox redactions)
│   ├── snapshots/              # insta snapshots (check mode only)
│   └── support/                # the sync root fixture chain + fixture-hygiene checker
├── schemas/                    # JSON schemas: fake-script.v1.json and claude-fixture.v1.json (test-side), diag-line/diag-detail.v1.json (obs line contracts)
├── crates/
│   ├── viola-core/             # normalised events, RefusalReason + NotDelivered, HumanTyping, WheelCause, validate_paste_text, ViolaName, Percent, `v` constants,
│   │                           #   SPINE_DEADLINE, Clock / SystemClock
│   │                           #   (+ proptest-regressions/, committed seeds)
│   ├── viola-pty/              # pty seam over portable-pty =0.8.1 (+ windows-sys kill fallback; HostTerminal raw mode: windows-sys Console / libc termios;
│   │                           #   PasteHandle: the child's input writer shared by the human copy and the one-write bracketed paste;
│   │                           #   host_stdin(): the host stdin (Windows console: viola's own ReadConsoleW reader, every 0x1A kept);
│   │                           #   `sideload` (Windows): the System32 DLL-search restriction + the absolute-path conpty.dll pre-load)
│   ├── viola-channel/          # JSON-RPC 2.0 ndjson over interprocess local sockets
│   ├── viola-state/            # ndjson logs, atomic snapshots, File::lock, the events reader (`events::read_from`:
│   │                           #   skips + counts torn / oversize lines; healing owed to route :87), tailing (with `ui`)
│   ├── viola-agent-claude/     # hook parsing, dialog mapping, R8 strip, shim resolution,
│   │                           #   capability ledger, the vt100 screen model (`screen`), statusline parsing
│   │                           #   (+ proptest-regressions/, committed seeds; src/snapshots/, the insta
│   │                           #   snapshots of the dialog decision bodies)
│   ├── viola-mcp/              # rmcp 3.4.1 stdio server, thin adapter over viola-channel
│   ├── viola-ui/               # axum 0.8.9 GET routes + SSE, Host allowlist
│   │   └── assets/             # embedded page: index.html, app.css (the single stylesheet); no JS build step
│   │                           #   and no tsconfig until the frontend-toolchain entry brings the React + TypeScript
│   │                           #   bundle (founder ruling 2026-09-30; its source and bundle layout OPEN)
│   └── viola-e2e/              # test-only: viola-harness (agent-run boot/run/status/cleanup/logs, plus the
│                               #   internal subcommands incl. `gate` and `pre-push`: harness::pre_push)
├── scripts/
│   ├── agent-run.{sh,ps1}      # identical shims over viola-harness
│   ├── conpty-vendor.sh        # re-vendor vendor/conpty/ from the pinned nupkg (+ --verify: sha256 + byte compare + signer; --probe)
│   ├── sync-crates.txt         # the single sync-crate list (CI job 3 + the sole-root tokio ban)
│   ├── deny-probes.sh          # negative probe per cargo-deny ban + a clean control
│   ├── lint-probes.sh          # each clippy ban fires, controls pass, fail-closed raw-event! grep
│   ├── release-check.sh        # target job 6: the release build carries `viola` only, no test-only feature (+ --probe)
│   ├── orphans-check.sh        # cargo modules orphans --deny per lib/bin target (+ --probe)
│   ├── g2-zero-panics.sh       # obs G2: 0 `event:"panic"` role lines under target/e2e-home, exempting only a
│   │                           #   `panic_location` of exactly `src/cmd/hook/seam.rs:<digits>` (fail-closed; + --probe)
│   ├── install-ripgrep.sh      # pinned, sha256-verified ripgrep 15.2.0 → target/tools/ripgrep
│   ├── install-node.sh         # <os-key> <dest>: the official Node build at ci.yml's NODE_PIN_* (parsed from the
│   │                           #   file text), sha256-verified, flattened into <dest> (+ --probe)
│   └── npm-audit.sh            # e2e-web lockfile: npm audit (every level) + registry.npmjs.org-only sources
│                               #   → target/npm-audit/ (+ --advisories-only, --probe)
├── vendor/conpty/<version>/x64/ # the committed Microsoft conpty.dll + OpenConsole.exe (binary per .gitattributes)
├── .config/nextest.toml        # nextest profiles `ci` and `mutants`, `fixed-port` group
├── fuzz/                       # separate cargo-fuzz workspace (own Cargo.lock; excluded from the root)
│   ├── rust-toolchain.toml     # channel = "nightly-2026-09-20" (fuzz only)
│   ├── fuzz_targets/{viola_name,channel_frame,hook_stdin,vt100_feed,paste_text}.rs
│   └── corpus/<target>/        # committed synthetic seeds
├── fixtures/
│   ├── claude/<cli-version>/   # hook-payload fixtures recorded by `viola verify`, plus relayed dialog fixtures (RELAYED.md)
│   └── fake-scripts/           # committed fake-agent turn scripts (synthetic)
├── e2e-web/                    # test-side Node only (Playwright; axe and the a11y lint land with the a11y chunks);
│   │                           #   the ts code-graph plane
│   ├── package.json            # pins @playwright/test 1.63.0 (exact; @axe-core/playwright lands with the a11y chunks)
│   ├── package-lock.json       # committed; audited by scripts/npm-audit.sh
│   ├── playwright.config.ts    # headless chromium, retries 0, forbidOnly, reporters pw.json + pw-junit.xml
│   ├── tsconfig.json           # noEmit, strict, e2e-web/** only
│   ├── stub/pipe.html          # the file:// reachability stub (one <h1>, no script or style)
│   ├── eslint.config.js        # eslint-plugin-jsx-a11y over the crates/viola-ui frontend sources
│   ├── .htmlvalidate.json      # html-validate over the embedded assets/index.html
│   ├── tests/*.spec.ts         # one spec per bay layout type; today the pipe stub's pipe-reachability.spec.ts
│   ├── fixtures/a11y.ts        # the shared makeAxeBuilder fixture
│   ├── schemas/a11y-row.v1.json  # tests-owned a11y violation-row schema (not obs schemas/)
│   ├── a11y/sc-coverage.json   # per-SC coverage map
│   └── test-results/           # gitignored outputs (a11y/, lint/)
├── a11y/
│   └── sr-pass/                # manual screen-reader passes: TEMPLATE.json, <date>-<at>.json
├── .github/
│   └── workflows/
│       ├── ci.yml              # push + PR: 3-OS test (+ the browser suite)/perf (hyperfine rows + gate --require
│       │                       #   perf)/lint (+ module orphans), msrv, fuzz-replay, 3-OS release
│       │                       #   (release-check), supply-chain (+ fuzz
│       │                       #   lockfile audit, npm lockfile audit); the workflow env holds the NODE_PIN_* lines
│       ├── nightly.yml         # weekly schedule + workflow_dispatch: cargo deny check advisories (root + fuzz/Cargo.lock),
│       │                       #   npm-advisories (npm-audit.sh --advisories-only) + fuzz time-box
│       └── windows-mutants.yml # workflow_dispatch only (no inputs), dispatched at the boundary audit, never a gate:
│                               #   windows-2025 run --mutants --package over each package's cfg(windows) files
├── refs/                       # brief and prior-art survey (arch input)
└── .andromeda/                 # pipeline runs and cache
```
