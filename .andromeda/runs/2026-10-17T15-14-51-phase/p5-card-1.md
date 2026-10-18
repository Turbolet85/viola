# P5 review — 2026-10-17-optional-plugin-install: "Optional plugin install"

Plan: `viola-0.1.0/chunks/2026-10-17-optional-plugin-install/plan.md` (592 lines, 453 of prose; 8 steps; 10 criteria
+ the `v1-42` claim). It rests on the founder's own answers to the four P4 forks (`inputs#I2`, relayed verbatim by the
Viola overseer, 2026-10-17T17:50Z).

## Goal
`viola plugin install` writes an MCP-only plugin in a local marketplace under `<home>/marketplace/` and prints the
two `claude plugin` commands the human runs. viola spawns no `claude` child and writes nothing under `~/.claude` (Q1).
- The plugin has no hooks.
- Its `.mcp.json` registers `viola` as the canonical absolute path of the running exe with
  `["--home", <home>, "mcp"]`.
- `--json` returns codes only.
- A widened home is refused with exit 21 before anything is written or logged.
- `--help`'s `setup:` row gains `plugin install`.

One live round on claude 2.1.287 then measures two things, within two starts (Q4):
- **Precedence:** which `viola` MCP server a wrapped session holding both plugins starts and answers with. It is
  recorded as a fact, with no 24th ledger row (Q2).
- **Exit 1:** whether a registered `viola mcp` that exits 1 blocks an unwrapped start.

The round's four `claude plugin` commands run at `--scope local` from `target/plugin-live-<stamp>/proj/` and are
removed after it. The residual under `~/.claude/plugins/` is measured as a diff (Q3).

## Top acceptance criteria
- The tree is exactly three files: `marketplace.json`, `plugin.json` and `.mcp.json`.
- The command is absolute, and nothing hooks-related appears anywhere in the tree.
- The tree is 0700 / 0600 under `umask 000`, and a re-run is byte-identical.
- A home at 0770 is refused with exit 21. Human mode prints the home pair; `--json` prints
  `{"v":1,"error":"instance-unreachable","detail":null}`. The home is left as found and no line is written.
