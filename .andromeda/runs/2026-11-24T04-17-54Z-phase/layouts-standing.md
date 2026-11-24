# layouts · standing conventions

- S1 · each new CLI verb owes a verb table row, a help row, a per-verb usage, an exit-cause table entry and a --json form within the shared machine contract · surface-cli
- S2 · each new human output line owes plain-mode and uncoloured-mode rendering, hand-written SGR styling only through the token roles, and a hint for every refusal cause · surface-cli
  contract/cli-token-rendering/callsign-role · CLI token rendering · Callsign role — The callsign token: SGR bold on NAME values only.
  contract/cli-token-rendering/color-accent · CLI token rendering · color-accent — The design system's attention SGR token at truecolor, 256 or 16-colour depth, set only on the DIALOG word and always followed by the uncoloured kind word.
  contract/cli-token-rendering/color-role-handoff · CLI token rendering · color-role-handoff — Not used; link output is plain.
  contract/cli-token-rendering/color-surface · CLI token rendering · color-surface-* — Never set; the terminal background belongs to the user.
  contract/cli-token-rendering/color-text-muted · CLI token rendering · color-text-muted — The stale token: SGR dim across a whole stale row, and never without the word stale in LIVE.
  contract/cli-token-rendering/color-text-primary-secondary · CLI token rendering · color-text-primary / color-text-secondary — The terminal's default foreground; the terminal owns ink.
  contract/cli-token-rendering/data-role · CLI token rendering · Data role — Tabular figures and timestamps.
  contract/cli-token-rendering/label-role · CLI token rendering · Label role — Uppercase column captions (NAME LIVE STATUS WHEEL DIALOG CLI) and group labels, printed plain.
  contract/cli-token-rendering/radius · CLI token rendering · radius-* — No terminal analogue; square brackets `[ ]` keep the ruled-rectangle reading.
  contract/viola-list-board/uncoloured-modes · viola list board · uncoloured modes — The board is uncoloured under --json, a piped stdout, NO_COLOR (an empty value included) or TERM=dumb.
  contract/viola-verify-output/plain-ascii · viola verify output · plain ASCII — All viola verify output is plain ASCII with no colour.
  decision/amber-means-dialog · Amber means DIALOG only — Amber means `DIALOG` and nothing else, and `stale` is SGR dim in the terminal.
  decision/hand-written-sgr-styling · Hand-written SGR styling module — Terminal styling and fixed-width column padding come from a small hand-written SGR module in the viola binary, with no colour or table crate.
  decision/one-token-vocabulary-on-cli · One token vocabulary resolved on the cli — The one design token vocabulary (color-*, space-*, radius-*, the typography roles) resolves on the cli to the SGR tokens, the terminal default foreground or not used, through ANSI depth, the two-space gutter, ASCII `->` / ` - ` and brackets for the ruled box.
