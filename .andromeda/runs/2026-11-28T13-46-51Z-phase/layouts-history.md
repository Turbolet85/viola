@history_item n=1 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (Surface: cli, 4 rows) — added `+ contract/cli-verbs/console` (no argument, `--home` only, no `--json`, exit 0 on quit/interrupt/end of input, 1 on a refusal or internal error), put `console` in the `board:` help row and in `~ decision/board-vs-strip` as a strip-less verb. Matters here: the actions must keep the verb argument-free and its exit set unchanged; a selection is the console's own state, never a CLI target, while each action it fires still acts on one strip named by its ViolaName.

@history_item n=2 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (Surface: console, 2 rows) — `+ decision/console-own-surface` (alternate screen, cursor hidden and restored on every end, plain ASCII, no colour; PROVISIONAL, delegate viola-overseer) and `+ decision/console-frame-floor` (80x24, `viola console` row 1, `q quit` on the last row, the one-line below-floor message). Matters here: every action mode, typed send text and `unable` line must sit inside that frame with no colour, the key line will grow past `q quit`, and the restore on every end must hold through each new mode (the no-key-trap clause).

@history_item n=3 marker=2026-11-28-fleet-console-view
**2026-11-28-fleet-console-view** (Surface: console, 2 rows) — `+ decision/console-fleet-body-lines` (fleet as body lines in the board's forest order and indent) and `+ decision/console-redraw-on-change` (re-read once a second, redraw only on a view change). PROVISIONAL, delegate viola-overseer. Matters here: a selection must survive the one-second re-read and the forest order, and an action's result and selection marker are view changes that trigger a redraw. Directive item 1 also bounds what this chunk may change in the view.

@history_item n=4 marker=2026-11-28-fleet-console-view
**2026-11-28-fleet-console-view** (Surface: console, 1 row) — `+ decision/console-agents-read-off-loop` (claude agents read on its own thread at most every 5 s, so a key never waits on it). Matters here: STATUS lags by up to 5 s and reads `unknown` after a failed read, so a send's readback must come from the frame verb's reply, never STATUS (carry cry-qjw64paz). By the same rule, a slow verb call must not block the key loop.

@history_item n=5 marker=2026-11-28-fleet-console-view
**2026-11-28-fleet-console-view** (Surface: cli, `~ decision/no-prompt-or-pager`) — reworded the row to "the terminal fleet console (`viola console`), its own surface, is governed by no verb-output rule". Matters here: the console's prompts (send text entry, any confirm) are allowed because they belong to the console's surface. The verbs it calls stay non-interactive, and their output rules still bind what they return.

@history_item n=6 marker=2026-11-28-fleet-console-view
**2026-11-28-fleet-console-view** (Surface: cli, `~ decision/shared-captions-fixed`) — the console reuses the six captions and adds `LAST` after them (PROVISIONAL). Matters here: the actions must not add or reorder columns; a selection cue cannot be a new caption.

@history_item n=7 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — the 0.2.0 census narrowed the CLI's "no TUI mode" ban to "no interactive prompt and no pager in a verb", gave the keyboard-driven fleet console (matrix#v2-14) its own surface, and had the console reuse the six captions (census C-192, C-211, C-214, on the founder's own words). Matters here: this ruling is why a console may hold key-driven modes while the CLI verbs stay prompt-free. An action must not bring an interactive prompt into the verbs themselves.

@history_item n=8 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** — Surface: cli: one token vocabulary on every surface, the differences being platform mechanics only (now `decision/one-token-vocabulary-on-cli`). Matters here: the console's selection cue and action lines draw from the same vocabulary (brackets, two-space gutter, ASCII `->` / ` - `) and stay colourless under console-own-surface.

@history_item n=9 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — fixed the wait/last lines: the `session-end` → turn-ended form, `dialog unknown`, the exit-21 `unable  <name>  instance-unreachable` + hint, `waiting:` only on a terminal stderr without `--json`, the catch-site `error: internal error`; wait/last joined the shared `unable` word column. Matters here: these are the lines a console `last` (or wait) action shows. Its `unable` line takes the word-column form, and `waiting:` is a TTY-stderr behaviour the console must not leak onto its own screen.

