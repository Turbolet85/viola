# Task — obs · 2026-11-01-unix-endpoint-and-home-hardening

## The chunk
Verifies the per-user Unix socket directory (owner, mode, not a symlink) before a wrapper binds and before a client connects, refusing a directory that fails and never narrowing it; the socket stays 0600 by chmod after the bind.
Checks the peer's euid on both sides of the local socket: the wrapper against a connecting client, every Unix client against the server it reached, before its first frame.
Gives each new refusal a closed code and a hint in the exit-cause catalogue's form, a row of `tests/cross_exit_causes.rs`, including test-plan §6's macOS case (socket directory loosened to 0755); P3 reads which exit each one is.
Makes the board's home check one rule under Unix strict modes on the paths the entry reaches, and records the macOS pid gap as a named carve-out: the server check there is a match with no comparison.
Re-derives `holder_gone`'s connect-probe measurement and the takeover of a leftover socket file beside the new checks, and measures the hypothesis that harness `cleanup` reads `endpoint_gone:false` and exits 1 after a force-kill on Unix.

## Reached files
- crates/viola-state/src/board.rs · edits `scan`'s home check
- tests/cross_exit_causes.rs · edits its rows, one per new refusal
- tests/support/home.rs · re-reads `holder_gone`, the root kill path's connect probe
- crates/viola-e2e/src/harness/cleanup.rs · re-reads its Unix rule for `endpoint_gone`, the socket path gone

## Needs
- The `detail` code catalogue and `process-exit`: a refused socket directory and a peer euid mismatch each take a closed code the catalogue must record, on the rule that every cause behind exit 21 and exit 1 has a distinct code.
- `schemas/diag-line.v1.json`: any new `detail` value or field on a home-level line must be admitted there, or the conformance gate fails on the new line.
- `run`'s start refusals: a refused socket directory at a wrapper's start joins the list of pre-child refusals, written as the two fixed stderr lines with no path or pid, in the correct pair for its level, before or after the log opens as P3 reads it.
- The `human::refuse` call-site counts per verb in the body's §3: a refusal site the chunk adds or moves changes the recorded count for `run`, `list` or `client`.
- `client::refuse_untrusted`, the one site `link`, `unlink` and the six frame verbs share: the client-side euid mismatch and a socket directory refused at connect reach those verbs through it or beside it, ahead of any frame.
- The CLI `wait` / `last` connect path: the shape of the existing `server-verify-failed` exit (one `process-exit` at WARN with `during:"connect"`, inside the dispatch span, no `channel-request` line) and its `--json` document are what a client refusal sits beside.
- Ordering on the client: a home or instance tree the instance check refuses ends before any role file opens and writes no line; the new connect-time refusals must state where they fall against that check.
- The MCP frame tools: a client-side refusal maps to `instance-unreachable` with a `detail`, its `isError` code the only record and no line, kept inside `mcp.tool_call`; this covers the Tokio client as well as the sync client, since every Unix client checks.
- The `hook` connect and `hook.dialog`: a refusal of this chunk reaches `hook` only as a fail-open: exit 0, no body, no stderr, any trace in `hook-<name>.ndjson` alone through a `hook-decision` fail-open `detail`.
- The wrapper-side euid refusal in `run`: it happens while the child runs, so nothing reaches the terminal; its trace, if any, is in `run-<name>.ndjson`, keyed by `conn` with no request behind it, and a refused connection is not a `channel-request` / `channel-response` pair.
- The `viola-channel` surface: its instrumentation stays sync with Tokio only behind its feature; a verified euid does not turn the self-reported `from` into an authenticated identity in any line.
- The endpoint on the `viola run` start scenario: the endpoint name and socket path appear in no span or line, and a bind or directory failure's `detail` carries no path or pid.
- `bind_endpoint`'s `create_private_dir` failure, today an anyhow error at the catch site reading `internal-error`: the typed refusal replaces that path for the verified directory, through a fixed-message error at the crate seam, logged once.
- `list` and `board::scan`: `ScanRefused::Instance`, the home pair against the instance pair, the `strict-modes-failed` code and the rule that a strict-modes refusal before any log opens writes no line; making the home check one rule changes which refusal a 0750 home reads once it holds an instance; `list`'s refusal pair carries no SGR.
- The strict-modes telemetry trigger: its closed list of the few places a strict-modes failure has a line; a path this entry brings under strict modes either stays off that list or joins it by an amendment.
- The macOS gap in the `wait` / `last` scenario and the error-capture section's server-verification wording: Linux and Windows compare, macOS compares no peer pid; the euid check does not close that gap, and no line or test reads the macOS verdict as a comparison.
- The harness `cleanup` document and `agent-run status`: `endpoint_gone` and the exit it drives on a force-killed Unix wrapper, whose leftover socket file stays; `status`'s closed `last_error` enum wherever a new cause reaches it.
- `agent-run logs` and the OS matrix: the new refusals' lines read the same on Linux and macOS under the same end-to-end suite, with a leg the chunk cannot prove staying in that suite's live Linux run.

