# Outcome — 2026-10-17-optional-plugin-install · implement

## SURFACED — the code is complete and every local gate is green; the live round's two readings are still unread

- **Gates:** entries 1-18 green (trail `.andromeda/runs/2026-10-17T18-08-07-implement/gate-2026-10-17-optional-plugin-install.json`):
  1-10 green on the first firing, 11 red then green after one fix, 12-18 green (pre-push 87.62 s). Entries 19-21 are
  the operator pass's, not fired (the pass waits for the round, `inputs#I5` Q2). No deferral.
- **Smoke:** ✓. The booted pair ran as P2 entries 12-17. The new verb ran on the rebuilt `target/debug/viola` in both
  rigs: exit 0, the `ok` document.
- **Rig 1:** the plan's rig premise was falsified. Inside this work tree, the local scope landed at the repository
  root. The guard stopped the round with no start.
- **Rig 2** (`inputs#I5`: `proj/` its own git repo, a fresh count of four commands): the guard passed, and the plugin
  was installed at `proj/`'s local scope (commands 1-2).
- **Start 1:** red. A modal held it, the rig typed no key, and it is not re-fired.
- **Start 2** (`inputs#I6`: one Enter on a read trust dialog, the cap raised to 3; rig `live-pty-trust.py`): red. The
  rig read the trust dialog's confirm row on screen but not as the selected row (`unselected`), so it typed no key.
  No `session-start` was logged, and no `viola mcp` started from either plugin. Not re-fired.
- **The round closed as it stands** (`inputs#I7`, the Viola overseer; the founder asleep, his forks PROVISIONAL):
  - start 3 unspent: 2 of 3 starts made;
  - commands 3-4 (the uninstall, the marketplace removal) ran from rig 2's `proj/` at 02:22:00Z / 02:22:01Z, both
    exit 0 `ok`;
  - no key typed, no `~/.claude.json` write, no root local scope.
- **The residual:** the CLI's orphaned plugin cache, `~/.claude/plugins/cache/viola/viola/0.1.0/` (3 files, its
  `.orphaned_at` stamped at the uninstall), and an unattributed `synced/` pair. `installed_plugins.json` and
  `known_marketplaces.json` are back to their before bytes, and `~/.claude/settings.json` is unchanged.
- **The open fact, for the founder:** which row 2.1.287's trust dialog selects by default. If it is `No, exit`, one
  Enter declines trust, and the one-key form cannot accept it.
- Rig 1's residual (the repository root's `.claude/settings.local.json`) was removed at 2026-10-18T01:44:31Z (the
  founder's Q3).
- **Unmet:** the `(manual, live)` criterion and v1-42's live half. **v1-42 stays `planned`, `ref` null**; its
  integration half is green. Record: `evidence/live-round.md`, `evidence/live-sessions.ndjson`.
- **The operator pass:** done. pre-push green, hygiene clean, pre-CI commit `43c64109`, push green; CI green 15/15 on that sha (`ci#39090008812`, attempt 1, wall 586 s). Record: `evidence/operator-pass.md`.
- **Next:** `/andromeda-wrap-session`. The outcome stays surfaced, and v1-42 stays planned (`inputs#I7`).

## Files changed
- New: `crates/viola-agent-claude/src/marketplace.rs`, `src/cmd/plugin.rs`, `tests/cli_plugin_install.rs`;
  `chunks/2026-10-17-optional-plugin-install/evidence/` (`live-pty.py`, `live-ledger.py` byte-equal copies,
  `live-sessions.ndjson`, `live-round.md`, `guard-controls.md`).
- Modified: `src/cmd/mod.rs`, `crates/viola-agent-claude/src/lib.rs`, `src/human/verbs.rs`, `src/human.rs`, `src/main.rs`,
  `tests/cli_output_plain.rs`.
- Out of list (scope record): `` `.github/workflows/windows-mutants.yml` · companion · serves `src/human/verbs.rs` · self ``.

## Deviations from the plan, with justifications
1. **The `.mcp.json` home is re-canonicalised after the tree's dirs are made** (`src/cmd/plugin.rs` `write_tree`). The plan
   names "the resolved canonical home", but `resolved_home` canonicalises before a new home exists. Through the linked
   `target/e2e-home` that spelling is the link's, so it misses the `resolved(home)` oracle. Re-reading it once the home
   exists is `run`'s own pattern (`src/cmd/run.rs:232-234`). The printed marketplace dir uses the same spelling.
2. **`src/human/verbs.rs` is a Windows-gated source.** `command_word` decides the shell family by `cfg!(windows)`
   (`shell_word(path, windows)` is pure and unit-tested for both families on every host).
   `contract_windows_mutation_scope` then required the file in `windows-mutants.yml`. It was added to the existing
   `viola-main` job beside `src/human/style.rs`, with no new job and no label change (the scope record's companion
   line). The wrap may owe a test-plan §10 mention.
3. **The human lines are built with `concat!`, one string per line** (testing.md 2026-10-17), and the escape-byte case
   removes `NO_COLOR` / `TERM` / `COLORTERM` before setting its own (testing.md 2026-10-17). The planning form's bytes
   are as the plan states.
4. **The tree's directories are made in parent-first order** (home, `marketplace/`, `marketplace/.claude-plugin/`,
   `marketplace/viola/`, `marketplace/viola/.claude-plugin/`), each through `create_private_dir`. An existing one is
   narrowed to 0700 only after it passed the strict check.
