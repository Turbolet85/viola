# Scope — 2026-10-17-optional-plugin-install

**Working entry (verbatim title + hint):** Optional plugin install — MCP-only plugin for unwrapped drivers, absolute-path
command, never two hook processes per event, MCP-server precedence measured as ledger row

**Capability:** `verification-matrix.json#v1-42` (Optional plugin install for unwrapped drivers; `planned`, unclaimed at
take-up).

## What it builds
- **The verb `viola plugin install`** — the one two-word verb (layout-templates §Flat verbs), served for the first time:
  a clap subcommand under the global `--home`, with the CLI's `--json` / exit-code contract like every other verb.
- **An MCP-only plugin in a local marketplace** under `<viola home>/marketplace/` (architecture §Directory tree:
  "`marketplace/`: written only by `viola plugin install`"): a marketplace manifest plus one plugin carrying
  `.claude-plugin/plugin.json` and `.mcp.json` registering the server `viola` as `{"command": <abs exe>, "args": ["mcp"]}`
  — and NO hooks (architecture §Plugin Scope). [intent-amended at P5 val-1: the `args` are `["--home", <home>, "mcp"]`,
  so an unwrapped driver's server runs on the home the install was made for, never the default home or the NOT-ratified
  `VIOLA_DIR` step; `mcp` takes the global `--home` (architecture §Occupied Resources); plan.md §Provenance]
- **The command is an absolute path, never a PATH lookup** — "the absolute path of the exe that ran `plugin install`"
  (architecture §Plugin Scope; security-plan's NEVER clause on `viola plugin install`'s `.mcp.json`).
- **Never two hook processes per event** — a wrapped session that also has the installed plugin runs only the
  `--plugin-dir` plugin's hooks, because the installed plugin carries none (architecture §Plugin Scope).
- **MCP-server precedence as a capability-ledger row** — whether Claude Code loads both a `--plugin-dir` plugin and an
  installed plugin of the same name, and which `viola` MCP server a wrapped session then uses, measured by a `viola
  verify` step beside the landed rows (CARRY of chunk 2026-09-28-capability-ledger-and-viola-verify; `LedgerRow::ALL`
  holds 23 rows at take-up, `crates/viola-agent-claude/src/ledger.rs:48`).
  [intent-amended at P5 val-1: the founder's own answer to P4 fork Q2 (`inputs#I2`) — measured once in a live round and
  recorded as a fact, no 24th `LedgerRow` and no verify step, by the ruled limit "a shape no send relies on needs no
  probe"]
- **`viola --help`'s `setup:` row gains `plugin install`** once the verb is served, and the `HELP` literal of
  `help_is_the_grouped_verb_table` and `help_lists_only_served_verbs` (`tests/cli_output_plain.rs`) move with it (CARRY of
  chunk 2026-10-17-cli-output-discipline, its `chunks/2026-10-17-cli-output-discipline/inputs/I3-relay-3.md.txt`, the
  operator's word).
- **The exit-1 `viola mcp` measurement** — whether a registered `viola mcp` that exits 1 (its start-time home check
  refusing) blocks Claude Code's start, measured here, the entry that holds the plugin-registration measurements (CARRY of
  chunk 2026-10-17-mcp-server-for-drivers, its `chunks/2026-10-17-mcp-server-for-drivers/inputs/I3-relay-3.md.txt`; that
  chunk's live starts were capped by the founder, `chunks/2026-10-17-mcp-server-for-drivers/inputs/I2-relay-2.md.txt` Q2
  and `chunks/2026-10-17-mcp-server-for-drivers/inputs/I4-relay-1.md.txt`, and both went to the passing path).

## Folded freight and premises
- How the installed plugin reaches Claude Code is not stated by the entry: viola may write the marketplace and print the
  CLI's own steps for the user, or spawn the CLI to run them. Verified at P3 as a real two-way choice: on 2.1.287 a local
  marketplace is added by `claude plugin marketplace add <path>` and installed by `claude plugin install viola@viola`, both
  writing a Claude Code settings scope (`--scope user|project|local`, default `user`). A spawn makes the CLI write under
  `~/.claude` on viola's call, which today viola never causes outside `viola verify`'s accepted transcript residual
  (security-plan, Data Protection) — a boundary widening, the founder's own word (`inputs#I1`; P4 fork Q1). Answered: write + print, no `claude` child (`inputs#I2` Q1).
- Measuring the precedence needs a session in which a same-named plugin is INSTALLED beside a `--plugin-dir` one
  (verified: `--plugin-dir` loads a plugin "for this session only", and the help does not say whether both same-named
  plugins load). Which `viola mcp` started is readable from whose `<home>/diagnostics/mcp.ndjson` gains a `process-start`
  when the two registrations name two homes. A `viola verify` probe doing it would install (and remove) a plugin in the
  CLI's configuration on every verify; a one-off live round records it once; which — and whether it is a 24th `LedgerRow`
  at all, given the ruled limit "a shape no send relies on needs no probe" — is the founder's word (`inputs#I1`; P4 fork
  Q2).
