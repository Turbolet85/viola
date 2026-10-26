group: architecture+security-plan
read: .claude/rules/api.md · 41 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/api.md:20   class: retired
  leaf-text: «`link` and `unlink` are the contract's and not built: the wrapper answers both `-32601`.»
  source: .andromeda/architecture.md:193 «`link` `{driver, driven}` → `{since, already_linked}` and `unlink` `{driver, driven}` → `{}`, each sent to the driven instance's wrapper; a missing, non-string or invalid name,»
  replace-with: «`link` `{driver, driven}` → `{since, already_linked}` (a held pair appends nothing and answers its `since` with `already_linked: true`) and `unlink` `{driver, driven}` → `{}` (an absent pair appends nothing), each sent to the driven instance's wrapper, reading no `from` and answered only after its event line and the snapshot write; a missing, non-string or invalid name, a `driven` that is not the wrapper's own name and a `driver` equal to `driven` are each `-32602` with `data: null`, no line written, and a failed record or remove is `-32603`.»

- leaf: .claude/rules/api.md:23   class: omission
  leaf-text: «(CLI `send` / `wait` / `last` / `answer` / `pause` / `release`, `viola hook`'s»
  source: .andromeda/architecture-detail/standard-contracts.md:51 «Before its first frame, the client of `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.event` and `hook.dialog`»
  replace-with: «(CLI `send` / `wait` / `last` / `answer` / `pause` / `release` / `link` / `unlink`, `viola hook`'s»

- leaf: .claude/rules/api.md:28   class: omission
  leaf-text: «exit 21, through `client::refused_document`): one JSON document»
  source: .andromeda/architecture-detail/standard-contracts.md:93 «`viola link <driver> <driven> --json` prints `{"v":1,"ok":{"since":…,"already_linked":…}}` (`since` a string, `already_linked` a boolean) and `viola unlink <driver> <driven> --json` prints `{"v":1,"ok":{}}`»
  replace-with: «exit 21, through `client::refused_document`; `viola link <driver> <driven> --json` prints `{"v":1,"ok":{"since":…,"already_linked":…}}` and `viola unlink <driver> <driven> --json` `{"v":1,"ok":{}}`, each exit 0, and their refused strict-modes check on the home and `instances/<driven>` prints exactly `{"v":1,"error":"instance-unreachable","detail":null}`, exit 21, stderr empty, no line in any log): one JSON document»

- leaf: .claude/rules/api.md:29   class: omission
  leaf-text: «failing `viola_state::strict::check_path` (nothing on stdout or stderr, no role file) · 2 usage»
  source: .andromeda/architecture-detail/conventions.md:26 «for `viola link` / `viola unlink` exit 1 also covers a refusal reply and an `ok` of another shape than the verb's (`link`: a string `since` and a boolean `already_linked`; `unlink`: an object)»
  replace-with: «failing `viola_state::strict::check_path` (nothing on stdout or stderr, no role file), `viola link` / `viola unlink`'s refusal reply or an `ok` of another shape than the verb's (`link`: a string `since` and a boolean `already_linked`; `unlink`: an object) · 2 usage»

- leaf: .claude/rules/api.md:29   class: omission
  leaf-text: «· 20 wrapper fault ·»
  source: .andromeda/architecture-detail/conventions.md:33 «`20` protocol fault from the wrapper, a `-32602` from `link` / `unlink` included (a missing, non-string or invalid name, a `driven` that is not the serving wrapper's own name,»
  replace-with: «· 20 wrapper fault (a `-32602` from `link` / `unlink` included: a missing, non-string or invalid name, a `driven` that is not the serving wrapper's own name, or a `driver` equal to `driven`, the self-pair refused by the wrapper only, never client-side) ·»

- leaf: .claude/rules/api.md:29   class: omission
  leaf-text: «(on `send`, `wait`, `last`, `answer`, `pause` and `release` also a server»
  source: .andromeda/architecture-detail/conventions.md:40 «Exit `21` also covers, on `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`, a server that is not the recorded wrapper»
  replace-with: «(on `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` also a server»

- leaf: .claude/rules/api.md:29   class: omission
  leaf-text: «the client's own exit-13 refusals come first, and the cause has no exit code»
  source: .andromeda/architecture-detail/conventions.md:40 «The client's own refusals (exit 13), and for `link` / `unlink` the instance check of the driven instance (exit 21), come first, before any connect»
  replace-with: «the client's own exit-13 refusals, and for `link` / `unlink` the instance check of the driven instance (exit 21), come first, before any connect, and the cause has no exit code»

- leaf: .claude/rules/api.md:29   class: omission
  leaf-text: «nothing created, no log opened). `viola hook` always exits 0.»
  source: .andromeda/architecture-detail/conventions.md:34 «`viola link` / `viola unlink`'s refused instance check of the driven instance (`check_instance` on the home and `instances/<driven>`, before any role file or log opens) is the same exit and document»
  replace-with: «nothing created, no log opened), and `viola link` / `viola unlink`'s refused instance check of the driven instance (`check_instance` on the home and `instances/<driven>`, before any role file or log opens: the same exit and document; human mode the instance-state `unable:` / `hint:` pair; no log opened, no line written, the mode left as found). `viola hook` always exits 0.»

- leaf: .claude/rules/api.md:30   class: omission
  leaf-text: «and `check_instance` per instance before any file under the home is opened. Interim ratified gaps»
  source: .andromeda/security-plan-detail/authentication-authorization.md:66 «No dated gap: both verbs run `viola_state::strict::check_instance` on the absolute home and `instances/<driven>` before `client::start`»
  replace-with: «and `check_instance` per instance before any file under the home is opened, and `viola link` / `viola unlink` run `check_instance` on the home and `instances/<driven>` before `client::start` (before any role file, log or snapshot read), then the liveness-only pre-check and, on Linux and Windows, the server check through `cmd::client::connect`, borrowing no dated gap. Interim ratified gaps»
