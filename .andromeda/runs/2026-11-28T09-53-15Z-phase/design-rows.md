# design-system · the rows that bind 2026-11-28-fleet-console-view — picked 80 of 366 (37 by every draw) · rule any · pin 5303

## brand-identity · ## Brand Identity
decision/dialog-pending-cocked-strip · Dialog-pending shown as a cocked strip — A `dialog_pending` session keeps its slot and shows the word `DIALOG` plus the dialog kind, the word in arrival-holder amber on the CLI.
decision/fixed-phraseology-vocabulary · Fixed phraseology vocabulary — Short fixed words (`open`, `RB`, `unable`, `human-typing`, `not-delivered`) are the vocabulary of every label, CLI column and refusal, with no filler and no guessing.
decision/one-strip-per-run-instance · One flight-progress strip per instance — Every `viola run` instance is shown as exactly one flight-progress strip.
decision/stable-session-slots · Stable session slots — Every session keeps a slot in the strip bay stable across state changes: wrapped sessions in name order, then unwrapped ones in name order; nothing is re-sorted by state, and a slot shifts only when a session arrives or leaves.
decision/stale-strip-dimmed-not-deleted · Stale strip dimmed, never deleted — A strip of `stale` liveness is dimmed to lamp-off, never deleted, and keeps its slot.
contract/flight-progress-strip/cli · Flight-progress strip · CLI — CLI is the sixth and last of the six fixed fields.
contract/flight-progress-strip/dialog · Flight-progress strip · DIALOG — DIALOG is the fifth of the six fixed fields.
contract/flight-progress-strip/live · Flight-progress strip · LIVE — LIVE is the second of the six fixed fields.
contract/flight-progress-strip/name · Flight-progress strip · NAME — NAME is the first of the six fixed fields, set at callsign weight.
contract/flight-progress-strip/no-reading · Flight-progress strip · field with no reading — A field with no reading prints `unknown` in its box, never a shimmer or a guess.
contract/flight-progress-strip/status · Flight-progress strip · STATUS — STATUS is the third of the six fixed fields.
contract/flight-progress-strip/wheel · Flight-progress strip · WHEEL — WHEEL is the fourth of the six fixed fields.
decision/cli-has-no-motion · No motion in the CLI — The cli (0.2 on a TTY, 0.0 otherwise) has no motion at all.
decision/rejected-style-presets · Rejected style presets — The Dark Mode (OLED), Real-Time Monitoring (pulsing dots, blink, toasts) and HUD / Sci-Fi FUI (glow, telemetry animation) presets are not used.
decision/strip-board-density · Calm strip-board density — The density is a strip board's, not a monitoring dashboard's: fixed fields, tabular numbers, one attention colour and one handoff colour, everything else paper and ink.

## color-palette · ## Color Palette
decision/busy-not-animated · Busy has no animation — The `busy` state has no spinner, pulse or animation.
decision/skipped-count-stays-visible · Skipped count stays visible — A nonzero `skipped` count stays visible and is never auto-cleared.
decision/amber-only-for-dialog-pending · Arrival amber for dialog-pending only — Arrival-holder amber #D97706 is the only attention colour, used for `dialog_pending` only. · except: never for errors, warnings, budget or hover
decision/no-green-accent · No run-green accent — The run-green accent of the Developer Tool palette is not used; amber is the single sharp accent.

## typography · ## Typography
decision/bold-only-name-column · Bold only for the NAME column — In the cli, bold is used only for the NAME column.
decision/cli-ascii-only · ASCII-only CLI output — CLI output is ASCII only, separators included, at high density.
decision/stale-rows-sgr-dim · Stale rows in SGR dim — In the cli, SGR dim marks `stale` rows and amber marks the `DIALOG` word.

## spacing · ## Spacing
decision/cli-two-space-columns · Two spaces between CLI columns — CLI spacing follows the fixed-width terminal grid, with two spaces between columns.

