# obs · cascade read · group 2 of 7

## Carried
- (.andromeda/obs-plan-detail/obs-scope-summary.md:183) The `hook.event` and `hook.dialog` clients must place the server verification between the stdin parse and the channel call, so that a refused verification makes no channel call yet still ends in the required `hook-decision` line, with any panic on the new path caught into one `panic` line.
  > - **Backend instrumentation hooks:** `tracing` spans around stdin parse → channel call (`hook.dialog` / `hook.event`) → decision emit, through the tracing-subscriber JSON logger. `hook-invoked` / `hook-decision` lines are required, and panics are caught with `catch_unwind` and emitted as one `level:"ERROR", event:"panic"` line (tests excerpt, Log format, Agent parsing).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:185) The hook's fail-open-on-mismatch path the scope names must log only to `hook-<name>.ndjson`, write nothing on stderr, write no decision body on stdout, and still exit 0 when that log write itself fails.
  > - **Exporter:** A file sink only, to `diagnostics/hook-<name>.ndjson`. stderr is forbidden on both. stdout carries only the decision body for `viola hook <event>`, and for `viola hook statusline` only the user's status-line command's output (arch Standard Contracts, Hook contract); the two hidden `--capture` arms (`verify`'s probe capture) write no sink and print nothing, except `--answers`' probe decision body. A failed log write is swallowed, and the hook still exits 0 (tests excerpt, Log format Constraints).
  (.andromeda/obs-plan.md:200)
  > - **Required telemetry signal:** fail-open paths log to `hook-<name>.ndjson` only, exit 0 with stderr empty, and drift reports go to `diagnostics/` only.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:189) The peer-credential and sysinfo start-time read added to every hook must keep each timed hook's `max` under its budget, adding no blocking I/O to logger init.
  > The latency budget is `max < 1.0 s` per hook (tests excerpt, Quality Gates). Logger init must not add blocking I/O beyond one file open and append.
  (.andromeda/obs-plan.md:230)
  > - **Trigger type:** perf-budget-instruments
  > - **Source:** tests excerpt, Coverage Triggers (hook deadlines / heartbeat) and Quality Gates (hyperfine `max`); brief O6 ("hook latency per tool call")
- (.andromeda/obs-plan-detail/obs-scope-summary.md:196) The sync and Tokio clients' connect, which the chunk edits to read the server's identity, must keep a client-call span and the `corr` on every channel call actually made, and record no `params` or result body.
  > - **Backend instrumentation hooks:** language-native `tracing` spans on client call and server dispatch. Propagation uses the `corr` field in log lines (JSON-RPC `id` for `channel-request` / `channel-response`; `dialog_id`; send `cursor`), per the tests excerpt Log format.
  (.andromeda/obs-plan.md:190)
  > - **Required telemetry signal:** channel lines never carry `params` or result bodies, and `from` is logged as unauthenticated.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:198) A failed or unmakeable verification must be logged in the calling client's own process file (`hook-<name>`, `mcp`, `cli-<name>`, or none for a CLI verb with no instance), with the same file sink and fields on every OS, so the macOS euid + dir gap adds no OS-specific field.
  > - **Exporter:** The log file of whichever process is on each side (`run-<name>`, `hook-<name>`, `mcp`, or none for short-lived CLI verbs without an instance).
  (.andromeda/obs-plan.md:239)
  > - **Required telemetry signal:** the same file sink and line schema on every OS, with identical fields and no OS-specific exporter.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:201) The edited client connect must keep supplying the connection discriminator, so every channel call that is made still joins across logs on `(conn, corr)`.
  > JSON-RPC ids are "integers, monotonic per connection" (arch Standard Contracts, JSON-RPC frame shapes). `corr` for `channel-request` / `channel-response` is therefore not unique across connections or processes, and a merged-log join needs `process`, `instance` and a connection discriminator.
  (.andromeda/obs-plan.md:264)
  > the channel join key (D-25: `(conn, corr)`, each side logs its own `instance`)
