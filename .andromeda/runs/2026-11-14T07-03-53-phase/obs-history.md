@history_item n=1 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows (1077 rows, each verified against the code); every picked row of this chunk is from that migration, so a row-level change here is a `~`/`+` proposal against these handles, not a body edit.

@history_item n=2 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — (version census 0.2.0) §4 must-trace: no frontend scenario; the keyboard-driven terminal fleet console tiled on Hyprland replaces it, its scenarios owed by «Console scaffold» (matrix#v2-14). This is the origin of the Hyprland seam: the seam supplies those later entries and adds no console scenario of its own.

@history_item n=3 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the OS target, Linux only) `decision/linux-only-os-target`: core on any current distribution, CI on `ubuntu-latest`, Windows/macOS cut (tag `multi-os-final`). The seam stays Linux-only and CI-green on a host with no compositor, hence the fake compositor.

@history_item n=4 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (closed words left `diag-line.v1.json` with their emitter) `decision/unemittable-schema-words-removed`, `os` closed to `linux`, `process-exit` `detail` at 19: a closed word is added only with an emitter and removed with it. Any seam outcome code (ok/unsupported/unreachable/malformed) put into a closed enum must be emitted by a build, and the enum count is re-measured by its members.

@history_item n=5 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (ConPTY sideload retired; openpty the one PTY backend) the OS-API entity now reads openpty, Unix socket, filesystem locks and atomic rename, sysinfo (`decision/os-apis-at-call-sites`). A Hyprland socket or `hyprctl` spawn is a new OS/external call site that this entity and its §1 copy must name.

@history_item n=6 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (one CI leg, two workflows, one deny target) `cargo deny` on `x86_64-unknown-linux-gnu` only (`decision/obs-deps-pass-cargo-deny`); the OS-gated test census fails a gate naming windows or macos (`decision/os-gated-test-census`). A new seam crate or dependency passes deny on that target, and any gated seam test must be named in the census.

@history_item n=7 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (named-pipe endpoint retired; server check on Linux) the client seams read the peer by `SO_PEERCRED` / `peer_creds` over a Unix socket. This is the precedent if the Hyprland transport is its request socket: peer reads are viola call sites that log nothing of their own.

@history_item n=8 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — (the OS-gated test census in the E2E layer) `tests/contract_os_gated_tests.rs` requires every OS-gated test function or file to have a `GATES` row with a reason. Seam tests the fake compositor runs on `ubuntu-latest` need no gate; a live-host-only case would need a row.

@history_item n=9 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (a refused socket directory is no strict-modes failure; `socket-dir-refused` in the log catalogue; Founder Direction 4 carve-out retired) `contract/process-exit-detail/socket-dir-refused`, `decision/socket-dir-one-code`, `contract/socket-dir-refused-exit/codes`, `contract/diag-detail/socket-dir-refused`, `contract/mcp-frame-tools/socket-dir-refused`: the channel socket dir's 0700/owner check writes its own closed code (a refused check, a failed create and a lost race read one word), no path, pid or uid. The scope says a Hyprland request socket needs "the same 0700 / owner checks the channel's socket dir has, or a recorded reason it does not" — this is that precedent; a foreign compositor socket under `$XDG_RUNTIME_DIR/hypr/` is not viola's own dir, so reuse of the code needs a decision.

