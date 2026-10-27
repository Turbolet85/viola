# security · cascade read · group 2 of 3

## Carried
- (.andromeda/security-plan-detail/input-validation.md:9) The paste rule must hold, as re-verified by the chunk, over decoded `char`s and as a whole-text refusal at every client and in the wrapper.
  > The check runs over decoded `char`s of the UTF-8 `&str`, never over raw bytes, because UTF-8 continuation bytes
  > 0x80–0xBF are not C1 controls. This stops a bracketed-paste breakout through an embedded `ESC[201~`. **Reject,
  > don't strip:** a text that holds a refused character is refused whole; no refused character is ever removed and
  > the rest typed.
  (.andromeda/security-plan-detail/input-validation.md:10)
  > **An empty typed text is refused:** a text whose typed text is empty, the empty text included, is refused
  > `not-delivered` / `empty-text` (`NotDelivered::EmptyText`, exit 13) on both sides
  (.andromeda/security-plan-detail/input-validation.md:11)
  > called client-side before sending (CLI `viola send` in `src/cmd/send.rs`: exit 13 before any frame and a
  > `send-refused{side:"client"}` line; the MCP `send` tool in `viola-mcp`, since chunk
  > 2026-10-17-mcp-server-for-drivers: `validate_paste_text`, then the empty-typed-text check, each refused as
  > `{"refusal":"not-delivered","detail":"control-character"|"empty-text"}` with one client-side `send-refused` line,
  > before any frame) and again by the wrapper's `send` (`src/run/send.rs`) before anything is typed — first in its
  > refusal order, and authoritative.
  (.andromeda/security-plan.md:132)
  > checked client-side and again, authoritatively, by the wrapper before anything is typed.
- (.andromeda/security-plan-detail/input-validation.md:14) Answer free text must pass the same check in `viola answer` before any frame and again in the wrapper's `answer` handler.
  > The same exact set as `send.text`: LF, CR and TAB allowed; every other C0 control, DEL and C1 refused with
  > `control-character`, checked first, on both sides: in `viola answer` before any frame (exit 13) and again,
  > authoritatively, in the wrapper's `answer` handler.
  (.andromeda/security-plan.md:134)
  > the same exact set as `send.text`, refused `control-character`, checked first on both sides
- (.andromeda/security-plan-detail/input-validation.md:17) Both sides of the channel must cap a frame before `read_line`, refuse an over-limit frame with `-32600` and a closed connection, and keep the default depth, the `v` check and the closed param types.
  > **Size cap:** wrap the stream reader in `Read::take(MAX_FRAME)` before `read_line`; an over-limit frame gets
  > `-32600` and the connection is closed. **Depth:** serde_json's default recursion limit of 128 (never
  > `unbounded_depth`). **Version:** the `v` check (`-32602`). **Closed types:** `behavior` is a closed enum
  > (`allow`/`deny`, `approve`/`revise`), `dialog_id` is `u64`, and `link`/`unlink` `driver`/`driven` go through
  > `ViolaName::try_new`.
  (.andromeda/security-plan-detail/input-validation.md:17)
  > **`hook.dialog` params (wrapper):** `kind` is a closed dialog kind, `data` is an object, `hook_event` is closed,
  > and `tool`, `input` and `continuation` are closed or absent; an invalid frame is not registered.
  (.andromeda/security-plan-detail/input-validation.md:19)
  > **`hook.event` re-validation (wrapper):** `kind` is one of the five hook kinds, `data` is an object, and for
  > `prompt-submitted` `text` is a string and `origin` is `harness` · `human`; an invalid event is not appended.
  (.andromeda/security-plan.md:136)
  > a `MAX_FRAME` cap before `read_line` (over it `-32600`, the connection closed), serde_json's default depth limit,
  > never `unbounded_depth`, the `v` check, closed types for `behavior`, `dialog_id` and names
