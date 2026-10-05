# design extract

## Relevance
partial — no token, typography or motion work. The chunk touches three CLI human-output surfaces: the `[  ] unconfirmable` send mirror (W2), the `viola verify` step counter whose row count grows (W1/W3/W4/W5), and verify's scrub refusal for recorded screen fixtures (W1 `--record`). The web `unconfirmable` readback state is a downstream binding only.

## Constraints
- design-system §Surface: cli §Component Patterns 2 requires the new `send` outcome `ok {confirmed:false, detail:"unconfirmable"}` to print on stdout, exit 0, as the mirror line `[  ] unconfirmable  <name>  local command, no measured post-condition`. Column padding must match the `[RB] read back` / `[/ ] unable` sample rows, and no `hint:` line is printed (it is not a refusal).
- design-system §Brand Identity (Signature element) and §Color Palette §Domain status colors row `Readback::unconfirmable` require `unconfirmable` to reuse the OPEN drawing (`[  ]`) and never the filled `[RB]`. Per the `Readback::read` row and web-spa component 2 §Sources, a `/clear` send confirmed by SessionStart `clear` plus a new `session_id` is a `read` (`[RB] read back`), not `unconfirmable`.
- design-system §Surface: cli §Component Patterns 5 (`verify`) requires one static appended stdout line per ledger row in the form `[NN/MM] <row-id> <description>  pass|fail`, followed by the summary line `stamped <ver>  <n> pass  <m> fail`. The denominator is `LedgerRow::ALL`'s length ("the count grows as owning chunks land their rows"). After this chunk it is no longer `/06`, and the word `fail` stays uncoloured.
- design-system §Surface: cli §Exit-code phraseology row 1 fixes verify's refusal set. A `--record` fixture (now screen text) that still holds a path or username must refuse with `unable: a recorded payload still holds a path or a username` plus its hint, on exit 1. A verify run with a failing row prints no stderr word: the summary line carries the fail count. Whether the existing scrub-refusal path already covers screen-content fixtures is research's question.
- design-system §Surface: cli §Colour decision order and §Streams: results go to stdout; context lines, refusals and `hint:` go to stderr. Under `--json` the whole result is one JSON document, so `unconfirmable` is carried by the `ok` payload and no mirror text is printed. The ASCII mirror `[  ]` still prints under piped, `NO_COLOR` and `TERM=dumb` output.
- design-system §Brand Identity (Verified aircraft type) and §Domain status colors `CliVerified::true|false` require the CLI field to read `<ver> verified` only for a stamped version. Stamping 2.1.288 (W5) should flip `list`'s CLI cell for 2.1.288 instances. Unstamped builds read `unverified-cli`, uncoloured.

## Patterns to follow
- The readback mirror vocabulary `[RB]` / `[  ]` / `[/ ]` with the fixed words `read back` / `open` / `unable` / `unconfirmable`, all padded to one readback-word column (design-system §Surface: cli §Component Patterns 2).
- The step counter is the project's ONE progress pattern: static appended lines, never redrawn (design-system §Surface: cli §Component Patterns 5; §Per-Surface Bans cli).
- Fixed-message refusals each followed by one `hint:` line. Hints never quote upstream text, paths or pids (design-system §Surface: cli §Component Patterns 2, "A hint never quotes the sent text or any upstream text").

## Anti-patterns to avoid
- NEVER a spinner, progress bar or live redraw while the live PTY probe waits for quiet or for captures. verify prints only its static step-counter lines (design-system §Anti-Patterns §Per-Surface Bans cli).
- NEVER print recorded screen text, upstream prompt text, paths or pids in verify's stdout lines, errors or hints (design-system §Anti-Patterns §Per-Surface Bans cli, "NEVER print upstream text, paths, pids…"). Row lines carry the row id and a fixed description only.
- NEVER `✓` / `✗`, emoji or colour on `pass` / `fail` / `unconfirmable`. Colour is reserved for `DIALOG` (amber) and `stale` (dim) (design-system §Anti-Patterns §Per-Surface Bans cli).

## Contract bindings
- design ↔ obs/architecture (CL-1): web-spa component 2 §Data dependency requires `unconfirmable` to be carried by a CL-1 outcome record for `ok`/`confirmed:false`, so the page's `<viola-readback data-rb="unconfirmable">` can render it. Whether this chunk's `send` writer emits that record, and whether the page consumes it yet, is research's question. Until it does, the page cannot show `unconfirmable` (§Until CL-1 lands).
- design ↔ a11y (Use of Color, SC 1.4.1): every new state (`unconfirmable`, `pass`/`fail`, `verified`) is carried by a printed word, never by colour (design-system §Color Palette §Semantic Colors).
- design ↔ security (NEVER-log floor): the scrub-refusal phraseology for screen-content fixtures (design-system §Surface: cli §Exit-code phraseology row 1) ties to security-plan's recorded-payload scrub.

## Acceptance criteria contributions
- A `viola send` of a ledger-listed local command with no measured post-condition prints exactly one stdout line starting `[  ] unconfirmable  <name>` and exits 0, with no `hint:` line. Under `--json` it prints only the JSON document (per design-system §Surface: cli §Component Patterns 2).
- A `/clear` send whose SessionStart `clear` plus new `session_id` post-condition is met prints `[RB] read back`, never `unconfirmable` (per design-system §Surface: web-spa §Component Patterns 2 §Sources and §Domain status colors `Readback::read`).
- `viola verify` prints one `[NN/MM] … pass|fail` stdout line per `LedgerRow::ALL` entry, where MM equals the new row count, then `stamped <ver>  <n> pass  <m> fail`. It uses no SGR, spinner or redraw (per design-system §Surface: cli §Component Patterns 5 and §Per-Surface Bans cli).
- A `--record` run whose scrubbed screen text still holds a path or the username exits 1 with `unable: a recorded payload still holds a path or a username` plus a hint, and writes no fixture (per design-system §Surface: cli §Exit-code phraseology).
