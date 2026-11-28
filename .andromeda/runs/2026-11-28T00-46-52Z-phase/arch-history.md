@history_item n=1 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** (diagnostics roots, config key, diag schemas, v exception) — set every role's codes-only log to its own home-level role file (`run-`/`hook-`/`mcp`/`ui-`/`cli-`), content detail only in the instance `detail-<role>.ndjson`, `diagnostics_level` the sole level source, process-log lines carrying no `v`. The console's «own diagnostics role» extends exactly this register (rows decision/codes-only-process-logs, contract/diagnostics-role-files/*, contract/diag-line/format); the `ui-` file named here was later cut (n=9).

@history_item n=2 marker=2026-09-24-log-redaction-and-never-log-floor
**2026-09-24-log-redaction-and-never-log-floor** — anyhow stays in the root bin (`main`, dispatch, the catch-site reporter `viola::obs::report_internal_error`), context chains only in the owner-only detail file, never a role line, stdout or stderr. A new console process role inherits this catch-site rule (rows decision/error-chain-to-diagnostics-only, decision/anyhow-only-in-bin).

@history_item n=3 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — added the workspace `print_stdout`/`print_stderr`/`dbg_macro` bans, `disallowed-macros` on the tracing level macros (`obs_event!` the one emitter) and the lint probes. A console that draws to a terminal must write through sanctioned writers, never print macros (rows decision/clippy-print-and-macro-bans, decision/no-print-macros, decision/obs-event-only-emitter).

@history_item n=4 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — the sole-root tokio ban per sync crate (`deny-sync.toml`, `scripts/sync-crates.txt`) and the four-family `deny.toml` (advisories, licences, sources, C/telemetry/feature bans). Any terminal-drawing or key-reading dependency the scaffold proposes (his fork) must pass these bans and keep the sync graph tokio-free (rows decision/cargo-deny-dependency-policy, decision/sync-crates-tokio-free).

@history_item n=5 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** (test-only crate, bins, env vars, paths) — Module Boundaries gained «each product crate is created by its first consumer», and the test-only `viola-e2e`/`viola-harness`/`AGENT_RUN_` prefix were registered. Binds the scope's «no crate without a real consumer and a test» and where a keystroke test driver and its env knobs may live (rows decision/cargo-workspace-flat-crates, decision/harness-env-var-prefix).

@history_item n=6 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (the second fake-agent test seam `FAKE_AGENT_HOOK_PANIC`) — a new unprefixed test seam was a boundary widening ratified by the founder, read only under `cfg(feature = "fake-agent")`. Precedent: if the keystroke driver needs a product-side test seam to expose console state, that seam is a widening needing his word (rows decision/test-seams-not-configuration, decision/env-var-viola-prefix).

@history_item n=7 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — clap built without `color` (its anstream route read CLICOLOR/NO_COLOR/TERM/COLORTERM/CI) and `src/human.rs` named the root bin's one human writer. The console must not bring a crate that re-opens an env-reading colour route, and its human output belongs to the human module (rows decision/clap-without-color-feature, decision/single-human-writer).

@history_item n=8 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** (the cli role's one internal-error printer) — `role_of` files every first word but `run`, `hook` and a leading flag under `cli`, and the `main` catch site prints the internal-error line once. A console verb gets a role through `role_of`; whether it is `cli` (prints the line) or a silent role like `run`/`mcp` (n=17, n=20) is a choice this chunk must make.

@history_item n=9 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** (0.1.0 is a pure CLI; the frontend moves to 0.2.0) — retired the view-only loopback GUI: no `ui` subcommand, no `ui-<port>` role file, no `UiError`, no `viola-ui` crate; four interfaces from one binary; the masters' GUI sections cut to a «deferred to 0.2.0, technology not chosen» stub. This is the stub the scaffold's surface section replaces, and it answers the scope's question «whether `ui` already is that role»: the `ui` role was removed (rows decision/four-interfaces-one-binary, overview/project-intent).

@history_item n=10 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0) — [Real-time Strategy], [GUI Control Scope], GUI contracts and Product type now say: no web or graphical frontend; the view beyond `viola list` is the keyboard-driven terminal fleet console tiled on Hyprland, owed by «Console scaffold», «Fleet console view» and «Console actions»; the console's contracts owed by «Console scaffold». The direct mandate for this chunk's surface section and the absence rows it must not breach (decision/terminal-fleet-console, absence/graphical-frontend, absence/graphical-interface, absence/web-or-graphical-frontend).

@history_item n=11 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — the colour layer `src/human/style.rs` reads `NO_COLOR`/`TERM`/`COLORTERM` (read, never set, deciding only SGR bytes), ratified by the founder as a widening, with the precedent that any further environment read or a colour fact reaching a control needs its own ratification. A console reading `TERM`, sizes or other terminal env (e.g. `COLUMNS`, not read today) is that kind of widening (rows decision/colour-env-decides-only-sgr, contract/human-output-colour-env/*, decision/no-environment-configuration).

@history_item n=12 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** (send and human split by phase) — the `human` module root over `src/human/` (mirror, escape, board, wait, verbs, hints) is the root bin's only human writer, split as a pure move under 800 code lines. The console's drawing and its later `unable` lines sit beside this module, and the cleanup precedent governs the carried M1/M3 refactors (row decision/single-human-writer).

@history_item n=13 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the Tokio client and viola-channel's tree-form ban; the MCP tools' contract) — Tokio confined to `viola-mcp` (and `viola-channel`'s feature), `role_of` excludes `mcp` from `cli` and `Role::Mcp` never prints the internal-error line. A console process runs on std threads unless it is ruled otherwise, and its role mapping follows this precedent (rows decision/hybrid-concurrency, decision/tokio-only-in-mcp-crate, decision/tokio-containment, decision/viola-channel-tokio-free-by-default).

@history_item n=14 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the instance check before any log, on both arms) — measured that opening a log sets an existing home to 0700, so a check after it cannot refuse a widened home; refused checks write no line. The console's own role log must open only after its strict-modes check (rows decision/strict-check-before-log-open, decision/refused-check-writes-no-line).

@history_item n=15 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — every checked entry point runs its strict-modes/instance check before any log; `open_role_file`/`write_detail` run `check_diagnostics` (symlink refused, read rule) before narrowing; the read rule defined. A new console role file is opened through these same opens and checks (rows decision/diagnostics-open-checks-modes, decision/strict-modes-check-before-log, decision/strict-read-rule, decision/instance-check-before-any-read).

@history_item n=16 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — every product reader of external bytes bounded in its own statement or named in `tests/contract_bounded_readers.rs` `EXCEPTIONS`, a census that goes red otherwise; serde_json `unbounded_depth` banned. The console's key reader on its terminal is a new reader the census will judge (rows decision/bounded-readers, decision/no-unbounded-json-depth, decision/serde-json-unbounded-depth-banned).

@history_item n=17 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `cli` catch site's `internal-error` document) — under `--json` the catch site prints the internal-error document; `run`, `revive`'s start, `mcp` and `hook` stay silent there. Fixes the surface a console verb's internal error takes (rows contract/cli-internal-error/output, contract/cli-exit-codes/1, decision/silent-roles-at-internal-error-catch).

@history_item n=18 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — Linux only (0.2.0 window features on Hyprland), CI on `ubuntu-latest` alone, [PTY] `openpty` with `HostTerminal` raw mode through libc termios and `std::io::stdin()`, libc the one OS binding (windows-sys out), cargo-deny on one triple, the OS-gate census faulting any other-OS gate, the relied set all rows of `LedgerRow::ALL` on every build. Sets the raw-mode and PTY mechanism a console and its keystroke driver build on, and the relied-set code the M3 carry touches (rows decision/linux-only, decision/libc-only-os-binding, decision/deny-single-target, decision/portable-pty-seam).

@history_item n=19 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — portable-pty pinned `=0.9.0` (pin-utils, RUSTSEC-2026-0441, closed by the move with `ignore = []`, every CI job green), the one-enum divergences ratified as named exceptions, the dialog-hold ceiling handed to 0.2.0's first entry. The PTY a keystroke test driver would use, the error-enum rule a new console error type must follow, and the precedent for the chrono carry: an advisory closed by an upgrade, not an ignore (rows decision/portable-pty-behind-pty-seam, decision/thiserror-one-enum-per-crate, decision/one-error-enum-per-crate).

@history_item n=20 marker=2026-10-26-tokio-rustsec-2026-0386-lock-bump
**2026-10-26-tokio-rustsec-2026-0386-lock-bump** — an advisory closed by moving the exact workspace pin to a patched version with no ignore widened, the advisory's record belonging to security-plan, the stack row carrying the version only. The direct precedent for carry `cry-h6vzhlce` (RUSTSEC-2026-0543 on chrono, its `deny.toml` ignore re-read at this boundary) (rows decision/chrono-timestamps, decision/workspace-dependency-pins).

@history_item n=21 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (counts, crate edges and one order finding) — `viola-agent-claude` took `chrono` at the workspace pin for its pure module, so chrono reaches more than the root; an upgrade or ignore decision for RUSTSEC-2026-0543 touches every crate that names it (rows decision/chrono-timestamps, decision/chrono-utc-millisecond-timestamps).

@history_item n=22 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — `viola-hyprland` registered as the one compositor seam with a closed request set and closed error words, hyprctl spawned directly with a closed argv. The console is tiled on Hyprland; any compositor call it later makes goes through this seam, and the scaffold itself dispatches nothing to the compositor (rows decision/terminal-fleet-console, name/crate/viola-hyprland, decision/no-shell-out).

@history_item n=23 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — the test-only `viola-live-rig` with vt100 as its child's screen model in `viola-e2e`, `AGENT_RUN_RIG` registered, test-only binaries kept out of release. The nearest precedent for a PTY-driven keystroke test driver; note this scope's driver asserts on state, never the screen, unlike the rig (rows decision/test-binaries-never-released, decision/harness-env-unread-by-product, name/binary/viola-live-rig).

@history_item n=24 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** — widened decision/run-never-reads-screen-content: the only row-text exit of the screen is read by verify alone (signature fixtures and the rig-only first-screen record). A console or its test driver reading screen rows would be a further widening of this row; the scope's state-not-screen driver keeps inside it (rows decision/run-never-reads-screen-content, decision/screen-rows-read-only-by-verify).

@history_item n=25 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** — registered the test-only self-spawn markers `ROOT_TIED_TEST_CHILD` and `HARNESS_TIED_TEST_CHILD` outside `VIOLA_*`. A console test that spawns itself under its own PTY follows this naming and registration (rows decision/viola-env-namespace-product-only, decision/harness-env-var-prefix).

@history_item n=26 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — root test waits share the one 12 s `WITHIN` bound; test channel sockets bind under a short `/tmp` root. Console tests (bounded under the 20 s lint per the scope) wait on this bound and these socket roots (rows decision/root-waits-share-one-bound, decision/test-isolated-viola-home).

@history_item n=27 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — test homes under `target/e2e-home`, linked to tmpfs on the dev host, made and removed only by the test-side keepers. The scope's «test-owned home» for every console test lives here (row decision/test-isolated-viola-home).

@history_item n=28 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the split modules, `relied_here` and the `viola-mcp` serde edge retired; the test-only child marker) — split `src/cmd/verify/typed.rs` into `typed/` with `framing_turns` in `turns.rs`, folded `relied_here` into `relied()`, registered `MCP_SERVE_TEST_CHILD`. Carry `cry-boujwuyr` M1 (one step function for `framing_turns`' per-paste probes) and M3 (the relied-row count from one source) work on exactly these modules.

@history_item n=29 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — `viola verify`'s `[NN/MM]` step lines take `MM` from `LedgerRow::ALL.len()` and counts carry their rule; Run B's pastes and `FRAMING_TURNS` grew per row. M3's «relied-row count from ONE source instead of literals moved by hand» is the follow-through of this rule (rows contract/viola-verify-output/step-line, contract/viola-verify-output/summary-line, decision/verify-child-set).

@history_item n=30 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — verified now means every relied row counts (`pass`, or `absent` on `browser-tools-prompt` alone), the step line printing `pass|absent|fail` and the summary's N counting rows that count. Any M3 single-source count must keep this counting rule, not a pass count (rows decision/capability-ledger, decision/cli-capability-ledger, decision/version-verified-all-relied-rows, decision/browser-tools-prompt-ledger-row).

@history_item n=31 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** — the human-always-wins rule reworded to «never delayed past the current paste's in-flight chunk and closing marker», with the paste bound and cut mark. A console that reads keys on the same host must not change what holds a human key (rows decision/human-always-wins-wheel, decision/paste-bound, decision/paste-cut-mark).

@history_item n=32 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** — adopt's start joined the roles silent at the internal-error catch site and the occupied command names grew. A console start verb (its name his fork) must not clash with the registered commands and must be placed in the silent-role list or not (rows decision/silent-roles-at-internal-error-catch, name/command/*).

@history_item n=33 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — `viola open` (a terminal window under `--origin`/`--place`) and the `window` record landed, the window features' first verbs. The console's tiled window is «Fleet console view»'s, but the scaffold's start verb sits beside these occupied names and the terminal-spawning precedent (rows name/command/open, decision/no-start-over-live-instance).

@history_item n=34 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows (decision/contract/absence/name/overview), a master holding what a planner must respect and the code the rest. The surface section and any redraw exception this chunk writes land as rows of this form, an exception named on its own row rather than a generalised rule.