- (.andromeda/obs-plan-detail/obs-scope-summary.md:209) The `send` / `wait` / `last` / `answer` tools' verification, made before their Tokio-client frames, must run inside each tool call's span.
  > - **Backend instrumentation hooks:** `tracing` spans per MCP tool call (`send · wait · last · answer · list`) wrapping the `viola-channel` Tokio client, through the tracing-subscriber JSON logger.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:214) An MCP tool whose verification fails must return `instance-unreachable` as an `isError` code only, mirrored by a codes-only line with no upstream text, absolute path or type name.
  > - `isError` results carry codes only: `refusal` / `detail`, `instance-unreachable`, `wrapper-fault`, `invalid-params` (its `detail` a closed field word; chunk 2026-10-17-mcp-server-for-drivers) (security Vector 5). Log lines follow the same codes-only rule for rejected calls (tests excerpt, Coverage Triggers, MCP).
  (.andromeda/obs-plan.md:205)
  > - **Required telemetry signal:** rejected tool calls are logged with codes only, with no upstream text, absolute paths or type names.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:229) Each client must run its verification after its role file has opened and its subscriber is installed, so a mismatch is logged; the strict-modes half, if P4 takes it into the clients, must settle its order against that log open, and the strict-modes line rule changes with it. The pid and start-time reads added to `viola-channel` and `viola-state` must stay sync, Tokio-free and free of C-building crates.
  > Init order per process:
  >   1. As the first statement in `main`, install a replacement panic hook that writes one JSON line and nothing to stderr. The default hook's stderr output breaks the hook contract and the `run` terminal.
  >   2. Parse clap.
  >   3. Resolve the home (`--home`, then the grandparent of `VIOLA_DIR`, then the default) and the instance (`VIOLA_NAME` or an argument).
  >   4. Open `diagnostics/<process file>` in append mode with 0600 permissions inside a 0700 directory.
  >   5. Install the subscriber with an explicit writer (never stdout).
  >   6. Emit `process-start`.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:225)
  > `cargo deny` bans Tokio in the sync crates' graph and bans C-building crates (arch Observability Hints, Build policy).
  (.andromeda/obs-plan.md:225)
  > - **Required telemetry signal:** obs dependencies stay Tokio-free in the sync crates, add no C-building crates and pass `cargo deny`.
  (.andromeda/obs-plan.md:59)
  > - **Instrumentability:** instrumentable, and the instrumentation must be sync: Tokio is forbidden in this crate's graph unless its feature is enabled.
  (.andromeda/obs-plan.md:63)
  > - **Instrumentability:** instrumentable, and the instrumentation must be sync: Stack bans Tokio here.
  (.andromeda/obs-plan.md:215)
  > a strict-modes failure has a line only on the `ledger-stamps` read in `run`'s version gate.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:264) A refused-before-frame outcome must be logged through the existing closed `event` values, or through a new one only with a Decisions Log entry; each line takes its event's `corr`, so a dialog hook refused before any `hook.dialog` response holds no `dialog_id`; and a swallowed hook-trace write must still end in exit 0 with nothing on stderr.
  > - `event`, a closed kebab-case enum. A new value needs a Decisions Log entry, like the §3 closed enums. Each value fixes what its `corr` holds:
  >     - `channel-request`, `channel-response`: `corr` = the JSON-RPC request `id`
  >     - `dialog-raised`, `dialog-answered`: `corr` = `dialog_id`
  >     - `hook-invoked`, `hook-decision`: `corr` = `dialog_id` for dialog hooks, otherwise null
  >     - `send-issued`, `send-confirmed`, `send-refused`: `corr` = the send `cursor`
  >     - `process-start`, `process-exit`, `panic`: `corr` = null
  (.andromeda/obs-plan-detail/obs-scope-summary.md:275)
  > The hook trace is the `hook-<name>.ndjson` file. A failed write there is swallowed, so the hook still exits 0 and never writes to stderr.
  (.andromeda/obs-plan.md:142)
  > - (c) `hook-invoked` is emitted before the wrapper assigns `dialog_id`, so `corr` can only be filled from `hook-decision` onward, or `hook-invoked` must be logged after the `hook.dialog` response.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:300) `run`'s collision check, whose `read_snapshot` call the chunk re-reads, must keep its distinct `already-live` exit-1 code with no path or pid; the new exit-21 cause, a failed server verification, must take its own distinct `detail` code on the same terms.
  > Exit-1 causes use a distinct `detail` code: already-live, squatted-name, sha-256 mismatch, `.cmd`/`.bat` child, and for a start made by `viola revive` also no-session, session-live and cwd-missing. A revive whose instance check refuses writes no line: the check precedes the log (chunk 2026-10-10-budget-governor). No path or pid appears in the detail.
  (.andromeda/obs-plan.md:253)
  > - **Required telemetry signal:** `release-from-driver` is its own logged event, and each cause behind exit 21 and exit 1 gets a distinct `detail` code with no path or pid.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:308) On the send path the verification must sit inside `send.client`, ahead of `channel.request`: a send refused by it writes no `channel-request` line, since no frame is written, and none of the later send events.
  > - **Must-trace spans:** `send.client` (cli/mcp) › `channel.request` › `run.readiness_gate` (vt100) › `pty.paste_write` › `run.confirm_window` (not opened for an unconfirmable send). Log events: `channel-request`, then `send-issued`, then `hook-invoked` (UserPromptSubmit, or SessionStart for a `/clear` confirmed by its new session or a `/compact` confirmed by its same session (verified CLI); none awaited for an unconfirmable send), then `send-confirmed` or `send-refused`, then `channel-response`.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:307)
  > - **Surfaces involved:** cli or ipc-internal (mcp) → ipc-internal (channel) → cli (`run`, PTY paste) → cli (`hook` UserPromptSubmit; `hook` SessionStart for a `/clear` confirmed by its new session or a `/compact` confirmed by its same session (verified CLI); no hook for an unconfirmable send) → `events.ndjson`
