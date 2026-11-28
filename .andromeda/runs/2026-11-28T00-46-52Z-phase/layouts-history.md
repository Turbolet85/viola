@history_item n=1 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — the 0.2.0 census: Surface: web-spa became "no web surface, viola renders no page", and the keyboard-driven terminal fleet console tiled on Hyprland (matrix#v2-14) gets its own surface section at the route entry «Console scaffold» (C-192). The flat-verbs ban was narrowed from "no interactive prompt, no TUI mode and no pager" to "no interactive prompt and no pager in a verb", because the fleet console is no verb's output, with its surface owed by «Console scaffold» (C-211). Shared vocabulary now says the console reuses the six captions, words and slot order (C-214). The founder confirmed all of this in his own words on 2026-11-08. Why it matters: this is the precedent that owes this chunk its surface section. It bounds the redraw-ban reconciliation, which keeps the CLI ban scoped to verbs (decision/flat-verbs, decision/no-prompt-or-pager, absence/web-surface). It also binds the console to the board's shared words.

@history_item n=2 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — cut Surface: web-spa to the stub «The frontend is deferred to 0.2.0; its technology is not chosen.» and made the cli surface stand alone. "Multi-surface coordination" became «Shared vocabulary», with its parity line on the six captions, words and slot order. Why it matters: this is the stub the console's surface section replaces. CLI-board parity is the 0.2.0 frontend's starting intent (the founder's ruling of 2026-10-16).

@history_item n=3 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** — moved the Decisions Log out of the body. Surface: cli now holds "one token vocabulary on both surfaces; the differences are platform mechanics only". Why it matters: this is the origin of decision/one-token-vocabulary-on-cli. A second terminal surface (the console) must resolve the same token vocabulary, differing only in platform mechanics.

@history_item n=4 marker=2026-10-01-t12-19-55-wrap
**2026-10-01-t12-19-55-wrap** — switched the web-spa tooling context from Lit to React + TypeScript for 0.2.0's node-graph view. The 2026-10-16 stub ("technology not chosen") and the 2026-11-08 census (no web surface) later replaced it. Why it matters: it shows how the 0.2.0 surface's technology moved by founder ruling. The console's drawing or terminal technology is again his fork (the scope's boundaries).

@history_item n=5 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — this entry has five sub-entries. The Tooling context became clap 4.6.7, std `IsTerminal`, the hand-written SGR module and its fixed-width padding, "nothing else" (the windows-sys VT enable was removed). `viola list`'s uncoloured modes became `--json`, a pipe, `NO_COLOR` (empty included) or `TERM=dumb`. verify's `MM` is all 24 relied rows on every build, and its stop is `SIGINT` / `SIGTERM`. The help and plugin-install wording lost its per-OS clauses. Why it matters: it states the closed tooling set (decision/hand-written-sgr-styling, decision/std-isterminal-tty-detection, decision/clap-derive-cli-parsing, contract/viola-list-board/uncoloured-modes). Any crate that draws or reads a terminal widens it, which is his fork. The target is Linux only.

@history_item n=6 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built `viola --help` as the grouped table over served verbs only. It also styled `viola list` on a TTY: bold NAME, `DIALOG` amber with its kind plain, a stale row dim, and depth from `COLORTERM` / `TERM`. With the width known from `viola_pty::host_size()` (never `COLUMNS`), an unwrapped NAME is cut to the column less 3 plus `...`, never inside an escape. Why it matters: this is the origin of decision/amber-means-dialog, decision/dialog-attention-only-on-board, contract/cli-token-rendering/color-accent, contract/viola-list-board/colour-depth and contract/viola-list-board/name-truncation. The width source (the host pty size, never `COLUMNS`) is the precedent for sizing a quarter-screen frame. DIALOG amber is limited to `viola list`.

@history_item n=7 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's Epoch 5 batch. It ratified "served verbs only" for `--help` and the board width from `host_size()`. It removed the provisional marks on the dialog-expired line, the plugin install lines and verify's interrupted pair. Why it matters: the width source and served-verbs-only help now stand on the founder's word. A console start verb or flag that enters `--help` must stay inside served-verbs-only.

@history_item n=8 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — `viola list` as built, with its column widths, refusals and catch-site membership. At that time the styled TTY form and `--help` were owed to "CLI output discipline". Why it matters: it set the board's static fixed-column rows and its one-stdout-write discipline (decision/human-tty-expression-level, decision/only-header-bay-line, decision/words-carry-every-state), which the console reuses through the shared vocabulary.

@history_item n=9 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — built the unwrapped rack. It left the design-target truncation of unwrapped NAMEs owed to "CLI output discipline", which 2026-10-17-cli-output-discipline later discharged. Why it matters: it is the lineage of contract/viola-list-board/name-truncation, the only truncation rule on the terminal grid, and a precedent for any rule that degrades the frame below its size.

@history_item n=10 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — the board draws wrapped rows as a tree with an indented NAME and adds a `-- LINKS --` block. decision/no-link-column now reads that the six shared columns, their captions and their slot order stay unchanged. The operator ratified it on 2026-11-27 (2026-11-27T19-07-11Z-wrap). Why it matters: this is the shared vocabulary the console must reuse (the six captions and slot order). Lineage trees in the console are the sibling «Fleet console view»'s, not this chunk's.

@history_item n=11 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — the first `viola verify` output structure: `[NN/MM]` step lines and the `stamped <ver>  <n> pass  <m> fail` summary. Why it matters: it is the origin of decision/single-progress-pattern and contract/viola-verify-output/step-line, the project's one progress pattern. Its lines are appended once and never redrawn, which bears on the redraw-ban reconciliation.

@history_item n=12 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — changed verify's `MM` from a literal to the rule `LedgerRow::ALL.len()`. Why it matters: this is the precedent for carry `cry-boujwuyr` M3, which asks for the relied-row count from one source instead of literals moved by hand.

@history_item n=13 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — changed verify's `MM` to the relied count, `LedgerRow::relied()`'s length, instead of `ALL.len()`. Why it matters: it sets the single source carry `cry-boujwuyr` M3 rests on (contract/viola-verify-output/step-line: "MM being the relied count").

@history_item n=14 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — the summary closes only a completed run. A run stopped before its rows are checked prints only its `unable:` / `hint:` pair and writes no summary and no stamp. Why it matters: it is the origin of contract/viola-verify/summary's "a run stopped before its rows are checked prints no summary".

@history_item n=15 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — verify's step line takes `pass|absent|fail`, and the summary's `n` counts an `absent` browser-tools-prompt row. The step-line row keeps "each appended once, with no progress bar and no redraw". The founder ratified it in 2026-11-22T17-29-41Z-wrap. Why it matters: contract/viola-verify-output/step-line and its summary row are picked, and the step line's "no redraw" clause is one of the redraw-ban rows this chunk must reconcile, keeping the clause for the CLI.

@history_item n=16 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** — the founder ratified the 2026-11-18-browser-tools-prompt-modal rows (step-line, summary, exit-0), along with the other Epoch 4 PROVISIONAL marks. Why it matters: the verify step-line and summary rows this chunk reads stand on the founder's word, so narrowing their no-redraw clause would be his fork.

@history_item n=17 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — added the `viola revive` output structure and the `setup:` help group. The Expression level's 0.0 list names the `viola run` / `viola revive` passthrough. Why it matters: it is the origin of the passthrough arm of decision/plain-output-modes (no colour, no non-ASCII glyph, no cursor control). A console that draws is a different case and must not be confused with passthrough.

@history_item n=18 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `--json` became the machine view of every data verb, `viola revive` included. Each refusal is one stdout document with stderr empty. Why it matters: it is the lineage of contract/cli-json-mode/one-document and contract/cli-exit-codes/json-form. Any console start verb or flag must say how it stands against the every-data-verb `--json` rule.

@history_item n=19 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — this entry has four sub-entries. The catch site prints `{"v":1,"error":"internal-error","detail":null}` on stdout under `--json` (stderr empty) in place of `error: internal error`. This covers wait/last and plugin install. Exit 21's document carries its cause in `detail`. The `viola revive --json` gap after the preflight was closed: a program not found or a start error prints `internal-error`, and a `pump_child` error prints no document because the child held the terminal. The founder ratified these in 2026-11-01-t02-46-44-wrap. Why it matters: it is the origin of contract/cli-internal-error/json, contract/viola-plugin-install/internal-error and contract/run-revive-start-json/internal-error. A console holding the terminal meets the same "no document after the terminal was held" trap.

@history_item n=20 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — the founder ratified the provisional records of 2026-10-31-exit-cause-code-catalogue. Why it matters: the picked internal-error and json-form rows stand on the founder's word.

@history_item n=21 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — `error: internal error` is printed once from the one catch site for every `cli` verb, with no hint. `waiting:` goes only to a terminal stderr. Why it matters: it is the origin of contract/cli-internal-error/human and decision/stdout-results-stderr-rest. The console's process and its new diagnostics role must say whether they go through the catch site.

@history_item n=22 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** — added contract/viola-help/setup-row (the setup row got its own row because the verb-groups literal passed its bound). It removed contract/cli-catch-site/internal-error because adopt's start is a silent role at the catch site, so naming revive alone was false. Why it matters: this is the precedent for adding a verb to `--help` (the setup-row literal must be edited exactly). Silent roles at the catch site bear on the console's own role.

@history_item n=23 marker=2026-11-24-viola-up
**2026-11-24-viola-up** — this entry has three sub-entries. decision/board-vs-strip gains `viola up`, which acts on the fleet. contract/cli-verbs/help and contract/viola-help/verb-groups gain `up` and `focus`. contract/cli-exit-codes/21 names up's board refusal. The operator ratified it in 2026-11-27T19-07-11Z-wrap. Why it matters: these are the rows a console start verb would extend (board-vs-strip, since the console shows the whole fleet; the help rows). It is the latest precedent for a fleet-wide verb.

@history_item n=24 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** — `switch` was added to the setup row and to contract/cli-verbs/help. contract/cli-verbs/run now says viola writes no byte to the terminal at a switch's respawn of the child. The operator ratified it in 2026-11-27T19-07-11Z-wrap. Why it matters: these are the current literals of contract/viola-help/setup-row and contract/cli-verbs/help, which a console verb would edit.

@history_item n=25 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — `board:` holds `list` and `focus`, a command for the human only that acts on the Hyprland window. contract/cli-exit-codes/21 names focus. The operator ratified it (amd-rc3osznz). Why it matters: the `board:` group is where a console verb could sit beside list and focus, and focus is the window-touching precedent for the tiled console.

@history_item n=26 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — `open` was added to the setup row and contract/cli-verbs/help. It also added contract/viola-revive/no-window (the window step reaches only a start that asks for it). Why it matters: it is part of the help rows' lineage and precedes window placement, which the console's tiled window (a sibling's) will meet.

@history_item n=27 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — built the `handoff:  link, unlink` help row and retired the "joins when its entry serves" wording. Why it matters: the help table lists only served verbs and never a pending one. A console verb is owed to the help only once it is served.

@history_item n=28 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp` is a hidden verb that appears in no help row, and the `HELP` literal is unchanged. "No human surface" stays true. Why it matters: this is the precedent behind decision/no-banner ("`viola hook` and `viola mcp` have no human surface"). It is the hidden-verb option for the console's start verb or flag, which is his fork.

@history_item n=29 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — the output structure of `viola plugin install`, with exit 1 at the catch site. It joined the setup row. Why it matters: it is the origin of contract/viola-plugin-install/internal-error and of `plugin install` as the one two-word verb (the exception in decision/flat-verbs).

@history_item n=30 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — retired exit 11 (`budget-paused`), never to be reused, and cut the BAY line to `BAY  skipped a - b - c`. Why it matters: contract/cli-exit-codes/11 is picked, so no new console refusal may take exit 11. The BAY line is the only header (decision/only-header-bay-line).
