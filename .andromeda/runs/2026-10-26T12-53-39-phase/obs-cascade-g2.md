# obs · cascade read · group 2 of 5

## Carried
- (.andromeda/obs-plan-detail/obs-scope-summary.md:196) The `link` / `unlink` verbs' client call and the wrapper's dispatch of the two methods must be traced by `tracing` spans and correlated by `corr` = the JSON-RPC `id`, their `params` (`driver`, `driven`) carrying no trace-context field.
  > - **Backend instrumentation hooks:** language-native `tracing` spans on client call and server dispatch. Propagation uses the `corr` field in log lines (JSON-RPC `id` for `channel-request` / `channel-response`; `dialog_id`; send `cursor`), per the tests excerpt Log format. Channel `params` declare no trace-context field (arch Standard Contracts, `send`: "No trace-context field is declared").
  (.andromeda/obs-plan.md:148)
  > - **CLI / MCP / hook → wrapper (local socket):** no W3C `traceparent`, because channel `params` declare no trace-context field (arch Standard Contracts). The correlation key is the tests `corr` (JSON-RPC `id`, `dialog_id`, send `cursor`) plus `process` and `instance`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:198) The wrapper's `channel-request` / `channel-response` for `link` and `unlink` must go to its own `run-<name>` file, and the verbs' lines only to the driven instance's role file when it resolves, with no process-log file when none does.
  > - **Exporter:** The log file of whichever process is on each side (`run-<name>`, `hook-<name>`, `mcp`, or none for short-lived CLI verbs without an instance).
  (.andromeda/obs-plan.md:109)
  > - **Surface:** cli (short-lived verbs: `send · wait · last · list · answer · pause · release · link · unlink · verify · plugin install · revive --list`)
  (.andromeda/obs-plan.md:111)
  > - **Exporter:** a file sink only, JSON-per-line to `diagnostics/` when an instance resolves — no network OTLP (viola makes no outbound network calls) and no stdout exporter (stdout carries `--json` results); the instance comes from `VIOLA_NAME` or an argument, never from the working directory; refusal and fault lines on stderr are fixed strings with no paths, pids, chains or upstream text.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:199) Each side of a `link` / `unlink` call must log its own `process` (`cli` on the verb, `run` on the wrapper), the peer's version read only as `sender`.
  > - **Service identity:** Each side logs its own `process`. The peer version is available as `params.sender` (`CARGO_PKG_VERSION`).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:200) The two methods' channel lines must be joinable across connections by the connection discriminator, a `-32603` on them must stay the fixed `"internal error"` with `data:null`, and the self-reported driver name on a `link` / `unlink` frame must never be logged as an authenticated identity.
  > - **Notes:**
  >   - JSON-RPC ids are "integers, monotonic per connection" (arch Standard Contracts, JSON-RPC frame shapes). `corr` for `channel-request` / `channel-response` is therefore not unique across connections or processes, and a merged-log join needs `process`, `instance` and a connection discriminator. This is a gap for Phase 3; the tests contract allows additive fields.
  >   - `-32603` must be `"internal error"` with `data:null` (security Vector 1).
  >   - `from` is self-reported and must not be logged as an authenticated identity (security Vector 1).
  (.andromeda/obs-plan.md:123)
  > - **Exporter:** the log file of whichever process is on each side; propagation is the `corr` field in log lines, since channel `params` declare no trace-context field, and a merged-log join needs `process`, `instance` and a connection discriminator, a gap for Phase 3; `-32603` is `"internal error"` with `data:null`, and the self-reported `from` is never logged as an authenticated identity.
  (.andromeda/obs-plan.md:264)
  > the channel join key (D-25: `(conn, corr)`, each side logs its own `instance`)
