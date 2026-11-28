# layout-templates · the rows that bind 2026-11-28-fleet-console-view — picked 36 of 241 (21 by every draw) · rule any · pin 5303

## surface-cli · ## Surface: cli
decision/board-ordered-by-name · Board ordered by name — viola list rows keep the board's own name order with no second sort, each wrapped child drawn directly under its parent with its NAME cell indented, so a child whose name sorts before its parent's is drawn after that parent, under it; nothing is re-sorted by state; NAME values are bold.
decision/board-row-escaping · Board row escaping — Every board field derived from `claude agents --json` has C0 and C1 controls, newline and tab included, escaped as hex text, so a row never splits and the columns stay fixed-width. · why: Ratified; stricter than the security floor, which keeps newline and tab for `wait` and `last` text.
decision/board-vs-strip · Board versus strip verbs — `viola list` shows the whole bay, `viola up` re-opens the whole fleet or the instances it names, `verify`, `plugin install` and `console` name no strip, and every other verb a person runs acts on one strip named by its `ViolaName`.
decision/hand-written-sgr-styling · Hand-written SGR styling module — Terminal styling and fixed-width column padding come from a small hand-written SGR module in the viola binary, with no colour or table crate.
decision/no-link-column · No link column on the board — The six shared board columns, their captions and their slot order stay unchanged, with no link column; links show in `link` / `unlink` verb output and in the board's -- LINKS -- block after the unwrapped rack.
decision/no-prompt-or-pager · No interactive prompt or pager — No verb has an interactive prompt or a pager, and the terminal fleet console is no verb's output.
decision/shared-captions-fixed · Shared captions fixed — The six captions, words and slot order are fixed, and the terminal fleet console reuses them.
contract/cli-global-flags/home · CLI global flags · --home — `--home` is the same on every verb.
contract/cli-token-rendering/label-role · CLI token rendering · Label role — Uppercase column captions (NAME LIVE STATUS WHEEL DIALOG CLI) and group labels, printed plain.
contract/cli-verbs/console · CLI verbs · viola console — Shows the terminal fleet console in the human's terminal; it takes no argument, the global `--home` only and no `--json`, and exits 0 on quit, interrupt or end of input, 1 on a refusal or an internal error.
contract/cli-verbs/list · CLI verbs · viola list — The board: every session as one strip row, with the BAY context header.
contract/mcp-tools/list · viola mcp tools · list — A driver's machine view of the viola list verb.
contract/viola-list-board/agents-read-failure · viola list board · failed agents read — A failed, oversize or malformed `claude agents --json` read prints the board as with no claude.
contract/viola-list-board/dialog-unknown · viola list board · DIALOG unknown — A pending dialog of unknown kind reads `DIALOG unknown`.
contract/viola-list-board/status-join · viola list board · STATUS — One `claude agents --json` read follows the board read, joined by child pid or agent session id; a joined wrapped row reads STATUS idle or busy, else unknown.
contract/viola-list-board/tree · viola list board · tree — Wrapped rows are drawn as a tree, each child under its parent with its NAME cell indented two spaces per depth; a row whose parent has no wrapped row is a root, and cycle members are drawn once, from the first in order.
contract/viola-list-board/wheel-unknown · viola list board · WHEEL unknown — A replayed row with no logged holder reads WHEEL `unknown`.
contract/viola-list-json/document · viola list --json · document — One JSON document on stdout with no header line, no colour and no glyph; its item and skipped shapes are the list payload contract.
contract/viola-list/columns · viola list · columns — NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, each padded to its widest word, with the two-space gutter between columns and no box-drawing borders.
decision/dialog-attention-only-on-board · DIALOG attention word only on the board — The amber DIALOG attention word appears only in viola list.
decision/no-spinner · No spinner — The cli shows no spinner and no elapsed-time counter.
decision/plain-output-modes · Plain output modes — Under --json, a non-TTY, NO_COLOR, TERM=dumb and the viola run / viola revive passthrough the cli prints no colour, no non-ASCII glyph and no cursor control.
decision/std-isterminal-tty-detection · TTY detection through std IsTerminal — Whether a stream is a terminal is detected with the standard library's IsTerminal, with no crate.
decision/stdout-results-stderr-rest · Results on stdout, the rest on stderr — Results go to stdout; `waiting:`, the issue line, refusals, hints and errors go to stderr, and the two are never mixed.
decision/truncate-only-unwrapped-names · Truncate only unwrapped names — Only unwrapped NAMEs are cut, with `...`, only where the terminal width is known and too narrow, never inside an escape; wrapped names are never clipped and no line ever wraps.
contract/cli-token-rendering/color-text-muted · CLI token rendering · color-text-muted — The stale token: SGR dim across a whole stale row, and never without the word stale in LIVE.
contract/cli-token-rendering/motion-tokens · CLI token rendering · Motion tokens — None; nothing redraws in place.
contract/cli-token-rendering/spacing · CLI token rendering · Spacing — The column gutter is two spaces on the terminal grid; inside one field ` - ` is the separator.
contract/mcp-tool-results/success · viola mcp tool results · success — Each success is isError false with the CLI verb's --json `ok` payload alone as structuredContent, without `v` or the `ok` wrapper.
contract/viola-list-board/cli-unknown · viola list board · CLI unknown — A row with no recorded version reads CLI `unknown`.
contract/viola-list-board/column-widths · viola list board · column widths — NAME is padded to the longer of 4 and the widest NAME cell over both racks in characters, its tree indent included, LIVE to 5, STATUS to 7, WHEEL to 23, DIALOG to 17, CLI last and unpadded; no line ends in a space.
contract/viola-list-board/name-truncation · viola list board · NAME truncation — With the terminal width known (from the host pty size, never COLUMNS), an unwrapped NAME longer than the column (capped at the width less 65) is cut to the column less 3 plus `...`, never inside a `\xHH` escape; a wrapped NAME is never cut.
contract/viola-list-board/plain-lines · viola list board · plain lines — The BAY line, caption, empty line and separator print plain.
contract/viola-list-board/unknown-width · viola list board · unknown terminal width — Nothing is truncated when the terminal width is unknown.

## surface-console · ## Surface: console
decision/console-frame-floor · Console frame floor 80x24 — The console's frame floor is 80x24: at or above it the whole frame is drawn from a cleared top-left, `viola console` on row 1 and `q quit` on the last row; below it one line, `viola console needs 80x24; this terminal is <cols>x<rows>`; an unread size counts as 0x0.
decision/console-own-surface · Terminal fleet console as its own surface — The terminal fleet console, `viola console`, is its own surface in the human's terminal, entered on the alternate screen with the cursor hidden and restored on every end, in plain ASCII with no colour. · provisional

# layout-templates · its absences, names and overviews, whole

## surface-web-spa · ## Surface: web-spa
absence/web-surface — A web surface: viola renders no page

## surface-cli · ## Surface: cli
absence/cli-deep-links — Deep links or URL coupling in the CLI
