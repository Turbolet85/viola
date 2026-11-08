@leaf_group group=design-system+layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=73
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=7 class=stale
leaf-text: The direction a 0.2.0 frontend inherits (0.1.0 renders no page) is an air-traffic-control tower strip bay after dark.
source: fct-iy6nu2f4 «The direction the CLI carries, and the terminal fleet console (matrix#v2-14) inherits, is a control-tower strip bay after dark.»
replace-with: The direction the CLI carries, and the terminal fleet console (matrix#v2-14) inherits, is an air-traffic-control tower strip bay after dark.
@leaf_edit n=2 leaf=.claude/docs/design-summary.md line=9 class=stale
leaf-text: frontend N/A (deferred to 0.2.0, its technology not chosen)
source: fct-wuobflqs «N/A — viola has no web or graphical frontend, so no frontend expression level.»
replace-with: frontend N/A (viola has no web or graphical frontend)
@leaf_edit n=3 leaf=.claude/docs/design-summary.md line=11 class=stale
leaf-text: a 16px 1px-ruled square cell whose one 0.1.0 place is the CLI mirror (where a 0.2.0 frontend draws it is that version's to name, its technology not chosen)
source: fct-zdzmu6qp «It is a fixed-width, 1px-ruled square cell; its one place is the CLI mirror below.»
replace-with: a fixed-width 1px-ruled square cell whose one place is the CLI mirror
@leaf_edit n=4 leaf=.claude/docs/design-summary.md line=11 class=retired
leaf-text: the word amber on the CLI; its 12px / 160ms translate and 4px amber holder band are a 0.2.0 frontend design input)
source: fct-ve72ntsx «On `dialog_pending` the row keeps its slot and shows the word `DIALOG` plus the dialog kind, the word amber on the CLI.»
replace-with: the word amber on the CLI)
@leaf_edit n=5 leaf=.claude/docs/design-summary.md line=13 class=retired
leaf-text: ## Key design tokens (a 0.2.0 frontend design input; 0.1.0 renders no page)
source: fct-nmkob2ak «N/A — viola renders no page, so no surface scale.»
replace-with: ## Key design tokens (viola renders no page; the cli's ink, type and spacing are the terminal's)
@leaf_edit n=6 leaf=.claude/docs/design-summary.md line=19 class=retired
leaf-text: | Surface Base / Inset | `#1E2124` | page, rack gap, tape, stale + unwrapped strips, inset cells |
source: fct-zwz5bcg2 «The palette is a monochrome ladder of three surfaces (#1E2124 → #2A2E33 → #4A5057) and three inks (#E6D8AE, #9C9278, #3B3A36).»
replace-with: | Surface (ladder 1) | `#1E2124` | first of three surfaces (no surface scale: viola renders no page) |
@leaf_edit n=7 leaf=.claude/docs/design-summary.md line=20 class=retired
leaf-text: | Surface Raised-1 | `#2A2E33` | a lit wrapped strip (buff ink only) |
source: fct-zwz5bcg2 «The palette is a monochrome ladder of three surfaces (#1E2124 → #2A2E33 → #4A5057) and three inks (#E6D8AE, #9C9278, #3B3A36).»
replace-with: | Surface (ladder 2) | `#2A2E33` | second of three surfaces (no surface scale: viola renders no page) |
@leaf_edit n=8 leaf=.claude/docs/design-summary.md line=23 class=retired
leaf-text: | On-paper (Text Hierarchy) | `#3B3A36` | text on a buff fill only (`RB`) |
source: fct-jbhyjxw7 «N/A — viola renders no page, so no text hierarchy; the cli's ink is the terminal's (Surface: cli).»
replace-with: | Ink (graphite) | `#3B3A36` | the third ink (no text hierarchy: viola renders no page) |
@leaf_edit n=9 leaf=.claude/docs/design-summary.md line=28 class=retired
leaf-text: Contrast rule: lamp-off, amber and blue fail AA as text on holder → they live only on anthracite (captions in the rack header, cocked DIALOG cell inset, markers in the rack gap, stale strips drop to anthracite).
source: fct-dufqiv2l «Handoff blue is never used, and backgrounds are never set: the terminal background belongs to the user.»
replace-with: CLI colour: amber on the `DIALOG` word only, SGR dim on a whole `stale` row, bold for NAME; handoff blue is never used and backgrounds are never set.
@leaf_edit n=10 leaf=.claude/docs/design-summary.md line=32 class=retired
leaf-text: - **Labels / headings / callsign:** `--font-label: "Bahnschrift", "DIN Alternate", "Avenir Next Condensed", "DejaVu Sans Condensed", sans-serif` (DIN; labels and headings semi-condensed via `font-stretch: 87.5%`, the callsign at normal width and never text-transformed)
source: fct-ojj7oowq «N/A — viola renders no page; the cli's type is the terminal's (below).»
replace-with: - **No type tokens:** viola renders no page; the cli's type is the terminal's.
@leaf_edit n=11 leaf=.claude/docs/design-summary.md line=33 class=retired
leaf-text: - **Fields / tape / data:** `--font-field: "Cascadia Mono", Consolas, "SF Mono", Menlo, "DejaVu Sans Mono", monospace` (no ligatures, tabular figures)
source: fct-qwc3u462 «**cli:** Terminal monospace (the user's configured font).»
replace-with: - **cli:** terminal monospace (the user's configured font); the brand shows through the fixed-column structure and the phraseology words.
@leaf_edit n=12 leaf=.claude/docs/design-summary.md line=34 class=retired
leaf-text: - Sizes: display 16 · callsign 15 · field 13 · code 12 · label 11 (px); weights 400 / 600. No web fonts (`font-src 'none'`).
source: fct-qwc3u462 «Bold is used only for the NAME column. SGR dim marks `stale` rows, and amber the `DIALOG` word. Output is ASCII only, and density is high.»
replace-with: - Bold only for the NAME column, SGR dim for `stale` rows, amber for the `DIALOG` word; ASCII only, high density.
@leaf_edit n=13 leaf=.claude/docs/design-summary.md line=34 class=retired
leaf-text: **Every contrast and render assertion must hold on the resolved Linux DejaVu families**; 0.1.0 renders no page, so no CI step renders these stacks or provides their families (how a 0.2.0 frontend checks them is that version's to decide).
source: fct-ojj7oowq «N/A — viola renders no page; the cli's type is the terminal's (below).»
replace-with: No font stacks or render assertions: viola renders no page.
@leaf_edit n=14 leaf=.claude/docs/design-summary.md line=38 class=retired
leaf-text: - **Base unit:** 4px — `space-micro` 4 · `xs` 8 · `sm` 12 · `md` 16 · `lg` 24 · `xl` 32; geometry `--strip-h` 32 · `--line-h` 20 · `--band-w` 4 · `--rb-size` 16; `ch`-sized tracks `--strip-cols`, `--strip-cols-wrapped`, `--tape-cols`.
source: fct-ae76qcah «CLI spacing is platform-controlled: a fixed-width terminal grid, two spaces between columns.»
replace-with: - No spacing tokens (viola renders no page): CLI spacing is platform-controlled, a fixed-width terminal grid, two spaces between columns.
@leaf_edit n=15 leaf=.claude/docs/design-summary.md line=42 class=retired
leaf-text: the 0.0–0.2 duration row applies (0–100ms, no entrance), and the cli (0.2 on a TTY, 0.0 otherwise) has no motion at all; the cocked strip's 12px / 160ms translate is a 0.2.0 frontend design input.
source: fct-4m7ag2lh «At 0.2 viola's cli (0.2 on a TTY, 0.0 otherwise) has no motion at all.»
replace-with: the cli (0.2 on a TTY, 0.0 otherwise) has no motion at all, and viola renders no page.
@leaf_edit n=16 leaf=.claude/docs/design-summary.md line=45 class=retired
leaf-text: - Borders-only (`--shadow: none`); radius 0 everywhere; one gradient (the refused strike).
source: fct-cwga6qbn «N/A — viola renders no page, so no depth tokens; the cli draws no depth.»
replace-with: - No depth or radius tokens (viola renders no page); the cli draws no depth, and square brackets `[ ]` keep the ruled-rectangle reading.
@leaf_edit n=17 leaf=.claude/docs/design-summary.md line=49 class=stale
leaf-text: deferred to 0.2.0; its technology is not chosen, and 0.1.0 renders no page (the colour, type, spacing, depth and radius sections stand as its design input)
source: fct-ll5xgfku «No web surface: viola renders no page; the keyboard-driven terminal fleet console tiled on Hyprland (matrix#v2-14) takes its own surface section at the route entry «Console scaffold».»
replace-with: no web surface: viola renders no page; the keyboard-driven terminal fleet console tiled on Hyprland (matrix#v2-14) takes its own surface section at the route entry «Console scaffold»
@leaf_edit n=18 leaf=.claude/docs/design-summary.md line=50 class=stale
leaf-text: six fixed columns and words a future page reuses
source: fct-mpm2axrl «The six captions, words and slot order are fixed, and the terminal fleet console (matrix#v2-14) reuses them.»
replace-with: six fixed columns and words the terminal fleet console (matrix#v2-14) reuses
@leaf_edit n=19 leaf=.claude/docs/design-summary.md line=52 class=retired
leaf-text: ## Component patterns (web-spa: a 0.2.0 design input; 0.1.0 renders no page)
source: fct-ll5xgfku «No web surface: viola renders no page;»
replace-with: ## Component patterns (viola renders no page; the CLI carries them)
@leaf_edit n=20 leaf=.claude/docs/design-summary.md line=55 class=stale
leaf-text: in 0.1.0 only the CLI `viola send` mirror, its 0.2.0 places that version's to name
source: fct-4be5q2ri «Point to the readback box in its one place: the `[RB]` / `[  ]` / `[/ ]` column of CLI `viola send`.»
replace-with: its one place the `[RB]` / `[  ]` / `[/ ]` column of CLI `viola send`
@leaf_edit n=21 leaf=.claude/docs/design-summary.md line=56 class=retired
leaf-text: - Transfer markers — outbound `→ builder  since …` and inbound `← overseer` in departure blue, in the rack gap on anthracite
source: fct-2enypnm7 «The CLI uses ASCII `->` and has no inbound form. `viola list` carries no link column in v1, so its six columns stay fixed. Links print only as `link` / `unlink` verb output (`overseer -> builder`).»
replace-with: - Transfer markers — only `link` / `unlink` verb output (`overseer -> builder`), plain, ASCII `->`, no inbound form; `viola list` has no link column
@leaf_edit n=22 leaf=.claude/docs/design-summary.md line=57 class=retired
leaf-text: - The tower tape — the ndjson tape played in order, one line per event, a refused send folded into its send line and a matched driver prompt into its send's readback, `skipped` counts kept where they happened
source: fct-ilu5rgbe «**No persistent footer.** Terminal scrollback is the history. The tape lives in `events.ndjson`.»
replace-with: - The tower tape — the ndjson tape in `events.ndjson`; the CLI has no tape, and terminal scrollback is the history
@leaf_edit n=23 leaf=.claude/docs/design-summary.md line=58 class=retired
leaf-text: - The ATIS header — sticky: `BAY`, `5H`, `7D`, `read … ago`, gate, `TAPE`, `skipped`
source: fct-bz2eawmt «**Cell order:** `BAY`, `5H <n> %`, `7D <n> %` (each followed by `expired` when the window has passed), `read <age> ago`, `gate open` or `budget-paused`, `skipped a - b - c`.»
replace-with: - The BAY line — the one static header, atop `viola list`: `BAY`, `5H`, `7D`, `read … ago`, gate, `skipped` (no TAPE cell)
@leaf_edit n=24 leaf=.claude/docs/design-summary.md line=59 class=retired
leaf-text: - The access refusal, error strips and the reserved v1.x brake buttons and Raised-3 confirmation (not built in v1)
source: fct-7zli33yx «- **Line form (stderr):** `unable  <name>  <reason>  <detail>`, fields separated by two spaces.»
replace-with: - Refusal lines — `unable  <name>  <reason>  <detail>` on stderr, then the cause's `hint:` line last
@leaf_edit n=25 leaf=.claude/docs/design-summary.md line=63 class=retired
leaf-text: - NEVER use a ninth hex value; the font stacks end at the generic `sans-serif` / `monospace`, never `system-ui`.
source: fct-zwz5bcg2 «All eight values come from the exploration's Color World: the tower cab strip bay at night.»
replace-with: - NEVER use a ninth hex value: all eight come from the Color World, and only two are hues.
@leaf_edit n=26 leaf=.claude/docs/design-summary.md line=67 class=retired
leaf-text: - NEVER use shadows, glow, gradients (except the strike) or a radius other than 0.
source: fct-tj74erhr «Rejected because nothing glows in a tower cab unless the lamp is on it.»
replace-with: - NEVER use glow or a terminal-green / neon "hacker" theme; the cli draws no depth.
@leaf_edit n=27 leaf=.claude/docs/design-summary.md line=68 class=retired
leaf-text: - Tape event text prints as plain text (the domain status rows).
source: fct-juf7qttk «Domain status guards: a refused send folds into its send line, never a separate line; `turn-ended` never fills a readback box; a nonzero `skipped` count stays visible, never auto-cleared;»
replace-with: - A refused send folds into its send line; `turn-ended` never fills a readback box; a nonzero `skipped` count stays visible; a bare strip is never cocked nor a target.
