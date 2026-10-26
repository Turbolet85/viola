# arch · Conventions — category read

## Carried
- (.andromeda/architecture-detail/conventions.md:34) The two verbs must exit 21 when the driven instance has no live endpoint, with the same `instance-unreachable` meaning as every other frame verb.
  > `21` instance unreachable (no live endpoint for the `ViolaName`);
- (.andromeda/architecture-detail/conventions.md:40) The server identity check's exit-21 clause names six verbs today; `link` / `unlink` must take the same no-frame, same-document, same-hint behaviour, and the verb list here is amended to name them when they are served.
  > Exit `21` also covers, on `send`, `wait`, `last`, `answer`, `pause` and `release`, a server that is not the recorded wrapper (the connected peer's pid or OS start time is not the snapshot's, or the check cannot be made): no frame is written, and the verb prints the same `{"v":1,"error":"instance-unreachable","detail":null}` under `--json` with stderr empty, or the `unable  <name>  instance-unreachable` line with the server-verify hint last on stderr (its wording: design-system, surface cli-send). The client's own refusals (exit 13) come first, and the cause has no exit code, `--json` code or `RefusalReason` of its own.
- (.andromeda/architecture-detail/conventions.md:33) The wrapper's `-32602` answers (a `driven` that is not the serving wrapper's name, a `driver` equal to `driven`) reach the verb as a protocol fault, so they must exit 20.
  > `20` protocol fault from the wrapper
- (.andromeda/architecture-detail/conventions.md:27) A usage error on the new verbs (a missing or extra positional) is clap's exit 2.
  > `2` usage error (clap, a second positional to `viola send` included; never produced by `viola hook`)
- (.andromeda/architecture-detail/conventions.md:26) `link` / `unlink` are `cli`-role verbs: a dispatch `Err` or caught panic prints only `error: internal error` from the `main` catch site, the enumerated verb list moves with them, and the `wait` / `last` split of exit 1 against exit 21 is the pattern their reply and connection faults must follow.
  > so `send`, `wait`, `last`, `list`, `answer`, `pause`, `release`, `verify`, `revive --list` and `plugin install`), and a dispatch `Err` or a caught panic in any of them prints exactly `error: internal error` on stderr once, from the `main` catch site, no hint — no dispatch arm prints it; for `viola wait` / `viola last` exit 1 also covers a refusal reply, a reply of no known shape, and any channel error but a failed connect or a closed / reset connection (those are exit 21);
- (.andromeda/architecture-detail/conventions.md:23) Exit 1 stays the internal-error edge for the new verbs.
  > `1` internal error (anyhow edge)
- (.andromeda/architecture-detail/conventions.md:46) If P3 reads `v1-13`'s MCP half as owed, an MCP success must carry the channel `ok` payload as `structuredContent`, shaped like CLI `--json`.
  > a success is a tool result with `isError: false` and the channel `ok` payload as `structuredContent`
- (.andromeda/architecture.md:89) Every `link` / `unlink` frame carries `v`; adding the `{since, already_linked}` result fields is additive with no bump, and a too-high `params.v` is answered `-32602` with `data.supported` and `data.wrapper`.
  > **Protocol versioning:** an integer `v` starting at `1` in every channel `params`, ndjson event line and snapshot. Adding a field or event kind is additive and needs no bump. Removing or changing a field's meaning bumps `v`. A wrapper receiving `params.v` higher than it supports answers `-32602` with `data.supported` and `data.wrapper` (its `CARGO_PKG_VERSION`).
- (.andromeda/architecture.md:90) Every `link` / `unlink` `params` carries the sender's version.
  > **Sender version:** every channel `params` carries `"sender": "<CARGO_PKG_VERSION>"`.
- (.andromeda/architecture.md:91) The method names are already reserved as `link` and `unlink` (no dot namespace), with integer monotonic ids.
  > **Channel methods:** lower-case, dot-namespaced for hook traffic. Requests: `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.dialog`. `list` is not a channel method. Notifications: `hook.event`. JSON-RPC ids are integers, monotonic per connection.
- (.andromeda/architecture.md:95) The served methods answer success as `result.ok`.
  > **Channel success:** `{"jsonrpc":"2.0","id":<n>,"result":{"ok":<payload>}}`.
- (.andromeda/architecture.md:96) The bad-params answers must be protocol faults with code `-32602`; the `-32601` answered today ends when the methods are served.
  > **Channel protocol fault:** `{"jsonrpc":"2.0","id":<n|null>,"error":{"code":<int>,"message":"<text>","data":<object|null>}}` with codes `-32700` parse error, `-32600` invalid request, `-32601` unknown method, `-32602` bad params / unsupported `v`, `-32603` internal error.
- (.andromeda/architecture.md:106) Any new error variant goes into the owning crate's one `<Crate>Error` enum, and `anyhow` stays in the root bin.
  > **Rust error types:** one error enum per crate, named `<Crate>Error` (`PtyError`, `ChannelError`, `StateError`, `AgentError`, `McpError`, `CoreError`)
- (.andromeda/architecture.md:110) The result fields `since` and `already_linked` and the params `driver` / `driven` are snake_case.
  > **JSON field names:** snake_case in every viola format
