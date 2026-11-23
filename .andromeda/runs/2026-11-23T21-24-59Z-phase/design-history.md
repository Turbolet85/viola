@history_item n=1 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** — Surface: cli, `~ contract/cli-exit-codes/2`: exit 2 gained the fixed `PAIR_AFTER_DASHES` line for a pair after `--` of `run` / `revive`, on stderr whatever `--json` says, no document. `open` adds one more fixed exit-2 line beside it, for a word after `--` other than `claude`, and takes `--model` / `--effort` the way that chunk's launch pair does. Row 2's closed list of named fixed lines has to grow again, the same way.

@history_item n=2 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (exit 2's fixed stderr lines, no `--json` document; the `--json` failures the witness drives) — exit 2's fixed lines print on stderr with no document under `--json`, as an exception to the one-stdout-document rule (`decision/json-one-document-on-stdout`). It also set up `tests/cli_error_bodies.rs`: each `--json` failure is one document, stderr empty, with no ESC, no path and no upstream text (`decision/json-failure-documents-clean`). `open`'s new exit-2 line belongs under that exception, and its `--json` error documents (`terminal-not-found`, `not-live`) fall under that witness.

@history_item n=3 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — the precedent for adding a setup verb. `revive` got fixed-message `unable:` / `hint:` pairs read in a fixed order that stops at the first refusal, with no path, pid or recorded directory (`decision/start-refusals-show-no-path-or-pid`), and no hint names `viola release`. It reused `run`'s already-live pair unchanged and joined `viola --help` in the `setup:` group right after `run`. `open`'s help row `setup:  open, run, revive, verify, plugin install` and its reuse of `already-live` / `stale-heartbeat` follow this pattern.

@history_item n=4 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — gave `revive --json` its exit-1 document `{"v":1,"error":"<code>","detail":null}`, with no hint and stderr empty, plus an `ok` document. It is the shape `open --json` copies for `{"v":1,"error":"<code>","detail":null}` and `{"v":1,"ok":{…}}`.

@history_item n=5 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`revive --json`'s documents and its codes; the catch site's `internal-error` document; a stale holder's own code; the D-20 log codes) — four things from this entry bear on `open`. (1) `revive --json`'s closed exit-1 code set: `open` adds `terminal-not-found` / `not-live` as codes of its own. (2) The catch site prints the `internal-error` document under `--json`, counts `--json` only before `--`, and stays silent for `run`, `revive`'s start, `mcp` and `hook`; `open` is not on that silent list. (3) A stale holder prints and logs `stale-heartbeat`, a live one `already-live`; `open` reads both before any spawn. (4) The D-20 log detail codes are where `open`'s codes-only log lines belong.

@history_item n=6 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (run's and revive's start refusals at the home-tree check) — the start checks the home tree before any log and with no child: a home-level refusal prints the home pair, an instance-level one the instance pair, exit 1, and `revive --json` prints `strict-modes-failed`. `open`'s exit 1 `strict-modes-failed` before any log follows this precedent and these fixed pairs.

@history_item n=7 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory pair at the start of run and revive; `revive --json`'s eleventh code) — added `run`'s and `revive`'s shared-start refusal `unable: the socket directory for <name> cannot be trusted` and the code `socket-dir-refused`. This was the latest growth of the closed exit-1 start refusal set that `open` checks before it spawns, or inherits from the `run` it starts.

@history_item n=8 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the socket-directory pair's wording confirmed) — the founder confirmed the socket-directory pair's wording and its OPEN mark was retired. The start-refusal wordings `open` reuses or sits beside are now settled, not planning forms.

@history_item n=9 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the cli Platform narrowed to Linux terminals; the .cmd/.bat refusal and `batch-script-child` retired) — the platform is Linux terminals only (`decision/linux-terminal-platform`). That backs `open`'s `foot` + Hyprland design with no other OS arm. `revive`'s closed `--json` codes went back to ten, so any code `open` adds extends that set as it now stands.

@history_item n=10 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — clap is built without its `color` feature (`decision/clap-without-colour-feature`), so help and usage are plain in every mode. `open`'s exit-2 allowlist refusal is clap's usage error, and its hidden flags are clap args, so they inherit this.

@history_item n=11 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `mcp` is a hidden verb (`#[command(hide = true)]`) and appears in no `--help` row. This is the precedent for hidden CLI surface: the hidden checked flags `open` hands the started `run` (origin, target workspace) must stay out of `--help` the same way.

@history_item n=12 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — `--help` lists the served verbs only, grouped, then `Options:`. Its width source and served-verbs-only rule were made the founder's at 2026-10-20-t06-58-31-wrap. `open` joins `setup:` only once it is served.

@history_item n=13 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — set the as-built `setup:` row to `run, revive, verify, plugin install`. That is the row `open` changes to `open, run, revive, verify, plugin install` (`contract/help-output/verb-groups`, `contract/viola-help/verb-groups`).

@history_item n=14 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the `handoff:` help row built) — the most recent help-row change: one row added, its tests holding the five groups byte for byte. Adding `open` to `setup:` amends the same two verb-groups rows and the same help tests.

@history_item n=15 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's word made `viola --help` listing the served verbs only (and the width source) his own. The help grouping `open` edits stands on the founder's ruling, not a delegate's.

@history_item n=16 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (Iconography: `~ decision/list-six-fixed-columns` · `~ contract/viola-list/columns`; Surface: cli: row-order, name-column-width) — the board's six columns, captions and slot order stay fixed. A tree indent sits inside NAME, links print in a `-- LINKS --` block, and each wrapped child is drawn under its parent. `open`'s `--parent` feeds that lineage, and its `window?` / `origin?` go to `list --json` only, keeping the board unchanged as these rows require.

@history_item n=17 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — built the unwrapped rack: bare strips, read-only, never a target (`decision/bare-strips-never-target`). It also added `revive`'s session-live pair in the fixed start-refusal order. `open`'s `--parent` and placement target must not resolve to an unwrapped session.

@history_item n=18 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Surface: cli 4 rows and 10 rows: `~ contract/cli-exit-codes/10-14`, `~ contract/cli-exit-codes/11`; Brand Identity: `~ decision/fixed-phraseology-vocabulary`; `~ decision/no-hint-names-viola-release`) — exit 11 is retired and never reused, and `budget-paused` left the fixed vocabulary. The scope's "no exit 11" rests on this, and `open`'s new words (`terminal-not-found`, `not-live`, `unplaced`) join the fixed vocabulary that row governs.

@history_item n=19 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — `+ contract/auto-verify-line/*`: `run` / `revive` print fixed stderr start lines as their automatic verify begins, silent under `--json`. The `run` that `open` starts in the foot window can print these lines. `open` must either own them or leave them in the new window, and its `--json` document must not carry them. The founder ratified these rows at 2026-11-22T17-29-41Z-wrap.

@history_item n=20 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0: brand, surface and bans scoped to the CLI) — no web or graphical surface. "No interactive prompt, no TUI mode and no pager" and the spinner / live-redraw ban are scoped to a verb's output, and the terminal fleet console (matrix#v2-14) gets its own surface. `open` is a verb, so it stays static with no spinner while it waits up to 30 s for liveness (`decision/no-spinner`, `decision/no-spinners-in-verb-output`). The console's window work stays out of scope.

@history_item n=21 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows, each verified against the code. Every picked row this chunk extends or adds (the help verb groups, exit codes 1 and 2, `revive --json` codes, a new time-bound row) is amended row by row (`+` / `~`) from here on.
