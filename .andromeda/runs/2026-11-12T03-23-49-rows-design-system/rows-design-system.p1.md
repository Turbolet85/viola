@row_none n=1 rec=1 type=0
what: a brand personality statement in metaphor
@row_none n=2 rec=2 type=S
@row_add n=1 row=decision/one-strip-per-run-instance area=brand-identity rec=3
name: One flight-progress strip per instance
rule: Every `viola run` instance is shown as exactly one flight-progress strip.
@row_add n=2 row=contract/flight-progress-strip/name area=brand-identity rec=3
boundary: Flight-progress strip
item: NAME
promise: NAME is the first of the six fixed fields, set at callsign weight.
@row_add n=3 row=contract/flight-progress-strip/live area=brand-identity rec=3
boundary: Flight-progress strip
item: LIVE
promise: LIVE is the second of the six fixed fields.
@row_add n=4 row=contract/flight-progress-strip/status area=brand-identity rec=3
boundary: Flight-progress strip
item: STATUS
promise: STATUS is the third of the six fixed fields.
@row_add n=5 row=contract/flight-progress-strip/wheel area=brand-identity rec=3
boundary: Flight-progress strip
item: WHEEL
promise: WHEEL is the fourth of the six fixed fields.
@row_add n=6 row=contract/flight-progress-strip/dialog area=brand-identity rec=3
boundary: Flight-progress strip
item: DIALOG
promise: DIALOG is the fifth of the six fixed fields.
@row_add n=7 row=contract/flight-progress-strip/cli area=brand-identity rec=3
boundary: Flight-progress strip
item: CLI
promise: CLI is the sixth and last of the six fixed fields.
@row_add n=8 row=contract/flight-progress-strip/no-reading area=brand-identity rec=3,25
boundary: Flight-progress strip
item: field with no reading
promise: A field with no reading prints `unknown` in its box, never a shimmer or a guess.
@row_add n=9 row=decision/stable-session-slots area=brand-identity rec=4
name: Stable session slots
rule: Every session keeps a stable slot in the strip bay for its whole life, and nothing is re-sorted.
@row_add n=10 row=decision/dialog-pending-cocked-strip area=brand-identity rec=5,20
name: Dialog-pending shown as a cocked strip
rule: A `dialog_pending` session keeps its slot and shows the word `DIALOG` plus the dialog kind, the word in arrival-holder amber on the CLI.
@row_add n=11 row=contract/send-readback/prompt area=brand-identity rec=6,17
boundary: Send readback
item: prompt send
promise: A prompt send counts, and its readback box fills, only once the matching `prompt-submitted` reads it back.
@row_add n=12 row=contract/send-readback/clear area=brand-identity rec=6,17
boundary: Send readback
item: /clear
promise: A `/clear` send counts only once its post-condition, a `session-start` with cause `clear` (a new session), is observed.
@row_add n=13 row=contract/send-readback/compact area=brand-identity rec=6,17
boundary: Send readback
item: /compact
promise: On a verified CLI a `/compact` send counts only once a `session-start` of cause `compact` carrying the session's own id is observed.
@row_add n=14 row=contract/send-readback/unconfirmable area=brand-identity rec=6
boundary: Send readback
item: unconfirmable
promise: On an unverified CLI a `/compact` send stays `unconfirmable`.
@row_add n=15 row=contract/send-readback/unable area=brand-identity rec=6
boundary: Send readback
item: unable
promise: A send not read back is reported as `unable` plus a typed reason.
@row_add n=16 row=contract/handoff/transfer-marker area=brand-identity rec=7
boundary: Handoff display
item: transfer marker
promise: Transfer of control is shown as a `driver → driven` transfer marker.
@row_add n=17 row=decision/event-tape-played-in-order area=brand-identity rec=7
name: Event tape played in order
rule: The ndjson event tape is played in order, with `skipped` counts kept where they happened.
@row_add n=18 row=decision/bare-strips-never-target area=brand-identity rec=8,32
name: Unwrapped sessions are never a target
rule: Wrapped instances sit in a holder; unwrapped sessions are bare strips, visible but never a `target`.
@row_add n=19 row=decision/stale-strip-dimmed-not-deleted area=brand-identity rec=9
name: Stale strip dimmed, never deleted
rule: A strip of `stale` liveness is dimmed to lamp-off, never deleted, and keeps its slot.
@row_add n=20 row=decision/budget-paused-stops-new-sends area=brand-identity rec=10
name: Budget-paused as minimum-fuel state
rule: The budget governor shows `5H` / `7D` figures, and `budget-paused` is a declared state that stops new sends while the running turn finishes.
@row_add n=21 row=decision/readings-carry-their-age area=brand-identity rec=11,25
name: Readings carry their age
rule: Every budget reading carries its age (`read 4m ago`), and old information is shown as old (`expired`), never as current.
@row_add n=22 row=contract/cli-verification/verified area=brand-identity rec=12
boundary: CLI verification state
item: verified
promise: A CLI version stamped by `viola verify` prints `verified`.
@row_add n=23 row=contract/cli-verification/unverified-cli area=brand-identity rec=12
boundary: CLI verification state
item: unverified-cli
promise: An unstamped CLI build runs transport only, shown as `unverified-cli`.
@row_add n=24 row=contract/flight-progress-strip/wheel-human area=brand-identity rec=13
boundary: Flight-progress strip
item: human
promise: A human holding control shows `human` in WHEEL at weight 600.
@row_add n=25 row=decision/pause-release-transfer-control area=brand-identity rec=13
name: Pause and release as control transfer
rule: `viola pause` is the human's "I have control" and `viola release` is "you have control".
@row_add n=26 row=decision/fixed-phraseology-vocabulary area=brand-identity rec=14
name: Fixed phraseology vocabulary
rule: Short fixed words (`open`, `RB`, `unable`, `human-typing`, `budget-paused`, `not-delivered`) are the vocabulary of every label, CLI column and refusal, with no filler and no guessing.
@row_add n=27 row=decision/readback-box area=brand-identity rec=15
name: The readback box as signature element
rule: The readback box is a fixed-width, 1px-ruled square cell whose one place is the CLI mirror, with three drawn states each also printed as a word.
@row_add n=28 row=contract/readback-box/open area=brand-identity rec=16
boundary: Readback box
item: open
promise: The `open` state is drawn as a buff outline on anthracite.
@row_add n=29 row=contract/readback-box/rb area=brand-identity rec=17
boundary: Readback box
item: RB / read back
promise: The `RB` state is filled solid buff with `RB` in graphite, filled instantly when the send's readback lands.
@row_add n=30 row=contract/readback-box/unable area=brand-identity rec=18
boundary: Readback box
item: unable
promise: The `unable` state is the outline plus one 1px `/` strike, with the typed reason in the next cell.
@row_add n=31 row=contract/readback-box/unconfirmable area=brand-identity rec=19
boundary: Readback box
item: unconfirmable
promise: The printed word `unconfirmable` reuses the open drawing and is never filled.
@row_add n=32 row=decision/turn-ended-never-fills-readback area=brand-identity rec=19,32
name: Turn-ended never fills the readback box
rule: A `turn-ended` event never fills a readback box.
@row_add n=33 row=contract/readback-box/cli-mirror area=brand-identity rec=19,36
boundary: Readback box
item: CLI mirror
promise: The CLI mirrors the box states as `[RB]` / `[  ]` / `[/ ]`.
@row_add n=34 row=decision/expression-level area=brand-identity rec=21,23,42
name: Expression level 0.2
rule: The base expression level `0.2` is the single source of truth for animation intensity, layout freedom, interaction complexity and motion across the whole project.
@row_none n=3 rec=22 type=S
@row_add n=35 row=decision/cli-has-no-motion area=brand-identity rec=23
name: No motion in the CLI
rule: The cli (0.2 on a TTY, 0.0 otherwise) has no motion at all.
@row_add n=36 row=overview/brand-identity area=brand-identity rec=24
text: The direction the CLI carries, and the terminal fleet console inherits, is a control-tower strip bay after dark: buff paper strips in anthracite holder boots on ruled aluminium racks. Every session holds the same slot for its whole life, and the only warm light is the one arrival holder cocked out of line when a session needs the human.
@row_add n=37 row=decision/no-success-before-confirmation area=brand-identity rec=25
name: Nothing announces success before confirmation
rule: Nothing announces success before it is confirmed; sends stay visibly open until they are read back.
@row_add n=38 row=decision/strip-board-density area=brand-identity rec=25
name: Calm strip-board density
rule: The density is a strip board's, not a monitoring dashboard's: fixed fields, tabular numbers, one attention colour and one handoff colour, everything else paper and ink.
@row_add n=39 row=decision/style-direction area=brand-identity rec=25
name: Precision and Density style direction
rule: The style direction is Precision & Density as primary with Utility & Function as secondary.
@row_add n=40 row=decision/rejected-style-presets area=brand-identity rec=25
name: Rejected style presets
rule: The Dark Mode (OLED), Real-Time Monitoring (pulsing dots, blink, toasts) and HUD / Sci-Fi FUI (glow, telemetry animation) presets are not used.
@row_none n=4 rec=26 type=S
@row_add n=41 row=decision/monochrome-palette-ladder area=color-palette rec=27
name: Monochrome palette ladder
rule: The palette is a monochrome ladder of three surfaces (#1E2124, #2A2E33, #4A5057) and three inks (#E6D8AE, #9C9278, #3B3A36).
why: All values come from the Color World of the tower cab strip bay at night.
@row_add n=42 row=decision/two-hues-one-role-each area=color-palette rec=27
name: Two hues, each locked to one role
rule: Only two hues are allowed beyond the monochrome ladder, and each is locked to one role.
@row_add n=43 row=decision/amber-only-for-dialog-pending area=color-palette rec=28,31
name: Arrival amber for dialog-pending only
rule: Arrival-holder amber #D97706 is the only attention colour, used for `dialog_pending` only.
except: never for errors, warnings, budget or hover
@row_add n=44 row=decision/blue-only-for-transfer-markers area=color-palette rec=29,31
name: Departure blue for handoff only
rule: Departure-holder blue #5B8DB8 is used for transfer markers and `link` / `unlink` names only, never as a fill, never as text on holder, never for a status.
@row_add n=45 row=decision/handoff-blue-lightened area=color-palette rec=30
name: Handoff blue lightened for contrast
rule: The route blue is #5B8DB8, lightened from #2563EB.
why: #2563EB reaches only 3.5:1 contrast and fails.
@row_add n=46 row=decision/no-green-accent area=color-palette rec=30
name: No run-green accent
rule: The run-green accent of the Developer Tool palette is not used; amber is the single sharp accent.
@row_add n=47 row=contract/color-roles/primary area=color-palette rec=31
boundary: Colour roles
item: Primary #E6D8AE
promise: Strip buff is the primary ink, focus rings, information-carrying borders and the read-back fill.
@row_add n=48 row=contract/color-roles/secondary area=color-palette rec=31
boundary: Colour roles
item: Secondary #9C9278
promise: Lamp-off buff is the secondary ink on anthracite, the in-strip field grid and all ink of a `stale` strip.
@row_add n=49 row=decision/refused-send-folds-into-send-line area=color-palette rec=32
name: Refused send folds into its send line
rule: A refused send folds into its send line, never a separate line.
@row_add n=50 row=decision/skipped-count-stays-visible area=color-palette rec=32
name: Skipped count stays visible
rule: A nonzero `skipped` count stays visible and is never auto-cleared.
@row_add n=51 row=decision/bare-strip-never-cocked area=color-palette rec=32
name: Bare strip never cocked
rule: A bare (unwrapped) strip is never shown cocked.
@row_add n=52 row=decision/busy-not-animated area=color-palette rec=32
name: Busy has no animation
rule: The `busy` state has no spinner, pulse or animation.
@row_none n=5 rec=33 type=8
@row_add n=53 row=contract/borders/subtle area=color-palette rec=34
boundary: Border progression
item: Subtle #4A5057
promise: The subtle rail border is decorative only (2.0:1).
@row_add n=54 row=contract/borders/standard area=color-palette rec=34
boundary: Border progression
item: Standard #9C9278
promise: The standard border is the field grid inside a lit strip and the box around an ATIS `expired` word.
@row_add n=55 row=contract/borders/emphasis area=color-palette rec=34
boundary: Border progression
item: Emphasis #E6D8AE
promise: The emphasis border is for borders whose presence is the state.
@row_add n=56 row=contract/borders/focus area=color-palette rec=34
boundary: Border progression
item: Focus #E6D8AE
promise: The focus ring is 2px solid, offset 2px, radius 0.
@row_add n=57 row=contract/borders/handoff area=color-palette rec=34
boundary: Border progression
item: Handoff #5B8DB8
promise: The handoff border is only the transfer markers' 2px left tick, the only blue rule, never on holder, never a strip or cell border.
@row_none n=6 rec=35 type=S
@row_add n=58 row=decision/cli-terminal-font area=typography rec=36
name: CLI uses the terminal's own font
rule: The cli uses the user's configured terminal monospace; the brand shows through the fixed-column structure and the phraseology words.
@row_add n=59 row=decision/bold-only-name-column area=typography rec=36
name: Bold only for the NAME column
rule: In the cli, bold is used only for the NAME column.
@row_add n=60 row=decision/stale-rows-sgr-dim area=typography rec=36
name: Stale rows in SGR dim
rule: In the cli, SGR dim marks `stale` rows and amber marks the `DIALOG` word.
@row_add n=61 row=decision/cli-ascii-only area=typography rec=36,46
name: ASCII-only CLI output
rule: CLI output is ASCII only, at high density.
@row_none n=7 rec=37 type=S
@row_add n=62 row=decision/cli-two-space-columns area=spacing rec=38,46
name: Two spaces between CLI columns
rule: CLI spacing follows the fixed-width terminal grid, with two spaces between columns.
@row_none n=8 rec=39 type=S
@row_none n=9 rec=40 type=S
@row_none n=10 rec=41 type=S
@row_none n=11 rec=43 type=S
@row_add n=63 row=decision/text-glyphs-only area=iconography rec=45
name: Custom text glyphs only
rule: Iconography is custom text glyphs only; phraseology words carry the meaning.
@row_add n=64 row=contract/cli-glyphs/arrow area=iconography rec=46
boundary: CLI glyphs
item: ->
promise: The arrow glyph is `->`, with no inbound form.
@row_add n=65 row=contract/cli-glyphs/field-separator area=iconography rec=46
boundary: CLI glyphs
item: " - "
promise: ` - ` separates parts inside one field.
@row_add n=66 row=contract/viola-list/columns area=iconography rec=46
boundary: viola list output
item: columns
promise: `viola list` carries no link column in v1, so its six columns stay fixed.
@row_add n=67 row=decision/no-decorative-icons area=iconography rec=47
name: No decorative icons
rule: Icons clarify, not decorate: no robot or person icons, no status dots, no checkmarks and no warning triangles, since every such state is a printed word.
@row_none n=12 rec=48 type=S
@row_add n=68 row=absence/web-page-surface area=surface-web-spa rec=49,44
capability: A web page or web surface of any kind
@row_none n=13 rec=50 type=S
@row_none n=14 rec=51 type=S
@row_add n=69 row=decision/linux-terminal-platform area=surface-cli rec=52
name: Linux terminals as the CLI platform
rule: The cli targets Linux terminals: the core on any current distribution, CI on `ubuntu-latest`.
@row_add n=70 row=contract/cli-surface/output-streams area=surface-cli rec=53
boundary: CLI surface
item: stdout/stderr
promise: The cli surface is the stdout and stderr of the `viola` binary.
@row_add n=71 row=contract/cli-callers/human area=surface-cli rec=54
boundary: CLI callers
item: human on a TTY
promise: The human calls the cli on a TTY.
@row_add n=72 row=contract/cli-callers/llm-driver area=surface-cli rec=54
boundary: CLI callers
item: LLM driver sessions
promise: LLM driver sessions call the cli via `--json` or MCP.
@row_none n=15 rec=55 type=S