- (.andromeda/security-plan-detail/input-validation.md:22) Hook stdin must be read through the over-size form so an over-limit payload is detected and fails open, with security-deciding fields in closed enums and the Question / Stop data kept to strings and a count.
  > **Size cap:** `Read::take(MAX_FRAME + 1)` on `read_to_end`, so an over-limit payload is detected, never silently
  > truncated; over the limit, fail open (exit 0, no body, `hook-decision` detail `oversize-stdin` +
  > `parse-rejected{parser:"hook-stdin", detail:"oversize"}` in `diagnostics/`).
  (.andromeda/security-plan-detail/input-validation.md:22)
  > **Question and Stop data:** each question's `option_details` takes an option's `description` and `preview` only
  > as strings, else `null`, and never refuses the dialog (`options` keeps its labels); the Stop's `background_tasks`
  > is read as an array of `serde::de::IgnoredAny` and carried only as its length, no element read, `null` when absent,
  > `null` or not an array
  (.andromeda/security-plan-detail/input-validation.md:23)
  > **Statusline payload (`hook statusline`):** the same cap and the same one-object rule, each ending in exit 0, empty
  > stderr, nothing printed and no `budget.json`.
  (.andromeda/security-plan-detail/input-validation.md:23)
  > stdin is read through `take(MAX_FRAME + 1)` and written raw (verify parses it later)
  (.andromeda/security-plan-detail/input-validation.md:23)
  > read through `take(64)` and matched exactly against the closed `ProbeAnswer` ids
  (.andromeda/security-plan-detail/input-validation.md:24)
  > It reads no `VIOLA_*`, does no obs init and opens no channel; any failure prints and writes nothing; stderr is
  > always empty and the exit always 0.
  (.andromeda/security-plan.md:138)
  > capped at `MAX_FRAME`, an over-limit payload detected, never silently truncated; the event maps into the closed
  > `HookEvent`, the payload must be one JSON object
