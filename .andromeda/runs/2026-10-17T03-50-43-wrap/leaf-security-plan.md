group: security-plan
read: .claude/docs/security-summary.md · 77 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 43 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/security.md:25   class: stale
  leaf-text: «a refusal is the state-files `unable:` / `hint:` pair, exit 1, no child, no log opened and no line written»
  source: .andromeda/security-plan.md:184 «printing the state-files `unable:` / `hint:` pair on stderr, or under `--json` exactly `{"v":1,"error":"strict-modes-failed","detail":null}` on stdout with stderr empty»
  replace-with: «a refusal is exit 1, no child, no log opened and no line written, printing the state-files `unable:` / `hint:` pair on stderr, or under `--json` exactly `{"v":1,"error":"strict-modes-failed","detail":null}` on stdout with stderr empty (chunk 2026-10-17-cli-machine-contract)»

- leaf: .claude/rules/security.md:25   class: omission
  leaf-text: «a logged id listed live elsewhere is the closed refusal `session-live`, no child;»
  source: .andromeda/security-plan.md:203 «a refused strict-modes check or a preflight refusal then prints exactly `{"v":1,"error":"<code>","detail":null}` on stdout (stderr empty, exit 1)»
  replace-with: «a logged id listed live elsewhere is the closed refusal `session-live`, no child; `--json` is accepted on both arms (chunk 2026-10-17-cli-machine-contract): a refused strict-modes check or a preflight refusal prints exactly `{"v":1,"error":"<code>","detail":null}` on stdout (stderr empty, exit 1), `<code>` one of the closed `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`, and `--list --json` prints `{"v":1,"ok":{"sessions":[{"ts","cause","id"},…]}}` in log order — codes, closed words and session-shaped ids only, never a path or pid; a refusal `run`'s start makes after a passed preflight prints no document (owed to the route entry "Exit-cause code catalogue");»

- leaf: .claude/rules/security.md:26   class: omission
  leaf-text: «else for the default home only `~/.claude/settings.json`»
  source: .andromeda/security-plan.md:214 «A home named by `--home` or reached through `VIOLA_DIR` reads no user settings.»
  replace-with: «else for the default home only (`canonical_home(default_home(…))`; a home named by `--home` or reached through `VIOLA_DIR` reads none) `~/.claude/settings.json`»

- leaf: .claude/rules/security.md:23   class: omission
  leaf-text: «the wrapper re-runs them (client checks and schemars schemas are advisory).»
  source: .andromeda/security-plan.md:203 «The CLI verbs' `VIOLA_DIR` step and the canonicalisation widen what chooses a CLI verb's home and are NOT ratified: the founder's own word is owed»
  replace-with: «the wrapper re-runs them (client checks and schemars schemas are advisory). Every CLI verb (`run` and `revive` included; `hook` keeps its own shape check) resolves its home through `resolved_home` (chunk 2026-10-17-cli-machine-contract): `--home` as given; else the `instance_of` home when `VIOLA_NAME` passes `ViolaName::try_new` and `VIOLA_DIR` is an absolute `<home>/instances/<name>` (a pair that fails the check falls through, never refuses); else `<user home>/.viola`; then `canonical_home` applies `std::fs::canonicalize` when the path exists (on Windows only the verbatim-drive prefix stripped), while a path that does not exist keeps `std::path::absolute`'s spelling and nothing is created. The CLI verbs' `VIOLA_DIR` step and the canonicalisation are NOT ratified: the founder's own word is owed (the Viola overseer's answer at that chunk's wrap, a delegate's, `inputs#I3`).»

- leaf: .claude/docs/security-summary.md:65   class: stale
  leaf-text: «the stderr pair, exit 1, no log, no line.»
  source: .andromeda/security-plan.md:184 «printing the state-files `unable:` / `hint:` pair on stderr, or under `--json` exactly `{"v":1,"error":"strict-modes-failed","detail":null}` on stdout with stderr empty»
  replace-with: «the stderr pair, or under `--json` (accepted on both arms since chunk 2026-10-17-cli-machine-contract) exactly `{"v":1,"error":"strict-modes-failed","detail":null}` on stdout with stderr empty, exit 1, no log, no line; a preflight refusal under `--json` is the same document with its closed code (`already-live` · `no-session` · `session-live` · `cwd-missing`), and `--list --json` prints only the `ts`, the closed cause word and the session-shaped id — never a path or pid.»

- leaf: .claude/docs/security-summary.md:67   class: omission
  leaf-text: «for the default home only, `~/.claude/settings.json`»
  source: .andromeda/security-plan.md:214 «A home named by `--home` or reached through `VIOLA_DIR` reads no user settings.»
  replace-with: «for the default home only (`canonical_home(default_home(…))`; a home named by `--home` or reached through `VIOLA_DIR` reads none), `~/.claude/settings.json`»

- leaf: .claude/docs/security-summary.md:64   class: omission
  leaf-text: «and decides nothing on it.»
  source: .andromeda/security-plan.md:203 «The CLI verbs' `VIOLA_DIR` step and the canonicalisation widen what chooses a CLI verb's home and are NOT ratified: the founder's own word is owed»
  replace-with: «and decides nothing on it. Every CLI verb, `list`, `run` and `revive` included (`hook` keeps its own shape check), resolves its home through `resolved_home` since chunk 2026-10-17-cli-machine-contract: `--home`, else the `instance_of` home when `VIOLA_NAME` passes `ViolaName::try_new` and `VIOLA_DIR` is an absolute `<home>/instances/<name>` (a failing pair falls through, never refuses), else `<user home>/.viola`, then `canonical_home` canonicalises a path that exists and creates nothing; the `VIOLA_DIR` step and the canonicalisation widen what chooses a CLI verb's home and are NOT ratified — the founder's own word is owed (`inputs#I3`).»
