# obs · cascade read · group 3 of 9

## Carried
- (.andromeda/obs-plan-detail/obs-scope-summary.md:293) The detail-file leg of `v1-47` that the chunk re-reads must hold these modes under the scan and at open, and a refused open must stay a dropped line, never a path in an error body.
  > Every file is 0600 inside a 0700 directory (Windows: home DACL), and the tests secret-scan test enforces it. The product holds it at open too: `open_role_file` and `write_detail` run `strict::check_diagnostics` on the existing `diagnostics/` folder and on the role or detail file before anything is created or narrowed — a symlink refused, a folder held to the read rule (Unix: no group or other bit; Windows: no read grant to another SID), a file to the read rule on Unix and the write rule on Windows — and a refusal is a failed open (no role file opened; a detail line dropped).
  (.andromeda/obs-plan-detail/obs-scope-summary.md:289)
  > If `mcp.ndjson` sits at home level, it must never carry user content.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:291)
  > `mcp` always writes `<home>/diagnostics/mcp.ndjson`, as tests assume (D-09; stderr JSON only when the file cannot be opened; landed at chunk 2026-10-17-mcp-server-for-drivers).
  (.andromeda/obs-plan.md:144)
  > a home-level `mcp.ndjson` never carries user content; every file is 0600 inside a 0700 directory; there is no rotation in v1.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:301) Either outcome of the `Refusal` carry must leave the `.cmd`/`.bat` child's distinct `detail` code standing, and every `run` and revive body the enumeration names must carry no path or pid, whichever the exit-cause catalogue chunk then owns.
  > Exit-1 causes use a distinct `detail` code: already-live, squatted-name, sha-256 mismatch, `.cmd`/`.bat` child, and for a start made by `viola revive` also no-session, session-live and cwd-missing. A revive whose instance check refuses writes no line: the check precedes the log (chunk 2026-10-10-budget-governor). A `run` or a revived start whose home-tree check (`check_home_tree`, through `checked_exe`) refuses writes no line either, so that exit-1 cause has no `detail` code: the check precedes the wrapper log, and the output is the home pair (a home-level refusal) or the instance pair (an instance-level one), revive `--json` the `strict-modes-failed` document. No path or pid appears in the detail.
  (.andromeda/obs-plan.md:161)
  > exit-1 causes use a distinct `detail` code, with no path or pid in it.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:309) The way the `channel-response` line comes to tell `-32602` causes apart must keep this line's place at the end of a send's log events.
  > Log events: `channel-request`, then `send-issued`, then `hook-invoked` (UserPromptSubmit, or SessionStart for a `/clear` confirmed by its new session or a `/compact` confirmed by its same session (verified CLI); none awaited for an unconfirmable send), then `send-confirmed` or `send-refused`, then `channel-response`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:310) `send`'s error bodies on every surface must carry these closed codes only, must hold `from` as self-reported, and must carry no sent text.
  > `from` (self-reported, not an identity), `confirmed`, `refusal`, `detail` (`input-not-ready` / `no-prompt-submitted` / `control-character` / `empty-text` / `turn-running`), `duration_ms`. Sent text never appears.
  (.andromeda/obs-plan.md:165)
  > sent text never appears.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:318) The `channel-response` line's contract read must keep this pair and its outcome on the `wait` / `last` path.
  > Log events: `channel-request` and `channel-response` with the outcome.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:319) `wait`'s error bodies must stay to `instance-unreachable` and exit 21, with no assistant message in any of them.
  > `outcome` (woken kind / `timed_out` / `instance-unreachable`), `exit_code` (21 when the wrapper is killed mid-wait), `duration_ms`. The `last_assistant_message` content never appears.
  (.andromeda/obs-plan.md:169)
  > the `last_assistant_message` content never appears.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:328) `answer`'s bodies on the CLI and MCP surfaces, including its refused instance check under `--json`, must be these codes and this document, with no pid, start time, path, endpoint or dialog text.
  > `refusal` / `detail` (`unknown-dialog` exit 13, `unverified-cli` exit 12, `server-verify-failed` on Linux and Windows: exit 21 on CLI `answer` with `during:"connect"`, fail-open on the dialog hook with `corr` null and `decision_emitted:false`, no pid, start time, path or endpoint), `deadline_hit`, `duration_ms`. The question, answer and tool `input` text never appear. A refused instance check writes no line and so carries none of these fields: CLI `answer` (`refuse_untrusted`, before any frame) exits 21 with the instance pair, or the `instance-unreachable` document under `--json`; the dialog hook exits 0 at once with empty streams, no frame and no line.
  (.andromeda/obs-plan.md:173)
  > the question, answer and tool `input` text never appear.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:336) Telling the `-32602` causes apart must keep `release-from-driver` its own logged event, admitted by the schema, and keep the `pause` and `release` pair on the line whose contract is read.
  > Log events: `send-refused` and `channel-request` / `channel-response` for `pause` and `release`. `release-from-driver` is logged as its own event (Founder Direction 4), admitted by `schemas/diag-line.v1.json`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:346) The status-line cap-driving case must leave the command string out of every home-level file and error body, and `budget-paused` must stay the code a paused call's error body carries.
  > `refusal="budget-paused"`, `exit_code` (11), `shell_exit_status`, `duration_ms`. The `statusline_command` string never appears outside `diagnostics/`.
  (.andromeda/obs-plan.md:87)
  > its duration and exit status only; the command string is never logged outside `diagnostics/`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:354) The channel surface's witness and the `channel-response` change must hold this scrub, the fixed `-32603` and an unauthenticated `from`; any way of telling causes apart must stay a result-class value, never a body.
  > A scrub rule on `channel-request` / `channel-response`: method, `id` / `corr`, result class (`ok` / `refusal` / `error` code) and `sender` version only, never `params` or result bodies.
  >   - `-32603` is logged as the fixed "internal error".
  >   - `from` is logged as unauthenticated.
  (.andromeda/obs-plan.md:123)
  > `-32603` is `"internal error"` with `data:null`, and the self-reported `from` is never logged as an authenticated identity.
  (.andromeda/obs-plan.md:190)
  > channel lines never carry `params` or result bodies, and `from` is logged as unauthenticated.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:363) Send text must reach none of the chunk's surfaces, and each deciding side's `control-character` or `empty-text` refusal must stay without the text.
  > A never-log rule for `send` text on stderr, `--json`, MCP and every process-log field outside `instances/<name>/diagnostics/`.
  >   - `send-refused` with detail `control-character` or `empty-text` is emitted by each deciding side, client and wrapper, without the text (tests excerpt, Coverage Triggers, Vector 2).
  (.andromeda/obs-plan.md:195)
  > `send` text never reaches stderr, `--json`, MCP or any process-log field outside `instances/<name>/diagnostics/`, and each deciding side's `send-refused` carries no text.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:372) The `v1-47` drift-report leg must hold at `diagnostics/` only, and a `server-verify-failed` the proof names must carry no pid, start time, path or endpoint.
  > A server-verify mismatch (Linux and Windows) is one `hook-decision{detail:"server-verify-failed"}` at WARN with no frame sent — on a dialog hook `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}` — and no pid, start time, path or endpoint in the line; macOS compares no peer pid (the recorded gap).
  >   - Exit is 0 and stderr stays empty.
  >   - Drift reports go to `diagnostics/` only.
  (.andromeda/obs-plan.md:67)
  > drift reports go to `diagnostics/` only, and R8-stripped `CLAUDE*` values are never logged anywhere.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:380) Every rejected `viola mcp` call must log codes only, with `detail` a closed field word, and a refused instance check must return `instance-unreachable` with a null `detail` and no line.
  > Rejected tool calls are logged with codes only (`refusal` / `detail` / `instance-unreachable` / `wrapper-fault` / `invalid-params`, whose `detail` is a closed field word `target · text · after · timeout_ms · dialog_id · response`; chunk 2026-10-17-mcp-server-for-drivers), with no upstream text, absolute paths or type names. One rejected call writes no line: a frame tool (`send`, `wait`, `last`, `answer`) whose instance check (`check_instance`, before `live_endpoint`) refuses returns `instance-unreachable` with `detail: null` and writes no frame.
  (.andromeda/obs-plan.md:127)
  > `isError` results and the log lines for rejected calls carry codes only.
  (.andromeda/obs-plan.md:205)
  > rejected tool calls are logged with codes only, with no upstream text, absolute paths or type names.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:387) The witnesses' planted stripped value must reach no sink, the strip must stay recorded by count or names, a serde-sourced chain must become the fixed message, and no switch may disable any of it.
  > A never-log floor for `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` and every R8-stripped `CLAUDE*` value, across all sinks and fixtures.
  >   - The R8 strip is logged as a count or variable-name list only, never values. Phase 3 must confirm names are admissible.
  >   - An anyhow chain containing a serde source becomes a fixed message before reaching stderr.
  >   - No env, flag or config may disable redaction.
  (.andromeda/obs-plan.md:47)
  > the only crate that uses anyhow, so the error-capture boundary: a chain that contains a serde source becomes a fixed message before it reaches stderr, and the full chain goes to `diagnostics/` only.
  (.andromeda/obs-plan.md:95)
  > the absolute paths in `snapshot.json` and config values are scrubbed from external output.
  (.andromeda/obs-plan.md:99)
  > static configuration and data; only the parse at load time is observable.
  (.andromeda/obs-plan.md:111)
  > refusal and fault lines on stderr are fixed strings with no paths, pids, chains or upstream text.
  (.andromeda/obs-plan.md:210)
  > a never-log floor for every R8-stripped `CLAUDE*` value across all sinks and fixtures; the strip is logged as a count or variable names only, never values, and Phase 3 confirms the names are admissible; a serde-sourced anyhow chain becomes a fixed message before stderr; no env, flag or config may disable redaction.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:397) The re-read owner-readable leg must hold these modes under the scan, and the enumeration must name the refused instance checks as bodies that leave no line.
  > `diagnostics/` files are 0600 in 0700 directories (Windows: home DACL), enforced by the tests secret-scan test.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:398)
  > the instance check of `send`, `wait`, `last`, `answer`, `pause` and `release` (`refuse_untrusted`, exit 21), `viola link` / `viola unlink`'s instance check of the driven instance (made before any role file opens; exit 21), the `viola mcp` frame tools' instance check (`instance-unreachable`), the `diagnostics/` opens (`check_diagnostics`: a refusal is a failed open)
