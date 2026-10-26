# arch · cascade read · group 3 of 3

## Carried
- (.andromeda/architecture-detail/conventions.md:8) The server-verification failure must stay outside the closed refusal set: it is no new `RefusalReason` or `detail`, and the CLI's own pre-frame checks keep their place in the fixed refusal order of `send` and `answer`.
  > - **`RefusalReason` (serialised kebab-case):** `human-typing` · `budget-paused` · `unverified-cli` · `not-delivered` · `unknown`.
  (.andromeda/architecture-detail/conventions.md:15)
  > When several refusals apply, the wrapper returns only the first in this order. For `send`: `not-delivered`/`control-character` first (`viola_core::validate_paste_text`: LF, CR and TAB pass; every other C0, DEL and C1 is refused, never stripped; the CLI checks before any frame and the wrapper checks again)
  (.andromeda/architecture-detail/conventions.md:15)
  > For `answer`: `not-delivered`/`control-character` first (`validate_paste_text` on the response's free text; the CLI checks before any frame and the wrapper checks again)
  (.andromeda/architecture.md:98)
  > **`RefusalReason` (serialised kebab-case):** `human-typing` · `budget-paused` · `unverified-cli` · `not-delivered` · `unknown` (`#[serde(other)]`), each `detail` from a closed set per reason, extended only in `viola-core`
- (.andromeda/architecture-detail/conventions.md:34) A mismatch, or a check that cannot be made, must exit every CLI frame verb with 21 and the `instance-unreachable` document, `wait` / `last` keeping exit 1 only for the channel errors the rule leaves there, while `viola hook` stays exit 0 with no body.
  > `21` instance unreachable (no live endpoint for the `ViolaName`)
  (.andromeda/architecture-detail/conventions.md:26)
  > for `viola wait` / `viola last` exit 1 also covers a refusal reply, a reply of no known shape, and any channel error but a failed connect or a closed / reset connection (those are exit 21)
  (.andromeda/architecture.md:100)
  > `21` instance unreachable, and a refused strict-modes check of `viola list` or `viola plugin install`.
  (.andromeda/architecture.md:225)
  > - `{"v":1,"error":"instance-unreachable","detail":null}` (exit 21)
  (.andromeda/architecture.md:102)
  > - **`viola hook` exit codes:** always 0, including on clap parse errors, channel failures and panics caught at the edge. Exit 2 is forbidden.
  (.andromeda/architecture.md:405)
  > Any internal failure in `hook` produces exit 0 with no body, so the dialog renders for the human.
- (.andromeda/architecture-detail/conventions.md:40) The `viola mcp` frame tools' mismatch outcome must be the `instance-unreachable` `isError: true` result with `detail: null`, text `refused: instance-unreachable`, no frame written, and the check must stay inside the Tokio client's crate while the hook and CLI checks stay blocking.
  > An unreachable instance and a wrapper protocol fault are also `isError: true` tool results. Their `structuredContent` is `{"error":"instance-unreachable"|"wrapper-fault","detail":…}`, the same as CLI `--json` (`instance-unreachable` with `detail: null`; `wrapper-fault` with the JSON-RPC fault's `{code,message,data}`).
  (.andromeda/architecture-detail/conventions.md:40)
  > every `isError` text is `refused: <code>`.
  (.andromeda/architecture.md:104)
  > a refusal, an unreachable instance, a wrapper protocol fault and an argument that fails re-validation (`invalid-params`, no frame written) are `isError: true` results shaped like CLI `--json`
  (.andromeda/architecture.md:410)
  > **Tokio containment.** New code on the `run`, `hook`, `send` or channel-server paths uses std threads and blocking I/O. Only `viola-mcp` may build a runtime.
- (.andromeda/architecture-detail/standard-contracts.md:24) The check must compare the connected server with the snapshot's own `pid` and `started_at`, and, behind the held item, an unreadable or newer snapshot must keep reading as absent for every client (no replayed `endpoint`, `pid` or `started_at` to verify against), so the verb exits 21 and the hook fails open.
  > A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored, and the state is rebuilt by replaying `events.ndjson`. Replay recovers only log-derived fields: `links`, `agent_session_id`, `wheel`, `budget_paused` and `budget_override_until`. Fields no event carries (`endpoint`, `pid`, `started_at`, `pinned_bin`, `statusline_command`, `cli_version`, `cli_verified`, `child_pid`, `pending_dialog`, `cwd`) are treated as absent. Liveness then rests on the heartbeat alone, so the instance is `live` or gone and never `stale`.
  (.andromeda/architecture-detail/standard-contracts.md:25)
  > `read_snapshot` keeps its results, and its five product callers (`src/cmd/client.rs`, `run`'s collision check, `src/cmd/hook.rs` twice, `src/cmd/hook/statusline.rs`; the rule: `read_snapshot(` under `src/`, outside test modules) still read every such snapshot as absent.
  (.andromeda/architecture.md:156)
  > `started_at` is the wrapper's own OS start time (sysinfo, whole seconds) rendered RFC 3339, so a reader can compare it with the same field of the recorded `pid`.
  (.andromeda/architecture.md:157)
  > - `pid` and `started_at` identify the wrapper process for the liveness check.
  (.andromeda/architecture.md:243)
  > When the snapshot's pid + start time no longer name a running process (the pid is dead, or alive with another start time) the session is `gone`, whatever the beat's age.
- (.andromeda/architecture-detail/standard-contracts.md:59) The hook client's verification must precede the id-less `hook.event` notification of every event, which no reply confirms, so a refused or failed check must count as not delivered and leave SessionEnd's direct-append fallback reachable, and `hook.dialog`'s failed check must leave the dialog to the human.
  > - `hook.event`, sent id-less by `hook` (a notification, no reply): `{ts, event:{kind, data}}`.
  (.andromeda/architecture.md:91)
  > Requests: `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.dialog`. `list` is not a channel method. Notifications: `hook.event`.
  (.andromeda/architecture.md:197)
  > `null` means no decision, so the hook prints nothing.
- (.andromeda/architecture-detail/standard-contracts.md:79) The statusline arm's `read_snapshot` call the chunk re-reads must keep its place after the arm's instance check, whatever reader it takes.
  > it canonicalises the instance directory, runs the instance check (`viola_state::strict::check_instance`), opens its log, reads stdin through `take(MAX_FRAME + 1)`, writes `budget.json` when the payload holds a `rate_limits` object, reads the snapshot, then runs the command.
- (.andromeda/architecture-detail/occupied-resources.md:8) Each client must verify the server it reached at the snapshot's recorded `endpoint`, and a rebuilt path (missing or unreadable snapshot) must never become a basis for a frame while the hold stands.
  > `run` resolves the path once and records it as `endpoint` in the instance snapshot. Every other process (`hook`, `mcp`, CLI verbs, and `run`'s collision check) connects to the recorded `endpoint`, because processes spawned inside the session may run with a different environment from the wrapper. It rebuilds the path by the same rule only when the snapshot is missing or unreadable.
  (.andromeda/architecture.md:260)
  > - **Windows:** named pipe `\\.\pipe\viola-<h12>`, one per `viola run`.
  (.andromeda/architecture.md:263)
  > every other process connects to the `endpoint` recorded in the instance snapshot.
  (.andromeda/architecture.md:129)
  > It is hand-written in `viola-channel` and recorded in the instance snapshot.
  (.andromeda/architecture.md:420)
  > Any process that can open an instance's endpoint can `send` prompts and `answer` dialogs, including PermissionRequest. v1 trusts every process of the same OS user and no other.
- (.andromeda/architecture-detail/occupied-resources.md:53) The hook's verification must read the snapshot of the instance `VIOLA_DIR` names, the same directory its endpoint and SessionEnd direct-append fallback come from, and the CLI verbs and `mcp` the one their resolved home names.
  > It is read by `hook` (for the recorded `endpoint`, the SessionEnd direct-append fallback, the statusline wrapper and the diagnostics directory) and by `mcp` and every CLI verb run inside the session.
  (.andromeda/architecture.md:298)
  > `hook`, `mcp` and every CLI verb run inside the session derive the viola home as its grandparent, so a session started with `--home` keeps its hooks, MCP server and nested CLI calls in that home.
- (.andromeda/architecture-detail/occupied-resources.md:91) The mismatch outcome's process-log lines must be recorded in these role-file lists, codes only (the CLI verbs' exit-21 `process-exit` detail, `mcp.ndjson`'s lines, `hook-<name>.ndjson`'s kinds), and `hook` must write nothing to stdout or stderr on a failed check.
  > (`cli-<name>.ndjson`: `send` / `wait` / `last` / `answer` / `pause` / `release` — `<name>` the target instance — write `process-start`, the client `channel-request` / `channel-response` lines (`wait`'s carry `after` / `timeout_ms` / `outcome`) and `process-exit{subject:"self"}`, exit 21 with `detail:"instance-dead"` + `during` at WARN, exit 20 with `detail:"wrapper-fault"`;
  (.andromeda/architecture-detail/occupied-resources.md:93)
  > `mcp.ndjson` (chunk 2026-10-17-mcp-server-for-drivers), written by `viola mcp` once its home and `diagnostics/` passed `check_path` (a refusal opens no role file and writes no line): `process-start{subject:"self"}`, the Tokio client's `channel-request` / `channel-response` lines,
  (.andromeda/architecture-detail/occupied-resources.md:93)
  > `hook-<name>.ndjson`: `hook-invoked`, `hook-decision`, `parse-rejected`;
  (.andromeda/architecture.md:412)
  > `hook` writes only its decision body to stdout and nothing to stderr;
- (.andromeda/architecture-detail/occupied-resources.md:100) If P4 takes the strict-modes half here, each client's `check_instance` must run before any log opens, and the readers list and the call-site count must record every new call site; the Windows DACL case stays owed to "Home and code-bearing file integrity".
  > Both checks come first because opening a log sets an existing home to 0700 (as measured on the statusline arm at chunk 2026-10-10-statusline-pass-through, `evidence/red-green.md` §3), and a check made after it passes a home at 0770.
  (.andromeda/architecture-detail/occupied-resources.md:100)
  > `check_instance` has these two product call sites (the rule: `check_instance(` under `src/`).
  (.andromeda/architecture-detail/occupied-resources.md:99)
  > Three readers check these files before they use them.
  (.andromeda/architecture-detail/occupied-resources.md:101)
  > No case widens the DACL of an instance tree on Windows yet; that case is owed to the working-route entry "Home and code-bearing file integrity".
  (.andromeda/architecture.md:325)
  > `viola list`, `viola revive` and `hook statusline` check these files before they use them, ahead of any log, because opening a log narrows an existing home.

## Bindings
- **conventions exit 21 ↔ conventions MCP result ↔ cross-cutting fail open**: one failed or impossible check maps to three outcomes by client: CLI exit 21 with the `instance-unreachable` document (.andromeda/architecture-detail/conventions.md:34), MCP `instance-unreachable` `isError` with no frame (.andromeda/architecture-detail/conventions.md:40), `hook` exit 0 with no body (.andromeda/architecture.md:405).
- **standard-contracts snapshot envelope ↔ occupied-resources IPC endpoints**: an unreadable or newer snapshot carries no `endpoint`, `pid` or `started_at` (.andromeda/architecture-detail/standard-contracts.md:24) while the endpoint rule rebuilds a path in exactly that case (.andromeda/architecture-detail/occupied-resources.md:8); under the hold no client verifies or frames on that rebuilt basis, the outcome staying absent / exit 21 / fail open.
- **standard-contracts `hook.event` ↔ occupied-resources `VIOLA_DIR`**: the notification has no reply (.andromeda/architecture-detail/standard-contracts.md:59), so the hook's own verification decides whether SessionEnd's direct-append fallback, reached through `VIOLA_DIR` (.andromeda/architecture-detail/occupied-resources.md:53), runs.
- **occupied-resources strict-modes readers ↔ standard-contracts statusline arm order**: the check-before-log order (.andromeda/architecture-detail/occupied-resources.md:100) is the same order the statusline arm keeps around its snapshot read (.andromeda/architecture-detail/standard-contracts.md:79); a strict-modes half taken here holds that order in every client.
- **occupied-resources role-file lines ↔ conventions exit codes**: the exit-21 `process-exit` detail the CLI verbs log (.andromeda/architecture-detail/occupied-resources.md:91) and the exit-21 code (.andromeda/architecture-detail/conventions.md:34) name one outcome; a mismatch detail added to one is recorded in both.
