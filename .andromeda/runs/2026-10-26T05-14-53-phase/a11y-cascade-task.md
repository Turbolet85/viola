# Task — a11y · 2026-10-26-tokio-rustsec-2026-0386-lock-bump

## The chunk
A dependency bump: the workspace's exact `tokio` pin and the locked tokio move from 1.53.1 to a patched version (`>=1.53.3`, pin form `=` kept) so `cargo deny check` stops failing on the unsound advisory RUSTSEC-2026-0386 and CI's `supply-chain` job reads green, with RUSTSEC-2017-0008 still the only advisory ignore.
Companion crates `tokio-macros`, `tokio-stream` and `tokio-util` move only where the new tokio requires it; no other dependency moves.
No product behaviour change: no source file changes unless the bump forces one (the patched tokio deprecates the safe `AsyncFd` constructors, which clippy's `-D warnings` would read as red in viola's own code), and no CLI output, wire contract, exit code, refusal word or log line changes.
It re-runs the existing gates only: `cargo deny check` with the tokio bans and the ban probes, fmt, clippy, the workspace tests, the rmcp-features and release checks, and reads CI's `supply-chain` job green on its push.

## Reached files
- Cargo.toml · edits `[workspace.dependencies]` tokio pin
- Cargo.lock · edits the tokio entry and, only where required, tokio-macros / tokio-stream / tokio-util
- deny.toml · re-reads `cargo deny check` (advisories, ignore list unchanged)
- deny-sync.toml · re-reads the tokio ban per sync crate
- scripts/channel-tokio-free.sh · re-reads viola-channel's tokio-free normal graph
- scripts/deny-probes.sh · re-reads every ban firing
- scripts/rmcp-features.sh · re-reads the rmcp feature check
- scripts/release-check.sh · re-reads the release-artifact check
- crates/viola-mcp/Cargo.toml · re-reads Tokio containment
- crates/viola-channel/Cargo.toml · re-reads Tokio containment (the `tokio` feature)
- crates/viola-e2e/Cargo.toml · re-reads Tokio containment (test-only)
- .github/workflows/ci.yml · re-reads the `supply-chain` job

## Needs
- The tokio version move itself: a `Cargo.toml` pin and `Cargo.lock` entry change, no source change unless a deprecation forces one.
- The crates that build Tokio and so receive the new version: `viola-mcp` (the stdio MCP server, tools send · wait · last · answer · list), `viola-channel`'s Tokio client behind its `tokio` feature, and the test-only `viola-e2e` clients.
- The MCP server and wrapper IPC channel are the plugin / ipc-internal surface with no UI: the bump reaches them only through their Tokio runtime and client, changing none of their frames or results.
- A possible forced source edit replacing a deprecated safe `AsyncFd` constructor in viola's own code, if P3 measures one; such an edit must change no output any surface emits.
- The no-behaviour-change boundary: every CLI verb's human, plain and `--json` output, refusal words, `hint:` lines and typed exit codes stay byte-identical, so the existing CLI output-discipline cases keep passing unchanged.
- The `viola run` / passed `viola revive` TUI boundary stays as it is: the run pump is std threads with no Tokio, so the bump does not reach the keystroke path, and the existing outer-PTY boundary cases are re-run, not rebuilt.
- The workspace tests re-run as an acceptance gate: locally inside `nextest-integration` / `nextest-e2e`, in CI inside the per-OS `test` job's `coverage` suite on all three OS legs, under `gate --require coverage,doctest` (no skip allowed); the existing a11y output-discipline cases ride inside them.
- CI's `supply-chain` job going green is the chunk's verdict; the per-OS `test` job row is read at the wrap.
- No ignore widening, `skip`, `allow` or licence exception; `deny.toml` policy unchanged.

## Standing conventions
- S1 · Any test case the chunk adds or touches runs inside the existing harness suites (`scripts/agent-run.* run`, CI's `coverage` suite); no new `suite` enum value and no separate a11y driver · body §1 A11y assertion harness specification (CI integration) / §9 CI Integration / §11 CI
- S2 · Zero-flakiness: `retries: 0`, no retry-once policy, `#[ignore]` banned, a flake keeps the chunk red until its root cause is fixed · body §10 SLO Invariants / §11 SLO
- S3 · No WCAG conformance claim rests on cli / tui output-discipline results · body §11 Strategy

## Not needed
- The CLI SGR rules (amber `DIALOG`, dim stale rows, bold NAME, no SGR under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json`, NAME cutting): the chunk changes no styling and no `src/human/style.rs` code (.andromeda/a11y-plan-detail/visual-design-verification.md).
- The `viola list` board structure — the `CAPTION` literal and its order, the BAY context line, the `-- UNWRAPPED - READ-ONLY --` separator, `--help` group labels: no board or help change (aria-landmarks, path-p6-board, cognitive-consistency).
- Building or changing P4 wheel cases or the outer-PTY boundary driver (keystrokes unblocked, focus / mouse / resize not moving the wheel, the Windows F-W3 fact): the chunk touches no pump or wheel code.
- The refusal layout procedures (`unable <reason> <detail>` then `hint:`, the hint-free `unknown`, hint-free `--json`): no refusal is added or changed (cognitive-errors).
- The fixed copy set and the pinned CLI strings: no copy changes (cognitive-plain-language).
- The time-bound inventory (the `viola wait` timeout, the MCP `wait` cap, the dialog hold, confirmation windows, the status-line deadline, the agents read, the budget pause): the bump changes none of them, even though the MCP server runs on Tokio (cognitive-timeouts).
- The owed `viola revive --json` document after a passed preflight: owned by the route entry "Exit-cause code catalogue", not this chunk.
- The manual NVDA / VoiceOver / Orca and keyboard passes: founder-owned supplements on P4 / P6, not triggered by a lock bump.
- The 0.2.0 frontend and any browser a11y leg: deferred, technology not chosen.
- The trigger inventory (visual-discrimination, keyboard-only, screen-reader-priority, and the not-triggered list): no trigger is added or re-scoped.
