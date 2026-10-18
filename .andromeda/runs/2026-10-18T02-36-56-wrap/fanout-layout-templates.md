proposals:
  - detector: D-layout-surface
    severity: warning
    section: '§Surface: cli → a new Output structure — `viola plugin install` (placed after Output structure — `viola revive`)'
    change: 'Add a wireframe and as-built paragraph for `viola plugin install [--json]` (chunk 2026-10-17-optional-plugin-install). Exits 0 / 1 / 2 / 21. On success, human mode prints one plain-ASCII stdout write of four planning-form lines: `viola@viola  written  <dir>` / `run these to add it to Claude Code:` / `  claude plugin marketplace add <dir>` / `  claude plugin install viola@viola`. `<dir>` is `<canonical home>/marketplace` passed through `command_word`: on Unix it is bare when every char is an ASCII letter, a digit or `_ - . / :`, else single-quoted; on Windows it is double-quoted only when it holds a space. No SGR, written once, the founder''s wording owed. `--json` prints `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}` with no path. A failed strict-modes check of the home or the marketplace tree dirs is exit 21 with nothing created and no log opened: human mode prints the reused `home_state_pair` `unable:` / `hint:` pair on stderr with stdout empty, and `--json` prints `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty. A failed exe read, dir creation or file write is the `cli` catch site''s `error: internal error`, exit 1. `viola plugin` alone is a clap usage error, exit 2. clap also serves its own `viola plugin help [install]`.'
    sidecar: 'layout-templates §Surface: cli — new Output structure — `viola plugin install` (wireframe + as built: the four planning-form lines, the `ok` document, the exit-21 refusal in both modes, exit 1 / 2), chunk 2026-10-17-optional-plugin-install.'
    rationale: 'The report''s Changes add a new user-facing CLI verb with human and `--json` output (Symbols / APIs: `viola plugin install`, the verb''s order steps 3 and 6, `write_plugin_installed`, `command_word`). Expected amendment 14 names "A new Output structure — `viola plugin install`". layout-templates has no wireframe entry for this surface, only its name in the `--help` row, so the surface is undocumented.'
    basis: 'src/cmd/plugin.rs:48-76; src/human/verbs.rs:57-72; src/human/verbs.rs:79-98'
  - detector: D-layout-surface
    severity: warning
    section: '§Surface: cli → Primary screens (commands)'
    change: 'Add a bullet: `viola plugin install` writes an MCP-only local marketplace under `<home>/marketplace/` (no hooks) and prints the two `claude plugin` commands that add it. It spawns no `claude`. Its output is under Output structure — `viola plugin install`.'
    sidecar: 'layout-templates Primary screens — `viola plugin install` listed as a served command (chunk 2026-10-17-optional-plugin-install).'
    rationale: 'The Primary screens list names every command surface, and this chunk adds `plugin install` (report Changes, Symbols / APIs; expected amendment 6, "spawns no `claude`"). The list does not name it yet.'
    basis: 'src/cmd/plugin.rs:48-76'
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: '§Surface: cli → Primary screens (commands)'
    change: 'The `viola --help` bullet: the setup row now holds `plugin install`, served by chunk 2026-10-17-optional-plugin-install. Drop "`plugin install` join[s] when … \"Optional plugin install\" serve[s]". Only the `handoff:` row still waits on "Explicit session links" (served-verbs-only stays PROVISIONAL).'
    sidecar: 'layout-templates :60 — `plugin install` served in `--help`''s setup row; only the handoff row stays pending.'
    rationale: 'Report Changes, `HELP_TEMPLATE` (`src/cmd/mod.rs:35`): `setup:    run, revive, verify, plugin install`. Expected amendment 14 lists `layout-templates.md:60` as a change. The pending-join claim is retired.'
    basis: 'layout-templates.md:60; src/cmd/mod.rs:35'
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: '§Surface: cli → Output structure — `viola --help`'
    change: 'As built: the setup row reads `setup:    run, revive, verify, plugin install` (chunk 2026-10-17-optional-plugin-install) and matches the wireframe''s row. The `HELP` literal and `help_lists_only_served_verbs` now take the two-word verb, and a bare `plugin` stays unserved. Remove "and `plugin install` with \"Optional plugin install\"". Only the `handoff:  link, unlink` row stays pending.'
    sidecar: 'layout-templates :216 — `viola --help` as built carries `plugin install` in setup:, chunk 2026-10-17-optional-plugin-install.'
    rationale: 'Counts / qualifiers moved, pattern `setup-row`: "`layout-templates.md:216` (as built, stale)". Expected amendment 14 lists `:216` as a change, and `:213` the wireframe as no change.'
    basis: 'layout-templates.md:216; src/cmd/mod.rs:35'
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: '§Surface: cli → Component — Primary navigation (verb structure)'
    change: 'Discoverability as built: `--help` prints board / traffic / wheel / setup over the served verbs, and setup includes `plugin install` (chunk 2026-10-17-optional-plugin-install). Only the handoff group still waits on its route entry (`inputs#I1` Q3 of chunk 2026-10-17-cli-output-discipline, PROVISIONAL).'
    sidecar: 'layout-templates :279 — Discoverability: `plugin install` served under setup.'
    rationale: 'Expected amendment 14 lists `layout-templates.md:279` as a change. The bullet still says `plugin install` joins later, which the served `HELP_TEMPLATE` retires (report Changes).'
    basis: 'layout-templates.md:279; src/cmd/mod.rs:35'
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: '§Surface: cli → Component — Primary content block 2: refusal lines and the `unable` column'
    change: 'Line form: the catch-site verb list ("A failed or panicked `cli` verb (`send`, … `revive --list`…)") gains `plugin install` (exit 1, `error: internal error`, no hint). The fixed-message exceptions gain `viola plugin install`''s exit-21 strict-modes refusal: it reuses the `home_state_pair` pair, stdout empty, and nothing is created and no log opened.'
    sidecar: 'layout-templates Primary content block 2 — `plugin install` joins the cli catch-site list and the exit-21 home-pair exceptions.'
    rationale: 'Report Changes, the verb''s order: step 3 (a refusal is exit 21, `human::home_state_pair` through `human::refuse`, stdout empty) and the failure bullet ("reaches the `cli` catch site: exit 1, `error: internal error`"). Expected amendment 14: "the catch-site verb list gains `plugin install`". The report gives no line for this site.'
    basis: 'src/cmd/plugin.rs:50-60'
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: '§Surface: cli → Component — Primary content block 2: refusal lines and the `unable` column'
    change: 'Exit codes as the typed tail: the exit-21 parenthetical "(for `viola list` the fixed state pair above instead)" also covers `viola plugin install`, whose exit 21 is the same kind of fixed home-state pair, or the `instance-unreachable` document under `--json`.'
    sidecar: 'layout-templates exit-21 tail — `plugin install` named beside `viola list` as a fixed state-pair refusal.'
    rationale: 'Report Changes, the verb''s order step 3: exit 21 prints `home_state_pair` in human mode and `{"v":1,"error":"instance-unreachable","detail":null}` under `--json`. The exit-21 tail names only `viola list` as taking the state pair. The report gives no line for this site.'
    basis: 'src/cmd/plugin.rs:50-60'
    dependent-of: D-layout-surface
note: 'Swept and left unchanged: the `--help` wireframe row (:213), the two-word-verb claim (:275) and the IA note (:320), which are true per report amendment 14. The banner bullet''s "`viola hook` / `viola mcp` have no human surface" stays true. Proposal 1 says `home_state_pair` is "reused" as the report does; the orchestrator should read its text from src/human (likely `viola list`''s home pair) before it pastes the literal into the wireframe.'
