proposals:
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions [Plugin Scope]
    change: 'Replace "Whether Claude Code loads both a `--plugin-dir` plugin and an installed plugin with the same name, and which `viola` MCP server a wrapped session then uses, is a capability-ledger row" with: that precedence is no ledger row (no `send` relies on it, the ruled limit; inputs#I2 Q2) and is not measured: the live round of chunk 2026-10-17-optional-plugin-install took none of its readings (both wrapped starts held by the folder-trust modal before SessionStart, start 3 unspent), owed on the CARRY on "Compact post-condition ledger row" (inputs#I8), its Windows leg on "Windows-only live measurements" (inputs#I3 card row 1). Also as built: `viola plugin install` writes only `<home>/marketplace/` (marketplace `viola`, plugin `viola@viola`, MCP-only, no `hooks` key in any document) and spawns no `claude`; the `.mcp.json` `command` is the canonical `current_exe` in forward slashes and `args` is `["--home", <canonical home>, "mcp"]`, built by `serde_json` (`viola_agent_claude::marketplace::marketplace_files`).'
    sidecar: 'arch [Plugin Scope]: plugin precedence is no capability-ledger row (ruled limit, inputs#I2 Q2), not measured, owed on the CARRY on "Compact post-condition ledger row"; the install command and args written as built (chunk 2026-10-17-optional-plugin-install).'
    rationale: 'The report''s Expected amendment 1 and Spec claims disproved (claim 2): the founder ruled no 24th LedgerRow (inputs#I2 Q2) and the round read none of the precedence readings (Deviations 4; Outcome: the live criterion UNMET, v1-42 un-claimed per inputs#I8). The locked text still says it is a capability-ledger row. Symbols: `marketplace_files` builds `command` from the canonical exe in forward slashes and `args` ["--home", home, "mcp"].'
    basis: .andromeda/architecture.md:97
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions [CLI Version Compatibility]
    change: 'The "Ledger rows:" bullet "Plugin precedence: whether a `--plugin-dir` plugin and an installed plugin with the same name both load, and which MCP server `viola` wins." leaves the ledger-row list as a row: it now reads as a measurement owed with no ledger row (no `send` relies on it, inputs#I2 Q2), unmeasured at chunk 2026-10-17-optional-plugin-install and owed on the CARRY on "Compact post-condition ledger row" (inputs#I8), its Windows leg on "Windows-only live measurements".'
    sidecar: 'arch [CLI Version Compatibility]: "Plugin precedence" leaves the ledger-row list; an owed measurement on the CARRY of inputs#I8, no row (chunk 2026-10-17-optional-plugin-install).'
    rationale: 'Report Expected amendment 2: the owed-row bullet leaves the list as a row (no row, inputs#I2 Q2). Same claim as the [Plugin Scope] primary, restated in the ledger-row list.'
    basis: .andromeda/architecture.md:84
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions [CLI Version Compatibility]
    change: 'Replace "plugin precedence stays owed" with: plugin precedence is no ledger row (inputs#I2 Q2); its reading, not taken at chunk 2026-10-17-optional-plugin-install, is a measurement owed on the CARRY on "Compact post-condition ledger row" (inputs#I8).'
    sidecar: 'arch [CLI Version Compatibility]: "plugin precedence stays owed" becomes a measurement owed on the CARRY, not a row (chunk 2026-10-17-optional-plugin-install).'
    rationale: 'Report Expected amendment 2 (`:88@c18434`): the sentence reads the precedence as an owed ledger row; the founder ruled no row.'
    basis: .andromeda/architecture.md:88
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions [Deployment / Distribution]
    change: 'Keep "Whether a registered `viola mcp` that exits 1 blocks the CLI''s start is unmeasured" but replace its reason "(the founder''s live-start cap)" with the attempt as recorded: chunk 2026-10-17-optional-plugin-install''s round made 2 starts of a cap raised to 3, both held by the folder-trust modal before SessionStart, start 3 unspent (inputs#I7); the reading is owed on the CARRY on "Compact post-condition ledger row" (inputs#I8), its Windows leg on "Windows-only live measurements".'
    sidecar: 'arch [Deployment / Distribution]: the exit-1 start reading stays unmeasured; the attempt recorded (2 of 3 starts, held at the trust dialog), owed on the CARRY of inputs#I8 (chunk 2026-10-17-optional-plugin-install).'
    rationale: 'Report Expected amendment 3 and Deviations 4: the exit-1 start was attempted inside the raised cap and not read; the stated reason (the cap) no longer explains why it is unmeasured.'
    basis: .andromeda/architecture.md:96
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources (Binary, subcommands and exit codes)
    change: 'Add: `plugin install` takes `[--json]` and nothing else but the global `--home` (`viola plugin` alone is a clap usage error; clap''s own `viola plugin help [install]` is served); it runs `check_path` on the home and each existing `marketplace/`, `marketplace/.claude-plugin/`, `marketplace/viola/`, `marketplace/viola/.claude-plugin/` before anything is created, written or opened, and exits 0 with its result (one stdout write: the `ok` document under `--json`, else the four planning-form lines naming `<canonical home>/marketplace` and the two `claude plugin` commands), 21 on a refused check (nothing created, no log opened), 1 for an internal error through the `cli` catch site (a failed `current_exe`, dir creation or file write) and 2 for a clap usage error (chunk 2026-10-17-optional-plugin-install).'
    sidecar: 'arch §Occupied Resources: `plugin install [--json]` registered with exits 0 / 1 / 2 / 21 and its check order (chunk 2026-10-17-optional-plugin-install).'
    rationale: 'Report Symbols: the new verb `viola plugin install [--json]` and its order steps 1-7 (exit 21 refusal, `cli` catch site exit 1, `viola plugin` alone exit 2); Expected amendment 4. The subcommand row lists the verb but registers no argument or exit code for it.'
    basis: .andromeda/architecture.md:301
  - detector: D-arch-resources
    severity: warning
    section: §Conventions (CLI exit codes)
    change: 'The `21` bullet gains: `viola plugin install`''s refused `check_path` of the home or an existing marketplace tree dir (`--json` `{"v":1,"error":"instance-unreachable","detail":null}` through `client::refused_document`, stderr empty; human mode the `home_state_pair` through `human::refuse`, stdout empty; nothing created, no log opened; chunk 2026-10-17-optional-plugin-install).'
    sidecar: 'arch §Conventions exit 21: gains `plugin install`''s refused strict-modes check (chunk 2026-10-17-optional-plugin-install).'
    rationale: 'Report Symbols, order step 3: a refusal is exit 21 with the `instance-unreachable` document. The exit-code registry names `viola list` as the only strict-modes use of 21.'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Conventions (CLI exit codes)
    change: 'In the `1` bullet''s catch-site list of verbs logged under `cli` ("so `send`, `wait`, `last`, `list`, `answer`, `pause`, `release`, `verify` and `revive --list`"), add `plugin install`: a dispatch `Err` (a failed `current_exe`, dir creation or file write) prints exactly `error: internal error` once from the `main` catch site, exit 1.'
    sidecar: 'arch §Conventions exit 1: `plugin install` joins the `cli`-role catch-site verbs (chunk 2026-10-17-optional-plugin-install).'
    rationale: 'Report Symbols: "A failed `current_exe`, dir creation or file write reaches the `cli` catch site: exit 1, `error: internal error`"; the dispatch arm runs process `Cli`.'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources (Filesystem)
    change: 'Replace "`marketplace/`: written only by `viola plugin install`." with: `marketplace/` with its dirs `.claude-plugin/`, `viola/`, `viola/.claude-plugin/` (each 0700 via `create_private_dir`, the home created when absent) and three files `.claude-plugin/marketplace.json`, `viola/.claude-plugin/plugin.json`, `viola/.mcp.json` (each 0600 via `replace_private`, rewritten whole on a re-run; any other entry left as found), written only by `viola plugin install` after `check_path` passed on the home and each existing tree dir, and never read back by viola (no trusted-file entry, no re-hash of the installed `command`; the integrity remainder owned by "Home and code-bearing file integrity", inputs#I3 card row 2).'
    sidecar: 'arch §Occupied Resources Filesystem: the `marketplace/` tree''s dirs, files and modes registered (chunk 2026-10-17-optional-plugin-install).'
    rationale: 'Report Schema / config: "The tree under `<home>/marketplace/` (5 dirs at 0700, 3 files at 0600) is a new on-disk shape written only by this verb and never read back by viola"; order step 5; Expected amendment 4 (`:342`, change); Beyond-the-plan row 2 (inputs#I3).'
    basis: .andromeda/architecture.md:342
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources (Filesystem, `diagnostics/`)
    change: 'The `cli` producers gain `plugin install`: only under a parsed `VIOLA_NAME`, `process-start{subject:"self"}` and `process-exit{subject:"self"}` into the caller''s `cli-<VIOLA_NAME>.ndjson` (the `cli.plugin_install` span opened by `write_tree` after the log opens); with no name it opens no `cli-*.ndjson`, and a refused check opens no role file and writes no line.'
    sidecar: 'arch §Occupied Resources diagnostics/: `plugin install` joins the `cli` producers, the self pair under a parsed name only (chunk 2026-10-17-optional-plugin-install).'
    rationale: 'Report Symbols, order steps 4-5 and 7 (`client::start` under a parsed `VIOLA_NAME`, `run::log_self_exit(0, None)`); Expected amendment 4 ("`diagnostics/`''s producers gain `plugin install`"); Outcome: the self pair under a name, no `diagnostics/` without one.'
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts (CLI `--json` output)
    change: 'Add a bullet: `viola plugin install --json` prints `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}` (exit 0, one write, no path), and on a refused strict-modes check `{"v":1,"error":"instance-unreachable","detail":null}` (exit 21, stderr empty) through `client::refused_document`, which now has `plugin install` as a caller beside `revive` (chunk 2026-10-17-optional-plugin-install).'
    sidecar: 'arch §Standard Contracts CLI --json: the install''s `ok` document and its `instance-unreachable` refusal (chunk 2026-10-17-optional-plugin-install).'
    rationale: 'Report Symbols, order steps 3 and 6, and "Remaining callers: `client::refused_document` gains a sixth caller (`plugin install`)"; Expected amendment 5.'
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources (Claude Code integration names)
    change: 'Add: the local marketplace written by `viola plugin install` is named `viola` (`viola_agent_claude::marketplace::NAME`), its one plugin `viola` with source `./viola`, referenced as `viola@viola` (`PLUGIN_REF`), whose `.mcp.json` registers the MCP server `viola` as `{"command":<canonical exe, forward slashes>,"args":["--home",<canonical home, forward slashes>,"mcp"]}`; the plugin.json version is `CARGO_PKG_VERSION`; no hooks.'
    sidecar: 'arch §Occupied Resources Claude Code integration names: marketplace `viola`, plugin ref `viola@viola` and the installed MCP server''s argv (chunk 2026-10-17-optional-plugin-install).'
    rationale: 'Report Symbols: `viola_agent_claude::marketplace` with `NAME = "viola"`, `PLUGIN_REF = "viola@viola"` and the three documents; the integration-names registry holds only the wrapped plugin''s names.'
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: 'The `viola-agent-claude/` row gains the pure `marketplace` module (`marketplace_files`: the three `viola plugin install` documents, `NAME`, `PLUGIN_REF`; the embedded `plugin_files` / `render` unchanged).'
    sidecar: 'arch Project directory structure: `viola-agent-claude` gains the pure `marketplace` module (chunk 2026-10-17-optional-plugin-install).'
    rationale: 'Report Crates / modules: "added module `viola_agent_claude::marketplace`"; the crate row lists its pure modules (`agents`, `statusline`) and not this one.'
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: 'The root bin rows gain: `cmd/plugin.rs` (the `plugin install` verb: `tree_passes` over the home and four marketplace dirs before any write or log, then `write_tree`); `human/verbs.rs` gains `write_plugin_installed` and `command_word` / `shell_word` (and is a Windows-gated source through `cfg!(windows)`, listed in `windows-mutants.yml`''s `viola-main` job); and `human.rs`''s caller list gains `plugin install`.'
    sidecar: 'arch Project directory structure: `cmd/plugin.rs`, the install''s human writers in `human/verbs.rs` and `plugin install` among `human`''s callers (chunk 2026-10-17-optional-plugin-install).'
    rationale: 'Report Files and Symbols: new `src/cmd/plugin.rs`; `src/human/verbs.rs` `write_plugin_installed` (57-72), `command_word` (74-77), `shell_word` (79-98), re-exported from `src/human.rs:25-26`; `human::refuse` census gains `plugin install` 1; Deviations 3 (Windows-gated source).'
note: 'No new dependency, crate, IPC method, endpoint, event or env var this chunk (report Dependencies, Crates, Schema); §Stack and §Inherited Defaults stand. The report''s "5 dirs at 0700" counts the home with the four marketplace dirs (order step 2 / step 5: four tree dirs made), so the Filesystem change names four.'
