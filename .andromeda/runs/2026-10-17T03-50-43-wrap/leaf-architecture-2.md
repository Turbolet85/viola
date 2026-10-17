group: architecture-2
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
read: .claude/docs/services/viola-ui.md · 9 · «None: the `viola_ui` log target in `src/obs.rs`, the reserve»
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»
read: .claude/docs/stack.md · 66 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 67 · «This project uses a **reference-based** CLAUDE.md architectu»

- leaf: .claude/docs/services/viola.md:21   class: stale
  leaf-text: «`--list` prints `<ts>  <cause>  <id>` per logged session and opens no process log. It takes no `--json` yet.»
  source: .andromeda/architecture.md:98 «`viola revive` takes `--json` on both arms (chunk 2026-10-17-cli-machine-contract): a refused strict-modes check or a preflight refusal prints `{"v":1,"error":"<code>","detail":null}`»
  replace-with: «`--list` prints `<ts>  <cause>  <id>` per logged session; under a parsed `VIOLA_NAME`, once its strict-modes check passed, it writes `process-start` and `process-exit{subject:"self"}` (`exit_code` 0, or 1 `detail:"no-session"` on an empty chain) into the caller's `cli-<VIOLA_NAME>.ndjson`, and with no name opens none. Both arms take `--json` (chunk 2026-10-17-cli-machine-contract): a refused strict-modes check or a preflight refusal prints `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`; `client::refused_document`) on stdout in place of its pair, stderr empty, the same exit 1 and the same log lines; `--list --json` prints `{"v":1,"ok":{"sessions":[…]}}` in log order, or the `no-session` document with exit 1; a refusal `run`'s start makes after a passed preflight prints no document (owed to the working-route entry "Exit-cause code catalogue").»

- leaf: .claude/docs/services/viola.md:34   class: stale
  leaf-text: «`role_of` files every first word but `run`, `revive`, `hook` and a leading `-` flag under `cli`: `send`, `wait`, `last`, `list`, `answer`, `pause`, `release`, `verify`; `revive` is a wrapper beside `run`, and its start arm logs as process `run`)»
  source: .andromeda/architecture.md:129 «`src/main.rs` `role_of` files every first word but `run`, `revive`, `hook` and a leading `-` flag under `cli`, and `revive` followed (after any `--home` pair) by `--list` before a `--` under `cli` too»
  replace-with: «`role_of` files every first word but `run`, `revive`, `hook` and a leading `-` flag under `cli`, and `revive` followed (after any `--home` pair) by `--list` before a `--` under `cli` too: `send`, `wait`, `last`, `list`, `answer`, `pause`, `release`, `verify`, `revive --list`; every other `revive` is a wrapper beside `run`, and its start arm logs as process `run`)»

- leaf: .claude/docs/services/viola.md:20   class: omission
  leaf-text: «`run` start order: program resolution (`.cmd`/`.bat` refusal) → collision check»
  source: .andromeda/architecture.md:400 «`run`'s start canonicalises the home after its wrapper log has created it, before the collision check, the pin and the endpoint bind.»
  replace-with: «`run` start order (the home is canonicalised by `canonical_home` once the wrapper log has created it, before the collision check, the pin and the endpoint bind; not ratified, the founder's word owed): program resolution (`.cmd`/`.bat` refusal) → collision check»

- leaf: .claude/docs/services/viola.md:23   class: stale
  leaf-text: «else for the default home only the user's `.claude/settings.json` (`cmd::default_home`)»
  source: .andromeda/architecture.md:400 «only the default home (`canonical_home(default_home(user_home))`, `default_home` shared by `resolve_home` and the source rule) falls back to the user's `<user home>/.claude/settings.json`»
  replace-with: «else for the default home only (`canonical_home(default_home(user_home))`) the user's `.claude/settings.json`, so a home named by `--home` or reached through `VIOLA_DIR` never reads it»

- leaf: .claude/docs/services/viola-state.md:6   class: stale
  leaf-text: «as landed the CLI verbs take `--home`, else the default, and only `hook` takes the `VIOLA_DIR` step»
  source: .andromeda/architecture.md:400 «`resolve_home(flag, name, dir, user_home)` in `src/cmd/mod.rs` resolves `--home` as given, else the `instance_of` home when both `VIOLA_NAME` and `VIOLA_DIR` pass the one shape check»
  replace-with: «as landed (chunk 2026-10-17-cli-machine-contract) `hook` and every CLI verb, `run` and `revive` included, take the `VIOLA_DIR` step through the root bin's `resolve_home` / `resolved_home` (`src/cmd/mod.rs`), only when `VIOLA_NAME` and `VIOLA_DIR` pass `instance_of`'s shape check, a failing pair falling through to the default, never refusing; the CLI verbs' step is not ratified, the founder's word owed»