## iconography · ## Iconography
decision/list-six-fixed-columns · viola list has six fixed columns — `viola list` carries no link column, so its six columns, their captions and their slot order stay fixed; a tree's indent stands inside the NAME cell and links print apart, in the board's `-- LINKS --` block.
decision/no-decorative-icons · No decorative icons — Icons clarify, not decorate: no robot or person icons, no status dots, no checkmarks and no warning triangles, since every such state is a printed word.
decision/text-glyphs-only · Custom text glyphs only — Iconography is custom text glyphs only; phraseology words carry the meaning.
contract/viola-list/columns · viola list output · columns — `viola list` carries no link column, so its six columns, their captions and their slot order stay fixed; a tree's indent stands inside the NAME cell and links print apart, in the board's `-- LINKS --` block.
contract/cli-separators/space-hyphen-space · CLI separators · ` - ` — Space, hyphen, space replaces a middle dot inside one field or label (`skipped 2 - 0 - 1`).
contract/cli-separators/two-spaces · CLI separators · two spaces — Two spaces separate columns, and a refusal's reason from its detail, as separate fields.

## surface-cli · ## Surface: cli · twin surface
decision/columns-never-read · COLUMNS is never read — Terminal width comes only from stdout's own terminal; `COLUMNS` is never read and a piped stdout has no width.
decision/fleet-console-not-verb-output · Fleet console is no verb's output — The terminal fleet console is no verb's output, so verb output rules do not govern its redraw.
decision/lines-never-wrapped · Lines never wrapped — CLI output lines are never wrapped.
decision/list-is-the-board · viola list is the board — `viola list` is the board, `verify`, `plugin install` and `console` act on no strip, `up` acts on the whole fleet or on the instances it names, and every other verb a person runs acts on one strip.
decision/list-rows-escape-all-controls · list rows escape every control character — Every field of `claude agents --json` origin has C0/C1 controls escaped as hex text, and in `list` table rows `\n` and `\t` are escaped too, so every row stays one fixed-width line. · why: stricter than the security plan's floor of keeping `\n` and `\t`; ratified · except: `wait` / `last` message text keeps `\n` and `\t`
decision/no-interactive-prompt-or-pager · No interactive prompt or pager — No verb has an interactive prompt or a pager.
decision/no-spinner · No spinner or elapsed-time counter — The CLI shows no spinner and no elapsed-time counter.
contract/viola-list-table/columns · viola list table · columns — Under a BAY line the table has the columns NAME, LIVE, STATUS, WHEEL, DIALOG and CLI; unwrapped sessions follow a `-- UNWRAPPED - READ-ONLY --` line.
contract/viola-list-table/row-order · viola list table · row order — Rows keep the board's name order, each wrapped child drawn directly under its parent.
decision/api-error-turn-end-shown-as-turn-end · API-error turn end shown as any turn end — A turn the CLI ends on an API error shows like any turn end: STATUS reads `idle`, `wait` reads `turn-ended`, no distinct status, colour or hint marks it, the only trace is one closed cause word of nine in `--json` and MCP `wait` event data, and the CLI's error text is shown nowhere. · guard: list_status_reads_idle_after_an_api_error_turn_end
decision/ascii-only-human-output · ASCII-only human output — Human output is ASCII only (`->`, `[RB]`, `[/ ]`, `[  ]`).
decision/hand-written-sgr-module · Hand-written SGR styling module — Styling is a small hand-written SGR module in the `viola` bin. · why: adds no dependency
decision/list-refuses-board-on-strict-modes-failure · Strict-modes failure refuses the whole board — A failed strict-modes check of the home or any instance refuses the whole list board with exit 21, in the fixed unable and hint pair or, under --json, the instance-unreachable document with detail strict-modes-failed. · why: The MCP list tool reads the same refusal document through the shared board reader, and the list tests pin exit 21 and the document byte for byte.
decision/no-handoff-blue-in-cli · No handoff blue in the CLI — The CLI never uses the handoff blue colour.
decision/no-terminal-background · No background colours — CLI output never sets a background colour. · why: the terminal background belongs to the user
decision/truncate-only-unwrapped-name · Only an unwrapped NAME is truncated — Where stdout's own terminal width is known, human tables truncate only the NAME of unwrapped rows (`...`); otherwise nothing is truncated.
contract/cli-streams/stderr · CLI streams · stderr — stderr carries context lines, refusals with each refusal's `hint:` line directly after it, and errors.
contract/cli-streams/stdout · CLI streams · stdout — stdout carries results and data: tables, send/wait results, `last` text.
contract/cli-surface/output-streams · CLI surface · stdout/stderr — The cli surface is the stdout and stderr of the `viola` binary.
contract/viola-console-refusals/home-refused · viola console refusals · home failing its read rule — A home failing its read rule refuses `viola console` with exit 1: `unable: viola console could not use its home` then `hint: make the viola home readable by you alone, then run viola console again`; no line is written and the mode is left as found.
contract/viola-console-refusals/no-terminal · viola console refusals · no terminal — With stdin or stdout not a terminal, `viola console` exits 1 before any home read with `unable: viola console needs a terminal` then `hint: run viola console in a terminal window`.
contract/viola-list-table/empty · viola list table · empty — With no wrapped sessions the board prints `no wrapped sessions  start one: viola run <name> -- claude`.

