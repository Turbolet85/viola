# Task — arch · 2026-10-27-bounded-inputs-at-every-boundary

## The chunk
Makes every input that crosses into a `viola` process bounded where it enters: every external reader size-capped through `Read::take(MAX_FRAME)` (or the `MAX_FRAME + 1` over-size form) before it buffers, a reader found without one bounded.
Holds every serde_json parse of external bytes at the default recursion limit (no `unbounded_depth` in the product graph), every decision read from outside a closed enum or validated newtype, and every name reaching a path join validated first (`ViolaName::try_new` or its own closed-shape validator).
Re-verifies the paste rule (`validate_paste_text`, then the typed text) at every client before any frame (CLI `send`, MCP `send`) and authoritatively in the wrapper before the first wheel read, refusing and never stripping.
Re-verifies that a vt100 screen-model panic degrades readiness to `input-not-ready` while the child's bytes still pass through to the human's terminal.
Adds the proof: a unit-level witness per property and per boundary, and an enumeration of the external readers, each with its cap, depth and type rule, so a new unbounded reader is visible.

## Reached files
- crates/viola-core/src/lib.rs
- crates/viola-channel/src/frame.rs
- crates/viola-channel/src/server.rs
- crates/viola-channel/src/client.rs
- crates/viola-channel/src/tokio_client.rs
- src/cmd/hook.rs
- src/cmd/hook/statusline.rs
- src/cmd/mcp.rs
- crates/viola-mcp/src/lib.rs
- crates/viola-mcp/src/tools.rs
- src/cmd/send.rs
- src/cmd/answer.rs
- src/cmd/client.rs
- src/cmd/revive.rs
- src/cmd/run.rs
- src/cmd/verify.rs
- src/run/send.rs · re-reads the wrapper's paste rule before the first wheel read
- src/run/agents.rs
- src/run/version_gate.rs
- src/run/budget.rs
- src/run/dialog.rs
- src/run/env.rs
- src/run/links.rs
- src/obs.rs
- src/run/gate.rs · re-reads the `catch_unwind` degrade to `input-not-ready`
- crates/viola-agent-claude/src/screen.rs · re-reads the `catch_unwind` site of the screen model's feed
- crates/viola-pty/src/pump.rs · re-reads the passthrough of the child's bytes beside a feed panic
- crates/viola-agent-claude/src/hook.rs
- crates/viola-agent-claude/src/agents.rs
- crates/viola-agent-claude/src/statusline.rs
- crates/viola-agent-claude/src/lib.rs
- crates/viola-state/src/events.rs
- crates/viola-state/src/snapshot.rs
- crates/viola-state/src/budget.rs
- crates/viola-state/src/stamps.rs
- crates/viola-state/src/replay.rs
- crates/viola-state/src/pin.rs
- fuzz/fuzz_targets/

