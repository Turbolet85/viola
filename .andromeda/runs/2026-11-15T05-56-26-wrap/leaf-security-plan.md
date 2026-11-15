@leaf_group group=security-plan
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_read leaf=.claude/rules/security.md lines=44
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=7 class=retired
leaf-text: (`statusline_command`, the plugin exec paths, the pinned binaries, the ledger stamps)
source: dec-o2f2lm2r «statusline_command is trusted only because of the home strict-modes check. · why: Write access to it equals code execution.»
replace-with: (the plugin exec paths, the pinned binaries, the ledger stamps)
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=17 class=retired
leaf-text: plugin files rewritten each start, and the instance's `settings.json` statusline override (0600, only for a pinned path of plain characters);
source: dec-q5t4jgj7 «The plugin files are rewritten whole at every start, a revived start included, with mode 0600 set before the write, never reusing an existing copy.»
replace-with: plugin files rewritten each start (0600, no settings override written);
@leaf_edit n=3 leaf=.claude/docs/security-summary.md line=28 class=retired
leaf-text: `snapshot.json` (`endpoint`, `pinned_bin`, `statusline_command`)
source: dec-qppxx7yu «No process other than the instance's own wrapper writes instances/<name>/snapshot.json. · why: the endpoint in it decides where clients connect»
replace-with: `snapshot.json` (`endpoint`, `pinned_bin`)
@leaf_edit n=4 leaf=.claude/docs/security-summary.md line=67 class=retired
leaf-text: The statusline pass-through (three founder-ratified widenings, live, 2026-10-10): the per-start `settings.json` override passed with `--settings`; the user's command run by `viola hook statusline` as one argument of `/bin/sh -c`, after the instance check (which runs before anything under the home is opened; a refused instance leaves no trace), bounded at 5 s (PROVISIONAL), stdout through `take(MAX_FRAME)`; and the two files that decide that command, `<home>/statusline-source.json` and, for the default home only (`canonical_home(default_home(…))`; a home named by `--home` or reached through `VIOLA_DIR` reads none), `~/.claude/settings.json`, read through `take(MAX_FRAME)`, never written, not on the trusted-file list. Since chunk 2026-10-13-owed-ledger-rows the CLI shapes it relies on are ledger rows (`statusline-override` · `statusline-shell` · `statusline-rate-limits`), relied on every build and stamped on 2.1.287 with the two `StatusLine` fixtures. The statusline hook reads no `rate_limits` and writes no file.
source: dec-o2df2556 «statusline_command is the only shell-out — The recorded command runs only from hook statusline, as /bin/sh -c with the command as one argument, its stdout capped and never parsed, its wait bounded»
replace-with: viola has no statusline pass-through: it reads no status-line source, records and runs no statusline command, writes no `settings.json` override, passes no `--settings` to the child, and runs no shell at all.
@leaf_edit n=5 leaf=.claude/docs/security-summary.md line=72 class=retired
leaf-text: only when the pinned path and the captures dir are shell-plain, exactly one probe `--settings <probe dir>/typed/statusline.json` after its `--plugin-dir` pair (0600, written by `replace_private`) whose status line is the hidden `hook statusline --capture <probe dir>/typed/statusline --shell "$0"`, no run or start added (the founder's own answer to P4 fork 3 of chunk 2026-10-13-owed-ledger-rows), once its first turn
source: dec-pbk4onnw «Statusline capture arm — The hidden statusline capture arm writes the payload raw to a claimed 0600 file and a shell verdict of exactly bin-sh when the --shell word equals /bin/sh byte for byte»
replace-with: once its first turn
@leaf_edit n=6 leaf=.claude/docs/security-summary.md line=72 class=retired
leaf-text: with the user's own status line in Runs C and D (Run B's status line is the probe capture its one probe `--settings` names)
source: dec-pbk4onnw «Statusline capture arm — The hidden statusline capture arm writes the payload raw to a claimed 0600 file and a shell verdict of exactly bin-sh when the --shell word equals /bin/sh byte for byte»
replace-with: with the user's own status line in Runs C and D
@leaf_edit n=7 leaf=.claude/docs/security-summary.md line=72 class=retired
leaf-text: an unclean set is refused whole; a `workspace.repo` naming this repository's own remote is kept in a fixture, its `workspace.repo.owner` neither rewritten by the user-word scrub nor refused (the founder's own word, chunk 2026-10-13-owed-ledger-rows).
source: dec-nvdmm3x2 «Before any write, verify --record scrubs the home prefix to ~ and the user word to <user>, and refuses the whole recording with exit 1 and nothing written when a home or absolute path, the user word or»
replace-with: an unclean set is refused whole.
@leaf_edit n=8 leaf=.claude/rules/security.md line=12 class=stale
leaf-text: the two `--capture` arms stay ahead of the check.
source: dec-ppuyroe4 «except: The capture arm runs ahead of that check.»
replace-with: the `--capture` arm stays ahead of the check.
@leaf_edit n=9 leaf=.claude/rules/security.md line=13 class=retired
leaf-text: exit 0 always; its statusline form `hook statusline --capture <DIR> [--shell <WORD>]` (Run B's probe settings only; the founder's own answer to P4 fork 3 of chunk 2026-10-13-owed-ledger-rows) holds the same bounds, writes the payload raw and only the closed `ShellVerdict` (`bin-sh` · `other`, never the word), and prints nothing.
source: dec-pbk4onnw «Statusline capture arm — The hidden statusline capture arm writes the payload raw to a claimed 0600 file and a shell verdict of exactly bin-sh when the --shell word equals /bin/sh byte for byte»
replace-with: exit 0 always.
@leaf_edit n=10 leaf=.claude/rules/security.md line=14 class=stale
leaf-text: at every entry point that reads `snapshot.json`, `ledger/stamps.json` or `statusline_command`,
source: dec-nuhvvyx3 «Every process that reads snapshot.json or ledger/stamps.json refuses when the home, its trusted folders or a trusted file is group- or world-writable, not owned by euid, or a symbolic link»
replace-with: at every entry point that reads `snapshot.json` or `ledger/stamps.json`,
@leaf_edit n=11 leaf=.claude/rules/security.md line=15 class=stale
leaf-text: Every exec-form `command` in `hooks.json`, `.mcp.json` and the `settings.json` statusline is the absolute pinned path
source: dec-qprwcjeq «Every exec-form command in the embedded hooks.json or .mcp.json and in plugin install's .mcp.json names viola by the absolute pinned path or the installing exe's absolute path, never through PATH.»
replace-with: Every exec-form `command` in `hooks.json` and `.mcp.json` is the absolute pinned path
@leaf_edit n=12 leaf=.claude/rules/security.md line=15 class=retired
leaf-text: Rewrite `plugin/` atomically on every start, and the instance's `settings.json` statusline override (`replace_private`, 0600, only for a pinned path of ASCII letters, digits and `_ - . / :`; passed as `--settings <instance dir>/settings.json` after `--plugin-dir`, a founder-ratified widening, live, 2026-10-10); re-hash
source: dec-q5t4jgj7 «The plugin files are rewritten whole at every start, a revived start included, with mode 0600 set before the write, never reusing an existing copy.»
replace-with: Rewrite `plugin/` atomically on every start (0600 set before the write; no `settings.json` override is written and no `--settings` passed); re-hash
@leaf_edit n=13 leaf=.claude/rules/security.md line=17 class=retired
leaf-text: Run B gets exactly one probe `--settings <probe dir>/typed/statusline.json` (0600, `replace_private`, only when the pinned path and the captures dir are shell-plain; no run or start added; the founder's own answer to P4 fork 3 of chunk 2026-10-13-owed-ledger-rows) whose status line is the hidden `hook statusline --capture` arm; once its first turn has settled
source: dec-pbk4onnw «Statusline capture arm — The hidden statusline capture arm writes the payload raw to a claimed 0600 file and a shell verdict of exactly bin-sh when the --shell word equals /bin/sh byte for byte»
replace-with: once Run B's first turn has settled
@leaf_edit n=14 leaf=.claude/rules/security.md line=17 class=retired
leaf-text: an unclean set is refused whole; a `workspace.repo` naming this repository's own remote is kept in a fixture, its `workspace.repo.owner` neither rewritten by the user-word scrub nor refused (the founder's own word, chunk 2026-10-13-owed-ledger-rows).
source: dec-nvdmm3x2 «Before any write, verify --record scrubs the home prefix to ~ and the user word to <user>, and refuses the whole recording with exit 1 and nothing written when a home or absolute path, the user word or»
replace-with: an unclean set is refused whole.
@leaf_edit n=15 leaf=.claude/rules/security.md line=27 class=retired
leaf-text: Statusline (two founder-ratified widenings, live, 2026-10-10): `run` reads the user's command at start from `<home>/statusline-source.json`, else for the default home only (`canonical_home(default_home(…))`; a home named by `--home` or reached through `VIOLA_DIR` reads none) `~/.claude/settings.json` (`take(MAX_FRAME)`; a `statusLine` of type `command`, non-empty, no NUL), writes neither file, and records the command only in `snapshot.json`; neither source is on the trusted-file list. `viola hook statusline` without `--capture` (the hidden capture form is taken first and is `viola verify`'s probe capture alone) canonicalises its instance directory and runs `check_instance` BEFORE anything under the home is read or opened (a check made after the log passes a home at 0770); a refused instance returns exit 0 at once with no `config.json` read, no role file opened, no line, no stdin read and no command run. A passed check then runs the command as one argument of `/bin/sh -c` (stdout through `take(MAX_FRAME)`, stderr discarded, a 5 s PROVISIONAL bound, the command in no line or span). It stays the only shell-out. A malformed or oversize payload ends in exit 0, empty stderr and nothing printed; the hook reads no `rate_limits` and writes no file.
source: dec-o2df2556 «statusline_command is the only shell-out — The recorded command runs only from hook statusline, as /bin/sh -c with the command as one argument, its stdout capped and never parsed, its wait bounded»
replace-with: Status line: viola reads no status-line source, records and runs no statusline command, has no `hook statusline` arm and runs no shell at all.
