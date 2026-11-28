# Founder forks — «Console actions» (2026-11-28-console-actions), the P3 block

Written 2026-11-28T14:28:05Z by the builder at the end of P3, under the overseer's directive (`inputs#I1`). The
operator is away. Every fork below is his to answer, relayed by the overseer verbatim. **The phase stops here**:
F1 is a boundary widening, and F2–F7 are product choices the build stands on (directive item 5).

## Window
- Run dir `.andromeda/runs/2026-11-28T13-46-51Z-phase/`; HEAD `a07542e9` = upstream; CI green 7/7 on the three
  shas since the last flip (ci#46149816293 · ci#46144426786 · ci#46130927638).
- Promoted `2026-11-28-console-actions` (pending, tx 5410); scope rev 3 (premises closed, tx 5416); research rev 1
  (tx 5415, its lists parsed, PROVISIONAL behind these forks).
- Row read: 2180 rows picked of 5033 across the seven masters, none at 0; the 7 history files read whole. Re-spawned
  once each, then passed: two absent pickers and obs-history (a cite to a marker its sidecar does not hold).
- Read-only compositor queries run: none. No live start, no real `claude`, no read of `~/.claude*` or `~/.viola`.

## Measurements (each from the code, a test or the masters)

### M1 — the CLI verbs, what each needs, and whether the console can reach it
19 served verbs (`src/cmd/mod.rs:66-114`; `hook` and `mcp` hidden, never a person's). Every verb is
`fn(home, args) -> ExitCode` (`src/cmd/mod.rs:128-300`). Each one inits its own process-global log subscriber and
writes straight to the process's stdout and stderr (`src/cmd/client.rs:44-47`, `src/cmd/send.rs:100-169`).
**Called in-process from the console, a verb would install a second subscriber over the console's and print over its
alternate screen.** So «through the CLI verbs, no second code path» (matrix#v2-15) can only mean: the console
**runs the verb as a direct child process of its own exe**, with `--home` passed and its output captured. The
precedent is `src/run/auto_verify.rs:233-262` (the pinned exe, `--home`, no shell, tied to its parent's death).
The child writes into the target instance's `cli-<name>.ndjson`, as at a shell (obs-plan
`contract/cli-log-file/*`). The console's own `console.ndjson` gets one line per key.

| verb | needs beyond the selected row's name | from a one-row selection | its longest wait |
|---|---|---|---|
| send | a text (stdin or `--file`) | yes, with a text entry | 53.5 s (8.5 gate + 5 paste + 40 `/compact`; `screen.rs:16-31`) |
| answer | a dialog id and a JSON response of the dialog's kind | yes, if the console reads the dialog and builds the response | short; the hold 60–3600 s bounds the driver |
| last | — | yes; prints a multi-line message | short |
| wait | `--after` / `--timeout-ms` optional | yes; it blocks until a turn event | unbounded unless `--timeout-ms` |
| pause · release | — | yes | short |
| focus | — | yes | 5 s hyprctl deadline |
| switch | at least one of `--model` / `--effort` (a model word, one of five effort words) | yes, with a choice or a text entry | 10 s reap bound |
| open (a child) | a NEW name, `--parent <selected>`, optional role / cwd / model / effort | yes, with a name entry | 30 s live wait |
| revive | — (`--id`, `--fork`, model / effort optional) | not in the console's terminal: a passed revive takes the terminal it runs in. `viola up <name>` re-opens a named instance in a terminal of its own (`src/cmd/up.rs:39-40`) | the start |
| up | names optional | the whole fleet, or the selected one | per terminal |
| adopt | a session id and a new name | from an UNWRAPPED row: the row's `sessionId` is no ViolaName and no target (`decision/unwrapped-name-never-target`) | the start |
| link · unlink | TWO names (driver, driven) | a one-row selection names one | short |
| list · console · verify · plugin install · run | — | not row actions (`list` is the board itself; `run` takes the terminal) | — |

### M2 — a send's readback today
The `send` verb's own reply (`src/cmd/send.rs:206-224`): `[RB] read back` (exit 0), `[  ] unconfirmable` (exit 0,
an `ok` with `confirmed: false`), or `[/ ] unable  <name>  <reason>  <detail>` with its hint (exit 10/12/13/14),
exit 21 with its cause's hint, exit 20 for a wrapper fault. The longest delivered send is 53.5 s, 53 times the
console's one-second re-read. So the action must run off the key loop, and the keys must stay live while it runs.
STATUS is never the readback (carry cry-qjw64paz).

### M3 — the refusals an action can print, by census
- Exit 21's six causes, one hint each (`UnreachableCause`; `tests/cross_exit_causes.rs`
  `no_two_codes_share_a_hint`); the frame verbs' strict-modes instance pair.
- send's not-delivered details (control-character · empty-text · turn-running · input-not-ready · paste-cut ·
  no-prompt-submitted) and human-typing · unverified-cli · unknown.