## surface · ## Surface: cli · twin surface-cli
decision/cli-colour-amber-and-dim-only · CLI colour limited to amber and dim — CLI output colours nothing but the `DIALOG` word and `stale` rows, so no blue or other colour SGR is ever emitted. · why: The quiz limits CLI colour to amber and dim.
decision/terminal-background-never-set · Terminal background never set — The CLI never sets a background colour, at any colour depth. · why: The terminal background belongs to the user.
contract/cli-colour-tokens/handoff-blue · CLI colour tokens · handoff blue — Handoff blue #5B8DB8 is not used at any colour depth and no index is reserved for it; link output (`overseer -> builder`) is plain.
contract/viola-list-board/failed-agents-read · viola list board · failed agents read — A failed, oversize or malformed `claude agents --json` read prints the board as with no `claude`.
contract/viola-list-board/row-escaping · viola list board · row escaping — The row mode escapes every control character of a row, DEL and C1 included, as `\xHH`; NAME's width is counted in characters.
contract/viola-list-board/status · viola list board · STATUS — A wrapped row joined to a `claude agents --json` row, by child pid or agent session id, reads STATUS `idle` or `busy`; any other wrapped row reads `unknown`.
decision/wheel-column-width-23 · WHEEL column padded to 23 — The board's WHEEL column is padded to 23 characters. · why: Ratified.
contract/cli-colour-tokens/callsign · CLI colour tokens · callsign — Bold (`\x1b[1m`) at every colour depth, on every NAME column value.
contract/cli-colour-tokens/stale · CLI colour tokens · stale — SGR dim (`\x1b[2m`) at every depth over a whole `stale` row, first byte to last; `stale` always in LIVE; on a cocked stale row only `DIALOG` is amber and not dim; NAME on a stale row is dim and bold; every span reopens dim after its reset.
contract/human-tables/width-source · human tables · width source — The table width is stdout's own terminal size; `COLUMNS` is not read, and a piped stdout has no width.
contract/viola-list-board/skip-counts · viola list board · skip counts — Nonzero skip counts print as bare numbers in key order (`skipped 2 - 0 - 1`).
contract/viola-list-board/strict-modes-refusal · viola list board · strict-modes refusal — A failed strict-modes check of the home, `instances/` or any one instance refuses the whole board with exit 21, nothing from state printed, in the fixed `unable:` / `hint:` form.
contract/viola-list-board/strict-modes-scan · viola list board · strict-modes scan — The scan holds the home to the read rule and `instances/` to the write rule, then each instance's set; a home another user may read refuses the whole board with the home-state pair, with or without an instance.
contract/viola-list-board/uncoloured-lines · viola list board · uncoloured lines — The BAY line, the caption, the empty-board line and the separator carry no SGR.
contract/viola-list-board/unwrapped-row · viola list board · unwrapped row — One row per unwrapped session, sorted by name: NAME the session's `sessionId`, LIVE `live`, its STATUS, and `n/a` in WHEEL, DIALOG and CLI.

