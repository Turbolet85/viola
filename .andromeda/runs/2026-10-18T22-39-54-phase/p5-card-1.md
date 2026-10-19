# P5 review card 1 — 2026-10-18-plugin-precedence-live-readings

Plan: `viola-0.1.0/chunks/2026-10-18-plugin-precedence-live-readings/plan.md` (364 lines, 12 gate entries,
7 acceptance criteria). Scope: `scope.md` (amended at P4 with the founder's answers). Research: `research.md`.

## Goal
Take the two plugin readings the 2026-10-17-optional-plugin-install round could not take, on claude 2.1.287 on the
Linux dev host, inside the founder's cap: which `viola` MCP server or servers a wrapped session holding both plugins
starts and which answers `list` (with its one `session-start`), and whether a registered `viola mcp` that exits 1
(home at 0770) blocks an unwrapped start. The rig passes the folder-trust dialog by his «Стрелка + Enter» and runs
under his «3 старта» (`inputs#I2`). No product code changes.

## Top acceptance criteria
- `rig-controls.py` prints `controls: 10 of 10 as expected` (the key rule's ten synthetic controls).
- `evidence/live-round.md` records, within 3 starts and four `claude plugin` commands:
  - per home, `process-start{subject:"self"}` counts in `mcp.ndjson`, and the home whose file gained the `list`
    call's `agents-probe` pair;
  - one `session-start` and one `hook-invoked{hook_event:"session-start"}`;
  - start 2's agents-row `status` with the refused home still 0770 and its log not grown;
  - the keys per start (arrows ≤ 2, Enter ≤ 1, the default row's offset), the wheel record and the release;
  - the residual (`~/.claude/plugins/` diff, the CLI's trust entry for `proj/` in `~/.claude.json`, the transcript count).
- No `CLAUDE*` value, absolute path, prompt or assistant text, session id or `detail-*.ndjson` line under `evidence/`.
- `binary(cli_plugin_install)` green unchanged; CI green on the operator pass's final HEAD.

## Touchpoints
- New: `viola-0.1.0/chunks/2026-10-18-plugin-precedence-live-readings/evidence/` (the copied `live-pty.py` and
  `live-ledger.py`, the new `live-pty-arrow.py` and `rig-controls.py`, the start ledger, `live-round.md`).
- Modified: none. Graph impact read, nothing changed: `home_passes` @ `src/cmd/mcp.rs:22`, `board_document` @
  `src/cmd/mcp.rs:35`, `marketplace_files` @ `src/cmd/plugin.rs:114`.

## Test commands (12 entries)
| # | role | entry | baseline |
|---|---|---|---|
| 1 | lint | `cargo fmt --all --check` | seen green at the last wrap |
| 2 | unit | `python3 …/evidence/rig-controls.py` | red — the script does not exist yet (step 1 writes it) |
| 3 | build | `cargo build --bin viola` | green — 0.13 s |
| 4 | integration | `run --integration --filter 'binary(cli_plugin_install)'` | green — 9 passed |
| 5–9 | lint · unit · integration · e2e · probe | clippy · `run --unit` · `run --integration` · `run --e2e` · `pre-push` | deferred (zero Rust delta); 7 and 8 baselined green with the defer voided (389 and 172 passed) |
| 10–12 | operator | hygiene · clean-tree push · `ci.py conclusion --sha HEAD --wait 1800` | operator pass |

The live round (steps 3–10) is attended, not an entry; the wrap re-verifies it from `evidence/`.

## Concretization preview — `verification-matrix.json#v1-42` (claim by this chunk)
- **Current acceptance:** "`viola plugin install` writes `<home>/marketplace/`: a marketplace `viola` whose one plugin
  `viola` holds only `.claude-plugin/plugin.json` and `.mcp.json`, the `.mcp.json` registering server `viola` as
  `{"command": <the canonical absolute path of the exe that ran it, forward slashes>, "args": ["--home", <home>,
  "mcp"]}`, with no hooks file, directory or key anywhere in the tree (`tests/cli_plugin_install.rs`); on claude 2.1.287
  on the Linux dev host the live round installs it at `--scope local` and
  `chunks/2026-10-17-optional-plugin-install/evidence/live-round.md` records which `viola` MCP server or servers a
  wrapped session holding both plugins started and which answered its `list` call (read from the two homes'
  `mcp.ndjson`), and that the session logged one `session-start` (one hook process per event); its Windows leg owed on
  "Windows-only live measurements"."
- **Proposed acceptance:** the integration half unchanged, word for word, up to "(`tests/cli_plugin_install.rs`);".
  Then: "on claude 2.1.287 on the Linux dev host the live round installs it at `--scope local` from a rig whose
  `proj/` is its own git repo, and `chunks/2026-10-18-plugin-precedence-live-readings/evidence/live-round.md` records
  which `viola` MCP server or servers a wrapped session holding both plugins started — per home, the count of
  `process-start{subject:"self"}` in `diagnostics/mcp.ndjson` — and which answered its `list` call — the home whose
  `mcp.ndjson` gained the `agents-probe` pair — and that the session logged one `session-start` in `events.ndjson` and
  one `hook-invoked{hook_event:"session-start"}` in `hook-<name>.ndjson` (one hook process per event); its Windows leg
  owed on "Windows-only live measurements"."
- **What moves:** the record's place (the earlier chunk's round took no reading), and the two readings stated as the
  counts that decide them (research Measured facts 2: the file attributes a line, `instance` does not). Nothing is
  dropped and no outcome is weakened. Method `integration` kept.

## Expected amendments the plan carries (the wrap's, not this card's)
- architecture [Plugin Scope] and [Deployment / Distribution] — the two owed sentences, to the measured readings.
- security-plan §Data Protection (`security-plan.md:262`) — the rig's trust keys (his own answer to fork 1, `inputs#I2`
  F1: the wrap's answer card) and the measured residual.
- security-plan §Input Validation and architecture Config management — the `VIOLA_DIR` home step ratified by his own
  word (`inputs#I2`); the canonicalisation stays NOT ratified.

## Beyond the plan
none
