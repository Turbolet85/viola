group: architecture+security-plan
read: .claude/rules/api.md · 42 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/api.md:30   class: omission
  leaf-text: «and `viola list`'s refused strict-modes check of the home, `instances/` or any one instance, which refuses the whole board (`--json` `{"v":1,"error":"instance-unreachable","detail":null}`, no log opened)»
  source: .andromeda/architecture.md:136 «`viola plugin install`'s refused strict-modes check of the home or an existing `marketplace/` tree dir is the same exit and document (human mode the home-state pair; nothing created, no log opened)»
  replace-with: «and `viola list`'s refused strict-modes check of the home, `instances/` or any one instance, which refuses the whole board (`--json` `{"v":1,"error":"instance-unreachable","detail":null}`, no log opened), and `viola plugin install`'s refused `check_path` of the home or an existing `marketplace/` tree dir, run before anything under the home is created, written or opened (the same exit and document; human mode the home-state pair; nothing created, no log opened)»

- leaf: .claude/rules/api.md:29   class: omission
  leaf-text: «prints no document, owed to "Exit-cause code catalogue")»
  source: .andromeda/architecture.md:262 «`{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}` (exit 0): `viola plugin install --json`, no path; its refused strict-modes check prints the `instance-unreachable` document (exit 21)»
  replace-with: «prints no document, owed to "Exit-cause code catalogue"; `viola plugin install --json` prints `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}` with no path, exit 0, and its refused strict-modes check the `instance-unreachable` document, exit 21, through `client::refused_document`)»
