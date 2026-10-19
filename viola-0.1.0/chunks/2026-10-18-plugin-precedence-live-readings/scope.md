# Scope — 2026-10-18-plugin-precedence-live-readings

**Working entry (verbatim title + hint):** Plugin precedence live readings — the two plugin readings on claude 2.1.287
(which `viola` MCP server or servers a wrapped session holding both the `--plugin-dir` plugin and the installed
`viola@viola` starts and which answers its `list` call; whether a registered `viola mcp` that exits 1 blocks an
unwrapped start), the method past the folder-trust dialog, and `v1-42`

**Capability named by the entry:** `verification-matrix.json#v1-42` (Optional plugin install for unwrapped drivers;
`planned`, unclaimed at take-up). Its integration half is built and green since chunk 2026-10-17-optional-plugin-install
(`tests/cli_plugin_install.rs`); its acceptance still owes the live reading of which server or servers start and which
answers `list`, and the wrapped session's one `session-start`, recorded on claude 2.1.287 on the Linux dev host; its
Windows leg owed on "Windows-only live measurements" (`matrix.py show --id v1-42`, its acceptance and its two 2026-10-18
notes). Verified at P3: the acceptance names the record's place as `chunks/2026-10-17-optional-plugin-install/evidence/live-round.md`,
so a claim by this chunk concretizes that place to this chunk's `evidence/` (P5). P4 reads the claim lean.

