@task_prose part=chunk
Closes viola-0.1.0: moves `portable-pty` off `=0.8.1` to a release whose graph drops `pin-utils` (RUSTSEC-2026-0441), so `cargo deny check advisories` and CI's `supply-chain` read green with no ignore-list widening.
Removes, or keeps with its reason, each Windows remnant left in code (item 17), the masters following, and re-measures `PTY_SIDELOAD_TEST_CASE`, `tests/cmd/` and the `thiserror::Error` derive census at HEAD.
Edits the masters where they meet the code: the env-var row, the one-enum-per-crate divergences (fold or ratify), the derive count, trycmd / assert_cmd text, the `portable-pty` pin text, F434's two whole-fact edits, and the no-leak and dialog-hold wording.
Measures live on Linux within two headless sessions: `v1-53`'s one `viola run`, a driver driving a wrapped builder through viola, a dialog hold near the 3600 s ceiling, and the channel-handle no-leak property under `/proc/<child pid>/fd`.
Re-runs every gate green on Linux (fmt, clippy, check, test, `cargo deny check`, the deny probes, the tokio-free checks, the release check, CI's seven jobs), and records the 17 unclaimed caps as proved or deferred, on the founder's word.
@task_file n=1 path=Cargo.toml mark=edits
what: the `portable-pty` dependency pin
@task_file n=2 path=Cargo.lock
@task_file n=3 path=crates/viola-pty
@task_file n=4 path=deny.toml mark=re-reads
what: `cargo deny check` (advisories RUSTSEC-2026-0441 green, ignore list not widened)
@task_file n=5 path=tests/cli_output_plain.rs mark=edits
what: the Windows remnants at lines 104 and 116
@task_file n=6 path=crates/viola-agent-claude/src/lib.rs mark=edits
what: the Windows remnant at line 2
@task_file n=7 path=crates/viola-e2e/src/harness/session.rs mark=edits
what: the Windows remnant at line 52
@task_file n=8 path=crates/viola-e2e/src/harness/mod.rs mark=edits
what: the `EXE_SUFFIX` joins
@task_file n=9 path=crates/viola-e2e/src/harness/boot.rs mark=edits
what: the `EXE_SUFFIX` join
@task_file n=10 path=crates/viola-e2e/src/harness/run/perf.rs mark=edits
what: the `EXE_SUFFIX` joins
@task_file n=11 path=crates/viola-e2e/src/harness/run/coverage.rs mark=edits
what: the `COVERAGE_FLOORS` «per OS» comment and the `COVERAGE_IGNORE` Windows-path comments
@task_file n=12 path=src/cmd/mod.rs mark=edits
what: `plain_drive` and its test cases
@task_file n=13 path=crates/viola-e2e/src/harness/run/mutants/host.rs mark=edits
what: the `windows` test-host key
@task_file n=14 path=tests/contract_os_gated_tests.rs mark=re-reads
what: the OS-gated census (39 OS-gated test functions, 2 gated files) that `v1-53`'s claim reads
@task_file n=15 path=.github/workflows/ci.yml mark=re-reads
what: CI's seven jobs green on `ubuntu-latest`, `supply-chain` included
@task_file n=16 path=scripts/deny-probes.sh mark=re-reads
what: the deny probes
@task_file n=17 path=scripts/channel-tokio-free.sh mark=re-reads
what: the tokio-free check of `viola-channel`'s normal graph
@task_file n=18 path=scripts/release-check.sh mark=re-reads
what: the release check
@task_item n=1 part=needs
The PTY decision: `portable-pty` pinned behind viola's own `pty` seam (spawn, read, write, resize, wait, kill). It names this entry as the owner of the move off `=0.8.1`. Under the new release, child exit is still detected on the process handle, `pty_backend()` still reads `openpty`, no human key is held behind a resize and DA1 is never answered. The Stack line's `=0.8.1` text follows the move.
@task_item n=2 part=needs
The supply-chain surface: RUSTSEC-2026-0441 reaches the tree through `portable-pty =0.8.1` → `nix 0.25.1` → `pin-utils`. It is read by `cargo deny check advisories` and CI's `supply-chain` job, whose `target/supply-chain/` artifact holds `deny.json` and `deny-fuzz.json`. The remedy may not widen the ignore list.
@task_item n=3 part=needs
Indirect, through the new `portable-pty` graph: the Tokio containment keystone binds it. The sole-root `cargo deny` `tokio` ban, the `cargo check` of the sync crates in `scripts/sync-crates.txt` (`viola-pty` among them) and the tokio-free channel check must stay green, and so must `deny.toml`'s licence allow list.
@task_item n=4 part=needs
Occupied Resources, Environment variables: `MCP_SERVE_TEST_CHILD` is registered there as a test-only marker. `PTY_SIDELOAD_TEST_CASE` is either registered beside it or stated as taking no row, its presence re-measured at HEAD (item 3).
@task_item n=5 part=needs
The Error Handling decision and its echoes: one `thiserror` enum per crate, `PtyError` the ratified exception. The open divergences `ServeError`, `StampsError`, `PinError`, `StampsReadError` and `Refused` each take a fold or a ratification. The derive count (nine stated, ten measured) is re-measured and re-worded in architecture fct-gctodeyz, the Conventions error-types line (fct-nhsg4k7g) and the Inherited Defaults Errors line.
@task_item n=6 part=needs
The architecture `project-directory-structure` contract key and the masters' trycmd / assert_cmd text pinning CLI strings under `tests/cmd/*.toml`. No manifest holds either tool and no `tests/cmd/` exists: the chunk decides tool or text, the absence re-measured at HEAD.
@task_item n=7 part=needs
The dialog hold: `dialog_hold_s` (60–3600 s, default 60) has a PROVISIONAL ceiling owned by this entry, in the Hook Contract decision, `config.json` and `schemas/config.v1.json`. The `timeout` 3615 on the PreToolUse and PermissionRequest hooks also applies. A hold near the ceiling is read live here.
@task_item n=8 part=needs
The Deployment / Distribution decision: the pinned copy `bin/<version>-<hash>/` is keyed by the first 16 hex of the content SHA-256. Measurement B (one tree building two files that differ in `.note.gnu.build-id`, `.symtab` and `.strtab`) is owed to this entry as a founder item.
@task_item n=9 part=needs
The Linux-only rule and its Process and path discipline: the Windows and macOS code is out of the tree (tag `multi-os-final`). A gate whose predicate names `windows` or `target_os = "macos"` reads as a fault. Masters that still describe a removed remnant (for example the CLI Conventions' Git Bash rewritten-path warning, which `plain_drive` served) follow the removal.
@task_item n=10 part=needs
The `viola-e2e` harness's binary paths (`viola-harness` builds, `target/perf/release/viola`, test bins) and its `run --coverage` floors and ignore regex (`target/lcov.info`). The `run --mutants` cfg-predicate host table, whose `windows` key item 17 names, also applies.
@task_item n=11 part=needs
The Linux IPC endpoint: a Unix socket `<socket dir>/viola-<h12>.sock` in a per-user 0700 directory (`$XDG_RUNTIME_DIR/viola/` or `/tmp/viola-<uid>/`), chmod 0600, a peer admitted only for the owning euid. The channel-handle no-leak property is measured on it: the wrapped child's `/proc/<child pid>/fd` holds no channel socket or listener. Then security-plan fct-p7rzex2i is re-worded off «recorded, not established».
@task_item n=12 part=needs
The mutation audit's survivor: an always-admit `peer_admitted` read at the three client seams survives every committed test. One account makes no foreign peer, and no seam may be added. The survivor stays a dated note at matrix#v1-44 and test-plan §1's untestable zone.
@task_item n=13 part=needs
The live-session rule: the real `claude` runs only in local live tests, never in CI. `run --local-live` refuses under `CI` and the `AGENT_RUN_*` harness variables. The live homes sit at `target/e2e-home/viola-live-<pid>/home`. `v1-53`'s claim reads the census and the `ubuntu-latest` JUnit (`target/nextest/ci/junit.xml`) alone.
@task_item n=14 part=needs
The driving surfaces the live «overseer drives a builder through viola» session uses: the wrapped start (`viola run <name> -- <program>`), the wheel starting at `driver`, `send` / `wait` / `answer` and the implicit link they record. The MCP tools reach a session as `mcp__plugin_viola_viola__<tool>`.
@task_item n=15 part=needs
`v1-13` and `v1-36` take their dispositions here. `link` and `release` (budget included) are CLI-only, so no MCP result exists for them. The `list` payload carries no link set, while the deferred 0.2.0 page's intent names one. Each clause is deferred with its reason or re-worded, on the founder's word.
@task_item n=16 part=needs
The Human Takeover / Wheel decision's closed non-editing list, measured on foot 1.28.0 only. The hypothesis that a terminal answering `CSI ? 6 n` or the kitty graphics query loses the wheel at a session's start needs a live terminal and a real `claude`. A shape joins the list only when measured, by its exact grammar.
@task_item n=17 part=needs
Session liveness and the killed-wrapper leg: a real `claude` read on Unix ending with its killed wrapper, and the founder-attended tab-close in a Linux terminal. The fake-agent reading `run_child_ends_when_its_wrapper_is_terminated` stands meanwhile, beside the heartbeat and pid + start-time `gone` rule.
@task_item n=18 part=needs
obs-plan log-coverage's `process-start` row (fct-yb5jpaot) is brought to the header's 2 cells: `pty_backend` `openpty` alone, `sideload_fallback` gone. It carries the same value the PTY decision states. The edit lands through the store's whole-fact form (`@fact id=… expect_rev=N whole=true`), as does removing the seven directory-tree pointer lines from `fct-irhgihut`.
@task_item n=19 part=needs
The founder's word owed on four PROVISIONAL items. The socket-directory pair's wording (the refused socket directory, `socket-dir-refused`) and the board's read rule on every OS (the strict-modes read rule `viola list` applies to the home and instance files) are two of them.
@task_item n=20 part=needs
CI/CD: GitHub Actions on `ubuntu-latest` only, its seven jobs (`test`, `perf`, `msrv`, `fuzz-replay`, `lint`, `release`, `supply-chain`) all green at the wrap. Setup's red run ci#42255961371 is closed against its own log. The `release-check` gate refuses an artifact built with `viola-channel`'s `test-support` feature.
@task_item n=21 part=needs
The verification matrix's claims and deferrals for the 17 unclaimed caps (`v1-01` … `v1-53`): `v1-01`'s CI-run outcome and advisories clause become provable once the `portable-pty` move lands. A cap whose acceptance names a 0.2.0 surface reads `deferred` only on the founder's word.
@task_standing n=1
convention: every OS-gated test names its reason in the census binary `tests/contract_os_gated_tests.rs`, and a gate predicate naming `windows` or `target_os = "macos"` reads as a fault
home: cross-cutting-patterns
@task_standing n=2
convention: every new test runs against the fake agent named after `--` in its own viola home (`--home`, under `target/e2e-home/`), never sharing an endpoint, log or `budget.json`. The real `claude` runs only in local live tests, refused under CI.
home: Established Decisions
@task_standing n=3
convention: a test-only environment marker sits outside the `VIOLA_*` namespace, is read by no `viola` build, and is registered in Occupied Resources → Environment variables
home: occupied-resources
@task_standing n=4
convention: every test-only path or scratch dir (temp-dir reports, `target/` probe and run dirs) is registered in Occupied Resources (Filesystem or Repository), is gitignored when under `target/`, and is never read or written by `viola`
home: occupied-resources
@task_standing n=5
convention: a new log line goes to its role's codes-only process log under `diagnostics/`, in the obs-owned format fixed by `schemas/diag-line.v1.json`. Content-bearing detail goes only to the instance's `diagnostics/detail-<role>.ndjson`.
home: Cross-cutting Patterns
@task_standing n=6
convention: a test-only feature or binary is never in a release build: it is gated behind its feature (`fake-agent`, `test-support`) and refused by the release check
home: occupied-resources
@task_item n=1 part=not_needed
The per-method channel contracts (`params` → `ok` payloads, `from` typing, `MAX_FRAME`, the protocol-fault codes): the chunk adds no verb, flag, env var or channel method and changes no payload.
@task_item n=2 part=not_needed
The `RefusalReason` value set, its detail sets and refusal order, and the CLI exit-code table: no refusal, detail or exit code is added or changed.
@task_item n=3 part=not_needed
The normalised event kinds, the Hook → kind map and the per-kind event `data` shapes: no event kind or field changes.
@task_item n=4 part=not_needed
The snapshot envelope, the instance snapshot's fields and the replay readers: no snapshot field or reader changes.
@task_item n=5 part=not_needed
The ledger stamps envelope, the capability-ledger rows and `viola verify`'s probe plugin and probe dirs: no ledger row or probe is added.
@task_item n=6 part=not_needed
The `hook statusline` contract, `statusline-source.json`, the per-session `settings.json` override and the budget thresholds' defaults: no statusline or budget mechanism changes.
@task_item n=7 part=not_needed
`viola plugin install` and its local marketplace tree: untouched.
@task_item n=8 part=not_needed
`viola revive`: its preflight, `--list` and its `--json` refusal codes are untouched.
@task_item n=9 part=not_needed
The naming and data-model conventions (`ViolaName` grammar, the FNV-1a endpoint-name hash, RFC 3339 timestamps, `Option` field omission, `Percent`): no identifier or wire format changes.
@task_item n=10 part=not_needed
The vt100 readiness gate and the delivery-confirmation window and its `unconfirmable` outcome: not changed.
@task_item n=11 part=not_needed
The snapshot writer's tempfile helpers and the crash-safe write discipline of logs and lock files: no writer changes.
@task_item n=12 part=not_needed
The pins of the other dependencies (clap, chrono, rmcp, interprocess, vt100, tempfile, serde_json, nutype): only `portable-pty` and the graph it brings move.
@task_item n=13 part=not_needed
The Mobile, Hosting and AI/ML rows and the Template patterns: N/A in v1 and unreached.
@task_item n=14 part=not_needed
The R8 identity strip and the `IDENTITY_FLOOR`, and the colour variables `NO_COLOR`, `TERM` and `COLORTERM`: no environment row of theirs changes.
@task_item n=15 part=not_needed
Protocol versioning (`v`, the sender version) and mixed-version tolerance: no format is added, removed or re-meant.