@history_item n=10 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — `answer` joined the catch-site verbs (exactly `error: internal error`, no hint). Matters here: an answer action's internal failure shows that one line with no hint, same as the shell verb.

@history_item n=11 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — `pause` and `release` joined the catch-site internal-error verbs. Matters here: the console's pause/release actions share the shell verb's failure line; no second path.

@history_item n=12 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — `[RB] read back` may also be confirmed by a local command's post-condition (`/clear`'s new session), not only by a matching prompt-submitted. Matters here: a console send's readback is the send verb's outcome, local-command confirmation included, never something the console works out itself.

@history_item n=13 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the `input-not-ready` hint became `send again, and if it repeats a human must look at the session` (the founder's wording). Matters here: a console send refused `input-not-ready` shows this exact hint under its `unable` line.

@history_item n=14 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — `viola send` refuses `control-character` and `empty-text` before any frame (`[/ ] unable`, exit 13, hint last), and the wrapper repeats those refusals. Kept: nothing measured whether the issue line prints on a client-side refusal. Matters here: the console's send text entry must route through the same client refusals (empty entry, control characters typed in the entry) and show them as the verb's line. Whether the open issue line shows before such a refusal is still unmeasured.

@history_item n=15 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — `/compact` is read back by its compacted-session post-condition inside a 40 s PROVISIONAL window, else `not-delivered` / `no-prompt-submitted`. Matters here: a console send's readback can take up to 40 s, far past the one-second re-read (the P3 premise on the readback's bound). The key loop must not block waiting for it.

@history_item n=16 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — `~ contract/viola-send-output/read-back`: the confirmation match widened to the typed text less a trailing run of space, TAB, CR and LF. Matters here: text from the console's entry (a trailing newline from Enter, for example) is confirmed by this rule. The console must pass the text through unchanged and let send decide.

@history_item n=17 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the founder ratified the trailing-only tolerance; a leading trim waits on the live reading carried on «Standing live rig with a screen model». Matters here: the console must not trim leading whitespace from typed text before calling send.

@history_item n=18 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — `~ contract/viola-send-output/unconfirmable` (`/remote-control`, `/cost`, `/usage`; `/mcp` conditions) and `+ contract/viola-send-output/read-back-mcp`. Matters here: a console send of a local command can end `[  ] unconfirmable` on stdout with exit 0, a third outcome the console must show as the verb shows it.

@history_item n=19 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — `/context` added to the unconfirmable list and `+ contract/viola-send-output/read-back-context`. Matters here: this widens the set of send outcomes the console's readback display must carry.

@history_item n=20 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** — `+ contract/viola-send-output/paste-cut-hint` (a paste-cut struck mirror line with its own hint). Matters here: a long console send can end paste-cut. Its hint tells a human to look at the session, which the console must show as is.

@history_item n=21 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** — ratified paste-cut-hint and `contract/viola-run/start-output`, and added `+ decision/permission-request-question-answerable`: a PermissionRequest question with no armed dialog is held and answerable by viola answer, except under a human key, an expired hold or an unverified CLI. Matters here: the console's answer action reaches those held dialogs as well, and a human key leaves the question to the human.

@history_item n=22 marker=2026-11-22-epoch-4-cleanup
**2026-11-22-epoch-4-cleanup** — lifted the provisional clause from the paste-cut hint; the words stand. Matters here: the console shows that hint exactly as the verb prints it.

@history_item n=23 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — `+ contract/viola-answer-output/not-delivered-hints`: answer carries send's hints for five not-delivered details, its own for `unknown-dialog`, and none for an opaque detail. Matters here: a console answer refusal's `unable` line and hint follow that table.

@history_item n=24 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the `dialog-expired  <name>  dialog <id>  cursor <n>` wait line (the dialog form). Matters here: a console answer can race an expired hold, which shows as `unknown-dialog`. That is the verb's refusal, not a console error.

@history_item n=25 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's Epoch 5 batch ratified the `dialog-expired` line, the board width from `host_size()` (never `COLUMNS`) and the served-verbs-only help. Matters here: these wordings are fixed. The console's own width handling follows the same rule: read the pty size, never `COLUMNS`.

@history_item n=26 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — a frame verb whose server check fails writes no frame, exits 21 and ends with `hint: the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`. The not-running hint stays only for the not-running cause. Matters here: an exit 21 from a console action has several causes, each with its own hint. The console must show the verb's own hint, not one generic unreachable line.

