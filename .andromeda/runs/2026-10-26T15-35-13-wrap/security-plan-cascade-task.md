# Task — security-plan · 2026-10-26-explicit-session-links

## The session
The driven instance's wrapper now serves the channel methods `link` and `unlink`. Before this session both answered `-32601`. The served set is now nine: `send`, `wait`, `last`, `hook.dialog`, `answer`, `pause`, `release`, `link`, `unlink`, plus the `hook.event` notification.
It added the CLI verbs `viola link <driver> <driven> [--json]` and `viola unlink <driver> <driven> [--json]`. Their instance is the driven one. They run in this order: `check_instance` on the home and `instances/<driven>`, BEFORE `client::start`; then the liveness pre-check; then the server identity check through `cmd::client::connect`; then one request.
The founder gave his own word at P4 (`inputs#I3`, 2026-10-26): the boundary widening is ratified in this instance-check-first form, with no dated gap taken. His P5 word (`inputs#I4`) carries the `v1-13` / `v1-36` dispositions as a CARRY onto "Version done-check".
Retired: the hold ("the explicit `viola link` / `viola unlink` are not built / held for the founder's word on the boundary widening"), the `-32601` answer for both methods, and `LinkSet::remove`'s dead-code allow. A product path now writes an `unlink` event line, the first one.
Measured: the CLI frame verbs go from six to eight. `check_instance(` product call sites go from 2 to 3. `cmd::client::connect` callers go from 6 to 8. `live_endpoint` product readers go up by 1. `tests/security_negatives_server_identity.rs` goes from 19 to 25 cases. No dependency, config key or schema changed.

## Reached files
- src/cmd/link.rs
- src/cmd/unlink.rs
- tests/channel_links.rs
- src/run/links.rs
- src/cmd/run.rs
- src/cmd/mod.rs
- src/human/verbs.rs
- src/human.rs
- tests/channel_endpoint.rs
- tests/cli_output_plain.rs
- tests/security_negatives_server_identity.rs
- tests/cli_links.rs
- .config/nextest.toml
- viola-0.1.0/verification-matrix.json
- viola-0.1.0/working-route.md