**Take-up direction:** the founder is asleep; this run plans through P4 and raises the forks that are his (the method
past the folder-trust dialog, a live `claude` start cap, any boundary widening) as one dialog, which the Viola overseer
answers «hold for the founder» (`inputs#I1`, the operator's direction). A fork so answered is held, never ratified, and
the plan stops at it.

## What it builds
- **The precedence reading** — on claude 2.1.287 on the Linux dev host, one wrapped start (`viola run`, which loads the
  `--plugin-dir` plugin) from a project where the installed `viola@viola` (`viola plugin install`, then `claude plugin
  marketplace add` + `claude plugin install`) is also enabled: which `viola` MCP server or servers the session starts
  and which answers its `list` call, read from the two homes' `diagnostics/mcp.ndjson`, and that the session logged one
  `session-start` (one hook process per event — the installed plugin carries no hooks). Recorded in this chunk's
  `evidence/`. (The entry; v1-42's acceptance.) Verified at P3 as the equality the attribution needs: the installed
  server is registered with `--home <ihome>` and so logs into `<ihome>`'s `mcp.ndjson`, while the `--plugin-dir`
  server is registered with `args` `["mcp"]` (`plugin/.mcp.json`) and reaches the wrapped home through the `VIOLA_DIR`
  the wrapper hands the child (`src/run/mod.rs:125`–`:126`); both inherit the same `VIOLA_NAME`, so the `instance`
  field cannot tell them apart and the home's file does (research §Measured facts 2).
- **The exit-1 reading** — one unwrapped start from a project where the installed `viola@viola` is enabled and its
  registered `viola mcp` exits 1 (its home at 0770, the home check's refusal): whether the start still reaches a
  usable session (`idle`) or the failing server blocks it. Recorded in `evidence/`. (The entry.) Verified at P3: the
  refusal is `home_passes` (`src/cmd/mcp.rs:22`, `:43`–`:50`) returning exit 1 before any role file opens, so the
  refused home's `mcp.ndjson` gains no line and the start's outcome is read from the CLI side (one `claude agents
  --json` read), never from a viola log.
- **The method past the folder-trust dialog** — a rig whose starts pass the CLI's folder-trust dialog, so that the two
  readings above can be taken at all (the entry). Its form is the founder's word (P4 fork), held under `inputs#I1`.
  Candidates, verified at P3 as the options P4 lists (research §Measured facts 1, 3, 4):
  - a pointer move then Enter typed into the rig's own pty, each only on a read screen — beyond the founder's one Enter
    (`chunks/2026-10-17-optional-plugin-install/inputs/I6-relay-3.md.txt` Q1), which cannot accept the dialog while its
    default row is not the confirm row; on the wrapped start the key is a human keystroke and moves the wheel to the
    human (`src/run/wheel.rs:42`), so the driver's `send` waits on a `viola release` the rig makes;
    [premise-corrected: a digit key has no recorded basis — the recorded 2.1.287 trust row carries no number,
    `fixtures/claude/2.1.287/Screen.modal.json`]
  - a backed-up, read-back trust record for the rig's root written into `~/.claude.json` and removed after;
  - a rig whose project root is already trusted — measured at P3 as the repository root alone (no trusted ancestor
    outside its work tree), so the local scope lands in the repository's own `.claude/settings.local.json`, read by
    every `claude` session started in this tree while it stands (research §Measured facts 3).
  [intent-amended at P4: the founder's own answer to fork 1 (`inputs#I2` F1 «Стрелка + Enter», relayed verbatim by
  the operator) — the rig reads its own pty, types at most 2 arrows toward the trust row and 1 Enter only when that
  row reads selected, never a desktop key, anything else no key and red; `viola release driver` before the wrapped
  send. The widening is his. The other candidates stand as the record of what was weighed.]
- **`v1-42`'s live half** — the precedence reading and the one `session-start` are v1-42's owed acceptance; the exit-1
  reading is the entry's, not the cap's. Verified at P3: the acceptance names no exit-1 reading (`matrix.py show --id v1-42`).

## Folded freight and premises
- CARRY (minted at the 2026-10-18-compact-post-condition-ledger-row wrap from its P5 review card's row 1, approved by the
  operator's own word, `chunks/2026-10-18-compact-post-condition-ledger-row/inputs/I4-relay-3.md.txt`; carrying out the
  founder's own answer to its P4 fork 3 «Отдельный новый вход», relayed verbatim by the operator,
  `chunks/2026-10-18-compact-post-condition-ledger-row/inputs/I3-relay-2.md.txt`, and the operator's wrap direction,
  `chunks/2026-10-18-compact-post-condition-ledger-row/inputs/I7-relay-1.md.txt`): CARRY 2 moves here whole, and that
  chunk made no plugin start (fork 4, «Здесь ноль»). Verified at take-up against those three files: the fork 3 options
  and the founder's words stand as quoted.
- CARRY (chunk 2026-10-17-optional-plugin-install, the Viola overseer's answer at its wrap,
  `chunks/2026-10-17-optional-plugin-install/inputs/I8-relay-1.md.txt` — a delegate's, PROVISIONAL, the founder's own
  word owed at the Epoch 5 boundary batch): its live round on claude 2.1.287 took none of its readings — the precedence
  reading and the exit-1 reading above — nor found the method past the folder-trust dialog. On the working route this
  block follows the first CARRY after ONE space, so `route.py` reads it as no separate block (`INDETERMINATE:
  working-route.md:156 — after ONE space`); it is folded here whole all the same. Verified at P3: `route.py pins` lists
  one `CARRY:` block on line 156 beside the `WATCH:`, and the line holds two.
- Measured-marked, kept verbatim from that CARRY ("measured at that chunk, `evidence/live-round.md`: 2 starts of a
  3-start cap red, start 3 unspent"): "`--scope local` run from a rig under `target/` writes the repository's own
  `.claude/settings.local.json` (the CLI takes the git work tree root), a rig made its own git repo is an untrusted
  project root whose start the trust dialog holds before SessionStart, and that dialog's confirm row is not its selected
  row, so one Enter cannot accept it". Verified at P3 against `chunks/2026-10-17-optional-plugin-install/evidence/live-round.md`
  (rig 1's guard table, `:62`–`:70`; rig 2's start 2, `:216`–`:217`; the CLI version on the host unchanged,
  `~/.local/share/mise/installs/claude/2.1.287/`). That record marks start 1's dialog identity as an inference (no
  screen kept), and the selected row's text was never kept.
- Hypothesis carried in the handoff (the open question): which row 2.1.287's trust dialog selects by default. Partly
  measured at P3 by a second screen model: the 2.1.287 modal fixture recorded by `viola verify` through the vt100
  model (`fixtures/claude/2.1.287/Screen.modal.json`, a start killed with no key; `signature_rows` keeps a literal row
  as rendered, `crates/viola-agent-claude/src/ledger.rs:1420`) holds `   Yes, I trust this folder` with no pointer, as
  the rig's own model read. So the default selection is not the confirm row; WHICH row it is stays unmeasured (only
  literal rows are kept), and so does whether a pointer move reaches the confirm row in one key.
- From that CARRY, verbatim: "a round rebuilds `target/debug/viola` first, since no gate builds it". Verified at P3:
  the harness builds into `target/harness` (`crates/viola-e2e/src/harness/boot.rs:121`, `mod.rs:338`), and the plugin
  round found `target/debug/viola` stale and rebuilt it (`live-round.md:36`–`:38`).
- watch: `run::send::budget::tests::send_budget_paused_is_refused_with_the_window_word_and_no_cursor` case_4 red once
  under a plain `cargo test` (libtest threads; its log capture found no `send-refused` line), green on three re-runs and
  in every nextest gate (0/3; since 2026-10-18-compact-post-condition-ledger-row; retires: a recurrence, or 3 green runs;
  the operator's direction, `chunks/2026-10-18-compact-post-condition-ledger-row/inputs/I7-relay-1.md.txt`). An
  observation, never an obligation: no criterion, task or gate entry. **Its form, for this chunk's wrap to re-author:**
  `route.py pins` reads the pin `clause UNPARSED` — its `since` and `retires` clauses sit inside one parenthesis with no
  k-of-N count; the wrap re-writes it in the watch grammar route-resolve writes (a k-of-N count, a `since {marker}`
  clause and a `retires:` clause, each in its own position), with this chunk's runs counted in (`inputs#I1`).
- Every live `claude` start this chunk makes counts against a cap that is the founder's own word (as at every chunk
  since 2026-10-13; his last plugin-round cap was 3 starts,
  `chunks/2026-10-17-optional-plugin-install/inputs/I6-relay-3.md.txt` Q2), held under `inputs#I1`. Every `claude
  plugin` command count likewise (four per rig at that chunk). Verified at P3 against that relay and `live-round.md:188`.
  [intent-amended at P4: the founder's own answer to fork 2 (`inputs#I2` F2 «3 старта») — at most 3 starts, a fresh
  count of four `claude plugin` commands per rig, a red never re-fired past the cap.]
- [intent-amended at P4] The founder's own word beside his fork answers (`inputs#I2`): the CLI verbs' `VIOLA_DIR` home
  step (every CLI verb, `viola mcp` included) is ratified. Not this chunk's work: an Expected amendment for the wrap
  (plan.md §Implementation notes). [intent-amended at P5: the operator's correction (`inputs#I3`) — his
  «Ратифицировать» answered a question describing the whole step (every CLI verb, `run`, `revive` and `mcp` included:
  the `VIOLA_DIR` home when `VIOLA_NAME` + `VIOLA_DIR` pass the shape check, else the default, then canonicalised), so
  the canonicalisation is ratified too and the wrap lifts both NOT-ratified marks.]

## CI read at take-up (Setup 5a)
- `bc2e3bf` (the last wrap's flip, HEAD) — `ci#39172977857`, verdict not yet available at take-up (in progress,
  2026-10-18T22:40Z); re-read at P3: **green**, checks 15/15, wall 628 s. Nothing to disposition.

## Boundaries
- No new product behaviour is planned by the entry: the readings measure `viola plugin install`'s tree and `viola run`'s
  `--plugin-dir` plugin as built. A reading that falsifies an architecture or security-plan claim lands as a wrap
  amendment, not a code change here, unless P4 finds a fix the readings force. Verified at P3 against the arch extract
  (its fourth acceptance contribution) and research's empty modify list.
- No keystroke into any CLI dialog by `viola verify` (security-plan); the rig's own pty is not verify's. No desktop key
  ever (`inputs/I6-relay-3.md.txt` Q1 of the plugin chunk). Read `hyprctl locked` before any start (host-linux.md
  2026-10-08).
- No write under `~/.claude` by viola; the CLI's own writes (its plugin cache, `installed_plugins.json`,
  `known_marketplaces.json`) are recorded as a before/after diff and removed by the CLI's own uninstall commands, as at
  the plugin chunk. The orphaned cache that chunk left (`~/.claude/plugins/cache/viola/viola/0.1.0/`) stays on the
  founder's desk. Verified at P3 against security-plan §Data Protection (the plugin live round's accepted residual
  class, the security extract) and `live-round.md` §The close.
- No write into the repository root's `.claude/settings.local.json` and no write to `~/.claude.json` without the
  founder's word. Verified at P3: both sit outside the accepted residual class (security-plan §Data Protection), and
  the overseer's close of the plugin round named both (`live-round.md:248`).
- The Windows legs of both readings are owed on "Windows-only live measurements" (its CARRY from the plugin chunk's P5
  card row 1); the `marketplace/` tree's integrity on "Home and code-bearing file integrity". Not this chunk's.