@history_item n=27 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (exit-21 lines, server check and catch site) — `link` / `unlink` joined the exit-21 `unable` lines; the server check covers eight frame verbs; the catch site counts a refusal reply, or an `ok` of another shape, as an internal error. Matters here: if the console offers link/unlink, `<name>` in their refusal is the driven instance, and the eight-verb set is the frame-verb list the console's actions draw on.

@history_item n=28 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (link / unlink result lines) — one stdout line each, plain: `link    <driver> -> <driven>  since <HH:MM:SSZ>` / `already linked` / `unlink  <driver> -> <driven>`, the names being the verb's own arguments. A self-pair is refused by the wrapper only (exit 20). Matters here: a link action needs two strips, which a one-row selection does not give. The arguments follow `decision/argument-order-mirrors-channel`, and the console must not refuse a self-pair itself.

@history_item n=29 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (strict-modes refusal of the driven instance) — exit 21 with the state-files pair, before any role file or log opens. The trap: opening a role file narrows a home to 0700, so a check placed after `client::start` passes a widened home. Matters here: a console action must call the verb's check order exactly. An in-process call that opens logs first would hide the refusal.

@history_item n=30 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — the frame verbs' exit-21 strict-modes instance pair (send after its text read, wait, last, answer, pause, release, link, unlink), and the home-tree refusals of the run and revive starts (exit 1, no log, no child). Matters here: a console action through any of these verbs shows that pair as its refusal. A console revive meets the start's home-tree pairs.

@history_item n=31 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the `unknown` refusal's detail is a code word or nothing, and `unknown`, `wrapper fault` and `internal error` still take no hint. Matters here: a console-shown refusal never carries peer free text. These three causes show no hint line on the console either.

