@history_item n=1 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0) — replaced §4's frontend stub: there is no frontend scenario because viola has no web or graphical frontend, and the terminal fleet console's scenarios (matrix#v2-14) are owed by the route entry «Console scaffold» (census C-312). This chunk is that owner. It must write the console's must-trace scenario in §4 and reconcile it with absence/web-or-graphical-frontend and absence/interactive-terminal-interface, which the console now contradicts.

@history_item n=2 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — cut the viola-ui entity, the web-spa surface and the `ui.*` spans from 0.1.0. It kept the closed vocabulary (`ObsProcess::Ui`, the `ui-<port>` role file) reserved, and the GUI view scenario was left as a stub «deferred to 0.2.0, technology not chosen». This is where the stub the scope names came from, and it is the precedent for how a UI role was once carried in the process enum, schema and role-file list.

@history_item n=3 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (the `ui` process and four events) — removed the reserved `ui` process and the `ui-<port>` role file. Role files became `run-<name>`, `hook-<name>`, `mcp` and `cli-<name>`, and `process` became `run|hook|mcp|cli` (`ObsProcess::ALL`, four). §1 is held current at «4 process roles». The third-party OFF targets were narrowed to rmcp, portable_pty, sysinfo and interprocess. This answers the scope's open question: `ui` is no longer a role. A console diagnostics role means a fifth `ObsProcess` value, a new role-file basename and a §1 count edit. Each of these touches the rows contract/diag-line/process, contract/log-line/process, contract/process-log/process, contract/diagnostics-files/paths and decision/log-file-basenames-follow-tests, and the tests-bound Log format copy. Any terminal-drawing crate's log target would join the OFF list (decision/third-party-crate-logs-off).

@history_item n=4 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (§5–§11) — set the seven `VIOLA_TARGETS` at INFO (`viola_ui` left), the metric grouping keys including `process`, and zero unlogged panics across `run`, `hook`, `mcp` and `cli`. A new console crate or role must be added to the per-module target list (decision/viola-crates-log-at-info, decision/unlisted-targets-log-off) and to the zero-unlogged-panics coverage (decision/zero-unlogged-panics).

@history_item n=5 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — retired §4's «GUI view» scenario heading and its deferral stub. §4 now holds six must-trace scenarios and states that there is no frontend scenario. The console scaffold adds the first scenario since then, so the count in decision/must-trace-paths-always-traced («6 must-trace paths») moves if the console path is made must-trace.

@history_item n=6 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** — `obs_event!` attaches `event`, plus `process` and `instance` from `ProcessCtx`, and `corr` is caller-supplied. The schema's default-deny is a top-level `unevaluatedProperties: false`, and `diag-detail.v1.json` is self-contained with the event enum inlined. A console role's lines get `process` from `ProcessCtx`, so the new role word must enter both schemas' enums under default-deny (decision/schema-top-level-unevaluated-properties, decision/detail-schema-self-contained).

@history_item n=7 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the mcp sink) — `viola mcp` always writes `<home>/diagnostics/mcp.ndjson`, its home resolved and checked with `diagnostics/` before the sink opens. The amendment then said stderr JSON was the fallback only when the file cannot be opened; the picked row decision/one-log-file-per-role now bans any stderr JSON fallback. This is the one role whose file has no instance name. It is the nearest precedent for a fleet-wide console process, which has no single instance: a fixed basename, and the home check before the open.

@history_item n=8 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the refused home check) — `viola mcp`'s start-time home and `diagnostics/` check joined the pre-init window, and its refused check (exit 1, nothing printed, no role file, no line) joined exemption 7. A console role that checks its home before its log follows the same shape: decision/pre-log-strict-refusals-write-no-line and decision/strict-modes-refusal-writes-no-line.

@history_item n=9 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — `role_of` names `revive` beside `run`: revive's start arm is role `run`, and `revive --list` opened no log, a sixth panic exemption. That exemption was recorded against the founder's direction of a structured log from every process. This is the precedent for classifying a new verb's role in `role_of` (decision/role-classified-before-clap), and the direction is the basis of «its own diagnostics role».

@history_item n=10 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — filed `revive --list` under role `cli` (`role_of`: `revive`, then `--list` before a `--`) and retired exemption 6, which brought it under the founder's direction of a structured log from every process. This is the precedent that a new process must log under a role rather than stand exempt, and that `role_of` reads argv before clap.

@history_item n=11 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (`role_of` and the catch site) — the role word `adopt` is classified before clap and is silent at the internal-error catch site, and contract/cli-internal-error/human-line lists the roles that print neither stderr form. The console's role must be added to `role_of` and to this catch-site list, with its own panic outcome by role (cf. contract/panic-outcome-by-role/*).

@history_item n=12 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the mcp self process-exit) — the `mcp` role writes no self `process-exit`, because `obs::internal_error_exit_line` covers `Run` and `Cli` only. This is a trap for a new role: its panic and internal-error `process-exit` line exist only if `internal_error_exit_line` is extended. The rows on panic outcome by role and on process-exit corr bear on it.