- answer's human-typing · unverified-cli · unknown-dialog; switch's exit-13 details (turn-running · dialog-pending ·
  no-session · child-not-ended · resume-failed); focus's six exit-1 codes; open's seven `--json` codes; up's four.
- The census is `tests/contract_error_bodies.rs` plus `tests/cross_exit_causes.rs`. A console that draws a verb's
  refusal draws the screened words its child printed, so it adds no error body of its own. A new console line
  showing them is a new human writer, which the census must name.

### M4 — what «no key trap» asks, by its a11y rows
**No row of any master states a no-key-trap rule.** Searched all seven masters for `key trap`, `trap` and `2.1.2`:
0 key rows (the 6 `trap` hits are `decision/self-bootstrapping-test-data` and two siblings, about bootstrapping).
SC 2.1.2 is owed by the route entry «Fleet a11y verification» (its hint names SC 2.1.1, 2.1.2, 1.4.1, 2.4.7, 3.2.4).
What stands today is design-system `contract/fleet-console-keys/q` · `/ctrl-c` · `/unbound`: `q` and Ctrl-C end the
console with exit 0, and every other key is ignored. The working reading: every mode an action adds is left by one
key, and Ctrl-C ends the console from every mode.

### M5 — what the key decoder must read
Today (`src/console/keys.rs:21-29`) there is one class per read of at most 64 bytes, and an arrow `\x1b[A` reads
`unbound` (its case, line 78). For any key beyond `q`/Ctrl-C the decoder must:
- read escape sequences (arrows CSI `A`/`B`/`C`/`D`, Home/End/PgUp/PgDn), whether split across reads or several in
  one read;
- tell a lone ESC from a sequence's start, by a short wait;
- if a text entry exists, take a paste: the console enables no bracketed paste today (its enter bytes are
  `\x1b[?1049h\x1b[?25l`, `src/console/mod.rs:222`), so a paste arrives as raw bytes, its newlines as CR.

A resize is not a byte: the size is polled every 100 ms (`src/console/mod.rs:29,197`). The console enables no focus
or mouse reporting, so no terminal reply arrives unasked. The decoder keeps the floor of security-plan
`decision/console-key-reader-bounded`: reads of at most 64 bytes, closed classes, no key byte on any line.
The driver feeds bytes into the console's own PTY (`tests/support/outer_pty.rs`) and reads `console-key` words and
line mode (`tests/tui_console.rs:39-143`): state, never the screen. A split sequence is fed as two writes.

