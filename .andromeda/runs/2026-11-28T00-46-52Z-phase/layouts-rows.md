# layout-templates · the rows that bind 2026-11-28-console-scaffold — picked 45 of 238 (31 by every draw) · rule any · pin 5229

## surface-cli · ## Surface: cli
decision/amber-means-dialog · Amber means DIALOG only — Amber means `DIALOG` and nothing else, and `stale` is SGR dim in the terminal.
decision/clap-derive-cli-parsing · clap derive for CLI parsing — The viola command line is parsed with clap 4 in its derive form.
decision/flat-verbs · Flat verbs plus a target — Each verb is one lower-case word with no nested noun hierarchy or subcommand tree. · except: `plugin install` is the one two-word verb.
decision/hand-written-sgr-styling · Hand-written SGR styling module — Terminal styling and fixed-width column padding come from a small hand-written SGR module in the viola binary, with no colour or table crate.
decision/no-banner · No banner anywhere — Normal output has no ASCII-art logo, no tagline and no version line; the version appears only in `--version` and `--help`, and `viola hook` and `viola mcp` have no human surface.
decision/no-persistent-footer · No persistent footer — The CLI keeps no persistent footer: terminal scrollback is the history and the tape lives in `events.ndjson`.
decision/no-progress-glyphs · No send progress glyphs — A send shows no `Sending…` spinner, no `✓` and no `done`.
decision/no-prompt-or-pager · No interactive prompt or pager — No verb has an interactive prompt or a pager, and the terminal fleet console is no verb's output.
decision/no-spinner · No spinner — The cli shows no spinner and no elapsed-time counter.
decision/one-token-vocabulary-on-cli · One token vocabulary resolved on the cli — The one design token vocabulary (color-*, space-*, radius-*, the typography roles) resolves on the cli to the SGR tokens, the terminal default foreground or not used, through ANSI depth, the two-space gutter, ASCII `->` / ` - ` and brackets for the ruled box.
decision/plain-output-modes · Plain output modes — Under --json, a non-TTY, NO_COLOR, TERM=dumb and the viola run / viola revive passthrough the cli prints no colour, no non-ASCII glyph and no cursor control.
decision/send-lines-appended-never-redrawn · Send lines appended, never redrawn — viola send lines are appended and never redrawn: the issue line stays and the outcome is printed under it.
decision/single-progress-pattern · Single progress pattern — The step counter of viola verify, which stamps a CLI version, is the project's only progress pattern.
decision/std-isterminal-tty-detection · TTY detection through std IsTerminal — Whether a stream is a terminal is detected with the standard library's IsTerminal, with no crate.
decision/stdout-results-stderr-rest · Results on stdout, the rest on stderr — Results go to stdout; `waiting:`, the issue line, refusals, hints and errors go to stderr, and the two are never mixed.
decision/words-carry-every-state · Words carry every state — Piped viola list output is the same characters with no SGR, and nothing is lost because the words carry every state.
contract/cli-exit-codes/json-form · CLI exit codes · --json form — Under `--json` the same outcome is one document on stdout, with no stderr line and no hint text.
contract/cli-global-flags/home · CLI global flags · --home — `--home` is the same on every verb.
contract/cli-internal-error/human · CLI internal error · human output — A failure or a caught panic prints `error: internal error` once, exit 1, with no hint.
contract/cli-internal-error/json · CLI internal error · --json output — Under --json a failure or a caught panic prints `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty, exit 1.
contract/cli-json-mode/one-document · CLI --json mode · one document — Under --json each verb prints exactly one document on stdout (`{"v":1,"ok":…}`, or the exit-21 / wrapper-fault document) and nothing on stderr.
contract/cli-token-rendering/color-accent · CLI token rendering · color-accent — The design system's attention SGR token at truecolor, 256 or 16-colour depth, set only on the DIALOG word and always followed by the uncoloured kind word.
contract/cli-token-rendering/color-surface · CLI token rendering · color-surface-* — Never set; the terminal background belongs to the user.
contract/cli-token-rendering/color-text-primary-secondary · CLI token rendering · color-text-primary / color-text-secondary — The terminal's default foreground; the terminal owns ink.
contract/cli-token-rendering/motion-tokens · CLI token rendering · Motion tokens — None; nothing redraws in place.
contract/cli-token-rendering/radius · CLI token rendering · radius-* — No terminal analogue; square brackets `[ ]` keep the ruled-rectangle reading.
contract/cli-verbs/help · CLI verbs · viola --help — The grouped verb table with the served verbs only: the board, traffic, wheel, handoff and setup rows, `board:` holding list and focus, `handoff:` holding link, unlink and `setup:` holding open, run, revive, adopt, up, switch, verify and plugin install.
contract/viola-help/verb-groups · viola --help output · verb group rows — Five rows `board:    list, focus`, `traffic:  send, wait, last, answer`, `wheel:    pause, release`, `handoff:  link, unlink` and the setup row, served verbs only, plain ASCII, then a blank line and `Options:`; no about line and no clap `Commands:` list.
contract/viola-list-board/uncoloured-modes · viola list board · uncoloured modes — The board is uncoloured under --json, a piped stdout, NO_COLOR (an empty value included) or TERM=dumb.
contract/viola-plugin-install/internal-error · viola plugin install · internal error — A failed exe read, dir creation or file write is `error: internal error`, exit 1, or under `--json` `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty.
contract/viola-verb-help/usage · viola <verb> --help output · per-verb usage — `viola <verb> --help` gives clap's per-verb usage unchanged, its usage line naming the program by its executable.
decision/board-vs-strip · Board versus strip verbs — `viola list` shows the whole bay, `viola up` re-opens the whole fleet or the instances it names, `verify` and `plugin install` name no strip, and every other verb a person runs acts on one strip named by its `ViolaName`.
decision/cli-holds-every-action · The CLI holds every action — The CLI is where every action lives.
decision/dialog-attention-only-on-board · DIALOG attention word only on the board — The amber DIALOG attention word appears only in viola list.
decision/human-tty-expression-level · Human TTY expression level — On a human TTY the cli prints static fixed-column rows and one static context header, with colour only as a second cue.
decision/only-header-bay-line · Only header is the BAY line — The only header on any verb is the one static BAY line at the top of `viola list`.
contract/cli-exit-codes/11 · CLI exit codes · 11 — Retired (was `budget-paused`), never reused; no refusal maps to 11.
contract/cli-token-rendering/spacing · CLI token rendering · Spacing — The column gutter is two spaces on the terminal grid; inside one field ` - ` is the separator.
contract/run-revive-start-json/internal-error · viola revive --json start refusals shared with run · internal-error — A program not found or an error of the start itself prints one exit-1 document with this code, stderr empty; a child-pump error prints no document, the child having held the terminal.
contract/viola-help/setup-row · viola --help output · setup row — The `setup:` verb group row prints exactly `setup:    open, run, revive, adopt, up, switch, verify, plugin install`, `adopt` after `revive` and `switch` after `up`.
contract/viola-list-board/colour-depth · viola list board · colour depth — On a terminal that takes colour, the depth is truecolor, 256 or 16, read from COLORTERM / TERM.
contract/viola-list-board/name-truncation · viola list board · NAME truncation — With the terminal width known (from the host pty size, never COLUMNS), an unwrapped NAME longer than the column (capped at the width less 65) is cut to the column less 3 plus `...`, never inside a `\xHH` escape; a wrapped NAME is never cut.
contract/viola-verify-output/step-line · viola verify output · step line — One stdout step line per relied ledger row, `[NN/MM] <row id> <row words>  pass|absent|fail`, in ledger order, MM being the relied count; `absent` is the browser-tools-prompt row's word alone, when no typed run met the prompt; each appended once, with no progress bar and no redraw.
contract/viola-verify-output/summary · viola verify output · summary — `stamped <ver>  <n> pass  <m> fail` is the last stdout line, `n` counting the rows that count, an `absent` browser-tools-prompt row among them; `fail` is never coloured.
contract/viola-verify/summary · viola verify output · summary line — `stamped <version>  <n> pass  <n> fail` closes a completed run; a run stopped before its rows are checked prints no summary.

# layout-templates · its absences, names and overviews, whole

## surface-web-spa · ## Surface: web-spa
absence/web-surface — A web surface: viola renders no page

## surface-cli · ## Surface: cli
absence/cli-deep-links — Deep links or URL coupling in the CLI