- (.andromeda/obs-plan-detail/obs-scope-summary.md:309) A send refused before any frame by a failed verification must be logged codes-only, with no sent text and its code outside the wrapper's send `detail` list, which stays closed.
  > - **Required log fields:** `event`, `process`, `instance`, `corr` (the JSON-RPC `id` on channel lines; the send `cursor` on send lines), `from` (self-reported, not an identity), `confirmed`, `refusal`, `detail` (`input-not-ready` / `no-prompt-submitted` / `control-character` / `empty-text` / `turn-running`), `duration_ms`. Sent text never appears.
  (.andromeda/obs-plan.md:195)
  > - **Required telemetry signal:** `send` text never reaches stderr, `--json`, MCP or any process-log field outside `instances/<name>/diagnostics/`, and each deciding side's `send-refused` carries no text.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:317) On the wait / last path the verification must sit inside `wait.block`, ahead of `channel.request`: a refused `wait` or `last` writes no `channel-request` / `channel-response` pair, and its outcome is the client's own exit-21 or `instance-unreachable` line.
  > - **Must-trace spans:** `wait.block` (client) › `channel.request` › `run.wait_dispatch`. Log events: `channel-request` and `channel-response` with the outcome.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:316)
  > - **Surfaces involved:** cli or ipc-internal (mcp) → ipc-internal (channel) → cli (`run`) + persistent store (`events.ndjson` tail)

## Bindings
- **obs-scope-summary's hook fail-open items ↔ log-coverage**: the hook's mismatch outcome is a `hook-decision` with a fail-open `detail` at WARN, and a CLI verb's is `process-exit` exit 21 at WARN, in the §6 levels map (.andromeda/obs-plan.md:460), each new code added to the closed catalog (.andromeda/obs-plan.md:451)
- **obs-scope-summary's send and wait / last paths ↔ span-trace-coverage**: the wait / last scenario's note that `server-verify-failed` "cannot occur … until client-side server verification lands" belongs to this entry and changes with it (.andromeda/obs-plan.md:361); the dialog scenario writes a dialog hook's `hook-invoked` only after the `hook.dialog` response, which never comes on a refused verification (.andromeda/obs-plan.md:369)
- **obs-scope-summary's hook budget ↔ slo-invariants-telemetry-budgets**: the added reads stay inside the hyperfine `max` gated by `gate --require perf` for `session-end` and the spine and `pre-tool-use` rows (.andromeda/obs-plan.md:649)
- **obs-scope-summary's channel client items ↔ slo-invariants-telemetry-budgets**: every channel call that is made logs `corr` + `conn` on both sides (.andromeda/obs-plan.md:644)
- **obs-scope-summary's verification failure lines ↔ error-capture-reporting**: a new typed verification error that crosses a crate seam ends in one logged codes-only `process-exit{detail}` or `hook-decision{detail}` (.andromeda/obs-plan.md:533), and an MCP reject is an `isError` code mirrored by a codes-only line (.andromeda/obs-plan.md:537)
- **obs-scope-summary's sync-crate rule ↔ log-coverage per-module levels**: `sysinfo` and `interprocess` stay `OFF`, so a failed start-time or peer-credential read is captured at viola's seam as a viola event (.andromeda/obs-plan.md:470)
- **obs-scope-summary's closed `event` enum ↔ obs-anti-patterns and the Decisions Log**: no `event` value is added without a Decisions Log entry (.andromeda/obs-plan.md:715)
- **obs-scope-summary's server pid and endpoint ↔ pii-scrubbing-compliance**: home-level lines stay path-free and `endpoint` is never logged (.andromeda/obs-plan.md:557); `pid` is a join key, never a grouping key (.andromeda/obs-plan.md:420)