- Every live `claude` start this chunk makes (the precedence round, the exit-1 round) counts against a cap that is the
  founder's own word (`inputs#I1`; P4 fork Q4), as at chunk 2026-10-17-mcp-server-for-drivers. Answered: two starts, a
  red recorded and never re-fired; the round's four `claude plugin` commands at `--scope local` from a rig dir under
  `target/` (`inputs#I2` Q3, Q4).
- Mechanism claim kept verbatim from the folded freight (architecture §Deployment): "on the operator ruling at the
  2026-09-27-instance-state-and-start-order plan that a registered command that exits 2 would block Claude Code" — a
  ruling, never a measurement; and "Whether a registered `viola mcp` that exits 1 blocks the CLI's start is unmeasured".
  Verified at P3 as stated: the architecture text calls the first a ruling and the second unmeasured; nothing in the tree
  measures either. The live round measures the exit-1 case.
- "The absolute path of the exe that ran `plugin install`" — leaned to the canonical `std::env::current_exe()` per
  architecture §Plugin Scope ("the absolute path of the exe that ran `plugin install`"); `pin_and_plugin` takes the same
  `current_exe()` before pinning (`src/cmd/run.rs:394`); the per-frame `v` + sender version is the stated safety net for an
  install not bound to a pinned copy (architecture §Deployment).
- A new ledger row (only on P4 fork Q2's ledger branch) changes `LedgerRow::ALL` (23 → 24, `ledger.rs:48`) and, if
  relied, the relied set that gates `cli_verified`: every home stamped before this chunk would read `cli_verified:false`
  until its next `viola verify`; its Windows reading is owed on "Windows-only live measurements".
- Obs: the dispatch span `cli.plugin_install` and, with `VIOLA_NAME` set, `process-start` / `process-exit{exit_code,
  detail}` to `cli-<name>.ndjson` (obs-plan §4 `verify` / `plugin install`). Verified: `role_of` files `plugin` as `cli`
  (`src/main.rs:94-119`); the span is not declared yet (0 hits for `cli.plugin` / `plugin_install`).
- Disk: the marketplace tree takes the home's 0700 dirs / 0600 files through `create_private_dir` and
  `viola_state::fs::replace_private`; a re-run rewrites it (idempotent). Verified: both helpers exist
  (`crates/viola-state/src/fs.rs:35`, `:263`).
- [premise-corrected: the embedded `render` is a raw string replace with no JSON escaping, `crates/viola-agent-claude/src/lib.rs:180-184`]
  The installed `.mcp.json` cannot reuse the wrapped plugin's `@@VIOLA_BIN@@` template for an arbitrary `current_exe` /
  home path: it is built through `serde_json` by a pure renderer in `viola-agent-claude`.

## CI read at take-up (Setup 5a)
- `9afbff4` (the last wrap's flip, HEAD) — `ci#39043417231`, green, checks 15/15, wall 608 s. Nothing to disposition.

## Boundaries
- No hooks in the installed plugin, ever; no PATH lookup in any `command`.
- The wrapped path (`viola run`'s `--plugin-dir` plugin) is unchanged except where the precedence measurement requires.
- No uninstall verb is named by the entry [inferred: out of scope unless the precedence probe needs its own removal].
- Windows legs of every live measurement are owed on "Windows-only live measurements" (no interactive Windows host).
- No write under `~/.claude` and no live `claude` start beyond what the founder's word allows (`inputs#I1`).