5. **Units beyond step 4:** `tree_dirs_are_the_home_and_the_four_marketplace_dirs`,
   `tree_passes_a_new_home_and_its_own_tree`, `tree_passes_refuses_any_dir_another_user_could_write`,
   `fwd_writes_every_backslash_as_a_slash` (`src/cmd/plugin.rs`), and
   `write_plugin_installed_is_the_planning_form_in_one_write`, `shell_word_quotes_only_what_the_shell_would_split`,
   `command_word_takes_this_host_family` (`src/human/verbs.rs`), all in listed files.
6. **`target/debug/viola` was stale** (16:15 local, the earlier chunk's build; no gate rebuilds it), so it was rebuilt
   by `cargo build --bin viola` before the round (sha256 prefix `ee27584342cef430`).
7. **The rig helpers** (the `~/.claude/plugins/` listing and the census) are scratchpad scripts, because the write
   guard refuses a write under `target/`. The listings themselves sit in the rig's `priv/`.
8. **Noted, not changed:** clap gives `viola plugin` its own `help` sub-verb (`viola plugin help install`), as the top
   level has `viola help`. The plan's "no other sub-verb" reads served verbs. The output is ASCII only for an ASCII
   home path. A non-ASCII home prints as it is spelt (quoted on Unix), so the two commands stay pasteable.
9. **Remove-the-guard control** (testing.md 2026-09-25), on the strict check at `src/cmd/plugin.rs:52`: neutralised,
   `plugin_install_refuses_a_widened_home` read red (`left: Some(0)`, `right: Some(21)`); restored, 9/9 green
   (`evidence/guard-controls.md`).

## Process hygiene
| process | started by | final state |
|---|---|---|
| the booted smoke pair (`p-plugin-smoke`: supervise, wrapper, fake agent) | a P2 gate entry (13) | terminated — entry 17 read `processes_gone:true`, `endpoint_gone:true` |
| the 3 `claude plugin` commands (2.1.287) | this run | terminated — each exited (0, 1, 0) |
| `target/debug/viola … plugin install` (rig) | this run | terminated — exit 0 |
| census at 2026-10-17T18:24:15Z (exe under the repository's `target/`, or cwd under the rig or home dirs) | — | 0 processes; no `viola-harness`, `supervise` or `viola-fake-agent` in `ps -eo pid,comm,args` |
| rig 2: the 2 `claude plugin` commands, the install verb | this run | terminated — each exited 0 |
| rig 2 start 1: `live-pty.py` → `viola run driver` → claude 2.1.287 | this run | terminated — the stop file, 3 presses, wrapper exit 0, child exit 1; census at 2026-10-18T01:47:27Z: 0 processes |
| rig 2 start 2: `live-pty-trust.py` → `viola run driver` → claude 2.1.287 | this run | terminated — the stop file, 3 presses, wrapper exit 0, child exit 1; census at 2026-10-18T02:19:31Z: 0 processes |

## Decisions & corrections
- The overseer's direction (`inputs#I7`): close the round as it stands, start 3 unspent, commands 3-4 now; no key, no `~/.claude.json` write, no root local scope; then the operator pass; surfaced, v1-42 planned. The orphaned cache is recorded and left as found.
- The founder's own answers (`inputs#I6`): one Enter into the rig's own pty on a read trust dialog, never a desktop key; the cap raised to 3; commands 3-4 after the last start. The rig was extended minimally as `evidence/live-pty-trust.py`; `live-pty.py` itself is unchanged.
- `hyprctl locked` read `true` before start 2. The start went ahead: the standing rule stops a step that focuses a window or types a key with `wtype`, and this start does neither.
- Start 3 held, my judgement brought to the founder: the same dialog would meet it with no key, so it would be spent with no reading.
- The founder's own answers, relayed (`inputs#I5`, 2026-10-18T01:42Z): Q1 `proj/` its own git repo with a fresh count of four commands; Q3 remove the residual now (done, 01:44:31Z). Q2 is the overseer's: the operator pass waits for the round.
- Start 2 held, my judgement brought to the founder: the same modal would meet it from the same `proj/`, so the start would be spent with no reading. The plugin stays installed at `proj/`'s local scope only.
- The founder's cap, relayed in the invocation (`inputs#I4`): two starts and the four commands at `--scope local`, each
  start ledgered first, a red never re-fired, a blocked start 2 recorded with a stop before any code change, no
  mutation run, then the operator pass. The round stopped earlier, at the (a) guard, so no start was ledgered.
- Guard refusals: the Bash guard refused a `cd` that moved the session cwd (re-issued as Read calls); the write guard
  refused two writes under `target/` (re-issued into the scratchpad). One re-issue each.
- Standing learnings met: testing.md 2026-10-17 (`concat!` literals; the colour facts removed per child);
  testing.md 2026-09-25 (the remove-the-guard run); host-linux.md (no `cd` that moves the cwd).
