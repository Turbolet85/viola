@carried n=1 label=design-system/typography fact=fct-wbty2r2c
why: the v1-40 witness under the outer PTY on Unix must read the NAME cell bold, dim on a stale row, and printed exactly as the name was typed, with only NAME carrying bold.
quote: The NAME cell. Never text-transformed: a `ViolaName` is lower-case `[a-z0-9-]` and must read exactly as it is typed as a `target`.
@carried n=1 run=2 label=design-system fact=fct-qwc3u462
quote: **cli:** Terminal monospace (the user's configured font). The brand shows through the fixed-column structure and the phraseology words (`open`, `RB`, `unable`, `DIALOG`, `stale`). The readback mirror is `[RB]` / `[  ]` / `[/ ]`. Bold is used only for the NAME column. SGR dim marks `stale` rows, and amber the `DIALOG` word. Output is ASCII only, and density is high.
@carried n=2 label=design-system/iconography fact=fct-2enypnm7
why: the shared fake-agent link, unlink and list cases must read on ubuntu-latest and macos-latest the same ASCII `->`, no inbound form and no link column that they read on Windows.
quote: The CLI uses ASCII `->` and has no inbound form. `viola list` carries no link column in v1, so the six columns shared with the web strip stay fixed. Links print only as `link` / `unlink` verb output (`overseer -> builder`).
@carried n=2 run=2 label=design-system fact=fct-ukpiycjz
quote: The CLI is ASCII only: `->` with no inbound form, two spaces between columns, ` - ` inside one field; `viola list` carries no link column in v1, so its six columns stay fixed.
@carried n=3 label=design-system/iconography fact=fct-g2p3izdb
why: the Unix list, BAY-line and refusal cases must read the two-space column separator and the ` - ` in-field separator byte for byte as on Windows, never a `·` or another OS-only glyph.
quote: The CLI is ASCII only:
  - Two spaces separate columns, and a refusal's reason from its detail (they are separate fields, as in the exit-code table).
  - ` - ` (space, hyphen, space) replaces `·` inside one field or label: `driver - budget-paused`, `skipped 0 - 0 - 0`, `-- UNWRAPPED - READ-ONLY --`.
@binding n=1
**typography (the NAME cell at callsign weight) ↔ the cli surface's tokens and `viola list` pattern**: the v1-40 integration witness, run under openpty on ubuntu-latest and macos-latest, must read NAME bold and printed exactly as typed (design-system/typography fct-wbty2r2c), the whole stale row SGR dim with amber only on the `DIALOG` word (design-system fct-qwc3u462), the tokens' reset after every styled span and no background (design-system fct-dufqiv2l), as the sample board marks them (design-system fct-erfdav33).
@binding n=2
**iconography (the cli's ASCII glyphs and separators) ↔ the cli component patterns**: every shared list, link and unlink case on Unix must read the same ASCII board and verb lines as on Windows: two spaces between columns and ` - ` inside a field (design-system/iconography fct-g2p3izdb), `->` with no inbound form and no link column (design-system/iconography fct-2enypnm7), the BAY line and `-- UNWRAPPED - READ-ONLY --` separator of the sample board (design-system fct-pcmgk4u5) and the `link    overseer -> builder` form (design-system fct-glxbi6ql), in piped, `NO_COLOR` and `TERM=dumb` output too (design-system fct-xlz4sgiu).
@binding n=3
**typography (the cli's terminal font) ↔ iconography (ASCII only)**: the cli depends on no face, so parity is read on the bytes, never a rendered font: Linux and macOS cases assert ASCII output identical to Windows' (design-system fct-qwc3u462) (design-system fct-ukpiycjz).