## Standing conventions
- S1 · A new log line goes through `obs_event!` with explicit typed fields and a static message equal to the event name, never a raw tracing macro · §11 Logs and §6 of the body, log-coverage.md
- S2 · A logged field must be named in the Section 6 catalog, default-deny, and validated by the schemas under gate G4 · §8 Default-deny posture and §9 of the body, log-coverage.md
- S3 · A `detail` code is closed and kebab-case and never holds a path or pid · §6 and §11 PII Scrubbing of the body, log-coverage.md
- S4 · A new `event` value or a renamed field needs a Decisions Log entry · §11 Logs and §12 of the body
- S5 · A new span has a static `<area>.<operation>` name, `skip_all` first and an explicit field allow-list, with no `err` or `ret`, under gate G1 · §4, §9 and §11 Spans of the body, span-trace-coverage.md
- S6 · A new line takes its level from the levels mapping: WARN for exit 21, ERROR for exit 1 or 20, INFO for other refusals · §6 of the body
- S7 · No telemetry on stdout and no print or `dbg!` macros in `hook`, `run` or `mcp` paths; human stderr goes through the owning output modules · §3 and §11 Logs of the body
- S8 · A new typed error has a fixed-message `Display` holding no path or payload, and anyhow stays at the root bin's dispatch edge · §7 of the body, error-capture-reporting.md
- S9 · A new integration test's viola home lives under `target/e2e-home/`, so G2, G4, the secret scan and the `diag-<os>` upload cover it · §9 of the body, ci-integration.md
- S10 · A new test takes no retry-once policy and no sleep-based log wait, and asserts by exit code or a `jq` / harness verdict · §10 and §11 SLO and CI of the body, slo-invariants-telemetry-budgets.md
- S11 · A new dependency stays Tokio-free in the sync crates, adds no C-building crate and passes `cargo deny` · §1 Telemetry triggers and §10 of the body, obs-scope-summary.md
- S12 · A new line uses the same file sink, fields and schema on every OS, with no OS-specific exporter · §1 Telemetry triggers of the body, obs-scope-summary.md

## Not needed
- The Budget governor scenario, `budget.json`, `budget-gate` and `STATUSLINE_DEADLINE`: no budget behaviour moves here.
- The Confirmed `send` scenario past its connect: paste, `text_bytes`, readback and the `/compact` confirmation window stay as they stand.
- The Dialog → `answer` scenario's `dialog_id` lifecycle, its `corr` timing and the unverified-CLI branch: only the hook's connect is reached.
- The wheel scenario: `wheel` transitions, automation refusal and `release-from-driver` are not moved.
- The R8 strip's `env_stripped_*` and `env_kept` fields on `process-start{subject:"claude-child"}`: no child-spawn environment changes.
- Windows-only telemetry: `sideload_fallback`, `conpty/` companions and `exit_source:"kill-fallback"`; no Windows behaviour moves.
- Torn-line healing and snapshot replay (`state-recovered`), with `read_snapshot_or_replay`'s six readers: the state store's recovery is not changed.
- The vt100 feed's `parse-rejected` lines and the PTY pump's boundary events: nothing here touches the screen or the pump.
- The derived `viola.*` metric recipes of §5 and their label discipline: the chunk adds no metric.
- CI artifact names, `retention-days`, the nightly `fuzz` job and the dispatch-only `windows-mutants.yml`: the scope adds no CI job or upload.
- The tier justification and exporter carve-outs of §1 and the pipeline pick of §2: the file-sink pipeline stands unchanged.
- The frontend: deferred to 0.2.0, so no frontend scenario applies.