## anti-patterns-never-do-these · ## Anti-Patterns (NEVER do these)
decision/no-auto-sorting-strips · No auto-sorting of strips — Strips needing input or with recent sessions are never re-ranked to the top; a strip is cocked in place. · why: Re-ranking breaks the founder's memory of where each session lives, and the `DIALOG` word already says "needs you".
decision/no-hardcoded-widths-or-wrapping · No hardcoded widths or wrapping — Widths are never hardcoded; output is truncated only where the terminal width is known, and never wrapped.
decision/no-shimmer-or-glow-theme · No skeleton shimmer or glow theme — No skeleton shimmer and no terminal-green or neon-glow theme; missing values print `unknown` in their box at once. · why: Nothing glows in a tower cab unless the lamp is on it.
decision/wheel-as-words · Wheel shown as words — The WHEEL field prints `human` / `driver`, never icons, emoji or a toggle; taking control is spoken with `viola pause`. · why: Taking control is never implied by a switch.
decision/escape-controls-in-human-output · Control characters escaped in human output — Human output of `last` and `list` escapes C0/C1 control characters.
decision/no-convergent-safe-choices · No convergence on safe design choices — The design never converges on common safe choices across generations; the readback box and the cocked strip stand in their place. · why: Agent dashboards converge on dots, toasts and graphs.
decision/no-traffic-light-status-dots · No traffic-light status dots — Status is never a green/yellow/red dot; `stale` is a coasting lamp-off track, not red, and the only colour state is the cocked strip's amber beside its `DIALOG` word. · why: A strip prints its status word in a fixed column.
decision/stdout-data-stderr-messages · Data on stdout, messages on stderr — Results go to stdout; `waiting:`, refusals with their `hint:` lines, and errors go to stderr.

## self-validation-protocol · ## Self-Validation Protocol
decision/domain-not-statistical-design · Design from the product's domain — Before UI output is presented, an interface another AI would produce from a similar prompt fails; the interface must emerge from this product's domain exploration.
decision/squint-test · Squint test — Before UI output is presented, hierarchy must still be perceivable with blurred eyes and nothing may jump out harshly. · except: `open` and `unconfirmable` readback boxes look identical under the squint, and only the word cell tells them apart, as intended.
decision/swap-test · Swap test — Before UI output is presented, a design that would not feel meaningfully different with a default typeface, default colours and a standard layout is redone with intent.

## surface-console · ## Surface: console
contract/fleet-console-keys/ctrl-c · Fleet console keys · Ctrl-C — Ctrl-C ends the fleet console with exit 0.
contract/fleet-console-keys/q · Fleet console keys · q — `q` quits the fleet console with exit 0.
contract/fleet-console-keys/unbound · Fleet console keys · any other key — Every key other than `q` and Ctrl-C is unbound and ignored; the end of the console's input also ends it with exit 0.

# design-system · its absences, names and overviews, whole

## brand-identity · ## Brand Identity
N/A — viola has no web or graphical frontend, so no frontend expression level.
overview/brand-identity — The direction the CLI carries, and the terminal fleet console inherits, is a control-tower strip bay after dark: buff paper strips in anthracite holder boots on ruled aluminium racks. Every session holds the same slot for its whole life, and the only warm light is the one arrival holder cocked out of line when a session needs the human.

## color-palette · ## Color Palette
Surface Scale (elevation hierarchy) › N/A — viola renders no page, so no surface scale. ¶ Text Hierarchy › N/A — viola renders no page, so no text hierarchy; the cli's ink is the terminal's (Surface: cli). ¶ Semantic Colors › N/A — viola renders no page, so no colour states; the cli's status guards below stand.

## typography · ## Typography
N/A — viola renders no page; the cli's type is the terminal's (below).

## spacing · ## Spacing
N/A — viola renders no page; the cli's spacing is the terminal grid (below).

## depth-strategy · ## Depth Strategy
N/A — viola renders no page, so no depth tokens; the cli draws no depth.

## border-radius · ## Border Radius
N/A — viola renders no page, so no radius tokens.
absence/rendered-page — A rendered page: viola renders no page, so no radius, motion or spacing tokens; the cli's type is the terminal's

## surface-web-spa · ## Surface: web-spa
absence/web-page-surface — A web page or web surface of any kind

## surface-cli · ## Surface: cli · twin surface
absence/cli-tape — A tape in the CLI
name/command/answer — answer
name/command/focus — focus
name/command/hook — hook
name/command/last — last
name/command/link — link
name/command/list — list
name/command/mcp — mcp
name/command/open — open
name/command/pause — pause
name/command/plugin-install — plugin install
name/command/release — release
name/command/revive — revive
name/command/run — run
name/command/send — send
name/command/unlink — unlink
name/command/up — up
name/command/verify — verify
name/command/wait — wait

## self-validation-protocol · ## Self-Validation Protocol
4. Token Test › N/A — viola renders no page, so no stylesheet tokens to trace. ¶ 4. Token Test › N/A — viola renders no page, so no stylesheet token to trace.