@history_item n=32 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — exit 21's `--json` document carries its cause in `detail`; the catch site prints the `internal-error` document under `--json`; revive gained `stale-heartbeat` and its post-preflight codes; new hints for unwrapped-name and endpoint-lost (`hint: <name>'s wrapper went away during the call; …`). Matters here: if the console reads a verb's machine form to tell causes apart, these detail codes are the catalogue. The endpoint-lost hint is the line a console call shows when its wrapper dies mid-call.

@history_item n=33 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — the founder ratified the four provisional records of 2026-10-31-exit-cause-code-catalogue. Matters here: the exit-21 cause catalogue and its hints are settled ground the console's refusal display can rest on.

@history_item n=34 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (socket-directory refusal and exit 21's sixth cause) — the run/revive start's socket-directory pair (exit 1), and `socket-dir-refused` as the frame verbs' sixth exit-21 cause, with its own hint and document. Matters here: one more refusal line and hint a console action can show, through both frame verbs and revive.

@history_item n=35 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (server-check hint and .cmd/.bat removal) — the server-check sentence is no longer scoped to an OS (Linux only), and the `.cmd`/`.bat` refusal left revive. Matters here: console actions assume Linux only, so no per-OS branch is needed in how they show refusals.

@history_item n=36 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — exit 11 retired and never reused, `budget-paused` read as `unknown` (14), `viola release --budget` and its lines removed, the send unable exits now 13 (or 10/12/14). Matters here: a console release action has no `--budget` form, and its exit table has no 11.

@history_item n=37 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — revive's output structure: a passed start prints nothing while the child holds the terminal; each refusal is exit 1 with one `unable:` line and one `hint:` line last, empty stdout. Matters here: revive is a passthrough verb whose child takes the terminal. A console revive cannot run it in the console's own terminal. It must be placed elsewhere or run without its passthrough (P3's in-process vs child-process premise), and its refusals are the `unable:` pair form, not the two-space line form.

@history_item n=38 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added the `-- UNWRAPPED - READ-ONLY --` rack (NAME is the escaped sessionId, `n/a` cells) and revive's fifth pair (`session is open in another claude`, its hint naming `--fork`). Matters here: an unwrapped row is read-only and its NAME is no ViolaName (`decision/targets-are-wrapped-names`). The console must not offer a frame-verb action on it, or must show the verb's `unwrapped-name` refusal. A revive of a session already open elsewhere meets that pair.

@history_item n=39 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — revive takes `--json`, its refusals one exit-1 document each, and `revive --list` sits at the cli catch site. Matters here: one way for a console revive to read a verb's outcome without the human pair, if P3 picks a child-process call.

@history_item n=40 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — viola list styled on a terminal; unwrapped NAMEs cut to the column less 3 plus `...` with the width known from `host_size()`. Matters here: a drawn cell can be cut, so a selection must name its row by the board's item, never by the drawn text (carry cry-qjw64paz).

@history_item n=41 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (board tree, LINKS block, no link column) — `~ decision/board-ordered-by-name` (each child under its parent, NAME indented), `+ contract/viola-list-board/tree`, `+ contract/viola-list-board/links-block`, `~ decision/no-link-column`. The 2026-11-27T19-07-11Z-wrap entry ratified the delegate words. Matters here: keyboard movement of a selection follows this tree order, not plain name order. The NAME cell carries an indent that is not part of the ViolaName passed to a verb. Links are shown by the link/unlink output, never by a column.

@history_item n=42 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** — revive gained `--model` / `--effort` (ratified 2026-11-27T19-07-11Z-wrap). Matters here: the arguments a console revive (and open) may pass. Which of them the console offers is the operator's open fork.

@history_item n=43 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — `+ contract/cli-verbs/open` (a new foot window beside its parent, a closed flag allowlist, no shell), `+ contract/viola-open/parent-default` (the parent defaults to the calling agent's ViolaName with origin `agent`, else no parent and origin `cli`), `+ contract/viola-revive/no-window`. Matters here: "open a child" is a v2-15 action. Called from the console, the parent must be the selected row's name passed explicitly, since the console is no agent, and the origin then reads `cli`. The child's window opens outside the console, so the console keeps its terminal.

@history_item n=44 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — `+ contract/cli-verbs/focus` (a CLI-only human control, one ViolaName plus `--json`), and the focus strict-modes refusal added to exit 21's fixed state pair. Ratified by the operator in «2026-11-24-viola-focus — the operator's answers to F-A and F-B ratified». Matters here: focus is a v2-15 console action for the human, and its refusal is the fixed state pair, not the `unable  <name>` line.

@history_item n=45 marker=2026-11-24-viola-up
**2026-11-24-viola-up** — `~ contract/viola-revive/no-window` (the hidden `--origin` replays the recorded workspace), `~ contract/viola-revive/usage`, `~ decision/board-vs-strip` (up re-opens the fleet), `~ contract/cli-exit-codes/21` (up's board refusal); ratified 2026-11-27T19-07-11Z-wrap. Matters here: a console revive places a window only when given `--origin`, which is the way it can move the revived session out of the console's terminal. Up is a whole-fleet verb, not a one-row action.

@history_item n=46 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** — `+ contract/cli-verbs/switch`, `+ contract/viola-switch-output/switched` / json-ok, switch joined `contract/frame-verbs/strict-modes-refused`, and run prints nothing at a switch's respawn (fork F1, PROVISIONAL, ratified 2026-11-27T19-07-11Z-wrap). Matters here: switch's console leg (carry cry-mf2w3ioc) goes through this verb. It needs at least one of `--model` / `--effort` (else exit 2), and it shows `<name>  switched  <model> <effort>` or the strict-modes pair.

@history_item n=47 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — `~ contract/cli-verbs/run` / revive: the start writes the CLI's trust record for an untrusted session folder, keyed by the recorded directory. Matters here: a console revive writes that trust record. Under the directive (no real `~/.claude*`), every console revive test must keep the test-scoped home.

@history_item n=48 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — `~ contract/viola-run/start-output`: before the child, a key-on start's automatic verify line and verify's own output reach the terminal, discarded under `revive --json`; ratified 2026-11-22T17-29-41Z-wrap. Matters here: a console revive's start can write to the terminal it inherits, a trap for the console's alternate-screen frame. `--json` or a separate terminal avoids it.

@history_item n=49 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** — `− contract/cli-catch-site/internal-error`, its promise standing under `contract/cli-internal-error/human` / `json`; adopt's start is silent at the catch site like revive's. Matters here: these are the rows that state a console action's internal-error line, and a revive start stays outside the catch site.