### M6 — what a text entry for a send needs, and what exists
Nothing exists: no line editor in the tree, and none allowed by the masters' recorded searches (a11y
`decision/cli-no-interactive-input`'s evidence: no rustyline · dialoguer · inquire · reedline · ratatui · crossterm).
An entry needs:
- an editing line (printable chars, Backspace, Enter to send, ESC to cancel), a bound (`MAX_FRAME`, the send
  verb's own cap; a smaller console cap is a choice), and the text handed unchanged to `viola send` on its stdin;
- a rule for a paste with newlines: a multi-line send, or a cut at the first CR.

The verb validates the text: control-character is refused, never stripped; empty-text; trailing CR/LF dropped. The
console must not trim or rewrite the text (design-system `contract/send-readback/prompt`;
`decision/refused-text-never-stripped`).

## Rows reached (searched: `viola console|fleet console|console's` over all seven masters — 65 rows; plus `acts on no strip|board-vs-strip|list-is-the-board`, `human-controls-cli-only|no-shell-out`, `key trap|trap|2.1.2`)
**Reached — each must be reworded, excepted or extended for the actions:**
- design-system `decision/list-is-the-board` — «`verify`, `plugin install` and `console` act on no strip»: a console
  acting on a selected strip contradicts it (product choice F1/F2).
- layout-templates `decision/board-vs-strip` — «`console` name[s] no strip»: the same.
- `src/console/mod.rs:5-6` (code, no row) — «sends no channel frame and calls no compositor»: a child verb does both
  on the console's behalf (F1).
- architecture `decision/no-shell-out` — holds: the children are spawned directly. Not reached, read as holding.
- a11y `decision/cli-no-interactive-input` and `decision/cli-not-keyboard-interactive`, obs-plan
  `decision/no-interactive-terminal-interface`, a11y `decision/static-linear-output` — each except names the console
  reading keys and redrawing on a resize or a fleet change. A selection move, a text entry, a confirm or an action's
  result line is a further widening of each except (F4, F5).
- design-system `contract/fleet-console-keys/unbound` — «every key other than q and Ctrl-C is unbound and ignored»:
  amended by every key bound (F3); `/q` and `/ctrl-c` kept.
- security-plan `decision/console-key-reader-bounded` — the closed three classes: amended to the sequence decoder's
  classes, keeping the 64-byte bound and «no key byte on any line».
- obs-plan `contract/diag-line-fields/console-key` — its closed words quit · interrupt · unbound grow by each bound
  key; `schemas/diag-line.v1.json` with it.
- layout-templates `decision/console-frame-floor` — «`q quit` on the last row»: the key line grows.
- design-system `contract/fleet-console-view/more-line` and the view carry — no scroll is built (F6).
- obs-plan `contract/console-output/stdout` — stdout carries only the screen. An action's child output is captured,
  never passed through: holds if the console draws the captured lines on its own screen.
- design-system `decision/fleet-console-not-verb-output` and layout-templates `decision/no-prompt-or-pager` — the
  console's prompts sit on its own surface, and the verbs stay prompt-free: holds, read as allowing F4/F5 inside the
  console.
- security-plan `decision/console-agents-child-outlives-quit` — the precedent: an action child running at a quit needs
  its own accepted row (F1's cost).
- architecture `decision/human-controls-cli-only` — release and pause stay off MCP: the console is no MCP surface.
  Holds.
- test-plan `decision/tests-never-read-host-sessions`, `decision/no-untied-viola-spawn`, `decision/run-under-test-owned-user-home`
  — bind every action test: the test-scoped PATH, the tied builder, a test-owned HOME for a start.

**Read as not reached:**
- architecture `decision/cli-human-text-json-exit-codes` and `decision/new-capabilities-as-verbs` — the console stays
  `--json`-free.
- layout-templates `contract/cli-verbs/console` — no argument, exit set unchanged.
- security-plan `contract/strict-modes-refusal/console` and the mid-run refusal rows — unchanged. A child's
  strict-modes refusal is that verb's own exit 21, drawn as its line.
- obs-plan `decision/console-agents-probe-volume` — action lines are human-paced, one per key.
- `decision/human-output-escapes-controls` / `decision/escape-controls-in-human-output` — hold. Every drawn child
  line is escaped like a frame cell.

## The playbook's escalate rows (read before this block)
`rul-r74hshif` (boundary widening) · `rul-p2zl2kgl` (what ratifies one: his live answer after it was shown) ·
`rul-o2tr55aj` (held items landed on his word) · `rul-bznresq5` (a widening he answered himself at a P4 fork). F1 is
of the first class. Only his own answer, given after he reads F1 here, ratifies it, never a delegate's. The wrap then
halts on it once as his answer card.

## Forks — each his

### F1 — The console runs each action as a child `viola` process (BOUNDARY WIDENING, `rul-r74hshif`)
Today the console sends no frame, calls no compositor and spawns only its `claude agents` read. Acting «through the
CLI verbs» means it spawns `<its own exe> --home <home> <verb> <name> …`, directly, no shell, tied to its death. The
child writes the verb's records, the channel frame and, for focus or open, the compositor call.
1. **(Recommended)** A direct child per action, its own canonical exe, `--home` passed, `VIOLA_NAME`/`VIOLA_DIR`
   removed from its env (so a release is never refused `release-from-driver`, and no link is recorded from the
   console), stdout and stderr captured through `take(MAX_FRAME)` and drawn escaped on the console's screen, one child
   at a time. Cost: one new subprocess crossing, one accepted residual (a child running at a quit, like the agents
   child), about 6 rows reworded (M-rows above), no new channel method, no new error body.
2. The same with `VIOLA_NAME` kept: a console started in a wrapped session sends as that session. Cost: its release
   is refused and its sends record links, so the console acts as a driver, not the human.
3. No actions in this version: the console stays a view, and the actions are a later version's. Cost: v2-15, v2-28's
   «driven from one console» and v2-31's console leg stay unmet in 0.2.0.

### F2 — Which verbs this entry offers
1. **(Recommended)** A first set of the one-row verbs that need no new input: last, pause, release, focus and revive
   (as `viola up <name>`). Then send and switch, with the text entry and choice of F4. answer, open a child and adopt
   go to a later entry. Cost: v2-15 is not claimed by this entry (its acceptance names all eight); v2-31's console
   leg can be.
2. All eight of v2-15 (open, focus, send, answer, last, pause, release, revive) plus switch in this entry. Cost: the
   answer's response builder per dialog kind and the open's name entry, about twice the work; likely past one
   implement window (F7).
3. Only send, the entry's named readback. Cost: «every verb by keyboard» unmet.

### F3 — The keys and the selection
1. **(Recommended)** Up/Down arrows and `j`/`k` move the selection over the wrapped rows in the board's forest order.
   Unwrapped rows are skipped: no verb takes their name. The selection is named by the board item's name. One lower-case
   letter per verb on the key line (`s` send · `l` last · `p` pause · `r` release · `f` focus · `v` revive · `m`
   switch). `q` and Ctrl-C still end the console from the list. When the fleet changes, the selection stays on the
   same name; if that name is gone, it moves to the nearest row in order. Cost: the key line grows past `q quit` (one
   row amended), and each key is a new `console-key` word.
2. The same keys but number keys `1`–`9` pick a row. Cost: rows past nine are unreachable, and the order shifts on
   every fleet change.
3. A command line (`:send builder`) in place of keys. Cost: a text entry for every action, and a typed name, which
   the selection exists to avoid.

### F4 — How a send's text is entered
1. **(Recommended)** One editing line on the key row: printable characters, Backspace, Enter sends, ESC cancels. The
   text is handed unchanged to `viola send`'s stdin. The bound is the verb's `MAX_FRAME`, and the verb's own refusals
   are drawn. A paste's CR ends the line: one line per send. Cost: one widening of the four «except» rows (a text
   entry), and no multi-line send from the console.
2. A multi-line entry, Enter a newline and Ctrl-D sends. Cost: a second key-trap reading, a larger surface, and a CR
   policy for the verb's typed-text normalisation.
3. Open `$EDITOR` on a temp file. Cost: a shell-free editor spawn with a file under the home, and an environment read
   (`EDITOR`) no master admits: a further widening.

### F5 — Whether a wheel-moving or destructive action asks a confirm, and in what words
1. **(Recommended)** No confirm for any first-set verb. Each is one key from a named row and its result line names
   that row. release and pause move the wheel back and forth, and revive refuses a live session itself. Cost: a
   mistyped key acts at once.
2. A one-key confirm for release and revive only: `release <name>? y/n`. Cost: one more mode to leave (Esc or `n`)
   and new console copy, pinned by literal.
3. A confirm for every action. Cost: every action takes two keys.

### F6 — Whether the view scrolls to reach a hidden row
1. **(Recommended)** The body follows the selection: when the selection moves past the body rows, the window moves
   by one row. `... <n> more` stays as the line for the rows below, plus a new `... <n> above` line. Cost:
   `contract/fleet-console-view/more-line` amended and one new row.
2. No scroll: only the rows on screen are selectable. Cost: a fleet taller than the terminal cannot be acted on.

### F7 — Whether one implement window holds the entry
1. **(Recommended)** Split it into two entries. This one: the sequence decoder, the selection, F6, the child runner
   (F1), and last · pause · release · focus · revive (as up). «Console send and switch»: the text entry, send's
   readback, switch (v2-31's leg). answer and open a child stay on a third entry or the second. Cost: two route
   entries inserted after this one, placed on his word.
2. One entry for the whole of F2's chosen set. Cost: likely past one implement window.

## Technical (the overseer's to answer at once; listed, not his)
- The decoder's shape (a byte-level state machine with a 50 ms lone-ESC wait), the action runner's thread (one child
  at a time on its own thread, the key loop live), the module layout (`src/console/action.rs`), the test homes (each
  action case under a test-owned home, the test-scoped PATH, the tied builder, fake-agent instances only), and whether
  an action adds a `console-action` event or rides `console-key` alone.

---

# The P4 block — 2026-11-28T14:40:31Z (after his answers, `inputs#I2`)

His answers to F1–F7 are folded into the scope (rev 4). Per the relay's item 3, the input forms of answer, open a
child and adopt were measured before planning:

- **open a child** needs a NEW name. The selected row supplies only `--parent`; the rest stays open's own default:
  the model and effort from config, the workspace from the parent's window, the cwd from the console's own folder
  (`src/cmd/open.rs:40-69,132`). Form, planned as the lean: `o`, then the child's name on the one editing line,
  Enter runs `viola open <name> --parent <selected> -- claude`. No new mode beyond F4's line.
- **adopt** is not planned. His F2 answer names v2-15's eight verbs plus switch, and adopt is not among them. Adopt
  also takes an unwrapped session's id, and his F3 skips unwrapped rows. The relay's `d` key is not taken up.
- **switch** (F3's `m`) is planned as the lean: one or two words on the editing line. A word that is one of the five
  effort words is `--effort`, any other word is `--model`, and the verb refuses what it refuses (exit 2 on a bad
  word).
- **answer** needs his word: Q8 below. A dialog's id and kind are in the snapshot (`crates/viola-state/src/snapshot.rs:56,107`,
  read into the board at `crates/viola-state/src/board.rs:375-384`, the kind only). A question's text and options
  are only in the instance's `events.ndjson` (`crates/viola-agent-claude/src/dialog.rs:150-166`).

Part 1 and Part 2's send, switch and open are planned meanwhile. Answer's steps are held behind Q8.

## Q8 — How the console answers a pending dialog (key `a` on a row whose DIALOG is pending)
Rows reached (searched `option text`, `question text`, `dialog-text-never-logged` over all seven masters):
- architecture `decision/option-text-never-in-human-line` — «Question option text is upstream content: carried into
  the event log, `hook.dialog`, CLI `wait --json` and the MCP `wait`, never into a human line». Arm 1 reaches it.
- security-plan `decision/question-option-text-crosses-boundary` — the ratified crossing names those four sinks and no
  fifth. Arm 1 adds the console screen.
- obs-plan `decision/dialog-text-never-logged` — holds in every arm: no text reaches a line, and `console-action`
  carries the verb and the exit code alone.
- security-plan `decision/upstream-text-is-never-a-command` — holds: the console draws the text and never acts on it.

The options:
1. All three kinds by keys. A permission takes `y` allow / `n` deny, and a plan `y` approve / `n` revise. A question
   draws its text and numbered options (escaped) in the body, and a digit picks an option; on a multi-select question
   digits toggle and Enter sends. Esc cancels. Cost: a boundary widening of the two rows above, since option text
   reaches a human line on the console. Also a new reader of the instance's events log in the console, a new mode,
   and a question with more than 9 options cannot be answered past the ninth.
2. **(Recommended)** Permission and plan by `y`/`n`, as in arm 1. A question is left to the human in the session:
   `f` focuses it, and `a` on a question draws one line, `<name>  question - answer it in the session (f)`. Cost:
   v2-15's «answer a pending dialog» is met for permissions and plans only. Its claim would be narrowed by a dated
   note, or left unclaimed, at his word. No widening. The console reads the dialog's id from the snapshot through
   the board (one board field more, kept out of `list --json`).
3. No answer from the console in 0.2.0. Cost: v2-15 cannot be claimed. Its answer leg goes to a later entry he
   names.

## Q9 — How the editing line shows text that is not ASCII (a send in Russian, say)
`decision/console-own-surface` (layout-templates) reads «in plain ASCII with no colour», and the console escapes
controls (design-system `decision/escape-controls-in-human-output`). The text handed to `viola send` is unchanged in
every arm (F4).
1. **(Recommended)** The editing line echoes the typed text as typed, UTF-8, with controls escaped. This is a named
   exception to «plain ASCII» for the editing line alone. Cost: one row excepted, and a wide character's column
   width read by a width rule no master states yet (one row added).
2. The echo escapes every non-ASCII character (`\u{...}`). Cost: a Russian text is unreadable while typed, and no row
   moves.
3. The echo shows `?` per non-ASCII character. Cost: the same, though shorter on screen.