@history_item n=13 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `cli` catch site under `--json`) — `json_asked` reads `--json` from argv before any `--`, and the `cli` catch site prints `{"v":1,"error":"internal-error","detail":null}` on stdout in place of `error: internal error`. The console's catch site must take one of the existing shapes or name its own (contract/cli-internal-error/json-document, contract/panic-outcome-by-role/cli-json).

@history_item n=14 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the `cli` internal-error line) — this entry defined the short-lived `cli` panic outcome. When an instance resolves, `process-exit{subject:"self", exit_code:1, detail:"internal-error"}` goes to `cli-<name>.ndjson` and the chain goes to `detail-cli.ndjson` only. After that comes exactly `error: internal error\n` in one `write_all`, with no hint, then exit 1. The same entry made the panic backtrace raw frames. This is the pattern a console role's panic outcome is set against (contract/panic-outcome-by-role/cli, decision/raw-unsymbolised-backtrace).

@history_item n=15 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`check_diagnostics`) — a role file or detail file opens only past `check_diagnostics` in `open_role_file` and `write_detail`. A symlink, or a group or other bit, refuses, and a refusal is a failed open: the line is dropped and the process has no panic sink. A console role file opened through the same path inherits this (decision/diagnostics-checked-at-open, decision/refused-diagnostics-drops-line, decision/refused-diagnostics-path-is-a-failed-open).

@history_item n=16 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the pre-init window and exemptions) — every check made before a role's first open, which narrows an existing home to 0700, joins the pre-init window and bounded exemptions 1 and 8, and a refused check writes no line. This is the founder's own answer to P4 fork Q2. If the console checks its home before its log, its refusal takes this no-line shape and must be listed in the exemptions (decision/strict-modes-before-diagnostics, decision/panic-before-role-file-leaves-no-record).

@history_item n=17 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (terminal subject and strict-modes lines) — the `terminal` subject became a `process-start` alone, with no `process-exit`, because the terminal outlives the verb. `viola up` writes `strict-modes-failed` on its own exit-1 `process-exit` for a home tree refused after its board scan, a third place in decision/strict-modes-failure-lines and an exception in decision/strict-modes-before-diagnostics. The same entry gave up exit 21 for its board scan. This is the precedent for a fleet-level verb that reads the board and spawns terminals, and the console likely sits on that path (contract/process-log/child-and-probe-lifecycle, contract/strict-modes-refusal-exit/1).

@history_item n=18 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (cli-log-file/up) — added contract/cli-log-file/up as the verb's own log-file contract, «as each other cli verb does», because the decision row listing the verbs could not take it within its bound. If the console's start verb logs under `cli`, it owes a cli-log-file row of its own. If it takes its own role, the role-file rows move instead.

@history_item n=19 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — `viola open` was instrumented under the `cli.open` dispatch span into the opened name's cli file. The `terminal` subject joined the closed `diag-line` subject enum, and the `run.window` span and an untraced window line were added. A console tile opened on Hyprland reuses the terminal-spawn and window telemetry (decision/window-line-untraced), and this is the per-verb row pattern (contract/cli-log-file/*).

@history_item n=20 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — `viola focus` logs as `cli` under a manual `cli.focus` dispatch span that carries no fields (contract/cli-log-file/focus, a picked row). focus joined decision/instance-check-before-log-init and contract/strict-modes-refusal-exit/21. This is the newest precedent for a human-only command's log shape and manual span (decision/cli-manual-dispatch-span).

@history_item n=21 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (2. Telemetry Strategy) — `hyprland` was added to the closed `<area>` list of decision/span-naming for the manual `skip_all` span `hyprland.read`. The console's spans need a new `<area>` word on this closed list, by the same amendment form.

@history_item n=22 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (1. Obs Scope Summary and 6. Log Coverage) — the hyprctl spawn joined decision/os-apis-at-call-sites and decision/per-parser-parse-failure-counts. The `hyprctl-probe` subject and the `hyprctl-json` parser were added to closed enums, each with its own contract row. A console that reads the board or Hyprland, or parses any new input, follows this pattern: a closed word with its emitter, and a row for each word.

@history_item n=23 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (2. Telemetry Strategy) — dropped `statusline` from the closed `<area>` list of decision/span-naming when its spans left. The list is kept exact both ways: an area enters with its spans and leaves with them.

@history_item n=24 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (closed words in `diag-line.v1.json`) — added the rule decision/unemittable-schema-words-removed: a closed schema word that no build can emit leaves with its emitter. `os` is closed at `linux`. Any role or event word the console adds must ship with an emitter in the same chunk. A reserved role word with no emitter, the `ui` pattern of 2026-10-16, is now barred.

@history_item n=25 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS target) — set the OS target to Linux only, with one file sink and one line schema on Linux, and CI on `ubuntu-latest`. The console and its keystroke driver assume Linux openpty alone (decision/linux-only-os-target, decision/single-file-sink-no-os-exporter, decision/one-linux-log-sink).

