# Codebase Research — 2026-10-17-optional-plugin-install

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 11
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 17 Session Additions; the 2026-10-10 entry applies to any live leg (the bare `claude` on this host resolves to 2.1.296, the stamped 2.1.287 sits at `~/.local/share/mise/installs/claude/2.1.287/claude` and goes first on the rig's `PATH`, read by `claude --version` by path before the first start).
- **Platform issues consulted:** none — no runner-only bullet (Setup 5a read one green run) and no CI-reading entry outside the operator leg.
- **External inputs:** `inputs#I1` — the operator's invocation: the P4 forks are raised as dialogs; a boundary widening and a live `claude` start cap are the founder's own word.

## Files inspected
- `src/cmd/mod.rs` (full) — the clap `Command` enum (no `Plugin` arm), `HELP_TEMPLATE`'s `setup:    run, revive, verify` row (`src/cmd/mod.rs:34`), `dispatch` and `resolved_home` / `canonical_home` (`src/cmd/mod.rs:239-278`).
- `src/main.rs` (30-119) — `role_of` (`src/main.rs:94-119`) files every first word other than `hook`, `mcp`, `revive` (without `--list`), `run` or a flag as `Role::Cli`, so `plugin` is a `cli` verb with no change to `role_of`.
- `src/cmd/run.rs` (385-411) — `pin_and_plugin` takes `std::env::current_exe()`, pins it, then writes `plugin_files(&pinned.path_fwd)` into `plugin/<key>/` through `create_private_dir` + `replace_private_shared`.
- `crates/viola-agent-claude/src/lib.rs` (100-189) — `PLUGIN_FILES` (three `include_str!` templates), `render` (a raw string replace of `@@VIOLA_VERSION@@` / `@@VIOLA_BIN@@`, no JSON escaping), `plugin_files`.
- `plugin/.mcp.json`, `plugin/.claude-plugin/plugin.json` (full) — the wrapped plugin's registration and manifest (name `viola`, description naming hooks).
- `src/cmd/list.rs` (1-130) — the strict-check-before-log order: `board::scan` first, a refusal is exit 21 with `{"v":1,"error":"instance-unreachable","detail":null}` or `human::home_state_pair`, then `client::start` only under `client::own_name()`, then `run::log_self_exit`.
- `src/cmd/client.rs` (10-125) — `start` (role-file open + `process-start`), `own_name`, `document`, `refused_document(code)`.
- `src/human.rs` (1-65) — `refuse` / `write_refusal`, `result` / `write_result`, `internal_error`; verb lines live in `src/human/verbs.rs`.
- `crates/viola-agent-claude/src/ledger.rs` (1-290) — `LedgerRow` (23 variants, `ALL: [Self; 23]` at `ledger.rs:48`), `id`, `words`, `relied()`.
- `tests/cli_output_plain.rs` (50-125) — the `HELP` literal (`tests/cli_output_plain.rs:56-68`) and `help_lists_only_served_verbs` (`tests/cli_output_plain.rs:79-123`), whose word filter drops any table word holding a space (`!word.contains(' ')`) and whose usage check reads `Usage: viola <verb>`'s second word, so a two-word verb needs that case reshaped, not only its list extended.
- `schemas/diag-line.v1.json` — `$defs.subject.enum` = `self · claude-child · version-probe · verify-probe · verify-pty-probe · agents-probe · statusline-shell` (read by a python `json.load`); no `plugin` value.
- `claude` 2.1.287 `--help`, `plugin --help`, `plugin marketplace --help`, `plugin marketplace add --help`, `plugin install --help`, `plugin uninstall --help`, `plugin marketplace remove --help`, `plugin validate --help`, `plugin list --help`, `mcp --help`, `mcp list --help` (by path, `~/.local/share/mise/installs/claude/2.1.287/claude`; no session started) — see Patterns detected.
- A scratch marketplace in the session scratchpad (`$TMPDIR/{scratchpad}/mkt/`), validated by 2.1.287's own `claude plugin validate --strict` (below).

## Graph impact (from the code-graph query; trace `.andromeda/runs/2026-10-17T15-14-51-phase/tree-query-2026-10-17-optional-plugin-install.json`)
- **plugin_files** — 3 callers: `cmd/run/pin_and_plugin` @ `src/cmd/run.rs:405`, two unit tests @ `crates/viola-agent-claude/src/lib.rs:589` / `:620` — the wrapped plugin's one writer; the installed plugin is a second, separate renderer, so `plugin_files` stays unchanged and its callers need no edit.
- **relied** — 5 callers: `ledger/verified` @ `crates/viola-agent-claude/src/ledger.rs:1207`, `cmd/verify/check_rows` @ `src/cmd/verify.rs:457`, three unit tests — touched only if a 24th row lands (P4 fork Q2).
- **role_of** — 2 callers (`main` @ `src/main.rs:53`, its unit table @ `src/main.rs:364`) — no change; a `plugin install` row in its table is a witness only.
- **refused_document** — 5 callers (`list` ×2, `revive`, `run`, `tests/cli_revive.rs`) — the shared `--json` refusal shape `plugin install` reuses.
- **write_plugin** (verify's probe plugin) — 5 callers in `src/cmd/verify.rs` and `tests/cli_fake_agent.rs` — the precedent for a probe-owned plugin dir, touched only under Q2's ledger branch.

## Patterns detected
- **The CLI's install surface, measured on 2.1.287** (`claude plugin marketplace add --help` / `plugin install --help`): a local marketplace is added by `claude plugin marketplace add <path>` (a URL, path or GitHub repo) and its plugin installed by `claude plugin install <plugin>@<marketplace>`; both take `--scope user|project|local` (default `user`) and `--json`; removal is `claude plugin uninstall <plugin> [--scope]` and `claude plugin marketplace remove <name> [--scope]`. Every one of them writes a Claude Code settings scope, so only the human or a `claude` child viola spawns can perform them (P4 fork Q1).
- **The manifest shape, measured** (`claude plugin validate --strict --json`, 2.1.287): `<root>/.claude-plugin/marketplace.json` `{"name","owner":{"name"},"description","plugins":[{"name","source":"./<dir>","description","author":{"name"}}]}` beside `<root>/<dir>/.claude-plugin/plugin.json` `{"name","version","description","author":{"name"}}` and `<root>/<dir>/.mcp.json` passes `--strict` with exit 0 on both the marketplace and the plugin; without `description` and `author` it reads `errors: []` and two warnings, exit 1 under `--strict` only. Whether `claude plugin validate` writes under `~/.claude` is unmeasured: a `find ~/.claude -maxdepth 3 -newer <scratch file>` after the first two runs listed `~/.claude/plugins/store/cc-plugin-diff_builtin-*.json` beside two session transcripts, and other live `claude` sessions run on this host, so the write is not attributed (no control was taken).
- **A session-only plugin load** (`claude --help`, 2.1.287): `--plugin-dir <path>` loads a plugin "for this session only", repeatable; `--setting-sources user,project,local` chooses which settings scopes load. Whether a `--plugin-dir` plugin and an installed plugin of the same name both load is not stated by the help and is the live measurement this chunk owes.
- **Which `viola mcp` answered is readable without a new capture arm** (`src/cmd/mcp.rs`, architecture §Plugin Scope, obs-plan §3 Log file location): `viola mcp` always writes `<home>/diagnostics/mcp.ndjson` (D-09), so two registrations naming two different homes (`--home A` in the installed `.mcp.json`; the wrapped session's own home through `VIOLA_DIR`) tell which server started by whose `mcp.ndjson` gains a `process-start` — the equality the precedence measurement needs, one start per reading.
- **Strict check before the log** (`src/cmd/list.rs:38-51`): check, refuse with no line, then `client::start` under a parsed `VIOLA_NAME`, then the work and `run::log_self_exit`.
- **The embedded plugin's string templating** (`crates/viola-agent-claude/src/lib.rs:180-184`): a raw replace that does not JSON-escape the substituted path; a home or exe path holding `"` or `\` would break `.mcp.json`. The wrapped path is safe because the pinned path is forward-slashed under the home; the installed `.mcp.json` carrying `--home <home>` and `current_exe` is built through `serde_json` instead.

## Conventions to follow
- **One module per verb in `src/cmd/`** (`src/cmd/mod.rs:1-13`), the arm in `dispatch` resolving the home through `resolved_home` (`src/cmd/mod.rs:239`), its chain to `cli_sink` only under the caller's own name (`src/cmd/mod.rs:150-158`, `list`'s form).
- **Writes under the home**: `create_private_dir` (`crates/viola-state/src/fs.rs:35`) and `replace_private` (`crates/viola-state/src/fs.rs:263`), 0700 / 0600.
- **Human text only through `crate::human`** (`src/human.rs:1-3`): a result line through `human::result`, a refusal pair through `human::refuse`, the home pair `human::home_state_pair` (`src/human/board.rs:25`).
- **`--json` documents through `client::document` / `client::refused_document`** (`src/cmd/client.rs:101-110`).
- **Claude-specific shapes only in `viola-agent-claude`** (the marketplace manifest and the installed plugin's files are a pure renderer there, beside `plugin_files`).

## New files to create
- `src/cmd/plugin.rs` — the `plugin install` verb: its clap args (`--json`), the home check, the marketplace write, its result line or document.
- `crates/viola-agent-claude/src/marketplace.rs` — the pure renderer of the marketplace tree: `marketplace.json`, the plugin's `plugin.json` and its `.mcp.json` (no hooks), built through `serde_json`.
- `tests/cli_plugin_install.rs` — the verb's root cases: the tree written, its modes, a re-run, the `--json` document and literal exits, the human line, no ESC byte.
- `viola-0.1.0/chunks/2026-10-17-optional-plugin-install/evidence/` — the live round's rig copies (`live-pty.py`, `live-ledger.py`, byte for byte from chunk 2026-10-17-mcp-server-for-drivers), its start ledger, the residual listings' digests and `live-round.md`.

## Files to modify
- `src/cmd/mod.rs` — the `Plugin` arm with its one `Install` sub-verb, its dispatch arm, `setup:` gaining `plugin install` in `HELP_TEMPLATE`.
- `crates/viola-agent-claude/src/lib.rs` — `pub mod marketplace;`.
- `src/human/verbs.rs` — the install result line.
- `src/human.rs` — its re-export.
- `src/main.rs` — a `plugin install` row in `role_of`'s unit table (a witness; the function is unchanged).
- `tests/cli_output_plain.rs` — the `HELP` literal's `setup:` row, and `help_lists_only_served_verbs` reshaped for the two-word verb.

## Sweeps
- `run, revive, verify` over `src crates tests fixtures docs .claude` (`*.rs *.md *.toml *.json`): 2 hits · 2 changed (`src/cmd/mod.rs:34`, `tests/cli_output_plain.rs:62`) · 0 no-change.
- `"plugin"` over `src crates tests` (`*.rs`): 16 hits · 1 changed (`tests/cli_output_plain.rs:120`, the unserved-word list) · 15 no-change (each a `plugin` path component in a `join`: the wrapped plugin dir, verify's probe plugin, the fake agent's and the drift tests' scratch plugins, the mcp test's home read).
- `plugin_install|cli\.plugin` over `src crates schemas` (`*.rs *.json`): 0 hits — the span is not declared yet (re-derived: `grep -rn 'plugin_install\|cli\.plugin' src crates schemas`).

## Open questions
- Who registers the plugin with Claude Code — the human running the CLI's two printed commands, or a `claude plugin …` child viola spawns (a write under `~/.claude` on viola's call)? → blocks: plan-decision (P4 fork Q1, the founder's own word, `inputs#I1`).
- Is the precedence a 24th `LedgerRow` with a `viola verify` probe (one more live start per verify and an install into the CLI's configuration on every verify), or one recorded live measurement with no probe (the CLAUDE.md ruled limit "a shape no send relies on needs no probe": no send relies on which `viola` server a wrapped session uses, the per-frame `v` + sender version guarding a version skew)? → blocks: plan-decision (P4 fork Q2, the founder's own word); the file list above is the no-ledger branch, and the ledger branch adds the verify, ledger, fake-agent, fixture and harness-literal files, written here after the fork.
- How many live `claude` 2.1.287 starts may this chunk make (the precedence reading and the exit-1 reading, one each)? → blocks: plan-decision (P4 fork Q3, the founder's own word).
- **Resolved at P4** by the founder's own answers, relayed verbatim by the Viola overseer (`inputs#I2`): Q1 write + print (no `claude` child, nothing under `~/.claude` written by viola); Q2 one live round, no 24th `LedgerRow` (the file lists above stand as the no-ledger branch); Q3 the round's four `claude plugin` commands at `--scope local` from a rig dir under `target/`, removed after, the residual measured; Q4 two live starts, a red recorded and never re-fired.
