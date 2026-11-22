# layout-templates · the rows that bind 2026-11-22-lineage-registry — picked 129 of 225 (99 by every draw) · rule any · pin 4060

## surface-cli · ## Surface: cli
decision/amber-means-dialog · Amber means DIALOG only — Amber means `DIALOG` and nothing else, and `stale` is SGR dim in the terminal.
decision/board-ordered-by-name · Board ordered by name — viola list rows are ordered by name and nothing is re-sorted by state; NAME values are bold.
decision/board-planning-forms-wording · Board planning forms as the ratified wording — The board's planning forms (bare skip counts, the empty board's three lines) are its wording. · why: confirmed as built by the founder, superseding the operator's word
decision/board-row-escaping · Board row escaping — Every board field derived from `claude agents --json` has C0 and C1 controls, newline and tab included, escaped as hex text, so a row never splits and the columns stay fixed-width. · why: Ratified; stricter than the security floor, which keeps newline and tab for `wait` and `last` text.
decision/board-vs-strip · Board versus strip verbs — `viola list` shows the whole bay, `verify` and `plugin install` name no strip, and every other verb a person runs acts on one strip named by its `ViolaName`.
decision/clap-derive-cli-parsing · clap derive for CLI parsing — The viola command line is parsed with clap 4 in its derive form.
decision/cli-field-separators · CLI field separators — ` - ` separates inside a CLI field and an arrow is `->`; there is no CLI inbound `<-` form.
decision/hand-written-sgr-styling · Hand-written SGR styling module — Terminal styling and fixed-width column padding come from a small hand-written SGR module in the viola binary, with no colour or table crate.
decision/hint-for-every-cause · A hint for every refusal cause — Directly under each refusal is `hint: <one plain instruction>`, keyed by reason or reason and detail, every cause with its own hint. · why: Exit 21 and exit 1 each have several causes, so an agent can tell the causes apart from the last stderr line. · except: No hint for the `unknown` refusal, for `wrapper fault` or for `internal error`.
decision/hint-last-on-stderr · Hint is the last stderr line — On a refusal, the `hint:` line is the last line on stderr.
decision/hints-name-next-verb · Hints name the next verb — `hint:` lines point to the next verb by name, as the CLI's breadcrumbs.
decision/hints-never-quote-or-locate · Hints never quote text or name paths — A hint never quotes the sent text or any upstream text, and never names a path or a pid.
decision/human-output-is-contract · Human output is a contract — The human columns and words are stable; new fields go into `--json` first, and adding a human column is a design change, not an implementation choice. · why: LLM drivers may read the human output.
decision/human-tty-expression-level · Human TTY expression level — On a human TTY the cli prints static fixed-column rows and one static context header, with colour only as a second cue.
decision/no-banner · No banner anywhere — Normal output has no ASCII-art logo, no tagline and no version line; the version appears only in `--version` and `--help`, and `viola hook` and `viola mcp` have no human surface.
decision/no-hint-names-release · No hint names viola release — Every hint is distinct and no hint names `viola release`; `release` is a human verb reached through `--help`. · why: The refusals that carry hints reach drivers, and a driver's `release` is refused.
decision/no-link-column · No link column on the board — The six shared board columns stay fixed; links are visible only in `link` / `unlink` verb output.
decision/no-persistent-footer · No persistent footer — The CLI keeps no persistent footer: terminal scrollback is the history and the tape lives in `events.ndjson`.
decision/no-prompt-or-pager · No interactive prompt or pager — No verb has an interactive prompt or a pager, and the terminal fleet console is no verb's output.
decision/no-spinner · No spinner — The cli shows no spinner and no elapsed-time counter.
decision/one-token-vocabulary-on-cli · One token vocabulary resolved on the cli — The one design token vocabulary (color-*, space-*, radius-*, the typography roles) resolves on the cli to the SGR tokens, the terminal default foreground or not used, through ANSI depth, the two-space gutter, ASCII `->` / ` - ` and brackets for the ruled box.
decision/plain-output-modes · Plain output modes — Under --json, a non-TTY, NO_COLOR, TERM=dumb and the viola run / viola revive passthrough the cli prints no colour, no non-ASCII glyph and no cursor control.
decision/shared-captions-fixed · Shared captions fixed — The six captions, words and slot order are fixed, and the terminal fleet console reuses them.
decision/shared-refusal-word-column · Shared refusal word column — The word half of the struck box (`unable` plus `reason  detail`) is the first-word column of every other verb's refusal line, while the struck glyph itself belongs to viola send alone.
decision/spoken-phraseology-verb-output · Spoken phraseology for wheel, handoff and dialog verbs — The output of viola pause, release, link, unlink and answer is spoken phraseology.
decision/std-isterminal-tty-detection · TTY detection through std IsTerminal — Whether a stream is a terminal is detected with the standard library's IsTerminal, with no crate.
decision/stdout-results-stderr-rest · Results on stdout, the rest on stderr — Results go to stdout; `waiting:`, the issue line, refusals, hints and errors go to stderr, and the two are never mixed.
decision/targets-are-wrapped-names · A CLI target is always a ViolaName — A CLI `<target>` is always a `ViolaName`, and unwrapped names are never valid targets.
decision/truncate-only-unwrapped-names · Truncate only unwrapped names — Only unwrapped NAMEs are cut, with `...`, only where the terminal width is known and too narrow, never inside an escape; wrapped names are never clipped and no line ever wraps.
decision/viola-list-styling · viola list styling — On a colour terminal NAME is bold, the `DIALOG` word is accent amber with its kind word plain, a `stale` row is dim end to end with `DIALOG` still amber, nothing else is styled, and no SGR is written when colour is off.
decision/violaname-never-case-transformed · ViolaName never case-transformed — `ViolaName` is a shared reference and is never case-transformed.
decision/words-carry-every-state · Words carry every state — Piped viola list output is the same characters with no SGR, and nothing is lost because the words carry every state.
contract/cli-exit-codes/21 · CLI exit codes · 21 — `unable  instance-unreachable`; for `viola list`, `viola plugin install` and the frame verbs' strict-modes refusal the fixed state pair instead.
contract/cli-exit-codes/json-form · CLI exit codes · --json form — Under `--json` the same outcome is one document on stdout, with no stderr line and no hint text.
contract/cli-json-mode/one-document · CLI --json mode · one document — Under --json each verb prints exactly one document on stdout (`{"v":1,"ok":…}`, or the exit-21 / wrapper-fault document) and nothing on stderr.
contract/cli-refusal-line/form · CLI refusal line · line form — `unable  <name>  <reason>  <detail>` on stderr, fields separated by two spaces; `viola send` pads `unable` into the mirror's word column after `[/ ]`.
contract/cli-token-rendering/callsign-role · CLI token rendering · Callsign role — The callsign token: SGR bold on NAME values only.
contract/cli-token-rendering/color-accent · CLI token rendering · color-accent — The design system's attention SGR token at truecolor, 256 or 16-colour depth, set only on the DIALOG word and always followed by the uncoloured kind word.
contract/cli-token-rendering/color-role-handoff · CLI token rendering · color-role-handoff — Not used; link output is plain.
contract/cli-token-rendering/color-text-muted · CLI token rendering · color-text-muted — The stale token: SGR dim across a whole stale row, and never without the word stale in LIVE.
contract/cli-token-rendering/color-text-primary-secondary · CLI token rendering · color-text-primary / color-text-secondary — The terminal's default foreground; the terminal owns ink.
contract/cli-token-rendering/label-role · CLI token rendering · Label role — Uppercase column captions (NAME LIVE STATUS WHEEL DIALOG CLI) and group labels, printed plain.
contract/cli-token-rendering/spacing · CLI token rendering · Spacing — The column gutter is two spaces on the terminal grid; inside one field ` - ` is the separator.
contract/cli-verbs/list · CLI verbs · viola list — The board: every session as one strip row, with the BAY context header.
contract/cli-verbs/revive · CLI verbs · viola revive <name> — Resumes a dead instance in place (its newest logged session, in the recorded directory, which also keys the trust record its start writes) and is passthrough like viola run.
contract/cli-verbs/revive-list · CLI verbs · viola revive <name> --list — Prints the instance's logged sessions, one static line each.
contract/cli-verbs/run · CLI verbs · viola run <name> -- claude — Passthrough; on a verified CLI the start writes the claude CLI's trust record for an untrusted session folder, flag only, so no trust dialog stands and no key is typed; prints nothing while the child runs.
contract/instance-unreachable-detail/strict-modes-failed · exit-21 --json detail · strict-modes-failed — One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
contract/link-output/already-linked · viola link / unlink output · already linked — A pair already held prints `link    <driver> -> <driven>  already linked`, exit 0.
contract/link-output/json-link · viola link / unlink output · link --json — Prints `{"v":1,"ok":{"since":<ts>,"already_linked":<bool>}}`.
contract/link-output/json-unlink · viola link / unlink output · unlink --json — Prints `{"v":1,"ok":{}}`.
contract/link-output/link · viola link / unlink output · link — Prints one stdout line in one write, plain in every mode, exit 0: `link    <driver> -> <driven>  since <HH:MM:SSZ>`, the reply's since as UTC clock time in whole seconds, a non-RFC 3339 since printed escaped.
contract/link-output/printed-names · viola link / unlink output · printed names — The names printed are the verb's own arguments, never names read back from the reply.
contract/link-output/unlink · viola link / unlink output · unlink — Prints `unlink  <driver> -> <driven>` in one write, exit 0, whether or not a pair was removed.
contract/link-output/wrapper-error · viola link / unlink output · wrapper error — A wrapper error, a driver equal to driven (-32602) included, is the wrapper-fault line, exit 20.
contract/run-revive-start-json/internal-error · viola revive --json start refusals shared with run · internal-error — A program not found or an error of the start itself prints one exit-1 document with this code, stderr empty; a child-pump error prints no document, the child having held the terminal.
contract/run-revive-start-json/pinned-hash-mismatch · viola revive --json start refusals shared with run · pinned-hash-mismatch — After a passed preflight, a pinned-hash mismatch prints one exit-1 document with this code, stderr empty.
contract/run-revive-start-json/socket-dir-refused · viola revive --json start refusals shared with run · socket-dir-refused — After a passed preflight, a refused socket directory prints one exit-1 document with this code, stderr empty; without `--json` the socket-directory pair.
contract/run-revive-start-json/squatted-name · viola revive --json start refusals shared with run · squatted-name — After a passed preflight, a squatted name prints one exit-1 document with this code, stderr empty.
contract/run-revive-start/home-tree-refused · viola run and viola revive starts · home tree refused — Before the log opens, a refused home, `ledger/`, `bin/`, `plugin/` or pinned copy prints `unable: the viola home is not private to you` / `hint: the viola home must be readable only by you; viola does not change its permissions`, a refused instance the state-files pair; exit 1, no log or child.
contract/run-revive-start/socket-dir-refused · viola run and viola revive starts · socket directory refused — Exit 1 before any snapshot, child or bind, stdout empty, directory left as found: `unable: the socket directory for <name> cannot be trusted` / `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
contract/viola-list-board/agents-read-failure · viola list board · failed agents read — A failed, oversize or malformed `claude agents --json` read prints the board as with no claude.
contract/viola-list-board/colour-depth · viola list board · colour depth — On a terminal that takes colour, the depth is truecolor, 256 or 16, read from COLORTERM / TERM.
contract/viola-list-board/column-widths · viola list board · column widths — NAME is padded to the longer of 4 and the longest name over both racks in characters, LIVE to 5, STATUS to 7, WHEEL to 23, DIALOG to 17, CLI last and unpadded; no line ends in a space.
contract/viola-list-board/empty-board · viola list board · empty board — An empty board prints `no wrapped sessions  start one: viola run <name> -- claude`.
contract/viola-list-board/name-truncation · viola list board · NAME truncation — With the terminal width known (from the host pty size, never COLUMNS), an unwrapped NAME longer than the column (capped at the width less 65) is cut to the column less 3 plus `...`, never inside a `\xHH` escape; a wrapped NAME is never cut.
contract/viola-list-board/plain-lines · viola list board · plain lines — The BAY line, caption, empty line and separator print plain.
contract/viola-list-board/single-write · viola list board · write order — viola list prints as one stdout write: the BAY line, the caption row, then one row per wrapped session in name order (or the empty line), then the unwrapped rack when there is one.
contract/viola-list-board/stale-row-dim · viola list board · stale row — A wrapped stale row is dim from its first byte to its last, NAME dim and bold.
contract/viola-list-board/stale-row-with-dialog · viola list board · stale row with a dialog — On a cocked and stale row, the DIALOG word stays amber and not dim, while the kind word and the rest of the row stay dim.
contract/viola-list-board/uncoloured-modes · viola list board · uncoloured modes — The board is uncoloured under --json, a piped stdout, NO_COLOR (an empty value included) or TERM=dumb.
contract/viola-list-board/unknown-width · viola list board · unknown terminal width — Nothing is truncated when the terminal width is unknown.
contract/viola-list-board/unwrapped-rack · viola list board · unwrapped rack — Unjoined agent rows print after the wrapped rows under `-- UNWRAPPED - READ-ONLY --`, sorted by name: NAME the escaped sessionId, LIVE live, its STATUS, n/a in WHEEL, DIALOG and CLI; with no unwrapped row no separator prints.
contract/viola-list-json/document · viola list --json · document — One JSON document on stdout with no header line, no colour and no glyph; its item and skipped shapes are the list payload contract.
contract/viola-list-json/strict-modes-failed · viola list --json · strict-modes-failed — A failed strict-modes check of the home, instances/ or any one instance prints `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with exit 21 and stderr empty, opens no log and leaves the home's mode as found.
contract/viola-list/columns · viola list · columns — NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, each padded to its widest word, with the two-space gutter between columns and no box-drawing borders.
contract/viola-list/strict-modes-refused · viola list · strict-modes refusal — Exit 21 refusing the whole board, stdout empty: the home pair for the home or `instances/`, the state-files pair for one instance.
contract/viola-list/unwrapped-separator · viola list · rack separator — `-- UNWRAPPED - READ-ONLY --` on its own line before the unwrapped rows, written only when unwrapped rows exist; the wrapped rack is always first, with no separator.
contract/viola-revive-json/already-live · viola revive --json error codes · already-live — The code for a live holder or a fresh replayed beat.
contract/viola-revive-json/cwd-missing · viola revive --json error codes · cwd-missing — The code for a missing recorded directory.
contract/viola-revive-json/no-session · viola revive --json error codes · no-session — The code for both no-logged-session pairs: no logged session, and no logged session with that id.
contract/viola-revive-json/refusal-document · viola revive --json · refusal document — Each refusal is one stdout document `{"v":1,"error":"<code>","detail":null}`, exit 1, empty stderr; without `--json` the pairs and rows stay byte-identical.
contract/viola-revive-json/session-live · viola revive --json error codes · session-live — The code for a session open in another claude.
contract/viola-revive-json/stale-heartbeat · viola revive --json error codes · stale-heartbeat — The code for a stale holder.
contract/viola-revive-json/strict-modes-failed · viola revive --json error codes · strict-modes-failed — The code for a refused home tree or instance state files.
contract/viola-revive-refusals/already-live · viola revive refusals · already live — `unable: <name> is already live` / `hint: viola list`, the same collision pair as `viola run`, with its stale sibling unchanged.
contract/viola-revive-refusals/directory-missing · viola revive refusals · recorded directory missing — `unable: <name>'s recorded directory is missing` / `hint: resume it by hand from a directory you choose: viola run <name> -- claude --resume <id>, ids from viola revive <name> --list`; no paths, no pids, no logged id.
contract/viola-revive-refusals/no-logged-session · viola revive refusals · no logged session — `unable: <name> has no logged session to resume` / `hint: start one: viola run <name> -- claude`.
contract/viola-revive-refusals/no-session-with-id · viola revive refusals · no logged session with that id — `unable: <name> has no logged session with that id` / `hint: viola revive <name> --list`.
contract/viola-revive-refusals/session-open-elsewhere · viola revive refusals · session open in another claude — `unable: <name>'s session is open in another claude` / `hint: close it there first, or start a copy: viola revive <name> --fork`.
contract/viola-revive-refusals/state-files-writable · viola revive refusals · state files writable by another user — `unable: <name>'s state files can be written by another user` / `hint: viola will not read them; make the viola home and its files owner-only`.
contract/viola-revive/list-cause · viola revive · --list cause field — The cause in a `viola revive --list` row is `startup`, `clear`, `resume` or `compact` as logged, and `unknown` otherwise.
contract/viola-revive/list-refusals · viola revive refusals · --list refusals — `viola revive --list` has two of the refusals: the state-files pair and the no-logged-session pair.
contract/viola-revive/list-row · viola revive · --list row — `viola revive <name> --list` prints one stdout row per logged session in log order, `<ts>  <cause>  <id>` with two spaces between fields, no header, no colour, plain ASCII, with an empty stderr.
contract/viola-revive/preflight-order · viola revive start · preflight order — The start reads its preflight in a fixed order: the holder, the logged session, session-live, then the recorded directory.
contract/viola-revive/session-live-check · viola revive start · session-live check — One `claude agents --json` read; a row whose `sessionId` is the logged id refuses; skipped under `--fork`; a failed read passes and the start goes on.
contract/viola-revive/start-output · viola revive · passed start output — A `viola revive` start that passes its preflight prints nothing of its own while the resumed child holds the terminal, as `viola run`.
contract/viola-run/start-output · viola run start · passed start output — Before the child, only a key-on start's automatic verify line and verify's own output reach the terminal (discarded under `revive --json`); the trust step adds no refusal, exit code, hint, `--json` code or terminal byte; then the terminal belongs to the wrapped claude TUI until it exits.
contract/viola-verb-help/usage · viola <verb> --help output · per-verb usage — `viola <verb> --help` gives clap's per-verb usage unchanged, its usage line naming the program by its executable.
decision/argument-order-mirrors-channel · Argument order mirrors the channel params — CLI argument order mirrors the channel `params`: `viola <verb> <target>` for one strip, `viola link|unlink <driver> <driven>` for a handoff, `viola answer <target> <dialog_id>`.
decision/dialog-attention-only-on-board · DIALOG attention word only on the board — The amber DIALOG attention word appears only in viola list.
decision/json-on-every-data-verb · --json on every data verb — `--json` is the machine view of every data verb, `viola revive` included in both its refusal document and `--list`.
decision/no-terminator-word · No terminator word — No `done.`, `success` or summary banner; the exit code is the terminator and the last printed line says what happened, to whom and where.
decision/only-header-bay-line · Only header is the BAY line — The only header on any verb is the one static BAY line at the top of `viola list`.
decision/wrapper-repeats-send-refusals · Wrapper repeats the send text refusals — The wrapper makes the same control-character and empty-text refusals for a frame sent straight to it as `viola send` makes itself.
contract/cli-exit-codes/11 · CLI exit codes · 11 — Retired (was `budget-paused`), never reused; no refusal maps to 11.
contract/cli-refusals/instance-unreachable · CLI refusals · instance-unreachable — With no live instance a verb exits 21, printing `unable  <name>  instance-unreachable` on stderr and a hint line as the last stderr line.
contract/cli-token-rendering/color-surface · CLI token rendering · color-surface-* — Never set; the terminal background belongs to the user.
contract/cli-verbs/send · CLI verbs · viola send <target> — Types at a turn boundary and reports read back, unable or unconfirmable.
contract/frame-verb-hints/instance-not-running · frame verb exit-21 hints · instance not running — `hint: <name> is not running; viola list shows the live instances` under `unable  <name>  instance-unreachable` for `wait`, `last`, `link`, `unlink`, and `send` in its mirror.
contract/frame-verb-hints/not-wrapped · frame verb exit-21 hints · name with no instance directory — `hint: <name> is not a wrapped session; only sessions started with viola run take commands`, the last stderr line.
contract/frame-verb-hints/server-verify-failed · frame verb exit-21 hints · endpoint not the recorded wrapper — No frame written, exit 21, `hint: the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`; under `--json` `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}`, stderr empty.
contract/frame-verb-hints/socket-dir-refused · frame verb exit-21 hints · socket directory refused — No frame written, exit 21, `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`; under `--json` the `socket-dir-refused` detail, stderr empty.
contract/frame-verb-hints/wrapper-went-away · frame verb exit-21 hints · wrapper lost during the call — `hint: <name>'s wrapper went away during the call; viola list shows whether it is still running`, the last stderr line.
contract/frame-verbs/strict-modes-refused · the frame verbs · strict-modes refusal — For send, wait, last, answer, pause, release, link and unlink: exit 21 before any role file or log opens, stdout empty, the state-files pair, mode left as found; under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, stderr empty.
contract/mcp-tools/list · viola mcp tools · list — A driver's machine view of the viola list verb.
contract/send-refusals/control-character · viola send refusals before any request · control-character — A text holding a refused character is refused before any frame: `[/ ] unable  <name>  not-delivered  control-character` on stderr, exit 13, its hint last; under `--json` one stdout document, no stderr line.
contract/send-refusals/empty-text · viola send refusals before any request · empty-text — A text empty once trailing CR and LF are removed is refused before any frame, exit 13, `[/ ] unable  <name>  not-delivered  empty-text` then `hint: the text is empty once its trailing newlines are removed; send a text with content`.
contract/viola-list-board/cli-unknown · viola list board · CLI unknown — A row with no recorded version reads CLI `unknown`.
contract/viola-list-board/dialog-unknown · viola list board · DIALOG unknown — A pending dialog of unknown kind reads `DIALOG unknown`.
contract/viola-list-board/status-join · viola list board · STATUS — One `claude agents --json` read follows the board read, joined by child pid or agent session id; a joined wrapped row reads STATUS idle or busy, else unknown.
contract/viola-list-board/wheel-unknown · viola list board · WHEEL unknown — A replayed row with no logged holder reads WHEEL `unknown`.
contract/viola-revive-json/list-document · viola revive --json · --list document — `--list --json` prints the rows as one document `{"v":1,"ok":{"sessions":[{"ts":…,"cause":…,"id":…}]}}` in log order, fields as the human rows, or the `no-session` document with exit 1.
contract/viola-revive/refusal-shape · viola revive refusals · refusal shape — Each `viola revive` refusal is exit 1 with one `unable:` line and one `hint:` line last on stderr and an empty stdout.
contract/viola-run/already-live · viola run start refusals · already live — Exit 1 with `unable: <name> is already live` on stderr and `hint: viola list` as the last stderr line; no paths, no pids.
contract/viola-send-output/unable · viola send output · unable — `[/ ] unable …` prints on stderr with exit 13 (or 10/12/14) when the wrapper refuses, or exit 13 when the client refuses the text before any frame (not-delivered with control-character or empty-text).
contract/viola-send-output/word-column · viola send output · word column — The word column is padded so open, read back, unable and unconfirmable line up, and the name always starts in the same column.
contract/wheel-verb-output/pause · wheel verb output · viola pause — Prints `<name>  wheel human  manual-pause  I have control`.
contract/wheel-verb-output/release · wheel verb output · viola release — Prints `<name>  wheel driver  you have control`.

# layout-templates · its absences, names and overviews, whole

## surface-web-spa · ## Surface: web-spa
absence/web-surface — A web surface: viola renders no page

## surface-cli · ## Surface: cli
absence/cli-deep-links — Deep links or URL coupling in the CLI
