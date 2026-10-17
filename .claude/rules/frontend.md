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
- Colour decision order (first match → no colour): `--json` · `viola run` · `viola hook`/`mcp` · `NO_COLOR` · `TERM=dumb` · non-TTY stdout · Windows VT enable failed. SGR only for amber `DIALOG`, dim `stale` rows, bold NAME — always beside its word; never on `unable` / `fail`. As built, `list` prints its uncoloured form on every stream, a TTY included (a wrapped row joined to one `claude agents --json` read by child pid or agent session id reads STATUS `idle` or `busy`, else `unknown`; each unjoined session follows under the `-- UNWRAPPED - READ-ONLY --` line, sorted by name, NAME its `sessionId`, LIVE `live`, `n/a` in WHEEL, DIALOG and CLI, with no separator when there is none and the board as with no `claude` on a failed read): the colour layer is owed to "CLI output discipline".
- stdout = results; stderr = `waiting:`, the send issue line, refusals `unable  <name>  <reason>  <detail>` (the exit-1 refusals of `run` / `revive` / `verify`, a `verify` stopped by Ctrl-C or TERM before its rows are checked (stdout empty, no summary), and `list`'s exit-21 strict-modes refusal take the fixed form `unable: <text>`) + one `hint:` line (none under `--json`; there `viola revive`'s strict-modes and preflight refusals are one stdout document `{"v":1,"error":"<code>","detail":null}` with stderr empty), errors. ASCII only (`->`, `[RB]` / `[  ]` / `[/ ]`, ` - `); no spinner, no redraw, no stack traces, no "done".
- `list` rows escape every control character (C0 with `\n`/`\t`, DEL, C1) as two-hex-digit escape text (`escape_row`); `wait`/`last` text escapes C0, DEL and C1 the same way but keeps `\n`/`\t`; nothing is stripped. No hint ever names `viola release`; no hint for `unknown`, `wrapper fault` or `internal error`.

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run._
