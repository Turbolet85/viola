# layout-templates · the rows that bind 2026-11-19-a-fired-scheduled-wake-keeps-the-wheel — picked 60 of 223 (35 by every draw) · rule any · pin 3197

## surface-cli · ## Surface: cli
decision/no-progress-glyphs · No send progress glyphs — A send shows no `Sending…` spinner, no `✓` and no `done`.
decision/no-terminator-word · No terminator word — No `done.`, `success` or summary banner; the exit code is the terminator and the last printed line says what happened, to whom and where.
decision/readback-mirror-signature · Readback mirror as the send signature — The readback mirror `[RB]` / `[  ]` / `[/ ]` is the first column of viola send output, in an issue line and an outcome line.
decision/send-lines-appended-never-redrawn · Send lines appended, never redrawn — viola send lines are appended and never redrawn: the issue line stays and the outcome is printed under it.
decision/single-progress-pattern · Single progress pattern — The step counter of viola verify, which stamps a CLI version, is the project's only progress pattern.
decision/spoken-phraseology-verb-output · Spoken phraseology for wheel, handoff and dialog verbs — The output of viola pause, release, link, unlink and answer is spoken phraseology.
decision/stdout-results-stderr-rest · Results on stdout, the rest on stderr — Results go to stdout; `waiting:`, the issue line, refusals, hints and errors go to stderr, and the two are never mixed.
decision/turn-ended-not-send-outcome · turn-ended is not a send outcome — `turn-ended` is never reported as a send outcome; it is the result of `viola wait`.
contract/cli-exit-codes/10 · CLI exit codes · 10 — `human-typing`.
contract/cli-exit-codes/json-form · CLI exit codes · --json form — Under `--json` the same outcome is one document on stdout, with no stderr line and no hint text.
contract/cli-json-mode/one-document · CLI --json mode · one document — Under --json each verb prints exactly one document on stdout (`{"v":1,"ok":…}`, or the exit-21 / wrapper-fault document) and nothing on stderr.
contract/cli-verbs/list · CLI verbs · viola list — The board: every session as one strip row, with the BAY context header.
contract/cli-verbs/run · CLI verbs · viola run <name> -- claude — Passthrough; on a verified CLI the start writes the claude CLI's trust record for an untrusted session folder, flag only, so no trust dialog stands and no key is typed; prints nothing while the child runs.
contract/cli-verbs/send · CLI verbs · viola send <target> — Types at a turn boundary and reports read back, unable or unconfirmable.
contract/cli-verbs/wait · CLI verbs · viola wait <target> — Returns one turn-boundary result, with no spinner.
contract/cursor/byte-offset · cursor · cursor — `cursor` is the byte offset `send` prints and `wait --after` consumes.
contract/readback-mirror/layout · the readback mirror line · layout — A fixed first column holding the four-character box (`[RB]`, `[  ]`, `[/ ]`), one space, the padded state word (`open` / `read back` / `unable` / `unconfirmable`), the two-space gutter, NAME, then the outcome fields.
contract/readback-vocabulary/words · readback vocabulary · state words — The readback vocabulary is `open`, `read back`, `unable`, `unconfirmable`, and the CLI brackets the box.
contract/send-json/output · viola send --json · typed document — `--json` prints only the typed document (`ok` with `submitted_at` and `cursor`, `ok` with `confirmed:false` and `unconfirmable`, or `refusal` with `detail`) with the typed exit code, no glyph and no hint.
contract/viola-answer-output/answered · viola answer output · answered — Prints `answered  <name>  dialog <id>`.
contract/viola-list-board/wheel-unknown · viola list board · WHEEL unknown — A replayed row with no logged holder reads WHEEL `unknown`.
contract/viola-list-json/document · viola list --json · document — One JSON document on stdout with no header line, no colour and no glyph; its item and skipped shapes are the list payload contract.
contract/viola-send-output/json-mode · viola send output · --json — Under --json the mirror is replaced by the typed ok / refusal document.
contract/viola-send-output/json-ok · viola send output · --json ok — Under --json a confirmed send prints `{"v":1,"ok":{"submitted_at":<ts>,"cursor":<n>}}` on stdout, with no glyph and no issue line.
contract/viola-send-output/piped-stdout · viola send output · piped stdout — When stdout is piped the TTY-only issue line is dropped and the outcome line alone carries the mirror.
contract/viola-send-output/read-back · viola send output · read back — `[RB] read back …` prints on stdout with exit 0 when the send is confirmed by a prompt-submitted whose prompt equals the typed text, or equals it with a trailing run of space, TAB, CR and LF removed; any leading or inner difference leaves the send unconfirmed.
contract/viola-send-output/unable · viola send output · unable — `[/ ] unable …` prints on stderr with exit 13 (or 10/12/14) when the wrapper refuses, or exit 13 when the client refuses the text before any frame (not-delivered with control-character or empty-text).
contract/viola-send-output/word-column · viola send output · word column — The word column is padded so open, read back, unable and unconfirmable line up, and the name always starts in the same column.
contract/viola-verify-output/exit-0 · viola verify output · exit 0 — Exit 0 when every relied row counts: it reads pass, or absent on the browser-tools-prompt row alone.
contract/viola-verify-output/exit-1 · viola verify output · exit 1 — Exit 1 on a failing row, a refusal (the unable: / hint: pairs) or a stop.
contract/viola-verify-output/failing-row · viola verify output · failing row — A run with a failing row exits 1 and still writes the stamp.
contract/viola-verify-output/step-line · viola verify output · step line — One stdout step line per relied ledger row, `[NN/MM] <row id> <row words>  pass|absent|fail`, in ledger order, MM being the relied count; `absent` is the browser-tools-prompt row's word alone, when no typed run met the prompt; each appended once, with no progress bar and no redraw.
contract/viola-verify-output/summary · viola verify output · summary — `stamped <ver>  <n> pass  <m> fail` is the last stdout line, `n` counting the rows that count, an `absent` browser-tools-prompt row among them; `fail` is never coloured.
contract/viola-verify/summary · viola verify output · summary line — `stamped <version>  <n> pass  <n> fail` closes a completed run; a run stopped before its rows are checked prints no summary.
contract/viola-wait-output/dialog · viola wait output · dialog — A dialog result prints `<kind>  <name>  dialog <id>  cursor <n>` on stdout with DIALOG not coloured, as a result and not the board.
decision/argument-order-mirrors-channel · Argument order mirrors the channel params — CLI argument order mirrors the channel `params`: `viola <verb> <target>` for one strip, `viola link|unlink <driver> <driven>` for a handoff, `viola answer <target> <dialog_id>`.
decision/board-ordered-by-name · Board ordered by name — viola list rows are ordered by name and nothing is re-sorted by state; NAME values are bold.
decision/dialog-attention-only-on-board · DIALOG attention word only on the board — The amber DIALOG attention word appears only in viola list.
decision/hint-last-on-stderr · Hint is the last stderr line — On a refusal, the `hint:` line is the last line on stderr.
decision/human-output-is-contract · Human output is a contract — The human columns and words are stable; new fields go into `--json` first, and adding a human column is a design change, not an implementation choice. · why: LLM drivers may read the human output.
decision/json-on-every-data-verb · --json on every data verb — `--json` is the machine view of every data verb, `viola revive` included in both its refusal document and `--list`.
decision/no-banner · No banner anywhere — Normal output has no ASCII-art logo, no tagline and no version line; the version appears only in `--version` and `--help`, and `viola hook` and `viola mcp` have no human surface.
decision/plain-output-modes · Plain output modes — Under --json, a non-TTY, NO_COLOR, TERM=dumb and the viola run / viola revive passthrough the cli prints no colour, no non-ASCII glyph and no cursor control.
decision/readback-mirror-no-colour · Readback mirror has no colour — The readback mirror is words and brackets only: `read back` is never green and `unable` is never red. · why: Colour on the CLI is reserved for `DIALOG` and `stale`.
decision/shared-refusal-word-column · Shared refusal word column — The word half of the struck box (`unable` plus `reason  detail`) is the first-word column of every other verb's refusal line, while the struck glyph itself belongs to viola send alone.
decision/words-carry-every-state · Words carry every state — Piped viola list output is the same characters with no SGR, and nothing is lost because the words carry every state.
decision/wrapper-repeats-send-refusals · Wrapper repeats the send text refusals — The wrapper makes the same control-character and empty-text refusals for a frame sent straight to it as `viola send` makes itself.
contract/cli-exit-codes/12 · CLI exit codes · 12 — `unverified-cli`.
contract/cli-exit-codes/13 · CLI exit codes · 13 — `not-delivered`.
contract/cli-refusal-line/form · CLI refusal line · line form — `unable  <name>  <reason>  <detail>` on stderr, fields separated by two spaces; `viola send` pads `unable` into the mirror's word column after `[/ ]`.
contract/readback-mirror/issue-and-outcome · the readback mirror line · issue and outcome lines — The TTY issue line on stderr is the open box and the outcome line under it is the closed box, the two lined up column for column.
contract/viola-answer-output/unknown-dialog · viola answer output · unknown-dialog — An answer to a dialog that is not pending prints `unable  <name>  not-delivered  unknown-dialog` on stderr, exit 13, then `hint: that dialog is not pending; viola list shows the current DIALOG`.
contract/viola-list-board/column-widths · viola list board · column widths — NAME is padded to the longer of 4 and the longest name over both racks in characters, LIVE to 5, STATUS to 7, WHEEL to 23, DIALOG to 17, CLI last and unpadded; no line ends in a space.
contract/viola-list/columns · viola list · columns — NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, each padded to its widest word, with the two-space gutter between columns and no box-drawing borders.
contract/viola-run/start-output · viola run start · passed start output — `viola run` prints nothing of its own, and the start's trust step adds no refusal, exit code, hint, `--json` code or terminal byte; the terminal belongs to the wrapped claude TUI until it exits.
contract/viola-send-output/input-not-ready-hint · viola send output · input-not-ready hint — An unable not-delivered input-not-ready outcome ends stderr with `hint: <name> was not ready for input; send again, and if it repeats a human must look at the session`.
contract/viola-send-output/issue-line · viola send output · issue line — `[  ] open …` is printed on stderr, only on a TTY, when the request goes to the wrapper.
contract/viola-verify-output/plain-ascii · viola verify output · plain ASCII — All viola verify output is plain ASCII with no colour.
contract/viola-verify-output/record · viola verify output · --record — --record adds no human line.
contract/viola-wait-output/dialog-expired · viola wait output · dialog-expired — A wait woken by the wrapper's record of an expired hold prints `dialog-expired  <name>  dialog <id>  cursor <n>`, ASCII, no SGR, written once.

# layout-templates · its absences, names and overviews, whole

## surface-web-spa · ## Surface: web-spa
absence/web-surface — A web surface: viola renders no page

## surface-cli · ## Surface: cli
absence/cli-deep-links — Deep links or URL coupling in the CLI
