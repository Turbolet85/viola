@history_item n=1 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0 — brand, surface and bans scoped to the CLI) — Surface: web-spa became «no web surface, the console taking its own surface section at «Console scaffold»» (C-177); the Navigation Pattern's «no interactive prompt, no TUI mode and no pager» was scoped to a verb, the console being no verb's output (C-183, today decision/fleet-console-not-verb-output and decision/no-interactive-prompt-or-pager); the spinner / progress-bar / live-redrawing ban was scoped «in a verb's output», «the console's redraw rule being «Console scaffold»'s»; the design direction became the one the terminal fleet console (matrix#v2-14) inherits. This is the founder-confirmed precedent the redraw-ban reconciliation rests on: the ban was narrowed to verbs, never lifted, and the console's own rule was left owed to this chunk.

@history_item n=2 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0 — the never-built page's design tokens retired) — palette, type, spacing, depth, radius and motion tokens retired whole; each section's premise now reads N/A, viola rendering no page (the cli's type, spacing and ink the terminal's); Core Colors' eight values, Motion's expression-level line (C-171) and the CLI glyphs (C-174) kept. The console's surface section must not revive page tokens: the N/A absences (absence/rendered-page) and the kept CLI glyph / colour rows are what it builds on.

@history_item n=3 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — cut Surface: web-spa to the stub «The frontend is deferred to 0.2.0; its technology is not chosen.» and cut web motion and web bans; the stub this chunk's scope names as the section it replaces. Brand Identity was kept untouched by the founder's ruling.

@history_item n=4 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — web references cut from Brand Identity, palette, Typography, Motion and the Signature Test; the per-surface expression table holds the two cli rows (0.2 TTY, 0.0 otherwise), the frontend expression level an N/A deferred to 0.2.0, Motion calibrated to base 0.2; Kept: `## Surface: web-spa` held its heading as «the setup grammar's surface slot for the deferred frontend, shared with layout-templates». Bears on where and at what expression level the console's surface section lands (decision/expression-level, decision/cli-has-no-motion).

@history_item n=5 marker=2026-10-01-t12-19-55-wrap
**2026-10-01-t12-19-55-wrap** — chose React + TypeScript for the then web-spa (and set the standing rule that relaxing a CSP directive or a ban is a boundary widening the founder rules); superseded by the 2026-11-08 census (no web surface). A trap: the console's technology is a terminal one, still unchosen, and any ban relaxed for it is a founder fork by this precedent.

@history_item n=6 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — clap built without its `color` feature so no dependency reads a colour or terminal variable and every styled byte is viola's own SGR module's (decision/clap-without-colour-feature, decision/hand-written-sgr-module). Constraint on any terminal-drawing dependency the console scaffold proposes (his fork per the scope).

@history_item n=7 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built the colour layer (DIALOG amber, stale rows dim, NAME bold), the width cut (unwrapped NAME only, `...`) and the width source `viola_pty::host_size()` with `COLUMNS` never read; `NO_COLOR` present incl. empty value; `--help` lists served verbs only. The register behind decision/columns-never-read, contract/human-tables/width-source, decision/truncate-only-unwrapped-name and the colour decision order the console frame's size reading and styling rest on.

@history_item n=8 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified the width from `viola_pty::host_size()` and `--help` listing served verbs only (was delegate PROVISIONAL). Settled ground for the quarter-screen frame's size source and for adding a console start verb to help.

@history_item n=9 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the cli Platform narrowed to Linux terminals; the width read by TIOCGWINSZ alone; the Windows VT enable retired from the colour rules) — Platform reads Linux terminals, CI `ubuntu-latest` (decision/linux-terminal-platform); `host_size()` is a `TIOCGWINSZ` ioctl only; colour decision order lost rung 7 and the Windows depth branch. The console's PTY test driver and frame sizing target Linux only.

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the stale-row colour witness and where it now stands) — `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` (tests/cross_list_rows.rs) drives `viola list` under an outer PTY on `ubuntu-latest`. Precedent for a PTY-driven test harness; trap: it asserts screen escape bytes, while this chunk's keystroke driver must assert state, never the screen.

@history_item n=11 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — the outer-PTY witness of the stale row (`TERM=xterm-256color`, exact DIM/BOLD/RESET sequence) landed; the existing PTY test machinery the console's keystroke driver can reuse or must differ from (state not bytes).

@history_item n=12 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — the 0.0 expression row and the no-colour ban name a passed `viola revive` as the same passthrough as `viola run`; `revive` joined the `setup:` help group. Origin of contract/cli-colour-decision-order/viola-run and decision/plain-output-modes the console must not collide with.

