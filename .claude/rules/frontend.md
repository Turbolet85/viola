---
paths:
  - "crates/viola-ui/assets/**"
  - "src/cmd/**"
---

# Frontend Rules

Path-scoped rules for viola's one built design surface, human CLI output (cli); the web-spa frontend is deferred to 0.2.0, its technology is not chosen, and 0.1.0 renders no page. Authoritative sources: `.andromeda/design-system.md` and `.andromeda/layout-templates.md`.

## Web-spa (deferred to 0.2.0)
- **Frontend:** deferred to 0.2.0; its technology is not chosen, and 0.1.0 renders no page.

## Tokens (the Token Test; a 0.2.0 frontend design input, 0.1.0 renders no page)
- Exactly eight hex values (`--c-*`), read only where a role alias is assigned; components use role aliases (`--surface-*`, `--ink*`, `--rule-*`, `--attention`, `--handoff`, `--focus-ring`, `--rb-*`). Every length is a token or a `ch` track in `--strip-cols` / `--strip-cols-wrapped` / `--tape-cols`; only the `760px` / `1024px` breakpoints are literal.
- Amber means `dialog_pending` only; blue means handoff only. Lamp-off, amber and blue text never sit on holder `#2A2E33` (they fail AA there).
- Borders-only depth: no `box-shadow`, `text-shadow`, glow or `drop-shadow`; radius 0 everywhere; dark only.
- Fonts: installed stacks only (`--font-label` Bahnschrift…, `--font-field` Cascadia Mono…); every contrast and render assertion must hold with the Linux DejaVu families resolved (the resolved family, not only the declared stack); 0.1.0 renders no page, so no CI step renders these stacks.

## Motion (expression 0.3 on the 0.2.0 page; 0.1.0 renders no page)
- Exactly two motions, each played once on a state change: the cocked-strip `translate` (12px over `--cock-dur`, 160ms ease-out) and a new tape line or transfer marker fading in (opacity over `--fade-dur`, ≤120ms). The readback fill, the strike, `stale` dimming and the return of a strip into line are instant; no hover effects except link underline and focus rings.

## Components
- The web components are 0.2.0 frontend design; 0.1.0 renders no page. The six captions NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, their words and slot order are fixed, and a future page reuses them; the readback vocabulary is `open`, `read back`, `unable`, `unconfirmable`.
- Nothing is shown as delivered until read back; `turn-ended` never fills a box.

## CLI surface (`src/cmd/**`)
- Colour decision order (first match → no colour): `--json` · `viola run` · `viola hook`/`mcp` · `NO_COLOR` (present, an empty value included) · `TERM=dumb` · non-TTY stdout · Windows VT enable failed. SGR only for amber `DIALOG`, dim `stale` rows, bold NAME — always beside its word; never on `unable` / `fail`. As built (chunk 2026-10-17-cli-output-discipline), `list` is uncoloured under any rung above; on a terminal that takes colour every NAME value is bold, the `DIALOG` word of a wrapped row with a pending dialog takes the attention token (its kind word plain), a wrapped `stale` row is dim from its first byte to its last (NAME dim and bold; on a stale row with a dialog only `DIALOG` amber and not dim), the BAY line, caption, empty-board line and separator carry no SGR, the spans are laid over the escaped, padded cells (the plain board's columns, one write), and the depth is truecolor / 256 / 16 from `COLORTERM` / `TERM` (256 on Windows once VT enabling has succeeded) (`COLUMNS`, `CLICOLOR`, `CLICOLOR_FORCE` and `CI` are not read). A wrapped row joined to one `claude agents --json` read by child pid or agent session id reads STATUS `idle` or `busy`, else `unknown`; each unjoined session follows under the `-- UNWRAPPED - READ-ONLY --` line, sorted by name, NAME its `sessionId`, LIVE `live`, `n/a` in WHEEL, DIALOG and CLI, with no separator when there is none and the board as with no `claude` on a failed read. With the terminal's width known (`viola_pty::host_size()`, never `COLUMNS`) only an unwrapped NAME is cut, to the column less 3 characters plus `...`, never inside an escape; a wrapped NAME is never cut, and a piped board or `--json` keeps every name whole.
- stdout = results (`viola mcp` writes only MCP frames, and exits 1 with nothing on stdout or stderr when its home or `diagnostics/` fails `check_path`); stderr = `waiting:`, the send issue line, refusals `unable  <name>  <reason>  <detail>` (the exit-1 refusals of `run` / `revive` / `verify`, a `verify` stopped by Ctrl-C or TERM before its rows are checked (stdout empty, no summary), `list`'s exit-21 strict-modes refusal and `plugin install`'s exit-21 strict-modes refusal (the home pair, stdout empty; under `--json` the `instance-unreachable` document, stderr empty) take the fixed form `unable: <text>`) + one `hint:` line (none under `--json`; there `viola revive`'s strict-modes and preflight refusals are one stdout document `{"v":1,"error":"<code>","detail":null}` with stderr empty), errors. ASCII only (`->`, `[RB]` / `[  ]` / `[/ ]`, ` - `); no spinner, no redraw, no stack traces, no "done". `viola --help` (and `viola help`) prints exactly `Usage: viola [--home <dir>] <verb> [args]`, the served-verb rows `board:` · `traffic:` · `wheel:` · `setup:` (`setup:` holding `plugin install`), then `Options:` (no about line, no clap `Commands:` list; pinned by the `HELP` literal in `tests/cli_output_plain.rs`); the `handoff:` row joins when "Explicit session links" serves `link` and `unlink`; per-verb `--help` is clap's usage, unchanged; the hidden `viola mcp` (`#[command(hide = true)]`, no argument but the global `--home`, run only by the plugin, never by a person) appears in no row, and the `HELP` literal is unchanged.
- `list` rows escape every control character (C0 with `\n`/`\t`, DEL, C1) as two-hex-digit escape text (`escape_row`); `wait`/`last` text escapes C0, DEL and C1 the same way but keeps `\n`/`\t`; nothing is stripped. No hint ever names `viola release`; no hint for `unknown`, `wrapper fault` or `internal error`.

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run._