- (.andromeda/architecture.md:111) Any closed word the verbs put on the wire is kebab-case.
  > **Enum string values on the wire:** kebab-case (`#[serde(rename_all = "kebab-case")]`), covering event kinds, refusal reasons, details and wheel holders (`human`, `driver`).
- (.andromeda/architecture.md:112) The chunk uses the existing `link` / `unlink` kinds and adds no kind; any new kind is added only in `viola-core`.
  > **Normalised event kinds (`viola-core`):** `session-start`, `turn-ended`, `prompt-submitted`, `question`, `permission`, `plan`, `session-end`, `activity`, `link`, `unlink`, `wheel`, `budget-gate`, `send-issued`, `send-confirmed`, `send-refused`, `dialog-expired`. New kinds are added only in `viola-core`, never with Claude-specific names.
- (.andromeda/architecture.md:123) The chunk adds no environment variable; any variable the product reads keeps the `VIOLA_` prefix.
  > **Environment variables:** `VIOLA_` prefix, SCREAMING_SNAKE_CASE, for every variable the product reads or sets.
- (.andromeda/architecture.md:125) The verbs' shape is fixed: single lower-case words, kebab-case long flags (`--json`, global `--home`), two positionals mirroring the channel `params`, and `from` added from the CLI's own `VIOLA_NAME` when set.
  > **CLI:** flags are kebab-case long options (`--json`, `--file`, `--budget`, and the global `--home`). Subcommands are single lower-case words. Verb arguments mirror the channel `params`:
- (.andromeda/architecture.md:125) The exact argument forms of the two verbs, and the CLI's `from`.
  > `viola link <driver> <driven>`, `viola unlink <driver> <driven>`.
- (.andromeda/architecture.md:125) The CLI adds `from` for these verbs as for every other.
  > Like the MCP server, the CLI adds `from` from its own `VIOLA_NAME` when set.
- (.andromeda/architecture.md:128) `driver` and `driven`, on the CLI, in the wrapper and when a logged name is shown again, must satisfy the `ViolaName` shape through nutype.
  > The instance key is `ViolaName`: ASCII `[a-z0-9-]`, 1–32 characters, starting with a letter, enforced by nutype.
- (.andromeda/architecture.md:130) `since` (the `link` line's `ts`, shown as `since …Z` in the human line) must be RFC 3339 UTC with milliseconds and a `Z`.
  > **Timestamps:** field `ts` on every event and frame, and `*_at` for other instants (`started_at`, `resets_at`, `written_at`). Always RFC 3339 UTC with milliseconds and a `Z` suffix, for example `2026-09-23T19:43:58.123Z` (chrono `to_rfc3339_opts(SecondsFormat::Millis, true)`).
- (.andromeda/architecture.md:131) Any optional result or params field is omitted when absent and read the same as `null`.
  > **Optional fields:** Rust `Option<T>` with `#[serde(default, skip_serializing_if = "Option::is_none")]`. viola omits absent fields on write, and readers treat missing and `null` the same.
- (.andromeda/architecture.md:134) The first product `unlink` append, like each `link` append, is one complete object plus `\n` in one `write`.
  > **ndjson line discipline:** one complete JSON object plus `\n` per single `write` call.

## Bindings
- **Exit 20 / 21 ↔ Standard Contracts (CLI `--json`)**: the `instance-unreachable` (exit 21) and `wrapper-fault` (exit 20) documents the verbs print under `--json` are the body's §Standard Contracts shapes (.andromeda/architecture.md:225, .andromeda/architecture.md:226); this category fixes the exit numbers (.andromeda/architecture-detail/conventions.md:34, .andromeda/architecture-detail/conventions.md:33).
- **Server-check exit 21 ↔ Instance snapshot + security plan**: the verb list at (.andromeda/architecture-detail/conventions.md:40) rests on the snapshot's `pid` / `started_at` server check (.andromeda/architecture.md:157); its hint wording is design-system's surface cli-send; the strict-modes half of crossing (c) stays the security plan's and "Home and code-bearing file integrity"'s.
- **Channel methods ↔ Standard Contracts `link` / `unlink`**: the reserved names (.andromeda/architecture.md:91) bind the method contract that still reads "the wrapper answers both `-32601`" (.andromeda/architecture.md:193), amended at wrap to the served `{since, already_linked}` / `{}` results only once the founder's P4 word names the form.
- **Event kinds ↔ Event data + wrapper appends**: the existing `link` / `unlink` kinds (.andromeda/architecture.md:112) carry `{driver, driven}` (.andromeda/architecture.md:212); the "No product path appends `unlink` yet" sentence (.andromeda/architecture.md:220) changes when the `unlink` method lands.
- **CLI verb forms ↔ Established Decisions [Links] / [MCP]**: the forms at (.andromeda/architecture.md:125) are the explicit verbs [Links] holds (.andromeda/architecture.md:51), CLI-only per [MCP] (.andromeda/architecture.md:43), which bears on `v1-13`'s MCP half (.andromeda/architecture-detail/conventions.md:46).
- **Timestamps ↔ Instance snapshot `links`**: `since` is the `link` line's `ts` (.andromeda/architecture.md:164) in the format of (.andromeda/architecture.md:130).
- **`ViolaName` ↔ the board CARRY**: a logged link name shown on a result line is parsed again through `ViolaName::try_new` (scope's second CARRY) against the shape of (.andromeda/architecture.md:128).