@history_item n=13 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Brand Identity: 1 row; Surface: cli rows) — decision/run-prints-nothing-while-wrapped and decision/start-refusals-show-no-path-or-pid extended to `adopt`; decision/silent-catch-sites and contract/cli-catch-site/silent-verbs gained `adopt` as a named silent role. Precedent for how a new process role (the console's) is added to the catch-site and silence registers.

@history_item n=14 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the catch site's `internal-error` document) — the catch site prints `error: internal error` or its `--json` document; `run`, `revive`'s start, `mcp` and `hook` stay silent at it. A console process holding the terminal needs a named catch-site behaviour alongside its own diagnostics role (decision/no-stack-traces, decision/no-hint-for-faults).

@history_item n=15 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (exit 20's closed `detail` and the fixed `wrapper fault` message; `unknown`'s detail) — faults print the code only, detail to `diagnostics/`; `unknown` takes no hint because it would have to guess. The ground of decision/no-hint-for-faults; any console fault line follows it.

@history_item n=16 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** — the latest move of contract/help-output/verb-groups and contract/viola-help/verb-groups (`setup: open, run, revive, adopt, up, switch, verify, plugin install`), contract/cli-refusal-line/fixed-unable-form and contract/cli-exit-codes/13 (four new details). The current wording any console start verb or flag (his fork) would extend; both help twins must move together.

@history_item n=17 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — added `focus` (board group `list, focus`, name/command/focus), its compositor-read refusals and exit-1 codes; contract/cli-exit-codes/1 and the fixed-form lists name it. Precedent for a board-side verb touching the compositor, and for the help board row the console could join; ratified by the operator (amd-wfszjzeo).

@history_item n=18 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (Surface: cli: 6 row(s) (1 of 2)) — decision/list-is-the-board reworded so `up` acts on the whole fleet; `up` joined the help `setup:` row and the fixed-form lists. A precedent the console (a fleet-wide surface) must square with the «list is the board» rule; ratified 2026-11-27 (amd-rt2dc2ig).

@history_item n=19 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — added `open`, the name/command register entry and the placement / origin words for Hyprland window placement. The command-name register a console verb name must be checked against, and the existing window-placement vocabulary the tiled console window will meet.

@history_item n=20 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `mcp` is a hidden verb, in no `viola --help` row (decision/mcp-hidden-verb). Precedent if the console's start is a hidden or role-only entry point rather than a served verb.

@history_item n=21 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the `handoff:` help row built) — `--help` gained its `handoff:` row between `wheel:` and `setup:`, held by the help tests. Precedent for how a help group row is added and which tests pin it.

@history_item n=22 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — `verify`'s static step counter, one appended line per row then a summary; origin of the project's one progress pattern (decision/step-counter-only-progress-pattern, decision/one-progress-pattern), which the console's redraw must not be read as contradicting.

@history_item n=23 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — contract/viola-verify-output/step-line, /summary and /summary-line gained the `absent` verdict; ratified by the founder (amd-spbq2ncg, 2026-11-22T17-29-41Z-wrap). The current text of the picked step-counter rows.

@history_item n=24 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — contract/refusal-hints/refusal-line: `send` and `answer` share hints per reason or detail, an opaque or absent `not-delivered` detail prints no hint. The refusal-line form the console's later `unable` lines inherit (owed to «Console actions»).

@history_item n=25 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** — contract/cli-exit-codes/13 gained `paste-cut` and contract/refusal-hints/not-delivered-paste-cut was added (human mode only); provisional clause lifted at 2026-11-22-epoch-4-cleanup. Picked rows the console's refusal display must reproduce verbatim.

@history_item n=26 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the `input-not-ready` hint reworded to the founder's text (contract/refusal-hints/not-delivered-input-not-ready).

@history_item n=27 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — decision/fixed-phraseology-vocabulary lost `budget-paused`; contract/cli-separators/space-hyphen-space's example became `skipped 2 - 0 - 1`; contract/cli-exit-codes/10-14 and /11 state 11 retired and never reused; decision/no-hint-names-viola-release's reason narrowed to human-typing. The vocabulary and exit registers any console label must use, with no budget word.

@history_item n=28 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — the arrow contract extended to `viola list`'s `-- LINKS --` block; NAME column width includes the tree indent (contract/viola-list-board/name-column-width); six fixed columns kept. The board forms and arrow glyph usage (contract/cli-glyphs/arrow) the console frame sits beside and its sibling view will reuse.

@history_item n=29 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — the board as built: row mode escapes every control character incl. DEL and C1 as `\xHH` (`escape_row`), and `list`'s exit-21 whole-board refusal. Origin of decision/list-rows-escape-all-controls and decision/escape-controls-in-human-output that reach any console text.

@history_item n=30 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — message mode spelled out: every control character but `\n` and `\t` prints as `\xHH` (contract/cli-message-escaping/control-characters).

@history_item n=31 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the board's one rule on the home) — the board's strict-modes scan holds the home to the read rule and `instances/` to the write rule (contract/viola-list-board/strict-modes-scan). Constraint on the console's own reads of the home and on the test-owned home its driver builds.

@history_item n=32 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (Surface: cli: 4 row(s) (1 of 2)) — contract/exit-21-json-cause/strict-modes-failed and contract/cli-stderr/fixed-message-refusal gained `up`; ratified 2026-11-27 (amd-szfmyrwo). The fixed-form and strict-modes registers a console refusal would extend.

@history_item n=33 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** — contract/cli-exit-codes/2 gained the fixed `PAIR_AFTER_DASHES` line. The current exit-2 row a console start flag's usage errors land under.

@history_item n=34 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** — the Decisions Log lifted into the body, among it Brand Identity's rejected presets #7, #31, #51 (decision/rejected-style-presets: OLED dark, real-time monitoring pulsing/blink/toasts, HUD/FUI glow). Bans that bind an interactive console's look.

@history_item n=35 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows (decision / contract / absence / name / overview). The console's surface section must be written as rows under this grammar, and its owed rows named, not built.