## Touches
- The strict-modes entry-point list in Authentication & Authorization (the report names `security-plan-detail/authentication-authorization.md:34` as stale). It lists `run`, `revive`, `list`, `hook statusline` and others, but not `viola link` / `viola unlink`. Both verbs now run `check_instance` before any role file, log or snapshot read.
- IPC client-side server verification (anti-impersonation). The CLI frame verbs are now eight (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`). `link` / `unlink` verify the server against a snapshot read AFTER a passed strict-modes check, so they are not a dated gap. The other six keep their dated gaps.
- The threat-model text naming "the six" CLI frame verbs (the report names `security-plan-detail/threat-model-summary.md:55` as stale). It moves to eight.
- The frame ban in Anti-Patterns § Authentication. Its dated-gap list (`hook.event`, `hook.dialog`, CLI `send`, `wait` / `last`, `answer`, `pause` / `release`, `viola mcp`'s tool frames) must not gain `link` / `unlink`. Those two meet the ban in full except for the shared euid / macOS items owed to "Unix endpoint and home hardening".
- macOS: the `link` / `unlink` server check is the same recorded gap (no peer pid). Their case is compiled out there, and the client euid check stays owed to "Unix endpoint and home hardening".
- Universal ban: no new channel method without a Security Decisions Log entry mapping it to the IPC controls. `link` / `unlink` are now served under the founder's word `inputs#I3`, and that word is the entry owed.
- Any security-plan line stating that `link` / `unlink` are unserved, held, answered `-32601`, or "not built" is retired by this session.
- Input Validation, the channel-frames row: "each method's params checked by the wrapper (`-32602`)" now covers `link` / `unlink`. The `-32602` causes are a bad name, a foreign `driven` and a self-pair, each with no line written.
- Bootstrap phases, input-validation-library-install: "Wire `ViolaName::try_new` into ... the channel `link`/`unlink` params" is now built, in the wrapper's `driver_of`.
- Anti-Patterns § Input: NEVER join `instances/<name>` with a string that did not come out of `ViolaName::try_new` (`link`/`unlink` params). Both the wrapper and the verbs (`instances/<driven>` after the clap name parse) now hold this for real.
- Anti-Patterns § Authentication: `from` is never identity. The explicit `link` / `unlink` methods do not read `from` at all. The implicit link from a `send` / `answer` still records it.
- Input Validation, the CLI-arguments row: the home strict-modes check runs before any snapshot read "except the ratified dated interim gaps". The two new verbs take no gap.
- Error Handling, the Channel error format: the `-32602` causes grow by the bad `link` / `unlink` name or pair. A `record` / `remove` failure is the fixed `-32603` `internal error`.
- CLI `--json` error output: the verbs' strict-modes refusal prints exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty. That is a code only, with no path, pid or endpoint.
- Exit codes on the CLI frame-verb refusal row: exit 21 for both the verbs' strict-modes refusal (as `list` / `plugin install`, not `revive`'s exit 1) and their server mismatch. A wrapper fault is exit 20.
- `server-verify-failed` writers: `link` / `unlink` now log `process-exit{exit_code:21, detail:"server-verify-failed", during:"connect"}` on a mismatch. Any list of the writers that names six verbs moves to eight.
- The `check_instance(` product call-site count moves from two to three. Its rule is the call sites under `src/`; `list`'s check sits in `viola-state`'s board scan and is not counted.
- `cmd::client::connect` callers move from 6 to 8. The report reads `authentication-authorization.md:25`, `:26` and `:28` as still true, since they describe other verbs' dated gaps.
- `live_endpoint` readers go up by 1. The report reads the readers by class ("the CLI frame verbs") as still true, including `authentication-authorization.md:27` and `input-validation.md:52`.
- The server-identity test inventory: `tests/security_negatives_server_identity.rs` now has 25 cases, eight verbs × two modes plus the controls. The report reads `authentication-authorization.md:73` as still true, since it names the file for other content.
- Threat Model, the IPC vector ("whoever can connect can type prompts..."). The served surface now also lets any same-OS-user process add or remove a driver link. The effect is a record only: a link line and the snapshot's links.
- Threat Model, the MCP vector: "leaving `release`, `pause`, `link` and `unlink` off the tool list is an affordance". This still holds: the MCP tool list is unchanged, and the CLI and the raw channel are now the only routes to the explicit link.
- The data-classification and log floors: the newly written product `unlink` event line in `events.ndjson` carries only the names `{driver, driven}` and `source: wrapper`. The wrapper's `channel-request` / `channel-response` pair carries `method` and no names.
- Human-mode output escaping (logging-redaction-wire): the link result line prints a reply `since` that is no RFC 3339 instant through `escape_message`. That function keeps `\n`, `\t` and non-ASCII and escapes the other controls. Such a byte can come only from a hand-altered log.
- Ordering hazard, indirect through the home-mode rule: opening a role file narrows an existing home to 0700, so a check placed after `client::start` passes a 0770 home. The verbs' check sits before it, as `revive`'s and `hook statusline`'s already do.