- `--json` success prints `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}`, with no path.
- The human result is a planning form (the founder's wording owed), ASCII, with no ESC byte:

  ```
  viola@viola  written  <marketplace dir>
  run these to add it to Claude Code:
    claude plugin marketplace add <marketplace dir>
    claude plugin install viola@viola
  ```

- `viola --help` prints `setup:    run, revive, verify, plugin install`. `help_lists_only_served_verbs` is reshaped
  for a two-word verb.
- `evidence/live-round.md` records the precedence, the one `session-start`, the exit-1 start reaching `idle` or not,
  and the residual diff, with at most 2 starts.

## Touchpoints
- **New:**
  - `crates/viola-agent-claude/src/marketplace.rs` (a pure renderer through `serde_json`, not the
    `@@VIOLA_BIN@@` template);
  - `src/cmd/plugin.rs`;
  - `tests/cli_plugin_install.rs`;
  - `evidence/`.
- **Modify:**
  - `src/cmd/mod.rs` (the subcommand and the help row);
  - `crates/viola-agent-claude/src/lib.rs`;
  - `src/human/verbs.rs` and `src/human.rs`;
  - `src/main.rs` (witness rows);
  - `tests/cli_output_plain.rs`.
- **No ledger, verify, fake-agent, fixture or schema change** (Q2).

## Test commands (21 entries)
- Standing lint, deny and unit entries, all seen green at the last wrap.
- Two new filtered entries, both red at baseline for the predicted reason (the tests do not exist yet):
  - `run --unit --filter 'test(/marketplace_/)'`: 0 selected;
  - `run --integration --filter 'binary(cli_plugin_install) | binary(cli_output_plain)'`: no such binary.
- The layers whole, green at baseline: integration 379/0 and E2E 171/0.
- The booted smoke (cleanup, boot, status, cleanup), green.
- G2 and G4.
- `pre-push`.
- The operator pass: hygiene, push, and `ci.py conclusion --wait 1800`.

The live round is attended (plan step 7), not an entry.

## Mechanical checks and validation-1
- Mechanical checks:
  - `gate.py run --dry-run`: 21 entries parsed.
  - `planlint.py check`: 0 hits.
  - `gate.py scope --lists-only`: parsed, 10 items.
  - `inputs.py verify`: 2 entries, 0 unparsed. Two cross-chunk `inputs#I3` / `inputs#I4` citations in `scope.md`
    were re-worded to those chunks' `inputs/` file paths.
- Every modify path exists, and none is a master.
- The cited sites were read: `resolved_home` `:239`, `canonical_home` `:273`, `check_path`, `client::start`,
  `refused_document`, `home_state_pair`, `create_private_dir`, `replace_private`, `log_self_exit` and `path_fwd`.
- The reused rig (`live-pty.py`, `live-ledger.py` from chunk 2026-10-17-mcp-server-for-drivers) has no host harm in
  that chunk's report.
- **Validation-1: aligned after intent amendments.**
  - `scope.md`'s ledger-row bullet and its `["mcp"]` args were annotated `[intent-amended at P5 val-1]`, citing
    `inputs#I2` Q2 and the plan's `--home` lean.
  - The cap bullet now names fork Q4 and the founder's answers.

## Concretization preview — `verification-matrix.json#v1-42` (claim at P5 step 4)
- **Current acceptance:** "An unwrapped driver installs viola's MCP tools without hooks, calling viola by absolute
  path; a session holding both the installed and the wrapper plugin never runs two hook processes for one event; which
  MCP server wins is measured and recorded."
- **Proposed concrete form:**
  - `viola plugin install` writes `<home>/marketplace/`: a marketplace `viola` whose one plugin `viola` holds only
    `.claude-plugin/plugin.json` and `.mcp.json`.
  - The `.mcp.json` registers server `viola` as `{"command": <canonical absolute path of the exe that ran it>,
    "args": ["--home", <home>, "mcp"]}`, and the tree has no hooks file, directory or key
    (`tests/cli_plugin_install.rs`).
  - On claude 2.1.287 on the Linux dev host, the live round installs it at `--scope local`.
    `evidence/live-round.md` records which `viola` MCP server or servers a wrapped session holding both plugins
    started and which answered its `list` call (the two homes' `mcp.ndjson`), and that the session logged one
    `session-start` (one hook process per event).
  - The Windows leg is owed on "Windows-only live measurements".
  - The method stays `integration`.
- **Never-weaken read:** every clause of the current acceptance is kept and made concrete. If start 1 reads red (no
  precedence reading), the measured-and-recorded clause is unmet, and the cap is un-claimed at the wrap with a dated
  note (the standing un-claim rule).

## Decisive leans (no fork)
- **The command is the canonical `current_exe`** (architecture [Plugin Scope]). It is not the pinned copy.
- **`args` are `["--home", <home>, "mcp"]`.** `mcp` takes the global `--home`. The installed server then never rests
  on the default home or the NOT-ratified `VIOLA_DIR` step.
- **A refused check exits 21 with `instance-unreachable` and writes no line,** as `list` does. In obs-plan §10
  exemption 7 this is the plan's lean, not ruled.
- **`marketplace/` stays off the trusted-file list.** viola never reads it, and adding a file to the list is the
  founder's ruling.

## Beyond the plan
- The Windows legs of the precedence reading and the exit-1 reading are owned by the existing entry "Windows-only live
  measurements". That entry gains them as freight at the wrap.
- The marketplace tree's integrity against a same-user writer (an unknown entry left as found, and no re-hash of the
  installed `command`) is owned by the existing entry "Home and code-bearing file integrity". That entry gains it as
  freight at the wrap.

Apply? (yes / review / cancel)