@history_item n=10 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the OS call sites the new checks add) the OS-APIs entity gained `peer_euid_of` and `check_socket_dir` (viola's one `symlink_metadata`) as named call sites, each with its logged word. Precedent for naming the Hyprland seam's reads as call sites in the entity.

@history_item n=11 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (`Refused::NotADirectory`'s fixed message) `decision/not-a-directory-refusal-message-fixed`: a new refusal variant reads a fixed, pathless message and adds nothing to the thiserror derive count when it is a variant of an existing type. A seam error type follows `decision/typed-errors-fixed-display` the same way.

@history_item n=12 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the socket-directory refusal's PII reading and clean-body witnesses) `decision/socket-dir-refusal-names-nothing`: `tests/security_negatives_socket_dir.rs` reads lines and documents with `assert_names_nothing` and `digit_runs`. Template for asserting the seam's lines carry no address, pid, path or socket name.

@history_item n=13 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (the server-identity negatives' no-number scan) `digit_runs` (whole numbers only) scans every output and line for a pid or start time. The seam's codes-only claim (no window address, pid) can be held by the same shared scanner.

@history_item n=14 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (the server check's call sites and its per-OS peer-pid read) the OS-APIs entity reason is a verbatim upstream copy kept current, extended per new call site. Precedent for extending `decision/os-apis-at-call-sites` / `decision/instrument-call-sites-not-os-internals` with the Hyprland seam.

@history_item n=15 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (a peer's reply screened by `sanitised_fault` and `is_code_word`) untrusted peer reply text is read into closed values (`is_code_word`: 1–64 bytes `[a-z0-9-]`), nothing of it reaching a document or line, and the screen adds no span and no line (`decision/clients-screen-peer-replies-to-code-words`). Direct precedent for parsing Hyprland replies tolerantly into closed viola types with window titles/classes reaching no line.

@history_item n=16 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the nine thiserror derives) the product's `thiserror::Error` census reads 9 named types. A new seam error type with its own derive moves this census and must be re-measured.

@history_item n=17 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (the thiserror derive census on the product sources) the census rule is a grep of `thiserror::Error` over `src` and `crates` outside `crates/viola-e2e`, count 9. A seam crate or module adding a derive changes this count by that rule.

@history_item n=18 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (the portable-pty pin at =0.9.0) third-party crates logging through `log` are OFF and their failures captured at viola's seam (D-11; `decision/third-party-crate-logs-off`, `decision/no-log-crate-bridge`). Any Hyprland client crate taken would join that rule; no log bridge.

@history_item n=19 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (D-05 `parse-rejected` gains the `mcp-stdin` parser) a new closed `parser` value takes a Decisions Log record, now kept in this amendments file (§12 points here); the schema enum is the count rule (10 values). A seam reply cut past `MAX_FRAME` or malformed, if logged as `parse-rejected`, needs a new parser word recorded the same way (`contract/diag-line-fields/parse-rejected`, `contract/parse-rejected-detail/oversize`, `decision/per-parser-parse-failure-counts`).

@history_item n=20 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (the `viola mcp` stdin line cap) `CappedLines` at `MAX_FRAME`: a longer line refused whole, never truncated or skipped, no byte of it logged (`contract/mcp-stdin/max-frame`, `contract/mcp-stdin/line-within-max-frame`, `contract/parse-rejected/mcp-stdin-oversize`). Precedent and trap for the seam's `take(MAX_FRAME + 1)` read: refuse whole, newline counted, a truncated reply would parse a padded value.

@history_item n=21 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (the MCP oversize case's canary check) the oversize case writes the canary inside its over-long line and asserts it in no home-level file (`decision/canary-only-in-detail-files`). Template for proving an oversize Hyprland reply leaks no byte.

@history_item n=22 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (session-live, the agents-probe callers) the external `claude agents --json` read: one `process-start`/`process-exit{subject:"agents-probe"}` pair per call at the call site (`run::agents::read_logged`), `parse-rejected{parser:"claude-agents-json"}` on oversize (`MAX_FRAME`) or malformed, `run_bounded`/`run_capped` unlogged. The closest precedent for a `hyprctl` spawn: a new external CLI is instrumented only at spawn, exit and parse (`decision/agents-cli-instrumented-at-boundary`, `contract/diag-subject/agents-probe`, `contract/parse-rejected/claude-agents-json`).

@history_item n=23 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — (verify-probe joins the spawn subjects) §6 names every child spawn as a `process-start`/`process-exit` pair; a new closed `subject` value takes a Decisions Log entry (D-35). A `hyprctl` spawn is a child spawn and would need its own subject word in `$defs.subject.enum` recorded this way (`contract/child-spawn-log/subject`).

@history_item n=24 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (verify's outcome on stdout …) Kept: §6's requirement of a pair for child spawns stood when verify's spawns logged none, and the fix was carried — trap: an unlogged spawn is a gap against §6, not an accepted shape.

@history_item n=25 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — (verify-pty-probe) a new spawn kind gained its own `subject` value (`verify-pty-probe`) in the event table, child-spawn set and schema enum, instrumented at the call site with no row text or path in a line. Second precedent for extending the closed subject set.

@history_item n=26 marker=2026-09-29-sideloaded-conpty
**2026-09-29-sideloaded-conpty** — (D-36 sideloaded ConPTY telemetry) a closed value needs a Log entry under §8 default-deny, D-35 the template; a degrade recorded as a closed word, never a path or hash. Template for recording the seam's closed outcome codes.

@history_item n=27 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** — (registry migration: the Decisions Log leaves the body) the Decisions Log is history kept outside the body; `diag-line.v1.json` `$defs.subject.enum` closes `subject`. Where a new event/subject/parser record for the seam lands.

@history_item n=28 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** — (obs_event! field set, schema default-deny keyword) `obs_event!` attaches `event`, `process`, `instance`; default-deny by top-level `unevaluatedProperties: false` (`decision/schema-top-level-unevaluated-properties`, `decision/default-deny-log-fields`). Any seam field must be named in the catalog and schema or the line fails G4.

@history_item n=29 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — (raw-tracing ban) level macros banned by `clippy.toml` path, raw `event!` by the fail-closed grep in `scripts/lint-probes.sh`; only a caller-crate `#![allow]` exempts (`decision/raw-tracing-ban`, `decision/obs-event-macro-only`, `decision/no-raw-tracing-macros`). The seam logs only through `obs_event!`; a new crate inherits the workspace lints.

@history_item n=30 marker=2026-09-24-log-redaction-and-never-log-floor
**2026-09-24-log-redaction-and-never-log-floor** — (anyhow scope follows the catch-site reporter) anyhow only at the root-bin dispatch edge and `viola::obs::report_internal_error` (`decision/anyhow-at-dispatch-edge-only`, `decision/anyhow-error-capture-boundary`). A seam crate uses typed errors, not anyhow.

@history_item n=31 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — (tracing-subscriber's dev-dependents named) tracing-subscriber is a product dependency of the root bin only; `viola-channel` and `viola-state` take it as a dev-dependency for unit-test line capture (`decision/subscriber-only-in-root-bin`). A new seam crate testing its lines takes the same dev-dependency exception, and the list of named dev-dependents grows.

@history_item n=32 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — (the reserved web vocabulary leaves) the third-party OFF targets are `rmcp`, `portable_pty`, `sysinfo`, `interprocess` (`decision/third-party-targets-off`, `decision/unlisted-targets-log-off`). A new external crate target must join this OFF list or stay unlisted (OFF by default).

@history_item n=33 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — (§5–§11: the web catalog rows leave) the INFO row names the seven `VIOLA_TARGETS` (`viola`, `viola_core`, `viola_pty`, `viola_channel`, `viola_state`, `viola_agent_claude`, `viola_mcp`) (`decision/viola-crates-log-at-info`). If the seam is a new workspace crate, its target is OFF unless added here — a silent-loss trap for its `obs_event!` lines.

@history_item n=34 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — Kept: the nine "env vars are not a configuration channel" restatements ban env as configuration and are not exhaustive over what a build reads (`decision/env-vars-not-config-channel`, `decision/no-env-log-levels`, `decision/env-vars-not-configuration`). Reading `HYPRLAND_INSTANCE_SIGNATURE` to locate the compositor is not configuration under these rows, though security-plan still asks its own Decisions Log entry.

@history_item n=35 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — (env_kept, D-34) environment variables are logged by name only, values never read into a line, no env map to a span or event (`decision/env-names-only`, `decision/r8-strip-logged-as-names-only`). A new environment read for the instance signature logs no value; the signature never reaches a line.

@history_item n=36 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** — (§1 kept current) §1 stays a labelled verbatim copy but is kept current, judged like any body amendment. Many picked rows are §1 twins (`1-obs-scope-summary` / `obs-scope-summary`); a seam change to the OS-APIs or untrusted-output rows must amend both twins.

@history_item n=37 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — (`~ decision/veil-redact-payload-types`, `~ decision/payload-types-derive-redact`) payload types that carry content derive veil Redact; a content-free type derives neither Redact nor Debug, since veil 0.3.0 refuses a Redact derive with no redact field. Seam reply types holding window titles/classes derive Redact; content-free closed outcome types derive neither.
