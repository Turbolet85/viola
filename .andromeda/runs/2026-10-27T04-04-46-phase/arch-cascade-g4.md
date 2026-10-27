# arch · cascade read · group 4 of 5

## Carried
- (.andromeda/architecture-detail/conventions.md:8) Every refusal the chunk's validators produce must be a value of the closed `RefusalReason` set with a `detail` from its reason's closed set, extended only in `viola-core`, and the paste rule must refuse `control-character` first and `empty-text` second, checked by the CLI before any frame and again by the wrapper before the first wheel read.
  > **`RefusalReason` (serialised kebab-case):** `human-typing` · `budget-paused` · `unverified-cli` · `not-delivered` · `unknown`. `unknown` is `#[serde(other)]`, so an older client can still read a refusal from a newer wrapper. `detail` values are kebab-case strings from a closed set per reason, extended only in `viola-core`:
  (.andromeda/architecture-detail/conventions.md:12)
  > `not-delivered`: `input-not-ready` · `no-prompt-submitted` · `turn-running` · `unknown-dialog` · `control-character` · `empty-text`
  (.andromeda/architecture-detail/conventions.md:15)
  > Readers treat an unrecognised `detail` as opaque text, never as an error. When several refusals apply, the wrapper returns only the first in this order. For `send`: `not-delivered`/`control-character` first (`viola_core::validate_paste_text`: LF, CR and TAB pass; every other C0, DEL and C1 is refused, never stripped; the CLI checks before any frame and the wrapper checks again), then `not-delivered`/`empty-text` (the typed text, the text with every CR LF pair and every other CR as one LF and without its trailing CR and LF characters ([Delivery Confirmation]), is empty; the CLI checks before any frame and the wrapper checks again, before the first wheel read; a text that also holds a refused character is still `control-character`), then `human-typing`,
  (.andromeda/architecture-detail/conventions.md:15)
  > For `answer`: `not-delivered`/`control-character` first (`validate_paste_text` on the response's free text; the CLI checks before any frame and the wrapper checks again), then `human-typing`, `unverified-cli`, `not-delivered`/`unknown-dialog`.
  (.andromeda/architecture.md:98)
  > each `detail` from a closed set per reason, extended only in `viola-core`, an unknown one opaque text to a reader; of several refusals only the first is returned, in a fixed order per verb, `control-character` first (any C0 but LF, CR and TAB, DEL or C1: refused, never stripped).
- (.andromeda/architecture-detail/conventions.md:27) `send`'s text read must be capped at `MAX_FRAME` with an over-cap or non-UTF-8 text a usage error (exit 2), the paste refusals must exit 13, and a `link` / `unlink` name the wrapper refuses `-32602` must exit 20, the self-pair refused by the wrapper only.
  > `2` usage error (clap, a second positional to `viola send` included; never produced by `viola hook`), and `viola send` text over `MAX_FRAME` or not UTF-8 (`error: the text must be readable UTF-8 of at most 16 MiB`)
  (.andromeda/architecture-detail/conventions.md:31)
  > `13` `not-delivered`
  (.andromeda/architecture-detail/conventions.md:33)
  > `20` protocol fault from the wrapper, a `-32602` from `link` / `unlink` included (a missing, non-string or invalid name, a `driven` that is not the serving wrapper's own name, or a `driver` equal to `driven`: the self-pair is refused by the wrapper only, never client-side)
  (.andromeda/architecture.md:100)
  > `2` usage error · `10` `human-typing` · `11` `budget-paused` · `12` `unverified-cli` · `13` `not-delivered` · `14` `unknown` refusal · `20` protocol fault from the wrapper
- (.andromeda/architecture-detail/conventions.md:52) The client-side paste rule the chunk re-verifies must refuse (exit 13) before any connect, so no frame is written for a refused text.
  > The client's own refusals (exit 13) and the strict-modes instance check (exit 21, `client::refuse_untrusted`: the target instance for `send`, `wait`, `last`, `answer`, `pause` and `release`, the driven instance for `link` / `unlink`) come first, before any connect, and the cause has no exit code, `--json` code or `RefusalReason` of its own.
- (.andromeda/architecture-detail/conventions.md:58) In the reached `src/cmd/send.rs` and `src/cmd/client.rs` the capped text read must stay ahead of the frame verbs' first step, the instance check, which opens nothing under the home and writes no line.
  > - Exit `21` also covers the refused strict-modes check of `send`, `wait`, `last`, `answer`, `pause` and `release` (`client::refuse_untrusted`: `check_instance` on the home and `instances/<name>`, the verb's first step, after its text read for `send`): human mode prints `link`'s instance-state `unable:` / `hint:` pair, and `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty; nothing under the home is opened and no line is written.
  (.andromeda/architecture-detail/standard-contracts.md:111)
  > - `viola send`, `viola wait`, `viola last`, `viola answer`, `viola pause` and `viola release` print the same `{"v":1,"error":"instance-unreachable","detail":null}` on stdout under `--json` (exit 21) for their refused strict-modes check of the home and the target instance, made by `client::refuse_untrusted` before anything under the home is opened, with stderr empty and no line in any log.
- (.andromeda/architecture-detail/conventions.md:64) Every MCP tool argument must be re-validated by its handler into a closed type (`target` through `ViolaName::try_new`, `u64` fields, `response` through `Response::parse`), a failure the `invalid-params` result naming a closed field word with no frame written, and an unknown tool name `-32602`.
  > As landed (chunk 2026-10-17-mcp-server-for-drivers): each handler re-validates its raw argument object (`target` through `ViolaName::try_new`, `text` a string, `after` / `timeout_ms` absent, `null` or a `u64`, `dialog_id` a `u64`, `response` an object through `Response::parse`), and a failure is an `isError: true` result `{"error":"invalid-params","detail":"<field>"}` (the closed field words `target` · `text` · `after` · `timeout_ms` · `dialog_id` · `response`) with no frame written; every `isError` text is `refused: <code>`. JSON-RPC errors are the MCP connection's own faults, an unknown tool name (`-32602`) and an internal handler fault
  (.andromeda/architecture.md:104)
  > a refusal, an unreachable instance, a wrapper protocol fault and an argument that fails re-validation (`invalid-params`, no frame written) are `isError: true` results shaped like CLI `--json`; JSON-RPC errors are only the MCP connection's own faults: an unknown tool name (`-32602`) and an internal handler fault (`-32603`).
  (.andromeda/architecture.md:128)
  > The instance key is `ViolaName`: ASCII `[a-z0-9-]`, 1–32 characters, starting with a letter, enforced by nutype.
  (.andromeda/architecture.md:188)
  > `permission`: `{behavior:"allow"|"deny", message?}`
  (.andromeda/architecture.md:189)
  > `plan`: `{behavior:"approve"|"revise", message?}`
- (.andromeda/architecture-detail/conventions.md:77) In the reached `crates/viola-mcp/src/tools.rs` a frame tool, MCP `send` with its client-side paste rule among them, must write no frame and no line before its checks pass, the instance check ahead of `live_endpoint`.
  > - A frame tool (`send`, `wait`, `last`, `answer`) first runs `viola_state::strict::check_instance` on the home and the target instance, before `live_endpoint`: a refusal returns the `instance-unreachable` `isError` result with `detail: null`, text `refused: instance-unreachable`, and writes no frame and no line.
- (.andromeda/architecture-detail/standard-contracts.md:10) The board must take a `claude agents --json` row's status only as a closed value, never treat an unwrapped row's `sessionId` name as a `ViolaName` or a valid `target`, and carry an unparseable budget value as `"unknown"`.
  > - `wrapped: false` marks a read-only unwrapped row from `claude agents --json`. Its `name` is the session identifier that `claude agents --json` reports (its `sessionId`; the join is ledger row `agents-join`). It is not a `ViolaName` and is never a valid `target`. Its `liveness` is `live` and `dialog_pending` is `false`, and `wheel`, `budget_paused`, `cli_version` and `cli_verified` are absent.
  >   - `liveness` is `live` or `stale`. Gone instances are omitted.
  >   - `status` is `idle`, `busy` or `unknown`.
  (.andromeda/architecture-detail/standard-contracts.md:14)
  > with either field `"unknown"` when unparseable and the whole window `"unknown"` when the statusline omits it
  (.andromeda/architecture.md:132)
  > **Unparseable external values** become the string `"unknown"` in viola's own records, never an error. This covers `resets_at`, `used_percentage` and a missing statusline window.
  (.andromeda/architecture.md:133)
  > An external budget reading's `used_percentage` is not that newtype: it is a `Reading<f64>` in `viola_core::BudgetWindow`, kept only from 0 to 100 and otherwise the word `"unknown"`.
- (.andromeda/architecture-detail/standard-contracts.md:24) `snapshot.json` must be read through the `MAX_FRAME` bound, its `v` read first, and an unsupported or unparseable envelope ignored and replayed from `events.ndjson`, never an error.
  > A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored, and the state is rebuilt by replaying `events.ndjson`.
  (.andromeda/architecture-detail/standard-contracts.md:24)
  > `snapshot::read_snapshot_classified` tells four cases apart through the same `MAX_FRAME` bound: the snapshot, no file, an unreadable one, and an unsupported one with the `v` it saw. It reads the envelope's `v` first: above 1 is unsupported whatever else the envelope holds, 1 must then parse whole, and anything else (a `v` of 0 included) is unreadable.
  (.andromeda/architecture.md:152)
  > A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored, and the state is rebuilt by replaying `events.ndjson`, which recovers only the log-derived fields (`links`, `agent_session_id`, `wheel`, `budget_paused`, `budget_override_until`); the replay writes no file.
- (.andromeda/architecture-detail/standard-contracts.md:35) A non-`v:1` or wrong-shaped `ledger/stamps.json` must be a closed malformed error to its reader and record `cli_verified:false` with one `parse-rejected{parser:"ledger-stamps"}` line, and a merge must keep unknown versions and fields.
  > A merge replaces only its own version's entry and keeps unknown versions and unknown top-level and `data` fields; bytes of the wrong shape are replaced whole. A version is verified only when every row of `LedgerRow::relied()` reads `pass`
  (.andromeda/architecture-detail/standard-contracts.md:35)
  > A non-`v:1` or wrong-shaped envelope is `AgentError::StampsMalformed` to a reader (`ledger::verified` returns `Result<bool, AgentError>`), and `run`'s gate then records `cli_verified:false` with one `parse-rejected{parser:"ledger-stamps"}` line.
  (.andromeda/architecture.md:155)
  > a malformed envelope makes `run`'s gate record `cli_verified:false`.
- (.andromeda/architecture-detail/standard-contracts.md:41) In the reached `crates/viola-state/src/stamps.rs` the stamps write must read the file only through its capped read, its errors fixed messages.
  > - `viola_state::stamps::update_stamps` runs `strict::check_stamps` on the home before it creates `ledger/` and again under its lock before its capped read; a refusal reads and writes nothing and returns `StampsError::StrictModes`. Its error is `StampsError` (`StrictModes` · `State`), each variant with a fixed message.
- (.andromeda/architecture-detail/standard-contracts.md:51) A `link` or `unlink` whose `driver` / `driven` fails validation must be answered `-32602` and write no line.
  > - A `link` or `unlink` answered `-32602` (its causes: §Standard Contracts, Channel methods) writes no line, and a failed record or remove is `-32603`.
  (.andromeda/architecture.md:194)
  > a missing, non-string or invalid name, a `driven` that is not the wrapper's own name and a `driver` equal to `driven` are each `-32602` with `data: null` ([Links]).
- (.andromeda/architecture-detail/standard-contracts.md:75) Channel `params` must be type-checked into closed shapes on the server: `wait`'s `after` / `timeout_ms` only absent or a `u64`, `from` only absent, `null` or a valid `ViolaName`, `release`'s `budget` only a bool, a `params.v` above the supported one `-32602` with `data.supported` and `data.wrapper`.
  > `after` and `timeout_ms` are each absent or a non-negative integer that fits `u64` (`null`, a negative, a fraction or a string is `-32602`, `data: null`); unknown fields are ignored.
  (.andromeda/architecture.md:179)
  > The frame envelope does not type `from`: each method that reads it (`send`, `wait`, `last`, `answer`, `pause`) takes it absent, `null` or a valid `ViolaName` and answers anything else `-32602` `invalid params` with `data: null`, never `-32600`; `release` refuses any string `from` (below).
  (.andromeda/architecture.md:192)
  > A string `from` is refused `-32602` `"invalid params"` with `data: {"reason":"release-from-driver"}` (`viola_channel::ProtocolError::ReleaseFromDriver`) plus one `release-from-driver` obs line; a `from` of another type, or a `budget` that is not a bool (`null` included), is `-32602` with `data: null`.
  (.andromeda/architecture.md:89)
  > A wrapper receiving `params.v` higher than it supports answers `-32602` with `data.supported` and `data.wrapper` (its `CARGO_PKG_VERSION`).
- (.andromeda/architecture-detail/standard-contracts.md:81) `link` and `unlink` must take no input from `from`, so a `from` of any shape is no decision input to them.
  > - `link` and `unlink` read no `from`: a `from` of any shape changes nothing. Each answers only after its event line and the snapshot write.
- (.andromeda/architecture-detail/standard-contracts.md:87) The wrapper must re-validate every `hook.event` into the closed hook kinds and `origin` values and append no invalid event, and the frame bound must hold on every channel line: at most `MAX_FRAME` bytes, a longer line `-32600` and closed, an id-less frame other than `hook.event` not dispatched.
  > The wrapper re-validates the event (`kind` is one of the five hook kinds `session-start` · `prompt-submitted` · `turn-ended` · `session-end` · `activity`; `data` is an object; for `prompt-submitted`, `text` is a string and `origin` is `harness` · `human`) and appends `{v, ts:<append time>, instance, kind, source:"hook", data}` to `events.ndjson`. An invalid event is not appended, and an append failure is `-32603`.
  (.andromeda/architecture.md:177)
  > A frame is one line of at most `MAX_FRAME` bytes, its `\n` counted; a longer line is answered `-32600` and the connection is closed. Only `hook.event` may be id-less: any other frame without an `id` is not dispatched and is logged `parse-rejected{parser:"channel-frame", detail:"malformed"}`. A client may add `conn` (obs-plan D-10) to `params`; the server logs it and strips it before dispatch, so no method takes it as input and it is never identity.
  (.andromeda/architecture.md:96)
  > with codes `-32700` parse error, `-32600` invalid request, `-32601` unknown method, `-32602` bad params / unsupported `v`, `-32603` internal error.
- (.andromeda/architecture-detail/standard-contracts.md:93) A failed, oversize or malformed `claude agents --json` read must leave the board as read, never an error.
  > A failed, oversize or malformed read leaves the board as `board::read` gave it (every `status` `unknown`, no unwrapped item).
  (.andromeda/architecture.md:200)
  > and a failed agents read leaves every `status` `unknown`.
- (.andromeda/architecture-detail/standard-contracts.md:99) The typed text the paste rule's re-verified step produces must be the sent text with every CR LF and lone CR as one LF and its trailing CR and LF removed, the text delivery matching compares, so no `send` types a text ending in a newline.
  > delivery matching compares it directly with the in-flight send's typed text, the sent text with every CR LF pair and every other CR as one LF and without its trailing CR and LF characters ([Delivery Confirmation]).
  (.andromeda/architecture-detail/standard-contracts.md:99)
  > No `send` relies on this: `send` never types a text that ends in a newline.
- (.andromeda/architecture-detail/standard-contracts.md:117) `hook statusline` must read stdin through `take(MAX_FRAME + 1)`, take the user's command only in the closed `user_command` shape, bound the command at 5 s and read its stdout through `take(MAX_FRAME)`, print nothing on oversize or malformed stdin and always exit 0 with empty stderr, its capture arm reading under its own bounds.
  > With `--capture <DIR>` it takes `viola verify`'s probe capture instead, before the pass-through ([Hook Contract]): no instance check, no `budget.json` write, no command run, nothing printed;
  (.andromeda/architecture-detail/standard-contracts.md:118)
  > The reader is `viola_agent_claude::statusline::user_command`: a command only from a `statusLine` of type `command`, non-empty, with no NUL.
  (.andromeda/architecture-detail/standard-contracts.md:119)
  > reads stdin through `take(MAX_FRAME + 1)`, writes `budget.json` when the payload holds a `rate_limits` object,
  (.andromeda/architecture-detail/standard-contracts.md:120)
  > bounded by `STATUSLINE_DEADLINE` (5 s, PROVISIONAL, `src/cmd/hook.rs`).
  (.andromeda/architecture-detail/standard-contracts.md:121)
  > It prints that command's stdout unchanged, read through `take(MAX_FRAME)`. It prints nothing when the command fails or passes the bound, when no command is recorded, and on oversize or malformed stdin. It always exits 0 with empty stderr.
  (.andromeda/architecture.md:241)
  > printing its stdout unchanged; it always exits 0 with empty stderr.
  (.andromeda/architecture.md:102)
  > **`viola hook` exit codes:** always 0, including on clap parse errors, channel failures and panics caught at the edge. Exit 2 is forbidden.
- (.andromeda/architecture-detail/standard-contracts.md:133) The `claude agents --json` read must be a direct spawn under the R8 strip, its stdout through `take(MAX_FRAME + 1)`, killed at 5 s, and parsed into rows or the closed `AgentsFailure`, any failure leaving every status `unknown` and never an error.
  > through the root bin's `run::agents` (`read_logged`: the R8 strip, stdout through `take(MAX_FRAME + 1)` by `version_gate::run_capped`, killed at 5 s, one `agents-probe` spawn pair under an open log) and parsed by the pure `viola_agent_claude::agents::parse` into rows or a closed `AgentsFailure` (`oversize` · `malformed`);
  (.andromeda/architecture-detail/standard-contracts.md:133)
  > If it is not found, fails, is over the cap or cannot be parsed, every row's `status` is `unknown`, no unwrapped rows are listed, and the outcome is never an error.
  (.andromeda/architecture.md:246)
  > If it is not found, fails, is over the cap or cannot be parsed, every `status` is `unknown`, no unwrapped row is listed, and the outcome is never an error.

## Bindings
- **conventions (paste refusal order) ↔ standard-contracts (typed text)**: `empty-text` is judged on the typed text (.andromeda/architecture-detail/conventions.md:15), the same CR LF / CR / trailing-newline rewrite delivery matching compares (.andromeda/architecture-detail/standard-contracts.md:99); one `typed_text` serves both, and `control-character` stays ahead of it on the client and the wrapper.
- **conventions (exit 2 `send` text cap) ↔ standard-contracts (channel frame bound)**: the CLI's text cap (.andromeda/architecture-detail/conventions.md:27) and the channel line bound (.andromeda/architecture.md:177) are the one `MAX_FRAME`; a reader bounded by another value breaks both.
- **conventions (MCP `invalid-params`) ↔ standard-contracts (channel `params` checks)**: the MCP server's closed-field re-validation (.andromeda/architecture-detail/conventions.md:64) is the client side, the wrapper's `-32602` checks (.andromeda/architecture-detail/standard-contracts.md:75, .andromeda/architecture.md:179) the authoritative side; each name passes `ViolaName::try_new` (.andromeda/architecture.md:128) on both.
- **conventions (exit 20 for `link` / `unlink`) ↔ standard-contracts (`-32602` writes no line)**: a refused `driver` / `driven` is exit 20 on the CLI (.andromeda/architecture-detail/conventions.md:33) and no line in the wrapper's log (.andromeda/architecture-detail/standard-contracts.md:51).
- **conventions (client refusal before connect) ↔ standard-contracts (frame verbs' refusal document)**: the client's exit 13 refusals and the instance check come before any connect (.andromeda/architecture-detail/conventions.md:52), after `send`'s capped text read (.andromeda/architecture-detail/conventions.md:58), with no line in any log (.andromeda/architecture-detail/standard-contracts.md:111).
- **standard-contracts (agents read bound) ↔ standard-contracts (`list` payload)**: the closed `AgentsFailure` (.andromeda/architecture-detail/standard-contracts.md:133) maps to the closed `status` `unknown` and no unwrapped row (.andromeda/architecture-detail/standard-contracts.md:10, .andromeda/architecture-detail/standard-contracts.md:93).
- **standard-contracts (`hook statusline` fail-open) ↔ conventions (hook exit codes)**: oversize or malformed stdin prints nothing and exits 0 (.andromeda/architecture-detail/standard-contracts.md:121), the hook's always-0, never-2 rule (.andromeda/architecture.md:102).
- **standard-contracts (snapshot and stamps readers) ↔ conventions (unparseable values)**: an unsupported or malformed own-format file is ignored or recorded unverified (.andromeda/architecture-detail/standard-contracts.md:24, .andromeda/architecture-detail/standard-contracts.md:35), as an unparseable external value is `"unknown"`, never an error (.andromeda/architecture.md:132).