## Needs
- `MAX_FRAME` (16 MiB, `viola-core`) as the one cap of every external reader, and the documented `MAX_FRAME + 1` over-size form where a reader must tell an over-cap input from one at the cap.
- The wrapper channel frame bound: one line of at most `MAX_FRAME` bytes, its `\n` counted; a longer line answered `-32600` and the connection closed; an id-less frame other than `hook.event` not dispatched and logged `parse-rejected{parser:"channel-frame", detail:"malformed"}`; `conn` stripped before dispatch, never input or identity — on the sync server and on both clients.
- `viola hook` stdin read through the cap, its tolerant payload parse (no `deny_unknown_fields`, `Option` / `#[serde(default)]`), a question's `option_details` description / preview kept only as strings else `null`, the Stop's `background_tasks` read as ignored elements and carried only as a length; any oversize or malformed stdin still exit 0, no body, no stderr (hook fail-open, exit 2 forbidden).
- The hidden `hook <event> --capture` arms' bounds: stdin through `take(MAX_FRAME + 1)`, the `--answers` id through `take(64)` into the closed `ProbeAnswer`, the statusline form's closed `ShellVerdict` (`bin-sh` · `other`).
- `viola mcp` stdin (rmcp stdio framing) and every MCP tool argument re-validated by the server, a failure `invalid-params` with no frame written; an unknown tool name `-32602`.
- The bounded `claude agents --json` read (`src/run/agents.rs`): a direct spawn under the R8 strip, stdout through `take(MAX_FRAME + 1)`, killed at 5 s; a failed, oversize or malformed read leaves every `status` `unknown`, lists no unwrapped row and is never an error; a row's status read as a closed value.
- The bounded `claude --version` spawn and the stamps verdict of `run::version_gate`; a malformed or refused `ledger/stamps.json` records `cli_verified:false`.
- Home and user files read through the cap: `config.json` (`five_hour` / `seven_day` into `Percent`, `dialog_hold_s` into `DialogHold` 60–3600; a bad value its key's default with one `parse-rejected{parser:"config-json"}` line, no value disabling the gate or making the hold zero or unbounded), `budget.json` only through `read_budget` (`take(MAX_FRAME + 1)`, tolerant parse, every failure no reading), `snapshot.json` (a higher `v` or a parse failure ignored and replayed from `events.ndjson`), `statusline-source.json` and `~/.claude/settings.json` (a `statusLine` of type `command`, non-empty, no NUL).
- The `events.ndjson` reader: three skip counts, a torn last line tolerated and counted, never rewritten; readers of viola's own formats skip and count unknown kinds and fields, carry `v`, and never use `deny_unknown_fields`.
- The user's statusline command stdout through `take(MAX_FRAME)`, stderr discarded, its 5 s PROVISIONAL bound, the command in no line or span.
- The child's PTY output into the vt100 feed: the passthrough never waits on the screen model; a feed panic logged `parse-rejected{vt100-feed, panicked}` and readiness degraded to `input-not-ready`, nothing typed.
- serde_json at its default recursion depth on every parse of external bytes (hook payloads, channel frames, MCP arguments, agents output, statusline payload, home files); `unbounded_depth` absent from the product graph.
- The two validation regimes: external payloads parsed tolerantly with the raw `serde_json::Value` kept for the fixture recorder, own formats versioned-tolerant; a channel `params.v` above the supported one answered `-32602` with `data.supported` and `data.wrapper`.
- Closed decision types on the wire: `RefusalReason` with `#[serde(other)] unknown`, each `detail` from its reason's closed set extended only in `viola-core`; event kinds normalised only in `viola-core`; `session-start` `cause` closed with `unknown`; the closed hook event argument list; an unknown channel method `-32601`.
- Channel `params` type checks: `from` absent, `null` or a valid `ViolaName`, anything else `-32602` with `data: null`; `release` refusing a string `from` as `release-from-driver` and a non-bool `budget` as `-32602`; `link` / `unlink` `driver` / `driven` missing, non-string, invalid, a `driven` not the wrapper's own or equal to `driver` each `-32602`; `answer`'s `response` normalised per dialog kind into closed `behavior` values.
- `ViolaName`: ASCII `[a-z0-9-]`, 1–32 characters, starting with a letter, by nutype, before every join into `instances/<name>` — a CLI positional, `VIOLA_NAME`, a channel or MCP `name` / `driver` / `driven`; the `VIOLA_NAME` / `VIOLA_DIR` home pair falling through on failure, never refusing.
- The closed-shape validators for the non-`ViolaName` path or argv inputs: a revive `--id` and a logged session id through `is_session_id` before `resume_args` builds one argv element; the pinned key `<version>-<hash>` with its 16-hex SHA-256 prefix.
- The paste rule: `validate_paste_text` over decoded `char`s allowing LF / CR / TAB and refusing every other C0, DEL and C1 as `not-delivered` / `control-character`, never stripping; then `typed_text` (every CR LF and lone CR one LF, trailing CR and LF removed) and an empty typed text refused `empty-text` (exit 13) right after `control-character`; client-side before any frame (CLI `send` after its text read, MCP `send`), wrapper-side before the first wheel read; `control-character` first in the refusal order.
- `send`'s text taken only from stdin or `--file` through the cap, never from a leading-slash argument; `answer`'s `response` JSON from stdin or `--file` through the cap.
- Tokio containment over the readers the chunk touches: the `run`, `hook`, `send` and channel-server paths stay std threads and blocking I/O, only `viola-mcp` builds a runtime.
- The fuzz targets `viola_name`, `channel_frame`, `hook_stdin`, `vt100_feed`, `paste_text` and `claude_agents_json` and their committed corpora, a boundary witness the proof may extend, the target and seed counts recorded by their `ls` rule.
- The `[inferred]` capability claim `verification-matrix.json#v1-46`, its `Last-Event-ID` / resume-cursor web clause carried to 0.2.0, the CLI half unweakened.

## Standing conventions
- S1 · Each new test runs in its own viola home (`--home`) under `target/e2e-home/viola-test-*/home`, never sharing an endpoint, log or `budget.json` · occupied-resources.md
- S2 · A new test-only endpoint is named outside the `viola-<h12>` namespace, unique per test, never bound by `viola` · occupied-resources.md
- S3 · A new fuzz target or seed lives in the separate `fuzz/` workspace with synthetic seeds kept byte-exact by `.gitattributes`, and the stated count carries its `ls` rule · occupied-resources.md
- S4 · A new log line is a codes-only process-log line under `diagnostics/` in the obs-owned line schema, content only in the instance detail files · §Cross-cutting Patterns (Diagnostic output channels)
- S5 · A new fixture is scrubbed and hygiene-walked against its tolerant schema, an unclean recording refused whole naming the file and the check code · occupied-resources.md
- S6 · A test-only binary, feature or seam never reaches a release build · occupied-resources.md
- S7 · Every OS-specific branch compiles and is tested on its own CI runner against the fake agent · §Cross-cutting Patterns (Cross-platform discipline)
- S8 · Product crates write no `print!` / `eprintln!` / `dbg!`; stdout stays reserved · §Cross-cutting Patterns (Diagnostic output channels)

## Not needed
- The wheel's byte-source claim, the `human-typing` rule and the closed non-editing key list: the chunk bounds inputs, not who holds the wheel.
- The budget gate's pause and lift semantics (`resets_at` passing, `release --budget`, the soft stop at the turn boundary) beyond how `config.json` and `budget.json` are read.
- The link set's replay, restore at start and implicit-link timing, beyond the validation of `driver` / `driven`.
- Session liveness timing (the 1 s heartbeat, the 5 s `live` window, the pid + start time `gone` verdict) and `claude agents --json`'s join of rows to wrapped instances.
- The pinned copy's no-clobber placement, its re-hash and the two-pin-key reproducible-build measurement owed to "Version done-check", beyond the pinned key's shape.
- Windows ConPTY sideloading, the System32 DLL-search restriction and the vendored ConPTY binaries' audit.
- Socket directories, SDDL, peer credentials, the server identity check and the home strict-modes checks: owned by "Unix endpoint and home hardening".
- The exit-cause code catalogue and the documents owed for `run`'s start refusals after a passed preflight.
- Delivery confirmation's window, the `prompt-submitted` read-back and the local-command post-conditions, beyond `typed_text`.
- The hook transport tiers, the spine deadline and the dialog hold's ceiling.
- `viola verify`'s probe-run design, its ledger rows and stamp writing, beyond the capture arms' input bounds.
- `viola plugin install`'s marketplace tree and the plugin files' exec-form `command` paths.
- The deferred frontend, GUI contracts, `Last-Event-ID` web clause, hosting, mobile and AI/ML rows.