## New facts
- F1 · authentication-authorization · `viola link` / `viola unlink` run `viola_state::strict::check_instance(<absolute home>, <home>/instances/<driven>)` before `client::start`, so before any role file or log opens and before any snapshot value is read. Then come the liveness pre-check and the server check through `cmd::client::connect` on Linux and Windows. There is no dated gap: the founder's own word at P4, `inputs#I3`, 2026-10-26.
- F2 · authentication-authorization · the wrapper serves the channel methods `link` and `unlink` (before: `-32601`), the served set now being nine methods plus the `hook.event` notification. The founder's word `inputs#I3` ratifies this boundary widening as mapped to the IPC controls: the listener and peer checks unchanged, and the CLI verbs going through the instance check, liveness and the server check.
- F3 · input-validation · the wrapper's `link` / `unlink` read `params.driver` and `params.driven` as strings through `ViolaName::try_new`. Each of these is `-32602` with `data: null` and no line written: a missing, non-string or invalid name; a `driven` other than the serving wrapper's own name; a `driver` equal to `driven`. Neither method reads `from`.
- F4 · input-validation · a `driver` equal to `driven` is refused by the wrapper only, never client-side, and reaches the CLI as a wrapper fault, exit 20.
- F5 · error-handling · in the wrapper's `link` / `unlink`, a `record` / `remove` failure is `ProtocolError::Internal` (`-32603`, fixed `internal error`). A reply of an unknown shape at the verb is an internal error, exit 1.
- F6 · error-handling · the verbs' strict-modes refusal is exit 21. In human mode it is the fixed instance-state `unable` / `hint:` pair through `human::refuse`. Under `--json` it is exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty. No log line is written and the directory's mode is left as found.
- F7 · data-protection · a product path now appends the `unlink` event line (`source: wrapper`, `data: {driver, driven}`) to `events.ndjson`, through `LinkSet::remove`, whose first product caller is the `unlink` method. The replies carry only `{since, already_linked}` (link) or `{}` (unlink).
- F8 · data-protection · the verbs print only their own parsed `ViolaName` arguments, never a name read back from the reply or the log. No driver name reaches the wrapper's process log: the channel pair carries `method:"link"|"unlink"` and no names.
- F9 · bootstrap-phases · `viola link`'s human result line prints the reply's `since` as a UTC clock time. A `since` that is no RFC 3339 instant (the restore's `unknown`) is printed through `escape_message`, which escapes C0/C1 and keeps `\n` and `\t`.
- F10 · threat-model-summary · on the IPC vector, any same-OS-user process that passes the endpoint controls can now create or remove a driver link through the served `link` / `unlink` methods. The effect is a record line and the snapshot's links only, and the self-reported `from` plays no part.
- F11 · authentication-authorization · tests hold the new controls. `channel_link_names_are_validated_by_the_wrapper` covers raw bad-name, foreign-`driven` and self-pair frames: `-32602`, no line. `link_verbs_refuse_a_widened_instance_before_any_log` covers a group-writable driven instance on Unix: refused before any log, mode as found, the remove-the-guard control red when the check is moved. The identity table's `link` / `unlink` rows check an impostor server: no frame, exit 21 in both modes, Linux and Windows.

## Untouched
- Dependency Security: no crate was added, bumped or banned, and the cargo-deny, zizmor, ConPTY vendor, pinning, CI-workflow and release-check text is all unmoved.
- API Security and the rmcp feature set: no MCP tool was added or removed. The MCP tools stay `send · wait · last · answer · list`.
- The Windows pipe SDDL, the SQOS client open, the Unix per-user socket directory, the socket chmod, `try_overwrite` and the server-side peer euid check: no listener or transport changed.
- The dated gaps of `hook.event`, `hook.dialog`, CLI `send`, `wait` / `last`, `answer`, `pause` / `release` and `viola mcp`'s tool frames: none was closed or widened by this session.
- Paste text and dialog free-text validation (`validate_paste_text`, `control-character`, `empty-text`, the CR/LF typing rule): the new methods carry names only, no text.
- `viola hook` and its fail-open rules, the hidden capture arms, and the statusline source and shell-out: unchanged.
- `viola revive` (its preflight, session-id argv and `--json` codes) and `viola list`'s board, escaping and colour facts: unchanged.
- `viola verify`'s PTY children, the probe dirs, `--record` fixtures and their scrub, and the capability-ledger stamp gate on dialog decisions: unchanged.
- Secret Management (the R8 strip, the identity floor, the artifact canary scan, the `env -i` pre-push launcher): unchanged.
- The Windows DLL search restriction, the ConPTY companion re-hash, and the pinned-binary and `plugin/` rewrite rules: unchanged.
- The `config.json` budget thresholds, the dialog hold and the `budget.json` read: unchanged.
- `MAX_FRAME` and the `Read::take` floors: no new external reader beyond the existing channel client.