- (.andromeda/obs-plan-detail/obs-scope-summary.md:407) The child pipe drain's cap-driving case must leave a past-cap or unparsable probe as `unknown` plus a counter, with the probes' lines kept and no row text in them.
  > `process-start` / `process-exit` for the `claude` child and for `claude agents --json` / `--version` probes.
  >   - Parse failures surface as `unknown` plus a counter.
  (.andromeda/obs-plan.md:83)
  > an external CLI: its spawn/exit and parse outcome; a parse failure becomes `"unknown"` plus a counter, and the output rows are untrusted.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:444) An endpoint that vanishes during `wait` must end in exactly this exit and code, one of the enumerated bodies.
  > endpoint vanished during `wait` (exit 21 / `instance-unreachable`);
- (.andromeda/obs-plan-detail/obs-scope-summary.md:453) The detail-file leg must be read by each OS's own permission form, and any field the `channel-response` line gains must be identical on every OS.
  > The same file sink and line schema on every OS. Platform differences must still yield identical fields:
  >   - permission application: Windows DACL vs Unix 0600/0700;
  (.andromeda/obs-plan-detail/obs-scope-summary.md:458)
  > the server check's peer-pid read: Linux `SO_PEERCRED` vs Windows `GetNamedPipeServerProcessId` vs macOS none (`SERVER_PID_REPORTED` false, so the verdict is `Matches` with no comparison and no `server-verify-failed` line is written there: the recorded gap, owned by "Unix endpoint and home hardening").
  (.andromeda/obs-plan.md:239)
  > the same file sink and line schema on every OS, with identical fields and no OS-specific exporter.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:481) Any change to the `channel-response` line must add to the adopted tests field and event list and rename or remove nothing.
  > The tests field and event list is adopted verbatim.
  (.andromeda/obs-plan.md:137)
  > (verbatim from tests excerpt, Section 3; binding. Obs may add fields and must not rename or remove any.)