- (.andromeda/security-plan-detail/input-validation.md:27) Each MCP handler must check its raw arguments itself and refuse with a closed field word and no frame.
  > each handler takes the raw argument object, never rmcp's typed deserialisation (a serde failure would carry serde
  > text): `target` through `ViolaName::try_new`, `text` a string, `after` / `timeout_ms` absent, `null` or a `u64`
  > (`wait`'s `timeout_ms` clamped to `WAIT_CAP` 50 000 ms, also its default), `dialog_id` a `u64`, `response` an object
  > through `Response::parse`, then `validate_paste_text` on `send`'s text and on every free-text answer value. A
  > failure is `isError: true` with `structuredContent` `{"error":"invalid-params","detail":"<field>"}` (the closed
  > field words `target` · `text` · `after` · `timeout_ms` · `dialog_id` · `response`), text `refused:
  > invalid-params`, and no frame
  (.andromeda/security-plan.md:140)
  > each handler checks the raw arguments itself (`ViolaName`, `u64`, the closed response enums, `validate_paste_text`
  > on text and free-text answers), a failure being `invalid-params` with a closed field word and no frame.
- (.andromeda/security-plan-detail/input-validation.md:30) Every CLI name must pass `ViolaName::try_new` in the clap parser, and `answer` input must be capped into the closed enums.
  > **Names:** every `ViolaName` argument uses `#[arg(value_parser = parse_viola_name)]`, which calls
  > `ViolaName::try_new`.
  (.andromeda/security-plan-detail/input-validation.md:30)
  > **`answer` response:** the JSON from stdin or `--file` is read with `Read::take(MAX_FRAME)` into the closed
  > `response` enums.
  (.andromeda/security-plan-detail/input-validation.md:31)
  > **`send` text:** from stdin or `--file` only, read with `Read::take(MAX_FRAME + 1)`; over the cap or not UTF-8 is
  > exit 2 with one fixed line, a second positional argument is a clap usage error, and the text then passes
  > `validate_paste_text` and the empty-typed-text check (`refusal_of`: `not-delivered` / `control-character`, then
  > `not-delivered` / `empty-text`; exit 13), both before any frame.
  (.andromeda/security-plan-detail/input-validation.md:32)
  > `--id <ID>` through a clap value parser that accepts only the session-id shape (anything else is a usage error,
  > exit 2)
  (.andromeda/security-plan-detail/input-validation.md:33)
  > `instances/<driven>` joined only from that parsed name and checked by `check_instance` before any snapshot read
  (.andromeda/security-plan-detail/input-validation.md:34)
  > `--home` as given; else the `instance_of` home when `VIOLA_NAME` passes `ViolaName::try_new` and `VIOLA_DIR` is an
  > absolute `<home>/instances/<name>` (a pair that fails the check falls through, never refuses); else `<user
  > home>/.viola`;
  (.andromeda/security-plan.md:142)
  > every name through `ViolaName::try_new`
- (.andromeda/security-plan-detail/input-validation.md:37) The `config.json` read must stay capped and tolerant, each value into its closed type, with no value switching a control off.
  > a value must be a JSON whole number that fits `Percent` (0–100, `Percent::try_new`), else that key alone takes its
  > default and is reported as one `parse-rejected{parser:"config-json", detail:"malformed"}` line per bad key
  (.andromeda/security-plan-detail/input-validation.md:37)
  > a JSON whole number of seconds that fits `DialogHold` (60–3600, `DialogHold::try_new`; default 60,
  > `DIALOG_HOLD_DEFAULT`)
  (.andromeda/security-plan-detail/input-validation.md:37)
  > `claude_env_keep` (the Unix R8 pass-list) is an array of at most 32 names, each matching `^CLAUDE[A-Z0-9_]*$`; a
  > bad value is rejected WHOLE (never partly accepted) and reported as `parse-rejected`, and an identity-floor name in
  > it has no effect (not an error). `v` is checked, and the read is capped at `MAX_FRAME`.
  (.andromeda/security-plan.md:144)
  > a tolerant parse capped at `MAX_FRAME`, `v` checked; a bad value takes its key's default, or is rejected whole, and
  > is reported as `parse-rejected`; no value switches the budget gate off or makes the dialog hold zero or unbounded
- (.andromeda/security-plan-detail/input-validation.md:52) Each own-state ndjson reader must cap every line, count an over-long, non-object or unterminated line as torn, and never write.
  > takes each line through `Read::take(MAX_FRAME + 1)`, its `\n` counted, and counts what it steps over, per read
  > (`Skipped{unknown_kinds, unknown_fields, torn_lines}`, not logged): an over-long line, a line that is not one JSON
  > object and an unterminated last line are `torn_lines`;
  (.andromeda/security-plan-detail/input-validation.md:52)
  > A reader never writes.
  (.andromeda/security-plan-detail/input-validation.md:54)
  > Over a replayed snapshot, which carries no `endpoint`, `pid`, `started_at` or `statusline_command`, `live_endpoint`
  > gives no endpoint
  (.andromeda/security-plan-detail/input-validation.md:54)
  > **Integrity:** read only after the home strict-modes check, at every entry point. `viola run`'s version gate reads
  > `ledger/stamps.json` through `viola_state::stamps::read_stamps_strict`, after the check: no lock, `take(MAX_FRAME +
  > 1)`, and a strict-modes refusal, an unreadable or a malformed file is one `parse-rejected{parser:"ledger-stamps"}`
  > WARN (detail `strict-modes-failed` on a refusal) and `cli_verified:false`.
  (.andromeda/security-plan-detail/input-validation.md:55)
  > The read runs in this order: `check_path` on the home, `check_path` on `budget.json`, a shared lock on
  > `budget.json.lock`, the file through `take(MAX_FRAME + 1)` (over the cap is no reading), a tolerant parse (an
  > unknown key is skipped), and no reading unless `v` is 1. Every failure is no reading and writes no line.
  (.andromeda/security-plan.md:154)
  > each ndjson line capped at `MAX_FRAME`, an over-long, non-object or unterminated line counted as torn; a reader
  > never writes; read only after the home strict-modes check
- (.andromeda/security-plan-detail/input-validation.md:58) The notify path filter must join only a validated name under the canonical home and ignore symlinks.
  > Tail only `instances/<name>/events.ndjson` where `<name>` passes `ViolaName::try_new`, under the canonical home.
  > Ignore symlinks (`symlink_metadata`) and names that fail parsing.
  (.andromeda/security-plan.md:156)
  > tail only `instances/<name>/events.ndjson` where `<name>` passes `ViolaName::try_new`, under the canonical home;
  > ignore symlinks and names that fail parsing.
- (.andromeda/security-plan-detail/input-validation.md:61) The `--version` and `claude agents --json` reads must stay capped, the agents read over-size-detected and parsed into rows or the closed failure, and an unwrapped row's name never a target.
  > **Parsing:** the `--version` probe is read with `Read::take(MAX_FRAME)` and parsed tolerantly. The `claude agents
  > --json` child of CLI `list`, of `viola revive`'s preflight and of `viola verify`'s Run B (chunk
  > 2026-10-16-unwrapped-sessions-on-the-board) is a direct spawn of the resolved program under the R8 strip, its stdout
  > read through `take(MAX_FRAME + 1)` and killed at 5 s, and parsed tolerantly by `viola_agent_claude::agents::parse`
  > (an element with no string `sessionId` skipped and counted, an extra key tolerated and counted) into rows or the
  > closed `AgentsFailure` (`oversize` · `malformed`, one `parse-rejected{parser:"claude-agents-json"}` line)
  (.andromeda/security-plan-detail/input-validation.md:61)
  > **Identifiers:** an unwrapped row's `name` (its `sessionId`) is never a `ViolaName` and never a valid `target`;
  (.andromeda/security-plan.md:158)
  > read under `MAX_FRAME` and parsed tolerantly, a failed `claude agents --json` read failing open
- (.andromeda/security-plan-detail/input-validation.md:64) The re-read vt100 feed must keep the human's bytes first, degrade a caught panic or dropped copy to `input-not-ready`, and never block passthrough on a full queue.
  > Feed through `std::panic::catch_unwind`: a tee on `run`'s pump output writes the human's bytes first and hands a
  > copy to a feed thread, which runs every feed and resize under the catch. A caught panic poisons the screen model
  > until the host size changes, with one `parse-rejected{parser:"vt100-feed", detail:"panicked"}` line per poisoning
  > and no screen text; a poisoned model reads `input-not-ready`, which confirmed `send` reports as
  > `not-delivered`/`input-not-ready`, and byte passthrough to the human terminal continues.
  (.andromeda/security-plan-detail/input-validation.md:64)
  > a full queue drops the copy and never blocks the passthrough
  (.andromeda/security-plan.md:160)
  > fed under `catch_unwind`, the human's bytes written first; a caught panic or a dropped copy poisons the screen
  > model, which then reads `input-not-ready`, while byte passthrough to the human terminal continues
- (.andromeda/security-plan-detail/input-validation.md:67) The agents read's output must be treated as untrusted: bounded by `MAX_FRAME`, its row names never a `ViolaName` or `target`.
  > Its output is untrusted: it is bounded by `MAX_FRAME`, and a row's `name` is never a `ViolaName` or `target` in CLI
  > `list` or the MCP `list` tool.
  (.andromeda/security-plan.md:162)
  > its output untrusted and bounded
- (.andromeda/security-plan-detail/input-validation.md:71) A `revive --id` and the logged session id must pass the closed session-id shape and reach the child only as one argv element built by `resume_args`.
  > An `--id` the log does not hold, or a log with no session, is `no-session`: exit 1 before any spawn. The shape is
  > closed (`viola_agent_claude::is_session_id`: 36 characters, hyphens at offsets 8, 13, 18 and 23, ASCII hex
  > elsewhere), and a malformed `--id` is a clap usage error (exit 2). The id is one argv element of a direct spawn,
  > never a shell, built only by `viola_agent_claude::resume_args`
  (.andromeda/security-plan.md:164)
  > the id after `--resume` has the closed session-id shape and is one argv element of a direct spawn, never a shell,
  > built only by `resume_args`
- (.andromeda/security-plan-detail/input-validation.md:74) The statusline source read at `run` start must stay capped, with the command taken only from the closed shape.
  > At every start `run` reads one source through `Read::take(MAX_FRAME)`: `<home>/statusline-source.json` whenever it
  > exists (the Claude settings shape; viola only reads it); else, only when the home is the default home
  (.andromeda/security-plan-detail/input-validation.md:74)
  > Parsing is tolerant and in the agent crate alone (`statusline::user_command`): a command only from a `statusLine`
  > of type `command`, non-empty, with no NUL.
  (.andromeda/security-plan.md:166)
  > one capped, tolerant read of `<home>/statusline-source.json`, else, for the default home only, the user's
  > `~/.claude/settings.json`
- (.andromeda/security-plan-detail/input-validation.md:77) The status line command's stdout must be capped and passed through unparsed, its wait bounded.
  > Its stderr is discarded. Its stdout is read through `take(MAX_FRAME)` and printed byte for byte, never parsed; a
  > failing command prints nothing. The wait is bounded by `STATUSLINE_DEADLINE` (5 s, PROVISIONAL, `src/cmd/hook.rs`;
  > detail `deadline`). No log line and no span carries the command.
  (.andromeda/security-plan.md:168)
  > its stdout is capped and printed byte for byte, never parsed, its wait bounded
- (.andromeda/security-plan-detail/input-validation.md:90) `MAX_FRAME` must remain the one 16 MiB const every external reader the enumeration names is capped by.
  > **Constants:** `MAX_FRAME = 16 MiB`, one const in `viola-core`, shared by channel frames, hook stdin (the
  > `--capture` arm's and `hook statusline`'s included), the statusline source read at `run` start (`take(MAX_FRAME)`),
  > the stdout of the user's statusline command (`take(MAX_FRAME)`), `answer` input, `viola send`'s stdin / `--file`
  > text (`take(MAX_FRAME + 1)`), child output
  (.andromeda/security-plan-detail/input-validation.md:90)
  > ndjson line reads, the `config.json` read and the `ledger/stamps.json` read (`take(MAX_FRAME + 1)`: over the cap
  > reads as absent in `update_stamps` and is an error in `read_stamps`).
  (.andromeda/security-plan.md:174)
  > **Parser surfaces (scope for fuzz / property coverage; tests owns the cases):** `validate_paste_text`, channel
  > ndjson framing with `MAX_FRAME`, the hook stdin parser (including `hook statusline`), the `claude agents --json`
  > output parser, and the vt100 feed under `catch_unwind`.
  (.andromeda/security-plan.md:176)
  > **Constants:** `MAX_FRAME` = 16 MiB, one const in `viola-core`, caps every external read (channel frames, hook
  > stdin, the statusline source and command output, `answer` input, sent text, child output, ndjson lines,
  > `config.json` and the stamps read)
- (.andromeda/security-plan-detail/data-protection.md:16) The pinned key that joins `bin/<key>/` and `plugin/<key>/` must have this closed shape before the join.
  > `<hash>` is a truncated SHA-256 of the exe bytes (the first 16 hex characters of the digest).
  (.andromeda/security-plan-detail/data-protection.md:17)
  > `viola run`'s start pins the bytes it has already checked (`viola_state::pin::pin_bytes`, under the key
  > `pin::pin_key`: `<VERSION>-<16 hex>`)
