group: architecture-2
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»

- leaf: .claude/docs/services/viola.md:25   class: retired
  leaf-text: «`remove` has no product caller and carries a dead-code allow until the `unlink` method is served; no channel method and no verb for `link` / `unlink` exists.»
  source: .andromeda/architecture-detail/established-decisions.md:150 «The explicit `viola link` / `viola unlink` and their two channel methods are built, with no dated gap: the wrapper serves `link` and `unlink`, and the `unlink` method is `LinkSet::remove`'s product caller»
  replace-with: «The served `link` / `unlink` channel methods (`LinkSet::link`, adding the pair through `record`, and `LinkSet::unlink`, removing it through `remove`, its product caller; sharing `driver_of`), sent by `viola link` / `viola unlink` (`src/cmd/link.rs`), read no `from` and answer only after the event line and the snapshot write: a held pair's `link` appends nothing and answers its `since` with `already_linked: true`, a new pair its new `link` line's `ts` with `already_linked: false`; `unlink` answers `{}` whether or not the pair was held, an absent pair appending nothing, a removed one one `unlink` line (`source: wrapper`); a missing, non-string or invalid name, a `driven` that is not the wrapper's own name and a `driver` equal to `driven` are each `-32602` with `data: null` and no line, and a failed record or remove is `-32603`.»

- leaf: .claude/docs/services/viola.md:34   class: stale
  leaf-text: «`send`, `wait`, `last`, `list`, `answer`, `pause`, `release`, `verify`, `revive --list`, `plugin install`»
  source: .andromeda/architecture-detail/conventions.md:26 «so `send`, `wait`, `last`, `list`, `answer`, `pause`, `release`, `link`, `unlink`, `verify`, `revive --list` and `plugin install`)»
  replace-with: «`send`, `wait`, `last`, `list`, `answer`, `pause`, `release`, `link`, `unlink`, `verify`, `revive --list`, `plugin install`»

- leaf: .claude/docs/services/viola.md:45   class: stale
  leaf-text: «every CLI frame verb (send, wait, last, answer, pause, release) opens»
  source: .andromeda/architecture-detail/occupied-resources.md:14 «the CLI frame verbs `send` / `wait` / `last` / `answer` / `pause` / `release` / `link` / `unlink`, and `mcp`'s four frame tools), checks that the process serving it is the snapshot's wrapper»
  replace-with: «every CLI frame verb (send, wait, last, answer, pause, release, link, unlink) opens»

- leaf: .claude/docs/services/viola.md:42   class: omission
  leaf-text: «`write_paused` / `write_released` in `src/human/verbs.rs`)»
  source: .andromeda/architecture-detail/occupied-resources.md:35 «`link` and `unlink` each take `<driver> <driven> [--json]` (one shared `LinkArgs`), the verb's instance being the driven one; each exits 0 with its one result line, or `{"v":1,"ok":<ok>}`»
  replace-with: «`write_paused` / `write_released` in `src/human/verbs.rs`); **Link verbs:** `src/cmd/link.rs` (`viola link|unlink <driver> <driven> [--json]`, one shared `LinkArgs`, the verb's instance the driven one: `viola_state::strict::check_instance` on the home and `instances/<driven>` before `client::start`, so before any role file, log or snapshot read (a refusal is exit 21, the instance-state `unable:` / `hint:` pair or under `--json` `{"v":1,"error":"instance-unreachable","detail":null}`, no line written, the mode left as found), then `client::connect` (the liveness-only `live_endpoint`, then the server identity check; exit 21 when either fails, no frame written) and one request, inside the `cli.link` / `cli.unlink` dispatch span; exit 0 with its one result line or `{"v":1,"ok":<ok>}` (`link`: `{since, already_linked}`; `unlink`: `{}`), 20 for a wrapper error (a `-32602` included, such as `driver` equal to `driven`, refused by the wrapper only), 1 for a refusal reply or an `ok` of another shape)»

- leaf: .claude/docs/services/viola-state.md:6   class: omission
  leaf-text: «shared by CLI `send` / `wait` / `last` / `answer` / `pause` / `release` and `viola-mcp`'s frame tools»
  source: .andromeda/architecture-detail/established-decisions.md:151 «Then come the liveness-only pre-check (`live_endpoint`), the server identity check through `cmd::client::connect`, and one request.»
  replace-with: «shared by CLI `send` / `wait` / `last` / `answer` / `pause` / `release` / `link` / `unlink` and `viola-mcp`'s frame tools»