- (.andromeda/obs-plan-detail/obs-scope-summary.md:488) `release-from-driver` must not stay a generic `-32602` fault once the causes are told apart, every named cause must carry no path or pid, and the `--json` documents and the `server-verify-failed` line must stay exactly as stated in the enumeration.
  > `release-from-driver` is its own logged event, not a generic -32602 wrapper fault.
  >   - Each cause behind exit 21 (dead instance, strict-mode fail, server-verify fail) and exit 1 (already live, squatted name, SHA-256 mismatch, `.cmd`/`.bat` child, and `viola revive`'s no logged session and missing recorded directory) gets a distinct `detail` code with no path or pid. `viola revive`'s strict-mode fail carries no logged `detail` as landed (no log line): its output is the stderr pair, or under `--json` the `{"v":1,"error":"strict-modes-failed","detail":null}` document (chunk 2026-10-17-cli-machine-contract), exit 1 (chunk 2026-10-10-budget-governor). `viola link` / `viola unlink`'s strict-mode fail (their instance check of the driven instance) carries no logged `detail` either (no line): its output is the fixed instance-state `unable` / `hint:` pair on stderr, or under `--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document, exit 21.
  >   - The server-verify fail is written on Linux and Windows as `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` in `cli-<name>.ndjson` by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` (for `link` / `unlink` the driven instance's file), distinct from `instance-dead`, with no pid, start time, path or endpoint; macOS writes none (no peer pid, the recorded gap).
  (.andromeda/obs-plan.md:253)
  > `release-from-driver` is its own logged event, and each cause behind exit 21 and exit 1 gets a distinct `detail` code with no path or pid, except a strict-modes check refused before any log opens, which writes no line (its cause is the fixed stderr pair or the `--json` document).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:498) The fix of the `-32602` over-count must cover every cause listed here, the `unknown` refusal must stay a code, and the two cap tests must meet the per-parser counts of the `viola mcp` stdin and `claude agents --json`.
  > The `RefusalReason` `unknown` fallback (exit 14).
  >   - Higher-`v` rejections (-32602; the code also carries `send`'s invalid params, `release` carrying `from`, and a `link` / `unlink` whose `driver` or `driven` is missing, not a string or no valid name, whose `driven` is not the serving wrapper's own name, or whose `driver` equals `driven`, so it is not a higher-`v` count alone).
  >   - Per-parser parse-failure counts: paste validation, channel framing at `MAX_FRAME`, the `viola mcp` stdin at `MAX_FRAME`, hook stdin, `claude agents --json`, vt100 under `catch_unwind`.
  (.andromeda/obs-plan.md:125)
  > its stdin read in lines of at most `MAX_FRAME`, a longer line refused whole and the session ended as at a closed stdin

## Bindings
- **`-32602` causes ↔ metric coverage**: the line must tell the causes listed at `(.andromeda/obs-plan-detail/obs-scope-summary.md:499)` apart so that `viola.compat.v_rejected` stops over-counting `(.andromeda/obs-plan.md:418)`; the metric and the line move together.
- **`channel-response` scrub ↔ log coverage**: the fields kept at `(.andromeda/obs-plan-detail/obs-scope-summary.md:354)` and any addition must be named in the §6 catalog, with the line's level by `error_code` kept `(.andromeda/obs-plan.md:459)` `(.andromeda/obs-plan.md:460)`.
- **`CLAUDE*` floor and serde chain ↔ error capture**: the fixed message at `(.andromeda/obs-plan-detail/obs-scope-summary.md:389)` is the capture layer at `(.andromeda/obs-plan.md:510)`, and the chain's only home is the detail file `(.andromeda/obs-plan.md:511)`.
- **Detail-file modes ↔ PII scrubbing and CI**: the owner-readable leg at `(.andromeda/obs-plan-detail/obs-scope-summary.md:397)` pairs with the detail-file schema `(.andromeda/obs-plan.md:577)`, and the floor's proof is the scan over its named inputs `(.andromeda/obs-plan.md:621)`, whose hit report names no matched bytes `(.andromeda/obs-plan.md:572)`.
- **Parse failure as `unknown` ↔ log coverage**: the counter at `(.andromeda/obs-plan-detail/obs-scope-summary.md:408)` is the one `parse-rejected{parser:"claude-agents-json"}` at `(.andromeda/obs-plan.md:482)`, with rows never logged verbatim `(.andromeda/obs-plan.md:557)`.