- (.andromeda/obs-plan-detail/obs-scope-summary.md:229) Each new verb must resolve the home and its instance (the driven argument) before it opens `diagnostics/<process file>`, install the subscriber on an explicit writer and only then emit `process-start`, so a refusal before step 4 leaves no file.
  >   Init order per process:
  >   1. As the first statement in `main`, install a replacement panic hook that writes one JSON line and nothing to stderr. The default hook's stderr output breaks the hook contract and the `run` terminal.
  >   2. Parse clap.
  >   3. Resolve the home (`--home`, then the grandparent of `VIOLA_DIR`, then the default) and the instance (`VIOLA_NAME` or an argument).
  >   4. Open `diagnostics/<process file>` in append mode with 0600 permissions inside a 0700 directory.
  >   5. Install the subscriber with an explicit writer (never stdout).
  >   6. Emit `process-start`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:259) The `unlink` method's product path must append exactly one arch Event line per removed pair to `events.ndjson`, the product stream kept apart from the process logs (no event payload copied into a process-log line).
  > - **Format:** JSON-per-line. There are two streams:
  >   - **Event stream:** product events in `instances/<name>/events.ndjson`, exactly the arch Event line `{"v":1,"ts","instance","kind","source":"hook|wrapper|cli","data"}`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:264) Any `event` value or line the plan adds for the explicit methods (changing edge flow E3's "no `event` value") must take a Decisions Log entry, and the two methods' channel lines carry `corr` = the JSON-RPC request `id`.
  >   - `event`, a closed kebab-case enum. A new value needs a Decisions Log entry, like the §3 closed enums. Each value fixes what its `corr` holds:
  >     - `channel-request`, `channel-response`: `corr` = the JSON-RPC request `id`
  (.andromeda/obs-plan.md:183)
  > Tests E1 (unwrapped hook no-op), E2 (R8 env strip), E3 (`link` / `unlink` transfer markers) and E5 (snapshot corruption → replay) are covered by the section 5 triggers and by E2E tracing expansion in Phase 3 if needed.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:284) The verbs' process log must be the home-level `cli-<name>.ndjson` of the driven instance, holding no user content.
  > - **Log file location:** Tests require `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:352) The wrapper's and the verbs' `channel-request` / `channel-response` for `link` and `unlink` must carry only method, `corr`, result class and `sender`, never `driver`, `driven`, `since` or `already_linked`, with `-32603` fixed and the driver unauthenticated.
  > - **Required telemetry signal:**
  >   - A scrub rule on `channel-request` / `channel-response`: method, `id` / `corr`, result class (`ok` / `refusal` / `error` code) and `sender` version only, never `params` or result bodies.
  >   - `-32603` is logged as the fixed "internal error".
  >   - `from` is logged as unauthenticated.
  (.andromeda/obs-plan.md:190)
  > - **Required telemetry signal:** channel lines never carry `params` or result bodies, and `from` is logged as unauthenticated.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:388) A verb that fails to decode the wrapper's `link` / `unlink` result must print only the fixed message, never a serde-sourced chain, on stderr or under `--json`.
  >   - An anyhow chain containing a serde source becomes a fixed message before reaching stderr.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:397) Should P4's form put the strict-modes instance check ahead of the verbs' snapshot read, its refusal's logging must be decided against this rule: either the verbs join the refusals that leave no line, or the line it gets is added here with its `detail` code.
  >   - Refusal telemetry for strict-modes failures, pinned-exe hash mismatches and ignored symlinks. As landed a strict-modes failure has a line only on the `ledger-stamps` read in `run`'s version gate. Four strict-modes refusals leave no line: `viola revive`'s instance check, `viola hook statusline`'s instance check, the checked read of `budget.json` (`read_budget_stamped`, which the budget gate takes through `read_budget` and `viola list` takes directly, where a refused file is no reading) (chunk 2026-10-10-budget-governor), and `viola list`'s scan of the home, `instances/` and each instance, which refuses the whole board with exit 21 (chunk 2026-10-11-the-board-viola-list).
  (.andromeda/obs-plan.md:215)
  > - **Required telemetry signal:** `diagnostics/` files are 0600 in 0700 directories (Windows: home DACL), plus refusal telemetry for pinned-exe hash mismatches and ignored symlinks; a strict-modes failure has a line only on the `ledger-stamps` read in `run`'s version gate.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:407) The verbs' human result lines render logged link names, so each name must pass `ViolaName::try_new` before it is shown, no C0/C1 control reaching the terminal raw.
  >   - C0/C1 controls are escaped in any human terminal rendering of logged rows.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:422) The `link` / `unlink` channel lines on both sides must carry the connection discriminator beside `corr`, since their JSON-RPC `id` is per connection.
  >   - The per-connection JSON-RPC `id` needs a discriminator for a merged-log join (section 2).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:456) The verbs' server check through `client::connect` must yield the same `server-verify-failed` line on Linux and Windows, and none on macOS, the recorded gap.
  >   - the server check's peer-pid read: Linux `SO_PEERCRED` vs Windows `GetNamedPipeServerProcessId` vs macOS none (`SERVER_PID_REPORTED` false, so the verdict is `Matches` with no comparison and no `server-verify-failed` line is written there: the recorded gap, owned by "Unix endpoint and home hardening").
- (.andromeda/obs-plan-detail/obs-scope-summary.md:487) Each cause behind the verbs' exit 21 and exit 1 must get a distinct closed `detail` code with no path or pid, and the list of verbs writing `process-exit{… detail:"server-verify-failed", during:"connect"}` must gain `link` and `unlink`.
  >   - Each cause behind exit 21 (dead instance, strict-mode fail, server-verify fail) and exit 1 (already live, squatted name, SHA-256 mismatch, `.cmd`/`.bat` child, and `viola revive`'s no logged session and missing recorded directory) gets a distinct `detail` code with no path or pid. `viola revive`'s strict-mode fail carries no logged `detail` as landed (no log line): its output is the stderr pair, or under `--json` the `{"v":1,"error":"strict-modes-failed","detail":null}` document (chunk 2026-10-17-cli-machine-contract), exit 1 (chunk 2026-10-10-budget-governor).
  >   - The server-verify fail is written on Linux and Windows as `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` in `cli-<name>.ndjson` by `send`, `wait`, `last`, `answer`, `pause` and `release`, distinct from `instance-dead`, with no pid, start time, path or endpoint; macOS writes none (no peer pid, the recorded gap).
  (.andromeda/obs-plan.md:253)
  > - **Required telemetry signal:** `release-from-driver` is its own logged event, and each cause behind exit 21 and exit 1 gets a distinct `detail` code with no path or pid.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:497) The two new `-32602` refusals (a `driven` that is not the serving wrapper's name, a `driver` equal to `driven`) must be named among the causes the higher-`v` rejection signal over-counts.
  >   - Higher-`v` rejections (-32602; the code also carries `send`'s invalid params and `release` carrying `from`, so it is not a higher-`v` count alone).

## Bindings
- **channel scrub and join (obs-scope-summary) ↔ log-coverage**: the two methods' `channel-request` / `channel-response` keep the boundary-call wrapper's field set with no argument digest (.andromeda/obs-plan.md:479), and their `-32602` response sits at `warn` (.andromeda/obs-plan.md:460), the level the scrub rule's result class reports (.andromeda/obs-plan-detail/obs-scope-summary.md:352).
- **Founder Direction 4 codes (obs-scope-summary) ↔ log-coverage**: each new `detail` code behind the verbs' exit 21 and exit 1 (.andromeda/obs-plan-detail/obs-scope-summary.md:487) joins the closed `detail` code catalog (.andromeda/obs-plan.md:451).
- **higher-`v` over-count (obs-scope-summary) ↔ metric-coverage**: the two new `-32602` causes (.andromeda/obs-plan-detail/obs-scope-summary.md:497) move the same over-count note on `viola.compat.v_rejected` (.andromeda/obs-plan.md:418), and `method` stays a closed grouping key (.andromeda/obs-plan.md:420).
- **client and dispatch spans (obs-scope-summary) ↔ telemetry-strategy**: the verbs' client span (.andromeda/obs-plan-detail/obs-scope-summary.md:196) needs a `<area>` the span-naming list holds, `cli.<verb>` or a new area (.andromeda/obs-plan.md:293).
- **event stream and closed enum (obs-scope-summary) ↔ span-trace-coverage**: the `unlink` line in `events.ndjson` and any `event` value added (.andromeda/obs-plan-detail/obs-scope-summary.md:264) change edge flow E3's statement that the explicit methods are not built (.andromeda/obs-plan.md:392).
- **server-verify fail (obs-scope-summary) ↔ span-trace-coverage**: the verbs' `server-verify-failed` exit (.andromeda/obs-plan-detail/obs-scope-summary.md:487) takes the `wait` / `last` shape, at WARN inside the dispatch span and with no `channel-request` line (.andromeda/obs-plan.md:361).
- **fixed fault message (obs-scope-summary) ↔ error-capture-reporting**: the serde-chain rule (.andromeda/obs-plan-detail/obs-scope-summary.md:388) is met on the verbs by the short-lived `cli` internal-error path (.andromeda/obs-plan.md:523).
- **logged link names (obs-scope-summary) ↔ pii-scrubbing-compliance**: a re-parsed name shown on a result line (.andromeda/obs-plan-detail/obs-scope-summary.md:407) never becomes a field or `detail` content of a home-level line (.andromeda/obs-plan.md:557).
- **CLI exporter (obs-scope-summary) ↔ the harness contract**: the verbs' stderr refusals are the fixed strings of (.andromeda/obs-plan.md:111), written through `human::refuse`, whose per-verb call-site count changes with them (.andromeda/obs-plan.md:311).