- (.andromeda/security-plan-detail/dependency-security.md:14) A bound added in the edited `tokio_client.rs` must stay behind `viola-channel`'s `tokio` feature, its normal graph tokio-free.
  > `viola-channel`, whose Tokio client (tokio `=1.53.3`, `io-util`) sits behind its `tokio` feature, left that list
  > at chunk 2026-10-17-mcp-server-for-drivers and is covered instead by the `cargo tree` form
  > `scripts/channel-tokio-free.sh` (its normal graph without the feature must print no `tokio v` line, its `--features
  > tokio` control one;
  (.andromeda/security-plan.md:256)
  > `viola-channel` covered instead by the `cargo tree` form `scripts/channel-tokio-free.sh`
- (.andromeda/security-plan-detail/bootstrap-phases.md:15) Every bounded read the chunk adds or moves must come after its entry point's strict-modes check.
  > every client reads the snapshot only after the instance's strict-modes check
  > (`viola_state::strict::check_instance`; the six CLI frame verbs and `link` / `unlink` through
  > `client::refuse_untrusted`, the hook event and dialog paths through `trusted`, `viola mcp`'s frame tools directly; a
  > refusal writes no line), then makes the liveness pre-check and the server check.
  (.andromeda/security-plan.md:328)
  > Wire `ViolaName::try_new` into the clap `value_parser`, the MCP handlers, the channel `link`/`unlink` params and
  > the notify path filter.
  (.andromeda/security-plan.md:330)
  > Add the `MAX_FRAME` const and the `Read::take` wrappers on every external reader.
- (.andromeda/security-plan-detail/secret-management.md:9) The agents read's direct spawn must stay under the R8 strip, and no bounded read may carry a stripped `CLAUDE*` value into any record.
  > They are removed from the child by the R8 rule and are never serialised into events, snapshots, diagnostics or the
  > ledger.
  (.andromeda/security-plan-detail/secret-management.md:9)
  > The strip, the registry reader and the logs handle names only and never read a value;
  (.andromeda/security-plan.md:369)
  > the strip handles names only and never reads a value.

## Bindings
- **input-validation ↔ bootstrap-phases**: every cap the chunk adds over a file under the home (Own state files, `config.json`, the statusline source) runs only after that entry point's strict-modes check, (.andromeda/security-plan-detail/input-validation.md:54) ↔ (.andromeda/security-plan-detail/bootstrap-phases.md:15)
- **input-validation ↔ secret-management**: the `claude agents --json` read is a direct spawn under the R8 strip, and `claude_env_keep`'s identity-floor name has no effect, (.andromeda/security-plan-detail/input-validation.md:61) / (.andromeda/security-plan-detail/input-validation.md:37) ↔ (.andromeda/security-plan-detail/secret-management.md:9)
- **input-validation ↔ data-protection**: the names-before-path-joins rule extends to the pinned key, whose closed shape is the truncated SHA-256 key, (.andromeda/security-plan-detail/input-validation.md:30) ↔ (.andromeda/security-plan-detail/data-protection.md:17)
- **input-validation ↔ dependency-security**: the channel frame cap on the Tokio client side stays behind the `tokio` feature, so the sync framing stays tokio-free, (.andromeda/security-plan-detail/input-validation.md:17) ↔ (.andromeda/security-plan-detail/dependency-security.md:14)
- **input-validation ↔ the body's parser surfaces**: the per-boundary witnesses the chunk adds read the named parser surfaces, (.andromeda/security-plan.md:174) ↔ (.andromeda/security-plan-detail/input-validation.md:90)
