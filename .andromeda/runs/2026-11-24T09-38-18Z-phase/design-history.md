@history_item n=1 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — landed `revive`'s exit-1 refusals as fixed `unable:` / `hint:` pairs read in a fixed order that stops at the first (state files, `run`'s already-live / stale pairs, `no logged session`, `no logged session with that id`, `recorded directory is missing`), none showing the recorded directory, a pid or a logged id and no hint naming `viola release`; `revive` prints nothing once the child starts; put the verb in the `setup:` help group. `adopt` is built on `revive`'s start, so its refusals, silence once the child runs and help placement extend this precedent (rows contract/cli-start-refusal/fixed-message, decision/start-refusals-show-no-path-or-pid, contract/viola-revive-refusals/order, contract/viola-run-output/child-started).

@history_item n=2 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added the live-elsewhere pair `unable: <name>'s session is open in another claude` / `hint: close it there first, or start a copy: viola revive <name> --fork` (founder's wording), read from one `claude agents --json` read after the logged session and before the recorded directory, skipped under `--fork`, a failed read passing; built the unwrapped rack (NAME = `sessionId`, LIVE `live`, rows sorted by name, STATUS by agents join). This is exactly the guard `adopt` reuses (contract/viola-revive-refusals/session-live, contract/viola-list-board/status, decision/bare-strips-never-target): a hint naming `revive <name> --fork` or "close it there" must be re-read for an adopt, and the "failed read passes" lean is a trap for a verb whose whole guarantee is never taking over a live session.

@history_item n=3 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — gave `revive --json` its exit-1 documents `{"v":1,"error":"<code>","detail":null}`, no hint, stderr empty, and `--list --json`; every other exit 1 had no document. Precedent for whether `adopt --json` prints a refusal document and which closed codes it carries (contract/viola-revive-output/json-refusal, decision/json-one-document-on-stdout, decision/hints-human-mode-only).

@history_item n=4 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (run's and revive's start refusals at the home-tree check; the six frame verbs' instance pair; a home under a regular file) — `run`'s start checks the home tree before any log (home pair at home level, instance pair at instance level, exit 1, no child); `revive`'s start does the same after a passed preflight, `strict-modes-failed` under `--json`; frame verbs print the instance-state pair at exit 21. An adopt start sharing `run`'s start inherits this check and its order relative to the preflight (contract/viola-run-start-refusals/home-tree-check, contract/viola-revive-refusals/home-tree-check, contract/fixed-refusal-pairs/home-state, contract/fixed-refusal-pairs/instance-state, contract/strict-modes/home-under-regular-file).

@history_item n=5 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`revive --json`'s documents and codes; the catch site's `internal-error`; the log's codes; a stale holder's own code) — one exit-1 document for every refusal of the start `revive` shares with `run`, closed codes incl. `session-live`, `no-session`, `cwd-missing`, `stale-heartbeat`, `squatted-name`, `pinned-hash-mismatch`, `internal-error`; `run` itself prints none; a `pump_child` error prints no document (child held the terminal); `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site; D-20 log codes `no-session` / `cwd-missing` for revive; `revive`'s strict-modes refusal writes no log line. A new start verb must decide its own code set, document and catch-site silence against these rows (contract/viola-revive-json-codes/*, decision/silent-catch-sites, contract/cli-catch-site/*, decision/no-json-after-child-held-terminal).

@history_item n=6 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory pair at run and revive's start; `revive --json`'s eleventh code; `socket-dir-refused` as exit 21's sixth cause and a log code; the board's rule on the home) — `run`'s start refuses an untrusted socket directory before any snapshot, child or bind with a fixed pair, `revive` reuses it, `revive --json` reads `socket-dir-refused` for a failed create and a refused check alike; the board holds the home to the read rule and refuses with the home pair. Row 1's character budget forced folding `revive`'s shared pairs — a trap when adding another start verb's lines to the exit-1 row (contract/viola-run-start-refusals/socket-dir, contract/viola-revive-refusals/socket-dir, contract/viola-revive-json-codes/socket-dir-refused, contract/exit-21-cause/socket-dir-refused, contract/viola-list-board/strict-modes-scan).

@history_item n=7 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the socket-directory pair's wording confirmed) — the founder confirmed the socket-directory pair's wording, retiring its OPEN planning-form mark; the pair `adopt` would inherit stands as his word (contract/refusal-hints/socket-dir).

@history_item n=8 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the .cmd/.bat refusal and batch-script-child retired; strict-modes readings without an OS arm) — `revive --json`'s closed codes are ten (`batch-script-child` retired), pinned in `tests/cross_exit_causes.rs`; strict-modes readings carry no OS arm. Any code set `adopt` reuses or extends starts from these ten and that test (contract/revive-json-refusal-code/*).

@history_item n=9 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the `--json` failures the witness drives; exit 2's fixed stderr lines; the no-hint fault line) — every CLI `--json` failure driven by `tests/cli_error_bodies.rs` prints one document, exits its literal code, stderr empty, no ESC, path or upstream text; faults print no hint; exit-2 fixed lines carry no document. A new verb's failures join this guard (decision/json-failure-documents-clean, decision/no-hint-for-faults, decision/no-internals-in-errors, decision/json-one-document-on-stdout).

@history_item n=10 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — added the five fixed automatic-verify stderr lines a `run` / `revive` start prints (verifying, did not verify, earlier red, verifying elsewhere, no stamp), one ASCII stderr write, no SGR, none under `--json`; ratified by the founder at 2026-11-22T17-29-41Z-wrap. An adopt start built on revive's start meets these lines and must say whether it prints them (contract/auto-verify-line/*).

@history_item n=11 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (the delegate's confirm ratified — 2026-11-20-a-new-cli-version-stamped-automatically) — the auto-verify-line rows stand on the founder's own word, no longer PROVISIONAL; changing them for `adopt` is a change to founder-ratified wording.

@history_item n=12 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** — exit 2 gained the fixed `PAIR_AFTER_DASHES` line for a pair after `--` of `run` or `revive`, no document whatever `--json` says. Bears on how `adopt` takes model / effort and argv after `--` (scope: the adopted session's model and effort source) (contract/cli-exit-codes/2).

@history_item n=13 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — the latest verb-addition precedent: `open` added to the `setup:` help row (both help rows), to the exit-1 row (`--json` documents for its refusals), to both fixed-`unable:`-form lists, and as `name/command/open`. `adopt` will edit the same closed lists (contract/help-output/verb-groups, contract/viola-help/verb-groups, contract/cli-exit-codes/1, contract/cli-refusal-line/fixed-unable-form, contract/cli-stderr/fixed-message-refusal, name/command/*).

@history_item n=14 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — `focus` added to the `board:` help row, both fixed-form lists (exit 1 and exit 21), the exit-1 row and `exit-21-json-cause/strict-modes-failed`, with its own closed exit-1 code rows; all on the delegate's word, PROVISIONAL. The same twin rows (help-output vs viola-help, cli-refusal-line vs cli-stderr) must each be amended; they were edited in separate passes here, a trap for missing one twin.

@history_item n=15 marker=2026-11-24-viola-up
**2026-11-24-viola-up** — `up` added to `setup:` between `revive` and `verify`, to the exit-1 fixed-form lists and `strict-modes-failed`; `decision/list-is-the-board` reworded for a verb acting on many strips; closed outcome words incl. `already-live` and `session-live` (the latter makes `up` exit 1), the not-live hint, the boot-time-unread pair; PROVISIONAL on the delegate. Nearest sibling start verb built on revive's start: `adopt`'s placement in `setup:`, its scope under list-is-the-board, and reuse of the session-live word follow it (contract/viola-up-outcome/already-live, contract/viola-up-outcome/session-live, decision/list-is-the-board).

@history_item n=16 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — the board draws wrapped rows as a tree in name order, each child under its parent, the indent inside NAME (width counted with it), links printed apart in a `-- LINKS --` block, six columns fixed. An adopted session with `--parent` lands in this tree (contract/viola-list-table/row-order, decision/stable-session-slots).

@history_item n=17 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — `--help` lists the served verbs only, grouped; the colour layer, NO_COLOR read, width from `host_size()`; ratified by the founder at 2026-10-20-t06-58-31-wrap. A new verb joins `--help` only once served (contract/viola-help/verb-groups, contract/cli-colour-decision-order/*, decision/no-hardcoded-widths-or-wrapping).

@history_item n=18 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `mcp` is a hidden verb, run by the plugin, never in `--help`. Bears on the scope's fork whether `adopt` is an MCP tool (conservative arm CLI-only) and on the help listing (contract/cli-callers/llm-driver).

@history_item n=19 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — `list`'s exit-21 whole-board refusal and its fixed home pair (first writer `list`), the board's row escaping of every control character; ratified at 2026-10-13-t06-38-35-wrap. The home pair and the escaping rule an adopted row and an adopt refusal reuse (contract/fixed-refusal-pairs/home-state, contract/viola-list-board/row-escaping, decision/list-rows-escape-all-controls).

@history_item n=20 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`link` / `unlink`'s exit-21 strict-modes refusal takes the fixed instance-state pair) — extended the closed fixed-`unable:`-form lists and row 21 to new verbs; noted `plugin install`'s home pair left out of those lists as a standing gap. Precedent and a known gap when `adopt` edits the same lists (contract/cli-strict-modes-refusal/plugin-install, contract/strict-modes-refusal/exit-21).

@history_item n=21 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — trimmed the fixed vocabulary and the reason no hint names `viola release` to the human-typing refusal alone. Constrains `adopt`'s hint wording (decision/fixed-phraseology-vocabulary, decision/no-hint-names-viola-release, decision/no-hint-names-release).

@history_item n=22 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0 — brand, surface and bans scoped to the CLI) — no interactive prompt, TUI mode or pager scoped to a verb's output; the spinner ban scoped to verb output. `adopt` takes no confirmation prompt (decision/no-interactive-prompt-or-pager, decision/no-spinners-in-verb-output).