@history_item n=26 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (the additive-schema rules) — the operator narrowed decision/diag-schema-additive-only, decision/log-schema-additive-only and decision/obs-aligns-to-tests-log-format to «may add; must not rename or remove a field the product still emits». A new role or event value is additive and takes a Decisions Log entry (decision/diag-schema-additive-only, contract/process-log/event).

@history_item n=27 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — made §3's Log format JSON schema copy verbatim with test-plan §3 again. A new process role or role-file basename edits that bound copy, and it must move together with test-plan's twin (decision/log-file-basenames-follow-tests).

@history_item n=28 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** — the fake agent, a root-package `[[bin]]`, is exempted from the print-macro ban by a crate-level `#![allow(clippy::print_stdout, clippy::print_stderr)]`, because lints are per package. A console that draws to its terminal, and a test driver, need a named exemption of this form or must use no print macros (decision/no-print-macros and its except).

@history_item n=29 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — an output module may carry a local `#[allow]` only where it uses a print macro. `src/human.rs` uses none: it writes through one `write_all` on the locked stderr. This is the precedent for how a console's terminal writer stays inside the print-macro ban and the no-telemetry-on-stdout rules (decision/no-telemetry-on-stdout, contract/stdout-by-role/cli).

@history_item n=30 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — raw `tracing` level macros are banned by path in `clippy.toml` `disallowed-macros`. Raw `event!` is caught by a fail-closed grep in `scripts/lint-probes.sh`, and a lint-probes failure is a build failure. Every console line must go through `obs_event!` (decision/no-raw-tracing-macros, decision/obs-event-macro-only, decision/lint-probes-failure-fails).

@history_item n=31 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** (2. Telemetry Strategy and 7. Error Capture) — the paste writer thread joined decision/panic-caught-at-every-role and decision/run-worker-panic-ends-run, which name an exact count of the threads under `catch_unwind`. Any console worker thread (input, redraw, tick) must be listed under `catch_unwind` the same way, or a panic there escapes the zero-unlogged-panics invariant.

@history_item n=32 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (the chaos home outside the scans, F4) — integration homes live under `target/e2e-home/`, scanned by G2, G4 and the secret scan. One founder-ruled carve-out home sits outside the scans. The keystroke driver's test-owned home takes the scanned default unless a carve-out is ruled (decision/zero-panics-gate, decision/schema-conformance-gate, decision/secret-scan-before-upload).

@history_item n=33 marker=2026-11-17-supply-chain-green-and-booted-tests-end
**2026-11-17-supply-chain-green-and-booted-tests-end** — set decision/direct-chrono-dependency: chrono is kept though unmaintained, RUSTSEC-2026-0543 stands in the cargo deny ignore list and is re-read at each epoch boundary, and jiff and time are not adopted. This is the row that carry `cry-h6vzhlce` re-reads at this boundary. An upgrade that removes the ignore amends this row, and a further epoch on his word keeps it.

@history_item n=34 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (decision/direct-chrono-dependency ratified) — the founder ratified the chrono row as landed, so its re-read-at-each-boundary clause is his standing word. The `cry-h6vzhlce` fork (upgrade, or keep the ignore one more epoch) is the operator's to answer against it.

@history_item n=35 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — verify's stdout became one `[NN/MM]` step line per relied row, with `MM` the relied count (then 21 and 18 by OS), and the summary sums to it. Later entries moved `MM` by hand at every new row, which is the literal-count drift that carry `cry-boujwuyr` M3 («the relied-row count from ONE source») targets (contract/verify-stdout/step-line, contract/verify-stdout/summary-line).

@history_item n=36 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the Windows host arms) — the relied count became «all 24 rows of `ALL`» (was 24 and 21 through `statusline::WRAPPED_HERE`), the closest the plan comes to a single source for the count. M3's refactor must keep verify's step and summary lines unchanged.

@history_item n=37 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — the step line's verdict gained `absent` for the browser-tools-prompt row, and the summary's pass count includes an absent row that counts. Exit 0 is «every relied ledger row counted». The founder ratified this on 2026-11-22. The M3 one-source count must keep these semantics (contract/verify-stdout/step-line, contract/verify-stdout/summary-line, contract/verify-exit-codes/0).

@history_item n=38 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — the post-gate turn read, the link record, `send-issued` and the paste run under one flight lock in `send_through`'s guarded section, so a turn known before the paste always refuses `turn-running` with no `send-issued` (contract/send-refused-detail/turn-running). Carry `cry-boujwuyr` M1's look at that guarded section must keep this ordering and the event order in contract/send-path-log-events/event-order.

@history_item n=39 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — vt100 is fed on `run`'s feed thread under per-call `catch_unwind` and used for gate outcomes only. Nothing is logged per byte, and no telemetry comes from screen content. The keystroke driver must likewise assert on state or log lines, never on screen bytes. The console must not derive telemetry from what it draws (decision/no-telemetry-from-screen, decision/no-telemetry-from-screen-content, decision/no-hot-path-instrumentation).
